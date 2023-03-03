use std::io::{Write, Read};
pub struct Io<'a> {
    i: &'a mut [u8],
    o: &'a mut [u8],
}

impl<'a> Write for Io<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Write::write(&mut self.o, buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Write::flush(&mut self.i).unwrap();
        Write::flush(&mut self.o)
    }
}

impl<'a> Read for Io<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        (&self.i[..]).read(buf)
    }
}

impl<'a> Io<'a> {
    pub fn new() -> Self {
        Io {
            i: &mut [][..],
            o: &mut [][..],
        }
    }

    pub fn send(&mut self, z: impl std::fmt::Display) -> &mut Self {
        write!(self.i, "{z}").unwrap();
        self
    }
    
    pub fn sendln(&mut self, z: impl std::fmt::Display) -> &mut Self {
        self.send(format!("{z}\n"))
    }
    
    pub fn print(&mut self, z: impl std::fmt::Display) -> &mut Self {
        write!(self.o, "{z}").unwrap();
        self
    }
    
    pub fn println(&mut self, z: impl std::fmt::Display) -> &mut Self {
        self.print(format!("{z}\n"))
    }
}
