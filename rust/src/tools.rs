use std::{
    collections::VecDeque,
    fmt::Debug,
    io::{Read, Write},
    str::FromStr,
};
pub struct Io {
    pub i: VecDeque<u8>,
    pub o: VecDeque<u8>,
}

pub type I<'a> = &'a mut Io;

impl Write for Io {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Write::write(&mut self.o, buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Write::flush(&mut self.i).unwrap();
        Write::flush(&mut self.o)
    }
}

impl Read for Io {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.i.read(buf)
    }
}

impl Io {
    pub fn new() -> Self {
        Io {
            i: VecDeque::new(),
            o: VecDeque::new(),
        }
    }

    /// Give an input to stdin.
    pub fn send(&mut self, z: impl std::fmt::Display) -> &mut Self {
        write!(self.i, "{z}").unwrap();
        self
    }

    /// Give an input to stdin, ending in a newline.
    pub fn sendln(&mut self, z: impl std::fmt::Display) -> &mut Self {
        self.send(format!("{z}\n"))
    }

    /// Print something to stdout.
    pub fn print(&mut self, z: impl std::fmt::Display) -> &mut Self {
        write!(self.o, "{z}").unwrap();
        self
    }

    /// Print something to stdout, ending in a newline.
    pub fn println(&mut self, z: impl std::fmt::Display) -> &mut Self {
        self.print(format!("{z}\n"))
    }

    /// Get the next input from stdin.
    pub fn next_char(&mut self) -> Option<char> {
        self.i.pop_front().map(|x| x as char)
    }

    /// Get the next line from stdin.
    pub fn next_line(&mut self) -> String {
        let mut v = String::new();

        let mut c = self.next_char();

        while let Some(p) = c && p != '\n' {
            v += &format!("{p}");
            c = self.next_char();
        }

        v
    }

    /// Convert the next line from stdin into `T`.
    pub fn get_next<T>(&mut self) -> T
    where
        T: FromStr,
        T::Err: Debug,
    {
        self.next_line().parse::<T>().unwrap()
    }

    /// Get the next input from stdout.
    pub fn read_char(&mut self) -> Option<char> {
        self.o.pop_front().map(|x| x as char)
    }

    /// Get the next line from stdout.
    pub fn read_line(&mut self) -> String {
        let mut v = String::new();

        let mut c = self.read_char();

        while let Some(p) = c && p != '\n' {
            v += &format!("{p}");
            c = self.read_char();
        }

        v
    }

    /// Convert the next line from stdout into `T`.
    pub fn read<T>(&mut self) -> T
    where
        T: FromStr,
        T::Err: Debug,
    {
        self.read_line().parse::<T>().unwrap()
    }
}

impl Default for Io {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Increment: std::ops::Add {
    const UNIT: Self;
    // x++
    fn post_incr(&mut self) -> &mut Self {
        let mut temp = self;
        self = self + Self::UNIT;
        temp
    }
    // ++x
    fn pre_incr(&mut self) -> &mut Self {
        self = self + Self::UNIT;
        self
    }
}

pub trait Decrement: std::ops::Sub {
    const UNIT: Self;
    // x--
    fn post_decr(&mut self) -> &mut Self {
        let mut temp = self;
        self = self - Self::UNIT;
        temp
    }
    // --x
    fn pre_decr(&mut self) -> &mut Self {
        self = self - Self::UNIT;
        self
    }
}

pub trait Compare<T> {
    fn compare(&self, other: &T) -> u32
}

impl Compare<String> for String {
    fn compare(&self: other: &String) {
        let mut i = std::cmp::min(a.len(), b.len());
        let mut x = 0;
        let mut y = 0;
        while { i -= 1; i } >= 0 {
            let x2 = {
                let temp = x;
                x += 1;
                temp
            };

            let y2 = {
                let temp = y;
                y += 1;
                temp
            };

            let result = self[x1..x1+1] - other[y1..y1+1];
            if result != 0 {
                return result;
            }
        }

        return self.len() - other.len()
    }
}