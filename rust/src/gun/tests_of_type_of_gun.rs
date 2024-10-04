use super::TypeOfGun;

#[test]
pub fn test_display_heavy() {
    let gun = TypeOfGun::Heavy;
    assert_eq!(gun.to_string(), "heavy");
}

#[test]
pub fn test_display_pistol() {
    let gun = TypeOfGun::Pistol;
    assert_eq!(gun.to_string(), "pistol");
}

#[test]
pub fn test_display_knife() {
    let gun = TypeOfGun::Knife;
    assert_eq!(gun.to_string(), "knife");
}
