use std::env::args;
use sysinfo::Pid;
#[cfg(target_os = "windows")]
use sysinfo::{PidExt};
#[cfg(not(target_os = "windows"))]
use sysinfo::{ProcessExt, Signal, System, SystemExt};
#[cfg(target_os = "windows")]
use windows;

///
#[cfg(target_os = "windows")]
fn stop_process_gracefully(pid: Pid) -> windows::core::Result<bool> {
    unsafe {
        windows::Win32::System::Console::FreeConsole()?;
        windows::Win32::System::Console::AttachConsole(pid.as_u32())?;
        windows::Win32::System::Console::SetConsoleCtrlHandler(None, true)?;
        windows::Win32::System::Console::GenerateConsoleCtrlEvent(0, 0)?;
    }

    Ok(false)
}

#[cfg(not(target_os = "windows"))]
#[derive(Debug)]
struct KillError;

///
#[cfg(not(target_os = "windows"))]
fn stop_process_gracefully(pid: Pid) -> Result<bool, KillError> {
    let mut system = System::new();
    system.refresh_processes();

    // There is a process with the given PID.
    if let Some(process) = system.process(pid) {
        // Try to kill the process.
        process.kill_with(Signal::Interrupt);
        Ok(true)
    } else {
        println!("No such process with PID {pid}");
        Ok(false)
    }
}

fn main() {
    // Get the given command line arguments.
    let command_line_arguments: Vec<_> = args().collect();

    // Too few command line arguments are given.
    if command_line_arguments.len() < 2 {
        println!("Please pass the receiver's PID");
        return;
    }

    let pid = command_line_arguments[1].parse().unwrap();

    println!("Sending signal to process with PID {pid}");

    let _ = stop_process_gracefully(pid);
}
