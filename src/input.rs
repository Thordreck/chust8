
use sdl2::keyboard::{ Scancode, KeyboardState };

const NUM_KEYS_KEYPAD : u8 = 16;

type Keymap = [Scancode; NUM_KEYS_KEYPAD as usize];

const DEFAULT_KEY_MAPPING : Keymap = [ Scancode::Q, Scancode::W, Scancode::E, Scancode::R, Scancode::T,
                                       Scancode::Y, Scancode::U, Scancode::I, Scancode::O, Scancode::P,
                                       Scancode::A, Scancode::S, Scancode::D, Scancode::F, Scancode::G,
                                       Scancode::H
                                     ];

pub struct Keypad<'lifetime>
{
    keyboard: KeyboardState<'lifetime>,
    keymap:   Keymap,
}

impl<'lifetime> Keypad<'lifetime>
{
    pub fn new(keyboard: KeyboardState<'lifetime>) -> Self
    {
        return Keypad { keyboard: keyboard, keymap: DEFAULT_KEY_MAPPING };
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
        return self.keyboard.is_scancode_pressed(scan_code);
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

    #[test]
    fn keypad_to_scancode()
    {
        let context     = sdl2::init().unwrap();
        let events_pump = context.event_pump().unwrap();

        let keypad  = Keypad::new(events_pump.keyboard_state());

        for hex in 0..NUM_KEYS_KEYPAD
        {
            assert_eq!(keypad.to_scancode(hex), keypad.keymap[hex as usize]);
        }
    }
}
