extern crate num;
extern crate image;
use num::Complex;
use std::str::FromStr;
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;
use std::io::Write;

/// This is used by rustdoc to generate documents
fn escape_time(c: Complex<f64>, limit:u32) -> Option<u32> {
  let mut z = Complex {re:0.0, im:0.0};
  for i in 0..limit {
    z = z * z+c;
    if z.norm_sqr() > 4.0 {
      return Some(i); //explcit return
    }
  }
  None //implicit return the last statement
}


///function template but only works for T which implements FromStr traits
fn parse_pair<T: FromStr>(s: &str, separator:char) -> Option<(T,T)> {
  match s.find(separator) {
    None => None,
    Some(index) => {
      match (T::from_str(&s[..index]), T::from_str(&s[index+1..])) {
        (Ok(l),Ok(r)) => Some((l,r)),
          _ => None
      }
    }
  }
}

///parse a pair of floating point numbers separated by a comma as a complex number
fn parse_complex(s: &str) -> Option<Complex<f64>> {
  match parse_pair(s, ',') {
    Some((re,im)) => Some(Complex {re,im}),
    None => None
  }
}

#[test]
fn test_parse_complex() {
  assert_eq!(parse_complex("1.25, -.0625"),
    Some(Complex {re: 1.25, im: -0.0625}));
  assert_eq!(parse_complex(",-0.0625"), None);
}

///Given the row and column of a pixel in the output image, return the corresponding
/// point on the complex plane
fn pixel_to_point(bounds: (usize,usize),
  pixel: (usize,usize),
  upper_left: Complex<f64>,
  lower_right: Complex<f64>) -> Complex<f64> {

  let (width,height) = (lower_right.re - upper_left.re,
    upper_left.im - lower_right.im);

  //Access the elements of a tuple using tuple.0 convention
  //Unlike C, C#, implict conversion is generally not allowed
  Complex {
    re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
    im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
  }
}

#[test]
fn test_pixel_to_point() {
  assert_eq!(pixel_to_point((100,100), (25,75),
    Complex {re:-1.0, im:1.0},
    Complex {re:1.0, im:-1.0}), Complex(re:-0.5, im:-0.5));
}


///Render given image to array of u8
fn render(pixels: &mut [u8], // pixels is a reference to type of mutable u7 array
  bounds: (usize,usize), //bounds is a tuple of (unsigned machine word size, unsigned machine word size)
  upper_left: Complex<f64>,
  lower_right: Complex<f64>) {

  assert!(pixels.len() == bounds.0 * bounds.1);

  for row in 0 .. bounds.1 {
    for column in 0 .. bounds.0 {
      let point = pixel_to_point(bounds, (column,row), upper_left, lower_right);
      pixels[row*bounds.0 + column] = match escape_time(point,255) {
        None=>0,
        Some(count) => 255 - count as u8
      }
    }
  }
}


///write the buffer pixels
/// return type is Result<() --> it's like void since it returns no useful information

fn write_image(filename: &str, pixels: &[u8], bounds: (usize,usize))
-> Result<(), std::io::Error> {
  let output = File::create(filename)?;
  //? is a short-cut for checking the result. return Error if Err(e) is returned.
  let encoder = PNGEncoder::new(output);
  encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;
  Ok(())
}

fn main() {
  let args: Vec<String> = std::env::args().collect();

  if args.len() != 5 {
    writeln!(std::io::stderr(),"Usage: mandelbrot FILE PIXELS UPPERLEFT LOWERRIGHT").unwrap();
    std::process::exit(1);
  }

  let bounds = parse_pair(&args[2],'x').expect("error parsing image dimensions");
  let upper_left = parse_complex(&args[3]).expect("error parsing left corner");
  let lower_right = parse_complex(&args[4]).expect("error parsing lower right");

  let mut pixels = vec![0;bounds.0 * bounds.1];

  render(&mut pixels,bounds,upper_left, lower_right);

  write_image(&args[1], &pixels, bounds).expect("error while writing image");

}
