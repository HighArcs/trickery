use super::tools::*;
#[test]
fn stdin_write() {
    let mut i = Io::new();

    i.send('1');

    assert_eq!(i.i.pop_back(), Some(b'1'));
}

#[test]
fn stdin_read() {
    let mut i = Io::new();

    i.send('1');

    assert_eq!(i.next_char(), Some('1'));
}

#[test]
fn stdout_write() {
    let mut i = Io::new();

    i.print('1');

    assert_eq!(i.o.pop_back(), Some(b'1'));
}

#[test]
fn stdin_readln() {
    let mut i = Io::new();

    i.sendln("hello world");

    assert_eq!(i.next_line(), "hello world".to_owned())
}
