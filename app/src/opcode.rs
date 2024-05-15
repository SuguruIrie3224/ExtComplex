pub enum Opcode{
    
    // imとレジスタの加算
    AddA = 0b_0000,
    AddB = 0b_0101,

    // imをレジスタにセット
    MovA = 0b_0011,
    MovB = 0b_0111,


    MovAB = 0b_0001,
    MovBA = 0b_0100,

    Jmp = 0b_1111,
    
    Jnc = 0b_1110,

    InA = 0b_0010,
    InB = 0b_0110,

    OutB = 0b_1001,
    OutIm = 0b_1011,
}