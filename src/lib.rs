
pub mod tokenizer;
pub mod parser;


pub mod tests{

    #[test]
    pub fn test(){
        let str = std::fs::read("res/Castlevania 3 OST[WIP].txt").unwrap();
        let str = str.as_slice();
        let str = std::str::from_utf8(str).unwrap();
        match crate::parser::read_text(str){
            Ok(info) => {
                println!("{:#?}", info);
            },
            Err(err) => {
                println!("{:#?}", err);
            },
        }
    }
}