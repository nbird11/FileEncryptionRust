// use encryptfile as ef;
use std::{/*env,*/ fs, io::stdin, io::Write};

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

// fn encrypt(in_file: String) {
//     let mut c = ef::Config::new();
//     c.input_stream(ef::InputStream::File(in_file.to_owned()));
//     c.output_stream(ef::OutputStream::File(
//         format!("{:#?}.ef", in_file.strip_suffix(".txt")).to_owned(),
//     ));
//     // c.add_output_option(ef::OutputOption::AllowOverwrite);
//     c.initialization_vector(ef::InitializationVector::GenerateFromRng);
//     c.password(ef::PasswordType::Text(
//         "p@ssw0rd".to_owned(),
//         ef::scrypt_defaults(),
//     ));
//     c.encrypt();
//     let _ = ef::process(&c).map_err(|e| panic!("error encrypting: {:?}", e));
// }

// fn decrypt(from_file: String) {
//     let mut c = ef::Config::new();
//     c.input_stream(ef::InputStream::File(format!("{from_file}").to_owned()))
//         .output_stream(ef::OutputStream::File(
//             format!("{:?}.txt", from_file.strip_suffix(".ef")).to_owned(),
//         ))
//         .add_output_option(ef::OutputOption::AllowOverwrite)
//         .password(ef::PasswordType::Text(
//             "p@ssw0rd".to_owned(),
//             ef::PasswordKeyGenMethod::ReadFromFile,
//         ))
//         .decrypt();
//     let _ = ef::process(&c).map_err(|e| panic!("error decrypting: {:?}", e));
// }

fn main() {
    loop {
        println!("\n\nMenu:");
        println!("  1: Encrypt local file");
        println!("  2: Decrypt local file");
        println!("  0: Quit");
        println!("Your choice:");
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();

        let input = buf.as_str().trim();

        // println!("DEBUG:\tinput = {:?}", input);

        match input {
            "1" => {
                println!("Path of file to encrypt:");
                let mut buf = String::new();
                stdin().read_line(&mut buf).unwrap();
                let trimmed_path = buf.as_str().trim();

                println!("DEBUG:\tfile = {:?}", trimmed_path);

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

                // encrypt(String::from(trimmed_path));
            }
            "2" => {
                println!("Path of file to decrypt:");
                let mut buf = String::new();
                stdin().read_line(&mut buf).unwrap();
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

                // decrypt(String::from(trimmed_path));
            }
            "0" => {
                return ();
            }
            _ => {
                println!("ERROR:\tInvalid input {input}. Please try again.");
            }
        }
    }
}
