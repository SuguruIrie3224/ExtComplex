use app::cpu::Cpu;

fn main(){
    println!("Hello, World!");

    let mut cpu = Cpu::new();

    let program: Vec<u8> = vec![
        0b_1011_0011,
        0b_1011_0110,
        0b_1011_1100,
        0b_1011_1000,

        0b_1011_1000,
        0b_1011_1100,
        0b_1011_0110,
        0b_1011_0011,

        0b_1011_0001,
        0b_1111_0000,

    ];

    cpu.load(program);

    loop {
        let data = cpu.fetch();
        let (opcode, operand) = cpu.decode(data);
        cpu.execute(opcode, operand);

        // Outputポートを2進数の4桁で表示する
        println!("{:04b}", cpu.output());

        // 1秒まつ
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}