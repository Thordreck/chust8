
use super::Interpreter;
use crate::opcodes::OpCode;


pub fn execute_opcode(opcode : OpCode, interpreter : &Interpreter) -> Result<(), String>
{
    use OpCode::*;
    match opcode
    {
        _0NNN(nnn)      => execute_0NNN(interpreter, nnn)?,
        _00EE           => execute_00EE(interpreter)?,
        _00E0           => execute_00E0(interpreter)?,
        _1NNN(u16),
        _2NNN(u16),
        _3XNN(u8, u8),
        _4XNN(u8, u8),
        _5XY0(u8, u8),
        _6XNN(u8, u8),
        _7XNN(u8, u8),
        _8XY0(u8, u8),
        _8XY1(u8, u8),
        _8XY2(u8, u8),
        _8XY3(u8, u8),
        _8XY4(u8, u8),
        _8XY5(u8, u8),
        _8XY6(u8, u8),
        _8XY7(u8, u8),
        _8XYE(u8, u8),
        _9XY0(u8, u8),
        _ANNN(u16),
        _BNNN(u16),
        _CXNN(u8, u8),
        _DXYN(u8, u8, u8),
        _EX9E(u8),
        _EXA1(u8),
        _FX07(u8),
        _FX0A(u8),
        _FX15(u8),
        _FX18(u8),
        _FX1E(u8),
        _FX29(u8),
        _FX33(u8),
        _FX55(u8),
        _FX65(u8),
    };

    Ok(())
}

fn execute_0NNN(interpreter : &mut Interpreter, nnn : u16) -> Result<(), String>
{
    Err(String::from("Instruction 0NNN not supported"))
}

fn execute_00EE(interpreter : &mut Interpreter) -> Result<(), String>
{
    Err(String::from("Instruction 00EE not implemented"))
}

fn execute_00E0(interpreter : &mut Interpreter, nnn : u16) -> Result<(), String>
{
    Err(String::from("Instruction 0NNN not implemented"))
}

#[cfg(test)]
mod tests
{
    use super::*;

    /*
    #[test]
    fn opcode_parsing() -> Result<(), String>
    {
        // TODO: implement this test properly
        let result = parse_opcode(0x00, 0xEE)?;

        assert_eq!(result, OpCode::_00EE);

        Ok(())
    }
    */
}
