use std::path::{Path, PathBuf};
use indicatif::{ProgressBar, ProgressIterator};
use anyhow::Result;
use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;
use tempfile::TempDir;

pub fn copy<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<()> {
    let mut stack = vec![PathBuf::from(from.as_ref())];

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    let pb = ProgressBar::new(stack.len() as u64);

    while let Some(working_path) = stack.pop() {
        pb.inc(1);

        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if let Some(f) = path.file_name() {
                let dest_path = dest.join(f);
                fs::copy(&path, &dest_path)?;
            }
        }
    }

    Ok(())
}

pub fn extract_zip<P: AsRef<Path>>(path: P) -> Result<TempDir> {
    let f = fs::File::open(path.as_ref())?;
    let mut zip = zip::ZipArchive::new(f)?;
    let tempdir = tempfile::tempdir()?;

    for i in (0..zip.len()).progress() {
        let mut f = zip.by_index(i)?;
        let outpath = match f.enclosed_name() {
            Some(f) => f.to_owned(),
            None => continue
        };

        let outpath = tempdir.path().join(&outpath);

        if f.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }

            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut f, &mut outfile)?;
        }

        #[cfg(unix)]
        if let Some(mode) = f.unix_mode() {
            fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
        }
    }

    Ok(tempdir)
}