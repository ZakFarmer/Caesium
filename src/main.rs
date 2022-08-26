pub mod constants;
pub mod particle;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use particle::Particle;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use rand::Rng;

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
                [rng.gen_range(0.0..800.0), rng.gen_range(0.0..800.0)],
                [rng.gen_range(-100.0..100.0), rng.gen_range(-100.0..100.0)],
            ));
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let body = ellipse::circle(0.0, 0.0, 8.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen
            clear(BLACK, gl);

            for i in 0..self.particles.len() {
                let transform = c
                    .transform
                    .trans(self.particles[i].position[0], self.particles[i].position[1]);

                ellipse(WHITE, body, transform, gl);
            }
        })
    }

    fn update(&mut self, args: &UpdateArgs) {
        for i in 0..self.particles.len() {
            self.particles[i].update(args.dt);

            for n in 0..self.particles.len() {
                if i != n {
                    let dx = self.particles[i].position[0] - self.particles[n].position[0];
                    let dy = self.particles[i].position[1] - self.particles[n].position[1];
                    let dist = ((dx * dx) + (dy * dy)).sqrt();

                    if dist < 8.0 {
                        self.particles[i].velocity[0] *= -1.0;
                        self.particles[i].velocity[1] *= -1.0;
                        self.particles[n].velocity[0] *= -1.0;
                        self.particles[n].velocity[1] *= -1.0;
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

    app.init(100);

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
