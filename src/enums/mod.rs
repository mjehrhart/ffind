#[allow(unused)]
pub mod enums {

    #[derive(Debug, PartialEq, Copy, Clone, Eq)]
    pub enum SearchType{
        Contains,
        Fuzzy,
        None,
        Pattern,
        Simple, 
    }

    #[derive(Debug, PartialEq, Copy, Clone, Eq)]
    pub enum FileAction {
        Delete,
        Save,
        None,
        Read, //got checksum
    }

    #[derive(Debug, PartialEq, Copy, Clone, Eq)]
    pub enum FileType {
        All,
        Audio,
        Document,
        Empty,
        Image,
        Other,
        Video,
    }

    #[derive(Debug, PartialEq, Clone, Copy, Eq)]
    pub enum MetaData<'a> {
        FileSize(i32),
        Created(&'a str),
    }
}
