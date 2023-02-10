use crate::compiler::aarch64_prelude::*;
use crate::compiler::Compiler;
use crate::ir::*;
use crate::register::RegId;

use machineinstr::aarch64::*;

pub struct AArch64Compiler {
    gpr_registers: [RegId; 31],
    fpr_registers: [RegId; 31],
    stack_reg: RegId,
}

impl AArch64Compiler {
    pub fn new(gpr_registers: [RegId; 31], fpr_registers: [RegId; 31], stack_reg: RegId) -> Self {
        Self {
            gpr_registers,
            fpr_registers,
            stack_reg,
        }
    }
    pub fn gpr(&self, index: u8) -> RegId {
        self.gpr_registers[index as usize]
    }

    pub fn fpr(&self, index: u8) -> RegId {
        self.fpr_registers[index as usize]
    }
}

impl Compiler for AArch64Compiler {
    type Item = AArch64Instr;

    fn compile(&self, item: Self::Item) -> IrBlock {
        println!("{item:?}");

        match item {
            AArch64Instr::MovzVar32(operand) | AArch64Instr::MovzVar64(operand) => {
                gen_movz(self, operand)
            }
            AArch64Instr::Adr(operand) => gen_adr(self, operand),
            AArch64Instr::Adrp(operand) => gen_adrp(self, operand),
            AArch64Instr::OrrShiftedReg64(operand) => gen_orr_shifted_reg(self, operand, Type::U64),
            AArch64Instr::OrrShiftedReg32(operand) => gen_orr_shifted_reg(self, operand, Type::U32),

            // Load and Stores
            AArch64Instr::LdrImm32(operand) => gen_ldr_imm(self, operand, Type::U32),
            AArch64Instr::LdrImm64(operand) => gen_ldr_imm(self, operand, Type::U64),
            AArch64Instr::StrImm32(operand) => gen_str_imm(self, operand, Type::U32),
            AArch64Instr::StrImm64(operand) => gen_str_imm(self, operand, Type::U64),
            AArch64Instr::LdrLitVar64(operand) => gen_ldr_lit_var64(self, operand),
            AArch64Instr::StpVar64(operand) => gen_stp_var(self, operand, Type::U64),
            AArch64Instr::StpVar32(operand) => gen_stp_var(self, operand, Type::U32),
            AArch64Instr::LdrhImm(operand) => gen_ldrh_imm(self, operand),
            AArch64Instr::LdrbImm(operand) => gen_ldrb_imm(self, operand),
            AArch64Instr::LdrReg32(operand) => gen_ldr_reg(self, operand, Type::U32),
            AArch64Instr::LdrReg64(operand) => gen_ldr_reg(self, operand, Type::U64),

            // Arithmetic instructions
            AArch64Instr::AddImm64(operand) => gen_add_imm64(self, operand),
            AArch64Instr::AddShiftedReg64(operand) => gen_add_shifted_reg64(self, operand),
            AArch64Instr::AddExtReg64(operand) => gen_add_ext_reg64(self, operand),
            AArch64Instr::SubImm64(operand) => gen_sub_imm64(self, operand),
            AArch64Instr::SubShiftedReg64(operand) => gen_sub_shifted_reg_64(self, operand),
            AArch64Instr::SubsShiftedReg64(operand) => gen_subs_shifted_reg_64(self, operand),
            AArch64Instr::SubsImm64(operand) => gen_subs_imm(self, operand, Type::U64),
            AArch64Instr::SubsImm32(operand) => gen_subs_imm(self, operand, Type::U32),

            // bitwise isntructions
            AArch64Instr::Ubfm64(operand) => gen_ubfm(self, operand, Type::U64),
            AArch64Instr::AndsImm64(operand) => gen_ands_imm64(self, operand),
            AArch64Instr::Sbfm64(operand) => gen_sbfm(self, operand, Type::U64),

            // Branch instructions
            AArch64Instr::BlImm(operand) => gen_bl_imm(self, operand),
            AArch64Instr::BImm(operand) => gen_b_imm(self, operand),
            AArch64Instr::Br(operand) => gen_br(self, operand),
            AArch64Instr::Blr(operand) => gen_blr(self, operand),
            AArch64Instr::BCond(operand) => gen_b_cond(self, operand),
            AArch64Instr::Cbz64(operand) => gen_cbz64(self, operand),
            AArch64Instr::Cbnz32(operand) => gen_cbnz(self, operand, Type::U32),
            AArch64Instr::Cbnz64(operand) => gen_cbnz(self, operand, Type::U64),
            AArch64Instr::Ret(operand) => gen_ret(self, operand),

            // Conditional Instructions
            AArch64Instr::CcmpImmVar32(operand) => gen_ccmp_imm(self, operand, Type::U32),
            AArch64Instr::CcmpImmVar64(operand) => gen_ccmp_imm(self, operand, Type::U64),
            AArch64Instr::Csel32(operand) => gen_csel32(self, operand),

            // Interrupt Instructions
            AArch64Instr::Svc(operand) => gen_svc(self, operand),
            AArch64Instr::Brk(operand) => gen_brk(self, operand),

            // Speical instructions
            AArch64Instr::Mrs(operand) => gen_mrs(self, operand),
            AArch64Instr::Nop | AArch64Instr::Wfi => {
                let mut block = IrBlock::new(4);

                let ir = Ir::Nop;
                let ds = BlockDestination::None;
                block.append(ir, ds);

                block
            }

            _ => unimplemented!("unimplemented instruction: {:?}", item),
        }
    }
}

