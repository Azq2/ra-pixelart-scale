use clap::Parser;
use librashader_runtime_gl::{FilterChainGL, GLFramebuffer, GLImage};
use librashader_common::{Size, Viewport};
use gl_headless::gl_headless;
use image::{GenericImage, DynamicImage, ImageBuffer, GenericImageView};
use gl::types::{GLuint};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// Pixel Art scaling algorithms from RetroArch.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	/// Scale method
	#[arg(short, long, default_value = "scalefx-9x")]
	method: String,

	/// Input image
	#[arg(short, long)]
	input: String,

	/// Output scale
	#[arg(short, long, default_value_t = 0.0)]
	scale: f64,

	/// Output filename
	#[arg(short, long)]
	output: String,

	/// Output resize
	#[arg(long)]
	resize: Option<String>,

	/// Alpha saving mode: none, split
	#[arg(long, default_value = "split")]
	alpha: String,

	/// Custom .slangp file
	#[arg(long)]
	custom_preset: Option<String>,
}

#[gl_headless(version = "3.3")]
unsafe fn main() -> ExitCode {
	let args = Args::parse();
	let shaders_dir = get_shaders_dir();
	let shaders_index = load_shaders_index(shaders_dir.as_str());

	let rgba_img = image::open(args.input.as_str()).expect("Failed to load image");
	let (width, height) = rgba_img.dimensions();

	let preset_path: PathBuf;
	let default_scale: f64;

	if args.custom_preset.is_none() {
		let preset_filename = shaders_index[args.method.clone()][1].as_str();
		if preset_filename.is_none() {
			eprintln!("Scaling method {} not found.", args.method);
			return ExitCode::from(1);
		}
		preset_path = Path::new(shaders_dir.as_str()).join(preset_filename.unwrap().to_string());
		default_scale = shaders_index[args.method.clone()][0].as_f64().unwrap();
	} else {
		preset_path = PathBuf::from(args.custom_preset.unwrap().as_str());
		default_scale = 1.0;
	}

	let mut output_scale = args.scale;
	if args.scale <= 0.0 {
		output_scale = default_scale;
	}

	let output_size = Size::new(
		((width as f64) * output_scale).round() as u32,
		((height as f64) * output_scale).round() as u32
	);

	eprintln!("Preset: {}", preset_path.to_str().unwrap());
	eprintln!("Input: {}", args.input);
	eprintln!("Input size: {}x{}", width, height);
	eprintln!("Output: {}", args.output);
	eprintln!("Output size: {}x{} ({}x)", output_size.width, output_size.height, output_scale);

	gl::Viewport(0, 0, width as _, height as _);

	let (_rendered_framebuffer, rendered_texture) = create_texture(width, height);
	let (output_framebuffer, output_texture) = create_texture(output_size.width, output_size.height);

	let rendered = GLImage {
		handle: rendered_texture,
		format: gl::RGBA8,
		size: Size {
			width: width,
			height: height,
		},
	};
	let viewport = Viewport {
		x: 0f32,
		y: 0f32,
		output: &GLFramebuffer::new_from_raw(output_texture, output_framebuffer, gl::RGBA8, output_size, 1),
		mvp: None,
	};

	let mut filter = FilterChainGL::load_from_path(preset_path.to_str().unwrap(), None).unwrap();
	if args.alpha == "split" {
		let (rgb_img, alpha_img) = split_alpha(&rgba_img);
		let rgb_scaled = process_image(&mut filter, &rgb_img, &rendered, &viewport, output_size, output_framebuffer);
		let alpha_scaled = process_image(&mut filter, &alpha_img, &rendered, &viewport, output_size, output_framebuffer);
		let rgba_scaled = merge_alpha(&rgb_scaled, &alpha_scaled);
		rgba_scaled.save(args.output).expect("Image save() error");
	} else if args.alpha == "none" {
		let rgba_scaled = process_image(&mut filter, &rgba_img, &rendered, &viewport, output_size, output_framebuffer);
		rgba_scaled.save(args.output).expect("Image save() error");
	} else {
		panic!("Invalid alpha mode: {}", args.alpha);
	}
	return ExitCode::from(0);
}

