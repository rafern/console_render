use std::ops::Add;
use std::ops::Sub;
use std::f64::consts::PI;

/// A 2D point
#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Add for Point {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;
    
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Point {
    pub fn from_normal(angle: f64, length: f64) -> Point {
        let a = angle.rem_euclid(PI * 2.0);
        
        if a >= 1.5 * PI {
            let t = a - 1.5 * PI;
            return Point{x: -length * t.cos(), y: length * t.sin()};
        }
        else if a >= PI {
            let t = a - PI;
            return Point{x: -length * t.sin(), y: -length * t.cos()};
        }
        else if a >= 0.5 * PI {
            let t = a - 0.5 * PI;
            return Point{x: length * t.cos(), y: -length * t.sin()};
        }
        else {
            return Point{x: length * a.sin(), y: length * a.cos()};
        }
    }
    
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    pub fn dot(self, other: Point) -> f64 {
        self.x * other.x + self.y * other.y
    }
    
    pub fn cross(self, other: Point) -> f64 {
        self.x * other.y - self.y * other.x
    }
    
    pub fn scale(self, factor: f64) -> Point {
        Point {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
    
    pub fn normal(self) -> Point {
        self.scale(1.0 / self.magnitude())
    }
}

/// A 2D line
#[derive(Copy, Clone, Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn from_angle(start: Point, length: f64, angle: f64) -> Line {
        Line{start: start, end: start + Point::from_normal(angle, length)}
    }
    
    pub fn intersection(&self, other: Line) -> Option<Point> {
        let this_gradient = self.end - self.start;
        let other_gradient = other.end - other.start;
        let cross_gradient = this_gradient.cross(other_gradient);
        
        if cross_gradient == 0.0 {
            return None;
            // TODO collinear lines maybe?
        }
        
        let start_dists = other.start - self.start;
        let start_cross = start_dists.cross(this_gradient); // TODO wrong formula?
        let this_start_dist = start_dists.cross(other_gradient) / cross_gradient;
        let other_start_dist = start_dists.cross(this_gradient) / cross_gradient;
        if this_start_dist >= 0.0 && this_start_dist <= 1.0 && other_start_dist >= 0.0 && other_start_dist <= 1.0 {
            return Some(self.start + this_gradient.scale(this_start_dist));
        }
        
        return None;
    }

    pub fn length(&self) -> f64 {
        (self.start - self.end).magnitude()
    }
    
    pub fn normal(&self) -> Point {
        let delta = self.start - self.end;
        delta.normal()
    }
}
