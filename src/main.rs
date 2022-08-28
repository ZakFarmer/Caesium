pub mod constants;
pub mod particle;
pub mod physics;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::thread::sleep;
use std::time::Duration;

use constants::{COULOMB_CONSTANT, ELECTRON_CHARGE, ELECTRON_MASS};
use glutin_window::GlutinWindow as Window;
use graphics::math::{add, mul_scalar};
use opengl_graphics::{GlGraphics, OpenGL};
use particle::Particle;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use rand::Rng;

use crate::constants::{
    DAMPING_FACTOR, GRAVITATIONAL_CONSTANT, PHYSICS_SCALE, SOLAR_MASS, SOLAR_RADIUS, TIMESCALE,
};
use crate::physics::scale_to_screen;

pub struct App {
    gl: GlGraphics,
    particles: Vec<Particle>,
}

impl App {
    fn init(&mut self, particle_num: i32) {
        let mut colours: [[f32; 4]; 4] = [
            [0.24, 0.19, 0.58, 1.0],
            [0.07, 0.439, 0.484, 1.0],
            [0.0, 0.58, 0.417, 1.0], // GREEN
            [0.32, 0.34, 0.34, 0.2],
        ];

        let mut rng = rand::thread_rng();

        self.particles = Vec::new();

        for _ in 0..particle_num {
            let particle_size: f64 = rng.gen_range(1.0..10.0);

            println!("Creating particle with size {}", particle_size);

            let particle = Particle::new(
                [0.0, 0.0],
                [
                    rng.gen_range(10.0..800.0) / PHYSICS_SCALE,
                    rng.gen_range(10.0..800.0) / PHYSICS_SCALE,
                ], //[rng.gen_range(0.0..800.0), rng.gen_range(0.0..800.0)],
                particle_size * SOLAR_RADIUS,
                [0.0, 0.0],
                particle_size * SOLAR_MASS,
                ELECTRON_CHARGE * rng.gen_range(1.0..2.0) as f64,
                colours[2], //rng.gen_range(0..4)],
            );

            self.particles.push(particle);
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen
            clear([0.0, 0.0, 0.0, 1.0], gl);

            for i in 0..self.particles.len() {
                let body = ellipse::circle(
                    0.0,
                    0.0,
                    f64::floor(self.particles[i].radius * PHYSICS_SCALE * 10.0 * 0.5),
                );
                let scaled_pos: [f64; 2] = scale_to_screen(self.particles[i].position);
                let transform = c.transform.trans(scaled_pos[0], scaled_pos[1]);

                ellipse(self.particles[i].colour, body, transform, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let mut angle: f64 = 0.0;
        let mut total_force: f64 = 0.0;

        for a in 0..self.particles.len() {
            angle = 0.0;
            total_force = 0.0;

            for b in 0..self.particles.len() {
                if a != b {
                    // Create distance vector for particle A
                    let distance: [f64; 2] = [
                        (self.particles[b].position[0] + self.particles[b].radius)
                            - (self.particles[a].position[0] + self.particles[a].radius),
                        (self.particles[b].position[1] + self.particles[b].radius)
                            - (self.particles[a].position[1] + self.particles[a].radius),
                    ];

                    let magnitude: f64 =
                        (distance[0] * distance[0] + distance[1] * distance[1]).sqrt();

                    if magnitude <= (self.particles[a].radius + self.particles[b].radius) * 2.0 {
                        self.particles[a].velocity = [
                            self.particles[a].velocity[0] * -DAMPING_FACTOR,
                            self.particles[a].velocity[1] * -DAMPING_FACTOR,
                        ];

                        self.particles[b].velocity = [
                            self.particles[b].velocity[0] * -DAMPING_FACTOR,
                            self.particles[b].velocity[1] * -DAMPING_FACTOR,
                        ];

                        self.particles[a].acceleration = [0.0, 0.0];

                        self.particles[b].acceleration = [0.0, 0.0];

                        continue;
                    }

                    angle = f64::asin(distance[1] / magnitude);

                    let force: f64 =
                        (GRAVITATIONAL_CONSTANT * self.particles[a].mass * self.particles[b].mass)
                            / (magnitude.powi(2));

                    total_force = total_force + force;
                }
            }

            let force_x = total_force * f64::cos(angle);
            let force_y: f64 = total_force * f64::sin(angle);

            self.particles[a].acceleration[0] = (force_x / self.particles[a].mass) * args.dt;
            self.particles[a].acceleration[1] = (force_y / self.particles[a].mass) * args.dt;

            self.particles[a].update(args.dt);
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Rust Particles", [800, 800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        particles: Vec::new(),
    };

    app.init(200);

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
