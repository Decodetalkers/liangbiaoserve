use crate::utils::{FoldTable, Infomation, ToLogin};
use anyhow::Result;
use sqlx::{Pool, Postgres};
mod illegaled;
use illegaled::*;
pub async fn logininto(pool: &Pool<Postgres>, tologin: ToLogin) -> Result<Option<Infomation>> {
    tologin.checklegal()?;
    let output = sqlx::query_as::<_, Infomation>(&format!(
        "SELECT name, icon from login 
            where name='{}' AND passward='{}'",
        tologin.name, tologin.passward
    ))
    .fetch_one(pool)
    .await?;
    Ok(Some(output))
}
pub async fn registinto(pool: &Pool<Postgres>, tologin: ToLogin) -> Result<Option<Infomation>> {
    tologin.checklegal()?;
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
pub async fn storageinto(pool: &Pool<Postgres>, path: String) -> Result<()> {
    sqlx::query(&format!("INSERT INTO tablefold (id) VALUES ('{}')", path,))
        .execute(pool)
        .await?;
    Ok(())
}
pub async fn get_folds(pool: &Pool<Postgres>) -> Result<Vec<FoldTable>> {
    let output = sqlx::query_as::<_, FoldTable>(
        r#"
        select id from tablefold ORDER BY random() LIMIT 5;
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(output)
}
