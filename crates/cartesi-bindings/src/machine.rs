//! Structures and functions for interacting with the Cartesi Machine.

use std::{ffi::CStr, error::Error, path::Path};

use crate::{bindings::*, configuration::{MachineConfig, RuntimeConfig}};

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

pub struct Machine {
    machine: *mut cm_machine
}

impl Drop for Machine {
    fn drop(&mut self) {
        unsafe {
            cm_delete_machine(self.machine);
        }
    }
}

impl Machine {
    // Create new machine instance from configuration.
    pub fn create(config: MachineConfig, runtime: RuntimeConfig) -> Result<Self, Box<dyn Error>> {
        let mut err_msg: *mut i8 = std::ptr::null_mut();
        let mut new_machine: *mut cm_machine = std::ptr::null_mut();

        unsafe {
            let config: *const cm_machine_config = &config.into();
            let runtime: *const cm_machine_runtime_config = &runtime.into();

            let result = cm_create_machine(config, runtime, &mut new_machine, &mut err_msg);

            if result != 0 {
                let msg = CStr::from_ptr(err_msg).to_str().unwrap().to_owned();
                cm_delete_cstring(err_msg);
                return Err(msg.into());
            }
        }

        Ok(Self {
            machine: new_machine
        })
    }

    // Create machine instance from previously serialized directory
    pub fn load(path: &Path, runtime: RuntimeConfig) -> Result<Self, Box<dyn Error>> {
        let mut err_msg: *mut i8 = std::ptr::null_mut();
        let mut new_machine: *mut cm_machine = std::ptr::null_mut();

        unsafe {
            let path: *const i8 = path.to_str().unwrap().as_ptr() as *const i8;
            let runtime: *const cm_machine_runtime_config = &runtime.into();

            let result = cm_load_machine(path, runtime, &mut new_machine, &mut err_msg);

            if result != 0 {
                let msg = CStr::from_ptr(err_msg).to_str().unwrap().to_owned();
                cm_delete_cstring(err_msg);
                return Err(msg.into());
            }
        }

        Ok(Self {
            machine: new_machine
        })
    }

    pub fn run(&mut self, cycle_end: u64) -> Result<BreakReason, Box<dyn Error>> {
        let mut err_msg: *mut i8 = std::ptr::null_mut();
        let mut break_reason: u32 = 0;

        unsafe {
            let result = cm_machine_run(self.machine, cycle_end,&mut break_reason, &mut err_msg);

            if result != 0 {
                let msg = CStr::from_ptr(err_msg).to_str().unwrap().to_owned();
                cm_delete_cstring(err_msg);
                return Err(msg.into());
            }
        }

        Ok(break_reason.into())
    }
}