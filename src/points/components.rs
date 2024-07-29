use bevy::prelude::*;

#[derive(Component)]
pub struct Points(u32);

impl Points {
    pub fn get(&self) -> u32 {
        return self.0;
    }

    pub(crate) fn add_points(&mut self, points: &u32) -> &Self {
        self.0 += points;
        return self;
    }

    pub(crate) fn subtract_points(&mut self, points: &u32) -> &Self {
        self.0 -= (*points).min(self.0);
        return self;
    }

    pub(crate) fn reset_points(&mut self) -> &Self {
        self.0 = 0;
        return self;
    }
}

impl Default for Points {
    fn default() -> Self {
        return Points(0);
    }
}
