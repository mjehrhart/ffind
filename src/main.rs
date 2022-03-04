 
mod finder;
mod enums;
use home::home_dir;
use clap::{Arg, Command};

fn main() {
    let mut ff = finder::finder::Finder::new();

    let default_path= home::home_dir().unwrap().as_path().display().to_string();
    //println!("default_path::{:?}", default_path );

    /* let matches = Command::new("FFinder")
        .version("1.0")
        .author("Matthew E. <mjehrhart@gmail.com>")
        .about("Search os for matched files") 
        .arg(
            Arg::new("pattern")
            .help("Pattern to search filename for fuzzy match.")
            .required(true) 
            .index(1)
        )
        .arg(
            Arg::new("directory")
            .help("The search starts in this directory")
            .required(false)
            .default_value( &default_path )
            .index(2)
        )
        //.arg(arg!(--pattern <VALUE>).default_value("./"))
        .get_matches();

        if matches.is_present("pattern") {
            if let Some(val) = matches.value_of("pattern") {
                ff.fuzzy_search = Some(val);
            }
        }

        if matches.is_present("directory") {
            if let Some(val) = matches.value_of("directory") {
                 ff.directory = Some(val);
                println!("ff.directory::{:?}", ff.directory );
            }
        } */
 
    //println!("directory: {:?}", matches.value_of("directory").expect("required"));
    //println!("pattern: {:?}", matches.value_of("pattern").expect("required"));

    let sd = "/Users/matthew/zz/";
    println!("default_path::{:?}", sd );

    ff.directory = Some(sd);
    ff.fuzzy_search = Some("minty");

    let mut filter = [false;7];
    filter[6] = true; //show files with no extension
    ff.rayon_walk_dir(&ff.directory.unwrap(), filter);

    let x = ff.list;
    println!("{:#?}", x); 
}
 