fn gen_movz(compiler: &AArch64Compiler, operand: HwImm16Rd) -> IrBlock {
    let mut block = IrBlock::new(4);
    let pos = operand.hw << 4;

    let ir = Ir::Value(Operand::imm(Type::U64, (operand.imm16 as u64) << pos));
    let ds = BlockDestination::Gpr(compiler.gpr(operand.rd));
    block.append(ir, ds);

    block
}

fn gen_adr(compiler: &AArch64Compiler, operand: PcRelAddressing) -> IrBlock {
    let mut block = IrBlock::new(4);
    let imm = sign_extend((operand.immhi as i64) << 2 | (operand.immlo as i64), 21);

    let ir = gen_ip_relative(imm);
    let ds = BlockDestination::Gpr(compiler.gpr(operand.rd));
    block.append(ir, ds);

    block
}

fn gen_adrp(compiler: &AArch64Compiler, operand: PcRelAddressing) -> IrBlock {
    let mut block = IrBlock::new(4);

    let imm = sign_extend(
        ((operand.immhi as i64) << 2 | (operand.immlo as i64)) << 12,
        33,
    );

    let ir = Ir::Add(
        Type::U64,
        Operand::ir(Ir::And(
            Type::U64,
            Operand::Ip,
            Operand::imm(Type::U64, 0xFFFFFFFF_FFFFF000),
        )),
        Operand::imm(Type::I64, imm as u64),
    );
    let ds = BlockDestination::Gpr(compiler.gpr(operand.rd));
    block.append(ir, ds);

    block
}

fn gen_orr_shifted_reg(compiler: &AArch64Compiler, operand: ShiftRmImm6RnRd, ty: Type) -> IrBlock {
    let mut block = IrBlock::new(4);
    let rm = compiler.gpr(operand.rm);
    let rd = compiler.gpr(operand.rd);

    if operand.imm6 == 0 && operand.shift == 0 && operand.rn == 0b11111 {
        let ir = Ir::Value(Operand::reg(ty, rm));
        let ds = BlockDestination::Gpr(rd);

        block.append(ir, ds);
    } else {
        // let rn = self.gpr(operand.rn);

        todo!()
    }

    block
}

fn gen_ldr_imm(compiler: &AArch64Compiler, operand: SizeImm12RnRt, ty: Type) -> IrBlock {
    let mut block = IrBlock::new(4);

    let (mut wback, post_index, _scale, offset) = decode_operand_for_ld_st_reg_imm(operand);
    let pre_offs = if post_index { 0 } else { offset };

    if wback && operand.rn == operand.rt && operand.rn != 31 {
        wback = false;
    }

    let dst = compiler.gpr(operand.rt);
    let src = if operand.rn == 31 {
        // If rn is 31, we use stack register instead of gpr registers.
        compiler.stack_reg
    } else {
        compiler.gpr(operand.rn)
    };

    let ir = Ir::Load(
        ty,
        Operand::ir(Ir::Add(
            Type::U64,
            Operand::reg(Type::U64, src),
            Operand::imm(Type::U64, pre_offs as u64),
        )),
    );
    let ds = BlockDestination::Gpr(dst);

    block.append(ir, ds);

    if wback {
        let ir = Ir::Add(
            Type::U64,
            Operand::reg(Type::U64, src),
            Operand::Immediate(Type::I64, offset as u64),
        );
        let ds = BlockDestination::Gpr(src);

        block.append(ir, ds);
    }

    block
}

