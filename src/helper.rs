pub mod helper {
    use crate::patch::patch::*;

    use std::error::Error;
    use std::ffi::{c_void, OsStr};
    use std::mem::size_of;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null;

    use windows::core::{self, PCWSTR};
    use windows::Win32::System::Memory::{PAGE_PROTECTION_FLAGS, VirtualProtect, PAGE_EXECUTE_READWRITE};
    use windows::Win32::System::LibraryLoader::GetModuleHandleW;


    pub unsafe fn write_to<T>(address: u32, value: T) -> Result<(), core::Error>
    where
        T: Copy,
    {
        let region = address as *mut T;

        let mut old_protect: PAGE_PROTECTION_FLAGS = Default::default();
        unsafe {
            // Disable virtual page protection
            VirtualProtect(region as *const c_void, size_of::<T>(), PAGE_EXECUTE_READWRITE, &mut old_protect)?;

            // Write
            *region = value;

            // Restore virtual page protection
            VirtualProtect(region as *const c_void, size_of::<T>(), old_protect, &mut old_protect)?;
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
    pub fn get_address_by_offset(module: Option<&str>, offset: u32) -> Result<u32, core::Error> {
        let module_name: Option<Vec<u16>> = module.map(|value| OsStr::new(value).encode_wide().chain(Some(0)).collect());
        let ptr = module_name.as_ref().map_or(null(), |v| v.as_ptr());

        Ok(unsafe { GetModuleHandleW(PCWSTR(ptr))? }.0 as u32 + offset)
    }


    pub fn apply_patchset(patchset: PatchSet) -> Result<(), Box<dyn Error>> {
        let base_address = get_address_by_offset(Some(&patchset.module), 0)?;

        for patch in patchset.set.iter() {
            let target_byte = unsafe { read_from::<u8>(base_address + patch.offset) };
            if target_byte != patch.org { return Err( Box::new(PatchError::ByteMismatch(patch.offset, patch.org, target_byte)) ); }
        }

        for patch in patchset.set.iter() {
            let patch_address = base_address + patch.offset;
            unsafe { write_to(patch_address, patch.new) }?;
        }

        Ok(())
    }

}