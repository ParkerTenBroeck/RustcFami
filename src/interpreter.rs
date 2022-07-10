use crate::sound_file::*;

pub struct Interpreter<'a>{
    file: &'a SoundFile,
    current_track: Option<&'a Track>,
    row: u8,
    pattern_order: u8,
}

impl<'a> Interpreter<'a>{
    pub fn new(file: &'a SoundFile) -> Self{
        Self {
            file,
            current_track: Default::default(),
            row: 0,
            pattern_order: 0, 
        }
    }

    pub fn start_track(&mut self, name: &str){
        self.reset();
        for track in &self.file.tracks{
            if track.name == name{
                self.current_track = Option::Some(track);
                return;
            }
        }
        self.current_track = Option::None;
    }

    pub fn next_row(&mut self){
        if let Option::Some(track) = self.current_track{
            self.play_current_row();
            self.row += 1;
            if self.row >= track.pattern_length as u8{
                self.pattern_order += 1;
                self.row = 0
            }
            if self.pattern_order >= track.pattern_order.len() as u8{
                //stop the song
                return;
            }
        }
    }

    fn play_current_row(&mut self){
        let track = self.current_track.unwrap();
        let curr_order = &track.pattern_order[self.pattern_order as usize];
        
        for i in 0..curr_order.1.len(){
            let current_order_note_pattern = &track.patterns[curr_order.1[i] as usize];
            let row = &current_order_note_pattern.rows[self.row as usize];
            self.play_sheet_note( &row.sheet_notes[i], i as u8);
        }
    }

    fn play_sheet_note(&mut self, note: &SheetNote, inst: u8){
        match inst{
            0 => {
                //square 1
            }
            1 => {
                //square 2
            }
            2 => {
                //triangle 1
            }
            3 => {
                //nosie 1
            }
            4 => {
                //DPCM
            }
            _ => {
                
            }
        }
    }

    pub fn run_frame(&mut self){

    }

    pub fn play_frame(&mut self){
        self.next_row();
        self.run_frame();
    }

    pub fn reset(&mut self){
        self.row = 0;
        self.pattern_order = 0;
    }
}