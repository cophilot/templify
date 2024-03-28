#[allow(dead_code)]
pub mod fs {

    pub fn templates_dir() -> FSItem {
        FSItem::new_root_dir(".templates")
    }

    pub fn dir(name: &str) -> FSItem {
        FSItem::new_root_dir(name)
    }

    pub fn file(name: &str) -> FSItem {
        FSItem::new_root_file(name)
    }

    #[derive(Clone)]
    pub struct FSItem {
        name: String,
        is_dir: bool,
        parent: Option<Box<FSItem>>,
    }

    impl FSItem {
        pub fn new_dir(name: &str, parent: FSItem) -> Self {
            FSItem {
                name: name.to_string(),
                is_dir: true,
                parent: Some(Box::new(parent)),
            }
        }

        pub fn new_root_dir(name: &str) -> Self {
            FSItem {
                name: name.to_string(),
                is_dir: true,
                parent: None,
            }
        }

        pub fn new_file(name: &str, parent: FSItem) -> Self {
            FSItem {
                name: name.to_string(),
                is_dir: false,
                parent: Some(Box::new(parent)),
            }
        }

        pub fn new_root_file(name: &str) -> Self {
            FSItem {
                name: name.to_string(),
                is_dir: false,
                parent: None,
            }
        }

        /// Create a directory in the current directory
        pub fn dir(&mut self, name: &str) -> FSItem {
            if !self.is_dir {
                panic!("INTERNAL: dir called on a file");
            }
            FSItem::new_dir(name, self.clone())
        }

        /// Create a file in the current directory
        pub fn file(&mut self, name: &str) -> FSItem {
            if !self.is_dir {
                panic!("INTERNAL: file called on a file");
            }
            FSItem::new_file(name, self.clone())
        }

        /// Get the full path of the file or directory
        pub fn to_path(&self) -> String {
            let mut path = self.name.clone();
            if let Some(parent) = &self.parent {
                // get path seperator
                path = format!("{}/{}", parent.to_path(), path);
            }
            path
        }

        /// Check that the file or directory exists
        pub fn check_all_exists(&mut self) -> &mut FSItem {
            let path = self.to_path();

            assert!(
                std::path::Path::new(&path).exists(),
                "🚨 Path does not exists: {}",
                path
            );
            self
        }

        /// Check that the file or directory does not exist
        pub fn check_not_exists(&mut self) -> &mut FSItem {
            let path = self.to_path();

            assert!(
                !std::path::Path::new(&path).exists(),
                "🚨 Path exists that shouldn't exists: {}",
                path
            );

            self
        }

        /// Checks if the directory is empty
        pub fn check_that_dir_is_empty(&mut self) -> &mut FSItem {
            if !self.is_dir {
                panic!("INTERNAL: check_that_dir_is_empty called on a file");
            }

            self.check_all_exists();

            let path = self.to_path();
            let mut entries = std::fs::read_dir(&path).unwrap();

            assert!(
                entries.next().is_none(),
                "🚨 Directory is not empty: {}",
                path
            );

            self
        }

        /// Checks that the directory is not empty
        pub fn check_that_dir_is_not_empty(&mut self) -> &mut FSItem {
            if !self.is_dir {
                panic!("INTERNAL: check_that_dir_is_not_empty called on a file");
            }

            self.check_all_exists();

            let path = self.to_path();
            let mut entries = std::fs::read_dir(&path).unwrap();

            assert!(entries.next().is_some(), "🚨 Directory is empty: {}", path);

            self
        }

        /// Checks if the file exists and contains the given string
        pub fn contains_string(&mut self, content: &str) -> &mut FSItem {
            if self.is_dir {
                panic!("INTERNAL: contains_string called on a directory");
            }

            self.check_all_exists();

            let path = self.to_path();
            let file_content = std::fs::read_to_string(&path).unwrap();
            assert!(
                file_content.contains(content),
                "🚨 File {} does not contain string: {}",
                path,
                content
            );
            self
        }

        /// Checks if the file exists and does not contain the given string
        pub fn not_contains_string(&mut self, content: &str) -> &mut FSItem {
            if self.is_dir {
                panic!("INTERNAL: not_contains_string called on a directory");
            }

            self.check_all_exists();

            let path = self.to_path();
            let file_content = std::fs::read_to_string(&path).unwrap();
            assert!(
                !file_content.contains(content),
                "🚨 File {} does contain string: {}",
                path,
                content
            );
            self
        }

        // remove the file or directory
        pub fn remove(&mut self) -> &mut FSItem {
            let path = self.to_path();
            // remove the file or directory
            if self.is_dir {
                std::fs::remove_dir_all(&path).unwrap();
            } else {
                std::fs::remove_file(&path).unwrap();
            }
            self
        }

        /// Remove the content from the file
        pub fn remove_content(&mut self, content: &str) -> &mut FSItem {
            if self.is_dir {
                panic!("INTERNAL: remove_content called on a directory");
            }
            let path = self.to_path();
            let file_content = std::fs::read_to_string(&path).unwrap();
            let new_content = file_content.replace(content, "");
            std::fs::write(&path, new_content).unwrap();
            self
        }

        /// Create the file or directory
        pub fn create(&mut self) -> &mut FSItem {
            let path = self.to_path();
            if self.is_dir {
                std::fs::create_dir_all(&path).unwrap();
            } else {
                std::fs::write(&path, "").unwrap();
            }
            self
        }

        // Create a file with the given content
        pub fn create_file(&mut self, content: &str) -> &mut FSItem {
            if self.is_dir {
                panic!("INTERNAL: create_file called on a directory");
            }
            let path = self.to_path();
            std::fs::write(&path, content).unwrap();
            self
        }

        /// Append content to the file
        pub fn append_line(&mut self, content: &str) -> &mut FSItem {
            if self.is_dir {
                panic!("INTERNAL: append_content called on a directory");
            }
            let path = self.to_path();
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(&path)
                .unwrap();
            std::io::Write::write_all(&mut file, "\n".as_bytes()).unwrap();
            std::io::Write::write_all(&mut file, content.as_bytes()).unwrap();
            self
        }
    }
}
