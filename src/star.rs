use rand::Rng;

pub struct Star {
    pub x: u16,
    pub y: u16,
    pub brightness: u8,
}

impl Star {
    pub fn new(x: u16, y: u16, brightness: u8) -> Self {
        Star { x, y, brightness }
    }

    pub fn character(&self) -> char {
        match self.brightness {
            0..=63 => '.',
            64..=127 => '*',
            128..=191 => '+',
            _ => '✦',
        }
    }

    pub fn twinkle(&mut self, rng: &mut impl Rng) {
        if rng.gen_range(0..100) < 15 {
            let change: i16 = rng.gen_range(-8..=8);
            let new_brightness = (self.brightness as i16 + change).clamp(0, 255);
            self.brightness = new_brightness as u8;
        }
    }
}
