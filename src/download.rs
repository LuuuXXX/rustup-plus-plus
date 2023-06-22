use std::{path::{PathBuf, Path}, ops, fs::{self, remove_file, OpenOptions}, env, cell::RefCell, io::Write};

use anyhow::{Result, Context};

use crate::{utils, Backend, TlsBackend, curl};

use url::Url;

#[derive(Clone)]
pub struct DownloadCfg {
    pub dist_root: String,
    pub download_dir: PathBuf,
}

pub struct File {
    path: PathBuf,
}

impl ops::Deref for File {
    type Target = Path;

    fn deref(&self) -> &Path {
        self.path.as_path()
    }
}


pub enum Event<'a> {
    ResumingPartialDownload,
    /// Received the Content-Length of the to-be downloaded data.
    DownloadContentLengthReceived(u64),
    /// Received some data.
    DownloadDataReceived(&'a [u8]),
}

impl DownloadCfg {
    pub fn download(&self, target_file_name: &String) -> Result<File> {
        utils::ensure_dir_exists(&"Download Directory".to_string(), &self.download_dir)?;

        let target_file = self.download_dir.join(Path::new(target_file_name));
        if target_file.exists() {
            fs::remove_file(&target_file).context("cleaning up previous download")?;
        }
        let url = utils::parse_url(&self.dist_root)?;

        if let Err(err) = download_file(&url, &target_file) {
            panic!("failed to download file {:?} from url: {}, \n cause: {:?}", target_file_name, url, err);
        }

        Ok(File { path: target_file })
    }
}

pub fn download_v1_manifest() {
    todo!()
}

fn download_file(url: &Url, path: &PathBuf) -> Result<()> {
    // Download the file

    println!("URL {:?}", url.to_string());
    println!("PATH {:?}", path);

    // Keep the curl env var around for a bit
    let use_curl_backend = env::var_os("RUSTUP_USE_CURL").is_some();
    let use_rustls = env::var_os("RUSTUP_USE_RUSTLS").is_some();

    let backend = if use_curl_backend {
        Backend::Curl
    } else {
        let tls_backend = if use_rustls {
            TlsBackend::Rustls
        } else {
            #[cfg(feature = "request-default-tls")] 
            {
                TlsBackend::Default
            }
            #[cfg(not(feature = "request-default-tls"))] 
            {
                TlsBackend::Rustls
            }
        };
        Backend::Reqwest(tls_backend)
    };

    let res = download_to_path_with_backend(&backend, url, path);

    res
}

fn download_with_backend(
    backend: &Backend, 
    url: &Url,
    callback: &dyn Fn(Event<'_>) -> Result<()>,
) -> Result<()> {
    match backend {
        Backend::Curl => curl::download(url, callback),
        Backend::Reqwest(_) => todo!(),
    }
}

fn download_to_path_with_backend(backend: &Backend, url: &Url, path: &Path) -> Result<()> {
    || -> Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .context("error creating file for download")?;

        let file = RefCell::new(file);

        download_with_backend(backend, url, &|event| {
            if let Event::DownloadDataReceived(data) = event {
                file.borrow_mut()
                .write_all(data)
                .context("unable to write downloaded to disk")?;
            }
            Ok(())
        })?;

        file.borrow_mut()
            .sync_data()
            .context("unable to write downloaded to disk")?;

        Ok(())
    }()
    .map_err(|e| {
        if let Err(file_err) = remove_file(path).context("cleaning up cached downloads") {
            file_err.context(e)
        } else {
            e
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download() {
        env::set_var("RUSTUP_USE_CURL", "true");
        let url = Url::parse("https://static.rust-lang.org/dist/2023-06-14/rust-nightly-x86_64-pc-windows-msvc.tar.gz");
        let path = PathBuf::from(r"D:\Normal\projects\rustup-plus-plus\rust-nightly-x86_64-pc-windows-msvc.tar.gz");
        if let Ok(url) = url {
            let _ = download_file(&url, &path);
        }
    }
}