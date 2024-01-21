use std::ffi::{CStr, CString};

use bindings::*;
use constants::*;

pub mod bindings;
pub mod constants;

pub const IMAGES_PATH: &str = "/usr/share/cartesi-machine/images";

macro_rules! cstr {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const i8
    };
    () => {
        "\0".as_ptr() as *const i8
    };
}

macro_rules! read_flag {
    ($fun:ident, $s:tt, $name:ident) => {
        pub fn $fun(&self) -> Result<$s, Box<dyn std::error::Error>> {
            unsafe {
                let mut err = std::ptr::null_mut::<i8>();
                let mut flag = Default::default();
                let result = bindings::$name(self.machine, &mut flag, &mut err);
                if result != 0 {
                    return Err(CStr::from_ptr(err).to_string_lossy().into());
                }
                Ok(flag)
            }
        }
    }
}

pub struct Machine {
    machine: *mut cm_machine,
}

#[derive(Debug)]
pub enum BreakReason {
    Failed,
    Halted,
    Limit,
    Yielded { manually: bool },
}

impl From<BreakReason> for u32 {
    fn from(reason: BreakReason) -> Self {
        match reason {
            BreakReason::Failed => 0,
            BreakReason::Halted => 1,
            BreakReason::Yielded { manually: true } => 2,
            BreakReason::Yielded { manually: false } => 3,
            BreakReason::Limit => 4,
        }
    }
}

impl From<u32> for BreakReason {
    fn from(reason: u32) -> Self {
        match reason {
            0 => BreakReason::Failed,
            1 => BreakReason::Halted,
            2 => BreakReason::Yielded { manually: true },
            3 => BreakReason::Yielded { manually: false },
            4 => BreakReason::Limit,
            _ => panic!("Invalid break reason: {}", reason),
        }
    }
}

pub const CONFIG_R: cm_machine_config = cm_machine_config {
    processor: cm_processor_config {
        x: [
            REG_X0, REG_X1, REG_X2, REG_X3, REG_X4, REG_X5, REG_X6, REG_X7, REG_X8, REG_X9,
            REG_X10, REG_X11, REG_X12, REG_X13, REG_X14, REG_X15, REG_X16, REG_X17, REG_X18,
            REG_X19, REG_X20, REG_X21, REG_X22, REG_X23, REG_X24, REG_X25, REG_X26, REG_X27,
            REG_X28, REG_X29, REG_X30, REG_X31
        ],
        f: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        pc: PC_INIT,
        marchid: MARCHID_INIT,
        mvendorid: MVENDORID_INIT,
        mimpid: MIMPID_INIT,
        misa: MISA_INIT,
        mcause: MCAUSE_INIT,
        mcycle: MCYCLE_INIT,
        fcsr: FCSR_INIT,
        icycleinstret: ICYCLEINSTRET_INIT,
        iflags: IFLAGS_INIT,
        ilrsc: ILRSC_INIT,
        mstatus: MSTATUS_INIT,
        mtvec: MTVEC_INIT,
        mscratch: MSCRATCH_INIT,
        mepc: MEPC_INIT,
        mtval: MTVAL_INIT,
        mie: MIE_INIT,
        mip: MIP_INIT,
        medeleg: MEDELEG_INIT,
        mideleg: MIDELEG_INIT,
        mcounteren: MCOUNTEREN_INIT,
        menvcfg: MENVCFG_INIT,
        stvec: STVEC_INIT,
        sscratch: SSCRATCH_INIT,
        sepc: SEPC_INIT,
        scause: SCAUSE_INIT,
        stval: STVAL_INIT,
        satp: SATP_INIT,
        scounteren: SCOUNTEREN_INIT,
        senvcfg: SENVCFG_INIT,
    },
    ram: cm_ram_config {
        length: 0x4000000,
        image_filename: cstr!("/usr/share/cartesi-machine/images/linux.bin")
    },
    flash_drive: cm_memory_range_config_array {
        entry: [
            cm_memory_range_config {
                start: 0x80000000000000,
                length: 0x6400000,
                shared: false,
                image_filename: cstr!("/usr/share/cartesi-machine/images/rootfs.ext2")
            },
            cm_memory_range_config {
                start: 0x90000000000000,
                length: 0x800000,
                shared: false,
                image_filename: cstr!("/tmp/dapp.ext2")
            }
        ].as_ptr() as *mut cm_memory_range_config,
        count: 2,
    },
    dtb: cm_dtb_config {
        image_filename: cstr!(),
        bootargs: cstr!("quiet earlycon=sbi console=hvc0 rootfstype=ext2 root=/dev/pmem0 rw init=/usr/sbin/cartesi-init"),
        init: cstr!("echo \"\n         .\n        / \\\\\n      /    \\\\\n\\\\---/---\\\\  /----\\\\\n \\\\       X       \\\\\n  \\\\----/  \\\\---/---\\\\\n       \\\\    / CARTESI\n        \\\\ /   MACHINE\n         '\n\"\nbusybox mkdir -p /run/drive-label && echo \"root\" > /run/drive-label/pmem0\nbusybox mkdir -p \"/mnt/dapp\" && busybox mount /dev/pmem1 \"/mnt/dapp\"\nbusybox mkdir -p /run/drive-label && echo \"dapp\" > /run/drive-label/pmem1\nUSER=dapp\n"),
        entrypoint: cstr!("/mnt/dapp/d-app"),
    },
    tlb: cm_tlb_config {
        image_filename: cstr!()
    },
    clint: cm_clint_config { mtimecmp: 0 },
    htif: cm_htif_config {
        fromhost: 0,
        tohost: 0,
        console_getchar: false,
        yield_manual: true,
        yield_automatic: true,
    },
    rollup: cm_rollup_config {
        has_value: true,
        rx_buffer: cm_memory_range_config {
            start: 0x60000000,
            length: 2 << 20,
            shared: false,
            image_filename: std::ptr::null(),
        },
        tx_buffer: cm_memory_range_config {
            start: 0x60200000,
            length: 2 << 20,
            shared: false,
            image_filename: std::ptr::null(),
        },
    },
    uarch: cm_uarch_config {
        processor: cm_uarch_processor_config {
            x: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            pc: 0x600000,
            cycle: 0,
            halt_flag: false,
        },
        ram: cm_uarch_ram_config {
            image_filename: cstr!()
        },
    },
};

