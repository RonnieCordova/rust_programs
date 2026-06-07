use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::ToolHelp::
{CreateToolhelp32Snapshot, TH32CS_SNAPPROCESS, PROCESSENTRY32W, Process32FirstW,
    Process32NextW};     

fn main() {

    
    unsafe{
        //aqui inicializo la variable de tipo PROCESSENTREY32W y creo la estructura
        // en memoria con el tamaño adecuado para almacenar la informacion de cada proceso
        let mut process_entry: PROCESSENTRY32W = std::mem::zeroed();
        process_entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;
        
        //creo un snapshot de los procesos en ejecucion (handle)
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);

        if snapshot == INVALID_HANDLE_VALUE {
            panic!("Error al crear snapshot de procesos");
        }

        //coloco el primer proceso del snapshot en la estructura process_entry
        Process32FirstW(snapshot, &mut process_entry);

        loop{
            //itero para extraer el nombre del proceso, buscando el primer caracter nulo (0) para determinar el final de la cadena
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