use webp::Encoder;

pub fn convert_to_webp(img: image::DynamicImage) -> webp::WebPMemory {
    let encoder = Encoder::from_image(&img).unwrap();
    let webp_img = encoder.encode(100.0);

    webp_img
}
