use std::{
    fmt::{Display, Error, Formatter},
    ops::Deref,
};

pub struct Coffee {
    size: String,
    is_skinny: bool,
    shots: usize,
    kind: String,
}

impl Default for Coffee {
    fn default() -> Self {
        Self {
            size: "small".to_owned(),
            is_skinny: false,
            shots: 1,
            kind: "latte".to_owned(),
        }
    }
}

impl Coffee {
    pub fn new(size: String, is_skinny: bool, shots: usize, kind: String) -> Self {
        Self {
            size,
            is_skinny,
            shots,
            kind,
        }
    }

    pub fn get_size(&self) -> &String {
        &self.size
    }

    pub fn get_price(&self) -> usize {
        (self.shots * 30)
            + match &self.size[..] {
                "extra large" => 470,
                "large" => 440,
                "medium" => 360,
                _ => 330,
            }
    }
}

impl Display for Coffee {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.size).unwrap();
        if self.is_skinny {
            write!(f, " skinny").unwrap();
        }

        write!(f, " {}-shot {}", self.shots, self.kind)
    }
}

pub struct SpecialityCoffee {
    parent: Coffee,
    flavor: String,
}

impl Deref for SpecialityCoffee {
    type Target = Coffee;
    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl Default for SpecialityCoffee {
    fn default() -> Self {
        Self {
            flavor: "vanilla".to_owned(),
            parent: Coffee::default(),
        }
    }
}

impl SpecialityCoffee {
    pub fn new(size: String, kind: String, flavor: String) -> Self {
        Self {
            flavor,
            parent: Coffee::new(size, false, 1, kind),
        }
    }

    pub fn raw(size: String, is_skinny: bool, shots: usize, kind: String, flavor: String) -> Self {
        Self {
            flavor,
            parent: Coffee::new(size, is_skinny, shots, kind),
        }
    }

    pub fn get_price(&self) -> usize {
        let bump = self.parent.get_price();

        if self.get_size() == "large" || self.get_size() == "extra large" {
            bump + 50
        } else {
            bump + 30
        }
    }
}

impl Display for SpecialityCoffee {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{} with {}", self.parent, self.flavor)
    }
}
