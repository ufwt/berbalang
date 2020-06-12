use std::fmt::Debug;
use std::io;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    ParseInt(std::num::ParseIntError),
    Channel(String),
    Parsing(String),
    Eval(fasteval::error::Error),
    MissingKey(String),
    NoVacancy,
}

macro_rules! impl_error_from {
    ($e:ty, $var:ident) => {
        impl From<$e> for Error {
            fn from(e: $e) -> Self {
                Self::$var(e)
            }
        }
    };
}

impl_error_from!(std::io::Error, IO);
impl_error_from!(fasteval::error::Error, Eval);
impl_error_from!(std::num::ParseIntError, ParseInt);

impl<T: Debug> From<std::sync::mpsc::SendError<T>> for Error {
    fn from(e: std::sync::mpsc::SendError<T>) -> Self {
        Self::Channel(format!("{:?}", e))
    }
}

impl From<std::sync::mpsc::RecvError> for Error {
    fn from(e: std::sync::mpsc::RecvError) -> Self {
        Self::Channel(format!("{:?}", e))
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(e: serde_json::error::Error) -> Self {
        Self::Parsing(e.to_string())
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Self::Parsing(e.to_string())
    }
}

// type RegisterParseError<C> = <<C as Cpu<'static>>::Reg as FromStr>::Err;
//
// impl<C: Cpu<'static>> From<<<C as Cpu<'static>>::Reg as FromStr>::Err> for Error {
//     fn from(_e: RegisterParseError<C>) -> Self {
//         Self::RegisterParsing("Failed to parse register identifier")
//     }
// }
