use rand::Rng;

pub struct ShootingStar {
    pub x: f32,
    pub y: f32,
    pub vel_x: f32,
    pub vel_y: f32,
    pub trail: Vec<(f32, f32)>,
    pub alive: bool,
}

impl ShootingStar {
    pub fn new(rng: &mut impl Rng, width: u16, height: u16) -> Self {
        let vel_x: f32 = rng.gen_range(1.5..3.5) * if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
        let vel_y: f32 = rng.gen_range(0.5..1.5);

        ShootingStar {
            x: rng.gen_range(0..width) as f32,
            y: rng.gen_range(0..height / 2) as f32,
            vel_x,
            vel_y,
            trail: Vec::new(),
            alive: true,
        }
    }

    pub fn update(&mut self, width: u16, height: u16) {
        self.trail.push((self.x, self.y));

        if self.trail.len() > 12 {
            self.trail.remove(0);
        }

        self.x += self.vel_x;
        self.y += self.vel_y;

        if self.x < 0.0 || self.x >= width as f32 || self.y >= height as f32 {
            self.alive = false;
        }
    }

    pub fn trail_char(index: usize, trail_len: usize) -> char {
        let ratio = index as f32 / trail_len as f32;
        if ratio > 0.75 {
            '✦'
        } else if ratio > 0.5 {
            '·'
        } else if ratio > 0.25 {
            '∙'
        } else {
            '.'
        }
    }
}
