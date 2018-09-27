#[macro_use]
extern crate clap;
extern crate reqwest;
extern crate dirs;
use clap::App;
use std::process::Command;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::fs::{self, DirBuilder};
fn main() {
    println!("Hello, world!");
	
	//println!("body = {:?}", get_page());
	
	
	setup_frc_dir();
	get_save("https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png");
	let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
	
    if let Some(matches) = matches.subcommand_matches("hello_there") {
        hello_there();
    }
	if let Some(matches) = matches.subcommand_matches("install") {
        
		if let Some(software) = matches.value_of("installs") {
			install(software);
		} else{
			println!("Eroor?");
		}
		
    }
	
}
fn create_file() -> std::io::Result<()>{
	let mut file = File::create("asdf.txt")?;
	file.write_all(b"HEYYY")?;
	Ok(())
}
fn get_page(url: &str) -> Result<String, reqwest::Error>{
	Ok(reqwest::get(url)?.text()?)
}

fn get_save(url: &str)-> Result<(), reqwest::Error>{
	let mut file = File::create("web.png").unwrap();
	let mut resp = reqwest::get(url)?;
	resp.copy_to(&mut file);
	Ok(())
}
fn setup_frc_dir(){
	let mut path = dirs::home_dir().unwrap();
	path.push("Downloads\\frc6644");
	//println!("{}", path.to_str().unwrap());
	DirBuilder::new()
		.recursive(true)
		.create(&path);
	path.push("installer");
	DirBuilder::new()
		.recursive(true)
		.create(&path);
}
enum Error{
	Page(reqwest::Error),
	Create(std::io::Error),
}

fn hello_there(){
	println!("General Kenobi");
}

fn install(software: &str){
	let mut softwares = HashMap::new();
	softwares.insert("Github", "./GitHubDesktopSetup.exe");
	softwares.insert("VSCode", "./VSCodeUserSetup-x64-1.27.2.exe");
	softwares.insert("JDK", "./jdkPortable_8_Update_181_online.paf.exe");
	softwares.insert("Git", "./Git-2.19.0-64-bit.exe");
	
	

   if let Some(o) = (softwares.get(software)){
	
		let output = Command::new(o).output().unwrap();
		println!("Installing: {}", software);
		println!("status: {}", output.status);
		println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
		println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
		assert!(output.status.success());
	
	}
	else{
		
		println!("sagfdf");
	
	}
}