mod console_render;
use console_render::framebuffer::Framebuffer;
use console_render::color::Color;
use console_render::geometry::{Line, Point};
use console_render::world::{Wall, World, Texture, TextureCell};
use std::f64::consts::PI;
use std::io::{self};

fn texture_cell_generator<'a>(x:usize, y:usize) -> &'a TextureCell {
    if y == 0 || y == 3 || (y < 3 && x == 5) || (y > 3 && x == 2) {
        return &TextureCell{character: None, fg_color: None, bg_color: Some(Color{r: 100, g: 100, b: 100})};
    }
    
    return &TextureCell{character: None, fg_color: None, bg_color: Some(Color{r: 255, g: 0, b: 0})};
}

fn main() {
    // Create world
    let texture = Texture::from_generator(6, 6, Point{x: 6.0, y: 6.0}, &texture_cell_generator);
    let mut world = World{
        framebuffer: Framebuffer::new(128, 48),
        walls: vec![
            Wall{
                line: Line{
                    start: Point{x: -10.0, y: 10.0},
                    end: Point{x: 10.0, y: 10.0},
                },
                texture: &texture,
            },
            Wall{
                line: Line{
                    start: Point{x: 10.0, y: 10.0},
                    end: Point{x: 10.0, y: -10.0},
                },
                texture: &texture,
            },
            Wall{
                line: Line{
                    start: Point{x: 10.0, y: -10.0},
                    end: Point{x: -10.0, y: -10.0},
                },
                texture: &texture,
            },
            Wall{
                line: Line{
                    start: Point{x: -10.0, y: -10.0},
                    end: Point{x: -10.0, y: 10.0},
                },
                texture: &texture,
            },
        ],
        pos: Point{x: 0.0, y: 0.0},
        cam_rot: 0.0,
        cam_hfov: PI * 0.5,
        cam_range: 30.0,
    };
    
    let mut running = true;
    while running {
        print!("{}\nWASD: move; QE: rotate camera; X: quit; Enter: next frame\n> ", world.render());
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                for c in input.chars() {
                    let lower_c = c.to_lowercase().next().unwrap();
                    if lower_c == 'w' {
                        world.pos = world.pos + Point::from_normal(world.cam_rot, 1.0);
                    }
                    else if lower_c == 's' {
                        world.pos = world.pos - Point::from_normal(world.cam_rot, 1.0);
                    }
                    else if lower_c == 'd' {
                        world.pos = world.pos + Point::from_normal(world.cam_rot + PI * 0.5, 1.0);
                    }
                    else if lower_c == 'a' {
                        world.pos = world.pos - Point::from_normal(world.cam_rot + PI * 0.5, 1.0);
                    }
                    else if lower_c == 'q' {
                        world.cam_rot -= PI / 32.0;
                    }
                    else if lower_c == 'e' {
                        world.cam_rot += PI / 32.0;
                    }
                    else if lower_c == 'x' {
                        running = false;
                    }
                }
            }
            Err(_) => running = false,
        }
    }
}
