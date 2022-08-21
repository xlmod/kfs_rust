
use core::str::from_utf8;
use crate::{
    keyboard,
    kprint,
    kprintln,
};

mod command;

const CMD_SIZE: usize = 1024;

struct Command {
    buffer: [u8; 1024],
    index: usize,
}

impl Command {
    fn new() -> Self {
        Self { buffer: [0; 1024], index: 0 }
    }

    fn get(&self) -> &str {
        from_utf8(&self.buffer[0..self.index]).unwrap()
    }

    fn read(&mut self) {
        loop {
            let key = keyboard::Key::get_key();
            match key.ascii_character {
                b'\x08' => {if self.index != 0 {
                        self.index -= 1;
                        self.buffer[self.index] = 0;
                        kprint!("\x08");
                }},
                b'\x0a' => {kprintln!(); break},
                b'\x09' => {},
                _ => {if self.index != CMD_SIZE {
                        self.buffer[self.index] = key.ascii_character;
                        self.index += 1;
                        kprint!("{}",
                            from_utf8(&[key.ascii_character]).unwrap());
                }},
            }
        }
    }

}

pub fn kshell() {
    kprintln!("Welcome to Kfs-{}", crate::VERSION);
    kprintln!();
    loop {
        let mut cmd = Command::new();
        kprint!("kshell# ");
        cmd.read();
        match cmd.get() {
            "exit" => { command::exit(); break},
            "shutdown" => command::shutdown(),
            "reboot" => command::reboot(),
            "clear" => command::clear_vt(),
            "next" => command::next_vt(),
            "help" => command::help(),
            "info" => command::info(),
            "read_serial" => command::read_serial(),
            "write_serial" => command::write_serial(),
            _ => {},
        }
    }
}
