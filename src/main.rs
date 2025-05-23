#![feature(if_let_guard)]
// TODO: Re-enable warnings when the language is more mature
#![allow(unused_variables)]
#![allow(dead_code)]

mod asm;
mod common;
mod compile;
mod error;
mod lexer;
mod type_check;

use compile::compile_assembly_code;
use std::io;
use std::path::Path;
use type_check::type_check_program;

const ASSEMBLY_FILE: &str = "test.asm";
const CODE_FILE: &str = "test.casa";

fn main() -> io::Result<()> {
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let code_file = crate_dir.join(CODE_FILE);
    let segments = lexer::parse_code_file(&code_file);
    dbg!(&segments);

    type_check_program(&segments);

    let assembly_code = asm::generate_assembly_code(&segments);
    let assembly_file = crate_dir.join(ASSEMBLY_FILE);
    std::fs::write(&assembly_file, format!("{}\n", assembly_code))?;
    println!("{}", &assembly_code);

    compile_assembly_code(&assembly_file)?;

    Ok(())
}
