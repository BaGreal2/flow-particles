use ggez::graphics::Color;
use rand::Rng;

#[derive(Clone)]
pub struct ParticleColor {
    pub base_color: Color,
    pub colors_range: Vec<Color>,
}

impl ParticleColor {
    pub fn new(base_color: Color) -> Self {
        let mut rng = rand::thread_rng();
        let mut colors_range = vec![base_color];
        let mut total_shift = 0.0;
        for _ in 0..7 {
            let color_shift = total_shift + rng.gen_range(0.01..0.1);
            total_shift = color_shift;
            let new_color = Color::new(
                (base_color.r + total_shift).clamp(0.0, 255.0),
                (base_color.g + total_shift).clamp(0.0, 255.0),
                (base_color.b + total_shift).clamp(0.0, 255.0),
                base_color.a,
            );
            colors_range.push(new_color);
        }
        return Self {
            base_color,
            colors_range,
        };
    }
}
