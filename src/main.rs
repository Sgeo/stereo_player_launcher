use anyhow::{Context, bail};

fn copy_to_temp(path: impl AsRef<std::path::Path>, temp: impl AsRef<std::path::Path>) -> Result<(), anyhow::Error> {
    let new_path = temp.as_ref().join(path.as_ref().file_name().with_context(|| format!("Unable to determine file name for {:?}", path.as_ref()))?);
    let safe_new_path = safe_file_name(&new_path);
    std::fs::copy(&path, &safe_new_path).with_context(||format!("Unable to copy {:?} to temporary name {:?}", path.as_ref(), safe_new_path))?;
    Ok(())
}

fn safe_file_name(path: impl AsRef<std::path::Path>) -> std::path::PathBuf {
    let mut buffer = path.as_ref().to_owned();
    buffer.set_file_name("song");
    if let Some(ext) = path.as_ref().extension() {
        buffer.set_extension(ext);
    }
    buffer
}

fn main() -> Result<(), anyhow::Error> {
    let mut exe_dir = std::env::current_exe().context("Unable to locate launcher exe")?;
    if !exe_dir.pop() {
        bail!("Unable to locate launcher exe's parent directory");
    }

    let tempdir = tempfile::tempdir().context("Unable to create temporary directory")?;


    for mus_file_name in std::env::args_os().skip(1) {
        let path: std::path::PathBuf = mus_file_name.into();
        let canonical_path = std::fs::canonicalize(&path).with_context(|| format!("Unable to canonicalize argument {:?}", path))?;
        let stem = canonical_path.file_stem().with_context(|| format!("{:?} has no stem?", canonical_path))?;
        let directory = canonical_path.parent().with_context(|| format!("{:?} doesn't have a parent directory?", canonical_path))?;
        for mus_dir_entry in std::fs::read_dir(directory).with_context(|| format!("Unable to read containing directory {:?}", directory))? {
            let potential_path = mus_dir_entry.context("Unable to examine music directory entry")?.path();
            if potential_path.file_stem() != Some(stem) {
                continue;
            }
            println!("Copying file {:?} to temporary directory.", potential_path);
            copy_to_temp(&potential_path, tempdir.path())?;
        }
    }

    let x64sc_path = match std::env::var_os("VICE") {
        Some(vice_os_string) => std::path::PathBuf::from(vice_os_string),
        None => exe_dir.join("GTK3VICE-3.5-win32").join("bin").join("x64sc")
    };

    let config_path = exe_dir.join("stereoplayer.conf");

    let d64_path = exe_dir.join("stereoplayer105.d64");

    let _vice = std::process::Command::new(x64sc_path).arg("-addconfig").arg(config_path).arg("-fs9").arg(tempdir.path()).arg("-8").arg(&d64_path).arg(&d64_path).output()?;

    

    Ok(())
}
