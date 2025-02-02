use clap::Parser;
use librashader_runtime_gl::{FilterChainGL, GLFramebuffer, GLImage};
use librashader_common::{Size, Viewport};
use gl_headless::gl_headless;
use image::{GenericImage, DynamicImage, ImageBuffer, GenericImageView};
use gl::types::{GLuint};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// PixelArt scaling algorithms from RetroArch.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	/// Scale method
	#[arg(short, long, default_value = "scalefx-9x")]
	method: String,

	/// List available methods
	#[arg(long)]
	list_methods: bool,

	/// Input image
	#[arg(short, long)]
	input: Option<String>,

	/// Output scale
	#[arg(short, long, default_value_t = 0.0)]
	scale: f64,

	/// Output filename
	#[arg(short, long)]
	output: Option<String>,

	/// Resize after scale (WxH or %)
	#[arg(long)]
	resize: Option<String>,

	/// Resize method: nearest, triangle, catmullrom, gaussian, lanczos3
	#[arg(long, default_value = "nearest")]
	resize_method: String,

	/// Alpha mode: auto, strip, bypass, split
	#[arg(long, default_value = "auto")]
	alpha: String,

	/// Custom .slangp file
	#[arg(long)]
	custom_preset: Option<String>,
}

struct ScaleMethod {
	name: String,
	file: String,
	scale_min: f64,
	scale_max: f64,
	alpha: bool,
}

fn main() -> ExitCode {
	let args = Args::parse();

	if args.list_methods {
		let config = get_config();
		for (key, _value) in config.as_object().unwrap() {
			eprintln!("{}", key);
		}
		return ExitCode::from(1);
	}

	if args.input.is_none() {
		eprintln!("--input is not specified");
		return ExitCode::from(1);
	}

	if args.output.is_none() {
		eprintln!("--output is not specified");
		return ExitCode::from(1);
	}

	let img = image::open(args.input.as_ref().unwrap().as_str()).expect("Failed to load image");
	let (width, height) = img.dimensions();

	let scale_method = get_scaling_method(&args).expect("Invalid --method value");
	let mut alpha_mode = args.alpha.clone();

	if alpha_mode == "auto" {
		if scale_method.alpha {
			alpha_mode = String::from("bypass");
		} else {
			alpha_mode = String::from("split");
		}
	}

	let mut scaled_img: Option<DynamicImage>;
	if alpha_mode == "split" {
		let (rgb_img, alpha_img) = split_alpha(&img);
		let scaled_rgb_img = scale_image(&args, &scale_method, &rgb_img);
		let scaled_alpha_img = scale_image(&args, &scale_method, &alpha_img);

		if !scaled_rgb_img.is_none() && !scaled_alpha_img.is_none() {
			scaled_img = Some(merge_alpha(&scaled_rgb_img.unwrap(), &scaled_alpha_img.unwrap()));
		} else {
			scaled_img = None;
		}
	} else if alpha_mode == "strip" {
		let (rgb_img, _alpha_img) = split_alpha(&img);
		let scaled_rgb_img = scale_image(&args, &scale_method, &rgb_img);
		if !scaled_rgb_img.is_none() {
			scaled_img = Some(scaled_rgb_img.unwrap());
		} else {
			scaled_img = None;
		}
	} else if alpha_mode == "bypass" {
		scaled_img = scale_image(&args, &scale_method, &img);
	} else {
		eprintln!("Invalid alpha mode: {}", args.alpha);
		return ExitCode::from(1);
	}

	if scaled_img.is_none() {
		eprintln!("Failed to scale image.");
		return ExitCode::from(1);
	}

	if !args.resize.is_none() {
		let new_width: u32;
		let new_height: u32;

		if args.resize.as_ref().unwrap().ends_with("%") {
			let cleaned_input: String = args.resize.unwrap().chars()
				.filter(|c| c.is_digit(10) || *c == '.')
				.collect();
			let scale = cleaned_input.parse::<f64>().expect("Invalid --resize value") / 100.0;
			new_width = ((width as f64) * scale).round() as u32;
			new_height = ((height as f64) * scale).round() as u32;
		} else {
			let resize_tmp = args.resize.as_ref().unwrap();
			new_width = resize_tmp.split("x").nth(0).unwrap().parse::<u32>().expect("Invalid --resize value");
			new_height = resize_tmp.split("x").nth(1).unwrap().parse::<u32>().expect("Invalid --resize value");
		}

		let resize_method = get_resize_method_by_name(args.resize_method.as_str()).expect("Invalid --resize-method");
		scaled_img = Some(scaled_img.unwrap().resize(new_width, new_height, resize_method));
	}


	let (scaled_width, scaled_height) = scaled_img.as_ref().unwrap().dimensions();
	println!("Image saved to: {} [{}x{} -> {}x{}]", args.output.as_ref().unwrap(), width, height, scaled_width, scaled_height);
	scaled_img.unwrap().save(args.output.as_ref().unwrap()).expect("Failed to save image");
	return ExitCode::from(0);
}

