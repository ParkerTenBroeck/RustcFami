
#[allow(unused)]
#[derive(Default, Debug)]
pub struct SoundFile{
    pub title: String,
    pub author: String,
    pub copyright: String,
    pub comment: String,

    pub machine: u32,
    pub expansion: u32,
    pub vibrato: u32,
    pub split: u32,
    pub playbackrate: (u32, u32),
    pub tuning: (i32, i32),

    pub macros: Vec<SongMacro>,
    pub inst2a03: Vec<Inst2A03>,
    pub keydpcm: Vec<KeyDPCM>,
    pub dpcmdef: Vec<SongDpcmSamples>,
    pub tracks: Vec<Track>,
}


#[derive(Debug)]
pub struct KeyDPCM{
    pub id: u8,
    pub inst_id: u8,
    pub midi_note: u32,
    //_3: Option<u8>,
    pub dpcm_id: u8,
    pub loop_key: bool,
    pub loop_point: u8,
    pub d_counter: Option<u8>,
}

#[derive(Debug)]
pub struct Inst2A03{
    pub id: u8,
    pub vol_macro: Option<u8>,
    pub arp_macro: Option<u8>,
    pub pitch_macro: Option<u8>,
    pub high_pitch_macro: Option<u8>,
    pub duity_macro: Option<u8>,
    pub name: String,
}

#[derive(Debug)]
pub struct Track{
    pub pattern_length: u32,
    pub speed: u32,
    pub temp: u32,
    pub name: String,
    pub comumns: Vec<u8>,
    pub pattern_order: Vec<(u8, Vec<u8>)>,
    pub patterns: Vec<Pattern>
}


#[derive(Debug)]
pub struct Pattern{
    pub id: u8,
    pub rows: Vec<Row>
}

#[derive(Debug)]
pub struct Row{
    pub id: u8,
    pub sheet_notes: Vec<SheetNote>
}


#[derive(Debug)]
pub struct SheetNote{
    pub note: Option<Note>,
    pub inst: Option<u8>,
    pub vol: Option<u8>,
    pub efx: [Option<Effect>; 3]
}

#[derive(Debug)]
pub struct SongDpcmSamples{
    pub id: u8,
    pub name: String,
    pub data: Vec<u8>
}

#[derive(Debug)]
pub struct SongMacro{
    pub m_type: u8,
    pub m_id: u8,
    pub m_loop: Option<u8>,
    pub m_release: Option<u8>,
    pub m_type_specific: u8,
    pub vals: Vec<i8>
}


#[derive(Debug, Clone, Copy)]
pub enum Note{
    Hex(u8),
    Midi(u32),
    Cut,
    Release,
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

impl TryFrom<&str> for Effect{
    type Error = Box<dyn std::error::Error>;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        let char = str.as_bytes()[0] as char;
        let num_str = &str[1..3];
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
}