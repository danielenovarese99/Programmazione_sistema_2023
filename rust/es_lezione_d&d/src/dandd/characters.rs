/* Characters example inspired by D&D (https://en.wikipedia.org/wiki/Dungeons_%26_Dragons)*/
use std::fmt::{Display, Formatter};

pub struct GeneralPoints<T> {
    life: T,
    strength: T,
}

impl<T: Default> GeneralPoints<T> {
    pub fn new() -> Self {
        Self::default()
    }
    /*pub fn attack(&mut self) -> () {

    }*/
    pub fn get_life(&self) -> &T {
        &self.life
    }

    pub fn set_life(&mut self, new_life: T) -> () {
        self.life = new_life;
    }
}

impl<T: Default> Default for GeneralPoints<T> {
    fn default() -> Self {
        Self {
            life: T::default(),
            strength: T::default(),
        }
    }
}

impl<T: Display> Display for GeneralPoints<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n\tlife: {}, strength: {}\n}}", self.life, self.strength)
    }
}

#[derive(Debug)]
pub struct WizardPoints<T> {
    spel_cast: T,
}

impl<T: Display> Display for WizardPoints<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Spelling Cast value: {}", self.spel_cast)
    }
}

impl<T: Default> WizardPoints<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Default> Default for WizardPoints<T> {
    fn default() -> Self {
        Self { spel_cast: T::default() }
    }
}

pub enum DDClasses<T> {
    ELF,
    THIEF,
    WARRIOR,
    WIZARD(WizardPoints<T>),
    NONE
}

pub struct DDCharacter<T> {
    pub name: String,
    pub character_class: DDClasses<T>,
    pub general_points: GeneralPoints<T>,
}

impl<T: Default> DDCharacter<T> {
    pub fn new(c: u8) -> Option<Self> {
        let general_points = GeneralPoints::new();

        let (name, char_class) = match c {
            1 => { ("My Elf".to_string(), DDClasses::ELF) },
            2 => { ("My Thief".to_string(), DDClasses::THIEF) },
            3 => { ("My Warrior".to_string(), DDClasses::WARRIOR) },
            4 => { ("My Wizard".to_string(), DDClasses::WIZARD(WizardPoints::new()) )}
             _ => {
                 ("".to_string(),
                  DDClasses::NONE)
             }
        };

        match char_class {
            DDClasses::NONE => None,
            _ => Some(
                Self {
                    name,
                    character_class: char_class,
                    general_points
                }
            )
        }
    }

    pub fn default_wizard() -> Self {
        Self {
            name: "My Wizard".to_string(),
            character_class: DDClasses::WIZARD(WizardPoints::new()),
            general_points: GeneralPoints::new(),
        }
    }

    pub fn default_thief() -> Self {
        Self {
            name: "My Thief".to_string(),
            character_class: DDClasses::THIEF,
            general_points: GeneralPoints::new(),
        }
    }

    pub fn default_elf() -> Self {
        Self {
            name: "My Elf".to_string(),
            character_class: DDClasses::ELF,
            general_points: GeneralPoints::new(),
        }
    }
}
