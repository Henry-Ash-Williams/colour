use regex::Regex; 

use rand::{
    distributions::{Distribution, Standard},
    Rng
};

use std::{
    fmt,
    error::Error,
    convert::TryInto
}; 

#[derive(Debug, Copy, Clone)] 
pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r, 
            g,
            b,
        }
    }

    pub fn random() -> Self {
        rand::random() 
    }

    pub fn default() -> Self {
        Self::new(0, 0, 0)
    }

    pub fn from_hex_string<S: AsRef<str>>(hex_string: S) -> Self {
        let re = Regex::new("^#[A-Fa-f0-9]{6}").unwrap(); 
        let hex_string: &str = hex_string.as_ref(); 

        if !re.is_match(hex_string) {
            panic!("invalid hex string")
        }

        RGB::new(
            u8::from_str_radix(&hex_string[1..3], 16).unwrap(),
            u8::from_str_radix(&hex_string[3..5], 16).unwrap(),
            u8::from_str_radix(&hex_string[5..7], 16).unwrap()
        )
    }

    pub fn blend(&self, other: Self, alpha: f64, beta: f64) -> Self {
        asser_eq!(alpha + beta, 1f64); 
        *self * alpha + other * beta
    }

    pub fn to_tuple(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b) 
    }
}

impl std::ops::Add<RGB> for RGB {
    type Output = RGB; 
    fn add(self, other: Self) -> Self::Output {
        Self {
            r: self.r + other.r, 
            g: self.g + other.g, 
            b: self.b + other.b, 
        }
    }
}

impl std::ops::Mul<f64> for RGB {
    type Output = RGB; 
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            r: (self.r as f64 * rhs) as u8, 
            g: (self.g as f64 * rhs) as u8, 
            b: (self.b as f64 * rhs) as u8, 
        }
    }
}

impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl Distribution<RGB> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RGB {
        RGB { 
            r: rng.gen_range(0..=255),
            g: rng.gen_range(0..=255),
            b: rng.gen_range(0..=255),
        }
    }
}

pub struct Gradient {
    gradient: Vec<RGB>,
    start: RGB,
    end: RGB, 
    steps: usize, 
}

impl Gradient {
    pub fn new(start: RGB, end: RGB, steps: usize) -> Self {
        Self {
            gradient: Self::generate_gradient(start, end, steps),
            start,
            end, 
            steps, 
        }
    }
    
    pub fn generate_gradient(start: RGB, end: RGB, steps: usize) -> Vec<RGB> {
        let mut gradient = vec![RGB::default(); steps]; 

        for (idx, c) in gradient.iter_mut().enumerate() {
            let a: f64 = idx as f64 / steps as f64; 
            let b: f64 = (steps - idx) as f64 / steps as f64; 
            *c = start.blend(end, a, b); 
            println!("{:.3} * {} + {:.3} * {} = {}", a, start, b, end, c);  
        }

        gradient 
    }

    pub fn generate_image<S: AsRef<str>>(&self, filename: S) -> Result<(), Box<dyn Error>> {
        let mut img_buf = image::ImageBuffer::new(
            600, 
            self.steps.try_into()?
        ); 

        for (row_idx, row) in img_buf.enumerate_rows_mut() {
            for p in row {
                let (r, g, b) = self.gradient[row_idx as usize].to_tuple(); 
                *p.2 = image::Rgb([r, g, b]); 
            }
        }

        img_buf.save(filename.as_ref())?; 

        Ok(())
    }
}

impl IntoIterator for Gradient {
    type Item = RGB; 
    type IntoIter = std::vec::IntoIter<Self::Item>; 

    fn into_iter(self) -> Self::IntoIter {
        self.gradient.into_iter() 
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let a = RGB::random(); 
    let b = RGB::random();
    let gradient = Gradient::new(a, b, 1024); 
    gradient.generate_image("gradient.png")?; 
    Ok(())
}
