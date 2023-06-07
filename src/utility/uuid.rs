use base64::engine::general_purpose;
use base64::Engine;
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Base64DeserializerError;

impl fmt::Display for Base64DeserializerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unable to deserialize base64 to type")
    }
}

pub trait ToBase64<T> {
    fn base64_serialize(param: T) -> String;
    fn base64_deserialize<S>(param: S) -> Result<T, Base64DeserializerError>
    where
        S: Into<String>;
}

impl ToBase64<Uuid> for Uuid {
    fn base64_serialize(param: Uuid) -> String {
        general_purpose::STANDARD_NO_PAD.encode(param)
    }

    fn base64_deserialize<S>(param: S) -> Result<Uuid, Base64DeserializerError>
    where
        S: Into<String>,
    {
        let decoded_result = general_purpose::STANDARD_NO_PAD.decode(param.into());
        if let Ok(decoded) = decoded_result {
            Uuid::from_slice(decoded.as_slice()).map_err(|_| Base64DeserializerError)
        } else {
            Err(Base64DeserializerError)
        }
    }
}
