use std::{error, fmt, io};
/// A useless Error just for the Demo
#[derive(Copy, Clone, Debug)]
pub struct  HbError;
impl fmt::Display for HbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error While Hbping this page.")
    }
}

impl error::Error for HbError {}

impl From<reqwest::Error> for HbError {
    fn from(_: reqwest::Error) -> Self {
        Self
    }
}

impl From<io::Error> for HbError {
    fn from(_: io::Error) -> Self {
        Self
    }
}

/// Load a page and return its HTML body as a `String`
pub async fn load_page(url: &str) -> Result<String, HbError> {
     Ok(reqwest::get(url).await?.text().await?)
    // Ok("".into())
}

pub fn add(a:i32, b:i32) -> i32{
    a + b
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
