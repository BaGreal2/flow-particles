use ggez::{graphics::{Canvas, Color}, Context};

use crate::{
    consts::{CELL_SIZE, HEIGHT, WIDTH},
    particle::Particle,
    colors::ParticleColor
};

#[derive(Clone)]
pub struct Effect {
    particles: Vec<Particle>,
    rows: usize,
    cols: usize,
    flow_field: Vec<f32>,
    pub colors: Vec<ParticleColor>,
    pub curve: f32,
    pub zoom: f32,
    pub color_index: usize
}

impl Effect {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            colors: vec![ParticleColor::new(Color::from_rgb(143, 20, 11)), ParticleColor::new(Color::from_rgb(14, 76, 32)), ParticleColor::new(Color::from_rgb(12, 16, 206)), ParticleColor::new(Color::from_rgb(116, 62, 8))],
            rows: 0,
            cols: 0,
            flow_field: Vec::new(),
            curve: 4.,
            zoom: 0.05,
            color_index: 0
        }
    }
    pub fn init(&mut self, amount_of_particles: usize) {
        for _ in 0..amount_of_particles {
            self.particles.push(Particle::new(&self.colors[self.color_index]));
        }
        self.rows = (HEIGHT / CELL_SIZE).floor() as usize;
        self.cols = (WIDTH / CELL_SIZE).floor() as usize;
        for y in 0..self.rows {
            for x in 0..self.cols {
                let angle =
                    (f32::cos(x as f32 * self.zoom) + f32::sin(y as f32 * self.zoom)) * self.curve;
                self.flow_field.push(angle);
            }
        }
    }

    pub fn render(&mut self, canvas: &mut Canvas, ctx: &mut Context) {
        self.particles.iter_mut().for_each(|particle| {
            particle.update(self.cols, self.rows, &self.flow_field);
            particle.draw(canvas, ctx);
        });
    }

    pub fn reset(&mut self) {
        self.particles = vec![];
        self.flow_field = vec![];
    }
}
