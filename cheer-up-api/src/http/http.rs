use crate::http::error::Error;

type Result<T, E = Error> = std::result::Result<T, E>;
