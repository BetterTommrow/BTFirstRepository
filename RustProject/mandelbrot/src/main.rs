use num::Complex;
use std::str::FromStr;

use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;

use std::env;
use num_cpus;

fn main() {
    // 获取自己电脑的核心数
    let available_cores = num_cpus::get();
    println!("Available cores: {}", available_cores);

    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprint!("Usage: {} File pixels Upper_left Lower_right", args[0]);
        eprint!("Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20", args[0]);
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x')
        .expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3])
        .expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4])
        .expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    render(&mut pixels, bounds, upper_left, lower_right);

    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}

/// 把`pixel`缓冲区写入名为`filename`的文件中
fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;

    Ok(())
}

fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>
) {
    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;

    {
        let bands: Vec<&mut [u8]> =
            pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;  // 每个 band 的高度应该是 rows_per_band
                let band_bounds = (bounds.0, height);
                let band_upper_left =
                    pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right =
                    pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

                spawner.spawn(move |_| {
                    for row in 0..height {
                        for col in 0..bounds.0 {
                            let point = pixel_to_point(band_bounds, (col, row), band_upper_left, band_lower_right);
                            band[row * bounds.0 + col] = match escape_time(point, 255) {
                                None => 0,
                                Some(count) => 255 - count as u8,
                            };
                        }
                    }
                });
            }
        }).unwrap();
    }
}



fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0., im: 0. };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im
    );
    Complex {
        re: upper_left.re + (pixel.0 as f64 * width) / (bounds.0 as f64),
        im: upper_left.im - (pixel.1 as f64 * height) / (bounds.1 as f64),
    }
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

// fn square_loop(mut x: f64) {
//     loop {
//         x = x * x;
//     }
// }

// fn square_add_loop(c: f64) {
//     let mut x = 0.;
//     loop {
//         x = x * x + c;
//     }
// }

// fn complex_square_add_loop(c: Complex<f64>) {
//     let mut z = Complex { re: 0., im: 0. };
//     loop {
//         z = z * z + c;
//     }
// }

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point((100, 200), (25, 175), Complex { re: -1.0, im: 1.0 }, Complex { re: 1.0, im: -1.0 }),
        Complex { re: -0.5, im: -0.75 }
    );
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex { re: 1.25, im: -0.0625 }));
    assert_eq!(parse_complex(",-0.0625"), None);
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
}