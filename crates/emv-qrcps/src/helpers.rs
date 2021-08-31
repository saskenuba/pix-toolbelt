use std::borrow::Cow;

use crc::{Algorithm, Crc, CRC_16_IBM_3740};

pub(crate) const CRC_ALGO: Algorithm<u16> = CRC_16_IBM_3740;

pub fn calculate_crc16(complete_pix: &str) -> u16 {
    let crc = Crc::<u16>::new(&CRC_ALGO);
    let mut digest = crc.digest();

    digest.update(complete_pix.as_bytes());
    digest.finalize()
}

pub fn finalize() {
    todo!()
}

pub fn validate(key: &str) -> bool {
    let total_size = key.len() - 4;
    let source_key_crc = &key[total_size..];

    let check = u16::from_str_radix(source_key_crc, 16).unwrap();
    let checked_string_without_crc = &key[..total_size];

    let crc = Crc::<u16>::new(&CRC_ALGO);
    let mut digest = crc.digest();

    digest.update(checked_string_without_crc.as_bytes());
    let final_crc = digest.finalize();
    check == final_crc
}

pub trait Encode {
    fn encode(&self) -> String;
}

pub trait Size {
    fn char_count(&self) -> i32;
}

impl Size for i32 {
    fn char_count(&self) -> i32 {
        ((*self as f32).log10() + 1.0).floor() as i32
    }
}

impl Size for &str {
    fn char_count(&self) -> i32 {
        self.len() as i32
    }
}

impl Size for Cow<'_, str> {
    fn char_count(&self) -> i32 {
        self.len() as i32
    }
}

impl Size for &Cow<'_, str> {
    fn char_count(&self) -> i32 {
        self.len() as i32
    }
}

impl Size for String {
    fn char_count(&self) -> i32 {
        self.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bacen_static_sample() -> &'static str {
        "00020126580014br.gov.bcb.pix0136123e4567-e12b-12d1-a456-4266554400005204000053039865802BR5913Fulano de \
         Tal6008BRASILIA62070503***63041D3D"
    }

    #[test]
    fn t_calculate_crc() {
        let sample = bacen_static_sample();
        let sample_size = bacen_static_sample().len();
        assert_eq!(format!("{:X}", calculate_crc16(&sample[..sample_size - 4])), "1D3D")
    }

    #[test]
    fn char_count_str() {
        let slice = "abcd";
        assert_eq!(4, slice.char_count());
    }

    #[test]
    fn char_count_numeric() {
        let number = 999;
        assert_eq!(3, number.char_count());
    }
}
