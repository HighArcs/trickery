use std::{
    fmt::{Display, Error, Formatter},
    ops::Deref,
};



pub trait Animal {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn speak(&self) -> String {
        return "".to_owned();
    }
}

impl Display for Animal {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!("{} ({} {}).", self.get_name(), self.speak(), self.speak())
    }
}

macro_rules! extend {
    ($struct:ident, $parent:ident) {
        impl Deref for Struct {
            type Target = $parent;
            fn deref(&self) -> &Self::Target {
                &self.parent
            }
        }
    }
}

pub struct Cow {
    parent: Animal,
}

impl Cow {
    pub fn new() -> Self {
        Self {
            parent: Animal {
                name: "cow".to_owned(),
            },
        }
    }

    pub fn speak() -> String {
        return "moo".to_owned()
    }
}

pub struct Pig {
    parent: Animal,
}

impl Pig {
    pub fn new() -> Self {
        Self {
            parent: Animal {
                name: "pig".to_owned(),
            },
        }
    }

    pub fn speak() -> String {
        return "oink".to_owned()
    }
}

pub struct Sheep {
    parent: Animal,
}

impl Sheep {
    pub fn new() -> Self {
        Self {
            parent: Animal {
                name: "sheep".to_owned(),
            },
        }
    }

    pub fn speak() -> String {
        return "baa".to_owned()
    }
}

extend!(Cow, Animal)
extend!(Pig, Animal)
extend!(Sheep, Animal)

pub struct Farmer {
    forename: String,
    surname: String
}

impl Farmer {
    pub fn new(f: Strnig, s: String) -> Self {
        Self { forename: f, surname: s }
    }
}

impl Display for Animal {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!("{} {}", self.forename, self.surname)
    }
}

pub struct Farm {
    the_farmer: Farmer,
    first_animal: Animal,

}