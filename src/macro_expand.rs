use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::ops::Add;
use crate::input_system::InputSystem;

#[derive(Debug)]
pub struct CompilerError {}

impl Display for CompilerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "something wrong when compiling.")
    }
}

type Result<T> = std::result::Result<T, CompilerError>;

pub struct MacroExpand {
    map: HashMap<String, String>,
}

impl MacroExpand {
    pub fn new() -> Self {
        MacroExpand {
            map: Default::default()
        }
    }
    pub fn add_macro(&mut self, m: String, v: String) {
        self.map.insert(m, v);
    }
    pub fn get_macro(&self, m: &str) -> Option<&String> {
        return self.map.get(m);
    }
    pub fn expand_macro(&self, m: &str) -> Result<String> {
        let v = self.get_macro(m);
        return match v {
            None => { Err(CompilerError{})}
            Some(v) => {
                self._expand_macro(v)
            }
        };
    }
    fn _expand_macro(&self, v: &String) -> Result<String> {
        let mut stack = vec![];
        let values = v.as_str().chars();
        let mut res = String::new();
        let mut template = String::new();
        let mut state = 0;
        for c in values{
            match c{
                '{'=>{
                    stack.push(c);
                    state = 1;
                }
                '}'=>{
                    if let None = stack.pop(){
                        return Err(CompilerError{})
                    }
                    match self._expand_macro(&template){
                        Ok(sub_macro)=>{
                            match self.get_macro(sub_macro.as_str()){
                                None => {return Err(CompilerError{})}
                                Some(s) => {res.push_str(s)}
                            }
                        }
                        Err(e)=>return Err(e)
                    }
                    state=0;
                    template.clear();
                }
                ch=>{
                    match state {
                        0 => {
                            res.push(ch);
                        }
                        1=>{
                            template.push(ch);
                        }

                        _ => {return Err(CompilerError{})}
                    }
                }
            }
        }
        if state!=0 {return Err(CompilerError{}) }
        Ok(res)
    }
    pub fn read_lines(&mut self, input: &mut Box<impl InputSystem>) -> Result<()> {
        while let Some(_) = input.current() {
            self.read_line(input)?
        }
        Ok(())
    }
    pub fn read_line(&mut self, input: &mut Box<impl InputSystem>) -> Result<()> {
        let mut key = None;
        let mut value = None;
        let mut state = 0;

        loop {
            match input.current() {
                None => { break; }
                Some('\n') => {
                    input.next();
                    break;
                }
                Some(' ') => {
                    if state == 1 { state = 2; }
                }
                Some(c) => {
                    match state {
                        0 => {
                            state = 1;
                            if key == None {
                                key = Some(String::new());
                            }
                            key = Some(key.unwrap().add(&c.to_string()));
                        }
                        1 => {
                            key = Some(key.unwrap().add(&c.to_string()));
                        }
                        2 => {
                            if value == None { value = Some(String::new()) }
                            value = Some(value.unwrap().add(&c.to_string()));
                        }
                        _ => {}
                    }
                }
            }
            println!("{}", input.current().unwrap());
            input.next();
        }
        match (key, value) {
            (Some(key), Some(value)) => {
                println!("{}:{}", key, value);
                self.add_macro(key, value);
            }
            _ => { return Err(CompilerError {}); }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::fs::OpenOptions;
    use crate::macro_expand::MacroExpand;
    use tempdir::TempDir;
    use crate::input_system::SimpleInputSystem;
    use std::io::Write;

    #[test]
    fn test_read_lines() {
        let mut macro_expand = MacroExpand::new();

        let dir = TempDir::new("test_create").unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = OpenOptions::new().create(true).write(true).open(file_path.clone()).unwrap();
        writeln!(file, "key1 value1").unwrap();

        writeln!(file, "key2 value2").unwrap();
        let mut input_system = SimpleInputSystem::new(file_path.to_str().unwrap().to_string());

        match macro_expand.read_lines(&mut Box::new(input_system)) {
            Ok(_) => {
                assert_eq!("value1", macro_expand.get_macro("key1").unwrap())
            }
            Err(e) => { panic!("{}", e) }
        }
    }
}