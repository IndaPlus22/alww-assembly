//Template courtesy of isaklar and maltebl, plus20
#[derive(Debug)]
pub enum OPCODE {}
impl OPCODE {
    pub const ADD: char = '黒';
    pub const LI: char = '白';
    pub const J: char = '赤';
    pub const BEQ: char = '紫';
    pub const ADDI: char = '緑';
}
pub enum REGISTRY {}
impl REGISTRY {
    pub const R1: char = '火'; //$a0
    pub const R2: char = '水'; //$a1
    pub const R3: char = '地'; //$a2
    pub const R4: char = '木'; //$v0
}

pub enum SYSCALL {}
impl SYSCALL {
    pub const PRINT: char = '書';
    pub const READ: char = '読';
    pub const TERMINATE: char = '殺';
}
