use identicon_rs::{error::IdenticonError, Identicon};

pub fn generate(id: &str) -> Result<Vec<u8>, IdenticonError> {
    Identicon::new(id).set_border(35).export_png_data()
}

#[cfg(feature = "napi")]
#[napi_derive::napi(js_name = "genIdenticon")]
pub fn generate_js(id: String) -> napi::Result<napi::bindgen_prelude::Buffer> {
    match generate(&id) {
        Ok(icon) => Ok(icon.into()),
        Err(err) => Err(napi::Error::from_reason(format!(
            "\n{}\n",
            crate::util::error_chain::format_error(&err)
        ))),
    }
}
