//! Module for configuration of the Cartesi Machine emulator. This module defines the
//! [MachineConfig] struct, which is used to configure the emulator.

use crate::bindings::*;
use crate::constants::*;

macro_rules! cstr {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const i8
    };
    () => {
        "\0".as_ptr() as *const i8
    };
}

macro_rules! map_to_cstring {
    ($s:expr) => {
        $s.clone().map(|x| to_leaky_cstring(&x)).unwrap_or(cstr!())
    };
}

pub fn to_leaky_cstring(s: &str) -> *const i8 {
    let cstring = std::ffi::CString::new(s).unwrap();
    let leaky = cstring.as_ptr();
    std::mem::forget(cstring);
    leaky as *const i8
}

/// Configuration of the Cartesi Machine emulator.
#[derive(Debug, Clone)]
pub struct MachineConfig {
    pub processor: ProcessorConfig,
    pub ram: RamConfig,
    pub dtb: DtbConfig,
    pub flash_drive: Vec<MemoryRangeConfig>,
    pub tlb: TlbConfig,
    pub clint: ClintConfig,
    pub htif: HtifConfig,
    pub rollup: Option<RollupConfig>,
    pub uarch: UarchConfig,
}

impl Default for MachineConfig {
    fn default() -> Self {
        unsafe {
            let config_ptr = cm_new_default_machine_config();
            let config = (*config_ptr).clone().into();
            cm_delete_machine_config(config_ptr);
            config
        }
    }
}

impl From<cm_machine_config> for MachineConfig {
    fn from(value: cm_machine_config) -> Self {
        MachineConfig {
            processor: value.processor.into(),
            ram: value.ram.into(),
            dtb: value.dtb.into(),
            flash_drive: unsafe {
                std::slice::from_raw_parts(value.flash_drive.entry, value.flash_drive.count)
                    .iter()
                    .map(|x| x.clone().into())
                    .collect::<Vec<MemoryRangeConfig>>()
            },
            tlb: value.tlb.into(),
            clint: value.clint.into(),
            htif: value.htif.into(),
            rollup: if value.rollup.has_value {
                Some(value.rollup.into())
            } else {
                None
            },
            uarch: value.uarch.into(),
        }
    }
}

impl From<MachineConfig> for cm_machine_config {
    fn from(value: MachineConfig) -> Self {
        let count = value.flash_drive.len();

        let arr = value
            .flash_drive
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<cm_memory_range_config>>();

        let entry = arr.as_ptr();

        std::mem::forget(arr);

        cm_machine_config {
            processor: value.processor.into(),
            ram: value.ram.into(),
            dtb: value.dtb.into(),
            flash_drive: cm_memory_range_config_array {
                count,
                entry: entry as *mut cm_memory_range_config,
            },
            tlb: value.tlb.into(),
            clint: value.clint.into(),
            htif: value.htif.into(),
            rollup: value.rollup.map(|x| x.into()).unwrap_or(cm_rollup_config {
                has_value: false,
                rx_buffer: unsafe { std::mem::zeroed() },
                tx_buffer: unsafe { std::mem::zeroed() },
            }),
            uarch: value.uarch.into(),
        }
    }
}

/// Rollup device state configuration
#[derive(Debug, Default, Clone)]
pub struct RollupConfig {
    /// Memory range that represents the RX buffer (receives data from the host)
    pub rx_buffer: Option<MemoryRangeConfig>,
    /// Memory range that represents the TX buffer (sends data to the host)
    pub tx_buffer: Option<MemoryRangeConfig>,
}

impl From<RollupConfig> for cm_rollup_config {
    fn from(value: RollupConfig) -> Self {
        cm_rollup_config {
            has_value: true,
            rx_buffer: value
                .rx_buffer
                .map(|x| x.into())
                .unwrap_or(cm_memory_range_config {
                    start: 0,
                    length: 0,
                    shared: false,
                    image_filename: std::ptr::null(),
                }),
            tx_buffer: value
                .tx_buffer
                .map(|x| x.into())
                .unwrap_or(cm_memory_range_config {
                    start: 0,
                    length: 0,
                    shared: false,
                    image_filename: std::ptr::null(),
                }),
        }
    }
}

