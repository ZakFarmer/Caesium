extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::PressEvent;
use piston::{Button, Key};
use rand::Rng;
use rust_particles::physics::generators::generate_particle;
use rust_particles::physics::nbody::step_simulation;
use rust_particles::physics::simulation::{Simulation3D, SimulationConfig3D};
use rust_particles::physics::vector::{Vector, Vector3D};

const ELECTRON_CHARGE: f32 = -1.602_176_6e-19;
const ELECTRON_MASS: f32 = 9.1093837 * 10e-31;

const PROTON_MASS: f32 = 1.67262192 * 10e-27;

const PARTICLE_COUNT: usize = 250;
const PARTICLE_WIDTH_FACTOR: f32 = 0.5 * 1e-30;

pub struct App<'a> {
    gl: GlGraphics,
    sim: &'a mut Simulation3D,
}

impl App<'_> {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let sim: &mut Simulation3D = self.sim;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen
            clear([0.0, 0.0, 0.0, 1.0], gl);

            for i in 0..sim.number {
                let particle_square = rectangle::square(
                    0.0,
                    0.0,
                    (sim.mass[i] / (PARTICLE_WIDTH_FACTOR * 1e4)).into(),
                );

                let transform = c
                    .transform
                    .trans(sim.position[i].x as f64, sim.position[i].y as f64);

                ellipse([1.0, 1.0, 1.0, 1.0], particle_square, transform, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        step_simulation(self.sim, args.dt as f32 * 10.0, 0.5);
    }
}

fn main() {
    let min_dist: f32 = 10.;
    let min_position: Vector3D = Vector3D::from_xy(0., 0.);
    let max_position: Vector3D = Vector3D::from_xy(900., 900.);

    let config = SimulationConfig3D::new(min_dist, min_position, max_position);

    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let mut sim: Simulation3D = Simulation3D::empty(PARTICLE_COUNT, config);

    for i in 0..sim.number {
        // Randomise whether this is a proton or an electron
        let random_num = rng.gen_range(0..100);

        let charge: f32 = if random_num < 50 {
            -ELECTRON_CHARGE
        } else {
            ELECTRON_CHARGE
        };

        let mass: f32 = if random_num < 50 {
            PROTON_MASS
        } else {
            ELECTRON_MASS
        };

        sim.set(
            i,
            &generate_particle(
                rng.gen_range(0..max_position.x as usize) as f32,
                rng.gen_range(0..max_position.y as usize) as f32,
                mass,
                charge,
            ),
        );
    }

    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "Rust Particles",
        [max_position.x as u32, max_position.y as u32],
    )
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        sim: &mut sim,
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.press_args() {
            match args {
                Button::Keyboard(Key::Comma) => {
                    println!("Timescale: {}", app.sim.timescale);
                    if app.sim.timescale <= 1.0 {
                        continue;
                    }

                    app.sim.timescale -= 1.0;
                }
                Button::Keyboard(Key::Period) => {
                    println!("Timescale: {}", app.sim.timescale);
                    if app.sim.timescale >= 49.0 {
                        continue;
                    }

                    app.sim.timescale += 1.0;
                }
                _ => {}
            }
        }
    }
}
