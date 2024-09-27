use super::{Gun, TypeOfGun};

#[test]
pub fn test_getters() {
    let name = "gun";
    let price = 100;
    let damage = 20;
    let gift = 30;
    let type_of_gun = TypeOfGun::Pistol;
    let gun = Gun::new(name.to_string(), price, damage, gift, type_of_gun);
    assert_eq!(name, gun.get_name());
    assert_eq!(price, gun.get_price());
    assert_eq!(damage, gun.get_damage());
    assert_eq!(gift, gun.get_gift());
    assert_eq!(type_of_gun, gun.get_type_of());
}

#[test]
pub fn check_eq_operator() {
    let name = "gun";
    let price = 100;
    let damage = 20;
    let gift = 30;
    let type_of_gun = TypeOfGun::Pistol;
    let first_gun = Gun::new(name.to_string(), price, damage, gift, type_of_gun);
    let second_gun = Gun::new(name.to_string(), price, damage, gift, type_of_gun);

    assert_eq!(first_gun, second_gun);
}

#[test]
pub fn check_ne_operator() {
    let first_name = "first gun";
    let second_name = "second gun";
    let price = 100;
    let damage = 20;
    let gift = 30;
    let type_of_gun = TypeOfGun::Pistol;
    let first_gun = Gun::new(first_name.to_string(), price, damage, gift, type_of_gun);
    let second_gun = Gun::new(second_name.to_string(), price, damage, gift, type_of_gun);

    assert_ne!(first_gun, second_gun);
}
