use crate::color::Color;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
pub struct Canvas {
    pub width: i32,
    pub height: i32,
    pub pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: i32, height: i32, default_color: Color) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![default_color; (height * width) as usize],
        }
    }

    pub fn write_pixel(mut self, x: i32, y: i32, c: Color) -> Self {
        let idx = self.i(x, y);
        self.pixels[idx] = c;
        self
    }

    pub fn pixel_at(&self, x: i32, y: i32) -> Color {
        self.pixels[self.i(x, y)]
    }

    pub fn to_ppm(&self) -> String {
        let header = format!("P3\n{} {}\n255", self.width, self.height);

        let mut body: String = String::new();
        let mut current_line_length = 0;

        for color_row in self.pixels.chunks(self.width as usize) {
            for color in color_row {
                for channel in color.de_normalized() {
                    let as_str = format!("{}", channel);

                    if current_line_length + as_str.len() >= 70 {
                        body.remove(body.len() - 1);
                        body.push('\n');
                        body.push_str(&as_str);
                        body.push(' ');

                        current_line_length = as_str.len() + 1;
                    } else if current_line_length + as_str.len() == 70 {
                        body.push_str(&as_str);
                        current_line_length = 70
                    } else {
                        body.push_str(&as_str);
                        body.push(' ');
                        current_line_length += as_str.len() + 1;
                    }
                }
            }

            body.remove(body.len() - 1);
            body.push('\n');
            current_line_length = 0;
        }

        format!("{}\n{}", header, body)
    }

    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        let filename = path;
        let content = self.to_ppm();
        let mut file = File::create(filename)?;
        file.write_all(content.as_bytes())
    }

    pub fn length(&self) -> usize {
        self.pixels.len()
    }

    fn i(&self, x: i32, y: i32) -> usize {
        ((self.width * y) + x) as usize
    }
}

#[cfg(test)]
mod canvas_test {
    use crate::canvas::Canvas;
    use crate::color::{black, color};

    #[test]
    fn create() {
        let canvas = Canvas::new(10, 20, black());

        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
        assert_eq!(canvas.pixels.len(), 200);
        assert!(canvas.pixels.into_iter().all(|pixel| pixel == black()));
    }

    #[test]
    fn write_pixel() {
        let mut canvas = Canvas::new(10, 20, black());
        let red = color(1.0, 0.0, 0.0);

        canvas = canvas.write_pixel(2, 3, red);

        assert_eq!(canvas.pixel_at(2, 3), red);
    }

    #[test]
    fn ppm_header() {
        let canvas = Canvas::new(5, 3, black());

        assert_eq!(
            canvas
                .to_ppm()
                .lines()
                .take(3)
                .collect::<Vec<&str>>()
                .join("\n"),
            "P3\n5 3\n255"
        )
    }

    fn get_ppm_body(ppm: &str) -> String {
        ppm.lines().skip(3).collect::<Vec<&str>>().join("\n")
    }

    #[test]
    fn ppm_body() {
        let mut canvas = Canvas::new(5, 3, black());
        println!("{}", canvas.length());

        let c1 = color(1.5, 0.0, 0.0);
        let c2 = color(0.0, 0.5, 0.0);
        let c3 = color(-0.5, 0.0, 1.0);

        canvas = canvas.write_pixel(0, 0, c1);
        canvas = canvas.write_pixel(2, 1, c2);
        canvas = canvas.write_pixel(4, 2, c3);

        assert_eq!(
            get_ppm_body(&canvas.to_ppm()),
            "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"
        );
    }

    #[test]
    fn splitting_long_lines() {
        let canvas = Canvas::new(10, 2, color(1.0, 0.8, 0.6));

        assert_eq!(
            get_ppm_body(&canvas.to_ppm()),
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
    }

    #[test]
    fn ends_with_newline() {
        assert_eq!(
            Canvas::new(5, 3, black()).to_ppm().chars().last().unwrap(),
            '\n'
        );
    }
}

