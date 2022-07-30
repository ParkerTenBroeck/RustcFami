use std::{sync::{Mutex, Arc}, f64::consts::PI};

use rodio::{OutputStreamHandle, Sink, OutputStream, Source};

pub struct HardwareInterface{
    h_2a03: Arc<Mutex<Apu>>,
    #[allow(unused)]
    stream_handle: OutputStreamHandle,
    #[allow(unused)]
    stream: OutputStream,
}

impl HardwareInterface{
    pub fn new() -> Self{
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let mut apu = Apu::new();
        apu.reset();
        
        apu.write_register(0x4000, 0x3F);
        apu.write_register(0x4001, 0x08);
        apu.write_register(0x4002, 0x5D);
        apu.write_register(0x4003, 0x00);
        apu.write_register(0x4015, 0x01);
        let source = Arc::new(Mutex::new(apu));
        sink.append(Thing{apu: source.clone() });
        sink.detach();
        Self { 
            h_2a03: source, 
            stream_handle,
            stream
        }
    }
    pub fn reset(&mut self) {
        self.h_2a03.lock().unwrap().reset();
    }
}

struct Thing{
    apu: Arc<Mutex<Apu>>,
}

impl Iterator for Thing{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.apu.lock().unwrap().next_sample();
        //let e = std::time::SystemTime::UNIX_EPOCH;
        //let dur = std::time::SystemTime::duration_since(&std::time::SystemTime::now(), e).unwrap();
        //let wave = (440.0 * (self.sample as f64) / 44100.0 * PI * 2.0).sin();
        println!("{}", out);
        //self.sample += 1;
        //Option::Some(wave as f32)
        Option::Some(out)
    }
}

impl Source for Thing{
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        44100
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}


struct Apu{
    reg: [u8; 15]
}

impl Apu{
    pub fn next_sample(&mut self) -> f32{
        0.0
    }
    pub fn reset(&mut self){

    }
    pub fn new() -> Self{
        Self { reg: [0; 15] }
    }

    pub fn write_register(&mut self, address: u16, value: u8) {
        // match address {
        //     0x4000..=0x4003 => self.pulse_0.write_register(address, value),
        //     0x4004..=0x4007 => self.pulse_1.write_register(address, value),
        //     0x4008..=0x400B => self.triangle.write_register(address, value),
        //     0x400C..=0x400F => self.noise.write_register(address, value),
        //     0x4010..=0x4013 => self.dmc.write_register(address, value),
        //     0x4015 => {
        //         self.pulse_0.set_enabled(value & 0b0000_0001 != 0);
        //         self.pulse_1.set_enabled(value & 0b0000_0010 != 0);
        //         self.triangle.set_enabled(value & 0b0000_0100 != 0);
        //         self.noise.set_enabled(value & 0b0000_1000 != 0);
        //         self.dmc.set_enabled(value & 0b0001_0000 != 0);
        //     }
        //     0x4017 => {
        //         let r = self.frame_counter.write_register(value, cycles);
        //         self.handle_frame_result(r);
        //     }
        //     _ => panic!("Bad APU address: {:04X}", address),
        // }
    }
}