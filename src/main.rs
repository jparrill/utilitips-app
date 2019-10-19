#[macro_use]
extern crate clap;

extern crate config;
extern crate serde;
extern crate serde_derive;
extern crate toml;

mod files;
mod settings;

use settings::Settings;
use clap::{Arg, App};

fn main() {
    // Initialized CLI
    let matches = App::new("utilitips")
        .version("0.1")
        .author("Juan Manuel P. <jparrill@redhat.com>")
        .about("Store your tips from cli in a easy way")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true))
        .arg(Arg::with_name("book")
            .short("b")
            .long("book")
            .value_name("BOOK")
            .help("Book to be created to store some topics (FOLDER NAME)")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("topic")
            .short("t")
            .long("topic")
            .value_name("TOPIC")
            .help("Topic to be created (MARKDOWN FILE)")
            .takes_value(true)
            .required(true))
        .get_matches();

    // Assign Args
    let config = matches.value_of("config").unwrap();
    let topic = matches.value_of("topic").unwrap();
    let book = matches.value_of("book").unwrap();
    
    //let mut buffer = String::new();
    println!("Reading config file...");
    let settings = Settings::new(config).unwrap();
    println!("You have all of this topics at {}/{}:\n",
        settings.global.repository_path, 
        settings.global.content_root_path);
    println!("Which topic you want to Add/Modify: ");

    //match io::stdin().read_line(&mut buffer) {
    //    Ok(_) => files::find_and_read(&buffer),
    //    Err(e) => println!("{:?}", e)
    //}
}
