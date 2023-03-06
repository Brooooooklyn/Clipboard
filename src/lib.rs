#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::borrow::Cow;
use std::env;
use std::io::Write;
use std::process::{Command, Stdio};
use duct::cmd;

use napi::{bindgen_prelude::*, JsBuffer};

#[napi]
pub struct Clipboard {
  inner: arboard::Clipboard,
}

fn clipboard_error_to_js_error(err: arboard::Error) -> Error {
  Error::new(Status::GenericFailure, format!("{err}"))
}

#[napi]
impl Clipboard {
  #[napi(constructor)]
  pub fn new() -> Result<Self> {
    Ok(Clipboard {
      inner: arboard::Clipboard::new().map_err(clipboard_error_to_js_error)?,
    })
  }

  /// Copy text to the clipboard. Has special handling for WSL and SSH sessions, otherwise
  /// falls back to the cross-platform `clipboard` crate
  #[napi]
  pub fn set_text(&mut self, text: String) -> Result<()> {
    if wsl::is_wsl() {
      set_wsl_clipboard(text)?;
    } else if env::var("SSH_CLIENT").is_ok() {
      // we're in an SSH session, so set the clipboard using OSC 52 escape sequence
      set_clipboard_osc_52(text);
    } else {
      // we're probably running on a host/primary OS, so use the default clipboard
      self
        .inner
        .set_text(text)
        .map_err(clipboard_error_to_js_error)?;
    }

    Ok(())
  }

  #[napi]
  pub fn get_text(&mut self) -> Result<String> {
    if wsl::is_wsl() {
      let stdout = cmd!("powershell.exe", "get-clipboard").read()?;
      Ok(stdout.trim().to_string())
    } else if env::var("SSH_CLIENT").is_ok() {
      Err(Error::new(Status::GenericFailure, "SSH clipboard not supported"))
    } else {
      // we're probably running on a host/primary OS, so use the default clipboard
      self.inner.get_text().map_err(clipboard_error_to_js_error)
    }
  }

  #[napi]
  /// Returns a buffer contains the raw RGBA pixels data
  pub fn get_image(&mut self, env: Env) -> Result<JsBuffer> {
    self
      .inner
      .get_image()
      .map_err(clipboard_error_to_js_error)
      .and_then(|image| unsafe {
        env.create_buffer_with_borrowed_data(
          image.bytes.as_ptr(),
          image.bytes.len(),
          image,
          |i, _| {
            drop(i);
          },
        )
      })
      .map(|b| b.into_raw())
  }

  #[napi]
  /// RGBA bytes
  pub fn set_image(&mut self, width: u32, height: u32, image: Buffer) -> Result<()> {
    self
      .inner
      .set_image(arboard::ImageData {
        width: width as usize,
        height: height as usize,
        bytes: Cow::Borrowed(image.as_ref()),
      })
      .map_err(clipboard_error_to_js_error)
  }
}

/// Set the clipboard contents using OSC 52 (picked up by most terminals)
fn set_clipboard_osc_52(text: String) {
  print!("\x1B]52;c;{}\x07", base64::encode(text));
}

/// Set the Windows clipboard using clip.exe in WSL
fn set_wsl_clipboard(s: String) -> Result<()> {
  let mut clipboard = Command::new("clip.exe").stdin(Stdio::piped()).spawn()?;
  let mut clipboard_stdin = clipboard
    .stdin
    .take()
    .ok_or_else(|| Error::new(Status::GenericFailure, "Could not get stdin handle for clip.exe"))?;

  clipboard_stdin.write_all(s.as_bytes())?;

  Ok(())
}
