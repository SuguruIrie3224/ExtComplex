/// レジスタの構造体
pub struct Register{
    register_a: u8,
    register_b: u8,
    pc: u8,
    carry: bool,
}

impl Register{
    /// 新しいレジスタを作成する
    pub fn new() -> Register{
        Register{
            register_a: 0b0000_0000,
            register_b: 0b0000_0000,
            pc: 0b0000_0000,
            carry: false,
        }
    }

    // get, set pc
    pub fn pc(&self) -> u8{
        self.pc
    }

    pub fn set_pc(&mut self, value: u8){
        self.pc = value;
    }

    // get, set register_a
    pub fn register_a(&self) -> u8{
        self.register_a
    }

    pub fn set_register_a(&mut self, value: u8){
        self.register_a = value;
    }

    // get, set register_b
    pub fn register_b(&self) -> u8{
        self.register_b
    }

    pub fn set_register_b(&mut self, value: u8){
        self.register_b = value;
    }

    // increment pc
    pub fn increment_pc(&mut self){
        self.pc += 1;
    }

    // get, set carry
    pub fn carry(&self) -> Option<bool>{
        Some(self.carry)
    }

    pub fn set_carry(&mut self, value: bool){
        self.carry = value;
    }
}