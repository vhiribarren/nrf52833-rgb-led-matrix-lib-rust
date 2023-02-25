use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use clap::{Parser, ValueEnum};
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView};

#[derive(Clone, ValueEnum)]
enum Mode {
    Icon,
    Stencil,
}

#[derive(Parser)]
struct Cli {
    in_image: PathBuf,
    #[arg(short, long)]
    out_rust: Option<PathBuf>,
    #[arg(value_enum, short, long, default_value_t = Mode::Icon)]
    mode: Mode,
    #[arg(short, long)]
    name: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let image = ImageReader::open(&args.in_image)?.decode()?;
    let name = match args.name {
        Some(n) => n,
        None => args
            .in_image
            .file_stem()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap(),
    }
    .to_uppercase();
    let output_string = match args.mode {
        Mode::Icon => generate_icon(&image, &name),
        Mode::Stencil => todo!(),
    };
    match args.out_rust {
        Some(file) => {
            if Path::new(file.as_os_str()).exists() {
                panic!();
            }
            fs::write(file, output_string)?;
        }
        None => println!("{}", output_string),
    }
    Ok(())
}

fn generate_icon(image: &DynamicImage, name: &str) -> String {
    let (width, height) = image.dimensions();
    let mut code = String::new();
    code.push_str("use crate::canvas::{Canvas, Color};");
    code.push_str(format!("pub const {name}: Canvas<{width}, {height}> = Canvas([").as_str());
    for y in 0..height {
        code.push_str("[");
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            let [r, g, b, a] = pixel.0;
            if a > u8::MAX / 2 {
                code.push_str(format!("Color::new({r},{g},{b}),").as_str());
            } else {
                code.push_str("Color::BLACK,");
            }
        }
        code.push_str("],");
    }
    code.push_str("]);");
    code
}
