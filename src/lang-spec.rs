//Template courtesy of isaklar and maltebl, plus20

#[repr(u8)]
enum OPCODE {
    ADD = 0b000,
    LI = 0b001,
    J = 0b010,
    SYSCALL = 0b011,
    BEQ = 0b100,
    ADDI = 0b101,
}
#[repr(u8)]
enum REGISTRY {
    R1 = 0b00,
    R2 = 0b01,
    R3 = 0b10,
    R4 = 0b11,
}

#[repr(u8)]
enum SYSCALL {
    PRINT = 0b00,
    READ = 0b01,
    TERMINATE = 0b10,
}

pub const OP_DICTIONARY: [(char, u8); 8] = [
    ('黒', OPCODE::ADD as u8),
    ('白', OPCODE::LI as u8),
    ('赤', OPCODE::J as u8),
    ('青', OPCODE::SYSCALL as u8),
    ('紫', OPCODE::BEQ as u8),
    ('緑', OPCODE::ADDI as u8),
];

pub const REG_DICTIONARY: [(char, u8); 4] = [
    ('火', OPCODE::R1 as u8),
    ('水', OPCODE::R2 as u8),
    ('地', OPCODE::R3 as u8),
    ('木', OPCODE::R4 as u8),
];

pub const SYSCALL_DICTIONARY: [(char, u8); 3] = [
    ('書', OPCODE::PRINT as u8),
    ('読', OPCODE::READ as u8),
    ('殺', OPCODE::TERMINATE as u8),
];
