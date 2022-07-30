use rustc_fami::{parser, interpreter::Interpreter};

pub fn main(){
    let str = std::fs::read("res/sega_tetris_theme_v2.txt").unwrap();
        let str = str.as_slice();
        let str = std::str::from_utf8(str).unwrap();
        match parser::read_text(str){
            Ok(info) => {
                //println!("{:#?}", info);
                let int = Interpreter::new(&info);
                std::thread::sleep(std::time::Duration::from_millis(10000));
                println!("asdasdasdasd");
            },
            Err(err) => {
                println!("{:#?}", err);
            },
        }
}