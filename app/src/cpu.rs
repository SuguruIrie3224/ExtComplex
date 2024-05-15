use crate::register::Register;
use crate::opcode::Opcode;
use crate::rom::Rom;
use crate::port::Port;

pub struct Cpu{
    register: Register,
    rom: Rom,
    port: Port,
}

impl Cpu {    
    // new
    pub fn new() -> Cpu{
        Cpu{
            register: Register::new(),
            rom: Rom::new(),
            port: Port::new(),
        }
    }

    // プログラムの読み込み
    pub fn load(&mut self, program: Vec<u8>) {
        self.rom.load(program);
    }

    // fetch
    pub fn fetch(&self) -> u8 {
        // メモリからデータを取得する
        let address = self.register.pc();
        
        self.rom.read(address)
    }

    // decode
    pub fn decode(&self,data: u8) -> (Opcode, u8) {
        // 先頭4ビットをopcodeとして取得する
        let opcode = data >> 4;
        // 残りの4ビットはオペランドとして取得する
        let operand = data & 0b0000_1111;

        // opcodeをOpcodeに変換する
        let opcode = match opcode {
            0b_0000 => Opcode::AddA,
            0b_0101 => Opcode::AddB,

            0b_0011 => Opcode::MovA,
            0b_0111 => Opcode::MovB,

            0b_0001 => Opcode::MovAB,
            0b_0100 => Opcode::MovBA,

            0b_1111 => Opcode::Jmp,
            0b_1110 => Opcode::Jnc,

            0b_0010 => Opcode::InA,
            0b_0110 => Opcode::InB,

            0b_1001 => Opcode::OutB,
            0b_1011 => Opcode::OutIm,

            _ => Opcode::AddA,
        };

        // InA, InB, OutBの場合はオペランドを0b_0000にする
        let operand = match opcode {
            Opcode::InA | Opcode::InB | Opcode::OutB => 0b_0000,
            _ => operand,
        };

        (opcode, operand)
    }

    // execute
    pub fn execute(&mut self, opcode: Opcode, operand: u8) {
        match opcode {
            Opcode::AddA => {
                // Aレジスタにオペランドを加算する(4bit)
                let value = self.register.register_a();
                self.register.set_register_a((value + operand) & 0b_1111);

                // オーバーフローフラグをセットする
                // このCPUは4bitのため、オーバーフローは5bit目で判定する
                let overflow = (value as u16 + operand as u16) > 0b_1111;
                self.register.set_carry(overflow);

            }
            Opcode::AddB => {
                // Bレジスタにオペランドを加算する(4bit)
                let value = self.register.register_b();
                self.register.set_register_b((value + operand) & 0b_1111);

                // オーバーフローフラグをセットする
                // このCPUは4bitのため、オーバーフローは5bit目で判定する
                let overflow = (value as u16 + operand as u16) > 0b_1111;
                self.register.set_carry(overflow);

            }
            Opcode::MovA => {
                // Aレジスタにオペランドをセットする
                self.register.set_register_a(operand);
                // キャリーフラグをセットする(ここでは必ずfalseにする)
                self.register.set_carry(false);

            }
            Opcode::MovB => {
                // Bレジスタにオペランドをセットする
                self.register.set_register_b(operand);
                // キャリーフラグをセットする(ここでは必ずfalseにする)
                self.register.set_carry(false);

            }
            Opcode::MovAB => {
                // Aレジスタの値をBレジスタにセットする
                let value = self.register.register_a();
                self.register.set_register_b(value);
                // キャリーフラグをセットする(ここでは必ずfalseにする)
                self.register.set_carry(false);

            }
            Opcode::MovBA => {
                // Bレジスタの値をAレジスタにセットする
                let value = self.register.register_b();
                self.register.set_register_a(value);
                // キャリーフラグをセットする(ここでは必ずfalseにする)
                self.register.set_carry(false);

            }
            Opcode::Jmp => {
                // オペランドのアドレスにジャンプする
                let address = operand;
                self.register.set_pc(address);

                // キャリーフラグをセットする(ここでは必ずfalseにする)
                self.register.set_carry(false);

            }
            Opcode::Jnc => {
                // キャリーフラグが立っていない場合、オペランドのアドレスにジャンプする
                if let Some(carry) = self.register.carry() {
                    if !carry {
                        let address = operand;
                        self.register.set_pc(address);
                    }
                }

                // キャリーフラグをセットする(ここでは必ずfalseにする)
                self.register.set_carry(false);

            }
            Opcode::InA => {
                // ポートから入力をAレジスタにセットする
                let data = self.port.input();
                self.register.set_register_a(data);

                // キャリーフラグをセットする(ここでは必ずfalseにする)
                self.register.set_carry(false);

            }
            Opcode::InB => {
                // ポートから入力をBレジスタにセットする
                let data = self.port.input();
                self.register.set_register_b(data);

                // キャリーフラグをセットする(ここでは必ずfalseにする)
                self.register.set_carry(false);

            }
            Opcode::OutB => {
                // Bレジスタの値をポートに出力する
                let data = self.register.register_b();
                self.port.set_output(data);

                // キャリーフラグをセットする(ここでは必ずfalseにする)
                self.register.set_carry(false);

            }
            Opcode::OutIm => {
                // オペランドをポートに出力する
                self.port.set_output(operand);

                // キャリーフラグをセットする(ここでは必ずfalseにする)
                self.register.set_carry(false);

            }
            _ => {
                // 未定義の命令は何もしない
            }
        }
        
        // プログラムカウンタをインクリメントする(ジャンプ命令の場合は除く)
        match opcode {
            Opcode::Jmp | Opcode::Jnc => {},
            _ => {
                self.register.increment_pc();
            }
        }
    }

}

impl Cpu {
    // Output Portの表示
    pub fn output(&self) -> u8 {
        self.port.output()
    }
}
