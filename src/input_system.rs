pub trait InputSystem{
    //丢弃当前字符，指向下一个
    fn next(&mut self);
    //获取当前字符
    fn current(&self)->Option<char>;
    //获取下一个字符，但是保留当前读取位置

    fn look_forward(&self)->Option<char>;
}

pub struct SimpleInputSystem{
    pub buffer:String,
    pub pos:usize,
}
use std::{fs::OpenOptions, io::Read};
impl SimpleInputSystem{
    pub fn new(file_path:String)->SimpleInputSystem{
        let mut file = OpenOptions::new().read(true).open(file_path).unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        SimpleInputSystem{
            buffer,
            pos:0,
        }

    }
}
//为simple_input_system实现InputSystem接口
impl InputSystem for SimpleInputSystem{
    fn next(&mut self){
        self.pos += 1;
    }
    fn current(&self)->Option<char>{
        self.buffer.chars().nth(self.pos)    }
    fn look_forward(&self)->Option<char>{
        self.buffer.chars().nth(self.pos+1)    }
}
#[cfg(test)]
mod test{
    use tempdir::TempDir;
    use std::fs::{OpenOptions};
    use std::io::Write;
    use crate::input_system::InputSystem;
    #[test]
    fn test_create(){
        let dir = TempDir::new("test_create").unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = OpenOptions::new().create(true).write(true).open(file_path.clone()).unwrap();
        write!(file,"he").unwrap();
        let mut input_system = super::SimpleInputSystem::new(file_path.to_str().unwrap().to_string());
        assert_eq!(input_system.current(),Some('h'));
        assert_eq!(input_system.look_forward(),Some('e'));
        input_system.next();    

        assert_eq!(input_system.current(),Some('e'));
        assert_eq!(input_system.look_forward(),None);
        input_system.next();
        assert_eq!(input_system.current(),None);
    }
}
