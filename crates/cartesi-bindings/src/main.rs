use std::fs;

use cartesi_bindings::{Machine, BreakReason, CONFIG_R};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let machine = Machine::create()?;

    let _config = machine.get_initial_config()?;

    let mut i = 0;

    loop {
        let break_reason = machine.run(u64::MAX)?;

        println!("Break reason: {:?}", break_reason);

        if let BreakReason::Yielded { manually } = break_reason {
            println!("Yielded");

            let exists_file = fs::metadata(&format!("epoch-0-input-{i}.bin")).is_ok();

            if exists_file {
                machine.reset_y_flag()?;

                if manually {
                    println!("Loading");
                    let content = fs::read(&format!("epoch-0-input-{i}.bin"))?;
                    machine.replace_memory_range(&CONFIG_R.rollup.rx_buffer)?;
                    machine.write_memory(
                        CONFIG_R.rollup.rx_buffer.start,
                        &content
                    )?;
                    i += 1;
                }
            } else {
                break;
            }
        } else {
            break
        }
    }


    Ok(())
}
