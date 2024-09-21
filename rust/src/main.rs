pub mod gun;
pub mod player;

pub mod setting;
pub mod game_time;

fn main() {
    let _a = gun::Gun::new("AK".to_string(), 2700, 31, 100, gun::TypeOfGun::Heavy);
}
