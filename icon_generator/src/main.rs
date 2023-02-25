/*
MIT License

Copyright (c) 2023 Vincent Hiribarren

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

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
