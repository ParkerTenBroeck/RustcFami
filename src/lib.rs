use interpreter::Interpreter;

#[macro_use]
extern crate bitfield;

pub mod tokenizer;
pub mod parser;
pub mod interpreter;
pub mod sound_file;
pub mod hardware_interface;




pub mod tests{
    use crate::interpreter::Interpreter;


    #[test]
    pub fn test(){
        let str = std::fs::read("res/sega_tetris_theme_v2.txt").unwrap();
        let str = str.as_slice();
        let str = std::str::from_utf8(str).unwrap();
        match crate::parser::read_text(str){
            Ok(info) => {
                //println!("{:#?}", info);
                let int = Interpreter::new(&info);
                std::thread::sleep(std::time::Duration::from_millis(10000));
            },
            Err(err) => {
                println!("{:#?}", err);
            },
        }
    }
}