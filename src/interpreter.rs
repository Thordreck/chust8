
use std::fs;

use sdl2::Sdl;

use crate::memory::{ Ram, ProgramCounter };
use crate::opcodes::OpCode;
use crate::display::Display;
use crate::input::Keypad;
use crate::audio::Speakers;

//mod instructions;

pub struct Interpreter
{
    ram      : Ram,
    pc       : ProgramCounter,
    context  : Sdl,
    display  : Display,
    keypad   : Keypad,
    speakers : Speakers,
}

// Public
impl Interpreter
{
    pub fn new() -> Result<Self, String>
    {
        let context  = sdl2::init()?; 
        let ram      = Ram::new();
        let pc       = ProgramCounter::new();
        let display  = Display::from_context(&context)?;
        let keypad   = Keypad::new(&context)?;
        let speakers = Speakers::new(&context)?;

        Ok(Self { ram, pc, context, display, keypad, speakers })
    }

    pub fn load_rom(&mut self, rom_file: &str) -> Result<(), String>
    {
        let contents = match fs::read(rom_file)
        {
            Ok(data) => data,
            Err(e)   => return Err(e.to_string()),
        };

        self.ram.dump(&contents)?;
        Ok(())
    }

    pub fn start(&mut self) -> Result<(), String>
    {
        /*
        'codes: loop
        {


        }
        */

        self.cpu_cycle()?;

        Ok(())
    }
}

// Private
impl Interpreter
{
    fn cpu_cycle(&mut self) -> Result<(), String>
    {
        // Extract two bytes from ram and parse the opcode
        let (msb, lsb) = self.extract_opcode_bytes()?;
        let opcode = OpCode::new(msb, lsb)?;

        // Get and execute the associated instruction
        //instructions::execute_opcode(opcodes::OpCode::_0NNN(2), self)?;
        //instructions::execute_opcode(opcodes::OpCode::_3XNN(2, 3), self)?;

        Ok(())
    }

    fn extract_opcode_bytes(&mut self) -> Result<(u8, u8), String>
    {
        let memory = self.ram.peek();

        let msb = memory[self.pc.value()];
        self.pc.advance(None)?;

        let lsb = memory[self.pc.value()];
        self.pc.advance(None)?;

        Ok((msb, lsb))
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::helpers::tests::*;

    #[test]
    fn test_load_rom() -> Result<(), String>
    {
        let mutex = test_lock()?;

        let mut interpreter = Interpreter::new()?;

        // Test loading a file that does not exist
        let invalid_load_result = interpreter.load_rom("invalid_file.ch8");
        assert!(invalid_load_result.is_err());

        Ok(())
    }

    #[test]
    fn test_cpu_cycle() -> Result<(), String>
    {
        let mutex = test_lock()?;

        let mut interpreter = Interpreter::new()?;

        interpreter.cpu_cycle()?;
        Ok(())
    }

}
