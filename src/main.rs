
use std::env;
use chust8::interpreter::Interpreter;

fn main() -> Result<(), String>
{
    let args: Vec<String> = env::args().collect();

    let rom_file = match args.len()
    {
        1 => {
                println!("Missing rom file. Usage is {} /path/to/rom", args[0]);
                std::process::exit(-1);
             },
        _ => args[1].clone(),
    };

    let mut interpreter = Interpreter::new();
    interpreter.load_rom(&rom_file)?;

    Ok(())
}
