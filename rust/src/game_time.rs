use std::cmp::Ordering;

#[cfg(test)]
mod test;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct GameTime {
    minute: u32,
    second: u32,
    millisecond: u32,
}

impl GameTime {
    pub fn new(minute: u32, second: u32, millisecond: u32) -> Self {
        Self {
            minute,
            second,
            millisecond,
        }
    }
}

impl Ord for GameTime {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.minute, self.second, self.millisecond).cmp(&(
            other.minute,
            other.second,
            other.millisecond,
        ))
    }
}
