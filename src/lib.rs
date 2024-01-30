mod patch;
use patch::patch::PatchSet;
use patch::i337;

mod helper;
use helper::helper::*;

mod config;
use config::conf;
use config::conf::Unwrap;

use std::ffi::OsStr;
use std::fs::{self, ReadDir};
use std::path::Path;

use windows::Win32::Foundation::{BOOL, HANDLE};
use windows::Win32::System::Console::AllocConsole;
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH};


fn main() {
    // Read the config
    let config = conf::read_safe(Path::new("runtime_patcher.conf"));


    // Attach a console so we can print stuff
    if config.get("console").unwrap().unwrap() {
        let _ = unsafe { AllocConsole() };
    }


    let patch_dir: String = config.get("patches_directory").unwrap().unwrap();

    let entries: ReadDir;
    let result = fs::read_dir(&patch_dir);
    match result {
        Ok(value) => entries = value,
        Err(err) => { println!("ERROR - Unable to read directory {}: {}", patch_dir, err); return; }
    }

    for entry in entries {
        let path = entry.unwrap().path(); // Why would this fail?

        if !path.is_file() || path.extension().unwrap_or_default() != "1337" { continue; }
        let content = fs::read_to_string(&path).expect(&format!("ERROR - Failed to read file: {}", &path.to_str().unwrap()));

        println!("INFO - Applying patch: {:?}", path.file_stem().unwrap_or(OsStr::new("")));
        let patchset: PatchSet;
        let result = i337::parse(&content);
        match result {
            Ok(value) => patchset = value,
            Err(err) => { println!("ERROR - {}", err); continue; }
        }

        let result = apply_patchset(patchset);
        match result {
            Ok(_) => continue,
            Err(err) => { println!("ERROR - Failed to apply patch: {}", err); continue; }
        }
    }
}


#[allow(unused_variables)]
#[no_mangle]
extern "system" fn DllMain(
    dll_module: HANDLE,
    call_reason: u32,
    lpv_reserverd: &u32,
) -> BOOL {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            main();
            return BOOL(1)
        }

        DLL_PROCESS_DETACH => {
            return BOOL(1)
        }

        DLL_THREAD_ATTACH => {
            return BOOL(1)
        }

        DLL_THREAD_DETACH => {
            return BOOL(1)
        }

        _ => {
            return BOOL(1)
        }
    }
}