use std::io::{self, prelude::*};
use std::io::{Seek, Write};
use std::iter::Iterator;
use std::ops::Add;

use zip::result::ZipError;
use zip::write::FileOptions;

use std::fs::{self, File};
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

const METHOD_STORED: Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Stored);

#[cfg(any(
    feature = "deflate",
    feature = "deflate-miniz",
    feature = "deflate-zlib"
))]
const METHOD_DEFLATED: Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Deflated);
#[cfg(not(any(
    feature = "deflate",
    feature = "deflate-miniz",
    feature = "deflate-zlib"
)))]
const METHOD_DEFLATED: Option<zip::CompressionMethod> = None;

#[cfg(feature = "bzip2")]
const METHOD_BZIP2: Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Bzip2);
#[cfg(not(feature = "bzip2"))]
const METHOD_BZIP2: Option<zip::CompressionMethod> = None;

#[cfg(feature = "zstd")]
const METHOD_ZSTD: Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Zstd);
#[cfg(not(feature = "zstd"))]
const METHOD_ZSTD: Option<zip::CompressionMethod> = None;

pub fn create_archive(directory: &String, destination: &String, extension: &str) -> i32 {
    println!(
        "Usage: {} <source_directory> {} <destination_zipfile>",
        directory, destination
    );

    let dst_file = "".to_string().add(&destination).add(&extension);
    for &method in [METHOD_STORED, METHOD_DEFLATED, METHOD_BZIP2, METHOD_ZSTD].iter() {
        if method.is_none() {
            continue;
        }
        match doit(directory, &dst_file, method.unwrap()) {
            Ok(_) => println!("done: {} written to {}", directory, dst_file),
            Err(e) => println!("Error: {:?}", e),
        }
    }
    0
}

pub fn create_archives(directory: &String, extension: &str) -> i32 {
    println!(
        "Usage: {} <source_directory> {} <destination_zipfile>",
        directory, directory
    );

    for &method in [METHOD_STORED, METHOD_DEFLATED, METHOD_BZIP2, METHOD_ZSTD].iter() {
        if method.is_none() {
            continue;
        }
        match doit_dir(directory, &extension, method.unwrap()) {
            Ok(_) => println!("done: {} written to {}", directory, directory),
            Err(e) => println!("Error: {:?}", e),
        }
    }
    0
}

fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &str,
    writer: T,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            println!("adding file {:?} as {:?} ...", path, name);
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            println!("adding dir {:?} as {:?} ...", path, name);
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}

fn doit(
    src_dir: &str,
    dst_file: &str,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let path = Path::new(dst_file);
    let file = File::create(&path).unwrap();

    let walkdir = WalkDir::new(src_dir);
    let it = walkdir.into_iter();

    zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, file, method)?;

    Ok(())
}

fn doit_dir(
    src_dir: &str,
    ext_file: &str,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }
    let mut entries = fs::read_dir(src_dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.

    entries.sort();
    let mut counter: u8 = 0;
    for entry in entries {
        let path = entry.as_path().display().to_string();
        println!("{}", &path);
        let file_name = "".to_string().add(&path).add("_").add(counter.to_string().as_str()).add(ext_file);
        let file = File::create(&file_name).unwrap();
        let walkdir = WalkDir::new(&path);
        let it = walkdir.into_iter();
        zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, file, method)?;
        counter += 1;
    }

    Ok(())
}
