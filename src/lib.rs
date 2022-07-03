use std::{error::Error};

use tokenizer::Tokenizer;

pub mod tokenizer;

#[derive(Default)]
struct SoundFile{
    title: String,
    author: String,
    copyright: String,
    comment: String,

    machine: u32,
    expansion: u32,
    vibrato: u32,
    playbackrate: (u32, u32),
    tuning: (u32, u32),
}

#[derive(Default)]
struct SongPattern{
    id: u32,
    rows: Vec<SongRow>,
}

#[derive(Default)]
struct SongRow{

}

pub fn main(){

}

pub fn read_text(file: String) -> Result<(), Box<dyn Error>>{
    let res = std::fs::read(file)?;
    let str = std::str::from_utf8(res.as_slice())?;
    
    let mut file = SoundFile::default();
    let mut tokenizer = Tokenizer::from_str(str);
    while let Option::Some(token) = tokenizer.next(){
        println!("{:?}", token);

        if true{
            continue;
        }
        match token{
            tokenizer::Token::Ident(ident) => {
                match ident.as_str(){
                    "TITLE" => {
                        match tokenizer.next(){
                            Some(_) => todo!(),
                            None => todo!(),
                        }
                    }
                    _ => {
                        return Result::Err(format!("Unknown command: {}", ident).into());
                    }
                }
            },
            tokenizer::Token::Number(_) => todo!(),
            tokenizer::Token::Colon => todo!(),
            tokenizer::Token::String(_) => todo!(),
            tokenizer::Token::Note(_) => todo!(),
            tokenizer::Token::Dot => todo!(),
            tokenizer::Token::DotDot => todo!(),
            tokenizer::Token::DotDotDot => todo!(),
            tokenizer::Token::NewLine => {
                //skip
            },
            tokenizer::Token::Empty => return Result::Err("Given empty??".into()),
            tokenizer::Token::Error(char, location) => return Result::Err(format!("Unreconized char: {:?} at {:?}", char, location).into()),
        }
    }

    Result::Ok(())
}

pub mod tests{
    use crate::read_text;


    #[test]
    pub fn test(){
        
        read_text("res/Castlevania 3 OST[WIP].txt".into()).unwrap();
    }
}