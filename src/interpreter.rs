
use std::fs;

use crate::memory::{ Ram, ProgramCounter };
use crate::opcodes::OpCode;

//mod instructions;

pub struct Interpreter
{
    ram : Ram,
    program_counter : ProgramCounter,
}

// Public
impl Interpreter
{
    pub fn new() -> Self
    {
        return Self { ram: Ram::new(), program_counter: ProgramCounter::new() };
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

        let msb = memory[self.program_counter.value()];
        self.program_counter.advance(None)?;

        let lsb = memory[self.program_counter.value()];
        self.program_counter.advance(None)?;

        Ok((msb, lsb))
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_load_rom()
    {
        let mut interpreter = Interpreter::new();

        // Test loading a file that does not exist
        let invalid_load_result = interpreter.load_rom("invalid_file.ch8");
        assert!(invalid_load_result.is_err());
    }

    #[test]
    fn test_cpu_cycle()
    {
        let mut interpreter = Interpreter::new();

        interpreter.cpu_cycle();
    }

}
