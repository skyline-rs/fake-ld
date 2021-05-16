use zip::{ZipWriter, CompressionMethod};
use zip::write::FileOptions;

use std::io::{BufWriter, Seek, Write};
use std::path::{Path, PathBuf};
use std::env;
use std::fs;

fn walk_dir<W>(zip: &mut ZipWriter<W>, path: &Path, parent_in_zip: Option<&Path>)
    where W: Write + Seek,
{
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path_in_zip = parent_in_zip
            .map(|parent| parent.join(entry.file_name()))
            .unwrap_or_else(|| PathBuf::from(entry.file_name()));

        let path = entry.path();

        let file_type = entry.file_type().unwrap();
        if file_type.is_dir() {
            walk_dir(zip, &path, Some(&path_in_zip))
        } else if file_type.is_file() {
            zip.start_file_from_path(
                &path_in_zip,
                FileOptions::default().compression_method(CompressionMethod::Deflated)
            ).unwrap();
            zip.write(&fs::read(&path).unwrap()).unwrap();
        }
    }
}

fn main() {
    let mut args = env::args();
    let _ = args.next();
    let folder = args.next().expect("missing folder name");
    let zip = args.next().expect("missing zip file name");

    let mut zip = ZipWriter::new(BufWriter::new(fs::File::create(zip).unwrap()));
    walk_dir(&mut zip, Path::new(&folder), None);
    zip.finish().unwrap();
}
