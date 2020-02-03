use super::framebuffer::Framebuffer;
use super::geometry::{Point, Line};
use std::f64::consts::PI;
use super::color::Color;
use std::string::String;

#[derive(Debug)]
pub struct TextureCell {
    pub character: Option<char>,
    pub fg_color: Option<Color>,
    pub bg_color: Option<Color>,
}

#[derive(Debug)]
pub struct Texture<'a> {
    pub cells: Vec<&'a TextureCell>,
    pub width: usize,
    pub height: usize,
    pub wrap: Point,
}

impl<'a> Texture<'a> {
    pub fn from_generator(width: usize, height: usize, wrap: Point, cell_generator: &dyn Fn(usize, usize) -> &'a TextureCell) -> Texture<'a> {
        let mut cells = vec![];
        for y in 0..height {
            for x in 0..width {
                cells.push(cell_generator(x, y));
            }
        }
        
        Texture{
            cells: cells,
            width: width,
            height: height,
            wrap: wrap,
        }
    }
    
    pub fn get_cell(&self, x: f64, y: f64) -> &TextureCell {
        let x_wrapped:usize = (x.rem_euclid(self.wrap.x) / self.wrap.x * self.width as f64).floor() as usize;
        let y_wrapped:usize = (y.rem_euclid(self.wrap.y) / self.wrap.y * self.height as f64).floor() as usize;
        &self.cells[y_wrapped * self.width + x_wrapped]
    }
}

#[derive(Debug)]
pub struct Wall<'a> {
    pub line: Line,
    pub texture: &'a Texture<'a>,
}

#[derive(Debug)]
pub struct World<'a> {
    pub framebuffer: Framebuffer,
    pub walls: Vec<Wall<'a>>,
    pub pos: Point,
    pub cam_rot: f64,
    pub cam_hfov: f64,
    pub cam_range: f64,
}

impl<'a> World<'a> {
    fn intersect_nearest(&self, angle: f64, cam_normal: Point) -> Option<(Point, &'a Texture<'a>, f64, f64)> {
        let cam_line = Line::from_angle(self.pos, self.cam_range, angle);
        
        let mut intersection:Option<(Point, &'a Texture, f64, f64)> = None;
        let mut best_dist:f64 = self.cam_range;
        for wall in &self.walls {
            let this_intersection = cam_line.intersection(wall.line);
            match this_intersection {
                Some(point) => {
                    let this_dist = (point - self.pos).magnitude();
                    if this_dist < best_dist {
                        let wall_normal = wall.line.normal();
                        let mut intersection_angle = cam_normal.dot(wall_normal).acos();
                        if intersection_angle > PI * 0.5 {
                            intersection_angle = PI - intersection_angle;
                        }
                        let intersection_dist = (point - wall.line.start).magnitude();
                        intersection = Some((point, wall.texture, intersection_angle, intersection_dist));
                        best_dist = this_dist;
                    }
                },
                None => {},
            }
        }
        
        return intersection;
    }

    pub fn render(&mut self) -> String {
        //self.framebuffer.clear(' ', Color{r: 255,g: 255,b: 255}, Color{r: 0,g: 0,b: 0});
        const SHADES:[char; 4] = ['\u{2593}', '\u{2592}', '\u{2591}', ' '];
        const SHADE_COLOR:Color = Color{r:0, g:0, b:0};
        let angle_start = self.cam_rot - self.cam_hfov * 0.5;
        let angle_step = self.cam_hfov / self.framebuffer.width as f64;
        let v_mid:usize = self.framebuffer.height / 2;
        let cam_normal = Point::from_normal(self.cam_rot, 1.0);
        for x in 0..self.framebuffer.width {
            let floor_h = self.framebuffer.height / 2;
            for y in 0..floor_h {
                self.framebuffer.set_cell(x, y, Some(' '), Some(Color{r: 255,g: 255,b: 255}), Some(Color{r: 100,g: 100,b: 100}));
            }
            for y in floor_h..self.framebuffer.height {
                self.framebuffer.set_cell(x, y, Some(' '), Some(Color{r: 255,g: 255,b: 255}), Some(Color{r: 127,g: 127,b: 0}));
            }
        
            let angle = angle_start + x as f64 * angle_step;
            let intersection = self.intersect_nearest(angle, cam_normal);
            
            match intersection {
                Some((point, texture, intersection_angle, intersection_dist)) => {
                    let dist = (point - self.pos).magnitude() * (self.cam_rot - angle).cos();
                    let range_percent = dist / self.cam_range;
                    
                    if range_percent >= 0.0 && range_percent < 1.0 {
                        let shade_char:char;
                        let angle_cos:f64 = intersection_angle.cos();
                        if intersection_angle > 1.30899694 {
                            // > 75 degrees
                            shade_char = ' ';
                        }
                        else if intersection_angle > 0.73303829 {
                            // > 42 degrees
                            shade_char = '\u{2591}';
                        }
                        else {
                            // <= 42 degrees
                            shade_char = '\u{2592}';
                        }
                        
                        // XXX I double tan'ed here to help with the warp on diagonal walls, but I have no idea why it works... huh
                        let wall_height:usize = ((1.0 - range_percent).tan().tan() * self.framebuffer.height as f64).floor() as usize;
                        if wall_height >= 1 {
                            let half_height = wall_height / 2;
                            let min;
                            if half_height > v_mid {
                                min = 0;
                            }
                            else {
                                min = v_mid - half_height;
                            }
                            let mut max = v_mid + wall_height - half_height;
                            if max > self.framebuffer.height {
                                max = self.framebuffer.height;
                            }
                            let min_f = v_mid as f64 - half_height as f64;
                            for h in min..max {
                                let texture_cell:&TextureCell = texture.get_cell(intersection_dist, ((h as f64 - min_f) / wall_height as f64) * 16.0);
                                self.framebuffer.set_cell(x, h, Some(shade_char), Some(Color{r:0,g:0,b:0}), texture_cell.bg_color);
                            }
                        }
                    }
                }
                None => {}
            }
        }
        
        self.framebuffer.get_string()
    }
}
