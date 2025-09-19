use std::{f64::consts::PI, fmt::Display, str::FromStr};

use crate::collisions::{Contains, Points};

pub struct Rect {
    pub width: f64,
    pub height: f64,
    pub x: f64,
    pub y: f64,
}

impl Contains for Rect {
    fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        return self.x <= x && self.x + self.width >= x && self.y <= y && self.y + self.height >= y;
    }
}

impl Points for Rect {
    fn points(&self) -> crate::collisions::PointIter {
        return vec![
            (self.x, self.y),
            (self.x + self.width, self.y),
            (self.x, self.y + self.height),
            (self.x + self.width, self.y + self.height),
        ]
        .into();
    }
}
pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

impl Contains for Circle {
    fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        let dx = self.x - x;
        let dy = self.y - y;
        return dx * dx + dy * dy <= self.radius * self.radius;
    }
}

impl Points for Circle {
    fn points(&self) -> crate::collisions::PointIter {
        return vec![(self.x, self.y)].into();
    }
}

pub trait Area {
    fn area(&self) -> f64;
}

impl Area for Rect {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        return self.radius * self.radius * PI;
    }
}

impl Default for Rect {
    fn default() -> Self {
        return Rect {
            x: 0.0,
            y: 0.0,
            height: 10.0,
            width: 10.0,
        };
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "Rectangle({},{})): {}x{}",
            self.x, self.y, self.height, self.width
        );
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Circle({},{})): {}", self.x, self.y, self.radius);
    }
}

impl FromStr for Rect {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();
        if parts.len() != 4 {
            return Err(anyhow::anyhow!("bad Rectangle from str"));
        }
        return Ok(Rect {
            x: parts[0].parse()?,
            y: parts[1].parse()?,
            width: parts[2].parse()?,
            height: parts[3].parse()?,
        });
    }
}

impl FromStr for Circle {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(anyhow::anyhow!("bad Circle from str"));
        }
        return Ok(Circle {
            x: parts[0].parse()?,
            y: parts[1].parse()?,
            radius: parts[2].parse()?,
        });
    }
}
