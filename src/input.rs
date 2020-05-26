
use sdl2::keyboard::{ Scancode, KeyboardState };
use sdl2::EventPump;

const NUM_KEYS_KEYPAD : u8 = 16;

type Keymap = [Scancode; NUM_KEYS_KEYPAD as usize];

const DEFAULT_KEY_MAPPING : Keymap = [ Scancode::Q, Scancode::W, Scancode::E, Scancode::R, Scancode::T,
                                       Scancode::Y, Scancode::U, Scancode::I, Scancode::O, Scancode::P,
                                       Scancode::A, Scancode::S, Scancode::D, Scancode::F, Scancode::G,
                                       Scancode::H
                                     ];

pub struct Keypad
{
    events : EventPump,
    keymap : Keymap,
}

impl Keypad
{
    pub fn new(context : &sdl2::Sdl) -> Result<Self, String>
    {
        let events = context.event_pump()?;
        let keymap = DEFAULT_KEY_MAPPING;

        Ok( Keypad { events, keymap } )
    }

    pub fn wait_for_key_pressed(&self) -> u8
    {
        loop
        {
            for hex in 0..NUM_KEYS_KEYPAD
            {
                if self.is_key_pressed(hex)
                {
                    return hex;
                }
            }
        }
    }

    pub fn is_key_pressed(&self, hex: u8) -> bool
    {
        let scan_code = self.to_scancode(hex);
        let keyboard  = self.events.keyboard_state();

        return keyboard.is_scancode_pressed(scan_code);
    }

    pub fn to_scancode(&self, hex: u8) -> Scancode
    {
        return self.keymap[hex as usize];
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::helpers::tests::*;

    #[test]
    fn keypad_to_scancode() -> Result<(), String>
    {
        let mutex = test_lock()?;

        let context = sdl2::init().unwrap();
        let keypad  = Keypad::new(&context)?;

        for hex in 0..NUM_KEYS_KEYPAD
        {
            assert_eq!(keypad.to_scancode(hex), keypad.keymap[hex as usize]);
        }

        Ok(())
    }
}
