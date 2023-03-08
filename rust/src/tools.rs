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
