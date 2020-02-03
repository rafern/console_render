/// A color struct
#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Get the manhattan distance between two colors
    pub fn dist(&self, color: &Color) -> u16 {
        ((color.r as i16 - self.r as i16).abs() + (color.g as i16 - self.g as i16).abs() + (color.b as i16 - self.b as i16).abs()) as u16
    }
} 
