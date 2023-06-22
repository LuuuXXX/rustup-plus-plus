use std::{cell::RefCell, time::Duration};

use url::Url;
use curl::easy::Easy;

use crate::Event;

use anyhow::{Result, Context};

const USER_AGENT: &str = concat!("rustup/", env!("CARGO_PKG_VERSION"));

pub fn download(
    url: &Url, 
    callback: &dyn Fn(Event<'_>) -> Result<()>,
) -> Result<()> {
    thread_local! {static EASY: RefCell<Easy> = RefCell::new(Easy::new())};
    EASY.with(|handle| {
        let mut handle = handle.borrow_mut();

        handle.url(url.as_ref())?;
        handle.follow_location(true)?;
        handle.useragent(USER_AGENT)?;
        // Set the download index 
        let _ = handle.resume_from(0);

        // Take the most 30s to connect
        handle.connect_timeout(Duration::new(30, 0))?;
        
        {
            let cberr = RefCell::new(None);
            let mut transfer = handle.transfer();

            // Data callback for libcurl which is called with data that's
            // downloaded. We just feed it into our hasher and also write it out
            // to disk.
            transfer.write_function(|data| {
                match callback(Event::DownloadDataReceived(data)) {
                    Ok(()) => Ok(data.len()),
                    Err(e) => {
                        *cberr.borrow_mut() = Some(e);
                        Ok(0)
                    }
                }
            })?;

            // If an error happens check to see if we had a filesystem error up
            // in `cberr`, but we always want to punt it up.
            transfer.perform().or_else(|e| {
                // If the original error was generated by one of our
                // callbacks, return it.
                match cberr.borrow_mut().take() {
                    Some(cberr) => Err(cberr),
                    None => {
                        // Otherwise, return the error from curl
                        if e.is_file_couldnt_read_file() {
                            Err(e).context("failed to read file or file could not be found")
                        } else {
                            Err(e).context("error during download")?
                        }
                    }
                }
            })?;

        }

        let code = handle.response_code()?;
        match code {
            0 | 200..=299 => {}
            _ => {
                panic!("Failed to download file, http error code : {}", code);
            }
        }

        Ok(())
    })
}