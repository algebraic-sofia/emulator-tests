use cartesi_bindings::Machine;

pub fn main() {
    let machine = Machine::create("echo 'UwU OwO'".to_string());
}