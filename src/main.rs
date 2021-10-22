#[allow(unused_imports)]
use cpal::{
    traits::{DeviceTrait, HostTrait,},
    available_hosts, default_host,
};
fn main() -> Result<(), anyhow::Error> {
    println!("Starting the program...");
    println!("Available hosts: {:?}", available_hosts());
    let host = default_host();
    print!("Using {:?} as the host", host.id());
    println!(" with {} as the output device", host.default_output_device().unwrap().name().unwrap());
    drop(host);
    Ok(())
}