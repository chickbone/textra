extern crate termios;

use std::io::{self, Read};
use std::os::unix::io::{AsRawFd, RawFd};
use termios::*;

struct InputRawMode {
    fd: RawFd,
    orig: Termios,
}

impl InputRawMode {
    fn new(stdin: &io::Stdin) -> io::Result<InputRawMode> {
        let fd = stdin.as_raw_fd();
        let mut termios = Termios::from_fd(fd)?;
        let orig = termios.clone();

        termios.c_cflag &= !(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
        termios.c_oflag &= !(OPOST);
        termios.c_cflag |= CS8;
        termios.c_iflag &= !(ECHO | ICANON | IEXTEN | ISIG);
        termios.c_cc[VMIN] = 1;
        termios.c_cc[VTIME] = 0;
        tcsetattr(fd, TCSAFLUSH, &mut termios)?;

        Ok(InputRawMode { fd, orig })
    }
}
impl Drop for InputRawMode {
    fn drop(&mut self) {
        tcsetattr(self.fd, TCSAFLUSH, &mut self.orig).unwrap();
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let _raw = InputRawMode::new(&stdin)?;
    for b in stdin.bytes() {
        let c = b? as char;
        println!("c:{}", c);
        if c == 'q' {
            break;
        }
    }
    Ok(())
}
