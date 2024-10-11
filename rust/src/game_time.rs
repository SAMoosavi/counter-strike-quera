use std::str::FromStr;

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
}

impl FromStr for GameTime {
    type Err = String;

    // Custom parsing logic for `Time`
    fn from_str(time: &str) -> Result<Self, Self::Err> {
        // Split the input string by ":"
        let time: Vec<u32> = time
            .split(":")
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect();

        if time.len() != 3 {
            return Err("the time is not correct!".to_string());
        }

        Ok(Self {
            minute: time[0],
            second: time[1],
            millisecond: time[2],
        })
    }
}
