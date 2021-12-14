use num::Complex;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    return None;
}

fn parse_pair<T: FromStr>(s: &str, seperator: char) -> Option<(T, T)> {
    match s.find(seperator) {
        None => None,
        Some(index) =>{
            match (T::from_str(&s[..index]), T::from_str(&s[index+1..])){
                (Ok(l), Ok(r))=> Some((l, r)),
                _=> None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10,20)));
    assert_eq!(parse_pair::<i32>("10,20xz", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair("0.5x1.5", 'x'), Some((0.5, 1.5)));

}

/// Parse a pair of floating-point numbers seperated by a comma
/// as a complex number.
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    parse_pair(s, ',').map(|(re, im)| {Complex{re, im} })
}

#[test]
fn test_parse_complex(){
    assert_eq!(parse_complex(""), None);
    assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex{re: 1.25, im: -0.0625}));
}

/// Given a row and column of a pixel in the output image, return the corresponding point
/// on the complex plane.
/// 
/// 'bounds' is a pair giving the width and height of the image in pixels.
/// 'pixel is a (column, row) pair indicating a particular pixel in that image.
/// the 'upper_left' and 'lower_right' parameters are points on the complex plane
/// designating the area our image covers.
fn pixel_to_point(
        bounds: (usize, usize),
        pixel: (usize, usize),
        upper_left: Complex<f64>,
        lower_right: Complex<f64>
) -> Complex<f64> {
    let (width, height) =  (lower_right.re - upper_left.re,
                                    upper_left.im - lower_right.im);
                                
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        // Subtract for imaginary because pixel.1 increases as we go down,
        // but the imaginary component increases as we go up.
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
    }
}