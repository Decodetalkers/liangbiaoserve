use crate::utils::{FoldTable, Infomation, Score, StudentForHelp, ToLogin};
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
pub async fn teacherlogininto(
    pool: &Pool<Postgres>,
    tologin: ToLogin,
) -> Result<Option<Infomation>> {
    tologin.checklegal()?;
    let output = sqlx::query_as::<_, Infomation>(&format!(
        "SELECT name, icon from teacherlogin 
            where name='{}' AND passward='{}'",
        tologin.name, tologin.passward
    ))
    .fetch_one(pool)
    .await?;
    Ok(Some(output))
}
pub async fn adminlogininto(pool: &Pool<Postgres>, tologin: ToLogin) -> Result<Option<Infomation>> {
    tologin.checklegal()?;
    let output = sqlx::query_as::<_, Infomation>(&format!(
        "SELECT name, icon from adminlogin 
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
pub async fn studenthelpinto(pool: &Pool<Postgres>, id: String) -> Result<()> {
    let output = sqlx::query_as::<_, StudentForHelp>(
        r#"
        SELECT * from studentforhelp where id = $1
        "#,
    )
    .bind(&id)
    .fetch_one(pool)
    .await;
    if output.is_ok() {
        return Err(anyhow::anyhow!("Alreadyhave"));
    }
    sqlx::query(
        r#"
        INSERT INTO studentforhelp (id) VALUES ($1)
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}
pub async fn delete_student_for_help(pool: &Pool<Postgres>, name: String) -> Result<()> {
    sqlx::query(
        r#"
        delete from studentforhelp where id = $1
        "#
        )
        .bind(name)
        .execute(pool)
        .await?;
    Ok(())
}
pub async fn get_forhelp_students(pool: &Pool<Postgres>) -> Result<Vec<StudentForHelp>> {
    let output = sqlx::query_as::<_, StudentForHelp>(
        r#"
        select id from studentforhelp
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(output)
}
pub async fn get_folds(pool: &Pool<Postgres>) -> Result<Vec<FoldTable>> {
    let output = sqlx::query_as::<_, FoldTable>(
        r#"
        select id from tablefold ORDER BY random() LIMIT 8;
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(output)
}
pub async fn storage_score(pool: &Pool<Postgres>, score: Score) -> Result<()> {
    let output = sqlx::query_as::<_, Score>(
        r#"
        SELECT * from score 
            where id = $1 AND name = $2
        "#,
    )
    .bind(score.id.clone())
    .bind(score.name.clone())
    .fetch_one(pool)
    .await;
    if output.is_err() {
        sqlx::query(
            r#"INSERT INTO score (id, name,duration,score,tabletype) VALUES ($1, $2,$3,$4,$5);"#,
        )
        .bind(score.id)
        .bind(score.name)
        .bind(score.duration)
        .bind(score.score)
        .bind(score.tabletype)
        .execute(pool)
        .await?;
    } else {
        sqlx::query(
            r#"
            UPDATE score 
            set duration = $1, score = $2
            where id = $3 and name = $4
            "#,
        )
        .bind(score.duration)
        .bind(score.score)
        .bind(score.id)
        .bind(score.name)
        .execute(pool)
        .await?;
    }
    Ok(())
}
pub async fn get_history(pool: &Pool<Postgres>, name: String) -> Result<Vec<Score>> {
    Ok(sqlx::query_as::<_, Score>(
        r#"
        SELECT * from score where name = $1
        "#,
    )
    .bind(name)
    .fetch_all(pool)
    .await?)
}
pub async fn get_all_history(pool: &Pool<Postgres>) -> Result<Vec<Score>> {
    Ok(sqlx::query_as::<_, Score>(
        r#"
        SELECT * from score
        "#,
    )
    .fetch_all(pool)
    .await?)
}
