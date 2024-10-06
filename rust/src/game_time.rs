#[cfg(test)]
mod test;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
            minute: time[0].trim().parse().unwrap(),
            second: time[1].trim().parse().unwrap(),
            millisecond: time[2].trim().parse().unwrap(),
        }
    }
}