fn gen_str_imm(compiler: &AArch64Compiler, operand: SizeImm12RnRt, ty: Type) -> IrBlock {
    let mut block = IrBlock::new(4);

    let (wback, post_index, _scale, offset) = decode_operand_for_ld_st_reg_imm(operand);
    let pre_offs = if post_index { 0 } else { offset };

    let rn = if operand.rn == 31 {
        compiler.stack_reg
    } else {
        compiler.gpr(operand.rn)
    };

    let rt = if operand.rt == 31 {
        Operand::imm(Type::U64, 0)
    } else {
        Operand::reg(Type::U64, compiler.gpr(operand.rt))
    };

    let ir = Ir::Value(rt);
    let ds = BlockDestination::MemoryRel(rn, pre_offs);
    block.append(ir, ds);

    if wback {
        let off_ty = match ty {
            Type::U64 => Type::I64,
            Type::U32 => Type::I32,
            _ => unreachable!(),
        };
        let ir = Ir::Add(
            ty,
            Operand::reg(ty, rn),
            Operand::imm(off_ty, offset as u64),
        );
        let ds = BlockDestination::Gpr(rn);

        block.append(ir, ds);
    }

    block
}

fn gen_ldr_lit_var64(compiler: &AArch64Compiler, operand: Imm19Rt) -> IrBlock {
    let mut block = IrBlock::new(4);

    let offset = sign_extend((operand.imm19 << 2) as i64, 21);

    let ir = Ir::Load(Type::U64, Operand::ir(gen_ip_relative(offset)));
    let ds = BlockDestination::Gpr(compiler.gpr(operand.rt));

    block.append(ir, ds);

    block
}

fn gen_stp_var(compiler: &AArch64Compiler, operand: LoadStoreRegPair, ty: Type) -> IrBlock {
    let mut block = IrBlock::new(4);

    let (wback, post_index) = decode_o_for_ld_st_pair_offset(operand.o);
    let offset = sign_extend(operand.imm7 as i64, 7) << 3;

    let dst = if operand.rn == 31 {
        compiler.stack_reg
    } else {
        compiler.gpr(operand.rn)
    };

    let offset_temp = if !post_index { offset } else { 0 };

    let data1 = compiler.gpr(operand.rt);
    let data2 = compiler.gpr(operand.rt2);

    let ir = Ir::Value(Operand::Register(ty, data1));
    let ds = BlockDestination::MemoryRel(dst, offset_temp);
    block.append(ir, ds);

    let ir = Ir::Value(Operand::Register(ty, data2));
    let ds = BlockDestination::MemoryRel(dst, offset_temp + 8);
    block.append(ir, ds);

    if wback {
        let ir = Ir::Add(
            Type::U64,
            Operand::Register(Type::U64, dst),
            Operand::Immediate(Type::I64, offset as u64),
        );
        let ds = BlockDestination::Gpr(dst);
        block.append(ir, ds)
    }

    block
}

fn gen_add_imm64(compiler: &AArch64Compiler, operand: ShImm12RnRd) -> IrBlock {
    let mut block = IrBlock::new(4);

    let rd = compiler.gpr(operand.rd);
    let rn = if operand.rn == 31 {
        compiler.stack_reg
    } else {
        compiler.gpr(operand.rn)
    };

    let imm = match operand.sh {
        0b00 => operand.imm12 as u64,
        0b01 => (operand.imm12 as u64) << 12,
        _ => unreachable!(),
    };

    let ir = Ir::Add(
        Type::U64,
        Operand::reg(Type::U64, rn),
        Operand::imm(Type::U64, imm),
    );
    let ds = BlockDestination::Gpr(rd);

    block.append(ir, ds);

    block
}

fn gen_add_shifted_reg64(compiler: &AArch64Compiler, operand: ShiftRmImm6RnRd) -> IrBlock {
    let mut block = IrBlock::new(4);

    let rn = compiler.gpr(operand.rn);
    let rm = compiler.gpr(operand.rm);
    let rd = compiler.gpr(operand.rd);

    let sh = shift_reg(
        rm,
        decode_shift(operand.shift),
        operand.imm6 as u64,
        Type::U64,
    );
    let ir = Ir::Add(
        Type::U64,
        Operand::reg(Type::U64, rn),
        Operand::Ir(Box::new(sh)),
    );

    let ds = BlockDestination::Gpr(rd);
    block.append(ir, ds);

    block
}

