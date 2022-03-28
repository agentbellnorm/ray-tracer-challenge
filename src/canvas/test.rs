#[cfg(test)]
mod canvas_test {
    use crate::canvas::Canvas;
    use crate::color::{color, Color};

    #[test]
    fn create() {
        let canvas = Canvas::new(10, 20, Color::black());

        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
        assert_eq!(canvas.pixels.len(), 200);
        assert!(canvas
            .pixels
            .into_iter()
            .all(|pixel| pixel == Color::black()));
    }

    #[test]
    fn write_pixel() {
        let mut canvas = Canvas::new(10, 20, Color::black());
        let red = color(1.0, 0.0, 0.0);

        canvas = canvas.write_pixel(2, 3, red);

        assert_eq!(canvas.pixel_at(2, 3), red);
    }

    #[test]
    fn ppm_header() {
        let canvas = Canvas::new(5, 3, Color::black());

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
        let mut canvas = Canvas::new(5, 3, Color::black());
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
            Canvas::new(5, 3, Color::black())
                .to_ppm()
                .chars()
                .last()
                .unwrap(),
            '\n'
        );
    }
}
