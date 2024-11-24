use clap::{Args, Parser, Subcommand};
use image::{DynamicImage, ImageBuffer};

const BIT_HIGH_VALUE: u8 = 255;
// Allow some margin when decoding, especially for lossy image types like jpg
const BIT_HIGH_VALUE_THRESHOLD: u8 = BIT_HIGH_VALUE / 2;

/// Returns the bit at the given index in the provided list of bytes. If the bit index is out of 
/// range, `None` will be returned.
fn bit_at_index(bit_index: usize, bytes: &[u8]) -> Option<u8> {
    let byte_index = bit_index / 8;

    if bytes.len() > byte_index {
        let byte = bytes[byte_index];
        let byte_bit_index = (bit_index % 8) as u8;
        let bit_shift = 7 - byte_bit_index;
        let bit_value = (byte >> bit_shift) & 1;

        Some(bit_value)
    } else {
        None
    }
}

/// Converts the given list of bytes into an image, where each byte is stored in 8 subpixels. 
/// This means the first byte is spread out over the first 3 pixels. If the bit is high (1) the 
/// value for the related subpixel will be set to 255 (#FF), If the bit is low (0) the value for the 
/// related subpixel will be set to 0 (#00).
/// 
/// If the given list of bytes does not fit into an exact number of pixels, the remaining subpixels
/// in the last pixel will be set to 0.
/// 
/// For example a value of 0b0101_0101 will be stored as 3 pixels with the following colors: 
/// #00FF00 #FF00FF #00FF00
#[allow(clippy::identity_op)]
fn bytes_to_image(bytes: &[u8]) -> DynamicImage {
    let pixels_per_byte = 8f32 / 3f32;
    let image_width = (bytes.len() as f32 * pixels_per_byte).ceil() as u32;
    let image_height = 1;

    let mut image_buffer = ImageBuffer::new(image_width, image_height);

    for (x, _, pixel) in image_buffer.enumerate_pixels_mut() {
        let r_bit_index = (x * 3 + 0) as usize;
        let g_bit_index = (x * 3 + 1) as usize;
        let b_bit_index = (x * 3 + 2) as usize;

        let r_bit = bit_at_index(r_bit_index, bytes);
        let g_bit = bit_at_index(g_bit_index, bytes);
        let b_bit = bit_at_index(b_bit_index, bytes);

        let r = r_bit.map_or(0, |x| x * BIT_HIGH_VALUE);
        let g = g_bit.map_or(0, |x| x * BIT_HIGH_VALUE);
        let b = b_bit.map_or(0, |x| x * BIT_HIGH_VALUE);

        *pixel = image::Rgb([r, g, b]);
    }

    DynamicImage::ImageRgb8(image_buffer)
}

/// Loops through each subpixel, collects them in chunks of 8, and then flattens all that into a 
/// single byte again. If the value of a subpixel is greater than BIT_HIGH_VALUE_THRESHOLD, it is
/// treated as a 1, otherwise it's a 0.
fn image_to_bytes(image: DynamicImage) -> Vec<u8> {
    let bytes = image
        .to_rgb8()
        .into_raw()
        .chunks_exact(8)
        .map(|byte_values| {
            let mut byte = 0;

            for (bit_index, bit_value) in byte_values.iter().enumerate() {
                if bit_value > &BIT_HIGH_VALUE_THRESHOLD {
                    let bit_shift = 7 - bit_index;
                    byte += 1 << bit_shift
                }
            }

            byte
        })
        .collect();

    bytes
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Encode(Encode),
    Decode(Decode),
}

#[derive(Args)]
struct Encode {
    #[arg(short, long, required = true)]
    message: String,

    #[arg(short, long, required = true)]
    output: String,
}

#[derive(Args)]
struct Decode {
    #[arg(short, long, required = true)]
    input: String,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Command::Encode(command) => {
            let bytes = command.message.as_bytes();
            let image = bytes_to_image(bytes);
            
            image.save(&command.output).expect("Failed to save image");
        },
        Command::Decode(command) => {
            let image = image::open(&command.input).expect("Failed to open input");
            let bytes = image_to_bytes(image);
            let output = String::from_utf8(bytes).expect("Failed to decode image");

            println!("{output}");
        },
    }
}
