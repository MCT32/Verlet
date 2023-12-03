use std::ops::{Sub, SubAssign, Add, Mul, Div, AddAssign};

use sfml::{graphics::{Color, RenderTarget, CircleShape, Transformable, Shape}, system::Vector2f};


#[derive(Clone, Copy)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalise(&self) -> Self {
        Vector { x: self.x / self.len(), y: self.y / self.len() }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f32> for Vector {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl From<(f32, f32)> for Vector {
    fn from(value: (f32, f32)) -> Self {
        Vector { x: value.0, y: value.1 }
    }
}

impl Into<(f32, f32)> for Vector {
    fn into(self) -> (f32, f32) {
        (self.x, self.y)
    }
}

impl Into<Vector2f> for Vector {
    fn into(self) -> Vector2f {
        Vector2f::from((self.x, self.y))
    }
}


pub struct Ball {
    position: Vector,
    position_last: Vector,
    acceleration: Vector,
    radius: f32,

    pub color: Color,
}

impl Ball {
    pub fn new<V: Into<Vector> + Copy>(position: V, radius: f32) -> Self {
        Ball {
            position: position.into(),
            position_last: position.into(),
            acceleration: Vector::from((0.0, 0.0)),
            radius,

            color: Color::RED,
        }
    }

    pub fn set_acceleration(&mut self, acceleration: Vector) {
        self.acceleration = acceleration;
    }

    pub fn constrain(&mut self, radius: f32) {
        let max_dist = radius - self.radius;

        if self.position.len() > max_dist {
            let direction = self.position.normalise();
            self.position = direction * max_dist;
        }
    }

    pub fn update(&mut self, dt: f32) {
        let velocity = self.position - self.position_last;
        self.position_last = self.position;
        self.position = self.position + velocity + self.acceleration * dt;
    }


    pub fn render<T: RenderTarget>(&self, target: &mut T) {
        let mut circle = CircleShape::new(1.0, 30);
        circle.set_origin((1.0, 1.0));
        circle.set_position(self.position + Vector::from((400.0, 300.0)));
        circle.set_fill_color(self.color);
        circle.scale((self.radius, self.radius));

        target.draw(&circle);
    }
}


pub struct World {
    balls: Vec<Ball>,
    gravity: Vector,
}

impl World {
    pub fn new() -> Self {
        World {
            balls: Vec::new(),
            gravity: Vector::from((0.0, 9.8)),
        }
    }

    pub fn add(&mut self, ball: Ball) {
        self.balls.push(ball);
    }

    pub fn with(mut self, ball: Ball) -> Self {
        self.add(ball);
        self
    }

    pub fn update(&mut self, dt: f32) {
        for i in self.balls.iter_mut() {
            i.set_acceleration(self.gravity);
            i.update(dt);
            i.constrain(250.0);
        }

        self.collide();
    }

    pub fn collide(&mut self) {
        for i in 0..self.balls.len() {
            for j in 0..self.balls.len() {
                if i != j {
                    let overlap = -(self.balls[i].position - self.balls[j].position).len() + self.balls[i].radius + self.balls[j].radius;

                    if overlap > 0.0 {
                        let direction = (self.balls[j].position - self.balls[i].position).normalise();
                        self.balls[i].position -= direction * (overlap / 2.0);
                        self.balls[j].position += direction * (overlap / 2.0);
                    }
                }
            }
        }
    }


    pub fn render<T: RenderTarget>(&self, target: &mut T) {
        for i in self.balls.iter() {
            i.render(target);
        }
    }
}