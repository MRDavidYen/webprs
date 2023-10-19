use webp::Encoder;

pub fn convert_to_webp(img: image::DynamicImage, quality: &f32) -> webp::WebPMemory {
    let encoder = Encoder::from_image(&img).unwrap();
    let webp_img = encoder.encode(*quality);

    webp_img
}
