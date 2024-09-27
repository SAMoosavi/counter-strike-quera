use super::TypeOfGun;

#[test]
pub fn test_display_heavy() {
    let gun = TypeOfGun::Heavy;
    assert_eq!(gun.to_string(), "Heavy");
}

#[test]
pub fn test_display_pistol() {
    let gun = TypeOfGun::Pistol;
    assert_eq!(gun.to_string(), "Pistol");
}

#[test]
pub fn test_display_knife() {
    let gun = TypeOfGun::Knife;
    assert_eq!(gun.to_string(), "Knife");
}
