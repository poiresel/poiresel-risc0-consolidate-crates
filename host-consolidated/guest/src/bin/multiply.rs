// TODO: Rename this file to change the name of this method from METHOD_NAME

#![no_main]
#![no_std]  // std support is experimental, but you can remove this to try it

use risc0_zkvm;

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // TODO: Implement your guest code here
    let a: u64 = env::read();
    let b: u64 = env::read();

    if a == 1 || b == 1 {
        panic!("Trivial factors")
    }
    let product = a.checked_mul(b).expect("Integer overflow");
    env::commit(&product);
}
