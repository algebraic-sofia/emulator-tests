// Cartesi Machine constants.

pub const PMA_SHADOW_STATE_START_DEF: u64 = 0x0;
pub const PMA_SHADOW_STATE_LENGTH_DEF: u64 = 0x1000;
pub const PMA_SHADOW_PMAS_START_DEF: u64 = 0x10000;
pub const PMA_SHADOW_PMAS_LENGTH_DEF: u64 = 0x1000;
pub const PMA_SHADOW_TLB_START_DEF: u64 = 0x20000;
pub const PMA_SHADOW_TLB_LENGTH_DEF: u64 = 0x6000;
pub const PMA_SHADOW_UARCH_STATE_START_DEF: u64 = 0x400000;
pub const PMA_SHADOW_UARCH_STATE_LENGTH_DEF: u64 = 0x1000;
pub const PMA_UARCH_RAM_START_DEF: u64 = 0x600000;
pub const PMA_UARCH_RAM_LENGTH_DEF: u64 = 0x200000;
pub const PMA_CLINT_START_DEF: u64 = 0x2000000;
pub const PMA_CLINT_LENGTH_DEF: u64 = 0xC0000;
pub const PMA_HTIF_START_DEF: u64 = 0x40008000;
pub const PMA_HTIF_LENGTH_DEF: u64 = 0x1000;
pub const PMA_FIRST_VIRTIO_START_DEF: u64 = 0x40010000;
pub const PMA_VIRTIO_LENGTH_DEF: u64 = 0x1000;
pub const PMA_LAST_VIRTIO_END_DEF: u64 = 0x40020000;
pub const PMA_DHD_START_DEF: u64 = 0x40030000;
pub const PMA_DHD_LENGTH_DEF: u64 = 0x1000;
pub const PMA_PLIC_START_DEF: u64 = 0x40100000;
pub const PMA_PLIC_LENGTH_DEF: u64 = 0x00400000;
pub const PMA_DTB_START_DEF: u64 = 0x7ff00000;
pub const PMA_DTB_LENGTH_DEF: u64 = 0x100000;
pub const PMA_RAM_START_DEF: u64 = 0x80000000;

pub const EMULATOR_VERSION_MAJOR: u64 = 9;
pub const EMULATOR_VERSION_MINOR: u64 = 15;
pub const EMULATOR_MARCHID: u64 = 15;

pub const CM_VERSION_MAJOR: u64 = EMULATOR_VERSION_MAJOR;
pub const CM_VERSION_MINOR: u64 = EMULATOR_VERSION_MINOR;

pub const XLEN: u64 = 64;

pub const MISA_EXT_S_SHIFT: u64 = ('S' as u64) - ('A' as u64);
pub const MISA_EXT_U_SHIFT: u64 = ('U' as u64) - ('A' as u64);
pub const MISA_EXT_I_SHIFT: u64 = ('I' as u64) - ('A' as u64);
pub const MISA_EXT_M_SHIFT: u64 = ('M' as u64) - ('A' as u64);
pub const MISA_EXT_A_SHIFT: u64 = ('A' as u64) - ('A' as u64);
pub const MISA_EXT_F_SHIFT: u64 = ('F' as u64) - ('A' as u64);
pub const MISA_EXT_D_SHIFT: u64 = ('D' as u64) - ('A' as u64);
pub const MISA_EXT_C_SHIFT: u64 = ('C' as u64) - ('A' as u64);
pub const MISA_MXL_SHIFT: u64 = XLEN - 2;

pub const MISA_EXT_S_MASK: u64 = 1 << MISA_EXT_S_SHIFT;
pub const MISA_EXT_U_MASK: u64 = 1 << MISA_EXT_U_SHIFT;
pub const MISA_EXT_I_MASK: u64 = 1 << MISA_EXT_I_SHIFT;
pub const MISA_EXT_M_MASK: u64 = 1 << MISA_EXT_M_SHIFT;
pub const MISA_EXT_A_MASK: u64 = 1 << MISA_EXT_A_SHIFT;
pub const MISA_EXT_F_MASK: u64 = 1 << MISA_EXT_F_SHIFT;
pub const MISA_EXT_D_MASK: u64 = 1 << MISA_EXT_D_SHIFT;
pub const MISA_EXT_C_MASK: u64 = 1 << MISA_EXT_C_SHIFT;

pub const CM_MARCHID: u64 = EMULATOR_MARCHID;

// pub const CM_MIMPID: u64 = CM_VERSION_MAJOR * 1000 + CM_VERSION_MINOR;
pub const CM_MIMPID: u64 = u64::MAX;

pub const MISA_MXL_VALUE: u64 = 2;
pub const MSTATUS_UXL_SHIFT: u64 = 32;
pub const MSTATUS_SXL_SHIFT: u64 = 34;

