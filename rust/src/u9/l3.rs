use std::fmt::{Display, Error, Formatter};

pub trait Animal<'a> {
    const NAME: &'a str;

    fn get_name(&self) -> String {
        Self::NAME.to_owned()
    }

    fn speak(&self) -> String;

    fn str(&self) -> String {
        format!("{} ({} {})", Self::NAME, self.speak(), self.speak())
    }
}

pub struct Cow;

impl<'a> Animal<'a> for Cow {
    const NAME: &'a str = "cow";

    fn speak(&self) -> String {
        return "moo".to_owned();
    }
}

pub struct Pig;

impl<'a> Animal<'a> for Pig {
    const NAME: &'a str = "pig";

    fn speak(&self) -> String {
        return "oink".to_owned();
    }
}

pub struct Sheep;

impl<'a> Animal<'a> for Sheep {
    const NAME: &'a str = "sheep";

    fn speak(&self) -> String {
        return "baa".to_owned();
    }
}

pub struct Farmer {
    forename: String,
    surname: String,
}

impl Farmer {
    pub fn new(f: String, s: String) -> Self {
        Self {
            forename: f,
            surname: s,
        }
    }
}

impl Display for Farmer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{} {}", self.forename, self.surname)
    }
}

pub struct Farm<T: for<'a> Animal<'a>, U: for<'a> Animal<'a>, V: for<'a> Animal<'a>> {
    the_farmer: Farmer,
    first_animal: T,
    second_animal: U,
    third_animal: V,
}

impl<T: for<'a> Animal<'a>, U: for<'a> Animal<'a>, V: for<'a> Animal<'a>> Display
    for Farm<T, U, V>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "{} had a farm.", self.the_farmer).unwrap();
        writeln!(f, "And on that farm he had a {}", self.first_animal.str()).unwrap();
        writeln!(f, "And on that farm he had a {}", self.second_animal.str()).unwrap();
        writeln!(f, "And on that farm he had a {}", self.third_animal.str())
    }
}
