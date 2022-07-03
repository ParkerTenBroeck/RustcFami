use std::{iter::Peekable, str::Chars};

enum TokenizerState{
    Default,
    Comment,
    Ident,
    String,
    Dot,
    DotDot,
    Number,
    Note,
    NewLine,
}
#[derive(Copy, Clone, Default, Debug)]
pub struct Location{
    real: usize,
    seen: usize,
}

pub struct Tokenizer<'a>{
    str: &'a str,
    iter: Peekable<Chars<'a>>,
    state: TokenizerState,
    start: Location,
    end: Location,
}

impl<'a> Tokenizer<'a>{
    pub fn from_str(str: &'a str) -> Self{
        Self{
            iter: str.chars().peekable(),
            str,
            state: TokenizerState::Default,
            start: Default::default(),
            end: Default::default(),
        }
    }

    fn str_last(&self) -> &str{
        //&self.str[self.start.real..(self.end.real)]
        self.str_loc_loc(self.start, self.end)
    }

    fn str_loc_loc(&self, start: Location, end: Location) -> &str{
        &self.str[start.real..(end.real)]
    }
}

impl<'a> Iterator for Tokenizer<'a>{
    type Item = Token;


    fn next(&mut self) -> Option<Self::Item> {


        loop{
            let mut consume = true;
            let mut ret = Option::None;
            let c;
            match self.iter.peek(){
                Some(char) => {
                    c = *char;
                },
                None => {
                    return Option::None
                },
            }
    
            match self.state{
    
                TokenizerState::Default => {
                    match c{
                        'a'..='z'|'A'..='Z' => {
                            self.state = TokenizerState::Ident;
                        }
                        '0'..='9'|'-' => {
                            self.state = TokenizerState::Number;
                        }
                        '#' => {
                            self.state = TokenizerState::Comment;
                        }
                        '"' => {
                            self.state = TokenizerState::String;
                        }
                        ':' => {
                            ret = Option::Some(Token::Colon);
                        }
                        '.' => {
                            self.state = TokenizerState::Dot;
                        }
                        '\n'|'\r' => {
                            self.state = TokenizerState::NewLine;
                        }
                        _ =>{
                            if c.is_whitespace(){
                                ret = Option::Some(Token::Empty);
                            }else{
                                ret = Option::Some(Token::Error(c, Location{real: self.end.real + c.len_utf8(), seen: self.end.seen + 1}));
                            }
                        }
                    }
                },
                TokenizerState::Comment => {
                    match c{
                        '\n' => {
                            ret = Option::Some(Token::Empty);
                            
                            self.state = TokenizerState::Default;
                        }
                        _ =>{
                            //continue
                        }
                    }
                },
                TokenizerState::Ident => {
                    match c{
                        'a'..='z'|'A'..='Z' => {
                            //continue
                        }
                        '-'|'#' => {
                            self.state = TokenizerState::Note;
                        }
                        _ =>{
                            self.state = TokenizerState::Default;
                            ret = Option::Some(Token::Ident(self.str_last().into()));
                            consume = false;
                        }
                    }
                },
                TokenizerState::String => {
                    match c{
                        '"' => {
                            self.state = TokenizerState::Default;
                            let str = self.str_last();
                            let str = &str[1..str.len()];
                            ret = Option::Some(Token::String(str.into()))
                        
                        }
                        _ => {
                            //continue
                        }
                    }
                },
                TokenizerState::Dot => {
                    match c{
                        '.' => {
                            self.state = TokenizerState::DotDot;
                        }
                        _ => {
                            self.state = TokenizerState::Default;
                            ret = Option::Some(Token::Dot);
                            consume = false;
                        }
                    }
                },
                TokenizerState::DotDot => {
                    match c{
                        '.' => {
                            self.state = TokenizerState::Default;
                            ret = Option::Some(Token::DotDotDot);
                        }
                        _ => {
                            self.state = TokenizerState::Default;
                            ret = Option::Some(Token::DotDot);
                            consume = false;
                        }
                    }
                },
                TokenizerState::Number => {
                    match c{
                        '0'..='9'|'a'..='f'|'A'..='F' => {}
                        _ => {
                            let str = self.str_last();
                            let mut num = FamiNumber{
                                dec: 0,
                                hex: 0,
                                effect: ('\0', 0),
                            };
                            match i32::from_str_radix(str, 10){
                                Ok(val) => num.dec = val,
                                Err(_) => {},
                            }
                            match u32::from_str_radix(str, 16){
                                Ok(val) => num.hex = val,
                                Err(_) => {},
                            }
                            if str.len() == 3{
                                match u8::from_str_radix(&str[1..str.len()], 16){
                                    Ok(val) => {
                                        num.effect.1 = val;
                                        num.effect.0 = str.chars().next().unwrap();
                                    }
                                    Err(_) => {

                                    }
                                }
                            }
                            self.state = TokenizerState::Default;
                            ret = Option::Some(Token::Number(num));
                            consume = false;
                        }
                    }
                },
                TokenizerState::NewLine => {
                    match c{
                        '\n'|'\r' => {
                            //continue
                        }
                        _ => {
                            consume = false;
                            self.state = TokenizerState::Default;
                            ret = Option::Some(Token::NewLine)
                        }
                    }
                },
                TokenizerState::Note => {
                    match c{
                        '0'..='7' => {
                            self.state = TokenizerState::Default;
                            let end = Location{real: self.end.real + c.len_utf8(), seen: self.end.seen + 1};
                            let str = self.str_loc_loc(self.start, end);
                            let off = (str.as_bytes()[2] as u32 - '0' as u32 + 1) * 12;
                            let note = &str[0..2];
                            let note = match note{
                                "C-" => {
                                    0
                                }
                                "C#" => {
                                    1
                                }
                                "D-" => {
                                    2
                                }
                                "D#" => {
                                    3
                                }
                                "E-" => {
                                    4
                                }
                                "F-" => {
                                    5
                                }
                                "F#" => {
                                    6
                                }
                                "G-" => {
                                    7
                                }
                                "G#" => {
                                    8
                                }
                                "A-" => {
                                    9
                                }
                                "A#" => {
                                    10
                                }
                                "B-" => {
                                    11
                                }
                                _ => {
                                    panic!("{} {}", str, note);
                                }
                            };
                            ret = Option::Some(Token::Note(off + note));
                        }
                        _ => {
                            self.state = TokenizerState::Default;
                            ret = Option::Some(Token::Error(c, Location{real: self.end.real + c.len_utf8(), seen: self.end.seen + 1}));
                        }
                    }
                },
            }
            if consume{
                let char = self.iter.next().unwrap();
                self.end.real += char.len_utf8();
                self.end.seen += 1;
            }
            match ret{
                Some(val) => {
                    self.start = self.end;
                    match val{
                        Token::Empty => {}
                        _ => {return Option::Some(val)}
                    }
                },
                None => {},
            }
        }
    }
}

#[derive(Debug)]
pub enum Token{
    Ident(String),
    Number(FamiNumber),
    Colon,
    String(String),
    Note(u32),
    Dot,
    DotDot,
    DotDotDot,
    NewLine,
    Empty,
    Error(char, Location),
}

#[derive(Debug)]
pub struct FamiNumber{
    dec: i32,
    hex: u32,
    effect: (char, u8)
}