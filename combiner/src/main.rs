mod args;
use args::Args;
use image::{
    imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageError,
    ImageFormat,
};
use std::{convert::TryInto, fs::File, io::BufReader};

#[derive(Debug)]
enum ImageDataErrors {
    DifferentImageFormats,
    BufferTooSmall,
    UnableToReadImageFromPath(std::io::Error),
    UnableToFromatImage(String),
    UnabletoDecodeImage(ImageError),
}

struct FloatingImage {
    widht: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

impl FloatingImage {
    fn new(widht: u32, height: u32, name: String) -> Self {
        let buffer_capacity = height * widht * 4;
        let buffer = Vec::with_capacity(buffer_capacity.try_into().unwrap());
        FloatingImage {
            widht,
            height,
            data: buffer,
            name,
        }
    }
    fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferTooSmall);
        }
        self.data = data;
        Ok(())
    }
}

fn main() -> Result<(), ImageDataErrors> {
    let args = Args::new();

    let (image_0, image_format_0) = find_image_from_path(args.image_0)?;
    let (image_1, image_format_1) = find_image_from_path(args.image_1)?;

    if image_format_0 != image_format_1 {
        return Err(ImageDataErrors::DifferentImageFormats);
    }

    let (image_0, image_1) = standartize_size(image_0, image_1);
    let mut output = FloatingImage::new(image_0.width(), image_0.height(), args.output);

    let combined_data = combine_images(image_0, image_1);
    output.set_data(combined_data)?;

    image::save_buffer_with_format(
        output.name,
        &output.data,
        output.widht,
        output.height,
        image::ColorType::Rgba8,
        image_format_0,
    )
    .unwrap();
    Ok(())
}

fn find_image_from_path(path: String) -> Result<(DynamicImage, ImageFormat), ImageDataErrors> {
    //let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    match Reader::open(&path) {
        Ok(image_reader) => {
            if let Some(image_format) = image_reader.format() {
                match image_reader.decode() {
                    Ok(image) => Ok((image, image_format)),
                    Err(e) => Err(ImageDataErrors::UnabletoDecodeImage(e)),
                }
            } else {
                return Err(ImageDataErrors::UnableToFromatImage(path));
            }
        }
        Err(e) => Err(ImageDataErrors::UnableToReadImageFromPath(e)),
    }
}

fn get_smallest_dimensions(dim_0: (u32, u32), dim_1: (u32, u32)) -> (u32, u32) {
    let pix_0 = dim_0.0 * dim_0.1;
    let pix_1 = dim_1.0 * dim_1.1;
    if pix_0 < pix_1 {
        dim_0
    } else {
        dim_1
    }
}

fn standartize_size(image_0: DynamicImage, image_1: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_dimensions(image_0.dimensions(), image_1.dimensions());
    println!("widht: {} \n height: {}", width, height);

    if image_1.dimensions() == (width, height) {
        (image_0.resize_exact(width, height, Triangle), image_1)
    } else {
        (image_0, image_1.resize_exact(width, height, Triangle))
    }
}

fn combine_images(image_0: DynamicImage, image_1: DynamicImage) -> Vec<u8> {
    let vec_0 = image_0.to_rgba8().into_vec();
    let vec_1 = image_1.to_rgba8().into_vec();

    alternative_pixels(vec_0, vec_1)
}

fn alternative_pixels(vec_0: Vec<u8>, vec_1: Vec<u8>) -> Vec<u8> {
    let mut combined_data = vec![0u8; vec_0.len()];

    let mut i = 0;
    while i < vec_0.len() {
        if i % 8 == 0 {
            combined_data.splice(i..=i + 3, set_rgba(&vec_0, i, i + 3));
        } else {
            combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
        }
        i += 4;
    }
    combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();
    for i in start..=end {
        let val: u8 = match vec.get(i) {
            Some(d) => *d,
            None => panic!("Index is out of bounds"),
        };
        rgba.push(val);
    }
    rgba
}
