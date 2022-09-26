use inkwell::context::Context;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let context = Context::create();

    Ok(())
}