
use strum_macros::EnumCount; // to get number of opcodes (OPCODE_COUNT variable)

// PartialEq and Debug needed to test values
#[derive(PartialEq, Debug)]
#[derive(EnumCount)]
pub enum OpCode
{
    _0NNN(u16),
    _00EE,
    _00E0,
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
}

impl OpCode
{
    pub fn new(msb : u8, lsb : u8) -> Result<Self, String>
    {
        let splitted_bytes = (msb >> 4, msb & 0x0F, lsb >> 4, lsb & 0x0F);

        use OpCode::*;

        match splitted_bytes
        {
            (0x0, 0x0, 0xE, 0xE) => Ok(_00EE),
            (0x0, 0x0, 0xE, 0x0) => Ok(_00E0),
            (0x0, n1,  n2,  n3 ) => Ok(_0NNN(to_u16(n1, n2, n3))),
            (0x1, n1,  n2,  n3 ) => Ok(_1NNN(to_u16(n1, n2, n3))),
            (0x2, n1,  n2,  n3 ) => Ok(_2NNN(to_u16(n1, n2, n3))),
            (0x3, x,   n1,  n2 ) => Ok(_3XNN(x, to_u8(n1, n2))),
            (0x4, x,   n1,  n2 ) => Ok(_4XNN(x, to_u8(n1, n2))),
            (0x5, x,   y,   0x0) => Ok(_5XY0(x, y)),
            (0x6, x,   n1,  n2 ) => Ok(_6XNN(x, to_u8(n1, n2))),
            (0x7, x,   n1,  n2 ) => Ok(_7XNN(x, to_u8(n1, n2))),
            (0x8, x,   y,   0x0) => Ok(_8XY0(x, y)),
            (0x8, x,   y,   0x1) => Ok(_8XY1(x, y)),
            (0x8, x,   y,   0x2) => Ok(_8XY2(x, y)),
            (0x8, x,   y,   0x3) => Ok(_8XY3(x, y)),
            (0x8, x,   y,   0x4) => Ok(_8XY4(x, y)),
            (0x8, x,   y,   0x5) => Ok(_8XY5(x, y)),
            (0x8, x,   y,   0x6) => Ok(_8XY6(x, y)),
            (0x8, x,   y,   0x7) => Ok(_8XY7(x, y)),
            (0x8, x,   y,   0xE) => Ok(_8XYE(x, y)),
            (0x9, x,   y,   0x0) => Ok(_9XY0(x, y)),
            (0xA, n1,  n2,  n3 ) => Ok(_ANNN(to_u16(n1, n2, n3))),
            (0xB, n1,  n2,  n3 ) => Ok(_BNNN(to_u16(n1, n2, n3))),
            (0xC, x,   n1,  n2 ) => Ok(_CXNN(x, to_u8(n1, n2))),
            (0xD, x,   y,   n  ) => Ok(_DXYN(x, y, n)),
            (0xE, x, 0x9,   0xE) => Ok(_EX9E(x)),
            (0xE, x, 0xA,   0x1) => Ok(_EXA1(x)),
            (0xF, x, 0x0,   0x7) => Ok(_FX07(x)),
            (0xF, x, 0x0,   0xA) => Ok(_FX0A(x)),
            (0xF, x, 0x1,   0x5) => Ok(_FX15(x)),
            (0xF, x, 0x1,   0x8) => Ok(_FX18(x)),
            (0xF, x, 0x1,   0xE) => Ok(_FX1E(x)),
            (0xF, x, 0x2,   0x9) => Ok(_FX29(x)),
            (0xF, x, 0x3,   0x3) => Ok(_FX33(x)),
            (0xF, x, 0x5,   0x5) => Ok(_FX55(x)),
            (0xF, x, 0x6,   0x5) => Ok(_FX65(x)),
            _ => Err(format!("Unrecognized instruction bytes {:x}, {:x}", msb, lsb))
        }
    }

