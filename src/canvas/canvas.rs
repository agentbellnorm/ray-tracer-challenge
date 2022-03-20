use crate::color::Color;

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

    pub fn pixel_at(self, x: i32, y: i32) -> Color {
        self.pixels[self.i(x, y)]
    }

    pub fn to_ppm(self) -> String {
        let header = format!("P3\n{} {}\n255", self.width, self.height);

        let mut body: String = String::new();
        let mut current_line_length = 0;

        for color_row in self.pixels.chunks(self.width as usize) {
            for color in color_row {
                for channel in color.de_normalized() {
                    let as_str = format!("{}", channel);

                    if current_line_length + as_str.len() >= 70 {
                        body.remove(body.len() - 1);
                        body.push_str("\n");
                        body.push_str(&as_str);
                        body.push_str(" ");

                        current_line_length = as_str.len() + 1;
                    } else if current_line_length + as_str.len() == 70 {
                        body.push_str(&as_str);
                        current_line_length = 70
                    } else {
                        body.push_str(&as_str);
                        body.push_str(" ");
                        current_line_length += as_str.len() + 1;
                    }
                }
            }

            body.remove(body.len() - 1);
            body.push_str("\n");
            current_line_length = 0;
        }

        format!("{}\n{}", header, body)
    }

    pub fn length(&self) -> usize {
        return self.pixels.len();
    }

    fn i(&self, x: i32, y: i32) -> usize {
        ((self.width * y) + x) as usize
    }
}
