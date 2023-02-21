use std::{
    fmt::{Display, Error, Formatter},
    ops::Deref,
};

pub struct Cone {
    flavor: String,
    waffle: bool,
}

impl Cone {
    pub fn new(f: String, w: bool) -> Self {
        Self {
            flavor: f,
            waffle: w,
        }
    }

    pub fn set_flavor(&mut self, f: String) {
        self.flavor = f;
    }
}

impl Display for Cone {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if self.waffle {
            write!(f, "waffle ").unwrap();
        }

        write!(f, "cone with {}", self.flavor)
    }
}

pub struct DoubleCone {
    parent: Cone,
    flavor1: String,
    flavor2: String,
    topping: Option<String>,
}

impl Deref for DoubleCone {
    type Target = Cone;
    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DoubleCone {
    pub fn with_one(f: String, w: bool) -> Self {
        Self {
            parent: Cone::new(f.clone(), w),
            flavor1: f.clone(),
            flavor2: f.clone(),
            topping: None,
        }
    }

    pub fn with_two(f1: String, f2: String, w: bool) -> Self {
        Self {
            parent: Cone::new(f1.clone(), w),
            flavor1: f1.clone(),
            flavor2: f2.clone(),
            topping: None,
        }
    }

    pub fn set_flavor_one(&mut self, f: String) {
        self.parent.set_flavor(f.clone());
        self.flavor1 = f.clone();
        self.flavor2 = f.clone();
    }

    pub fn set_flavor_two(&mut self, f1: String, f2: String) {
        self.parent.set_flavor(f1.clone());
        self.flavor1 = f1.clone();
        self.flavor2 = f2.clone();
    }

    pub fn add_topping(&mut self, t: String) {
        self.topping = Some(t);
    }
}

impl Display for DoubleCone {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "double {}", self.parent).unwrap();
        if self.flavor1 == self.flavor2 {
            write!(f, " x2").unwrap();
        } else {
            write!(f, " and {}", self.flavor2).unwrap();
        }

        if let Some(x) = &self.topping {
            write!(f, " with {x}").unwrap();
        }

        write!(f, "")
    }
}