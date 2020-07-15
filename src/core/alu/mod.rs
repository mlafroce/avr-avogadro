use super::register_bank::RegisterBank;
use super::memory_bank::MemoryBank;
use super::Instruction;
use super::RawInstruction;
use super::PointerRegister;

/// # ALU
///
/// Decodes and executes instructions
pub struct Alu;
/// Arithmetic instructions (sum, substract, etc) and logic (and, or, etc)
mod arithmetic_logic;
/// Arithmetic instructions (sum, substract, etc) and logic (and, or, etc)
mod multiplication;
/// Transfer instructions (load, store and their variants)
mod transfer;
/// Call and jump Instruction
mod call_jmp;
/// Branch Instruction
mod branch;
/// Misc instructions
mod zero_reg;

pub const MOVW_OP: RawInstruction = 0x11;
pub const MULS_OP: RawInstruction = 0x12;
pub const MULSU_OP: RawInstruction = 0x13;
pub const FMUL_OP: RawInstruction = 0x14;
pub const FMULS_OP: RawInstruction = 0x15;
pub const FMULSU_OP: RawInstruction = 0x16;

impl Alu {
    /// Executes decoded operation, using registers in register_bank and data
    /// in memory_bank
    pub fn execute(instruction: &Instruction,
        register_bank: &mut RegisterBank, memory_bank: &mut MemoryBank) {
        match instruction {
            Instruction::Nop => (),
            Instruction::Branch { op, test_set, offset } =>
                Alu::execute_branch(*op, *test_set, *offset, register_bank),
            Instruction::CallJmp {is_call, relative, address} =>
                Alu::execute_calljmp(*is_call, *relative, *address, register_bank, memory_bank),
            Instruction::InOut {is_in, reg, address} =>
                Alu::execute_inout(*is_in, *reg, *address, register_bank, memory_bank),
            Instruction::PushPop {is_pop, reg} =>
                Alu::execute_push_pop(*is_pop, *reg,
                register_bank, memory_bank),
            Instruction::RegConstOp {op, rd, constant} =>
                Alu::execute_arith_with_constant(
                *op, *rd, *constant, register_bank),
            Instruction::TransferIndirect{is_load, pointer, dest, offset} =>
                Alu::execute_transfer_indirect(*is_load, *pointer, *dest,
                 *offset, register_bank, memory_bank),
            Instruction::TransferChangePointer{is_load, pointer, dest, post_inc} =>
                Alu::execute_transfer_change_pointer(*is_load, *pointer, *dest,
                 *post_inc, register_bank, memory_bank),
            Instruction::TwoRegOp {op, rd, rr} => Alu::execute_arithmetic(
                *op, *rd, *rr, register_bank, memory_bank),
            Instruction::OneRegOp {rd, op} => Alu::execute_one_reg_arithmetic(
                *op, *rd, register_bank),
            Instruction::ZeroRegOp{op} =>
                Alu::execute_zero_reg_op(*op, register_bank, memory_bank),
            _ => warn!("Execute - Unknown Instruction: {}", instruction)
        }
    }

    /// Executes arithmetic instructions
    fn execute_arithmetic(op: RawInstruction, rd: u8, rr: u8,
        register_bank: &mut RegisterBank, memory_bank: &MemoryBank) {
        let rdu = rd as usize;
        let rru = rr as usize;
        match op {
            0x1 => {
                let carry = register_bank.get_carry_as_u8();
                Alu::compare(rdu, rru, register_bank, carry)
            },
            0x2 => {
                let carry = register_bank.get_carry_as_u8();
                Alu::substract(rdu, rru, register_bank, carry);
            },
            0x3 => Alu::add(rdu, rru, register_bank, 0),
            0x4 => Alu::comp_skip(rdu, rru, register_bank, memory_bank),
            0x5 => Alu::compare(rdu, rru, register_bank, 0),
            0x6 => Alu::substract(rdu, rru, register_bank, 0),
            0x7 => {
                let carry = register_bank.get_carry_as_u8();
                Alu::add(rdu, rru, register_bank, carry)
            },
            0x8 => Alu::and(rdu, rru, register_bank),
            0x9 => Alu::eor(rdu, rru, register_bank),
            0xA => Alu::or(rdu, rru, register_bank),
            0xB ..= 0xF
                => Alu::mov(rdu, rru, register_bank), 
            MOVW_OP => Alu::movw(rdu, rru, register_bank), 
            MULS_OP => Alu::muls(rdu, rru, register_bank), 
            MULSU_OP => Alu::mulsu(rdu, rru, register_bank), 
            FMUL_OP => Alu::fmul(rdu, rru, register_bank), 
            FMULS_OP => Alu::fmuls(rdu, rru, register_bank), 
            _   => unreachable!()
        }
    }

