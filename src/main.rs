use anyhow::{Error, Result};
use std::io;
use io::{Read, Write, BufReader};
use structopt::clap::AppSettings;
use structopt::StructOpt;
use std::fs::{File};
use secrecy::Secret;

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands)]
enum Opt {
    #[structopt(alias = "insert")]
    Add,
    #[structopt(alias = "list")]
    Show {
        filename: String
    },
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Add => add(),
        Opt::Show { filename } => show(&filename)
    };
}

fn add() -> Result<(), Error> {
    println!("Enter your credential: ");
    io::stdout().flush()?;

    let mut credential = String::new();
    io::stdin().read_line(&mut credential)?;

    let password = rpassword::prompt_password_stdout("Enter a password: ")?;
    let passphrase = rpassword::prompt_password_stdout("Enter a passphrase: ")?;

    let encrypted_pwd = encrypt(password, passphrase);

    // io::stdout().flush()?;

    // println!("Enter your filename: ");
    // let filename = String::new();
    // io::stdin().read_line(&mut credential)?;

    let mut file = File::create(format!("{}/{}.txt", home_dir(), "test"))?;
    file.write(&encrypted_pwd?);
    
    Ok(())
}

fn show(filename: &str) -> Result<(), Error> {
    let file = File::open(format!("{}/{}.txt",  home_dir(), filename))?;
    let mut buffer = BufReader::new(file);
    
    let mut encrypted = vec![];
    buffer.read_to_end(&mut encrypted)?;
    
    let passphrase = rpassword::prompt_password_stdout("Enter your passphrase: ")?;
    let decrypted_pwd = decrypt(&encrypted, passphrase)?;
    let decrypted_pwd = String::from_utf8(decrypted_pwd)?;
    
    println!("{}", decrypted_pwd);
    Ok(())    
}

fn encrypt(password: String, passphrase: String) -> Result<Vec<u8>, Error> {
    let encryptor = age::Encryptor::with_user_passphrase(Secret::new(passphrase));

    let mut encrypted = vec![];
    let mut writer = encryptor.wrap_output(&mut encrypted, age::Format::Binary)?;
    writer.write_all(&password.as_bytes())?;
    writer.finish()?;

    Ok(encrypted)
}

fn decrypt(encrypted_pwd: &[u8], passphrase: String) -> Result<Vec<u8>, Error> {
    let decryptor = match age::Decryptor::new(&encrypted_pwd[..])? {
        age::Decryptor::Passphrase(d) => d,
        _ => unreachable!(),
    };

    let mut decrypted = vec![];
    let mut reader = decryptor.decrypt(&Secret::new(passphrase), None)?;
    reader.read_to_end(&mut decrypted)?;

    Ok(decrypted)
}

fn home_dir() -> String {
    dirs::home_dir()
    .unwrap()
    .into_os_string()
    .into_string()
    .unwrap()
}
