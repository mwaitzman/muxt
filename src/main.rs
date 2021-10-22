#![allow(unused_imports)]
use cpal::{
    traits::{DeviceTrait, HostTrait,},
    available_hosts, default_host,
};
use crossterm::{
    terminal,
    Result, execute, queue, style,
};
use std::io::{self, Write};
fn main() -> Result<()> {
    println!("Starting the program...");
    let mut stdout = io::stdout();
    run(&mut stdout)
}
fn run<W>(w: &mut W) -> crossterm::Result<()>
where
    W: Write,
{
    execute!(
        w, 
        terminal::EnterAlternateScreen,
    )?;
    terminal::enable_raw_mode()?;
    execute!(
        w,
        style::Print(format!("{:?}\n", available_hosts()))
    )?;
    let host = default_host();
    execute!(
        w,
        style::Print(format!("Using {:?} as the host", host.id())),
        style::Print(format!(" with {} as the output device", host.default_output_device().unwrap().name().unwrap())),
    )?;
    drop(host);
    loop {
    }
    /*#[allow(unreachable_code)]
    Ok(())*/
}