unsafe fn process_image(
	filter: &mut FilterChainGL,
	img: &DynamicImage,
	rendered: &GLImage,
	viewport: &Viewport<&GLFramebuffer>,
	output_size: Size<u32>,
	fbo: GLuint
) -> DynamicImage {
	let data = img.clone().into_rgba8().into_raw();
	gl::BindTexture(gl::TEXTURE_2D, rendered.handle);
	gl::TexSubImage2D(gl::TEXTURE_2D, 0, 0, 0, rendered.size.width as _, rendered.size.height as _, gl::RGBA, gl::UNSIGNED_BYTE, data.as_ptr() as _);

	filter.frame(&rendered, &viewport, 120, None).expect("Can't process shader.");

	let mut tmp_buffer = vec![0u8; (output_size.width * output_size.height * 4) as usize];
	gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
	gl::ReadPixels(
		0,
		0,
		output_size.width as _,
		output_size.height as _,
		gl::RGBA,
		gl::UNSIGNED_BYTE,
		tmp_buffer.as_mut_ptr() as _,
	);
	let buffer: ImageBuffer<image::Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(output_size.width, output_size.height, tmp_buffer).unwrap();
	return DynamicImage::from(buffer);
}

fn load_shaders_index(shaders_dir: &str) -> serde_json::Value {
	let index_file = Path::new(shaders_dir).join("index.json");
	let file = std::fs::File::open(index_file).unwrap();
	return serde_json::from_reader(file).unwrap();
}

fn get_shaders_dir() -> String {
	let exe_file = std::env::current_exe().unwrap();
	let exe_dir = exe_file.parent().unwrap();

	if exe_dir.starts_with("/usr/bin") || exe_dir.starts_with("/usr/local/bin") {
		return exe_dir.join("../ra-pixelart-scale/shaders").to_str().unwrap().to_string();
	} else if Path::exists(&exe_dir.join("shaders")) {
		return exe_dir.join("shaders").to_str().unwrap().to_string();
	} else {
		return Path::new(env!("CARGO_MANIFEST_DIR")).join("shaders").to_str().unwrap().to_string();
	}
}

fn split_alpha(input_img: &DynamicImage) -> (DynamicImage, DynamicImage) {
	let rgba_img = input_img.to_rgba8();

	let (width, height) = rgba_img.dimensions();
	let mut rgb_img = DynamicImage::new(width, height, image::ColorType::Rgb8);
	let mut alpha_img = DynamicImage::new(width, height, image::ColorType::Rgb8);

	for y in 0..height {
		for x in 0..width {
			let pixel = rgba_img.get_pixel(x, y);
			let rgb_pixel = image::Rgba([pixel[0], pixel[1], pixel[2], 255]);
			let alpha_pixel = image::Rgba([pixel[3], pixel[3], pixel[3], 255]);
			rgb_img.put_pixel(x, y, rgb_pixel);
			alpha_img.put_pixel(x, y, alpha_pixel);
		}
	}

	return (rgb_img, alpha_img);
}

fn merge_alpha(rgb_img: &DynamicImage, alpha_img: &DynamicImage) -> DynamicImage {
	let (width, height) = rgb_img.dimensions();
	let mut rgba_img = DynamicImage::new(width, height, image::ColorType::Rgba8);

	for y in 0..height {
		for x in 0..width {
			let pixel = rgb_img.get_pixel(x, y);
			let alpha = alpha_img.get_pixel(x, y);
			let rgba_pixel = image::Rgba([pixel[0], pixel[1], pixel[2], alpha[0]]);
			rgba_img.put_pixel(x, y, rgba_pixel);
		}
	}

	return rgba_img;
}

unsafe fn create_texture(width: u32, height: u32) -> (GLuint, GLuint) {
	let mut framebuffer = 0;
	let mut texture = 0;

	gl::GenFramebuffers(1, &mut framebuffer);
	gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);

	gl::GenTextures(1, &mut texture);
	gl::BindTexture(gl::TEXTURE_2D, texture);

	gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as _);
	gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as _);
	gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as _);
	gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as _);

	gl::TexStorage2D(gl::TEXTURE_2D, 1, gl::RGBA8, width as _, height as _);
	gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, texture, 0);

	if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
		panic!("failed to create fbo")
	}

	return (framebuffer, texture);
}
