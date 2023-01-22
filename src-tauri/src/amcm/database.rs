use crate::amcm::AMCM_DIR;
use sea_orm::{Database, DbErr};

pub async fn init_db() -> Result<(), DbErr> {
    let db_url = format!(
        "sqlite:{}?mode=rwc",
        AMCM_DIR.join("amcm.sqlite").to_str().unwrap()
    );
    let db = Database::connect(db_url).await?;

    Ok(())
}
