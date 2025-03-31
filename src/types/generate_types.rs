/// The Definition of Files to be created
pub(crate) struct FileToCreate {
    pub path: String,
    pub is_dir: bool,
    pub file_content: Option<String>,
}
