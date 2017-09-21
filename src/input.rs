extern crate sdl2;
use self::sdl2::event::Event;
use self::sdl2::keyboard::Scancode;

pub struct Input {
    input_driver : sdl2::EventPump
}

impl Input {
    pub fn new(context: &sdl2::Sdl) -> Input {
        Input {
            input_driver: context.event_pump().unwrap()
        }
    }

    pub fn poll(&mut self) -> Result<Vec<bool>, &'static str> {
        let mut events = vec![false; 16];

        for event in self.input_driver.poll_iter() {
            if let Event::Quit {..} = event {
               return Err("User exited program!")
            }
        };

        let state : Vec<sdl2::keyboard::Scancode> = self.input_driver.keyboard_state().pressed_scancodes().collect();

        for key in state {
            match key {
                Scancode::B => {
                    events[0x0] = true;
                }
                Scancode::Num4 => {
                    events[0x1] = true;
                },
                Scancode::Num5 => {
                    events[0x2] = true;
                },
                Scancode::Num6 => {
                    events[0x3] = true;
                },
                Scancode::R => {
                    events[0x4] = true;
                },
                Scancode::T => {
                    events[0x5] = true;
                },
                Scancode::Y => {
                    events[0x6] = true;
                },
                Scancode::F => {
                    events[0x7] = true;
                },
                Scancode::G => {
                    events[0x8] = true;
                },
                Scancode::H => {
                    events[0x9] = true;
                },
                Scancode::V => {
                    events[0xA] = true;
                },
                Scancode::N => {
                    events[0xB] = true;
                },
                Scancode::Num7 => {
                    events[0xC] = true;
                },
                Scancode::U => {
                    events[0xD] = true;
                },
                Scancode::J => {
                    events[0xE] = true;
                },
                Scancode::M => {
                    events[0xF] = true;
                }
                _ => continue
            }
        }

        Ok(events)
    }
}