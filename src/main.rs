use encryptfile as ef;
use std::io;

fn encrypt(in_file: String) {
    let mut c = ef::Config::new();
    c.input_stream(ef::InputStream::File(in_file.to_owned()));
    c.output_stream(ef::OutputStream::File(
        format!("{:#?}.ef", in_file.strip_suffix(".txt")).to_owned(),
    ));
    // c.add_output_option(ef::OutputOption::AllowOverwrite);
    c.initialization_vector(ef::InitializationVector::GenerateFromRng);
    c.password(ef::PasswordType::Text(
        "p@ssw0rd".to_owned(),
        ef::scrypt_defaults(),
    ));
    c.encrypt();
    let _ = ef::process(&c).map_err(|e| panic!("error encrypting: {:?}", e));
}

fn decrypt(from_file: String) {
    let mut c = ef::Config::new();
    c.input_stream(ef::InputStream::File(format!("{from_file}").to_owned()))
        .output_stream(ef::OutputStream::File(
            format!("{:?}.txt", from_file.strip_suffix(".ef")).to_owned(),
        ))
        .add_output_option(ef::OutputOption::AllowOverwrite)
        .password(ef::PasswordType::Text(
            "p@ssw0rd".to_owned(),
            ef::PasswordKeyGenMethod::ReadFromFile,
        ))
        .decrypt();
    let _ = ef::process(&c).map_err(|e| panic!("error decrypting: {:?}", e));
}

fn main() {
    loop {
        println!("\n\nMenu:");
        println!("  1: Encrypt local file");
        println!("  2: Decrypt local file");
        println!("  0: Quit");
        println!("Your choice:");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();

        let input = buf.as_str().trim();

        // println!("DEBUG:\tinput = {:?}", input);

        match input {
            "1" => {
                println!("Path of file to encrypt:");
                let mut buf = String::new();
                io::stdin().read_line(&mut buf).unwrap();
                let trimmed_path = buf.as_str().trim();

                println!("DEBUG:\tfile = {:?}", trimmed_path);

                encrypt(String::from(trimmed_path));
            }
            "2" => {
                println!("Path of file to decrypt:");
                let mut buf = String::new();
                io::stdin().read_line(&mut buf).unwrap();
                let trimmed_path = buf.as_str().trim();

                decrypt(String::from(trimmed_path));
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
