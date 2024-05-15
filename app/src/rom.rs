pub struct Rom {
    data: Vec<u8>,
}

impl Rom {
    pub fn new() -> Rom {
        Rom {
            data: Vec::new(),
        }
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.data = program;
    }

    pub fn read(&self, address: u8) -> u8 {
        self.data[address as usize]
    }
}