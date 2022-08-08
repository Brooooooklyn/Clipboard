#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::borrow::Cow;

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

  #[napi]
  pub fn set_text(&mut self, text: String) -> Result<()> {
    self
      .inner
      .set_text(text)
      .map_err(clipboard_error_to_js_error)?;
    Ok(())
  }

  #[napi]
  pub fn get_text(&mut self) -> Result<String> {
    self.inner.get_text().map_err(clipboard_error_to_js_error)
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
