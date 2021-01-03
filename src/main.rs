/// Basic vector arithmetics.

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

use rand;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn sqnorm(self) -> f32 {
        self.dot(self)
    }

    pub fn norm(self) -> f32 {
        self.sqnorm().sqrt()
    }

    pub fn unit(self) -> Self {
        self / self.norm()
    }

    pub fn random_unit() -> Self {
        loop {
            let v = Vector{
                x: rand::random::<f32>(),
                y: rand::random::<f32>(),
                z: rand::random::<f32>()
            };

            if v.sqnorm() >= 1.0 {
                continue
            }

            return v.unit();
        }
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

impl AddAssign<Vector> for Vector {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Add<f32> for Vector {
    type Output = Self;

    fn add(self, other: f32) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other
        }
    }
}

impl Add<Vector> for f32 {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self + other.x,
            y: self + other.y,
            z: self + other.z
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

impl Sub<Vector> for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

/// Minimal ray abstraction.
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

    pub fn at(self, t: f32) -> Vector {
        self.origin + t * self.direction
    }
}

/// Geometry
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Hit {
    pub t: f32,    // Distance along the ray to the intersection with the shape
    pub p: Vector, // Cartesian coordinates of the intersection
    pub n: Vector, // Outer surface normal at the intersection
}

impl Hit {
    pub fn new(t: f32, p: Vector, n: Vector) -> Self {
        Self {
            t: t,
            p: p,
            n: n.unit()
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    pub center: Vector,
    pub radius: f32
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let eps = 1E-3;

        let o = ray.origin - self.center;
        let b = ray.direction.dot(o);
        let c = o.sqnorm() - self.radius * self.radius;
        let discriminant = b * b - c;

        if discriminant < 0.0 {
            return None;
        }

        let d = discriminant.sqrt();

        let t1 = - b + d;
        let t2 = - b - d;

        if t1 < eps && t2 < eps {
            return None;
        }

        let t: f32 = match (t1 >= eps, t2 >= eps) {
            (false, true) => t2,
            (true, false) => t1,
            (true, true)  => t1.min(t2),
            _ => unreachable!()
        };

        let p = ray.at(t);
        let n = p - self.center;

        Some(Hit::new(t, p, n))
    }
}

pub struct World {
    pub objects: Vec<Box<dyn Hittable>>
}

impl World {
    pub fn new() -> World {
        World {
            objects: vec![]
        }
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let hits: Vec<Hit> = self.objects.iter()
            .map(|obj| obj.hit(&ray))
            .filter(|hit| hit.is_some())
            .map(|hit| hit.unwrap())
            .collect();

        if hits.is_empty() {
            return None
        }

        let nearest_hit = hits.iter().fold(hits[0], |a, b| {
            if a.t > b.t { *b } else { a }
        });

        return Some(nearest_hit);
    }
}

/// Ray tracing algorithm.
pub fn background_color(ray: &Ray) -> Vector {
    let y = ray.direction.y;
    let t = 0.5 * (y + 1.0);
    let blue  = Vector {x: 0.5, y: 0.7, z: 1.0};
    let white = Vector {x: 1.0, y: 1.0, z: 1.0};

    (1.0 - t) * white + t * blue
}

pub fn ray_color(ray: &Ray, world: &World, depth: u8) -> Vector {
    if depth == 0 {
        return Vector {x: 0.0, y: 0.0, z: 0.0};
    }

    let hit = world.hit(ray);

    if !hit.is_none() {
        let h = hit.unwrap();
        let d = h.n + Vector::random_unit();
        return 0.5 * ray_color(&Ray{origin: h.p, direction: d}, world, depth - 1);
    }

    background_color(ray)
}

/// Window and viewport related setup.
const IMAGE_WIDTH:  usize = 500;
const IMAGE_HEIGHT: usize = 500;

const ASPECT_RATIO: f32 = IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32;

const VIEWPORT_WIDTH: f32 = 2.0;
const VIEWPORT_HEIGHT: f32 = VIEWPORT_WIDTH / ASPECT_RATIO;
const VIEWPORT_FOCUS_DISTANCE: f32 = 1.0;

/// Rendering algorithm parameters.
const SAMPLES_PER_PIXEL: u32 = 100;
const RECURSION_DEPTH: u8 = 7;

/// Basic geometric constants.
const OG: Vector = Vector{x: 0.0, y: 0.0, z: 0.0};
const EX: Vector = Vector{x: 1.0, y: 0.0, z: 0.0};
const EY: Vector = Vector{x: 0.0, y: 1.0, z: 0.0};
const EZ: Vector = Vector{x: 0.0, y: 0.0, z: 1.0};

/// Auxiliary functions.
use sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Point;
use sdl2::render::{Canvas, RenderTarget};

pub fn to_rgb(vec: Vector) -> Color {
    Color::RGB(
        (255.0 * vec.x.sqrt()) as u8,
        (255.0 * vec.y.sqrt()) as u8,
        (255.0 * vec.z.sqrt()) as u8
    )
}

pub fn render_image<T: RenderTarget>(image: &[[Vector; IMAGE_WIDTH]; IMAGE_HEIGHT], canvas: &mut Canvas<T>) {
    for i in 0 .. IMAGE_HEIGHT {
        for j in 0 .. IMAGE_WIDTH {
            let vector = image[i][j] / (SAMPLES_PER_PIXEL as f32);
            canvas.set_draw_color(to_rgb(vector));
            canvas.draw_point(Point::new(j as i32, IMAGE_HEIGHT as i32 - i as i32)).unwrap();
        }
    }
    canvas.present();
}

fn main() {
    // Initialize the window.
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Raytracer Demo", IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()
        .unwrap();

    let black = Vector{x: 0.0, y: 0.0, z: 0.0};
    let mut image = [[black; IMAGE_WIDTH]; IMAGE_HEIGHT];

    let mut world = World::new();
    world.objects.push(Box::new(
        Sphere{
            center: Vector{ x: 0.0, y: 0.0, z: -1.0},
            radius: 0.5
        }
    ));
    world.objects.push(Box::new(
        Sphere{
            center: Vector{ x: 0.0, y: -100.5, z: -1.0},
            radius: 100.0
        }
    ));

    // For each pixel we cast a ray.
    for n in 0 .. SAMPLES_PER_PIXEL {
        for i in 0 .. IMAGE_HEIGHT {
            for j in 0 .. IMAGE_WIDTH {
                // Calculate coordinates of the point relative to the
                // viewport.
                let u = (j as f32 + rand::random::<f32>()) / (IMAGE_WIDTH  as f32 - 1.0);
                let v = (i as f32 + rand::random::<f32>()) / (IMAGE_HEIGHT as f32 - 1.0);

                let x = (u - 0.5) * VIEWPORT_WIDTH;
                let y = (v - 0.5) * VIEWPORT_HEIGHT;

                // Construct a ray going through the point on the
                // viewport.
                let ray = Ray::new(OG, x * EX + y * EY - VIEWPORT_FOCUS_DISTANCE * EZ - OG);

                // Perform ray tracing and see what color the ray should
                // be.
                let color = ray_color(&ray, &world, RECURSION_DEPTH);
                image[i][j] += color;
            }
        }
        println!("{:?}", n);
        render_image(&image, &mut canvas);
    }

    let mut event_pump = sdl_context.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'main,
                _ => {}
            }
        }
    }
}
