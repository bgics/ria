use num::complex::{self, Complex};

pub struct RenderConfig {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub width: usize,
    pub height: usize,
}

pub struct JuliaSet {
    buffer: Vec<Vec<usize>>,
    max_iters: usize,
    render_config: RenderConfig,
    params: JuliaSetParams,
}

impl JuliaSet {
    pub fn new(max_iters: usize, c: Complex<f64>, render_config: RenderConfig) -> JuliaSet {
        let params = JuliaSetParams::new(c);
        let mut julia_set = JuliaSet {
            buffer: vec![],
            max_iters,
            render_config,
            params,
        };
        julia_set.compute();
        julia_set
    }

    pub fn render(&self) {
        for row in &self.buffer {
            let mut line = String::with_capacity(row.len());
            for val in row {
                let point = match val {
                    0..2 => ' ',
                    2..5 => '.',
                    5..10 => '•',
                    10..30 => '*',
                    30..100 => '+',
                    100..200 => 'x',
                    200..400 => '$',
                    400..700 => '#',
                    _ => '%',
                };
                line.push(point);
            }
            println!("{}", line);
        }
    }

    pub fn update_c(&mut self, c: Complex<f64>) {
        self.params = JuliaSetParams::new(c);
        self.compute();
    }

    fn compute(&mut self) {
        let RenderConfig {
            x_min,
            x_max,
            y_min,
            y_max,
            width,
            height,
        } = self.render_config;

        let mut buffer = Vec::with_capacity(height);

        for img_y in 0..height {
            let mut row = Vec::with_capacity(width);

            for img_x in 0..width {
                let x_percent = img_x as f64 / width as f64;
                let y_percent = img_y as f64 / height as f64;

                let cx = x_min + (x_max - x_min) * x_percent;
                let cy = y_min + (y_max - y_min) * y_percent;

                let escaped_at = self.escape_time(complex::c64(cx, cy));

                row.push(escaped_at);
            }

            buffer.push(row);
        }

        self.buffer = buffer;
    }

    fn escape_time(&self, z0: Complex<f64>) -> usize {
        let mut z = z0;
        let r_sqr = self.params.r_sqr();
        let c = self.params.c();

        for i in 0..self.max_iters {
            if z.norm_sqr() > r_sqr {
                return i;
            }
            z = z * z + c
        }
        self.max_iters
    }
}

struct JuliaSetParams {
    c: Complex<f64>,
    r_sqr: f64,
}

impl JuliaSetParams {
    fn new(c: Complex<f64>) -> JuliaSetParams {
        let r = ((1.0 + (1.0 + 4.0 * c.norm()).sqrt()) / 2.0).ceil();
        let r_sqr = r * r;
        JuliaSetParams { c, r_sqr }
    }

    fn r_sqr(&self) -> f64 {
        self.r_sqr
    }

    fn c(&self) -> Complex<f64> {
        self.c
    }
}
