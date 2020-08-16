use anyhow::{Error, Result};
use io::{BufReader, Read, Write};
use secrecy::Secret;
use std::fs::{self, File};
use std::io;
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands, about="Password manager using age-encryption")]
enum Opt {
    /// Creates a new entry containing an encrypted password.
    #[structopt(alias = "create")]
    Add { entry: Option<String> },
    /// Displays a specific entry.
    #[structopt(alias = "display")]
    Show { entry: Option<String> },
    /// Lists entries.
    #[structopt()]
    List,
    /// Deletes one or all entries.
    #[structopt()]
    Delete {
        #[structopt(short, long)]
        all: bool,
        #[structopt(short, long)]
        entry: bool,
    },
}

fn main() {
    // creates the .psswd folder in user's OS Home directory
    match fs::create_dir(full_dir()) {
        Ok(()) => {}
        Err(e) => {
            if !e.to_string().contains("File exists") {
                println!("{}, {}", e, home_dir());
            }
        }
    };

    match Opt::from_args() {
        Opt::Add { entry } => add(entry),
        Opt::Show { entry } => show(entry),
        Opt::List => list(),
        Opt::Delete { all, entry } => delete(all, entry),
    }
    .unwrap();
}

fn add(entry: Option<String>) -> Result<(), Error> {
    let mut shortname = String::new();

    if let Some(name) = entry {
        shortname = name;
    } else {
        print!("Enter the shortname for your password entry: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut shortname)?;
    }

    let password = rpassword::prompt_password_stdout("Enter a password: ")?;
    let passphrase = rpassword::prompt_password_stdout("Enter a passphrase: ")?;

    let encrypted_pwd = encrypt(password, passphrase);

    let file = File::create(format!("{}/{}", full_dir(), shortname.trim()));

    match file {
        Ok(mut file) => file.write_all(&encrypted_pwd?).unwrap(),
        Err(e) => println!(
            r"Error while creating password file. Make sure to run 'passwrd config' first. 
{}",
            e
        ),
    }

    Ok(())
}

fn show(entry: Option<String>) -> Result<(), Error> {
    match entry {
        Some(name) => {
            match File::open(format!("{}/{}", full_dir(), name.trim())) {
                Ok(_) => println!("The entry {} was deleted.", name),
                Err(e) => println!("Error when trying to delete entry {}. {}", name, e),
            };
            Ok(())
        }
        None => {
            print!("Enter the shortname for the password you want to show: ");
            io::stdout().flush()?;

            let mut shortname = String::new();
            io::stdin().read_line(&mut shortname)?;

            let file = File::open(format!("{}/{}", full_dir(), shortname.trim()))?;
            let mut buffer = BufReader::new(file);

            let mut encrypted = vec![];
            buffer.read_to_end(&mut encrypted)?;

            let passphrase = rpassword::prompt_password_stdout("Enter your passphrase: ")?;
            let decrypted_pwd = decrypt(&encrypted, passphrase)?;
            let decrypted_pwd = String::from_utf8(decrypted_pwd)?;

            println!("{}", decrypted_pwd);
            Ok(())
        }
    }
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

fn delete(all: bool, entry: bool) -> Result<(), Error> {
    if entry {
        print!("Enter the name of the entry you want to delete: ");
        io::stdout().flush()?;

        let mut entry_name = String::new();
        io::stdin().read_line(&mut entry_name)?;
        match fs::remove_file(format!("{}/{}", full_dir(), entry_name.trim())) {
            Ok(()) => println!("Entry {} was deleted", entry_name.trim()),
            Err(e) => println!("{}", e),
        };
    }

    if all {
        match fs::remove_dir_all(full_dir()) {
            Ok(()) => {
                println!("Your entries have been deleted");
            }
            Err(e) => println!("{}", e),
        };
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
