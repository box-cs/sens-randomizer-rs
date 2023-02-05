use rand::{thread_rng, Rng};

pub struct SensitivityRandomizer {
    minimum: f32,
    maximum: f32,
}

impl SensitivityRandomizer {
    pub fn new(minimum: f32, maximum: f32) -> Self {
        Self { minimum, maximum }
    }

    pub fn generate_random_sensitivity(&self) -> f32 {
        let mut rng = thread_rng();
        rng.gen_range(self.minimum..self.maximum)
    }
}
