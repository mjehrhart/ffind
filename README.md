# ffinder
Fast Finder is a cli written in rust using rayon parrellism for super fast results. This is faster than the standard 'find' command. FFinder is built to be simple to use, superior speed, and return extremly reliable results.

## Install ffinder

```
brew tap mjehrhart/ff
install mjehrhart/ff/ff
```

## Update/Upgrade
```
brew reinstall mjehrhart/ff/ff
brew upgrade mjehrhart/ff/ff
``` 

![ff4](https://user-images.githubusercontent.com/97703291/162531806-c9607850-fa7b-4db9-8983-54bea7e0844c.gif)


## Args, Flags, & Parameters 
```
-f, --file_type <file_type>        To filter the search by file type -
                                   All, Audio, Document, Empty, Image, Other, Video [default: 0]
-h, --search-hidden                Traverse hidden directories
    --help                         Print help information
-p, --search-photos                By default Photos Library is ignored
-s, --search_type <search_type>    Search Algorithm Type -
                                   Contains Text, Fuzzy Search, Pattern Match, Simple Match
                                   [default: 0]
-t, --threads <threads>            Number of threads to use in parrellism [default: 35]
-V, --version                      Print version information
```

## Examples

Basic usage. This will search the home directory, minus the Photos Library.photoslibrary. The user's Photos Library is ignored by default as are hidden files.  You can set a flag to include Photos Library(-p) and hidden files(-h).  
```
ff minty
```

To perform a search in the current working directory add '.' at the end of the command.
```
ff minty .
```

Also, you can search by file type. This is especially useful when working with large data sets or images.  

Search by media type.   
    All => 0  
    Audio => 1  
    Document => 2  
    Empty(no extension) => 3  
    Image => 4  
    Other => 5  
    Video => 6  

Below we are searching for the word 'beethoven' (case insensitive) in all audio file names'.
```
ff beethoven -f 1
```
 
The user can set the number of threads to use. Fast Finder uses Rayon Parrellism for very fast lookups.
```
ff minty -t 1000
```

<img width="100%" alt="Screen Shot Fast Find" src="https://user-images.githubusercontent.com/97703291/162533291-371513ae-fb8c-46cc-9965-b2392d20f203.png">

<img width="100%" alt="Screen Shot Fast Find" src="https://user-images.githubusercontent.com/97703291/162533618-377d1549-474a-4ec1-b08d-4a5f24b332ad.png">

<img width="100%" alt="Screen Shot Fast Find" src="https://user-images.githubusercontent.com/97703291/162533499-b87a387e-1663-4d90-8556-2e0a2084fa2f.png">
 
