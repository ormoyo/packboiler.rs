use dialoguer::Completion;

pub struct PathCompletion;
impl Completion for PathCompletion {
    fn get(&self, input: &str) -> Option<String> {
        let path = std::path::Path::new(input);
        let name = path.file_name()?.to_str()?;

        let parent = path.parent().unwrap_or(path);
        let current_dir = std::env::current_dir().ok()?;

        let parent_absolute = if parent.as_os_str().is_empty() {
            &current_dir
        } else {
            parent
        };

        let files = std::fs::read_dir(parent_absolute).ok()?;
        for file in files {
            let file = match file {
                Ok(file) => file,
                Err(_) => continue,
            };
            let filename = file.file_name();
            let filename_str = match filename.to_str() {
                Some(str) => str,
                None => continue,
            };

            if filename_str.starts_with(name) {
                let path = parent.join(filename);
                return Some(path.to_string_lossy().to_string());
            }
        }
        None
    }
}
