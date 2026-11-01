#![no_main]
#![no_std]

extern crate alloc;
use alloc::vec;
use core::fmt::Write;

const SCREEN_WIDTH: usize = 256;
const SCREEN_HEIGHT: usize = 256;
const SCREEN_DIMENSION: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

const THETA_SPACING: f64 = 0.07;
const PHI_SPACING: f64 = 0.02;
const R1: f64 = 1.0;
const R2: f64 = 2.0;
const K2: f64 = 5.0;
const K1: f64 = (SCREEN_WIDTH as f64) * K2 * 3.0 / (8.0 * (R1 + R2));

fn render_frame(a: f64, b: f64) {
    uefi::system::with_stdout(|screen| {
        let cos_a = libm::cos(a);
        let cos_b = libm::cos(b);
        let sin_a = libm::sin(a);
        let sin_b = libm::sin(b);

        let mut output = vec![vec![' '; SCREEN_HEIGHT]; SCREEN_WIDTH];
        let mut zbuffer = vec![vec![0.0; SCREEN_HEIGHT]; SCREEN_WIDTH];

        let theta_step = core::iter::successors(Some(0.0), |&theta| {
            let end = 2.0 * core::f64::consts::PI;
            if theta < end {
                Some(theta + THETA_SPACING)
            } else {
                None
            }
        });

        for theta in theta_step {
            let cos_theta = libm::cos(theta);
            let sin_theta = libm::sin(theta);

            let phi_step = core::iter::successors(Some(0.0), |&phi| {
                let end = 2.0 * core::f64::consts::PI;
                if phi < end {
                    Some(phi + THETA_SPACING)
                } else {
                    None
                }
            });

            for phi in phi_step {
                let cos_phi = libm::cos(phi);
                let sin_phi = libm::sin(phi);

                let circle_x = R2 + R1 * cos_theta;
                let circle_y = R1 * sin_theta;

                let x = circle_x * (cos_b * cos_phi + sin_a * sin_b * sin_phi)
                    - circle_y * cos_a * sin_b;
                let y = circle_x * (sin_b * cos_phi - sin_a * cos_b * sin_phi)
                    + circle_y * cos_a * cos_b;
                let z = K2 + cos_a * circle_x * sin_phi + circle_y * sin_a;
                let ooz = 1.0 / z;

                let xp = SCREEN_WIDTH as f64 / 2.0 + K1 * ooz * x;
                let yp = SCREEN_HEIGHT as f64 / 2.0 - K1 * ooz * y;

                let l =
                    cos_phi * cos_theta * sin_b - cos_a * cos_theta * sin_phi - sin_a * sin_theta
                        + cos_b * (cos_a * sin_theta - cos_theta * sin_a * sin_phi);
                if l > 0.0 {
                    if ooz > zbuffer[xp as usize][yp as usize] {
                        zbuffer[xp as usize][yp as usize] = ooz;
                        let luminance_index = l * 8.0;

                        output[xp as usize][yp as usize] =
                            b".,-~:;=!*#$@"[luminance_index as usize] as char;
                    }
                }
            }
        }

        screen.set_cursor_position(0, 0);
        for j in 0..SCREEN_HEIGHT {
            for i in 0..SCREEN_WIDTH {
                screen.write_char(output[i][j]);
            }

            screen.write_char('\n');
        }
    });
}

#[uefi::entry]
fn main() -> uefi::Status {
    uefi::helpers::init().unwrap();

    loop {
        if let Ok(Some(key)) = uefi::system::with_stdin(|screen| screen.read_key()) {
            if key
                == uefi::proto::console::text::Key::Special(
                    uefi::proto::console::text::ScanCode::ESCAPE,
                )
            {
                break;
            }
        }

        render_frame(1.0, 1.0);
    }

    uefi::Status::SUCCESS
}
