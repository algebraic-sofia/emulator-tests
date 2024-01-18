use std::ffi::CStr;

use bindings::{
    cm_machine_config, cm_machine_runtime_config, cm_memory_range_config,
    cm_memory_range_config_array, cm_processor_config, cm_ram_config, cm_dtb_config, cm_tlb_config, cm_clint_config, cm_htif_config, cm_rollup_config, cm_uarch_config, cm_uarch_ram_config, cm_uarch_processor_config, cm_htif_runtime_config, cm_concurrency_runtime_config, cm_machine,
};

mod bindings;

pub struct Machine {}

impl Machine {
    pub fn create(exec_args: String) -> Machine {
        let machine: *const bindings::cm_machine = std::ptr::null();

        let config = cm_machine_config {
            processor: cm_processor_config {
                x: Default::default(),
                f: Default::default(),
                pc: 0,
                fcsr: 0,
                mvendorid: u64::MAX,
                marchid: u64::MAX,
                mimpid: u64::MAX,
                mcycle: 0,
                icycleinstret: 0,
                mstatus: 0,
                mtvec: 0,
                mscratch: 0,
                mepc: 0,
                mcause: 0,
                mtval: 0,
                misa: 0,
                mie: 0,
                mip: 0,
                medeleg: 0,
                mideleg: 0,
                mcounteren: 0,
                menvcfg: 0,
                stvec: 0,
                sscratch: 0,
                sepc: 0,
                scause: 0,
                stval: 0,
                satp: 0,
                scounteren: 0,
                senvcfg: 0,
                ilrsc: 0,
                iflags: 0,
            },
            ram: cm_ram_config {
                length: 64 << 20,
                image_filename: "uwu.bin\0".as_ptr() as *const i8,
            },
            flash_drive: cm_memory_range_config_array {
                entry: vec![cm_memory_range_config {
                    start: 0x80000000000000,
                    length: 0x40000000,
                    shared: false,
                    image_filename: "uwu.bin\0".as_ptr() as *const i8,
                }]
                .as_ptr() as *mut cm_memory_range_config,
                count: 1,
            },
            dtb: cm_dtb_config {
                bootargs: format!("console=hvc0 rootfstype=ext2 root=/dev/pmem0 rw quiet -- {}\0", exec_args).as_ptr() as *const i8,
                init: std::ptr::null(),
                entrypoint: std::ptr::null(),
                image_filename: std::ptr::null(),
            },
            tlb: cm_tlb_config {
                image_filename: std::ptr::null(),
            },
            clint: cm_clint_config {
                mtimecmp: 0,
            },
            htif: cm_htif_config {
                fromhost: 0,
                tohost: 0,
                console_getchar: false,
                yield_manual: false,
                yield_automatic: false,
            },
            rollup: cm_rollup_config {
                has_value: false,
                rx_buffer: cm_memory_range_config {
                    start: 0,
                    length: 0,
                    shared: false,
                    image_filename: std::ptr::null(),
                },
                tx_buffer: cm_memory_range_config {
                    start: 0,
                    length: 0,
                    shared: false,
                    image_filename: std::ptr::null(),
                }, 
            },
            uarch: cm_uarch_config {
                processor: cm_uarch_processor_config {
                    x: Default::default(),
                    pc: 0,
                    cycle: 0,
                    halt_flag: false
                },
                ram: cm_uarch_ram_config {
                    image_filename: std::ptr::null(),
                }
            }
        };

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

        /*
        if (new_machine == nullptr) {
            throw std::invalid_argument("invalid new machine output");
        }
        */

        // allocate new machine without null pointer

        let mut machine : *mut cm_machine = std::ptr::null_mut::<cm_machine>();
        let mut err: *mut i8 = std::ptr::null_mut();

        unsafe {
            let result = bindings::cm_create_machine(
                &config,
                &runtime_config,
                &mut machine,
                &mut err
            );

            println!("{:?}", result);
            
            // print error
            if !err.is_null() {
                let err = CStr::from_ptr(err);
                println!("{:?}", err);
            }
        }


        Machine {}
    }
}