fn gen_sub_imm64(compiler: &AArch64Compiler, operand: ShImm12RnRd) -> IrBlock {
    let mut block = IrBlock::new(4);

    let rd = compiler.gpr(operand.rd);
    let rn = if operand.rn == 31 {
        compiler.stack_reg
    } else {
        compiler.gpr(operand.rn)
    };

    let imm = match operand.sh {
        0b00 => operand.imm12 as u64,
        0b01 => (operand.imm12 as u64) << 12,
        _ => unreachable!(),
    };

    let ir = Ir::Sub(
        Type::U64,
        Operand::reg(Type::U64, rn),
        Operand::imm(Type::U64, imm),
    );
    let ds = BlockDestination::Gpr(rd);

    block.append(ir, ds);

    block
}

fn gen_sub_shifted_reg_64(compiler: &AArch64Compiler, operand: ShiftRmImm6RnRd) -> IrBlock {
    let mut block = IrBlock::new(4);

    let rd = compiler.gpr(operand.rd);
    let rn = compiler.gpr(operand.rn);
    let rm = compiler.gpr(operand.rm);

    let sh = shift_reg(
        rm,
        decode_shift(operand.shift),
        operand.imm6 as u64,
        Type::U64,
    );

    let ir = Ir::Sub(
        Type::U64,
        Operand::reg(Type::U64, rn),
        Operand::Ir(Box::new(sh)),
    );

    let ds = BlockDestination::Gpr(rd);

    block.append(ir, ds);

    block
}

fn gen_subs_shifted_reg_64(compiler: &AArch64Compiler, operand: ShiftRmImm6RnRd) -> IrBlock {
    let mut block = IrBlock::new(4);

    let rn = compiler.gpr(operand.rn);
    let rm = compiler.gpr(operand.rm);

    let sh = shift_reg(
        rm,
        decode_shift(operand.shift),
        operand.imm6 as u64,
        Type::U64,
    );
    let ir = Ir::Subc(
        Type::U64,
        Operand::reg(Type::U64, rn),
        Operand::Ir(Box::new(sh)),
    );

    let ds = if operand.rd == 31 {
        BlockDestination::None
    } else {
        BlockDestination::Gpr(compiler.gpr(operand.rd))
    };

    block.append(ir, ds);

    block
}

fn gen_subs_imm(compiler: &AArch64Compiler, operand: ShImm12RnRd, size: Type) -> IrBlock {
    let mut block = IrBlock::new(4);

    let imm = match operand.sh {
        0b00 => operand.imm12 as u64,
        0b01 => (operand.imm12 as u64) << 12,
        _ => unreachable!(),
    };

    let rn = if operand.rn == 0b11111 {
        compiler.stack_reg
    } else {
        compiler.gpr(operand.rn)
    };

    // If rd is 31, its alias is CMP(immediate).
    let ds = if operand.rd == 0b11111 {
        BlockDestination::None
    } else {
        BlockDestination::Gpr(compiler.gpr(operand.rd))
    };

    let ir = Ir::Subc(size, Operand::reg(size, rn), Operand::imm(size, imm));

    block.append(ir, ds);

    block
}

fn gen_ands_imm64(compiler: &AArch64Compiler, operand: LogicalImm) -> IrBlock {
    let mut block = IrBlock::new(4);

    let (imm, _) = decode_bit_masks(operand.n, operand.imms, operand.immr, true, 64);
    let rn = Operand::reg(Type::U64, compiler.gpr(operand.rn));

    let ir = Ir::And(Type::U64, rn, Operand::imm(Type::U64, imm));
    let ds = BlockDestination::Gpr(compiler.gpr(operand.rd));
    block.append(ir.clone(), ds);

    let ds = BlockDestination::None;
    let ir = Ir::Addc(Type::U64, Operand::ir(ir), Operand::imm(Type::U64, 0)); // Only for flag setting
    block.append(ir, ds);

    block
}

