use crate::utils::{Infomation, ToLogin};
use sqlx::{Pool, Postgres};
pub async fn logininto(
    pool: &Pool<Postgres>,
    tologin: ToLogin,
) -> Result<Option<Infomation>, sqlx::Error> {
    let output = sqlx::query_as::<_, Infomation>(&format!(
        "SELECT name, icon from login 
            where name='{}' AND passward='{}'",
        tologin.name, tologin.passward
    ))
    .fetch_one(pool)
    .await?;
    Ok(Some(output))
}
pub async fn registinto(
    pool: &Pool<Postgres>,
    tologin: ToLogin,
) -> Result<Option<Infomation>, sqlx::Error> {
    let output = sqlx::query_as::<_, Infomation>(&format!(
        "SELECT name, icon from login 
            where name='{}' ",
        tologin.name
    ))
    .fetch_one(pool)
    .await;
    if output.is_err() {
        sqlx::query(&format!(
            "INSERT INTO login (passward, name, icon)
            VALUES ('{}','{}','https://www.baidu.com')",
            tologin.passward, tologin.name,
        ))
        .execute(pool)
        .await?;
        Ok(Some(Infomation::start(tologin.name)))
    } else {
        Ok(None)
    }
}