fn scale_image(args: &Args, scale_method: &ScaleMethod, img: &DynamicImage) -> Option<DynamicImage> {
	let (width, height) = img.dimensions();

	let mut output_scale = args.scale;
	if args.scale <= 0.0 {
		output_scale = f64::max(2.0, scale_method.scale_min);
	}
	let out_width = ((width as f64) * output_scale).round() as u32;
	let out_height = ((height as f64) * output_scale).round() as u32;

	if scale_method.scale_max != 100.0 {
		if output_scale.ceil() != output_scale || output_scale < scale_method.scale_min || output_scale > scale_method.scale_max {
			eprintln!(
				"Warning: scale {} is not supported by {}, real scale changed to {} and then resized with nearest-neighbor.",
				output_scale, scale_method.name.clone(), scale_method.scale_max
			);
		}
	}

	if args.custom_preset.is_none() && args.method.starts_with("rust-") {
		if args.method == "rust-xbrz" {
			return scale_xbrz(img, out_width, out_height);
		} else if args.method == "rust-xbr" {
			return scale_xbr(img, out_width, out_height);
		} else if args.method == "rust-scalenx" {
			return scale_scalenx(img, out_width, out_height);
		} else if args.method == "rust-eagle" {
			return scale_eagle(img, out_width, out_height);
		} else if args.method == "rust-mmpx" {
			return scale_mmpx(img, out_width, out_height);
		} else if args.method == "rust-hqx" {
			return scale_hqx(img, out_width, out_height);
		} else {
			panic!("Internal error?");
		}
	} else {
		let shaders_dir = get_shaders_dir();
		let preset_path = Path::new(shaders_dir.as_str()).join(scale_method.file.clone());
		unsafe {
			return Some(scale_with_shader(&preset_path, img, out_width, out_height));
		}
	}
}

fn scale_xbrz(img: &DynamicImage, out_width: u32, out_height: u32) -> Option<DynamicImage> {
	let (width, height) = img.dimensions();

	let real_scale = (out_width as f32) / (width as f32);
	let scale = f32::max(1.0, f32::min(6.0, real_scale.ceil()));

	let scaled_width = width * (scale as u32);
	let scaled_height = height * (scale as u32);

	let rgba_img = img.to_rgba8();
	let bytes = xbrz::scale_rgba(
		&rgba_img,
		width as _,
		height as _,
		scale as _
	);

	let tmp_buffer: ImageBuffer<image::Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(scaled_width, scaled_height, bytes).unwrap();
	let mut scaled_image = DynamicImage::from(tmp_buffer);

	if scaled_width != out_width || scaled_height != out_height {
		scaled_image = scaled_image.resize(out_width, out_height, image::imageops::FilterType::Nearest);
	}

	return Some(scaled_image);
}

