pub mod finder {

    use crate::enums::enums::*;
    use async_recursion::async_recursion;
    use futures::TryFutureExt;
    use fuzzy_matcher::skim::SkimMatcherV2;
    use fuzzy_matcher::FuzzyMatcher;
    use home::home_dir;
    use jwalk::WalkDir;
    use rayon::prelude::*;
    use std::ffi::OsStr;
    use std::fs::metadata;
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex};
    use std::time::Instant;
    use tokio::{fs, join};

    use futures::{stream, Stream, StreamExt}; // 0.3.1
    use std::io;
    use tokio::fs::DirEntry;

    #[derive(Debug, Clone)]
    pub struct Found {
        pub name: String,
        pub path: String,
    }

    #[derive(Debug, Clone)]
    pub struct Finder<'a> {
        pub directory: Option<&'a str>,
        pub search_pattern: Option<&'a str>,
        pub search_type: SearchType,
        pub flag_skip_hidden: bool,
        pub flag_skip_photos: bool,
        pub list: Vec<Found>,
        pub thread_count: i32,
    }

    #[allow(unused)]
    impl<'a> Finder<'a> {
        pub fn new() -> Finder<'a> {
            Finder {
                directory: None,
                flag_skip_hidden: true,
                flag_skip_photos: true,
                list: vec![],
                search_pattern: None,
                search_type: SearchType::None,
                thread_count: 25,
            }
        }

        pub fn fast_walk_dir(&mut self, path: &str, filter: [bool; 7]) {
            //
            //flag --hidden ::checked::required
            //flag --skip_photos ::checked
            //option --file_type ::checked::required; FileType
            //option --match pattern: checked; SearchType
            //option --threads ::checked
            //

            //self.flag_skip_hidden
            for dir_entry_result in jwalk::WalkDirGeneric::<((), Option<u64>)>::new(&path)
                .skip_hidden(self.flag_skip_hidden)
                .parallelism(jwalk::Parallelism::RayonNewPool(
                    self.thread_count.try_into().unwrap(),
                ))
                .process_read_dir(|_, dir_entry_results| {})
            {
                //let regex = regex::Regex::new(self.search_pattern.unwrap()).unwrap(); 
                //println!("self.flag_skip_hidden...{}", self.flag_skip_hidden);
                match dir_entry_result {
                    Ok(dir_entry) => {

                        let path: String = dir_entry.path().as_path().display().to_string(); 
                        let mut flag_continue = true;
                      
                        //1 -- flag skip_photos
                        if self.flag_skip_photos {
                            let photos = String::from("/Pictures/Photos Library.photoslibrary");
                            let user_home =
                                home::home_dir().unwrap().as_path().display().to_string();

                            let joined = [user_home, photos].join("");
                            if path == joined {
                                println!("Skipping Photos Library...{}", path);
                                flag_continue = false;
                                //enum
                            }
                        }

                        if flag_continue {
                            if !dir_entry.file_type.is_dir() {
                                let path: String = dir_entry.path().as_path().display().to_string();

                                //2 -- option flag_type
                                let x = self.filter_file_type(&path, filter);
                                match x {
                                    Some(_) => {
                                        //3 -- option pattern match
                                        let x = self.filter_pattern_match(&path, filter);
                                        match x {
                                            Some(_) => {
                                                //println!("Option<String>::{:#?}", x);
                                                let name = dir_entry
                                                    .path()
                                                    .file_name()
                                                    .unwrap()
                                                    .to_str()
                                                    .unwrap()
                                                    .to_ascii_lowercase();

                                                self.list.push(Found {
                                                    name: name,
                                                    path: path,
                                                })
                                            }
                                            None => {}
                                        }
                                    }
                                    None => {}
                                }
                            }
                        }
                    }
                    Err(error) => {
                        println!("Read dir_entry error: {}", error);
                    }
                }
            }
        }

        pub fn stream_paths(
            &mut self,
            path: impl Into<PathBuf>,
        ) -> impl Stream<Item = io::Result<DirEntry>> + Send + 'static {
            //
            async fn one_level(
                path: PathBuf,
                to_visit: &mut Vec<PathBuf>,
            ) -> io::Result<Vec<DirEntry>> {
                let mut dir = fs::read_dir(path).await?;
                let mut files = Vec::new();

                let regex = regex::Regex::new("minty").unwrap();

                while let Some(child) = dir.next_entry().await? {
                    let path = child.path().to_str().unwrap().to_owned();
                    let name = child
                        .path()
                        .as_path()
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_owned();

                    //flag --hidden
                    //
                    //

                    if child.metadata().await?.is_dir() {
                        if name.starts_with('.') {
                            // do nothing
                        } else {
                            to_visit.push(child.path());
                        }
                    } else {
                        let found = regex.captures(&name);
                        match found {
                            Some(_) => files.push(child),
                            None => {}
                        }
                    }
                }

                Ok(files)
            }

            stream::unfold(vec![path.into()], |mut to_visit| async {
                let path = to_visit.pop()?;
                let file_stream = match one_level(path, &mut to_visit).await {
                    Ok(files) => stream::iter(files).map(Ok).left_stream(),
                    Err(e) => stream::once(async { Err(e) }).right_stream(),
                };

                Some((file_stream, to_visit))
            })
            .flatten()
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
                None => {
                    //println!("ext:: {:?}", &path);
                    return crate::enums::enums::FileType::Empty;
                }
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
        }

        pub fn filter_pattern_match(&mut self, path: &str, filter: [bool; 7]) -> Option<String> {
            let name = std::path::Path::new(&path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            match self.search_pattern {
                Some(pattern) => match self.search_type {
                    SearchType::Contains => {
                        if name.contains(self.search_pattern.unwrap()) {
                            Some(path.to_string())
                        } else {
                            None
                        }
                    }
                    SearchType::Fuzzy => {
                        let matcher = SkimMatcherV2::default();
                        let res = matcher.fuzzy_match(&path, &self.search_pattern.unwrap());
                        match res {
                            Some(_) => Some(path.to_string()),
                            None => None,
                        }
                    }
                    SearchType::Pattern => {
                        let regex = regex::Regex::new(pattern).unwrap();
                        let found = regex.captures(&name);
                        match found {
                            Some(_) => Some(path.to_string()),
                            None => None,
                        }
                    }
                    SearchType::Simple => {
                        if name == self.search_pattern.unwrap() {
                            Some(path.to_string())
                        } else {
                            None
                        }
                    }
                    SearchType::None => Some(path.to_string()),
                },
                None => None,
            }
        }

        pub fn filter_file_type(&mut self, path: &str, filter: [bool; 7]) -> Option<String> {
            //Return if filter[6] (ALL) is true
            if filter[0] {
                return Some(path.to_string());
            }

            let pattern = Finder::get_file_type(&path);
            match pattern {
                FileType::All => {
                    if filter[0] {
                        return Some(path.to_string());
                    }
                }
                FileType::Audio => {
                    if filter[1] {
                        return Some(path.to_string());
                    }
                }
                FileType::Document => {
                    if filter[2] {
                        return Some(path.to_string());
                    }
                }
                FileType::Empty => {
                    if filter[3] {
                        return Some(path.to_string());
                    }
                }
                FileType::Image => {
                    if filter[4] {
                        return Some(path.to_string());
                    }
                }
                FileType::Other => {
                    if filter[5] {
                        return Some(path.to_string());
                    }
                }
                FileType::Video => {
                    if filter[6] {
                        return Some(path.to_string());
                    }
                }
            } 
            None
        }
    }

    //Helpers
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

//#[async_recursion]
// -> Result<String, Box<dyn Error>>
// -> Result<(), std::io::Error>
// -> Box<dyn futures::Future<Output = ()>>
// -> Result<tokio::fs::ReadDir, Box<dyn std::error::Error>>
// -> Result<String, Box<dyn std::error::Error>>
