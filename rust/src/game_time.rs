use std::cmp::Ordering;

#[cfg(test)]
mod test;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct GameTime {
    hour: u32,
    minute: u32,
    second: u32,
    millisecond: u32,
}

impl GameTime {
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
        (self.hour, self.minute, self.second, self.millisecond).cmp(&(
            other.hour,
            other.minute,
            other.second,
            other.millisecond,
        ))
    }
}