fn gen_bl_imm(compiler: &AArch64Compiler, operand: Imm26) -> IrBlock {
    let mut block = IrBlock::new(4);

    let ir = Ir::Add(Type::U64, Operand::Ip, Operand::imm(Type::U64, 4));
    let ds = BlockDestination::Gpr(compiler.gpr(30));

    block.append(ir, ds);

    let imm = sign_extend((operand.imm26 << 2) as i64, 28);

    let ir = gen_ip_relative(imm);
    let ds = BlockDestination::Ip;

    block.append(ir, ds);

    block
}

fn gen_b_imm(_compiler: &AArch64Compiler, operand: Imm26) -> IrBlock {
    let mut block = IrBlock::new(4);

    let imm = sign_extend((operand.imm26 << 2) as i64, 28);

    let ir = gen_ip_relative(imm);
    let ds = BlockDestination::Ip;

    block.append(ir, ds);

    block
}

fn gen_br(compiler: &AArch64Compiler, operand: UncondBranchReg) -> IrBlock {
    let mut block = IrBlock::new(4);

    let ir = Ir::Value(Operand::reg(Type::U64, compiler.gpr(operand.rn)));
    let ds = BlockDestination::Ip;

    block.append(ir, ds);

    block
}

fn gen_b_cond(_compiler: &AArch64Compiler, operand: Imm19Cond) -> IrBlock {
    let mut block = IrBlock::new(4);

    let offset = operand.imm19 << 2;
    let ir = Ir::If(
        Type::U64,
        condition_holds(operand.cond),
        Operand::ir(gen_ip_relative(offset as i64)),
        Operand::ir(gen_ip_relative(4)),
    );
    let ds = BlockDestination::Ip;

    block.append(ir, ds);

    block
}

fn gen_cbz64(compiler: &AArch64Compiler, operand: Imm19Rt) -> IrBlock {
    let mut block = IrBlock::new(4);

    let offset = sign_extend((operand.imm19 << 2) as i64, 21);

    let is_zero = cmp_eq_op_imm64(Operand::Register(Type::U64, compiler.gpr(operand.rt)), 0);
    let ir = Ir::If(
        Type::U64,
        is_zero,
        Operand::ir(gen_ip_relative(offset)),
        Operand::ir(gen_ip_relative(4)),
    );
    let ds = BlockDestination::Ip;

    block.append(ir, ds);

    block
}

fn gen_cbnz(compiler: &AArch64Compiler, operand: Imm19Rt, ty: Type) -> IrBlock {
    let mut block = IrBlock::new(4);

    let offset = sign_extend((operand.imm19 << 2) as i64, 21);

    let is_zero = if ty == Type::U64 {
        cmp_eq_op_imm64(Operand::Register(ty, compiler.gpr(operand.rt)), 0)
    } else {
        cmp_eq_op_imm32(Operand::Register(ty, compiler.gpr(operand.rt)), 0)
    };

    let ir = Ir::If(
        Type::U64,
        is_zero,
        Operand::ir(gen_ip_relative(4)),
        Operand::ir(gen_ip_relative(offset)),
    );
    let ds = BlockDestination::Ip;

    block.append(ir, ds);

    block
}

fn gen_ccmp_imm(compiler: &AArch64Compiler, operand: CondCmpImm, ty: Type) -> IrBlock {
    let mut block = IrBlock::new(4);

    let rn = compiler.gpr(operand.rn);

    let subc = Operand::void_ir(Ir::Subc(
        ty,
        Operand::reg(ty, rn),
        Operand::imm(ty, operand.imm5 as u64),
    ));

    let ir = Ir::If(
        Type::Void,
        condition_holds(operand.cond),
        Operand::ir(Ir::Or(
            Type::U64,
            Operand::Flag,
            Operand::ir(Ir::BitCast(Type::U64, subc)),
        )),
        Operand::ir(replace_bits(Operand::Flag, operand.nzcv as u64, 60..64)),
    );
    let ds = BlockDestination::Flags;

    block.append(ir, ds);

    block
}

fn gen_csel32(compiler: &AArch64Compiler, operand: RmCondRnRd) -> IrBlock {
    let mut block = IrBlock::new(4);

    let rn = if operand.rn == 31 {
        Operand::imm(Type::U32, 0)
    } else {
        Operand::reg(Type::U32, compiler.gpr(operand.rn))
    };

    let rm = if operand.rm == 31 {
        Operand::imm(Type::U32, 0)
    } else {
        Operand::reg(Type::U32, compiler.gpr(operand.rm))
    };
    let rd = compiler.gpr(operand.rd);

    let ir = Ir::If(Type::U32, condition_holds(operand.cond), rn, rm);
    let ds = BlockDestination::Gpr(rd);

    block.append(ir, ds);

    block
}

