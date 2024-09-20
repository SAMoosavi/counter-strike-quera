pub mod gun;
pub mod player;

pub mod setting;

fn main() {
    let _a = gun::Gun::new("AK".to_string(), 2700, 31, 100, gun::TypeOfGun::Heavy);
}
