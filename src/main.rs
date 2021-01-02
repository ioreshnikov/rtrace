use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
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
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Self {
        Self {
            origin: origin,
            direction: direction.unit()
        }
    }
}

type Color = Vector;

pub fn ray_color(ray: &Ray) -> Color {
    let y = ray.direction.y;
    let t = 0.5 * (y + 1.0);
    let blue  = Color {x: 0.5, y: 0.7, z: 1.0};
    let white = Color {x: 1.0, y: 1.0, z: 1.0};
    return t * white + (1.0 - t) * blue;
}

const WINDOW_WIDTH:  u32 = 600;
const WINDOW_HEIGHT: u32 = 600;

const ASPECT_RATIO: f32 = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;

const VIEWPORT_WIDTH: f32 = 2.0;
const VIEWPORT_HEIGHT: f32 = VIEWPORT_WIDTH / ASPECT_RATIO;
const VIEWPORT_FOCUS_DISTANCE: f32 = 1.0;

const OG: Vector = Vector{x: 0.0, y: 0.0, z: 0.0};
const EX: Vector = Vector{x: 1.0, y: 0.0, z: 0.0};
const EY: Vector = Vector{x: 0.0, y: 1.0, z: 0.0};
const EZ: Vector = Vector{x: 0.0, y: 0.0, z: 1.0};

use sdl2;

impl Color {
    pub fn to_rgb(self) -> sdl2::pixels::Color {
        sdl2::pixels::Color::RGB(
            (255.0 * self.x) as u8,
            (255.0 * self.y) as u8,
            (255.0 * self.z) as u8
        )
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()
        .unwrap();

    for i in 0 .. WINDOW_HEIGHT {
        for j in 0 .. WINDOW_WIDTH {
            // Calculate coordinates of the point relative to the
            // viewport.
            let u = j as f32 / (WINDOW_WIDTH  as f32 - 1.0);
            let v = i as f32 / (WINDOW_HEIGHT as f32 - 1.0);

            let x = (u - 0.5) * VIEWPORT_WIDTH;
            let y = (v - 0.5) * VIEWPORT_HEIGHT;

            let ray = Ray::new(OG, x * EX + y * EY + VIEWPORT_FOCUS_DISTANCE * EZ);
            let color = ray_color(&ray);

            canvas.set_draw_color(color.to_rgb());
            canvas.draw_point(sdl2::rect::Point::new(j as i32, i as i32)).unwrap();
        }
    }
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {}
            }
        }
    }
}