impl Machine {
    pub fn create() -> Result<Machine, Box<dyn std::error::Error>> {
        let runtime_config = cm_machine_runtime_config {
            concurrency: cm_concurrency_runtime_config {
                update_merkle_tree: 0,
            },
            htif: cm_htif_runtime_config {
                no_console_putchar: false,
            },
            skip_root_hash_check: false,
            skip_version_check: false,
        };

        let mut machine = Machine {
            machine: std::ptr::null_mut::<cm_machine>(),
        };

        let mut err: *mut i8 = std::ptr::null_mut();

        unsafe {
            let result = bindings::cm_create_machine(
                &CONFIG_R,
                &runtime_config,
                &mut machine.machine,
                &mut err,
            );

            if result != 0 {
                return Err(CStr::from_ptr(err).to_string_lossy().into());
            }
        }

        Ok(machine)
    }

    pub fn load(directory: String) -> Result<Machine, Box<dyn std::error::Error>> {
        unsafe {
            let mut err = std::ptr::null_mut::<i8>();

            let runtime_config = cm_machine_runtime_config {
                concurrency: cm_concurrency_runtime_config {
                    update_merkle_tree: 0,
                },
                htif: cm_htif_runtime_config {
                    no_console_putchar: false,
                },
                skip_root_hash_check: false,
                skip_version_check: false,
            };

            let mut machine = Machine {
                machine: std::ptr::null_mut::<cm_machine>(),
            };

            let name = &CString::new(directory).unwrap();
            let result = bindings::cm_load_machine(name.as_ptr(), &runtime_config, &mut machine.machine, &mut err);

            if result != 0 {
                return Err(CStr::from_ptr(err).to_string_lossy().into());
            }

            Ok(machine)
        }
    }

    pub fn run(&self, target_cycle: u64) -> Result<BreakReason, Box<dyn std::error::Error>> {
        unsafe {
            let mut err = std::ptr::null_mut::<i8>();
            let mut break_reason = 0;
            let result =
                bindings::cm_machine_run(self.machine, target_cycle, &mut break_reason, &mut err);
            if result != 0 {
                return Err(CStr::from_ptr(err).to_string_lossy().into());
            }
            let break_reason = BreakReason::from(break_reason);
            Ok(break_reason)
        }
    }

