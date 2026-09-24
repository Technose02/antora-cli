mod relative_path_impl;
pub use relative_path_impl::{
    RelativeDir, RelativeFile, RelativePath, create_relative_dir_from_dirname,
    create_relative_file_from_filename, new_relative_path,
};

mod error;
pub use error::Error;

mod pathentry;
pub use pathentry::{Dirname, Filename};

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn relative_path_as_file_works() {
        let mut dir =
            create_relative_dir_from_dirname(Dirname::try_from(String::from("erster")).unwrap());
        dir.push_dir(Dirname::try_from(String::from("zweiter")).unwrap());
        let file = dir.push_file(Filename::try_from(String::from("dritter.vier")).unwrap());
        assert!(file.to_string() == "erster/zweiter/dritter.vier")
    }
}
