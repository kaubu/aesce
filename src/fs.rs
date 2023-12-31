use std::path::PathBuf;
use std::collections::HashMap;

use chrono::prelude::*;
use uuid::Uuid;

#[derive(Clone)]
pub struct File {
    // 67e55044-10b1-426f-9247-bb680e5fe0c8
    uuid: Uuid,
    // // /home/tom/
    // location: String,
    // emails.txt
    name: String,
    // // /home/tom/emails.txt
    // absolute_path: String,
    // ./files/67e55044-10b1-426f-9247-bb680e5fe0c8.af
    real_path: PathBuf,
    // 2014-11-28T12:00:09Z
    created_date: DateTime<chrono::Utc>,
    // 2014-12-03T23:45:22Z
    modified_date: DateTime<chrono::Utc>,
}

impl File {
    pub fn new(location: String, name: String) -> File {
        let uuid = Uuid::new_v4();
        // let absolute_path = format!("{}{}", location, name);
        let mut real_path = PathBuf::from(crate::FILES_DIRECTORY);
        real_path.push(format!("{}.af", uuid.to_string()));
        let created_date = Utc::now();
        let modified_date = created_date.clone();

        File {
            uuid,
            // location,
            name,
            // absolute_path,
            real_path,
            created_date,
            modified_date,
        }
    }
}

type DirectoryListing<'a> = HashMap<String, Directory<'a>>;
type FileListing = HashMap<String, File>;

enum ParentDir<'a> {
    Root,
    Sub(DirectoryListing<'a>),
}

pub struct Directory<'a> {
    name: String,
    parent: &'a ParentDir<'a>,
    // children: &'a HashMap<String, DirectoryEntry>,
    directories: DirectoryListing<'a>,
    files: FileListing,
}

impl<'a> Directory<'a> {
    pub fn create_root_dir() -> Directory<'a> {
        Directory {
            name: "/".to_string(),
            parent: &ParentDir::Root,
            directories: HashMap::new(),
            files: HashMap::new(),
        }
    }

    pub fn get_path(dir: &Directory, reverse_dirs: Vec<String>) -> String {
        loop {
            match dir.parent {
                ParentDir::Sub(d) => {
                    let parent_dir = d.get(&dir.name)
                        .expect("[error] could not open parent dir");
                    
                    let mut reverse_dirs = reverse_dirs.clone();
                    reverse_dirs.push(dir.name.clone());

                    Directory::get_path(parent_dir, reverse_dirs);
                },
                ParentDir::Root => {
                    return reverse_dirs.into_iter().rev().collect();
                },
            }
        }
    }

    pub fn add_file(&mut self, file: File) {
        if self.files.contains_key(&file.name) {
            println!("[error] file already exists!");
            return;
        }
        
        self.files.insert(file.name.to_string(), file);
    }

    // fn add_child(&mut self, dir_entry: DirectoryEntry) {
    //     // If adding a file
    //     if let DirectoryEntry::File(f_one) = &dir_entry {
    //         (&self.children).into_iter().any(|c| {
    //             if let DirectoryEntry::File(f_two) = *(*c).clone() {
    //                 return f_one.name == f_two.name
    //             }

    //             false
    //         });
    //     }

    //     self.children.push(Box::new(dir_entry));
    // }

    // fn remove_child(&mut self,);

    // fn get_path
    // recursively goes up `parent`s until it finds the root directory
}