use super::health::Health;

pub struct Being {
    health: Health
}

impl Being {
    pub fn new() -> Being {
        Being {
            health: Health::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Being;

    #[test]
    fn should_be_alive_when_spawned() {
        let being = Being::new();
        assert_eq!(being.health.is_full(), true);
    }
}
