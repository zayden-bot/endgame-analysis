pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    WeaponNotFound(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::WeaponNotFound(weapon) => write!(f, "Weapon {} not found", weapon),
        }
    }
}

impl std::error::Error for Error {}
