use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

trait RgbChannels {
    fn r(&self) -> u8;

    fn g(&self) -> u8;

    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    fn r(&self) -> u8 { self.r }
    fn g(&self) -> u8 { self.g }
    fn b(&self) -> u8 { self.b }
}

#[derive(PartialEq, Debug, Clone)]
enum RgbErr {
    NoLeadingHash,
    InvalidLength,
    NotHexCode,
}

fn from_hex(s: &str) -> Result<u8, RgbErr> {
    u8::from_str_radix(s, 16).map_err(|_| RgbErr::NotHexCode)
}

impl FromStr for Rgb {
    type Err = RgbErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if &s[0..1] != "#" {
            return Err(RgbErr::NoLeadingHash);
        }
        if s.len() != 7 {
            return Err(RgbErr::InvalidLength);
        }
        let r: u8 = from_hex(&s[1..3])?;
        let g: u8 = from_hex(&s[3..5])?;
        let b: u8 = from_hex(&s[5..7])?;
        Ok(Rgb{ r, g, b })
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

fn main() {
    println!("RGB: {}", "#aabbcc".parse::<Rgb>().unwrap());
}

#[test]
fn every_color() {
    let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

    for ((r, g), b) in colors {
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        let color: Rgb = hex.parse().unwrap();
        assert_eq!(hex, format!("{}", color));
    }
}

#[test]
fn too_short() {
    let result: Result<Rgb, _> = "#1234".parse();
    assert_eq!(result, Err(RgbErr::InvalidLength));
}

#[test]
fn not_a_hex_code() {
    let result: Result<Rgb, _> = "#0000?0".parse();
    assert_eq!(result, Err(RgbErr::NotHexCode));
}

#[test]
fn invalid_literals() {
    let result: Result<Rgb, _> = "#0000?0".parse();
    assert_eq!(result, Err(RgbErr::NotHexCode));
}

#[test]
fn no_leading_hash() {
    let result: Result<Rgb, _> = "aabbcc".parse();
    assert_eq!(result, Err(RgbErr::NoLeadingHash));
}

#[test]
fn out_of_bounds() {
    let result: Result<Rgb, _> = "#00gg00".parse();
    assert_eq!(result, Err(RgbErr::NotHexCode));
}

