use std::f64::consts::PI;
use std::time::Duration;

use std::thread;

use julia_set::{JuliaSet, RenderConfig};
use num::{Complex, complex};

fn main() {
    let render_config = RenderConfig {
        x_min: -1.5,
        x_max: 1.5,
        y_min: -1.5,
        y_max: 1.5,
        width: 100,
        height: 36,
    };

    let angle_range: Vec<_> = (0..300).map(|n| (n as f64 / 300.0) * 2.0 * PI).collect();
    let mut julia_set = JuliaSet::new(1000, Complex::ZERO, render_config);

    for (i, angle) in angle_range.into_iter().enumerate() {
        let c = 0.7885 * complex::c64(angle.cos(), angle.sin());
        julia_set.update_c(c);

        println!("\x1B[2J\x1B[1;1H");
        println!("Frame: {}, c = {:.4} + {:.4}i", i, c.re, c.im);
        julia_set.render();

        thread::sleep(Duration::from_millis(50));
    }
}
