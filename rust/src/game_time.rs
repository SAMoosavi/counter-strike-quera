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

    pub fn new_from_str(time: &str) -> Self {
        let time = time.split(":").collect::<Vec<&str>>();

        Self {
            minute: time.get(0).unwrap().trim().parse().unwrap(),
            second: time.get(1).unwrap().trim().parse().unwrap(),
            millisecond: time.get(2).unwrap().trim().parse().unwrap(),
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
