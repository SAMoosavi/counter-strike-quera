use super::{Gun, Guns, TypeOfGun};
use std::rc::Rc;

#[test]
pub fn add_gun_should_return_error_when_gun_name_is_exist() {
    let mut guns = Guns::new();
    let name = "knife";
    assert!(guns
        .add_gun(name.to_string(), 100, 20, 100, TypeOfGun::Knife)
        .is_ok());
    assert!(guns
        .add_gun(name.to_string(), 10, 2, 500, TypeOfGun::Knife)
        .is_err());
}

#[test]
pub fn add_gun_should_add_a_gun_in_list_of_gun() {
    let mut guns = Guns::new();
    let name = "knife";
    let price = 100;
    let damage = 20;
    let gift = 10;
    let type_of = TypeOfGun::Knife;
    assert!(guns
        .add_gun(name.to_string(), price, damage, gift, type_of)
        .is_ok());
    assert_eq!(guns.list.len(), 1);
    assert_eq!(
        guns.list[0],
        Rc::new(Gun::new(name.to_string(), price, damage, gift, type_of))
    );
}

#[test]
pub fn get_knife_when_has_not_knife_shold_be_retur_error() {
    let mut guns = Guns::new();
    guns.add_gun("not a knife".to_string(), 100, 10, 10, TypeOfGun::Heavy)
        .unwrap();
    assert!(guns.get_knife().is_err());
}

#[test]
pub fn get_knife_when_has_knife_shold_be_retur_knife() {
    let mut guns = Guns::new();
    let name = "knife";
    let price = 100;
    let damage = 20;
    let gift = 10;
    let type_of = TypeOfGun::Knife;
    guns.add_gun(name.to_string(), price, damage, gift, type_of)
        .unwrap();
    let result = guns.get_knife();
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        Rc::new(Gun::new(name.to_string(), price, damage, gift, type_of))
    );
}

#[test]
pub fn get_gun_func_when_get_a_name_of_gun_does_not_exist_should_be_return_error() {
    let mut guns = Guns::new();
    let name = "test gun";
    let fake_name = "fake gun";
    guns.add_gun(name.to_string(), 100, 10, 10, TypeOfGun::Heavy)
        .unwrap();
    assert!(guns.get_gun(&fake_name).is_err());
}

#[test]
pub fn get_gun_func_when_get_a_name_of_gun_exists_should_be_return_gun() {
    let mut guns = Guns::new();
    let name = "knife";
    let price = 100;
    let damage = 20;
    let gift = 10;
    let type_of = TypeOfGun::Knife;
    guns.add_gun(name.to_string(), price, damage, gift, type_of)
        .unwrap();
    let result = guns.get_gun(&name);
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        Rc::new(Gun::new(name.to_string(), price, damage, gift, type_of))
    );
}

#[test]
pub fn get_guns_func_when_get_knife_type_should_be_error() {
    let guns = Guns::new();
    assert!(guns.get_guns_with_type(TypeOfGun::Knife).is_err());
}

#[test]
pub fn get_guns_type_func_when_get_a_type_of_gun_should_be_return_list_of_gun_with_this_type() {
    let mut guns = Guns::new();
    let type_of = TypeOfGun::Heavy;
    let other_type_of = TypeOfGun::Pistol;
    guns.add_gun("gun 1".to_string(), 100, 10, 20, type_of)
        .unwrap();
    guns.add_gun("gun 2".to_string(), 100, 10, 20, type_of)
        .unwrap();
    guns.add_gun("gun 3".to_string(), 100, 10, 20, type_of)
        .unwrap();
    guns.add_gun("gun with other type".to_string(), 50, 10, 20, other_type_of)
        .unwrap();

    let result = guns.get_guns_with_type(type_of);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 3);
}
