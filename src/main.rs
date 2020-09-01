use argparse::{ArgumentParser, Store, List};
use std::path::PathBuf;
use std::str::FromStr;
use image::{GenericImageView, DynamicImage};


fn main() {
    let mut image_paths: Vec<String> = vec![];
    let mut output_path = String::new();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Image concatenate.");
        ap.refer(&mut image_paths).add_argument("images", List, "input images");
        ap.refer(&mut output_path).add_option(&["--output", "-o"], Store, "output image path");
        ap.parse_args_or_exit();
    }

    let images: Vec<DynamicImage> = image_paths
        .iter()
        .map(|path| image::open(path.clone()).expect(format!("image not found {:?}", path).as_str()))
        .collect();

    assert!(images.iter().map(|image| image.dimensions().0).fold((true, None), |x, width| {
        match x.1 {
            None => (true, Some(width)),
            Some(w) => (x.0 && width == w, Some(w))
        }
    }).0);

    let mut result_image = image::ImageBuffer::new(images.first().unwrap().width(),
                                                   images.iter().map(|image| image.dimensions().1).sum()
    );

    let mut y = 0;
    for (i, image) in images.iter().enumerate() {
        image::imageops::replace(&mut result_image, image, 0, y);
        y += image.dimensions().1;
    }

    result_image.save(output_path).unwrap();
}
