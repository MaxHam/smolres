use clap::{Parser, ValueEnum};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "smolres")]
#[command(version, about)]
pub struct Args {
    // Path to input image file
    #[arg(short, long, value_parser=validate_input_path)]
    pub input: PathBuf,

    // Path to output image file
    #[arg(short, long, value_parser=validate_output_path)]
    pub output: Option<PathBuf>,

    // Scale of virtualized resolution
    #[arg(short, long, default_value_t = 16)]
    pub resolution: u16,
    // Color depth of individual pixelds
    #[arg(short, long, default_value_t = 2)]
    pub bit_depth: u8,

    // Algorithm to be used for the pixel interpolation
    #[arg(short, long)]
    pub algorithm: Option<Algorithm>,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Algorithm {
    Nearestneighbor,
    AverageArea,
}
impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Algorithm::Nearestneighbor => "nearest",
            Algorithm::AverageArea => "average",
        };
        write!(f, "{}", s)
    }
}
pub fn default_output_path(input: &PathBuf, resolution: u16, algorithm: Algorithm) -> PathBuf {
    let parent = input.parent().unwrap_or_else(|| Path::new(""));
    let stem = input.file_stem().unwrap_or_default().to_string_lossy();
    let ext = input.extension().and_then(|e| e.to_str()).unwrap_or("jpeg"); // fallback if extension is missing or not valid UTF-8
    let filename = format!("{}_res{}_{}.{}", stem, resolution, algorithm, ext);
    parent.join(filename)
}

/**
*  Checks whether the path exists and the file is a `.jpeg`.
* TODO: Add other file types like .png
* TODO: Optimize mut and borrowing here */
fn validate_input_path(path: &str) -> Result<PathBuf, String> {
    let mut pb = &PathBuf::from(path);

    // add validators here
    pb = validate_existance(pb)?;
    pb = validate_file_extension(pb)?;
    return Ok(pb.to_owned());
}

fn validate_output_path(path: &str) -> Result<PathBuf, String> {
    let mut pb = &PathBuf::from(path);
    pb = validate_file_extension(pb)?;

    if let Some(parent) = pb.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).expect("Failed to create parent directory");
        }
    }
    return Ok(pb.to_owned());
}

fn validate_existance(path: &PathBuf) -> Result<&PathBuf, String> {
    if !path.exists() {
        return Err(format!("Path does not exist: {}", path.display()));
    }

    return Ok(path);
}

fn validate_file_extension(path: &PathBuf) -> Result<&PathBuf, String> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase());

    if let Some(ref ext) = ext {
        if ext != "jpg" && ext != "jpeg" {
            return Err(format!("Invalid file extension: {}", path.display()));
        }
    } else {
        return Err(format!("No file extension found: {}", path.display()));
    }

    return Ok(path);
}
#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;

    use crate::cli::validate_existance;
    use crate::cli::validate_file_extension;
    use crate::cli::validate_input_path;
    use crate::cli::validate_output_path;

    #[test]
    fn test_file_exists() {
        // Create a temporary file
        let tmp_dir = env::temp_dir();
        let file_path = tmp_dir.join("test_file.txt");
        fs::write(&file_path, "test").expect("Failed to write temp file");

        let result = validate_existance(&file_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), &file_path);

        // Clean up
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_file_does_not_exist() {
        let file_path = Default::default();
        let result = validate_existance(&file_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_extensions() {
        let valid_cases = ["image.jpg", "pic.jpeg", "image.JPG"];
        for file in valid_cases {
            // Create a temporary file
            let tmp_dir = env::temp_dir();
            let file_path = tmp_dir.join(file);
            fs::write(&file_path, "test").expect("Failed to write temp file");
            let result = validate_file_extension(&file_path);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), &file_path);

            // Cleanup
            fs::remove_file(file_path).unwrap();
        }
    }

    #[test]
    fn test_file_unsupported_extension() {
        let valid_cases = ["image.png", "pic.txt", "image.webp"];
        for file in valid_cases {
            let tmp_dir = env::temp_dir();
            let file_path = tmp_dir.join(file);
            fs::write(&file_path, "test").expect("Failed to write temp file");
            let result = validate_file_extension(&file_path);
            assert!(result.is_err());

            // Cleanup
            fs::remove_file(file_path).unwrap();
        }
    }

    #[test]
    fn test_input_valid_path() {
        // Create a temporary file
        let tmp_dir = env::temp_dir();
        let file_path = tmp_dir.join("test_file.jpg");
        fs::write(&file_path, "test").expect("Failed to write temp file");

        let result = validate_input_path(file_path.to_str().unwrap());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), file_path);

        // Clean up
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_output_valid_path() {
        // Create a temporary file
        let tmp_dir = env::temp_dir();
        let file_path = tmp_dir.join("test_file.jpg");
        fs::write(&file_path, "test").expect("Failed to write temp file");

        let result = validate_output_path(file_path.to_str().unwrap());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), file_path);

        // Clean up
        fs::remove_file(file_path).unwrap();
    }
    #[test]
    fn test_output_invalid_path_no_parent_dir() {
        let file_path: &str = "does/not/exist";
        let result = validate_output_path(file_path);
        assert!(result.is_err());
    }
}
