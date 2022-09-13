#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn write_output(path: &str, data: &str) {
    // create a file (overwriting if one is present)
    std::fs::File::create(format!("{}{}", path, "\\questions.json"))
        .and_then(|mut file| {
            // then, with that file, write our string as bytes directly to the file
            std::io::Write::write_all(&mut file, data.as_bytes())
                .unwrap_or_else(|err| println!("{:?}", err)); // handle the error by printing and proceeding if we hit one
            Ok(())
        })
        .unwrap_or_else(|err| println!("{:?}", err)); // handle the error by printing and proceeding if we hit one
}

fn main() {
    // boilerplate
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![write_output])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
