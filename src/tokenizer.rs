use std::{iter::Peekable, str::Chars, ops::Add, error::Error};

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
    Dash,
    DashDash,
    Equal,
    EqualEqual,
}
#[derive(Copy, Clone, Default, Debug)]
pub struct Location{
    real: usize,
    seen: usize,
    line: usize,
    column: usize,
}

impl Add<char> for Location{
    type Output = Self;

    fn add(mut self, char: char) -> Self::Output {
        self.real += char.len_utf8();
        self.seen += 1;
        if char == '\n'{
            self.line += 1;
            self.column = 0;
        }else{
            self.column += 1;
        }
        self
    }
}

pub struct Tokenizer<'a>{
    str: &'a str,
    iter: Peekable<Chars<'a>>,
    state: TokenizerState,
    start: Location,
    last: Location,
}

impl<'a> Tokenizer<'a>{
    pub fn from_str(str: &'a str) -> Self{
        Self{
            iter: str.chars().peekable(),
            str,
            state: TokenizerState::Default,
            start: Default::default(),
            last: Default::default(),
        }
    }

    fn str_last(&self) -> &str{
        //&self.str[self.start.real..(self.end.real)]
        self.str_loc_loc(self.start, self.last)
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
            let current;
            match self.iter.peek(){
                Some(char) => {
                    c = *char;
                    current = self.last + c;
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
                        '0'..='9' => {
                            self.state = TokenizerState::Number;
                        }
                        '-' => self.state = TokenizerState::Dash,
                        '=' => self.state = TokenizerState::Equal,
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
                                ret = Option::Some(Token::Error(c, current));
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
                        '0'..='9' => {
                            self.state = TokenizerState::Number;
                        }
                        '-'|'#' => {
                            self.state = TokenizerState::Note;
                        }
                        _ =>{
                            self.state = TokenizerState::Default;
                            ret = Option::Some(Token::IdentNum(self.str_last().into()));
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
                        '-' => {
                            self.state = TokenizerState::Note;
                        }
                        _ => {
                            self.state = TokenizerState::Default;
                            let str = self.str_last();
                            ret = Option::Some(Token::IdentNum(IdentNum::from_str(str)));
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
                            let str = self.str_loc_loc(self.start, current);
                            let off = (str.as_bytes()[2] as u32 - '0' as u32 + 1) * 12;
                            let note = &str[0..2];
                            loop{
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
                                        ret = Option::Some(Token::Error(note.chars().next().unwrap(), self.start));
                                        break;
                                    }
                                };
                                ret = Option::Some(Token::Note(Note::Midi(off + note)));
                                break;
                            }
                        }
                        '#' => {
                            self.state = TokenizerState::Default;
                            let str = self.str_loc_loc(self.start, current);
                            let mut iter =  str.chars();
                            let c = iter.next().unwrap().to_uppercase().next().unwrap();
                            match c {
                                'A'..='F' => {
                                    ret = Option::Some(Token::Note(Note::Hex(c as u8 - 'A' as u8 + 10)));
                                }
                                '0'..='9' => {
                                    ret = Option::Some(Token::Note(Note::Hex(c as u8 - '0' as u8)));
                                }
                                _ => {
                                    ret = Option::Some(Token::Error(c, self.start));
                                }
                            }
                            let c2 = iter.next().unwrap();
                            if c2 != '-'{
                                ret = Option::Some(Token::Error(c2, self.start + c));
                            }
                        }
                        _ => {
                            self.state = TokenizerState::Default;
                            ret = Option::Some(Token::Error(c, current));
                        }
                    }
                },
                TokenizerState::Dash => {
                    match c{
                        '-' => self.state = TokenizerState::DashDash,
                        _ => {
                            self.state = TokenizerState::Number;
                            consume = false;
                        }
                    }
                },
                TokenizerState::DashDash => {
                    self.state = TokenizerState::Default;
                    match c{
                        '-' => {
                            ret = Option::Some(Token::Note(Note::Cut));
                            self.state = TokenizerState::Default
                        },
                        _ => {
                            ret = Option::Some(Token::Error(c, current));
                        }
                    }  
                },
                TokenizerState::Equal => {
                    match c{
                        '=' => self.state = TokenizerState::EqualEqual,
                        _ => {
                            ret = Option::Some(Token::Error(c, current));
                        }
                    }
                }
                TokenizerState::EqualEqual => {
                    self.state = TokenizerState::Default;
                    match c{
                        '-' => {
                            ret = Option::Some(Token::Note(Note::Release));
                            self.state = TokenizerState::Default
                        },
                        _ => {
                            ret = Option::Some(Token::Error(c, current));
                        }
                    } 
                },
            }
            if consume{
                let _ = self.iter.next().unwrap();
                self.last = current;
            }
            match ret{
                Some(val) => {
                    self.start = self.last;
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

pub trait SkipNLPeekable{
    fn peek_skipping_nl(&mut self) -> Option<&Token>;
}

impl<'a> SkipNLPeekable for Peekable<Tokenizer<'a>> {
    fn peek_skipping_nl(&mut self) -> Option<&Token> {
        while let Option::Some(Token::NewLine) = self.peek(){
            let _ = self.next();
        }
        self.peek()
    }
}

#[derive(Debug, Clone)]
pub enum Token{
    IdentNum(IdentNum),
    Colon,
    String(String),
    Note(Note),
    Dot,
    DotDot,
    DotDotDot,
    NewLine,
    Empty,
    Error(char, Location),
}

#[derive(Debug, Clone, Copy)]
pub enum Note{
    Hex(u8),
    Midi(u32),
    Cut,
    Release,
}

#[derive(Debug, Clone)]
pub struct IdentNum{
    string: String
}

impl IdentNum{
    pub fn hex(&self) -> Result<u32, Box<dyn Error>>{
        Result::Ok(u32::from_str_radix(self.string.as_str(), 16)?)
    }

    pub fn dec(&self) -> Result<i32, Box<dyn Error>>{
        Result::Ok(i32::from_str_radix(self.string.as_str(), 10)?)
    }

    pub fn effect(&self)  -> Result<Effect, Box<dyn Error>>{
        let char = self.string.as_bytes()[0] as char;
        let num_str = &self.string[1..3];
        macro_rules! num {
            () => {
                u8::from_str_radix(num_str, 16)?
            };
        }
        macro_rules! num_or_default {
            ($default:expr) => {
                match u8::from_str_radix(num_str, 16){
                    Ok(val) => val,
                    Err(_) => $default,
                }
            };
        }
        macro_rules! num_x_y {
            () => {
                {
                    let num = u8::from_str_radix(num_str, 16)?;
                    let x = num >> 4;
                    let y = num & 0b1111;
                    (x,y) 
                }       
            };
        }

        macro_rules! num_option {
            () => {
                {
                    let num = u8::from_str_radix(num_str, 16)?;
                    if num == 0 {Option::None} else {Option::Some(num)}        
                }
            };
        }
        
        match char{
            '0' => Ok(Effect::Arpeggio(num_x_y!().0, num_x_y!().1)),
            '1' => Ok(Effect::PitchSlideUp(num_option!())),
            '2' => Ok(Effect::PitchSlideDown(num_option!())),
            '3' => Ok(Effect::AutomaticPortamento(num_option!())),
            '4' => Ok(Effect::Arpeggio(num_x_y!().0, num_x_y!().1)),
            '7' => Ok(Effect::VibratoEffect(if num_x_y!().0 == 0 {None} else {Some(num_x_y!())})),
            'A' => {
                if 0 == num_x_y!().0 {
                    return Ok(Effect::VolumeSlide(false, num_x_y!().1))
                } else if  0 == num_x_y!().1 {
                    return Ok(Effect::VolumeSlide(true, num_x_y!().0))
                } else {
                    return Err("Invalid slide num".into())
                }
            },
            'B' => Ok(Effect::JumpToPattern(num!())),
            'C' => Ok(Effect::Halt),
            'D' => Ok(Effect::SkipFrameStartAtRow(num!())),
            //'E' => Ok(Effect::JumpToPattern(num!())),
            'F' => {
                let num = num!();
                match num {
                    0x00..=0x1F => Ok(Effect::SpeedOrTempo(Option::Some(num), None)),
                    0x20..=0xFF => Ok(Effect::SpeedOrTempo(None, Some(num))),
                }
            },
            'G' => Ok(Effect::NoteDelay(num!())),
            'H' => Ok(Effect::HardwareSweepUp(num_x_y!().0, num_x_y!().1)),
            'I' => Ok(Effect::HardwareSweepDown(num_x_y!().0, num_x_y!().1)),
            'P' => Ok(Effect::FinePitch(num_or_default!(0x80))),
            'Q' => Ok(Effect::NoteSlideUp(num_x_y!().0, num_x_y!().1)),
            'R' => Ok(Effect::NoteSlideDown(num_x_y!().0, num_x_y!().1)),
            'S' => Ok(Effect::MuteDelay(num!())),
            'V' => Ok(Effect::MuteDelay(num!())),
            'W' => Ok(Effect::AquareDuityNoiseN163Mode()),
            'X' => Ok(Effect::DPCMSampleSpeedOverride(num!())),
            'Y' => Ok(Effect::DPCMSampleOffset(num!() as u32 * 64)),
            'Z' => Ok(Effect::DPCMDeltaCounter(num!())),
            _ => {
                Result::Err("Unknown Effect number".into())
            }
        }
    }

    pub fn from_str(str: &str) -> Self{
        IdentNum { string: str.into() }
    }

    pub fn as_str(&self) -> &str{
        self.string.as_str()
    }
}
#[derive(Debug, Clone, Copy)]
pub enum Effect{
    Arpeggio(u8, u8),
    PitchSlideUp(Option<u8>),
    PitchSlideDown(Option<u8>),
    AutomaticPortamento(Option<u8>),
    VibratoEffect(Option<(u8, u8)>),
    TremoloEffect(Option<(u8, u8)>),
    VolumeSlide(bool, u8),
    JumpToPattern(u8),
    Halt,
    SkipFrameStartAtRow(u8),
    SpeedOrTempo(Option<u8>, Option<u8>),
    NoteDelay(u8),
    HardwareSweepUp(u8, u8),
    HardwareSweepDown(u8, u8),
    FSDModulationDepth,
    FDSModulationSpeed,
    FinePitch(u8),
    NoteSlideUp(u8, u8),
    NoteSlideDown(u8, u8),
    MuteDelay(u8),
    AquareDuityNoiseN163Mode(/*TODO */),
    DPCMSampleSpeedOverride(u8),
    DPCMSampleOffset(u32),
    DPCMDeltaCounter(u8),
}

impl From<&str> for IdentNum{
    fn from(str: &str) -> Self {
        Self::from_str(str)
    }
}

impl PartialEq<str> for IdentNum{

    fn eq(&self, other: &str) -> bool {
        self.string == other
    }
}