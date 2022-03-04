pub mod finder {

    use crate::enums::enums::*;
    use fuzzy_matcher::skim::SkimMatcherV2;
    use fuzzy_matcher::FuzzyMatcher;
    use rayon::prelude::*;
    use std::ffi::OsStr;
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex};
    use std::time::Instant;
    use tokio::{fs, join};

    #[derive(Debug, Clone)]
    pub struct Meta {
        name: String,
        path: String,
    }

    #[derive(Debug, Clone)]
    pub struct Finder<'a> {
        pub directory: Option<&'a str>,
        pub fuzzy_search: Option<&'a str>,
        pub list: Vec<Meta>,
    }

    impl<'a> Finder<'a> {
        pub fn new() -> Finder<'a> {
            Finder {
                directory: None,
                fuzzy_search: None,
                list: vec![],
            }
        }
        pub fn rayon_walk_dir(&mut self, path: &str, filter: [bool; 7]) {
            let start = Instant::now();
            //
            // Flags - hidden directory,
            //
            //
            fn read_dir(
                entries: Arc<Mutex<Vec<(String, String)>>>,
                s: &rayon::Scope<'_>,
                base_path: PathBuf,
                filter: [bool; 7],
            ) {
                //Works Belows
                let bp = base_path.clone();
                let temp = base_path.file_name().unwrap();
                //let path: String = String::from(temp.to_string_lossy());

                for entry in std::fs::read_dir(bp).unwrap_or_else(|e| {
                    panic!("Error reading dir: {:?}, {}", temp, e);
                }) {
                    let entry = entry;

                    match &entry {
                        Ok(ent) => {
                            let entry = entry.unwrap();
                            let path = entry.path();

                            // Flags - hidden directory,
                            if !path.starts_with(".") {
                                let metadata = entry.metadata().unwrap();

                                if metadata.is_dir() {
                                    let move_entries = entries.clone();
                                    s.spawn(move |s1| read_dir(move_entries, s1, path, filter));
                                } else if metadata.is_file() {
                                    let p = path.as_path().display().to_string();

                                    let ft = Finder::get_file_type(&p);
                                    let mut flag_continue = false;
                                    match ft {
                                        FileType::Audio => {
                                            if filter[0] {
                                                flag_continue = true;
                                            }
                                        }
                                        FileType::Document => {
                                            if filter[1] {
                                                flag_continue = true;
                                            }
                                        }
                                        FileType::Image => {
                                            if filter[2] {
                                                flag_continue = true;
                                            }
                                        }
                                        FileType::Other => {
                                            if filter[3] {
                                                flag_continue = true;
                                            }
                                        }
                                        FileType::Video => {
                                            if filter[4] {
                                                flag_continue = true;
                                            }
                                        }
                                        FileType::None => {
                                            if filter[5] {
                                                flag_continue = true;
                                                println!("{}", p);
                                            }
                                        }
                                        FileType::All => {
                                            if filter[6] {
                                                flag_continue = true;
                                            }
                                        }
                                    }

                                    if flag_continue {
                                        let async_results = Finder::async_file_metadata_join(&p);
                                        let x = futures::executor::block_on(async_results);

                                        entries.lock().unwrap().push(x);
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            println!("{:?}", &entry);
                            println!("####################{}", e);
                        }
                    }
                }
            }

            //*************************************************************************************************************************//
            pub fn walk_files(
                base_path: &std::path::Path,
                filter: [bool; 7],
            ) -> std::vec::Vec<(String, String)> {
                let entries = Arc::new(Mutex::new(Vec::new()));

                let base_path = base_path.to_owned();
                let move_entries = entries.clone();
                let ret = rayon::scope(move |s| {
                    s.spawn(move |s1| read_dir(move_entries, s1, base_path, filter))
                });

                let entries = Arc::try_unwrap(entries).unwrap().into_inner().unwrap();
                entries
            }
            //*************************************************************************************************************************//

            let path = std::path::Path::new(path);
            let flag = !path.starts_with(".");
            if flag {
                let mut list = walk_files(path, filter);
                //println!("{:#?}", list);

                for item in list {
                    let metadata = std::fs::metadata(&item.1); //unwrap()
                    match metadata {
                        Ok(md) => {
                            if !md.is_dir() {
                                //Continue if FileType equals "...."
                                /* if flag_continue == true {}
                                 */

                                let matcher = SkimMatcherV2::default();
                                //matcher.simple_match(choice, pattern, first_match_indices, false, false)
                                let res = matcher.fuzzy_match(&item.0, &self.fuzzy_search.unwrap());
                                match res {
                                    Some(_) => {
                                        let x = item.clone();
                                        self.list.push(Meta {
                                            name: x.0,
                                            path: x.1,
                                        })
                                    }
                                    None => {}
                                }
                            }
                        }
                        Err(_) => {
                            println!("Some big error here. but does the program exit???")
                        }
                    }
                }
            }

            let duration = start.elapsed();
            println!("duration::{:#?}", duration);
        }
        pub fn walk_dir(&mut self, path: &str, filter: [bool;7]){
            
        }
        async fn async_file_metadata_join(path: &str) -> (String, String) {
            //// -1
            let fno = get_file_name_os(path);

            //// 0
            let fpo = get_file_path_os(path);

            //// 1
            //let fnc = Finder::get_file_byte_checksum(path, chunk_size);

            //// 2
            //let fc = Finder::get_file_created(path);

            //// 3
            //let fps = Finder::get_file_points_system(path);

            //// 4
            //let fs = Finder::get_file_size(path);

            join!(fno, fpo)
        }
        fn get_file_type(path: &str) -> crate::enums::enums::FileType {
            let ext = std::path::Path::new(&path)
                .extension()
                .and_then(OsStr::to_str);

            match ext {
                None => return FileType::None,
                Some(_) => {
                    let ext = ext.unwrap().to_lowercase();
                    match ext.as_str() {
                        "jpg" | "png" | "heic" | "jpeg" | "tiff" | "tif" | "psd" | "tga"
                        | "thm" | "dds" => return FileType::Image,
                        "avi" | "mov" | "mpg" | "mpeg" | "mp4" => return FileType::Video,
                        "doc" | "docx" | "txt" | "vcs" | "xls" | "pdf" | "ppt" | "zip" => {
                            return FileType::Document
                        }
                        "tta" | "sln" | "mogg" | "oga" | "wma" | "wav" | "vox" | "voc" | "raw"
                        | "ogg" | "mpc" | "mp3" | "m4p" | "m4b" | "m4a" | "gsm" | "flac" | "au"
                        | "ape" | "amr" | "aiff" | "act" | "aax" | "aac" | "aa" | "3gp" => {
                            return FileType::Audio
                        }
                        _ => return FileType::Other,
                    };
                }
            }

            //println!("get_file_type)_ ::{}", &path);
            let ext = ext.unwrap().to_lowercase();
        }
    }
    async fn get_file_name_os(path: &str) -> String {
        let name = std::path::Path::new(&path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        name.to_string()
    }
    async fn get_file_path_os(path: &str) -> String {
        path.to_string()
    }

    //Helpers
    pub fn type_of<T>(_: T) -> &'static str {
        std::any::type_name::<T>()
    }
}
