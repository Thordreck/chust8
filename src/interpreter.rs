
use std::fs;

use sdl2::Sdl;

use crate::memory::{ Ram, ProgramCounter };
use crate::opcodes::OpCode;
use crate::display::Display;
use crate::input::Keypad;
use crate::audio::Speakers;
use crate::registers::{ IRegister, DataRegister, AllDataRegisters };
use crate::stack::Stack;
use crate::timer::Timer;
use crate::clock::*;

//mod instructions;

pub struct Interpreter
{
    ram            : Ram,
    pc             : ProgramCounter,
    context        : Sdl,
    display        : Display,
    keypad         : Keypad,
    speakers       : Speakers,
    i_register     : IRegister,
    data_registers : AllDataRegisters,
    stack          : Stack,
    cpu_limiter    : RateLimiter,
    timer_limiter  : RateLimiter,
    delay_timer    : Timer,
    sound_timer    : Timer,
}

// Public
impl Interpreter
{
    pub fn new() -> Result<Self, String>
    {
        let context        = sdl2::init()?; 
        let ram            = Ram::new();
        let pc             = ProgramCounter::new();
        let display        = Display::from_context(&context)?;
        let keypad         = Keypad::new(&context)?;
        let speakers       = Speakers::new(&context)?;
        let i_register     = IRegister::new();
        let data_registers = DataRegister::all();
        let stack          = Stack::new();
        let cpu_limiter    = RateLimiter::new(DEFAULT_CPU_FREQUENCY);
        let timer_limiter  = RateLimiter::new(DEFAULT_TIMERS_FREQUENCY);
        let delay_timer    = Timer::new();
        let sound_timer    = Timer::new();

        let mut interpreter = Interpreter
                {
                  ram, pc, context, display, keypad,
                  speakers, i_register, data_registers,
                  stack, cpu_limiter, timer_limiter,
                  delay_timer, sound_timer
                };

        Ok(interpreter)
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
        Ok(())
    }

    fn cpu_step(&mut self) -> Result<(), String>
    {
        // Extract two bytes from ram and parse the opcode
        let (msb, lsb) = self.extract_opcode_bytes()?;
        let opcode = OpCode::new(msb, lsb)?;

        println!("Cpu step done!");
        // Get and execute the associated instruction
        //instructions::execute_opcode(opcodes::OpCode::_0NNN(2), self)?;
        //instructions::execute_opcode(opcodes::OpCode::_3XNN(2, 3), self)?;

        Ok(())
    }

    fn tick_timers(&mut self) -> Result<(), String>
    {
        //self.delay_timer.unwrap().tick()?;
        //self.sound_timer.unwrap().tick()?;

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