pub const PMA_RAM_START: u64 = 0x80000000;
pub const PC_INIT: u64 = PMA_RAM_START;
pub const FCSR_INIT: u64 = 0;
pub const MVENDORID_INIT: u64 = 0x6361727465736920;
pub const MARCHID_INIT: u64 = CM_MARCHID;
pub const MIMPID_INIT: u64 = CM_MIMPID;
pub const MCYCLE_INIT: u64 = 0;
pub const ICYCLEINSTRET_INIT: u64 = 0;
pub const MSTATUS_INIT: u64 =
    (MISA_MXL_VALUE << MSTATUS_UXL_SHIFT) | (MISA_MXL_VALUE << MSTATUS_SXL_SHIFT);

pub const MTVEC_INIT: u64 = 0;
pub const MSCRATCH_INIT: u64 = 0;
pub const MEPC_INIT: u64 = 0;
pub const MCAUSE_INIT: u64 = 0;
pub const MTVAL_INIT: u64 = 0;

pub const MISA_INIT: u64 = (MISA_MXL_VALUE << MISA_MXL_SHIFT)
    | MISA_EXT_S_MASK
    | MISA_EXT_U_MASK
    | MISA_EXT_I_MASK
    | MISA_EXT_M_MASK
    | MISA_EXT_A_MASK
    | MISA_EXT_F_MASK
    | MISA_EXT_D_MASK
    | MISA_EXT_C_MASK;

pub const PRV_U: u64 = 0;
pub const PRV_S: u64 = 1;
pub const PRV_HS: u64 = 2;
pub const PRV_M: u64 = 3;

pub const IFLAGS_PRV_SHIFT: u64 = 3;

pub const MIE_INIT : u64 = 0;
pub const MIP_INIT : u64 = 0;
pub const MEDELEG_INIT : u64 = 0;
pub const MIDELEG_INIT : u64 = 0;
pub const MCOUNTEREN_INIT : u64 = 0;
pub const STVEC_INIT : u64 = 0;
pub const SSCRATCH_INIT : u64 = 0;
pub const SEPC_INIT : u64 = 0;
pub const SCAUSE_INIT : u64 = 0;
pub const STVAL_INIT : u64 = 0;
pub const SATP_INIT : u64 = 0;
pub const SCOUNTEREN_INIT : u64 = 0;
pub const ILRSC_INIT: u64 = u64::MAX;
pub const IFLAGS_INIT: u64 = PRV_M << IFLAGS_PRV_SHIFT;

pub const MTIMECMP_INIT: u64 = 0;
pub const FROMHOST_INIT: u64 = 0;
pub const TOHOST_INIT: u64 = 0;
pub const MENVCFG_INIT: u64 = 0;
pub const SENVCFG_INIT: u64 = 0;
pub const UARCH_HALT_FLAG_INIT: u64 = 0;
pub const UARCH_X_INIT: u64 = 0;
pub const UARCH_PC_INIT: u64 = PMA_UARCH_RAM_START_DEF;
pub const UARCH_CYCLE_INIT: u64 = 0;
pub const MHARTID_INIT: u64 = 0;
pub const FDTADDR_INIT: u64 = PMA_DTB_START_DEF;

pub const REG_X0 : u64 = 0;
pub const REG_X1 : u64 = 0;
pub const REG_X2 : u64 = 0;
pub const REG_X3 : u64 = 0;
pub const REG_X4 : u64 = 0;
pub const REG_X5 : u64 = 0;
pub const REG_X6 : u64 = 0;
pub const REG_X7 : u64 = 0;
pub const REG_X8 : u64 = 0;
pub const REG_X9 : u64 = 0;
pub const REG_X10 : u64 = MHARTID_INIT;
pub const REG_X11 : u64 = FDTADDR_INIT;
pub const REG_X12 : u64 = 0;
pub const REG_X13 : u64 = 0;
pub const REG_X14 : u64 = 0;
pub const REG_X15 : u64 = 0;
pub const REG_X16 : u64 = 0;
pub const REG_X17 : u64 = 0;
pub const REG_X18 : u64 = 0;
pub const REG_X19 : u64 = 0;
pub const REG_X20 : u64 = 0;
pub const REG_X21 : u64 = 0;
pub const REG_X22 : u64 = 0;
pub const REG_X23 : u64 = 0;
pub const REG_X24 : u64 = 0;
pub const REG_X25 : u64 = 0;
pub const REG_X26 : u64 = 0;
pub const REG_X27 : u64 = 0;
pub const REG_X28 : u64 = 0;
pub const REG_X29 : u64 = 0;
pub const REG_X30 : u64 = 0;
pub const REG_X31 : u64 = 0;