fn scale_xbr(img: &DynamicImage, out_width: u32, out_height: u32) -> Option<DynamicImage> {
	let (width, height) = img.dimensions();

	let data = img.clone().into_rgba8().into_raw();
	let input_block = xbr::Block::new(data, width, height);

	let scaled_block = xbr::x2(input_block);

	let tmp_buffer: ImageBuffer<image::Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(scaled_block.width, scaled_block.height, scaled_block.bytes).unwrap();
	let mut scaled_image = DynamicImage::from(tmp_buffer);

	if scaled_block.width != out_width || scaled_block.height != out_height {
		scaled_image = scaled_image.resize(out_width, out_height, image::imageops::FilterType::Nearest);
	}

	return Some(scaled_image);
}

fn scale_scalenx(img: &DynamicImage, out_width: u32, out_height: u32) -> Option<DynamicImage> {
	let (width, _height) = img.dimensions();
	let real_scale = (out_width as f32) / (width as f32);
	let scale = f32::max(2.0, f32::min(3.0, real_scale.ceil()));

	let mut scaled_image: DynamicImage;
	if scale == 2.0 {
		scaled_image = magnify::convert(img.clone(), magnify::Algorithm::Scale2X);
	} else if scale == 3.0 {
		scaled_image = magnify::convert(img.clone(), magnify::Algorithm::Scale3X);
	} else {
		panic!("Internal error?");
	}

	let (scaled_width, scaled_height) = scaled_image.dimensions();
	if scaled_width != out_width || scaled_height != out_height {
		scaled_image = scaled_image.resize(out_width, out_height, image::imageops::FilterType::Nearest);
	}

	return Some(scaled_image);
}

fn scale_eagle(img: &DynamicImage, out_width: u32, out_height: u32) -> Option<DynamicImage> {
	let mut scaled_image = magnify::convert(img.clone(), magnify::Algorithm::Eagle);

	let (scaled_width, scaled_height) = scaled_image.dimensions();
	if scaled_width != out_width || scaled_height != out_height {
		scaled_image = scaled_image.resize(out_width, out_height, image::imageops::FilterType::Nearest);
	}

	return Some(scaled_image);
}

fn scale_mmpx(img: &DynamicImage, out_width: u32, out_height: u32) -> Option<DynamicImage> {
	let tmp_buffer = mmpx::magnify(&img.to_rgba8());
	let mut scaled_image = DynamicImage::from(tmp_buffer);

	let (scaled_width, scaled_height) = scaled_image.dimensions();
	if scaled_width != out_width || scaled_height != out_height {
		scaled_image = scaled_image.resize(out_width, out_height, image::imageops::FilterType::Nearest);
	}

	return Some(scaled_image);
}

fn scale_hqx(img: &DynamicImage, out_width: u32, out_height: u32) -> Option<DynamicImage> {
	let (width, height) = img.dimensions();
	let real_scale = (out_width as f32) / (width as f32);
	let scale = f32::max(2.0, f32::min(4.0, real_scale.ceil()));

	let scaled_width = width * (scale as u32);
	let scaled_height = height * (scale as u32);
	let input_pixels = img.clone().into_rgba8().into_raw();
	let output_pixels_cnt = (scaled_width * scaled_height) as usize;
	let mut output_pixels32: Vec<u32> = vec![0; output_pixels_cnt];
	let input_pixels32: &[u32] = unsafe { std::slice::from_raw_parts(input_pixels.as_ptr() as *const u32, input_pixels.len() / 4) };

	if scale == 2.0 {
		hqx::hq2x(input_pixels32, &mut output_pixels32, width as _, height as _);
	} else if scale == 3.0 {
		hqx::hq3x(input_pixels32, &mut output_pixels32, width as _, height as _);
	} else if scale == 4.0 {
		hqx::hq4x(input_pixels32, &mut output_pixels32, width as _, height as _);
	} else {
		panic!("Internal error?");
	}

	let output_pixels = u32_vec_to_u8_vec(output_pixels32);
	let tmp_buffer: ImageBuffer<image::Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(scaled_width, scaled_height, output_pixels).unwrap();
	let mut scaled_image = DynamicImage::from(tmp_buffer);

	let (scaled_width, scaled_height) = scaled_image.dimensions();
	if scaled_width != out_width || scaled_height != out_height {
		scaled_image = scaled_image.resize(out_width, out_height, image::imageops::FilterType::Nearest);
	}

	return Some(scaled_image);
}

