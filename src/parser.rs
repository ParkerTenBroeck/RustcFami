use std::{error::Error};

use crate::{tokenizer::{Tokenizer, self, Token, SkipNLPeekable}, sound_file::*};


fn option_note(token: Option<Token>) -> Result<Option<Note>, Box<dyn Error>>{
    match token{
        Some(val) => {
            match val{
                Token::Note(str) => {
                    return Ok(Some(str));
                }
                Token::DotDotDot => {
                    return Ok(None)
                }
                _ => {
                    return Err("Note or ... String found other".into());
                }
            }
        },
        None => Result::Err("Expected Some got None".into()),
    }
}

fn option_instrament(token: Option<Token>) -> Result<Option<u8>, Box<dyn Error>>{
    match token{
        Some(val) => {
            match val{
                Token::IdentNum(num) => {
                    return Ok(Some(num.hex()?.try_into()?));
                }
                Token::DotDot => {
                    return Ok(None)
                }
                _ => {
                    return Err("Expected Hex or .. found other".into());
                }
            }
        },
        None => Result::Err("Expected Some got None".into()),
    }
}

fn option_volume(token: Option<Token>) -> Result<Option<u8>, Box<dyn Error>>{
    match token{
        Some(val) => {
            match val{
                Token::IdentNum(num) => {
                    let num = num.hex()?;
                    if num > 15{
                        return Err("Hex number too big".into())
                    }
                    return Ok(Some(num.try_into()?));
                }
                Token::Dot => {
                    return Ok(None)
                }
                _ => {
                    return Err("Expected Hex or .. found other".into());
                }
            }
        },
        None => Result::Err("Expected Some got None".into()),
    }
}

fn option_effect(token: Option<Token>) -> Result<Option<Effect>, Box<dyn Error>>{
    match token{
        Some(val) => {
            match val{
                Token::IdentNum(num) => {
                    return Ok(Some(num.effect()?));
                }
                Token::DotDotDot => {
                    return Ok(None)
                }
                _ => {
                    return Err("Expected Effect or ... found other".into());
                }
            }
        },
        None => Result::Err("Expected Some got None".into()),
    }
}

fn expect_str(token: Option<Token>) -> Result<String, Box<dyn Error>>{
    match token{
        Some(val) => {
            match val{
                Token::String(str) => {
                    return Ok(str);
                }
                _ => {
                    return Err("Expected String found other".into());
                }
            }
        },
        None => Result::Err("Expected Some got None".into()),
    }
}

fn expect_nl(token: Option<Token>) -> Result<(), Box<dyn Error>>{
    match token{
        Some(val) => {
            match val{
                Token::NewLine => {
                    return Ok(());
                }
                _ => {
                    return Err("Expected NewLine found other".into());
                }
            }
        },
        None => Result::Err("Expected Some got None".into()),
    }
}

fn expect_colon(token: Option<Token>) -> Result<(), Box<dyn Error>>{
    match token{
        Some(val) => {
            match val{
                Token::Colon => {
                    return Ok(());
                }
                _ => {
                    return Err("Expected Colon found other".into());
                }
            }
        },
        None => Result::Err("Expected Some got None".into()),
    }
}

fn expect_dec_num(token: Option<Token>) -> Result<i32, Box<dyn Error>>{
    match token{
        Some(val) => {
            match val{
                Token::IdentNum(num) => {
                    return Ok(num.dec()?);
                }
                _ => {
                    return Err("Expected NewLine found other".into());
                }
            }
        },
        None => Result::Err("Expected Some got None".into()),
    }
}
fn expect_opt_dec_num(token: Option<Token>) -> Result<Option<u8>, Box<dyn Error>>{
    match token{
        Some(val) => {
            match val{
                Token::IdentNum(num) => {
                    let num = num.dec()?;
                    if num < 0{
                        return Ok(None);
                    }else{
                        return Ok(Option::Some(num.try_into()?));
                    }
                }
                _ => {
                    return Err("Expected NewLine found other".into());
                }
            }
        },
        None => Result::Err("Expected Some got None".into()),
    }
}

fn expect_hex_num(token: Option<Token>) -> Result<u32, Box<dyn Error>>{
    match token{
        Some(val) => {
            match val{
                Token::IdentNum(num) => {
                    return Ok(num.hex()?);
                }
                _ => {
                    return Err("Expected NewLine found other".into());
                }
            }
        },
        None => Result::Err("Expected Some got None".into()),
    }
}

