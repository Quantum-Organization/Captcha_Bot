use image::{self, Rgb, RgbImage};
use imageproc::{
    drawing::{draw_filled_rect, draw_line_segment_mut, draw_text_mut},
    rect::Rect,
};
use rusttype::{Font, Scale};
use std::io::Cursor;

pub fn draw_text(code: &String) -> Vec<u8> {
    let mut base = RgbImage::new(200, 100);

    let mut image = draw_filled_rect(
        &mut base,
        Rect::at(0, 0).of_size(200, 100),
        Rgb([94, 99, 221]),
    );

    let font_scale = 39;
    let font = Vec::from(include_bytes!("../assets/Stroke.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let scale = Scale {
        x: font_scale as f32,
        y: font_scale as f32,
    };


    let mut last_pos = None;

    code.chars().enumerate().for_each(|(i, c)| {
        let random_x = rand::random::<u32>() % 30;
        let random_y = rand::random::<u32>() % 10;
        let x = random_x as i32 + (i * 30) as i32;
        let y = random_y as i32 + (i * 10) as i32;

        draw_text_mut(
            &mut image,
            Rgb([255, 255, 255]),
            x,
            y,
            scale,
            &font,
            &c.to_string(),
        );

        if let Some((last_x, last_y)) = last_pos {
            draw_line_segment_mut(
                &mut image,
                (last_x as f32, last_y as f32 + font_scale as f32 / 2.0),
                (x as f32, y as f32 + font_scale as f32 / 2.0),
                Rgb([255, 255, 255]),
            );

            draw_line_segment_mut(
                &mut image,
                (last_x as f32, last_y as f32 + font_scale as f32 / 2.0 + 1.0),
                (x as f32, y as f32 + font_scale as f32 / 2.0 + 1.0),
                Rgb([255, 255, 255]),
            );

            draw_line_segment_mut(
                &mut image,
                (last_x as f32, last_y as f32 + font_scale as f32 / 2.0 - 1.0),
                (x as f32, y as f32 + font_scale as f32 / 2.0 - 1.0),
                Rgb([255, 255, 255]),
            );
        }

        last_pos = Some((x, y));
    });

    for _ in 0..1000 {
        let x = rand::random::<u32>() % 200;
        let y = rand::random::<u32>() % 100;

        let color = Rgb([
            rand::random::<u8>(),
            rand::random::<u8>(),
            rand::random::<u8>(),
        ]);
        image.put_pixel(x, y, color);
    }

    let mut bytes = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
        .unwrap();

    bytes
}
