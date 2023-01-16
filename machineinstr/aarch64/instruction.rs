use crate::aarch64::*;

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

// AArch64 instruction
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AArch64Instr {
    AddImm32(ShImm12RnRd),
    AddsImm32(ShImm12RnRd),
    SubImm32(ShImm12RnRd),
    SubsImm32(ShImm12RnRd),
    AddImm64(ShImm12RnRd),
    AddsImm64(ShImm12RnRd),
    SubImm64(ShImm12RnRd),
    SubsImm64(ShImm12RnRd),

    AndImm32(LogicalImm),
    OrrImm32(LogicalImm),
    EorImm32(LogicalImm),
    AndsImm32(LogicalImm),
    AndImm64(LogicalImm),
    OrrImm64(LogicalImm),
    EorImm64(LogicalImm),
    AndsImm64(LogicalImm),

    Addg(AddSubImmWithTags),
    Subg(AddSubImmWithTags),

    Extr32(ExtractImm),
    Extr64(ExtractImm),

    Sbfm32(Bitfield),
    Bfm32(Bitfield),
    Ubfm32(Bitfield),
    Sbfm64(Bitfield),
    Bfm64(Bitfield),
    Ubfm64(Bitfield),

    AddShiftedReg32(RmRnRd),
    AddsShiftedReg32(RmRnRd),
    SubShiftedReg32(RmRnRd),
    SubsShiftedReg32(RmRnRd),
    AddShiftedReg64(RmRnRd),
    AddsShiftedReg64(RmRnRd),
    SubShiftedReg64(RmRnRd),
    SubsShiftedReg64(RmRnRd),

    AddExtReg32(AddSubtractExtReg),
    AddsExtReg32(AddSubtractExtReg),
    SubExtReg32(AddSubtractExtReg),
    SubsExtReg32(AddSubtractExtReg),
    AddExtReg64(AddSubtractExtReg),
    AddsExtReg64(AddSubtractExtReg),
    SubExtReg64(AddSubtractExtReg),
    SubsExtReg64(AddSubtractExtReg),

    FmAddSinglePrecision(RmRaRnRd),
    FmSubSinglePrecision(RmRaRnRd),
    FnmAddSinglePrecision(RmRaRnRd),
    FnmSubSinglePrecision(RmRaRnRd),
    FmAddDoublePrecision(RmRaRnRd),
    FmSubDoublePrecision(RmRaRnRd),
    FnmAddDoublePrecision(RmRaRnRd),
    FnmSubDoublePrecision(RmRaRnRd),
    FmAddHalfPrecision(RmRaRnRd),
    FmSubHalfPrecision(RmRaRnRd),
    FnmAddHalfPrecision(RmRaRnRd),
    FnmSubHalfPrecision(RmRaRnRd),

    StrbImm(Imm12RnRt),
    LdrbImm(Imm12RnRt),
    Ldrsb32(Imm12RnRt),
    Ldrsb64(Imm12RnRt),
    StrImmSimdFP8(Imm12RnRt),
    LdrImmSimdFP8(Imm12RnRt),
    StrImmSimdFP128(Imm12RnRt),
    LdrImmSimdFP128(Imm12RnRt),
    StrhImm(Imm12RnRt),
    LdrhImm(Imm12RnRt),
    LdrshImm32(Imm12RnRt),
    LdrshImm64(Imm12RnRt),
    StrImmSimdFP16(Imm12RnRt),
    LdrImmSimdFP16(Imm12RnRt),
    StrImm32(Imm12RnRt),
    LdrImm32(Imm12RnRt),
    LdrswImm(Imm12RnRt),
    StrImmSimdFP32(Imm12RnRt),
    LdrImmSimdFP32(Imm12RnRt),
    StrImm64(Imm12RnRt),
    LdrImm64(Imm12RnRt),
    Prfm(Imm12RnRt),
    StrImmSimdFP64(Imm12RnRt),
    LdrImmSimdFP64(Imm12RnRt),

    StrbRegExtReg(LoadStoreRegRegOffset),
    StrbRegShiftedReg(LoadStoreRegRegOffset),
    LdrbRegExtReg(LoadStoreRegRegOffset),
    LdrbRegShiftedReg(LoadStoreRegRegOffset),
    LdrsbRegExtReg64(LoadStoreRegRegOffset),
    LdrsbRegShiftedReg64(LoadStoreRegRegOffset),
    LdrsbRegExtReg32(LoadStoreRegRegOffset),
    LdrsbRegShiftedReg32(LoadStoreRegRegOffset),
    StrRegSimdFP(LoadStoreRegRegOffset),
    LdrRegSimdFP(LoadStoreRegRegOffset),
    StrhReg(LoadStoreRegRegOffset),
    LdrhReg(LoadStoreRegRegOffset),
    LdrshReg64(LoadStoreRegRegOffset),
    LdrshReg32(LoadStoreRegRegOffset),
    StrReg32(LoadStoreRegRegOffset),
    LdrReg32(LoadStoreRegRegOffset),
    LdrswReg(LoadStoreRegRegOffset),
    StrReg64(LoadStoreRegRegOffset),
    LdrReg64(LoadStoreRegRegOffset),
    PrfmReg(LoadStoreRegRegOffset),

    Stp32(LoadStoreRegPairOffset),
    Ldp32(LoadStoreRegPairOffset),
    StpSimdFP32(LoadStoreRegPairOffset),
    LdpSimdFP32(LoadStoreRegPairOffset),
    Stgp(LoadStoreRegPairOffset),
    Ldpsw(LoadStoreRegPairOffset),
    StpSimdFP64(LoadStoreRegPairOffset),
    LdpSimdFP64(LoadStoreRegPairOffset),
    Stp64(LoadStoreRegPairOffset),
    Ldp64(LoadStoreRegPairOffset),
    StpSimdFP128(LoadStoreRegPairOffset),
    LdpSimdFP128(LoadStoreRegPairOffset),
    
    Sturb(LoadStoreRegUnscaledImm),
    Ldurb(LoadStoreRegUnscaledImm),
    Ldursb64(LoadStoreRegUnscaledImm),
    Ldursb32(LoadStoreRegUnscaledImm),
    SturSimdFP8(LoadStoreRegUnscaledImm),
    LdurSimdFP8(LoadStoreRegUnscaledImm),
    SturSimdFP128(LoadStoreRegUnscaledImm),
    LdurSimdFP128(LoadStoreRegUnscaledImm),
    Sturh(LoadStoreRegUnscaledImm),
    Ldurh(LoadStoreRegUnscaledImm),
    Ldursh64(LoadStoreRegUnscaledImm),
    Ldursh32(LoadStoreRegUnscaledImm),
    SturSimdFP16(LoadStoreRegUnscaledImm),
    LdurSimdFP16(LoadStoreRegUnscaledImm),
    Stur32(LoadStoreRegUnscaledImm),
    Ldur32(LoadStoreRegUnscaledImm),
    Ldursw(LoadStoreRegUnscaledImm),
    SturSimdFP32(LoadStoreRegUnscaledImm),
    LdurSimdFP32(LoadStoreRegUnscaledImm),
    Stur64(LoadStoreRegUnscaledImm),
    Ldur64(LoadStoreRegUnscaledImm),
    Prefum(LoadStoreRegUnscaledImm),
    SturSimdFP64(LoadStoreRegUnscaledImm),
    LdurSimdFP64(LoadStoreRegUnscaledImm),
    

    BImm(Imm26),
    BlImm(Imm26),

    BCond(Imm19Cond),
    BcCond(Imm19Cond),

    Tbz(B5B40Imm14Rt),
    Tbnz(B5B40Imm14Rt),

    Cbz32(CmpAndBranchImm),
    Cbnz32(CmpAndBranchImm),
    Cbz64(CmpAndBranchImm),
    Cbnz64(CmpAndBranchImm),

    Csel32(RmCondRnRd),
    Csinc32(RmCondRnRd),
    Csinv32(RmCondRnRd),
    Csneg32(RmCondRnRd),
    Csel64(RmCondRnRd),
    Csinc64(RmCondRnRd),
    Csinv64(RmCondRnRd),
    Csneg64(RmCondRnRd),

    Movn32(Imm16Rd),
    Movz32(Imm16Rd),
    Movk32(Imm16Rd),
    Movn64(Imm16Rd),
    Movz64(Imm16Rd),
    Movk64(Imm16Rd),

    AndShiftedReg32(ShiftRmImm6RnRd),
    BicShiftedReg32(ShiftRmImm6RnRd),
    OrrShiftedReg32(ShiftRmImm6RnRd),
    OrnShiftedReg32(ShiftRmImm6RnRd),
    EorShiftedReg32(ShiftRmImm6RnRd),
    EonShiftedReg32(ShiftRmImm6RnRd),
    AndsShiftedReg32(ShiftRmImm6RnRd),
    BicsShiftedReg32(ShiftRmImm6RnRd),

    AndShiftedReg64(ShiftRmImm6RnRd),
    BicShiftedReg64(ShiftRmImm6RnRd),
    OrrShiftedReg64(ShiftRmImm6RnRd),
    OrnShiftedReg64(ShiftRmImm6RnRd),
    EorShiftedReg64(ShiftRmImm6RnRd),
    EonShiftedReg64(ShiftRmImm6RnRd),
    AndsShiftedReg64(ShiftRmImm6RnRd),
    BicsShiftedReg64(ShiftRmImm6RnRd),

    Madd32(DataProc3Src),
    Msub32(DataProc3Src),
    Madd64(DataProc3Src),
    Msub64(DataProc3Src),
    Smaddl(DataProc3Src),
    Smsubl(DataProc3Src),
    Smulh(DataProc3Src),
    Umaddl(DataProc3Src),
    Umsubl(DataProc3Src),
    Umulh(DataProc3Src),

    RbitVar32(RnRd),
    Rev16Var32(RnRd),
    RevVar32(RnRd),
    ClzVar32(RnRd),
    ClsVar32(RnRd),
    RbitVar64(RnRd),
    Rev16Var64(RnRd),
    Rev32(RnRd),
    RevVar64(RnRd),
    ClzVar64(RnRd),
    ClsVar64(RnRd),

    Br(UncondBranchReg),
    Blr(UncondBranchReg),
    Ret(UncondBranchReg),
    ERet(UncondBranchReg),
    Drps(UncondBranchReg),

    Nop,
    Yield,
    Wfe,
    Wfi,
    Sev,
    Sevl,

    Adr(PcRelAddressing),
    Adrp(PcRelAddressing),

    Svc(ExceptionGen),
    Hvc(ExceptionGen),
    Smc(ExceptionGen),
    Brk(ExceptionGen),
    Hlt(ExceptionGen),
    TCancle(ExceptionGen),
    DcpS1(ExceptionGen),
    DcpS2(ExceptionGen),
    DcpS3(ExceptionGen),
}

