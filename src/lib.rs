use std::{any::Any, ffi::CStr, panic, path::PathBuf};

use clap::Parser;
use server::Server;

mod protocol;
mod server;
mod vexfat;
mod utils;

#[derive(Parser, Debug)]
#[command(version, arg_required_else_help = true)]
pub struct Args {
    /// Path to OPL root directory to map into vexFAT.
    pub root: PathBuf,

    /// OPL prefix.
    #[arg(short, long)]
    pub prefix: Option<String>,
}

#[unsafe(no_mangle)]
pub extern "C" fn run_vexfat_server(game_path_pointer: *const i8) -> i32 {

    println!("UDPBD-VexFAT Started");

    let cstr: &CStr = unsafe { CStr::from_ptr(game_path_pointer) };
    // Get a copy-on-write Cow<'_, str>, then extract the
    // allocated String (or allocate a fresh one if needed).
    let path: String = cstr.to_string_lossy().into_owned();
    let mut root: PathBuf = PathBuf::new();
    root.push(path);
    let prefix: Option<String> = Option::None;
    let args: Args = Args {root, prefix};
    
    let reuslt: Result<(), Box<dyn Any + Send>> = panic::catch_unwind(|| {
        Server::new(&args).unwrap().run();
    });
    if reuslt.is_err() {
        return -1;
    }
    return 0;
}
