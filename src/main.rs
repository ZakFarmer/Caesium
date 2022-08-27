pub mod constants;
pub mod particle;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use constants::{COULOMB_CONSTANT, ELECTRON_CHARGE, ELECTRON_MASS};
use glutin_window::GlutinWindow as Window;
use graphics::math::{add, mul_scalar};
use opengl_graphics::{GlGraphics, OpenGL};
use particle::Particle;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use rand::Rng;

use crate::constants::{DAMPING_FACTOR, GRAVITATIONAL_CONSTANT, PHYSICS_SCALE};

pub struct App {
    gl: GlGraphics,
    particles: Vec<Particle>,
}

impl App {
    fn init(&mut self, particle_num: i32) {
        let mut rng = rand::thread_rng();

        self.particles = Vec::new();

        for _ in 0..particle_num {
            self.particles.push(Particle::new(
                [1.0, -100.0],
                [rng.gen_range(10.0..800.0), rng.gen_range(10.0..800.0)], //[rng.gen_range(0.0..800.0), rng.gen_range(0.0..800.0)],
                [0.0, 0.0],
                rng.gen_range(1.0..2.0) * 10e4,
                ELECTRON_CHARGE * rng.gen_range(1..2) as f64,
                [1.0, 1.0, 1.0, rng.gen_range(0.0..1.0) as f32],
            ));
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let body = ellipse::circle(0.0, 0.0, 1.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen
            clear([0.0, 0.0, 0.0, 1.0], gl);

            for i in 0..self.particles.len() {
                let transform = c
                    .transform
                    .trans(self.particles[i].position[0], self.particles[i].position[1]);

                ellipse(self.particles[i].colour, body, transform, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        for a in 0..self.particles.len() {
            let mut total_force: [f64; 2] = [0.0, 0.0];

            for b in 0..self.particles.len() {
                if a != b {
                    // Create distance vector for particle A
                    let distance = [
                        self.particles[a].position[0] - self.particles[b].position[0],
                        self.particles[a].position[1] - self.particles[b].position[1],
                    ];

                    // Newton's law of universal gravitation - F = G(m1 * m2) / r^2
                    let force: [f64; 2] = [
                        GRAVITATIONAL_CONSTANT * self.particles[a].mass * self.particles[b].mass
                            / (distance[0] * distance[0]),
                        GRAVITATIONAL_CONSTANT * self.particles[a].mass * self.particles[b].mass
                            / (distance[1] * distance[1]),
                    ];

                    total_force = add(total_force, force);

                    if total_force[0].is_nan() {
                        total_force[0] = 0.0;
                    }

                    if total_force[1].is_nan() {
                        total_force[1] = 0.0;
                    }
                }
            }

            println!("Total force: {:?}", total_force);

            self.particles[a].acceleration = mul_scalar(
                total_force,
                PHYSICS_SCALE * args.dt * (1.0 / self.particles[a].mass),
            );

            println!("Acceleration: {:?}", self.particles[a].acceleration);

            if self.particles[a].position[0] < 0.0 {
                self.particles[a].acceleration[0] = 0.0;
                self.particles[a].position[0] = 0.0;
                self.particles[a].velocity[0] = -self.particles[a].velocity[0] * DAMPING_FACTOR;
            }

            if self.particles[a].position[1] < 0.0 {
                self.particles[a].acceleration[1] = 0.0;
                self.particles[a].position[1] = 0.0;
                self.particles[a].velocity[1] = -self.particles[a].velocity[1] * DAMPING_FACTOR;
            }

            if self.particles[a].position[0] > 700.0 {
                self.particles[a].acceleration[0] = 0.0;
                self.particles[a].position[0] = 700.0;
                self.particles[a].velocity[0] = -self.particles[a].velocity[0] * DAMPING_FACTOR;
            }

            if self.particles[a].position[1] > 700.0 {
                self.particles[a].acceleration[1] = 0.0;
                self.particles[a].position[1] = 700.0;
                self.particles[a].velocity[1] = -self.particles[a].velocity[1] * DAMPING_FACTOR;
            }

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
