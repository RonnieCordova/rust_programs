use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::ToolHelp::
{CreateToolhelp32Snapshot, TH32CS_SNAPPROCESS, PROCESSENTRY32W, Process32FirstW,
    Process32NextW};     

fn main() {

    
    unsafe{
        // zero out the struct and set dwSize so the WinAPI knows what version we're passing
        let mut process_entry: PROCESSENTRY32W = std::mem::zeroed();
        process_entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;
        
        // snapshot of all running processes — kernel keeps this frozen until we close the handle
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);

        if snapshot == INVALID_HANDLE_VALUE {
            panic!("Error al crear snapshot de procesos");
        }

        // load first process into process_entry and initialize the iterator
        Process32FirstW(snapshot, &mut process_entry);

        loop{
            // szExeFile is a [u16; 260] C-style string — find the null terminator to slice only the actual name
            let fin = process_entry.szExeFile.iter().position(|&c| c == 0)
            .unwrap_or(process_entry.szExeFile.len());
            let name_process = String::from_utf16_lossy(&process_entry.szExeFile[..fin]);

            println!("{}", name_process);

            let next_process = Process32NextW(snapshot, &mut process_entry);
            if next_process == 0 {
                break;
            }

             
        }
        CloseHandle(snapshot);
    }
    
}