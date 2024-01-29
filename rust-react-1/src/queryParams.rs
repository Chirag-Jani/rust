extern crate image;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ImageParams {
    pub width: u32,
    pub height: u32,
    pub cx: f32,
    pub cy: f32,
}

pub fn generate_image(params: &ImageParams) -> Vec<u8> {
    let mut ret = Vec::<u8>::new();
    let iterations = 280;

    for y in 0..params.height {
        for x in 0..params.width {
            let inner_height = params.height as f32;
            let inner_width = params.width as f32;
            let inner_y = y as f32;
            let inner_x = x as f32;

            let mut zx = (3.0 * (inner_x - 0.5 * inner_width)) / inner_width;
            let mut zy = (2.0 * (inner_y - 0.5 * inner_height)) / inner_height;

            let mut i = iterations;

            while zx * zx + zy * zy < 4.0 && i > 1 {
                let tmp = zx * zx - zy * zy + params.cx;
                zy = 2.0 * zx * zy + params.cy;
                zx = tmp;
                i -= 1;
            }

            // guesswork to make the rgb color values look okay
            // you can play with the bit shifting to change colors
            // but iterations also affect that
            ret.push((i << 1) as u8);
            ret.push((i << 2) as u8);
            ret.push((i << 3) as u8);
            ret.push(255);
        }
    }

    ret
}
