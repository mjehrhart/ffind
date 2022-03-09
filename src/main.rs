mod enums;
mod finder;
use clap::{Arg, Command};
use colored::*;
use home::home_dir;

use crate::finder::finder::Found;

#[tokio::main]
async fn main() {

    let mut ff = finder::finder::Finder::new();
    ff.search_type = enums::enums::SearchType::Contains;
    ff.flag_skip_hidden = true;
    ff.thread_count = 35;

    let default_path = home::home_dir().unwrap().as_path().display().to_string();

    let mut directory: &str = "";
    let mut pattern: &str = "";
    let mut filter = [false; 7];

    //flag --hidden ::checked::required
    //flag --skip_photos ::checked
    //option --file_type ::checked::required; FileType
    //option --match pattern: checked; SearchType
    //option --threads ::checked
    let matches = Command::new("FFinder")
        .version("1.0")
        .author("Matthew E. <mjehrhart@gmail.com>")
        .about("Search os for matched files")
        .arg(
            //Arg
            Arg::new("pattern")
                .help("Pattern to search filename to match")
                .required(true)
                .index(1),
        )
        .arg(
            //Option
            Arg::new("directory")
                .help("The search starts in this directory")
                .required(false)
                .default_value(&default_path)
                .index(2),
        )
        .arg(
            //Option
            Arg::new("file_type")
                .long("file_type")
                .short('f')
                .help("To filter the search by file type -\nAll, Audio, Document, Empty, Image, Other, Video")
                .takes_value(true)
                .default_value("0")
                .required(false) 
        )
        .arg(
            //Option
            Arg::new("search_type")
                .long("search_type")
                .short('s')
                .help("Search Algorithm Type -\nContains Text, Fuzzy Search, Pattern Match, Simple Match")
                .takes_value(true)
                .default_value("0")
                .required(false) 
        )
        .arg(
            //Option
            Arg::new("threads")
                .long("threads")
                .short('t')
                .help("Number of threads to use in parrellism")
                .takes_value(true)
                .default_value("35")
                .required(false) 
        )
        .arg(
            //Flag
            Arg::new("photos")
                .long("search-photos")
                .short('p')
                .required(false)
                .takes_value(false)
                .help("By default Photos Library is ignored"),
        )
        .arg(
            //Flag
            Arg::new("search hidden")
                .long("search-hidden")
                .short('h')
                .required(false)
                .takes_value(false) 
                .help("Traverse hidden directories"),
        )
        //.arg(arg!(--pattern <VALUE>).default_value("./"))
        .get_matches();

    //Arg::Pattern
    if matches.is_present("pattern") {
        if let Some(val) = matches.value_of("pattern") {
            pattern = val;
        }
    }

    //Option::Directory
    if matches.is_present("directory") {
        if let Some(val) = matches.value_of("directory") {
            directory = val;
        }
    }

    //Option::Threads
    if matches.is_present("threads") {
        if let Some(val) = matches.value_of("threads") {
            ff.thread_count = val.parse().unwrap();
        }
    }

    //Option::Search Type
    if matches.is_present("search_type"){
        if let Some(val) = matches.value_of("search_type") {
    
            for c in val.chars(){
                match Some(c) {
                    Some('0') => {
                        ff.search_type = enums::enums::SearchType::Contains;
                    },
                    Some('1') => {
                        ff.search_type = enums::enums::SearchType::Fuzzy;
                    },
                    Some('2') => {
                        ff.search_type = enums::enums::SearchType::Pattern;
                    },
                    Some('3') => {
                        ff.search_type = enums::enums::SearchType::Simple;
                    },
                    None => todo!(),
                    Some(_) => {} 
                } 
            } 
        }
    } else {
        filter[0] = true; //traverse all extensions
    }

    //Option::File Type
    if matches.is_present("file_type"){
        if let Some(val) = matches.value_of("file_type") {
    
            for c in val.chars(){
                match Some(c) {
                    Some('0'..='9') => {
                        let index:usize = c.to_string().parse().unwrap();
                        filter[index] = true;
                    },
                    None => todo!(),
                    Some(_) => {} 
                } 
            } 
        }
    } else {
        filter[0] = true; //traverse all extensions
    }

    //Flag::Hidden Folders
    if matches.is_present("search hidden") {
        //h
        ff.flag_skip_hidden = false;
    } else {
        ff.flag_skip_hidden = true;
    }

    //Flag::Photos Library
    if matches.is_present("photos") {
        //p
        ff.flag_skip_photos = false;
    } else {
        ff.flag_skip_hidden = true;
    }

    //******************************************************************************************************************/
    //Function One
    let start = std::time::Instant::now();

    ff.directory = Some(&*directory);
    ff.search_pattern = Some(&*pattern); 
    ff.fast_walk_dir(&ff.directory.unwrap(), filter); //1.159

    let x = ff.list;
    for item in x.iter() {
        let len1 = item.name.len();
        let len2 = item.path.len();
        println!(
            "{}{}",
            &item.path[..len2 - len1].black().bold(),
            item.name.blue().bold()
        );
        //println!("  {}", item.name.blue());
    }

    //println!("values::{:#?}", x.len());
    println!("Found::{:#?}", x.len());
    println!("Duration::{:#?}", start.elapsed());
    println!("Directory::{}", directory.to_string().green());

    /* //Function Two:: Streaming

    let start = std::time::Instant::now();
    let mut ff = finder::finder::Finder::new();
    ff.flag_skip_hidden = true;
    ff.thread_count = 50;
    ff.directory = Some(&*directory);
    ff.fuzzy_search = Some(&*pattern);

    let root_path = &ff.directory.unwrap();
    let paths = ff.stream_paths(root_path);
    //let paths = finder::finder::Finder::stream_paths(root_path);

    futures::StreamExt::for_each(paths, |entry| async {
        match entry {
            Ok(entry) => {
                println!("{:?}", entry.path().as_path())
            }
            Err(e) => eprintln!("encountered an error: {}", e),
        }
    })
    .await;
    println!("duration::{:#?}", start.elapsed());
    */

    //keep
    //let a = futures::executor::block_on( ff.walk(&ff.directory.unwrap(), filter));
}