fn gen_svc(_compiler: &AArch64Compiler, operand: ExceptionGen) -> IrBlock {
    let mut block = IrBlock::new(4);

    let ir = Ir::Value(Operand::imm(Type::U16, operand.imm16 as u64));
    let ds = BlockDestination::SystemCall;

    block.append(ir, ds);

    block
}

fn gen_brk(_compiler: &AArch64Compiler, operand: ExceptionGen) -> IrBlock {
    let mut block = IrBlock::new(4);

    let ir = Ir::Value(Operand::imm(Type::U16, operand.imm16 as u64));
    let ds = BlockDestination::Exit;

    block.append(ir, ds);

    block
}

fn gen_ubfm(compiler: &AArch64Compiler, operand: Bitfield, ty: Type) -> IrBlock {
    let mut block = IrBlock::new(4);

    let src = Operand::Register(ty, compiler.gpr(operand.rn));
    let r = Operand::Immediate(Type::U8, operand.immr as u64);

    let (wmask, tmask) = decode_bit_masks(
        operand.n,
        operand.imms,
        operand.immr,
        false,
        (ty.size() * 8) as u8,
    );

    let bot = Ir::And(
        ty,
        Operand::ir(Ir::Rotr(ty, src, r)),
        Operand::Immediate(ty, wmask),
    );
    let ir = Ir::And(ty, Operand::ir(bot), Operand::Immediate(ty, tmask));
    let ds = BlockDestination::Gpr(compiler.gpr(operand.rd));

    block.append(ir, ds);

    block
}

fn gen_sbfm(compiler: &AArch64Compiler, operand: Bitfield, ty: Type) -> IrBlock {
    let mut block = IrBlock::new(4);

    let src = Operand::Register(ty, compiler.gpr(operand.rn));
    let r = Operand::Immediate(Type::U8, operand.immr as u64);
    let datasize = (ty.size() * 8) as u8;

    let (wmask, tmask) = decode_bit_masks(operand.n, operand.imms, operand.immr, false, datasize);

    let bot = Ir::And(
        ty,
        Operand::ir(Ir::Rotr(ty, src.clone(), r)),
        Operand::Immediate(ty, wmask),
    );

    let top = replicate_reg64(compiler.gpr(operand.rn), operand.imms);

    let lhs = Ir::And(ty, Operand::ir(top), Operand::Immediate(ty, !tmask));
    let rhs = Ir::And(ty, Operand::ir(bot), Operand::Immediate(ty, tmask));

    let ir = Ir::Or(ty, Operand::ir(lhs), Operand::ir(rhs));
    let ds = BlockDestination::Gpr(compiler.gpr(operand.rd));

    block.append(ir, ds);

    block
}

fn gen_ldrb_imm(compiler: &AArch64Compiler, operand: SizeImm12RnRt) -> IrBlock {
    let mut block = IrBlock::new(4);

    let (wback, post_index, _scale, offset) = decode_operand_for_ld_st_reg_imm(operand);

    let pre_offs = if post_index { 0 } else { offset };

    let dst = compiler.gpr(operand.rt);
    let src = if operand.rn == 31 {
        // If rn is 31, we use stack register instead of gpr registers.
        compiler.stack_reg
    } else {
        compiler.gpr(operand.rn)
    };

    let ir = Ir::Load(
        Type::U8,
        Operand::ir(Ir::Add(
            Type::U64,
            Operand::reg(Type::U64, src),
            Operand::imm(Type::U64, pre_offs as u64),
        )),
    );
    let ir = Ir::ZextCast(Type::U32, Operand::ir(ir));
    let ds = BlockDestination::Gpr(dst);

    block.append(ir, ds);

    if wback {
        let ir = Ir::Add(
            Type::U64,
            Operand::reg(Type::U64, src),
            Operand::imm(Type::I64, offset as u64),
        );
        let ds = BlockDestination::Gpr(src);

        block.append(ir, ds);
    }

    block
}

