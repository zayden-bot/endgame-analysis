use zayden_core::ErrorResponse;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    WeaponNotFound(String),
}

impl Error {
    pub fn weapon_not_found(weapon: &str) -> Self {
        let response = format!("Weapon {} not found", weapon);

        Self::WeaponNotFound(response)
    }
}

impl ErrorResponse for Error {
    fn to_response(&self) -> &str {
        match self {
            Self::WeaponNotFound(response) => response,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
