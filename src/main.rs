
use std::env;
use chust8::interpreter::Interpreter;

fn main() -> Result<(), String>
{
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1
    {
        return Err(format!("Missing rom file. Usage is {} /path/to/rom", args[0]));
    }

    let mut interpreter = Interpreter::new()?;
    interpreter.load_rom(&args[1])?;

    Ok(())
}
