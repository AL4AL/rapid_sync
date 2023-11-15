Rapid sync
A simple alternative to tools like rdiff-backup or rsync implemented using rust

Usage: `rapid_sync [OPTIONS] --source-base <SOURCE_BASE> --destination-base <DESTINATION_BASE>`

    Options:
        -s, --source-base <SOURCE_BASE>            
        -d, --destination-base <DESTINATION_BASE>  
        -i, --items <ITEMS>                        
        -v, --verbosity <VERBOSITY>                [default: 0]
        -a, --abort-on-error                       
        -h, --help                                 Print help
        -V, --version                              Print version

Example:
    
    ./rapid_sync -s /path/to/source/directory -d /path/to/destination/directory -i folder_1 -i folder_2 -i myfile.txt

Known problems:
- verbosity flag doesn't work
- abort-on-error flag doesn't work
