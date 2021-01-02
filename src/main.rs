use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x: x, y: y, z: z }
    }

    pub fn dot(self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn sqnorm(self) -> f32 {
        self.dot(&self)
    }

    pub fn norm(self) -> f32 {
        self.sqnorm().sqrt()
    }

    pub fn unit(self) -> Self {
        self / self.norm()
    }
}

impl Add<Vector> for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Div<f32> for Vector {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            x: other * self.x,
            y: other * self.y,
            z: other * self.z
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Ray {
    origin: Vector,
    direction: Vector
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Self {
        Self {
            origin: origin,
            direction: direction.unit()
        }
    }
}

fn main() {
    let og = Vector::new(0.0, 0.0, 0.0);

    let ex = Vector::new(1.0, 0.0, 0.0);
    let ey = Vector::new(0.0, 1.0, 0.0);
    let ez = Vector::new(0.0, 0.0, 1.0);

    println!("{:?}", ex);
    println!("{:?}", ex + ey);
    println!("{:?}", ex + ey + ez);
    println!("{:?}", 2.0 * ez);
    println!("{:?}", ez * 2.0);
    println!("{:?}", ez / 2.0);

    println!("{:?}", ex.norm());
    println!("{:?}", ex.dot(&ex));
    println!("{:?}", (ex + ey + ez).unit());

    let ray = Ray::new(og, ez);
    println!("{:?}", ray);

    println!("Hello, world!");
}
