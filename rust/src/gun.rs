use std::fmt;
use std::rc::Rc;

#[cfg(test)]
mod tests_of_type_of_gun;

#[cfg(test)]
mod tests_gun;

#[cfg(test)]
mod tests_guns;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TypeOfGun {
    Heavy,
    Pistol,
    Knife,
}

impl fmt::Display for TypeOfGun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl TypeOfGun {
    pub fn to_string(&self) -> &str {
        match &self {
            TypeOfGun::Heavy => "Heavy",
            TypeOfGun::Pistol => "Pistol",
            TypeOfGun::Knife => "Knife",
        }
    }

    pub fn to_enum(name: &str) -> Result<TypeOfGun, String> {
        match &name.to_lowercase().replace("_", "-").replace(" ", "-")[0..] {
            "heavy" => Ok(TypeOfGun::Heavy),
            "pistol" => Ok(TypeOfGun::Pistol),
            "knife" => Ok(TypeOfGun::Knife),
            _ => Err(format!("the name of {} isn't correct.", name)),
        }
    }
}

#[derive(Debug, Clone, Eq)]
pub struct Gun {
    name: String,
    price: u32,
    damage: u32,
    gift: u32,
    type_of: TypeOfGun,
}

impl Gun {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_price(&self) -> u32 {
        self.price
    }

    pub fn get_damage(&self) -> u32 {
        self.damage
    }

    pub fn get_gift(&self) -> u32 {
        self.gift
    }

    pub fn get_type_of(&self) -> &TypeOfGun {
        &self.type_of
    }

    pub fn new(name: String, price: u32, damage: u32, gift: u32, type_of: TypeOfGun) -> Gun {
        Gun {
            name,
            price,
            damage,
            gift,
            type_of,
        }
    }
}

impl fmt::Display for Gun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Gun {{ name: {}, price: {}, damage: {}, gift: {}, type_of: {} }}",
            self.name, self.price, self.damage, self.gift, self.type_of
        )
    }
}
impl PartialEq for Gun {
    fn eq(&self, other: &Self) -> bool {
        self.damage == other.damage
            && self.gift == other.gift
            && self.name == other.name
            && self.price == other.price
            && self.type_of == other.type_of
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Guns {
    list: Vec<Rc<Gun>>,
}

impl Default for Guns {
    fn default() -> Self {
        Self::new()
    }
}

impl Guns {
    pub fn new() -> Self {
        Self { list: vec![] }
    }

    pub fn create_gun(
        &mut self,
        name: String,
        price: u32,
        damage: u32,
        gift: u32,
        type_of: TypeOfGun,
    ) -> Result<(), String> {
        if self.list.iter().any(|gun| name == gun.get_name()) {
            return Err("the gun is exist!".to_string());
        } else if type_of == TypeOfGun::Knife
            && self
                .list
                .iter()
                .any(|gun| gun.get_type_of() == &TypeOfGun::Knife)
        {
            return Err("The knife exist".to_string());
        }

        self.list
            .push(Rc::new(Gun::new(name, price, damage, gift, type_of)));
        Ok(())
    }

    pub fn add_gun(&mut self, gun: &Rc<Gun>) -> Result<(), String> {
        if self.list.iter().any(|x| gun.get_name() == x.get_name()) {
            return Err("the gun is exist!".to_string());
        } else if gun.get_type_of() == &TypeOfGun::Knife
            && self
                .list
                .iter()
                .any(|gun| gun.get_type_of() == &TypeOfGun::Knife)
        {
            return Err("The knife exist".to_string());
        }

        self.list.push(gun.clone());
        Ok(())
    }

    pub fn get_knife(&self) -> Result<Rc<Gun>, ()> {
        for gun in &self.list {
            if gun.get_type_of() == &TypeOfGun::Knife {
                return Ok(gun.clone());
            }
        }
        Err(())
    }

    pub fn get_gun(&self, name: &str) -> Result<Rc<Gun>, String> {
        match self.list.iter().position(|gun| gun.get_name() == name) {
            Some(index) => Ok(self.list[index].clone()),
            None => Err("invalid category gun".to_string()),
        }
    }

    pub fn get_guns_with_type(&self, type_of_gun: TypeOfGun) -> Result<Vec<Rc<Gun>>, ()> {
        if type_of_gun == TypeOfGun::Knife {
            return Err(());
        }
        Ok(self
            .list
            .iter()
            .filter(|gun| gun.get_type_of() == &type_of_gun)
            .cloned()
            .collect())
    }
}
