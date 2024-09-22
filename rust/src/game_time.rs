use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct GameTime {
    hour: u32,
    minute: u32,
    second: u32,
    millisecond: u32,
}

impl GameTime {
    #[allow(dead_code)]
    pub fn new(hour: u32, minute: u32, second: u32, millisecond: u32) -> Self {
        Self {
            hour,
            minute,
            second,
            millisecond,
        }
    }
}

impl Ord for GameTime {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.hour, self.minute, self.second, self.millisecond)
            .cmp(&(other.hour, other.minute, other.second, other.millisecond))
    }
}

#[cfg(test)]
mod tests_game_time {
    use super::GameTime;
    #[test]
    pub fn test_cmp() {
        let time1 = GameTime::new(10, 10, 10, 10);
        let time2 = GameTime::new(10, 10, 10, 9);
        assert!(time1 > time2);
        let time3 = GameTime::new(10, 10, 9, 10);
        assert!(time1 > time3);
        let time4 = GameTime::new(10, 9, 10, 10);
        assert!(time1 > time4);
        let time5 = GameTime::new(9, 10, 10, 10);
        assert!(time1 > time5);
        let time6 = GameTime::new(10, 10, 10, 10);
        assert_eq!(time1, time6);
        let time7 = GameTime::new(10, 10, 10, 11);
        assert!(time1 < time7);
        let time8 = GameTime::new(10, 10, 11, 10);
        assert!(time1 < time8);
        let time9 = GameTime::new(10, 11, 10, 10);
        assert!(time1 < time9);
        let time10 = GameTime::new(11, 10, 10, 10);
        assert!(time1 < time10);
    }
    #[test]
    pub fn test_new() {
        let time = GameTime::new(10, 9, 8, 7);
        assert_eq!(time.hour, 10);
        assert_eq!(time.minute, 9);
        assert_eq!(time.second, 8);
        assert_eq!(time.millisecond, 7);
    }
}