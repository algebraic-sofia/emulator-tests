mod bindings;
pub mod configuration;
pub mod constants;
pub mod machine;

#[cfg(test)]
mod test {
    use crate::{
        bindings::cm_machine_config, configuration::{
            DtbConfig, MachineConfig, MemoryRangeConfig, RamConfig, RollupConfig, RuntimeConfig,
        }, constants::{LINUX_BOOTARGS, LINUX_INIT}, machine::Machine
    };

    #[test]
    pub fn test() {
        let machine_config = MachineConfig {
            rollup: Some(RollupConfig {
                rx_buffer: Some(MemoryRangeConfig {
                    start: 0x60000000,
                    length: 2 << 20,
                    shared: false,
                    image_filename: None,
                }),
                tx_buffer: Some(MemoryRangeConfig {
                    start: 0x60200000,
                    length: 2 << 20,
                    shared: false,
                    image_filename: None,
                }),
            }),
            dtb: DtbConfig {
                bootargs: Some(LINUX_BOOTARGS.to_owned()),
                init: Some(LINUX_INIT.to_owned()),
                entrypoint: Some("/mnt/dapp/d-app".to_string()),
                image_filename: None,
            },
            ram: RamConfig {
                length: Some(0x4000000),
                image_filename: Some("/usr/share/cartesi-machine/images/linux.bin".to_string()),
            },
            flash_drive: vec![
                MemoryRangeConfig {
                    start: 0x80000000000000,
                    length: 0x6400000,
                    shared: false,
                    image_filename: Some(
                        "/usr/share/cartesi-machine/images/rootfs.ext2".to_string(),
                    ),
                },
                MemoryRangeConfig {
                    start: 0x90000000000000,
                    length: 0x800000,
                    shared: false,
                    image_filename: Some("/tmp/dapp.ext2".to_string()),
                },
            ],
            ..Default::default()
        };

        let t: cm_machine_config = machine_config.clone().into();

        println!("Machine config: {:?}", t.flash_drive);

        let runtime = RuntimeConfig::default();

        let mut machine = Machine::create(machine_config, runtime).unwrap();
        let result = machine.run(u64::MAX).unwrap();

        println!("Machine run result: {:?}", result);

    }
}
