use std::sync::Arc;
use crate::gun::Guns;
use crate::player::Player;

pub struct Team {
    players: Vec<Arc<Player>>,
    guns: Guns,
}

impl Team {
    pub fn default() -> Self {
        Self {
            players: vec!(),
            guns: Guns::new(),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::gun::Guns;
    use super::Team;
    #[test]
    pub fn test_default() {
        let team = Team::default();
        assert_eq!(team.guns, Guns::new());
        assert_eq!(team.players, vec!());
    }
    
}