pub fn read_text(str: &str) -> Result<SoundFile, Box<dyn Error>>{
   
    let mut file = SoundFile::default();
    let mut tokenizer = Tokenizer::from_str(str).peekable();
    while let Option::Some(token) = tokenizer.next(){

        match token{
            tokenizer::Token::IdentNum(ident) => {
                match ident.as_str(){
                    "TITLE" => {
                        file.title = expect_str(tokenizer.next())?;
                        expect_nl(tokenizer.next())?;
                    }
                    "AUTHOR" => {
                        file.author = expect_str(tokenizer.next())?;
                        expect_nl(tokenizer.next())?;
                    }
                    "COPYRIGHT" => {
                        file.copyright = expect_str(tokenizer.next())?;
                        expect_nl(tokenizer.next())?;
                    }
                    "COMMENT" => {
                        file.comment = format!("{}{}\n", file.comment, expect_str(tokenizer.next())?);
                        expect_nl(tokenizer.next())?;
                    }
                    "MACHINE" => {
                        file.machine = expect_dec_num(tokenizer.next())?.try_into()?;
                        expect_nl(tokenizer.next())?;
                    }
                    "EXPANSION" => {
                        file.expansion = expect_dec_num(tokenizer.next())?.try_into()?;
                        expect_nl(tokenizer.next())?;
                    }
                    "VIBRATO" => {
                        file.vibrato = expect_dec_num(tokenizer.next())?.try_into()?;
                        expect_nl(tokenizer.next())?;
                    }
                    "SPLIT" => {
                        file.split = expect_dec_num(tokenizer.next())?.try_into()?;
                        expect_nl(tokenizer.next())?;
                    }
                    "PLAYBACKRATE" => {
                        file.playbackrate.0 = expect_dec_num(tokenizer.next())?.try_into()?;
                        file.playbackrate.1 = expect_dec_num(tokenizer.next())?.try_into()?;
                        expect_nl(tokenizer.next())?;
                    }
                    "TUNING" => {
                        file.tuning.0 = expect_dec_num(tokenizer.next())?;
                        file.tuning.1 = expect_dec_num(tokenizer.next())?;
                        expect_nl(tokenizer.next())?;
                    }
                    "MACRO" => {
                        let mut song_macro = SongMacro{
                            m_type:  expect_dec_num(tokenizer.next())?.try_into().unwrap(),
                            m_id: expect_dec_num(tokenizer.next())?.try_into()?,
                            m_loop: expect_opt_dec_num(tokenizer.next())?.try_into()?,
                            m_release: expect_opt_dec_num(tokenizer.next())?.try_into()?,
                            m_type_specific: expect_dec_num(tokenizer.next())?.try_into()?,
                            vals: Default::default(),
                        };
                        expect_colon(tokenizer.next())?;
                        while let Option::Some(tok) = tokenizer.next(){
                            match tok{
                                Token::IdentNum(num) => {
                                    song_macro.vals.push(num.dec()?.try_into()?);
                                },
                                Token::NewLine => {break;},
                                _ => {
                                    return Err("Unexpected token type".into());
                                }
                            }
                        }
                        file.macros.push(song_macro);
                    }
                    "DPCMDEF" => {
                        let id =  expect_dec_num(tokenizer.next())?.try_into()?;
                        let len = expect_dec_num(tokenizer.next())? .try_into()?;
                        let name = expect_str(tokenizer.next())?;
                        expect_nl(tokenizer.next())?;
                        let mut data = Vec::new();

                        while let Option::Some(Token::IdentNum(str)) = tokenizer.peek_skipping_nl(){
                            if str == "DPCM"{
                                let _ = tokenizer.next();//accept peek
                                expect_colon(tokenizer.next())?;
                                while let Option::Some(Token::IdentNum(num)) = tokenizer.peek(){
                                    data.push(num.hex()? as u8);
                                    let _ = tokenizer.next();//accept peek
                                }
                                expect_nl(tokenizer.next())?
                            }else{
                                break;
                            }
                        }
                        if data.len() != len{
                            return Result::Err(format!("DPCMDEF provided data doesnt match size given, listed: {}, given: {}", len, data.len()).into());
                        }
                        file.dpcmdef.push(SongDpcmSamples { id, name, data });
                    }
                    "INST2A03" => {
                        let inst = Inst2A03{
                            id: expect_dec_num(tokenizer.next())?.try_into()?,
                            vol_macro: expect_opt_dec_num(tokenizer.next())?,
                            arp_macro: expect_opt_dec_num(tokenizer.next())?,
                            pitch_macro: expect_opt_dec_num(tokenizer.next())?,
                            high_pitch_macro: expect_opt_dec_num(tokenizer.next())?,
                            duity_macro: expect_opt_dec_num(tokenizer.next())?,
                            name: expect_str(tokenizer.next())?,
                        };
                        file.inst2a03.push(inst);
                    }
                    "KEYDPCM" => {
                        let key = KeyDPCM{
                            id: expect_dec_num(tokenizer.next())?.try_into()?,
                            inst_id: expect_dec_num(tokenizer.next())?.try_into()?,
                            midi_note: {
                                let oct: u32 = expect_dec_num(tokenizer.next())?.try_into()?;
                                let note: u32 = expect_dec_num(tokenizer.next())?.try_into()?;
                                (oct + 1) * 12 + note
                            },
                            dpcm_id:  expect_dec_num(tokenizer.next())?.try_into()?,
                            loop_key: {
                                let num = expect_dec_num(tokenizer.next())?;
                                if num == 0{
                                    false
                                }else if num == 1{
                                    true
                                }else{
                                    return Err("Unexpected loop value, can only be 0 or 1".into());
                                }
                            },
                            loop_point: expect_dec_num(tokenizer.next())?.try_into()?,
                            d_counter: expect_opt_dec_num(tokenizer.next())?,
                            
                        };
                        file.keydpcm.push(key);
                    }
                    "TRACK" => {
                        let mut track = Track{
                            pattern_length: expect_dec_num(tokenizer.next())?.try_into()?,
                            speed: expect_dec_num(tokenizer.next())?.try_into()?,
                            temp: expect_dec_num(tokenizer.next())?.try_into()?,
                            name: expect_str(tokenizer.next())?,
                            comumns: Default::default(),
                            patterns: Default::default(),
                            pattern_order: Default::default(),
                        };
                        

                        if let Option::Some(Token::IdentNum(val)) = tokenizer.peek_skipping_nl(){
                            if val == "COLUMNS"{
                                let _ = tokenizer.next();//accept peek
                                expect_colon(tokenizer.next())?;
                                while let Option::Some(Token::IdentNum(num)) = tokenizer.peek(){
                                    track.comumns.push(num.hex()?.try_into()?);
                                    let _ = tokenizer.next();//accept peek
                                }
                            }else{
                                return Err("Missing COLUMNS after TRACK".into())
                            }
                        }else{
                            return Err("Expected ident found other".into())
                        }

                        while let Option::Some(Token::IdentNum(val)) = tokenizer.peek_skipping_nl(){
                            if val == "ORDER"{
                                let mut order_data = Vec::new();
                                
                                let _ = tokenizer.next();//accept peek
                                let id = expect_hex_num(tokenizer.next())?;
                                expect_colon(tokenizer.next())?;
                                while let Option::Some(Token::IdentNum(num)) = tokenizer.peek(){
                                    order_data.push(num.hex()?.try_into()?);
                                    let _ = tokenizer.next();//accept peek
                                }
                                expect_nl(tokenizer.next())?;
                                if order_data.len() != track.comumns.len(){
                                    return Err("Order data and column data cannot be different lengths??".into())
                                }
                                track.pattern_order.push((id.try_into()?, order_data));
                            }else{
                                break;
                            }
                        }

                        //let mut patterns = Vec::new();
                        while let Option::Some(Token::IdentNum(val)) = tokenizer.peek_skipping_nl(){
                            if val == "PATTERN"{
                                
                                let _ = tokenizer.next();//accept peek
                                let mut pattern = Pattern{
                                    id: expect_hex_num(tokenizer.next())?.try_into()?,
                                    rows: Default::default(),
                                };
                                
                                while let Option::Some(Token::IdentNum(val)) = tokenizer.peek_skipping_nl(){
                                    if val == "ROW"{
                                        let _ = tokenizer.next();//accept peek
                                        let mut row = Row{
                                            id: expect_hex_num(tokenizer.next())?.try_into()?,
                                            sheet_notes: Default::default(),
                                        };
                                        
                                        for i in 0..track.comumns.len(){
                                            expect_colon(tokenizer.next())?;
                                            let mut sheet_note = SheetNote{
                                                note: option_note(tokenizer.next())?,
                                                inst:  option_instrament(tokenizer.next())?,
                                                vol: option_volume(tokenizer.next())?,
                                                efx: Default::default(),
                                            };
                                            for j in 0..track.comumns[i]{
                                                sheet_note.efx[j as usize] = option_effect(tokenizer.next())?;
                                            }
                                            row.sheet_notes.push(sheet_note);
                                        }
                                        if pattern.rows.len() != track.pattern_length as usize{
                                            return Err("Given pattern length doesn match actual pattern length".into());
                                        }

                                        pattern.rows.push(row);
                                    }else{
                                        break;//return Err("Expected Ident ROW found other".into());
                                    }
                                }
                                track.patterns.push(pattern);
                                //orders.push((id, order_data));
                            }else{
                                break;
                            }
                        }
                        file.tracks.push(track);

                    }
                    _ => {
                        return Result::Err(format!("Unknown command: {}", ident.as_str()).into());
                    }
                }
            },
            tokenizer::Token::NewLine => {
                //skip
            },
            tokenizer::Token::Empty => return Result::Err("Given empty??".into()),
            tokenizer::Token::Error(char, location) => return Result::Err(format!("Unreconized char: {:?} at {:?}", char, location).into()),
            _ => {

            }
        }
    }
    Result::Ok(file)
}

