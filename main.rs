use image::{ImageBuffer, Rgb, RgbImage};

fn render() {
    const IMAGE_WIDTH: u32 = 1024;
    const IMAGE_HEIGHT: u32 = 768;

    let mut frame_buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for (x, y, pixel) in frame_buffer.enumerate_pixels_mut() {
        let r = (255.0 * (x as f32 / IMAGE_WIDTH as f32)) as u8;
        let g = (255.0 * (y as f32 / IMAGE_HEIGHT as f32)) as u8;
        let b = 0;

        *pixel = Rgb([r, g, b]);
    }

    frame_buffer.save("out.png").unwrap();
}

fn main() {
    println!("Hello, world!");
    render();
}
