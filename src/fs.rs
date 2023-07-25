use std::path::PathBuf;

use chrono::prelude::*;
use uuid::Uuid;

#[derive(Debug)]
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
        let absolute_path = format!("{}{}", location, name);
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

enum DirectoryEntry {
    Directory(Directory),
    File(File),
}

struct Directory {
    name: String,
    parent: Box<DirectoryEntry>,
    children: Vec<Box<DirectoryEntry>>,
}

impl Directory {
    fn add_child(&mut self, dir_entry: DirectoryEntry) {
        // If adding a file
        if let DirectoryEntry::File(f) = &dir_entry {
            if self.children.iter().find(|x| {
                let mut exists = false;
                
                if let DirectoryEntry::File(g) = x.clone() {
                    exists = g.name == f.name;
                }

                exists
            }).is_some() {
                println!("[error] file '{}' already exists!", f.name);
                return;
            }
        }

        self.children.push(Box::new(dir_entry));
    }

    // fn remove_child(&mut self,);

    // fn get_path
    // recursively goes up `parent`s until it finds the root directory
}