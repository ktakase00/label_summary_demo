#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Yaml(serde_yaml::Error),
    Reqwest(reqwest::Error),
    Json(serde_json::Error),
    ToStr(reqwest::header::ToStrError),
    ParseInt(std::num::ParseIntError),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Self::Io(err)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Error {
        Self::Yaml(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Self::Reqwest(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Self::Json(err)
    }
}

impl From<reqwest::header::ToStrError> for Error {
    fn from(err: reqwest::header::ToStrError) -> Error {
        Self::ToStr(err)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Error {
        Self::ParseInt(err)
    }
}