    fn execute_arith_with_constant(op: RawInstruction, rd: u8, constant: u8,
        register_bank: &mut RegisterBank) {
        let rdu = rd as usize;
        match op {
            0x3 => Alu::cpi(rdu + 16, constant, register_bank),
            0x4 => Alu::sbci(rdu + 16, constant, register_bank),
            0x5 => Alu::subi(rdu + 16, constant, register_bank),
            0x6 => Alu::ori(rdu + 16, constant, register_bank),
            0x7 => Alu::andi(rdu + 16, constant, register_bank),
            // Technically a transfer instruction
            0xE => Alu::load_immediate(rdu + 16, constant, register_bank),
            0x96 => Alu::adiw(rdu, constant, register_bank),
            0x97 => Alu::sbiw(rdu, constant, register_bank),
            _ => warn!("Execute arith - Unknown arithmetic instruction opcode: {:x}", op)
        }
    }

    fn execute_one_reg_arithmetic(op: u8, rd: u8, register_bank: &mut RegisterBank) {
        let rdu = rd as usize;
        match op {
            0x0 => Alu::com(rdu, register_bank),
            0x1 => Alu::neg(rdu, register_bank),
            0x2 => Alu::swap(rdu, register_bank),
            0x3 => Alu::inc(rdu, register_bank),
            0x5 => Alu::asr(rdu, register_bank),
            0x6 => Alu::lsr(rdu, register_bank),
            0x7 => Alu::ror(rdu, register_bank),
            0xA => Alu::dec(rdu, register_bank),
            _ => warn!("Execute arith - Unknown arithmetic instruction opcode: {:x}", op)
        }
    }

    fn execute_zero_reg_op(op: u8,
        register_bank: &mut RegisterBank, memory_bank: &MemoryBank) {
        match op {
            0x0 => Alu::ret(false, register_bank, memory_bank),
            0x1 => Alu::ret(true, register_bank, memory_bank),
            //0x8 => Alu::sleep(),
            //0x9 => Alu::break(),
            //0xa => Alu::wdr(),
            //0xc => Alu::load_store_pm(),
            //0xd => Alu::load_store_pm(),
            //0xe => Alu::load_store_pm(),
            //0xf => Alu::load_store_pm(),
            _ => warn!("Execute zero reg op - Unknown instruction opcode: {:x}", op)
        }
    }

    fn execute_inout(is_in: bool, reg: u8, address: u8,
        register_bank: &mut RegisterBank, memory_bank: &mut MemoryBank) {
        Alu::in_out(is_in, reg, address, register_bank, memory_bank);
    }

    fn execute_push_pop(is_pop: bool, reg: u8,
        register_bank: &mut RegisterBank, memory_bank: &mut MemoryBank) {
        Alu::push_pop(is_pop, reg, register_bank, memory_bank);
    }

    fn execute_transfer_indirect(is_load: bool, pointer: PointerRegister, reg: u8, offset: u8,
        register_bank: &mut RegisterBank, memory_bank: &mut MemoryBank) {
        Alu::transfer_indirect(is_load, pointer, reg, offset, register_bank, memory_bank);
    }

    fn execute_transfer_change_pointer(is_load: bool, pointer: PointerRegister, reg: u8, post_inc: bool,
        register_bank: &mut RegisterBank, memory_bank: &mut MemoryBank) {
        Alu::transfer_change_pointer(is_load, pointer, reg, post_inc, register_bank, memory_bank);
    }
}
