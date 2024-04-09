
pub struct Weapon {
    pub damage: i32,
    pub attack_speed: f32,

    attack_timer: f32,
}

impl Weapon {
    pub fn update_attack(&mut self, delta: f32) {
        if self.attack_timer < self.attack_speed {
            self.attack_timer += delta;
        }
    }


    pub fn attack(&mut self) -> bool {
        if self.attack_timer >= self.attack_speed {
            self.attack_timer -= self.attack_speed;
            return true;
        }
        return false;
    } 
}

impl Default for Weapon {
    fn default() -> Self {
        Weapon {
            damage: 1,
            attack_speed: 0.1,

            attack_timer: 0.
        }
    }
}