fn gen_ret(compiler: &AArch64Compiler, operand: UncondBranchReg) -> IrBlock {
    let mut block = IrBlock::new(4);

    let ir = Ir::Value(Operand::reg(Type::U64, compiler.gpr(operand.rn)));
    let ds = BlockDestination::Ip;
    block.append(ir, ds);

    block
}

fn gen_add_ext_reg64(compiler: &AArch64Compiler, operand: AddSubtractExtReg) -> IrBlock {
    let mut block = IrBlock::new(4);

    let ext_type = decode_reg_extend(operand.option);
    let shift = operand.imm3;
    assert!(shift <= 4);

    let op1 = if operand.rn == 31 {
        compiler.stack_reg
    } else {
        compiler.gpr(operand.rn)
    };
    let op2 = extend_reg(compiler.gpr(operand.rm), ext_type, shift, 64 / 8);

    let ir = Ir::Add(Type::U64, Operand::reg(Type::U64, op1), Operand::ir(op2));
    let ds = if operand.rd == 31 {
        compiler.stack_reg
    } else {
        compiler.gpr(operand.rd)
    };
    let ds = BlockDestination::Gpr(ds);

    block.append(ir, ds);

    block
}

fn gen_ldrh_imm(compiler: &AArch64Compiler, operand: SizeImm12RnRt) -> IrBlock {
    let mut block = IrBlock::new(4);

    let (_wback, post_index, _scale, offset) = decode_operand_for_ld_st_reg_imm(operand);

    let src = if operand.rn == 31 {
        compiler.stack_reg
    } else {
        compiler.gpr(operand.rn)
    };

    let offset_temp = if !post_index { offset } else { 0 };

    let ir = Ir::Load(
        Type::U16,
        Operand::ir(Ir::Add(
            Type::U64,
            Operand::Register(Type::U64, src),
            Operand::Immediate(Type::I64, offset_temp as u64),
        )),
    );

    let ir = Ir::ZextCast(Type::U32, Operand::ir(ir));
    let ds = BlockDestination::Gpr(compiler.gpr(operand.rt));

    block.append(ir, ds);

    block
}

fn gen_mrs(compiler: &AArch64Compiler, operand: SysRegMov) -> IrBlock {
    let mut block = IrBlock::new(4);

    // TODO: emulate system registers
    let op = match (
        operand.o0 + 2,
        operand.op1,
        operand.crn,
        operand.crm,
        operand.op2,
    ) {
        (0b11, 0b011, 0b1101, 0b0000, 0b010) => Operand::Immediate(Type::U64, 0x00000000004DFD58), // tpidr_el10, get current thread.
        _ => unimplemented!("MRS: {:x?}", operand),
    };

    let ir = Ir::Value(op);
    let ds = BlockDestination::Gpr(compiler.gpr(operand.rt));

    block.append(ir, ds);

    block
}

fn gen_ldr_reg(compiler: &AArch64Compiler, operand: LoadStoreRegRegOffset, ty: Type) -> IrBlock {
    let mut block = IrBlock::new(4);

    let ext_type = decode_reg_extend(operand.option);
    let shift = if operand.s == 1 { operand.size } else { 0 };

    let offset = extend_reg(compiler.gpr(operand.rm), ext_type, shift, ty.size() as u8);
    let offset = Ir::SextCast(Type::I64, Operand::ir(offset));

    let src = if operand.rn == 31 {
        compiler.stack_reg
    } else {
        compiler.gpr(operand.rn)
    };

    let ir = Ir::Load(
        ty,
        Operand::ir(Ir::Add(
            Type::U64,
            Operand::dbg("from register1:", Operand::Register(Type::U64, src)),
            Operand::dbg("from register2:", Operand::ir(offset)),
        )),
    );
    let ir = Ir::ZextCast(ty, Operand::ir(ir));
    let ds = BlockDestination::Gpr(compiler.gpr(operand.rt));

    block.append(ir, ds);

    block
}

fn gen_blr(compiler: &AArch64Compiler, operand: UncondBranchReg) -> IrBlock {
    let mut block = IrBlock::new(4);

    let ir = Ir::Add(Type::U64, Operand::Ip, Operand::Immediate(Type::U64, 4));
    let ds = BlockDestination::Gpr(RegId(30));

    block.append(ir, ds);

    let ir = Ir::Value(Operand::Register(Type::U64, compiler.gpr(operand.rn)));
    let ds = BlockDestination::Ip;

    block.append(ir, ds);

    block
}