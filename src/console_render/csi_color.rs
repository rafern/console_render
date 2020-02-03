use super::color::Color;

/// A color used for CSI escape codes
#[derive(Debug)]
pub struct CSIColor {
    pub color: Color,
    pub code: u8,
}

pub const CSI_FG: [CSIColor; 16] = [
    CSIColor {
        color: Color{r: 12, g: 12, b: 12},
        code: 30,
    },
    CSIColor {
        color: Color{r: 197, g: 15, b: 31},
        code: 31,
    },
    CSIColor {
        color: Color{r: 19, g: 161, b: 14},
        code: 32,
    },
    CSIColor {
        color: Color{r: 193, g: 156, b: 0},
        code: 33,
    },
    CSIColor {
        color: Color{r: 0, g: 55, b: 218},
        code: 34,
    },
    CSIColor {
        color: Color{r: 136, g: 23, b: 152},
        code: 35,
    },
    CSIColor {
        color: Color{r: 58, g: 150, b: 221},
        code: 36,
    },
    CSIColor {
        color: Color{r: 204, g: 204, b: 204},
        code: 37,
    },
    CSIColor {
        color: Color{r: 118, g: 118, b: 118},
        code: 90,
    },
    CSIColor {
        color: Color{r: 231, g: 72, b: 86},
        code: 91,
    },
    CSIColor {
        color: Color{r: 22, g: 198, b: 12},
        code: 92,
    },
    CSIColor {
        color: Color{r: 249, g: 241, b: 165},
        code: 93,
    },
    CSIColor {
        color: Color{r: 59, g: 120, b: 255},
        code: 94,
    },
    CSIColor {
        color: Color{r: 180, g: 0, b: 158},
        code: 95,
    },
    CSIColor {
        color: Color{r: 97, g: 214, b: 214},
        code: 96,
    },
    CSIColor {
        color: Color{r: 242, g: 242, b: 242},
        code: 97,
    },
];

pub const CSI_BG: [CSIColor; 16] = [
    CSIColor {
        color: Color{r: 12, g: 12, b: 12},
        code: 40,
    },
    CSIColor {
        color: Color{r: 197, g: 15, b: 31},
        code: 41,
    },
    CSIColor {
        color: Color{r: 19, g: 161, b: 14},
        code: 42,
    },
    CSIColor {
        color: Color{r: 193, g: 156, b: 0},
        code: 43,
    },
    CSIColor {
        color: Color{r: 0, g: 55, b: 218},
        code: 44,
    },
    CSIColor {
        color: Color{r: 136, g: 23, b: 152},
        code: 45,
    },
    CSIColor {
        color: Color{r: 58, g: 150, b: 221},
        code: 46,
    },
    CSIColor {
        color: Color{r: 204, g: 204, b: 204},
        code: 47,
    },
    CSIColor {
        color: Color{r: 118, g: 118, b: 118},
        code: 100,
    },
    CSIColor {
        color: Color{r: 231, g: 72, b: 86},
        code: 101,
    },
    CSIColor {
        color: Color{r: 22, g: 198, b: 12},
        code: 102,
    },
    CSIColor {
        color: Color{r: 249, g: 241, b: 165},
        code: 103,
    },
    CSIColor {
        color: Color{r: 59, g: 120, b: 255},
        code: 104,
    },
    CSIColor {
        color: Color{r: 180, g: 0, b: 158},
        code: 105,
    },
    CSIColor {
        color: Color{r: 97, g: 214, b: 214},
        code: 106,
    },
    CSIColor {
        color: Color{r: 242, g: 242, b: 242},
        code: 107,
    },
];
