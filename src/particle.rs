use ggez::{
    glam::{vec2, Vec2},
    graphics::{Canvas, Color, DrawParam, Mesh},
    Context,
};
use rand::Rng;

use crate::consts::{CELL_SIZE, HEIGHT, WIDTH};
use crate::colors::ParticleColor;

#[derive(Clone)]
pub struct Particle {
    pos: Vec2,
    vel: Vec2,
    vel_mod: f32,
    angle: f32,
    history: Vec<Vec2>,
    history_max: usize,
    history_start: usize,
    timer: i32,
    color: Color,
}

impl Particle {
    pub fn new(color: &ParticleColor) -> Self {
        let mut rng = rand::thread_rng();
        let pos = vec2(
            rng.gen_range(0.0..WIDTH - CELL_SIZE),
            rng.gen_range(0.0..HEIGHT - CELL_SIZE),
        );
        let history_max = rng.gen_range(10..200);
        let colors = color.colors_range.clone();
        Self {
            pos,
            vel: vec2(0., 0.),
            vel_mod: rng.gen_range(1.0..5.0),
            angle: 0.,
            history: vec![pos],
            history_max,
            history_start: 0,
            timer: history_max as i32 * 2,
            color: colors[rng.gen_range(0..colors.len())],
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas, ctx: &mut Context) {
        if self.history.len() - 1 - self.history_start > 1 {
            self.color.a = 0.65;
            let line_mesh =
                Mesh::new_line(ctx, &self.history[self.history_start..], 1., self.color).unwrap();
            canvas.draw(&line_mesh, DrawParam::default());
        }
    }

    pub fn update(&mut self, cols: usize, rows: usize, flow_field: &Vec<f32>) {
        self.timer -= 1;
        if self.timer >= 1 {
            let x = (self.pos.x / CELL_SIZE).floor() as usize;
            let y = (self.pos.y / CELL_SIZE).floor() as usize;
            let mut index = y * cols + x;
            if index >= cols * rows {
                index = cols * rows - 1;
            }
            self.angle = flow_field[index];
            self.vel = vec2(self.angle.cos(), self.angle.sin());
            self.pos += self.vel * self.vel_mod;
            self.history.push(self.pos);
            if self.history.len() > self.history_max {
                self.history_start += 1;
            }
        } else if self.history_start != self.history.len() - 1 {
            self.history_start += 1;
        } else {
            self.reset();
        }
    }
    pub fn reset(&mut self) {
        let mut rng = rand::thread_rng();
        self.pos = vec2(
            rng.gen_range(0.0..WIDTH - CELL_SIZE),
            rng.gen_range(0.0..HEIGHT - CELL_SIZE),
        );
        self.history = vec![self.pos];
        self.timer = self.history_max as i32 * 2;
        self.history_start = 0;
    }
}