#[gl_headless(version = "3.3")]
unsafe fn scale_with_shader(preset_path: &PathBuf, img: &DynamicImage, out_width: u32, out_height: u32) -> DynamicImage {
	let mut filter = FilterChainGL::load_from_path(preset_path.to_str().unwrap(), None).unwrap();

	let (width, height) = img.dimensions();
	let output_size = Size::new(out_width, out_height);

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

	let data = img.clone().into_rgba8().into_raw();
	gl::BindTexture(gl::TEXTURE_2D, rendered.handle);
	gl::TexSubImage2D(gl::TEXTURE_2D, 0, 0, 0, rendered.size.width as _, rendered.size.height as _, gl::RGBA, gl::UNSIGNED_BYTE, data.as_ptr() as _);

	filter.frame(&rendered, &viewport, 120, None).expect("Can't process shader.");

	let mut tmp_buffer = vec![0u8; (output_size.width * output_size.height * 4) as usize];
	gl::BindFramebuffer(gl::FRAMEBUFFER, output_framebuffer);
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

fn get_scaling_method(args: &Args) -> Option<ScaleMethod> {
	if !args.custom_preset.is_none() {
		let method: ScaleMethod = ScaleMethod {
			name: String::from("custom"),
			file: args.custom_preset.clone().unwrap(),
			scale_min: 1.0,
			scale_max: 1.0,
			alpha: true
		};
		return Some(method);
	} else {
		let config = get_config();
		let method_json = &config[args.method.clone()];
		if method_json.is_null() {
			return None;
		}
		let method: ScaleMethod = ScaleMethod {
			name: method_json["name"].as_str().unwrap().to_string(),
			file: method_json["file"].as_str().unwrap().to_string(),
			scale_min: method_json["minScale"].as_f64().unwrap(),
			scale_max: method_json["maxScale"].as_f64().unwrap(),
			alpha: method_json["alpha"].as_bool().unwrap(),
		};
		return Some(method);
	}
}

fn get_shaders_dir() -> String {
	let exe_file = std::env::current_exe().unwrap();
	let exe_dir = exe_file.parent().unwrap();

	if exe_dir.starts_with("/usr/bin") || exe_dir.starts_with("/usr/local/bin") {
		return exe_dir.join("../share/ra-pixelart-scale/shaders").to_str().unwrap().to_string();
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

fn get_config() -> serde_json::Value {
	return serde_json::from_str(include_str!("index.json")).unwrap();
}

fn get_resize_method_by_name(filter_name: &str) -> Option<image::imageops::FilterType> {
	match filter_name.to_lowercase().as_str() {
		"nearest"		=> Some(image::imageops::FilterType::Nearest),
		"triangle"		=> Some(image::imageops::FilterType::Triangle),
		"catmullrom"	=> Some(image::imageops::FilterType::CatmullRom),
		"gaussian"		=> Some(image::imageops::FilterType::Gaussian),
		"lanczos3"		=> Some(image::imageops::FilterType::Lanczos3),
		_				=> None,
	}
}

fn u32_vec_to_u8_vec(input: Vec<u32>) -> Vec<u8> {
	let byte_length = input.len() * 4;
	let byte_slice: &[u8] = unsafe { std::slice::from_raw_parts(input.as_ptr() as *const u8, byte_length) };
	return byte_slice.to_vec();
}
