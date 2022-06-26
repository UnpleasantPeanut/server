use std::cmp::{min, max};

const MAX_HEALTH_POINTS: u16 = 10000;
const MIN_HEALTH_POINTS: u16 = 0;

pub struct Health {
    current_health: u16,
    max_health: u16,
    min_health: u16,
    is_empty: bool
}

impl Health {
    pub fn new() -> Health {
        Health {
            current_health: MAX_HEALTH_POINTS,
            max_health: MAX_HEALTH_POINTS,
            min_health: MIN_HEALTH_POINTS,
            is_empty: false
        }
    }

    pub fn is_full(&self) -> bool {
        !self.is_empty
    }

    pub fn give(&mut self, amount: u16) {
        if self.is_empty {
            return;
        }
        self.current_health = min(self.current_health + amount, self.max_health);
    }

    pub fn take(&mut self, amount: u16) {
        if self.is_empty {
            return;
        }
        self.current_health = max(self.current_health - amount, self.min_health);
        if self.current_health == self.min_health {
            self.is_empty = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Health;
    use super::MAX_HEALTH_POINTS;
    use super::MIN_HEALTH_POINTS;

    #[test]
    fn should_create_a_new_health() {
        let health = Health::new();
        assert_eq!(health.is_empty, false);
        assert_eq!(health.is_full(), true);
    }

    #[test]
    fn should_be_able_to_take_health_away() {
        let half_of_max_health = MAX_HEALTH_POINTS/2;
        let mut health = Health::new();
        health.take(half_of_max_health);
        assert_eq!(health.is_full(), true);
        assert_eq!(health.current_health, half_of_max_health);
    }

    #[test]
    fn should_be_able_to_give_health_back() {
        let half_of_max_health = MAX_HEALTH_POINTS/2;
        let mut health = Health::new();
        health.take(half_of_max_health);
        health.give(half_of_max_health);
        assert_eq!(health.current_health, MAX_HEALTH_POINTS);
        assert_eq!(health.is_full(), true);
    }

    #[test]
    fn should_be_dead_if_health_reaches_min() {
        let mut health = Health::new();
        health.take(MAX_HEALTH_POINTS);
        assert_eq!(health.current_health, MIN_HEALTH_POINTS);
        assert_eq!(health.is_full(), false);
    }

    #[test]
    fn should_be_unable_to_gain_health_if_dead() {
        let mut health = Health::new();
        health.take(MAX_HEALTH_POINTS);
        health.give(MAX_HEALTH_POINTS);
        assert_eq!(health.current_health, MIN_HEALTH_POINTS);
        assert_eq!(health.is_full(), false);
    }

    #[test]
    fn should_be_unable_to_gain_more_health_than_max() {
        let mut health = Health::new();
        health.give(1);
        assert_eq!(health.current_health, MAX_HEALTH_POINTS);
    }
}
