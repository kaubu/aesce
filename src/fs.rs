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
        // Check if the child being added is a directory or file, so that
        // searching for a duplicate name is easy
        match dir_entry {
            DirectoryEntry::Directory(dir) => {
                // If a file or directory with that name exists
                if self.children.iter().find(|x| {
                    match **x {
                        DirectoryEntry::Directory(d) => {
                            d.name == dir.name
                        },
                        _ => false,
                    }
                }).is_some() {
                    println!(
                        "[error] directory or file with that name already exists!"
                    );
                    return;
                }
            },
            DirectoryEntry::File(file) => {

            },
        }

        

        self.children.push(Box::new(dir_entry));
    }

    // fn remove_child(&mut self,);
}