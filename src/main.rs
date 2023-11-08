use std::{fs::{File, create_dir_all, remove_file, copy, remove_dir, create_dir}, path::{PathBuf, Path}};
use clap::Parser;
use walkdir::WalkDir;
use std::io::prelude::*;
use sha2::{Sha256, Digest};

#[derive(Parser)]
#[command(name = "Rapid sync")]
#[command(author = "Dev alpha <Kalantar98@Gmail.com>")]
#[command(version = "0.0.1")]
#[command(about = "Rapid sync, easily sync your files across local filesystem", long_about = None)]
struct Cli {
    #[arg(short,long)]
    source_base: String,
    #[arg(short,long)]
    destination_base: String,
    #[arg(short,long)]
    items: Vec<String>,
    #[arg(short,long, default_value_t = 0)]
    verbosity: u8,
    #[arg(short,long, default_value_t = true)]
    abort_on_error: bool,
}


fn hash_file<P: AsRef<Path>>(path: P) -> Option<String> {
    let mut file = File::open(path).ok()?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).ok()?;
    let mut hasher = Sha256::new();
    hasher.update(&buffer);
    Some(format!("{:x}", hasher.finalize()))
}

fn main() {
    let Cli {
        source_base,
        destination_base,
        items,
        verbosity, // TODO: Implement this
        abort_on_error, // TODO: Implement this
    } = Cli::parse();
    
    let source_base = source_base.trim().trim_end_matches('/').to_owned();
    let destination_base = destination_base.trim().trim_end_matches('/').to_owned();

    if !PathBuf::from(&source_base).is_dir() {
        println!("Provided source base does not exists or is not a directory: {}", source_base);
        return;
    }

    let destination_base_path_buf = PathBuf::from(&destination_base);
    if !destination_base_path_buf.exists() {
        println!("Creating destination base path");
        create_dir_all(&destination_base).unwrap();
    } else if destination_base_path_buf.is_file() {
        println!("There is a file (not a directory) which matches the destination base path");
        return;
    }

    for entry in WalkDir::new(&source_base) {
        match entry {
            Ok(entry) => {
                if entry.depth() == 0 {
                    continue;
                }

                for item in &items {
                    let relative_to_root_entry_path = entry.path()
                        .strip_prefix(&source_base).unwrap()
                        .to_str().unwrap()
                        .trim_start_matches('/');
                    
                    if relative_to_root_entry_path.starts_with(item) {
                        // Here finally we have only the items which specified in the command
                        let destination_path_buf = PathBuf::from(format!("{}/{}", destination_base, relative_to_root_entry_path));
                        if destination_path_buf.exists() {
                            if entry.path().is_file() {
                                if destination_path_buf.is_file() {
                                    if hash_file(entry.path()) != hash_file(&destination_path_buf) {
                                        remove_file(&destination_path_buf).unwrap();
                                        println!("Copying from {} to {}", entry.path().display(), destination_path_buf.display());
                                        copy(entry.path(), &destination_path_buf).unwrap();
                                    }
                                } else {
                                    remove_dir(&destination_path_buf).unwrap();
                                    println!("Copying from {} to {}", entry.path().display(), destination_path_buf.display());
                                    copy(entry.path(), &destination_path_buf).unwrap();
                                }
                            } else if destination_path_buf.is_file() {
                                remove_file(&destination_base_path_buf).unwrap();
                                create_dir(&destination_path_buf).unwrap();
                            }
                            
                        } else {
                            if entry.path().is_dir() {
                                create_dir_all(&destination_path_buf).unwrap();
                            } else {
                                println!("Copying from {} to {}", entry.path().display(), destination_path_buf.display());
                                copy(entry.path(), destination_path_buf).unwrap();
                            }
                        }
                    }
                }
            },
            Err(msg) => println!("{}", msg),
        }
    }
}
