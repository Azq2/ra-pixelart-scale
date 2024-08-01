use clap::Parser;
use librashader_runtime_gl::options::FilterChainOptionsGL;
use librashader_runtime_gl::{FilterChainGL, GLFramebuffer, GLImage};
use librashader_common::{Size, Viewport};
use librashader::presets::{Scale2D, ScaleFactor, ScaleType, Scaling};
use librashader::presets::ShaderPreset;
use librashader::presets::context::VideoDriver;
use gl_headless::gl_headless;
use image::GenericImageView;
use image::ImageBuffer;
use gl::types::{GLchar, GLenum, GLint, GLsizei, GLuint, GLvoid};
use std::ops::Mul;
use num_traits::AsPrimitive;
use std::path::Path;
use std::process::ExitCode;

/// Pixel Art scaling algorithms from RetroArch.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	/// Scale method
	#[arg(short, long, default_value = "scalefx-9x")]
	method: String,

	/// Custom shader
	#[arg(short, long, default_value = "")]
	shader: String,

	/// Input one frame
	#[arg(short, long)]
	input: String,

	/// Output filename
	#[arg(short, long)]
	output: String,
}

#[gl_headless(version = "3.3")]
unsafe fn main() -> ExitCode {
	let args = Args::parse();
	let shaders_dir = get_shaders_dir();

	// Load input image
	let img = image::open(args.input.as_str()).expect("Failed to load image");
	let (width, height) = img.dimensions();
	let data = img.into_rgba8().into_raw();

	let shader_path = Path::new(&shaders_dir).join(format!("{}.slangp", args.method));
	if !shader_path.exists() {
		eprintln!("Scaling method {} not found.", args.method);
		return ExitCode::from(1);
	}

	let output_size = calc_output_size(shader_path.to_str().unwrap(), Size::new(width, height));

	eprintln!("Shader: {}", shader_path.to_str().unwrap());
	eprintln!("Input: {}", args.input);
	eprintln!("Input size: {}x{}", width, height);
	eprintln!("Output: {}", args.output);
	eprintln!("Output size: {}x{}", output_size.width, output_size.height);

	gl::Viewport(0, 0, width as _, height as _);

	let (_rendered_framebuffer, rendered_texture) = create_texture(width, height);
	let (output_framebuffer, output_texture) = create_texture(output_size.width, output_size.height);

	gl::BindTexture(gl::TEXTURE_2D, rendered_texture);
	gl::TexSubImage2D(gl::TEXTURE_2D, 0, 0, 0, width as _, height as _, gl::RGBA, gl::UNSIGNED_BYTE, data.as_ptr() as _);

	// Render shader
	let rendered = GLImage {
		handle: rendered_texture,
		format: gl::RGBA8,
		size: Size {
			width: width,
			height: height,
		},
	};

	let output = GLFramebuffer::new_from_raw(output_texture, output_framebuffer, gl::RGBA8, output_size, 1);

	let viewport = Viewport {
		x: 0f32,
		y: 0f32,
		output: &output,
		mvp: None,
	};

	gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

	let mut filter = FilterChainGL::load_from_path(shader_path.to_str().unwrap(), None).unwrap();
	filter.frame(&rendered, &viewport, 0, None).unwrap();

	gl::BindFramebuffer(gl::FRAMEBUFFER, output_framebuffer);
	render_as_png(args.output.as_str(), output_size);

	return ExitCode::from(0);
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

unsafe fn render_as_png(file: &str, size: Size<u32>) {
	let mut data = vec![0u8; (size.width * size.height * 4) as usize];
	gl::ReadPixels(
		0,
		0,
		size.width as _,
		size.height as _,
		gl::RGBA,
		gl::UNSIGNED_BYTE,
		data.as_mut_ptr() as _,
	);
	let buffer: ImageBuffer<image::Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(size.width, size.height, data).unwrap();
	buffer.save(file).expect("Image saving failed");
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

unsafe fn calc_output_size(shader_path: &str, original_size: Size<u32>) -> Size<u32> {
	let preset = ShaderPreset::try_parse_with_driver_context(shader_path, VideoDriver::GlCore).unwrap();

	let mut iterator = preset.shaders.iter().enumerate().peekable();
	let mut target_size = original_size;

	while let Some((_index, shader_config)) = iterator.next() {
		let next_size = scale_pass::<u32>(
			shader_config.scaling.clone(),
			target_size,
			original_size,
			original_size,
		);
		target_size = next_size;
	}

	return target_size;
}

// From librashader-runtime/src/scaling.rs
fn scale_pass<T>(scaling: Scale2D, source: Size<T>, viewport: Size<T>, original: Size<T>) -> Size<T>
where
	T: Mul<ScaleFactor, Output = f32> + Copy + Ord + 'static,
	f32: AsPrimitive<T>,
{
	const MAX_TEXEL_SIZE: f32 = 16384f32;

	let width = match scaling.x {
		Scaling {
			scale_type: ScaleType::Input,
			factor,
		} => source.width * factor,
		Scaling {
			scale_type: ScaleType::Absolute,
			factor,
		} => factor.into(),
		Scaling {
			scale_type: ScaleType::Viewport,
			factor,
		} => viewport.width * factor,
		Scaling {
			scale_type: ScaleType::Original,
			factor,
		} => original.width * factor,
	};

	let height = match scaling.y {
		Scaling {
			scale_type: ScaleType::Input,
			factor,
		} => source.height * factor,
		Scaling {
			scale_type: ScaleType::Absolute,
			factor,
		} => factor.into(),
		Scaling {
			scale_type: ScaleType::Viewport,
			factor,
		} => viewport.height * factor,
		Scaling {
			scale_type: ScaleType::Original,
			factor,
		} => original.height * factor,
	};

	Size {
		width: std::cmp::min(
			std::cmp::max(width.round().as_(), 1f32.as_()),
			MAX_TEXEL_SIZE.as_(),
		),
		height: std::cmp::min(
			std::cmp::max(height.round().as_(), 1f32.as_()),
			MAX_TEXEL_SIZE.as_(),
		),
	}
}
