pub struct HardwareInterface{
    h_2a03: I2A03
}

#[derive(Debug)]
pub struct I2A03{
    square1: (),
    square2: (),
    triangle: (),
    noise: (),
}

impl I2A03{
    pub fn write_register(&mut self, add: u16, data: u8){

    }
}

struct SquareChannel{

}

impl SquareChannel{
    pub fn write_register(&mut self, add: u16, data: u8){

    }
}