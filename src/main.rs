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

use crate::constants::PHYSICS_SCALE;

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
                [0.0, 0.0],
                [rng.gen_range(10.0..800.0), rng.gen_range(10.0..800.0)], //[rng.gen_range(0.0..800.0), rng.gen_range(0.0..800.0)],
                [rng.gen_range(-5.0..5.0), rng.gen_range(-5.0..5.0)],
                rng.gen_range(1.0..10.0) * ELECTRON_MASS,
                ELECTRON_CHARGE * rng.gen_range(1..5) as f64,
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
            self.particles[a].update(args.dt);

            for b in 0..self.particles.len() {
                if a != b {
                    // Create distance vector for particle A
                    let distance_a = [
                        self.particles[a].position[0] - self.particles[b].position[0],
                        self.particles[a].position[1] - self.particles[b].position[1],
                    ];

                    // Create distance vector for particle B
                    let distance_b: [f64; 2] = [
                        self.particles[b].position[0] - self.particles[a].position[0],
                        self.particles[b].position[1] - self.particles[a].position[1],
                    ];

                    // Coulomb's law - F = k * q1 * q2 / r^2
                    let force_a: [f64; 2] = [
                        COULOMB_CONSTANT * self.particles[a].charge * self.particles[b].charge
                            / (distance_a[0] * distance_a[0]),
                        COULOMB_CONSTANT * self.particles[a].charge * self.particles[b].charge
                            / (distance_a[1] * distance_a[1]),
                    ];

                    let force_b: [f64; 2] = [
                        COULOMB_CONSTANT * self.particles[b].charge * self.particles[a].charge
                            / (distance_b[0] * distance_b[0]),
                        COULOMB_CONSTANT * self.particles[b].charge * self.particles[a].charge
                            / (distance_b[1] * distance_b[1]),
                    ];

                    let acceleration_a: [f64; 2] = [
                        force_a[0] / self.particles[a].mass,
                        force_a[1] / self.particles[a].mass,
                    ];

                    let acceleration_b: [f64; 2] = [
                        force_b[0] / self.particles[b].mass,
                        force_b[1] / self.particles[b].mass,
                    ];

                    self.particles[a].acceleration = mul_scalar(
                        add(self.particles[a].acceleration, acceleration_a),
                        PHYSICS_SCALE,
                    );

                    self.particles[b].acceleration = mul_scalar(
                        add(self.particles[b].acceleration, acceleration_b),
                        PHYSICS_SCALE,
                    );

                    if self.particles[a].acceleration[0].is_nan()
                        || self.particles[a].acceleration[0].is_infinite()
                    {
                        self.particles[a].acceleration[0] = 0.0;
                    }

                    if self.particles[a].acceleration[1].is_nan()
                        || self.particles[a].acceleration[1].is_infinite()
                    {
                        self.particles[a].acceleration[1] = 0.0;
                    }

                    if self.particles[b].acceleration[0].is_nan()
                        || self.particles[b].acceleration[0].is_infinite()
                    {
                        self.particles[b].acceleration[0] = 0.0;
                    }

                    if self.particles[b].acceleration[1].is_nan()
                        || self.particles[b].acceleration[1].is_infinite()
                    {
                        self.particles[b].acceleration[1] = 0.0;
                    }
                }
            }
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

    app.init(1000);

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
