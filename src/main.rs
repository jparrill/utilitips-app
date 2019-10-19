extern crate config;
extern crate serde;

extern crate serde_derive;
extern crate toml;

mod files;
mod settings;

use settings::Settings;


fn main() {
    //let mut buffer = String::new();
    println!("Reading config file...");
    let settings = Settings::new().unwrap();
    println!("You have all of this topics at {}/{}:\n",
        settings.global.repository_path, 
        settings.global.content_root_path);
    println!("Which topic you want to Add/Modify: ");

    //match io::stdin().read_line(&mut buffer) {
    //    Ok(_) => files::find_and_read(&buffer),
    //    Err(e) => println!("{:?}", e)
    //}
}
