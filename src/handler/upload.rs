use crate::error::CustomError;
use beatoraja_play_recommend::{Account, MySQLClient, ScoreRepository, SqliteClient};
use bytes::BufMut;
use futures::TryStreamExt;
use warp::filters::multipart::{FormData, Part};
use warp::{Rejection, Reply};

pub async fn upload_score_handler(
    form: FormData,
    token: String,
) -> std::result::Result<impl Reply, Rejection> {
    let account = super::get_account(token)?;
    let dir_name = account.user_id();
    save_sqlite_file(form, dir_name.clone(), "score".into()).await?;
    update_score_data(account, dir_name).await
}

pub async fn upload_score_log_handler(
    form: FormData,
    token: String,
) -> std::result::Result<impl Reply, Rejection> {
    let account = super::get_account(token)?;
    let dir_name = account.user_id();
    save_sqlite_file(form, dir_name.clone(), "scorelog".into()).await?;
    update_score_data(account, dir_name).await
}

async fn update_score_data(account: Account, dir_name: String) -> Result<String, Rejection> {
    let score_file_name = format!("./files/{}/score.db", dir_name);
    let scorelog_file_name = format!("./files/{}/scorelog.db", dir_name);

    let _score_file = tokio::fs::read(&score_file_name)
        .await
        .map_err(|_| CustomError::FileIsNotFound.rejection())?;
    let _scorelog_file = tokio::fs::read(&scorelog_file_name)
        .await
        .map_err(|_| CustomError::FileIsNotFound.rejection())?;

    let sqlite_client = SqliteClient::new(
        scorelog_file_name.clone(),
        "".into(),
        score_file_name.clone(),
    );
    let mysql_client = MySQLClient::new();

    let scores = sqlite_client.score();
    mysql_client
        .save_score(account, scores)
        .map_err(|_| CustomError::SaveIsNotComplete.rejection())?;

    let _remove_score = tokio::fs::remove_file(&score_file_name)
        .await
        .map_err(|_| CustomError::FileIsNotDeleted.rejection())?;
    let _remove_score_log = tokio::fs::remove_file(&scorelog_file_name)
        .await
        .map_err(|_| CustomError::FileIsNotDeleted.rejection())?;
    Ok("Score Is Updated.".into())
}

pub async fn upload_song_data_handler(
    form: FormData,
    token: String,
) -> std::result::Result<String, Rejection> {
    let account = super::get_account(token)?;
    let dir_name = account.user_id();
    save_sqlite_file(form, dir_name.clone(), "songdata".into()).await?;

    let song_file_name = format!("./files/{}/songdata.db", dir_name);
    let _song_file = tokio::fs::read(&song_file_name)
        .await
        .map_err(|_| CustomError::FileIsNotFound.rejection())?;

    let sqlite_client = SqliteClient::new("".into(), song_file_name.clone(), "".into());
    let mysql_client = MySQLClient::new();

    let songs = sqlite_client
        .song_data()
        .map_err(|_| CustomError::FileIsInvalid.rejection())?;

    mysql_client
        .save_song(&songs)
        .map_err(|_| CustomError::SaveIsNotComplete.rejection())?;

    let _remove_data = tokio::fs::remove_file(&song_file_name)
        .await
        .map_err(|_| CustomError::FileIsNotDeleted.rejection())?;

    Ok("SongData Is Updated.".into())
}

async fn save_sqlite_file(
    form: FormData,
    dir_name: String,
    file_name: String,
) -> std::result::Result<String, Rejection> {
    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        eprintln!("form error: {}", e);
        warp::reject::reject()
    })?;
    for part in parts {
        return if part.name() == "file" {
            let value = part
                .stream()
                .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
                })
                .await
                .map_err(|_| CustomError::ReadingFileError.rejection())?;
            tokio::fs::create_dir_all(format!("./files/{}", dir_name))
                .await
                .map_err(|_| CustomError::DirectoryCouldNotCreated.rejection())?;
            let file_name = format!("./files/{}/{}.db", dir_name, file_name);
            tokio::fs::write(&file_name, value).await.map_err(|e| {
                eprint!("error writing file: {}", e);
                CustomError::WritingFileError.rejection()
            })?;
            Ok(file_name)
        } else {
            Err(warp::reject::reject())
        };
    }
    return Ok("".into());
}
