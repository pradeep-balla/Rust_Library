use windows_sys::Win32::System::TpmBaseServices::{Tbsi_GetDeviceInfo, TPM_DEVICE_INFO};
use std::mem;

// pub fn is_tpm2() -> bool{ //for checking other versions of tpm uncomment this
//     false
// }
pub fn is_tpm2() -> bool {
    unsafe {
        // allocate the struct the API expects
        let mut device_info: TPM_DEVICE_INFO = mem::zeroed();

        // size of our struct
        let size = std::mem::size_of::<TPM_DEVICE_INFO>() as u32;

        // call API (only two args!)
        let hr = Tbsi_GetDeviceInfo(
            size,
            &mut device_info as *mut _ as *mut _,
        );

        if hr != 0 {
            eprintln!("Failed to get TPM info (HRESULT = {:#X})", hr);
            return false;
        }

        // tpmVersion = 1 means TPM 1.2, 2 means TPM 2.0
        match device_info.tpmVersion {
            2 => {
                println!("Detected TPM version 2.0");
                true
            }
            1 => {
                println!("Detected TPM version 1.2");
                false
            }
            _ => {
                println!("Detected unknown TPM version: {}", device_info.tpmVersion);
                false
            }
        }
    }
}
