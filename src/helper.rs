use crate::patch::{self, *};

use std::error::Error;
use std::ffi::{c_void, OsStr};
use std::mem::size_of;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null;

use windows::core::{self, PCWSTR};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Memory::{
    VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS,
};

pub unsafe fn write_to<T>(address: u32, value: T) -> core::Result<()>
where
    T: Copy,
{
    let region = address as *mut T;

    let mut old_protect: PAGE_PROTECTION_FLAGS = Default::default();
    unsafe {
        // Disable virtual page protection
        VirtualProtect(
            region as *const c_void,
            size_of::<T>(),
            PAGE_EXECUTE_READWRITE,
            &mut old_protect,
        )?;

        // Write
        *region = value;

        // Restore virtual page protection
        VirtualProtect(
            region as *const c_void,
            size_of::<T>(),
            old_protect,
            &mut old_protect,
        )?;
    };

    Ok(())
}

pub unsafe fn read_from<T>(address: u32) -> T
where
    T: Copy,
{
    let region = address as *mut T;
    unsafe { *region }
}

// I asked ChatGPT to simplify my version (a match statement), this is what I got.
pub fn get_module_address(module: Option<&str>) -> core::Result<u32> {
    let module_name: Option<Vec<u16>> =
        module.map(|value| OsStr::new(value).encode_wide().chain(Some(0)).collect());
    let ptr = module_name.as_ref().map_or(null(), |v| v.as_ptr());

    Ok(unsafe { GetModuleHandleW(PCWSTR(ptr))? }.0 as u32)
}

pub fn apply_patchset(patchset: PatchSet) -> Result<(), Box<dyn Error>> {
    let base_address = get_module_address(Some(&patchset.module))?;

    for patch in patchset.set.iter() {
        let target_byte = unsafe { read_from::<u8>(base_address + patch.offset) };
        if target_byte == patch.org {
            continue;
        }

        return Err(Box::new(patch::Error::ByteMismatch(
            patch.offset,
            patch.org,
            target_byte,
        )));
    }

    for patch in patchset.set.iter() {
        let patch_address = base_address + patch.offset;
        unsafe { write_to(patch_address, patch.new) }?;
    }

    Ok(())
}
