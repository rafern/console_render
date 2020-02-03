use super::color::Color;
use super::csi_color::{CSIColor, CSI_FG, CSI_BG};
use std::string::String;
use std::vec::Vec;

/// A framebuffer. Stores frame data such as dimensions and cells (colors
/// and characters for each console cell)
#[derive(Debug)]
pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    chars: Vec<char>,
    fg_colors: Vec<u8>,
    bg_colors: Vec<u8>,
}

impl Framebuffer {
    /// Create a new framebuffer
    pub fn new(width: usize, height: usize) -> Framebuffer {
        let chars_cap = width * height;
        let colors_cap = chars_cap * 3;
    
        let mut fb = Framebuffer {
            width: width,
            height: height,
            chars: Vec::with_capacity(chars_cap),
            fg_colors: Vec::with_capacity(colors_cap),
            bg_colors: Vec::with_capacity(colors_cap),
        };
        
        fb.chars.resize(chars_cap, ' ');
        fb.fg_colors.resize(colors_cap, 0);
        fb.bg_colors.resize(colors_cap, 0);
        
        fb
    }
    
    /// Clear the framebuffer with a character and foreground/background color
    pub fn clear(&mut self, clear_char: char, clear_fg_color: Color, clear_bg_color: Color) {
        let (mut char_pos, mut color_pos, buffer_size) = (0usize, 0usize, self.width * self.height);
        
        while char_pos < buffer_size {
            // Clear character
            self.chars[char_pos] = clear_char;
            
            // Clear foreground color
            self.fg_colors[color_pos    ] = clear_fg_color.r;
            self.fg_colors[color_pos + 1] = clear_fg_color.g;
            self.fg_colors[color_pos + 2] = clear_fg_color.b;
            
            // Clear background color
            self.bg_colors[color_pos    ] = clear_bg_color.r;
            self.bg_colors[color_pos + 1] = clear_bg_color.g;
            self.bg_colors[color_pos + 2] = clear_bg_color.b;
            
            // color_pos could just be char_pos * 3, but thats less efficient
            char_pos += 1;
            color_pos += 3;
        }
    }
    
    /// Set a cell's character, foreground and or background colors
    pub fn set_cell(&mut self, x: usize, y: usize, character: Option<char>, fg_color: Option<Color>, bg_color: Option<Color>) {
        // Abort if out of bounds
        if x >= self.width || y >= self.height {
            return;
        }
    
        let char_pos = y * self.width + x;
        let color_pos = char_pos * 3;
        
        // Set character
        match character {
            Some(character) => self.chars[char_pos] = character,
            None => {},
        };
        
        // Set foreground color
        match fg_color {
            Some(color) => {
                self.fg_colors[color_pos    ] = color.r;
                self.fg_colors[color_pos + 1] = color.g;
                self.fg_colors[color_pos + 2] = color.b;
            },
            None => {},
        };
        
        // Set background color
        match bg_color {
            Some(color) => {
                self.bg_colors[color_pos    ] = color.r;
                self.bg_colors[color_pos + 1] = color.g;
                self.bg_colors[color_pos + 2] = color.b;
            },
            None => {},
        };
    }
    
    /// Moves console cursor
    fn console_cursor(buf: &mut Vec<u8>, x: usize, y: usize) {
        buf.extend_from_slice(b"\x1b[");
        buf.extend_from_slice((y + 1).to_string().as_bytes());
        buf.extend_from_slice(b";");
        buf.extend_from_slice((x + 1).to_string().as_bytes());
        buf.extend_from_slice(b"H");
    }
    
    /// Clears the console
    fn console_clear(buf: &mut Vec<u8>) {
        buf.extend_from_slice(b"\x1b[2J");
    }
    
    /// Pick nearest color from array. Returns index, not code
    fn pick_nearest_csi_color(colors: &[CSIColor; 16], target: &Color) -> isize {
        let mut picked:isize = 0;
        let mut picked_dist:u16 = colors[0].color.dist(target);
        
        for i in 1..16 {
            let dist = colors[i].color.dist(target);
            if dist < picked_dist {
                picked = i as isize;
                picked_dist = dist;
            }
        }
        
        picked
    }
    
    /// Sets the console back/foreground color
    fn console_color(buf: &mut Vec<u8>, color: &Color, csi_colors: &[CSIColor; 16], csi_last: &mut isize) {
        // TODO Windows?
        // Get nearest back/foreground color CSI code
        let this_code = Framebuffer::pick_nearest_csi_color(csi_colors, color);
        if *csi_last < 0 || this_code != *csi_last {
            // Set to nearest if different than last color
            *csi_last = this_code;
            buf.extend_from_slice(b"\x1b[");
            buf.extend_from_slice(&csi_colors[*csi_last as usize].code.to_string().as_bytes());
            buf.extend_from_slice(b"m");
        }
    }
    
    /// Resets console colors
    fn console_reset(buf: &mut Vec<u8>) {
        buf.extend_from_slice(b"\x1b[0m");
    }
    
    /// Stops console output
    fn console_xoff(buf: &mut Vec<u8>) {
        buf.push(0x13u8); // XOFF ASCII
    }
    
    /// Continues console output
    fn console_xon(buf: &mut Vec<u8>) {
        buf.push(0x11u8); // XON ASCII
    }
    
    /// Render a frame to a string
    pub fn get_string(&self) -> String {
        // Make buffer
        let mut buf:Vec<u8> = Vec::new();
        
        // Clear console
        Framebuffer::console_xoff(&mut buf);
        Framebuffer::console_reset(&mut buf);
        Framebuffer::console_clear(&mut buf);
        Framebuffer::console_cursor(&mut buf, 0, 0);
        
        // Print cells
        let mut char_pos = 0;
        let mut color_pos = 0;
        let mut last_fg: isize = -1;
        let mut last_bg: isize = -1;
        let mut utf8_buf = [0; 4];
        for y in 0..self.height {
            // Append line feed
            // TODO Windows?
            if y != 0 {
                buf.push(0x0au8); // LF ASCII
            }
            
            for _ in 0..self.width {
                // Set colors
                let this_fg = Color{
                    r: self.fg_colors[color_pos    ],
                    g: self.fg_colors[color_pos + 1],
                    b: self.fg_colors[color_pos + 2]
                };
                Framebuffer::console_color(&mut buf, &this_fg, &CSI_FG, &mut last_fg);
                
                let this_bg = Color{
                    r: self.bg_colors[color_pos    ],
                    g: self.bg_colors[color_pos + 1],
                    b: self.bg_colors[color_pos + 2]
                };
                Framebuffer::console_color(&mut buf, &this_bg, &CSI_BG, &mut last_bg);
                
                // Set character
                buf.extend_from_slice(self.chars[char_pos].encode_utf8(&mut utf8_buf).as_bytes());
                
                // Next positions in buffers. More efficient than y * width + x every iteration
                char_pos += 1;
                color_pos += 3;
            }
        }
        
        // Reset colors
        Framebuffer::console_xon(&mut buf);
        Framebuffer::console_reset(&mut buf);
        return String::from_utf8(buf).unwrap();
    }
}
