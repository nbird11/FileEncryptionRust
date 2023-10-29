use std::{fs, io, io::Write};

/// Basic and unsafe cryptografic algorithm.
///
/// Takes each byte in the file and shifts it by `shift_by`.
///
/// Parameter `backwards: bool` will "decrypt" the given `text`, rather than "encrypting."
///
/// Returns the new bytes (in the form of `Vec<u8>`) of the shifted content.
fn byte_shift(text: Vec<u8>, shift_by: u8, backwards: bool) -> Vec<u8> {
    text.iter()
        .map(|byte| {
            if backwards {
                byte.wrapping_sub(shift_by)
            } else {
                byte.wrapping_add(shift_by)
            }
        })
        .collect()
}

/// Program main.
///
/// Uses menu style for getting the user's choice of encryption versus decryption.
///
/// TODO Make this program a command-line application rather than an infinite
/// loop state-system.
fn main() {
    loop {
        // Print menu
        println!("\n\nMenu:");
        println!("  1: Encrypt local file");
        println!("  2: Decrypt local file");
        println!("  0: Quit");

        // Get user choice
        println!("Your choice:");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let input = buf.as_str().trim();

        match input {
            // Encrypt local file
            "1" => {
                println!("Path of file to encrypt:");
                let mut buf = String::new();
                io::stdin().read_line(&mut buf).unwrap();
                let trimmed_path = buf.as_str().trim();

                match fs::read(trimmed_path) {
                    Ok(contents) => {
                        let new_contents = byte_shift(contents, 2, false);
                        let mut new_file = fs::OpenOptions::new()
                            .write(true)
                            .open(trimmed_path)
                            .unwrap();

                        if let Err(e) = new_file.write_all(&new_contents) {
                            println!("ERROR:\t{:?}", e)
                        }
                    }

                    Err(e) => {
                        println!("Could not open file `{trimmed_path}` : {e}")
                    }
                }
            }

            // Decrypt local file
            "2" => {
                println!("Path of file to decrypt:");
                let mut buf = String::new();
                io::stdin().read_line(&mut buf).unwrap();
                let trimmed_path = buf.as_str().trim();

                match fs::read(trimmed_path) {
                    Ok(contents) => {
                        let new_contents = byte_shift(contents, 2, true);
                        let mut new_file = fs::OpenOptions::new()
                            .write(true)
                            .open(trimmed_path)
                            .unwrap();

                        if let Err(e) = new_file.write_all(&new_contents) {
                            println!("ERROR:\t{:?}", e)
                        }
                    }

                    Err(e) => {
                        println!("Could not open file `{trimmed_path}` : {e}")
                    }
                }
            }

            // Quit program
            "0" => {
                return ();
            }

            // Invalid response
            _ => {
                println!("ERROR:\tInvalid input {input}. Please try again.");
            }
        }
    }
}
