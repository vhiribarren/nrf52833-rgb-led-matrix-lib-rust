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

use clap::error::ErrorKind;
use clap::{CommandFactory, Parser, ValueEnum};
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView};

#[derive(Clone, ValueEnum)]
enum Mode {
    Icon,
    Stencil,
}

#[derive(Clone, ValueEnum, PartialEq)]
enum Format {
    Binary,
    Text,
}

#[derive(Parser)]
#[command(author, version, about, long_about)]
/// Utility to generate Rust files from images, to be used with the
/// nrf52833-rgb-led-matrix lib
///
/// It should only be used on tiny images, like 32x32 pixels.
///
/// In icon mode, the alpha channel must be above 0.5.
///
/// In stencil mode, the alpha channel must be above 0.5, and as long as another
/// color than white is used, it is considered part of the stencil.
struct Cli {
    /// Input file to convert to Rust code for the nrf52833-rgb-led-matrix lib
    image_file: PathBuf,
    #[arg(short, long)]
    /// By default, output to standard output. A file can be declared as output target.
    ///
    /// Required when output format is binary
    output_file: Option<PathBuf>,
    /// Generate RGB canvas images (icons), or stencils (one color, less heavy)
    #[arg(value_enum, short, long, default_value_t = Mode::Icon)]
    mode: Mode,
    /// Select textual Rust code, or binary (to use with e.g. include_bytes! macro)
    #[arg(value_enum, short, long, default_value_t = Format::Text)]
    format: Format,
    /// Name of Rust element. By default, try to uppercase and use the filename
    #[arg(short, long)]
    name: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    if args.format == Format::Binary && args.output_file.is_none() {
        let mut cmd = Cli::command();
        cmd.error(
            ErrorKind::ArgumentConflict,
            "When format is binary, an output file must be provided",
        )
        .exit()
    }

    let image = ImageReader::open(&args.image_file)?
        .decode()
        .map_err(|_| "Unsupported image format.")?;
    let element_name = match args.name {
        Some(n) => n,
        None => args
            .image_file
            .file_stem()
            .ok_or("No file name")?
            .to_os_string()
            .into_string()
            .map_err(|_| "Error while converting filename to element name")?,
    }
    .to_uppercase();
    let output_data = match (args.mode, args.format) {
        (Mode::Icon, Format::Text) => generate_icon_txt(&image, &element_name).into_bytes(),
        (Mode::Icon, Format::Binary) => generate_icon_bin(&image),
        (Mode::Stencil, Format::Text) => generate_stencil_txt(&image, &element_name).into_bytes(),
        (Mode::Stencil, Format::Binary) => generate_stencil_bin(&image),
    };
    match args.output_file {
        Some(file) => {
            if Path::new(file.as_os_str()).exists() {
                return Err(format!("Filename {} already exists", file.display()))?;
            }
            fs::write(file, output_data)?;
        }
        None => {
            let output_string = String::from_utf8(output_data)?;
            println!("{output_string}")
        }
    }
    Ok(())
}

#[allow(clippy::single_char_add_str)]
fn generate_icon_txt(image: &DynamicImage, element_name: &str) -> String {
    let (width, height) = image.dimensions();
    let mut array_rows = String::new();

    for y in 0..height {
        array_rows.push_str("[");
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            let [r, g, b, a] = pixel.0;
            if a > u8::MAX / 2 {
                array_rows.push_str(format!("Color::new({r},{g},{b}),").as_str());
            } else {
                array_rows.push_str("Color::BLACK,");
            }
        }
        array_rows.push_str("],");
    }

    format!(
        r#"use crate::canvas::{{Canvas, Color}};
pub const {element_name}: Canvas<{width}, {height}> = Canvas([
{array_rows}
]);"#
    )
}

fn generate_icon_bin(image: &DynamicImage) -> Vec<u8> {
    let (width, height) = image.dimensions();
    let mut data = Vec::<u8>::new();
    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            let [r, g, b, a] = pixel.0;
            if a > u8::MAX / 2 {
                data.push(r);
                data.push(g);
                data.push(b)
            } else {
                (0..3).for_each(|_| data.push(0));
            }
        }
    }
    data
}

#[allow(clippy::single_char_add_str)]
fn generate_stencil_txt(image: &DynamicImage, element_name: &str) -> String {
    let (width, height) = image.dimensions();
    let mut array_rows = String::new();

    for y in 0..height {
        array_rows.push_str("[");
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            let [r, g, b, a] = pixel.0;
            if a > u8::MAX / 2 && r < u8::MAX && g < u8::MAX && b < u8::MAX {
                array_rows.push_str("1,");
            } else {
                array_rows.push_str("0,");
            }
        }
        array_rows.push_str("],");
    }

    format!(
        r#"use crate::canvas::Stencil;
pub const {element_name}: Stencil<{width}, {height}> = Stencil([
{array_rows}
]);"#
    )
}

#[allow(clippy::single_char_add_str)]
fn generate_stencil_bin(image: &DynamicImage) -> Vec<u8> {
    let (width, height) = image.dimensions();
    let mut data = Vec::<u8>::new();
    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            let [r, g, b, a] = pixel.0;
            if a > u8::MAX / 2 && r < u8::MAX && g < u8::MAX && b < u8::MAX {
                data.push(1);
            } else {
                data.push(0);
            }
        }
    }
    data
}
