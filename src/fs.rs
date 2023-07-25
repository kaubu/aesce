use chrono::prelude::*;
use uuid::Uuid;


#[derive(Debug)]
pub struct File {
    // 67e55044-10b1-426f-9247-bb680e5fe0c8
    uuid: Uuid,
    // /home/tom/
    location: String,
    // emails.txt
    name: String,
    // /home/tom/emails.txt
    absolute_path: String,
    // 2014-11-28T12:00:09Z
    created_date: DateTime<chrono::Utc>,
    // 2014-12-03T23:45:22Z
    modified_date: DateTime<chrono::Utc>,
}

impl File {
    pub fn new(location: String, name: String) -> File {
        let uuid = Uuid::new_v4();
        let absolute_path = format!("{}{}", location, name);
        let created_date = Utc::now();
        let modified_date = created_date.clone();

        File {
            uuid,
            location,
            name,
            absolute_path,
            created_date,
            modified_date,
        }
    }
}