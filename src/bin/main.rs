use std::fs::OpenOptions;
use tempdir::TempDir;
use std::io::Write;
use compiler::input_system::SimpleInputSystem;
use compiler::macro_expand::MacroExpand;

fn main() {
    let mut m = MacroExpand::new();
    m.add_macro(String::from("D"),String::from("[0-9]"));
    m.add_macro(String::from("regx"),String::from("{D}*a|{D}+"));
    match m.expand_macro("regx"){
        Ok(s) => {
            println!("{}",s)
        }
        Err(_) => {}
    }
}
