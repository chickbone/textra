extern crate termios;

use std::io::{self, Read};
use std::os::unix::io::{AsRawFd, RawFd};

struct InputRawMode {
    fd: RawFd,
    orig: termios::Termios,
}

impl InputRawMode {
    fn new(stdin: &io::Stdin) -> io::Result<InputRawMode> {
        use termios::*;

        let fd = stdin.as_raw_fd();
        let mut termios = Termios::from_fd(fd)?;
        let orig = termios.clone();

        termios.c_cflag &= !(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
        termios.c_oflag &= !(OPOST);
        termios.c_cflag |= CS8;
        termios.c_iflag &= !(ECHO | ICANON | IEXTEN | ISIG);
        termios.c_cc[VMIN] = 0;
        termios.c_cc[VTIME] = 1;
        tcsetattr(fd, TCSAFLUSH, &mut termios)?;

        Ok(InputRawMode { fd, orig })
    }
}
impl Drop for InputRawMode {
    fn drop(&mut self) {
        termios::tcsetattr(self.fd, termios::TCSAFLUSH, &mut self.orig).unwrap();
    }
}

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let _raw = InputRawMode::new(&stdin)?;
    let mut one_byte: [u8; 1] = [0];

    loop {
        let size = stdin.read(&mut one_byte)?;
        let c = if size > 0 { one_byte[0] as char } else { '\0' };

        if c.is_control() {
            print!("{}\r\n", c);
        } else {
            print!("char: {}\r\n", c);
        }

        if c == 'q' {
            break;
        }
    }
    Ok(())
}
