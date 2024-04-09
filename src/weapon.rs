/// Generic weapon struct for the player to have \
/// TODO: extract this more as a generic, and have implementations for different kinds of weapons?
pub struct Weapon {
    pub damage: i32,
    pub attack_speed: f32,

    attack_timer: f32,
}

impl Weapon {

    pub fn increment_attack_timer(&mut self, delta: f32) {
        if self.attack_timer < self.attack_speed {
            self.attack_timer += delta;
        }
    }

    /// Checks if our attack_timer reached the specified attack speed, if yes, it resets it.
    pub fn can_attack(&mut self) -> bool {
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