    pub fn get_initial_config(&self) -> Result<&'static cm_machine_config, Box<dyn std::error::Error>> {
        unsafe {
            let mut config_ptr: *const cm_machine_config = std::ptr::null();
            let mut err = std::ptr::null_mut::<i8>();
            let result = bindings::cm_get_initial_config(self.machine, &mut config_ptr, &mut err);
            if result != 0 {
                return Err(CStr::from_ptr(err).to_string_lossy().into());
            }
            Ok(&*config_ptr)
        }
    }

    pub fn replace_memory_range(
        &self,
        new_range: *const cm_memory_range_config,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let mut err = std::ptr::null_mut::<i8>();
            let result = bindings::cm_replace_memory_range(self.machine, new_range, &mut err);
            if result != 0 {
                return Err(CStr::from_ptr(err).to_string_lossy().into());
            }
            Ok(())
        }
    }

    pub fn write_memory(
        &self,
        address: u64,
        data: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let mut err = std::ptr::null_mut::<i8>();
            let result =
                bindings::cm_write_memory(self.machine, address, data.as_ptr(), data.len(), &mut err);
            if result != 0 {
                return Err(CStr::from_ptr(err).to_string_lossy().into());
            }
            Ok(())
        }
    }

    

    read_flag!(read_mcycles, u64, cm_read_mcycle);
    read_flag!(read_y_flag, bool, cm_read_iflags_Y);
    read_flag!(read_h_flag, bool, cm_read_iflags_H);
    read_flag!(read_pc, u64, cm_read_pc);
    read_flag!(read_marchid, u64, cm_read_marchid);
    read_flag!(read_mimpid, u64, cm_read_mimpid);
    read_flag!(read_mvendorid, u64, cm_read_mvendorid);
    read_flag!(read_icycleinstret, u64, cm_read_icycleinstret);
    read_flag!(read_mstatus, u64, cm_read_mstatus);
    read_flag!(read_menvcfg, u64, cm_read_menvcfg);
    read_flag!(read_mtvec, u64, cm_read_mtvec);
    read_flag!(read_mscratch, u64, cm_read_mscratch);
    read_flag!(read_mepc, u64, cm_read_mepc);
    read_flag!(read_mcause, u64, cm_read_mcause);
    read_flag!(read_mtval, u64, cm_read_mtval);
    read_flag!(read_misa, u64, cm_read_misa);
    read_flag!(read_mie, u64, cm_read_mie);
    read_flag!(read_mip, u64, cm_read_mip);
    read_flag!(read_medeleg, u64, cm_read_medeleg);
    read_flag!(read_mideleg, u64, cm_read_mideleg);
    read_flag!(read_mcounteren, u64, cm_read_mcounteren);
    read_flag!(read_stvec, u64, cm_read_stvec);
    read_flag!(read_sscratch, u64, cm_read_sscratch);
    read_flag!(read_sepc, u64, cm_read_sepc);
    read_flag!(read_scause, u64, cm_read_scause);
    read_flag!(read_stval, u64, cm_read_stval);
    read_flag!(read_satp, u64, cm_read_satp);
    read_flag!(read_scounteren, u64, cm_read_scounteren);
    read_flag!(read_senvcfg, u64, cm_read_senvcfg);
    read_flag!(read_ilrsc, u64, cm_read_ilrsc);
    read_flag!(read_iflags, u64, cm_read_iflags);
    read_flag!(read_htif_tohost, u64, cm_read_htif_tohost);
    read_flag!(read_htif_tohost_dev, u64, cm_read_htif_tohost_dev);
    read_flag!(read_htif_tohost_cmd, u64, cm_read_htif_tohost_cmd);
    read_flag!(read_htif_tohost_data, u64, cm_read_htif_tohost_data);
    read_flag!(read_htif_fromhost, u64, cm_read_htif_fromhost);
    read_flag!(read_htif_ihalt, u64, cm_read_htif_ihalt);
    read_flag!(read_htif_iconsole, u64, cm_read_htif_iconsole);
    read_flag!(read_htif_iyield, u64, cm_read_htif_iyield);
    read_flag!(read_clint_mtimecmp, u64, cm_read_clint_mtimecmp);
    read_flag!(read_fcsr, u64, cm_read_fcsr);

    pub fn reset_y_flag(&self) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let mut err = std::ptr::null_mut::<i8>();

            let result = bindings::cm_reset_iflags_Y(self.machine, &mut err);

            if result != 0 {
                return Err(CStr::from_ptr(err).to_string_lossy().into());
            }
        }

        Ok(())
    }

}
