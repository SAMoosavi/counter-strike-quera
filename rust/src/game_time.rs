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

    fn from_str(time: &str) -> Result<Self, Self::Err> {
        let parts: Result<Vec<u32>, _> = time.split(':').map(|s| s.trim().parse::<u32>()).collect();

        match parts {
            Ok(values) => {
                if values.len() != 3 {
                    return Err("The time format is incorrect.".to_string());
                }

                Ok(Self {
                    minute: values[0],
                    second: values[1],
                    millisecond: values[2],
                })
            }
            Err(_) => Err("The time format is incorrect.".to_string()),
        }
    }
}
