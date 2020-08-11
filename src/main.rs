use anyhow::{Error, Result};
use std::io;
use io::{Read, Write, BufReader};
use structopt::clap::AppSettings;
use structopt::StructOpt;
use std::fs::{File, self};
use secrecy::Secret;

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands)]
enum Opt {
    #[structopt(alias = "insert")]
    Add,
    #[structopt()]
    Show,
    #[structopt()]
    Config,
    #[structopt()]
    List
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Add => add(),
        Opt::Show => show(),
        Opt::Config => config(),
        Opt::List => list(),
    };
}

fn add() -> Result<(), Error> {
    print!("Enter the shortname for your password entry: ");
    io::stdout().flush()?;

    let mut shortname = String::new();
    io::stdin().read_line(&mut shortname)?;

    let password = rpassword::prompt_password_stdout("Enter a password: ")?;
    let passphrase = rpassword::prompt_password_stdout("Enter a passphrase: ")?;

    let encrypted_pwd = encrypt(password, passphrase);

    let file = File::create(format!("{}/{}", full_dir(), shortname.trim()));
    
    match file {
        Ok(mut file) => file.write_all(&encrypted_pwd?).unwrap(),
        Err(e) => println!(r"Error while creating password file. Make sure to run 'passwrd config' first. 
{}", e)
    }
    
    Ok(())
}

fn show() -> Result<(), Error> {
    print!("Enter the shortname for the password you want to show: ");
    io::stdout().flush()?;

    let mut shortname = String::new();
    io::stdin().read_line(&mut shortname)?;

    let file = File::open(format!("{}/{}",  full_dir(), shortname.trim()))?;
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

fn config() -> Result<(), Error> {
    match fs::create_dir(full_dir()) {
        Ok(()) => {
            println!("Created new .psswd folder in {}", home_dir());
        },
        Err(e) => {
            println!("{}, {}", e, home_dir());
        },
    };

    Ok(())
}

fn list() -> Result<(), Error> {
    let paths = fs::read_dir(full_dir()).unwrap();
    let mut files = Vec::new();
    for path in paths {
        files.push(path.unwrap().file_name().into_string().unwrap());
    }

    for file in files {
        println!("{}", file);
    }

    Ok(())
}

fn home_dir() -> String {
    dirs::home_dir()
    .unwrap()
    .into_os_string()
    .into_string()
    .unwrap()
}

fn full_dir() -> String {
    format!("{}/{}", home_dir(), ".psswd")
}