    pub fn disassembly(&self) -> String
    {
        use OpCode::*;

        match *self
        {
            _0NNN(nnn)     => format!("JUMP_MACHINE {:#X}", nnn),
            _00EE          => format!("RETURN"),
            _00E0          => format!("CLEAR"),
            _1NNN(nnn)     => format!("JUMP {:#X}", nnn),
            _2NNN(nnn)     => format!("CALL {:#X}", nnn),
            _3XNN(x, nn)   => format!("SKIP_IF_EQ v{}, {}", x, nn),
            _4XNN(x, nn)   => format!("SKIP_IF_NEQ v{}, {}", x, nn),
            _5XY0(x, y)    => format!("SKIP_IF_EQ v{}, v{}", x, y),
            _6XNN(x, nn)   => format!("MOV v{}, {}", x, nn),
            _7XNN(x, nn)   => format!("ADD v{}, {}", x, nn),
            _8XY0(x, y)    => format!("MOV v{}, v{}", x, y),
            _8XY1(x, y)    => format!("OR v{}, v{}", x, y),
            _8XY2(x, y)    => format!("AND v{}, v{}", x, y),
            _8XY3(x, y)    => format!("XOR v{}, v{}", x, y),
            _8XY4(x, y)    => format!("ADD v{}, v{}", x, y),
            _8XY5(x, y)    => format!("SUB v{}, v{}", x, y),
            _8XY6(x, _)    => format!("RSHIFT v{}", x),
            _8XY7(x, y)    => format!("MOV v{0}, v{1} - v{0}", x, y),
            _8XYE(x, _)    => format!("LSHIFT v{}", x),
            _9XY0(x, y)    => format!("SKIP_IF_NEQ v{}, v{}", x, y),
            _ANNN(nnn)     => format!("MOV I, {:#X}", nnn),
            _BNNN(nnn)     => format!("JUMP_V0 {:#X}", nnn),
            _CXNN(x, nn)   => format!("RAND v{}, {}", x, nn),
            _DXYN(x, y, n) => format!("DRAW v{}, v{}, {}", x, y , n),
            _EX9E(x)       => format!("SKIP_IF_KEY v{}", x),
            _EXA1(x)       => format!("SKIP_IF_NOT_KEY v{}", x),
            _FX07(x)       => format!("MOV v{}, DELAY", x),
            _FX0A(x)       => format!("MOV v{}, KEY", x),
            _FX15(x)       => format!("MOV DELAY, v{}", x),
            _FX18(x)       => format!("MOV v{}, SOUND", x),
            _FX1E(x)       => format!("ADD I, v{}", x),
            _FX29(x)       => format!("MOV I, v{}", x),
            _FX33(x)       => format!("BIN I, v{}", x),
            _FX55(x)       => format!("BATCH I, v{}", x),
            _FX65(x)       => format!("BATCH v{}, I", x),
        }
    }
}

// Private helper functions
fn to_u8(n1 : u8, n2 : u8) -> u8
{
    n1 << 4 | n2
}

fn to_u16(n1 : u8, n2 : u8, n3 : u8) -> u16
{
    (n1 as u16) << 8 | (n2 as u16) << 4 | (n3 as u16)
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn parsing() -> Result<(), String>
    {
        // TODO: implement this test properly
        // Probably using a part of a real rom
        let opcode = OpCode::new(0x00, 0xEE)?;

        assert_eq!(opcode, OpCode::_00EE);

        Ok(())
    }

    #[test]
    fn disassembly()
    {
        use OpCode::*;

        static DISASSEMBLY_TABLE : [(OpCode, &str); OPCODE_COUNT] =
        [
           (_0NNN(333),     "JUMP_MACHINE 0x14D"),
           (_00EE,          "RETURN"),
           (_00E0,          "CLEAR"),
           (_1NNN(137),     "JUMP 0x89"),
           (_2NNN(279),     "CALL 0x117"),
           (_3XNN(3, 58),   "SKIP_IF_EQ v3, 58"),
           (_4XNN(4, 69),   "SKIP_IF_NEQ v4, 69"),
           (_5XY0(3, 5),    "SKIP_IF_EQ v3, v5"),
           (_6XNN(8, 99),   "MOV v8, 99"),
           (_7XNN(0, 25),   "ADD v0, 25"),
           (_8XY0(7, 8),    "MOV v7, v8"),
           (_8XY1(1, 9),    "OR v1, v9"),
           (_8XY2(10, 12),  "AND v10, v12"),
           (_8XY3(7, 14),   "XOR v7, v14"),
           (_8XY4(3, 0),    "ADD v3, v0"),
           (_8XY5(3, 8),    "SUB v3, v8"),
           (_8XY6(5, 8),    "RSHIFT v5"),
           (_8XY7(0, 8),    "MOV v0, v8 - v0"),
           (_8XYE(7, 9),    "LSHIFT v7"),
           (_9XY0(3, 9),    "SKIP_IF_NEQ v3, v9"),
           (_ANNN(0x340),   "MOV I, 0x340"),
           (_BNNN(0x800),   "JUMP_V0 0x800"),
           (_CXNN(6, 69),   "RAND v6, 69"),
           (_DXYN(0, 2, 7), "DRAW v0, v2, 7"),
           (_EX9E(0),       "SKIP_IF_KEY v0"),
           (_EXA1(1),       "SKIP_IF_NOT_KEY v1"),
           (_FX07(2),       "MOV v2, DELAY"),
           (_FX0A(3),       "MOV v3, KEY"),
           (_FX15(4),       "MOV DELAY, v4"),
           (_FX18(5),       "MOV v5, SOUND"),
           (_FX1E(6),       "ADD I, v6"),
           (_FX29(7),       "MOV I, v7"),
           (_FX33(8),       "BIN I, v8"),
           (_FX55(9),       "BATCH I, v9"),
           (_FX65(10),      "BATCH v10, I"),
        ];

        for (opcode, disassembly) in DISASSEMBLY_TABLE.iter()
        {
            assert_eq!(disassembly, &&opcode.disassembly()[..],
                       "Original opcode {:?}", opcode);
        }
    }
}