impl From<cm_rollup_config> for RollupConfig {
    fn from(value: cm_rollup_config) -> Self {
        RollupConfig {
            rx_buffer: Some(value.rx_buffer.into()),
            tx_buffer: Some(value.tx_buffer.into()),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct UarchRamConfig {
    pub image_filename: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct UarchProcessorConfig {
    pub x: [u64; 32usize],
    pub pc: u64,
    pub cycle: u64,
    pub halt_flag: bool,
}

#[derive(Debug, Default, Clone)]
pub struct UarchConfig {
    pub processor: UarchProcessorConfig,
    pub ram: UarchRamConfig,
}

impl From<UarchConfig> for cm_uarch_config {
    fn from(value: UarchConfig) -> Self {
        cm_uarch_config {
            processor: cm_uarch_processor_config {
                x: value.processor.x,
                pc: value.processor.pc,
                cycle: value.processor.cycle,
                halt_flag: value.processor.halt_flag,
            },
            ram: cm_uarch_ram_config { image_filename: map_to_cstring!(value.ram.image_filename) },
        }
    }
}

impl From<cm_uarch_config> for UarchConfig {
    fn from(value: cm_uarch_config) -> Self {
        UarchConfig {
            processor: UarchProcessorConfig {
                x: value.processor.x,
                pc: value.processor.pc,
                cycle: value.processor.cycle,
                halt_flag: value.processor.halt_flag,
            },
            ram: UarchRamConfig {
                image_filename: Some(unsafe {
                    std::ffi::CStr::from_ptr(value.ram.image_filename)
                        .to_str()
                        .unwrap()
                        .to_string()
                }),
            },
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct TlbConfig {
    image_filename: Option<String>,
}

impl From<TlbConfig> for cm_tlb_config {
    fn from(value: TlbConfig) -> Self {
        cm_tlb_config {
            image_filename: map_to_cstring!(value.image_filename),
        }
    }
}

impl From<cm_tlb_config> for TlbConfig {
    fn from(value: cm_tlb_config) -> Self {
        TlbConfig {
            image_filename: Some(unsafe {
                std::ffi::CStr::from_ptr(value.image_filename)
                    .to_str()
                    .unwrap()
                    .to_string()
            }),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ClintConfig {
    mtimecmp: Option<u64>,
}

impl From<ClintConfig> for cm_clint_config {
    fn from(value: ClintConfig) -> Self {
        cm_clint_config {
            mtimecmp: value.mtimecmp.unwrap_or(0),
        }
    }
}

impl From<cm_clint_config> for ClintConfig {
    fn from(value: cm_clint_config) -> Self {
        ClintConfig {
            mtimecmp: Some(value.mtimecmp),
        }
    }
}

/// HTIF device state configuration
#[derive(Debug, Default, Clone)]
pub struct HtifConfig {
    /// Value of fromhost CSR
    fromhost: Option<u64>,
    /// Value of tohost CSR
    tohost: Option<u64>,
    /// Make console getchar available?
    console_getchar: Option<bool>,
    /// Make yield manual available?
    yield_manual: Option<bool>,
    /// Make yield automatic available?
    yield_automatic: Option<bool>,
}

impl From<HtifConfig> for cm_htif_config {
    fn from(value: HtifConfig) -> Self {
        cm_htif_config {
            fromhost: value.fromhost.unwrap_or(0),
            tohost: value.tohost.unwrap_or(0),
            console_getchar: value.console_getchar.unwrap_or(false),
            yield_manual: value.yield_manual.unwrap_or(true),
            yield_automatic: value.yield_automatic.unwrap_or(true),
        }
    }
}

impl From<cm_htif_config> for HtifConfig {
    fn from(value: cm_htif_config) -> Self {
        HtifConfig {
            fromhost: Some(value.fromhost),
            tohost: Some(value.tohost),
            console_getchar: Some(value.console_getchar),
            yield_manual: Some(value.yield_manual),
            yield_automatic: Some(value.yield_automatic),
        }
    }
}

/// Memory range configuration
#[derive(Debug, Default, Clone)]
pub struct MemoryRangeConfig {
    /// Memory range start position
    pub start: u64,
    /// Memory range length
    pub length: u64,
    /// Target changes to range affect image file?
    pub shared: bool,
    /// Memory range image file name
    pub image_filename: Option<String>,
}

impl From<MemoryRangeConfig> for cm_memory_range_config {
    fn from(value: MemoryRangeConfig) -> Self {
        cm_memory_range_config {
            start: value.start,
            length: value.length,
            shared: value.shared,
            image_filename: map_to_cstring!(value.image_filename),
        }
    }
}

impl From<cm_memory_range_config> for MemoryRangeConfig {
    fn from(value: cm_memory_range_config) -> Self {
        MemoryRangeConfig {
            start: value.start,
            length: value.length,
            shared: value.shared,
            image_filename: Some(unsafe {
                std::ffi::CStr::from_ptr(value.image_filename)
                    .to_str()
                    .unwrap()
                    .to_string()
            }),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct DtbConfig {
    /// Bootargs to pass to kernel
    pub bootargs: Option<String>,
    /// Initialization commands to be executed as root on boot
    pub init: Option<String>,
    /// Commands to execute the main application
    pub entrypoint: Option<String>,
    /// ROM image file
    pub image_filename: Option<String>,
}

impl From<DtbConfig> for cm_dtb_config {
    fn from(value: DtbConfig) -> Self {
        cm_dtb_config {
            bootargs: map_to_cstring!(value.bootargs),
            init: map_to_cstring!(value.init),
            entrypoint: map_to_cstring!(value.entrypoint),
            image_filename: map_to_cstring!(value.image_filename),
        }
    }
}

impl From<cm_dtb_config> for DtbConfig {
    fn from(value: cm_dtb_config) -> Self {
        DtbConfig {
            bootargs: Some(unsafe {
                std::ffi::CStr::from_ptr(value.bootargs)
                    .to_str()
                    .unwrap()
                    .to_string()
            }),
            init: Some(unsafe {
                std::ffi::CStr::from_ptr(value.init)
                    .to_str()
                    .unwrap()
                    .to_string()
            }),
            entrypoint: Some(unsafe {
                std::ffi::CStr::from_ptr(value.entrypoint)
                    .to_str()
                    .unwrap()
                    .to_string()
            }),
            image_filename: Some(unsafe {
                std::ffi::CStr::from_ptr(value.image_filename)
                    .to_str()
                    .unwrap()
                    .to_string()
            }),
        }
    }
}

/// RAM state configuration
#[derive(Debug, Default, Clone)]
pub struct RamConfig {
    /// Length of the RAM in bytes
    pub length: Option<u64>,
    /// Path to the RAM image file
    pub image_filename: Option<String>,
}

impl From<RamConfig> for cm_ram_config {
    fn from(value: RamConfig) -> Self {
        cm_ram_config {
            length: value.length.unwrap_or(0x4000000),
            image_filename: map_to_cstring!(value.image_filename),
        }
    }
}

impl From<cm_ram_config> for RamConfig {
    fn from(value: cm_ram_config) -> Self {
        RamConfig {
            length: Some(value.length),
            image_filename: Some(unsafe {
                std::ffi::CStr::from_ptr(value.image_filename)
                    .to_str()
                    .unwrap()
                    .to_string()
            }),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct CSR {
    pub fcsr: Option<u64>,
    pub vendor_id: Option<u64>,
    pub arch_id: Option<u64>,
    pub imp_id: Option<u64>,
    pub cycle: Option<u64>,
    pub icycleinstret: Option<u64>,
    pub mstatus: Option<u64>,
    pub mtvec: Option<u64>,
    pub mscratch: Option<u64>,
    pub mepc: Option<u64>,
    pub mcause: Option<u64>,
    pub mtval: Option<u64>,
    pub misa: Option<u64>,
    pub mie: Option<u64>,
    pub mip: Option<u64>,
    pub medeleg: Option<u64>,
    pub mideleg: Option<u64>,
    pub mcounteren: Option<u64>,
    pub menvcfg: Option<u64>,
    pub stvec: Option<u64>,
    pub sscratch: Option<u64>,
    pub sepc: Option<u64>,
    pub scause: Option<u64>,
    pub stval: Option<u64>,
    pub satp: Option<u64>,
    pub scounteren: Option<u64>,
    pub senvcfg: Option<u64>,
    pub ilrsc: Option<u64>,
    pub iflags: Option<u64>,
}

/// Initial configuration of the processor.
#[derive(Debug, Default, Clone)]
pub struct ProcessorConfig {
    /// General-purpose registers
    pub registers: Option<[u64; 32]>,
    /// Floating-point registers
    pub float_registers: Option<[u64; 32]>,
    /// Program counter value
    pub program_counter: Option<u64>,
    /// Control and Status Registers
    pub csr: Option<CSR>,
}

impl From<ProcessorConfig> for cm_processor_config {
    fn from(value: ProcessorConfig) -> Self {
        cm_processor_config {
            x: value.registers.unwrap_or([
                REG_X0, REG_X1, REG_X2, REG_X3, REG_X4, REG_X5, REG_X6, REG_X7, REG_X8, REG_X9,
                REG_X10, REG_X11, REG_X12, REG_X13, REG_X14, REG_X15, REG_X16, REG_X17, REG_X18,
                REG_X19, REG_X20, REG_X21, REG_X22, REG_X23, REG_X24, REG_X25, REG_X26, REG_X27,
                REG_X28, REG_X29, REG_X30, REG_X31,
            ]),
            f: value.float_registers.unwrap_or([0; 32]),
            pc: value.program_counter.unwrap_or(PC_INIT),
            marchid: value
                .csr
                .as_ref()
                .and_then(|csr| csr.arch_id)
                .unwrap_or(MARCHID_INIT),
            mvendorid: value
                .csr
                .as_ref()
                .and_then(|csr| csr.vendor_id)
                .unwrap_or(MVENDORID_INIT),
            mimpid: value
                .csr
                .as_ref()
                .and_then(|csr| csr.imp_id)
                .unwrap_or(MIMPID_INIT),
            misa: value
                .csr
                .as_ref()
                .and_then(|csr| csr.misa)
                .unwrap_or(MISA_INIT),
            mcause: value
                .csr
                .as_ref()
                .and_then(|csr| csr.mcause)
                .unwrap_or(MCAUSE_INIT),
            mcycle: value
                .csr
                .as_ref()
                .and_then(|csr| csr.cycle)
                .unwrap_or(MCYCLE_INIT),
            fcsr: value
                .csr
                .as_ref()
                .and_then(|csr| csr.fcsr)
                .unwrap_or(FCSR_INIT),
            icycleinstret: value
                .csr
                .as_ref()
                .and_then(|csr| csr.icycleinstret)
                .unwrap_or(ICYCLEINSTRET_INIT),
            iflags: value
                .csr
                .as_ref()
                .and_then(|csr| csr.iflags)
                .unwrap_or(IFLAGS_INIT),
            ilrsc: value
                .csr
                .as_ref()
                .and_then(|csr| csr.ilrsc)
                .unwrap_or(ILRSC_INIT),
            mstatus: value
                .csr
                .as_ref()
                .and_then(|csr| csr.mstatus)
                .unwrap_or(MSTATUS_INIT),
            mtvec: value
                .csr
                .as_ref()
                .and_then(|csr| csr.mtvec)
                .unwrap_or(MTVEC_INIT),
            mscratch: value
                .csr
                .as_ref()
                .and_then(|csr| csr.mscratch)
                .unwrap_or(MSCRATCH_INIT),
            mepc: value
                .csr
                .as_ref()
                .and_then(|csr| csr.mepc)
                .unwrap_or(MEPC_INIT),
            mtval: value
                .csr
                .as_ref()
                .and_then(|csr| csr.mtval)
                .unwrap_or(MTVAL_INIT),
            mie: value
                .csr
                .as_ref()
                .and_then(|csr| csr.mie)
                .unwrap_or(MIE_INIT),
            mip: value
                .csr
                .as_ref()
                .and_then(|csr| csr.mip)
                .unwrap_or(MIP_INIT),
            medeleg: value
                .csr
                .as_ref()
                .and_then(|csr| csr.medeleg)
                .unwrap_or(MEDELEG_INIT),
            mideleg: value
                .csr
                .as_ref()
                .and_then(|csr| csr.mideleg)
                .unwrap_or(MIDELEG_INIT),
            mcounteren: value
                .csr
                .as_ref()
                .and_then(|csr| csr.mcounteren)
                .unwrap_or(MCOUNTEREN_INIT),
            menvcfg: value
                .csr
                .as_ref()
                .and_then(|csr| csr.menvcfg)
                .unwrap_or(MENVCFG_INIT),
            stvec: value
                .csr
                .as_ref()
                .and_then(|csr| csr.stvec)
                .unwrap_or(STVEC_INIT),
            sscratch: value
                .csr
                .as_ref()
                .and_then(|csr| csr.sscratch)
                .unwrap_or(SSCRATCH_INIT),
            sepc: value
                .csr
                .as_ref()
                .and_then(|csr| csr.sepc)
                .unwrap_or(SEPC_INIT),
            scause: value
                .csr
                .as_ref()
                .and_then(|csr| csr.scause)
                .unwrap_or(SCAUSE_INIT),
            stval: value
                .csr
                .as_ref()
                .and_then(|csr| csr.stval)
                .unwrap_or(STVAL_INIT),
            satp: value
                .csr
                .as_ref()
                .and_then(|csr| csr.satp)
                .unwrap_or(SATP_INIT),
            scounteren: value
                .csr
                .as_ref()
                .and_then(|csr| csr.scounteren)
                .unwrap_or(SCOUNTEREN_INIT),
            senvcfg: value
                .csr
                .as_ref()
                .and_then(|csr| csr.senvcfg)
                .unwrap_or(SENVCFG_INIT),
        }
    }
}

impl From<cm_processor_config> for ProcessorConfig {
    fn from(value: cm_processor_config) -> Self {
        ProcessorConfig {
            registers: Some(value.x),
            float_registers: Some(value.f),
            program_counter: Some(value.pc),
            csr: Some(CSR {
                fcsr: Some(value.fcsr),
                vendor_id: Some(value.mvendorid),
                arch_id: Some(value.marchid),
                imp_id: Some(value.mimpid),
                cycle: Some(value.mcycle),
                icycleinstret: Some(value.icycleinstret),
                mstatus: Some(value.mstatus),
                mtvec: Some(value.mtvec),
                mscratch: Some(value.mscratch),
                mepc: Some(value.mepc),
                mcause: Some(value.mcause),
                mtval: Some(value.mtval),
                misa: Some(value.misa),
                mie: Some(value.mie),
                mip: Some(value.mip),
                medeleg: Some(value.medeleg),
                mideleg: Some(value.mideleg),
                mcounteren: Some(value.mcounteren),
                menvcfg: Some(value.menvcfg),
                stvec: Some(value.stvec),
                sscratch: Some(value.sscratch),
                sepc: Some(value.sepc),
                scause: Some(value.scause),
                stval: Some(value.stval),
                satp: Some(value.satp),
                scounteren: Some(value.scounteren),
                senvcfg: Some(value.senvcfg),
                ilrsc: Some(value.ilrsc),
                iflags: Some(value.iflags),
            }),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ConcurrencyRuntimeConfig {
    update_merkle_tree: Option<u64>,
}

impl From<ConcurrencyRuntimeConfig> for cm_concurrency_runtime_config {
    fn from(value: ConcurrencyRuntimeConfig) -> Self {
        cm_concurrency_runtime_config {
            update_merkle_tree: value.update_merkle_tree.unwrap_or(0),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct HtifRuntimeConfig {
    no_console_putchar: Option<bool>,
}

impl From<HtifRuntimeConfig> for cm_htif_runtime_config {
    fn from(value: HtifRuntimeConfig) -> Self {
        cm_htif_runtime_config {
            no_console_putchar: value.no_console_putchar.unwrap_or(false),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct RuntimeConfig {
    pub concurrency: ConcurrencyRuntimeConfig,
    pub htif: HtifRuntimeConfig,
    pub skip_root_hash_check: Option<bool>,
    pub skip_version_check: Option<bool>,
}

impl From<RuntimeConfig> for cm_machine_runtime_config {
    fn from(value: RuntimeConfig) -> Self {
        cm_machine_runtime_config {
            concurrency: value.concurrency.into(),
            htif: value.htif.into(),
            skip_root_hash_check: value.skip_root_hash_check.unwrap_or(false),
            skip_version_check: value.skip_version_check.unwrap_or(false),
        }
    }
}
