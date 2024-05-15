pub struct Port {
    input: u8,
    output: u8,
}

impl Port {
    pub fn new() -> Port {
        Port {
            input: 0b_0000,
            output: 0b_0000,
        }
    }

    // input get, set
    pub fn input(&self) -> u8 {
        // 値は4ビットのみ
        self.input & 0b_1111
    }

    pub fn set_input(&mut self, data: u8) {
        // 値は4ビットのみ
        self.input = data & 0b_1111;
    }

    // output get, set
    pub fn output(&self) -> u8 {
        // 値は4ビットのみ
        self.output & 0b_1111
    }

    pub fn set_output(&mut self, data: u8) {
        // 値は4ビットのみ
        self.output = data & 0b_1111;
    }
}