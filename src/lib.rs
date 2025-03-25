use async_trait::async_trait;
use bungie_api::DestinyInventoryItemDefinition;
use sqlx::{AnyConnection, Database, FromRow, Pool};

pub mod dimwishlist;
pub mod endgame_analysis;
pub mod error;
pub mod tierlist;
pub mod weapon;

pub use dimwishlist::DimWishlistCommand;
pub use tierlist::TierListCommand;
pub use weapon::WeaponCommand;

pub use error::Error;
use error::Result;

#[async_trait]
pub trait DestinyWeaponManager<Db: Database> {
    async fn get(pool: &Pool<Db>, name: &str) -> sqlx::Result<DestinyWeapon>;

    async fn get_by_prefix(pool: &Pool<Db>, name: &str) -> sqlx::Result<Vec<DestinyWeapon>>;

    async fn insert(
        conn: &mut AnyConnection,
        weapon: &DestinyInventoryItemDefinition,
        perks: &[Vec<&DestinyInventoryItemDefinition>],
    ) -> sqlx::Result<()>;

    async fn delete_all(conn: &mut AnyConnection) -> sqlx::Result<()>;
}

#[derive(FromRow)]
pub struct DestinyWeapon {
    pub id: i64,
    pub icon: String,
    pub name: String,
    pub column_1: Vec<i64>,
    pub column_2: Vec<i64>,
    pub perk_1: Vec<i64>,
    pub perk_2: Vec<i64>,
}

#[async_trait]
pub trait DestinyPerkManager<Db: Database> {
    async fn get(pool: &Pool<Db>, name: &str) -> sqlx::Result<DestinyPerk>;

    async fn get_all(pool: &Pool<Db>, names: &[String]) -> sqlx::Result<Vec<DestinyPerk>>;

    async fn insert(
        conn: &mut AnyConnection,
        perk: &DestinyInventoryItemDefinition,
    ) -> sqlx::Result<()>;

    async fn delete_all(conn: &mut AnyConnection) -> sqlx::Result<()>;
}

#[derive(FromRow)]
pub struct DestinyPerk {
    pub id: i64,
    pub name: String,
    pub description: String,
}
