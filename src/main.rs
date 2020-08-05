use anyhow::{Error, Result};
use io::Write;
use scrypt::ScryptParams;
use std::io;
use structopt::StructOpt;
use std::fs::File;

#[derive(StructOpt, Debug)]
#[structopt(name = "psswd", about="small password manager")]
enum Opt {
    /// Verbose mode (-v, -vv, -vvv, etc.)
    // #[structopt(short, long, parse(from_occurrences))]
    // verbose: u8,

    // #[structopt(short, long)]
    // add: Add,
    Add,
    Show,
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Add => add_entry(),
        Opt::Show => show(),
    };

    // if opt.new {
    //     println!("test");
    // }
    // println!("{:#?}", opt);
}

fn add_entry() -> Result<(), Error> {
    println!("Enter your credential: ");
    io::stdout().flush()?;

    let mut entry = String::new();
    io::stdin().read_line(&mut entry)?;

    let password = rpassword::prompt_password_stdout("Enter a password: ")?;

    let params = ScryptParams::recommended();
    // let encrypted = scrypt::scrypt_simple(&password, &params).unwrap();
    let encrypted = scrypt::scrypt_simple(&password, &params).unwrap();
    
    let home_dir = dirs::home_dir().unwrap().into_os_string().into_string().unwrap();

    let mut file = File::create(format!("{}/test.txt", home_dir))?;
    file.write(&encrypted.as_bytes());

    
    Ok(())
}

fn show() -> Result<(), Error> {
    // unimplemented!()
    let verif = scrypt::scrypt_check("test", &encrypted).is_ok();
    Ok(())    
}
