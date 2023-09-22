use actix_web::{web, HttpResponse};
use sqlx::{PgPool};
use uuid::Uuid;
use chrono::Utc;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(f_form, f_pool),
    fields(
        subscriber_email = %f_form.email,
        subscriber_name = %f_form.name
    )
)]
pub async fn subscribe(
    f_form: web::Form<FormData>,
    f_pool: web::Data<PgPool>
) -> HttpResponse {

    match insert_subscriber(&f_pool, &f_form).await
    {
        Ok(_) => HttpResponse::Ok ().finish(),
        Err(_) => HttpResponse:: InternalServerError().finish()
    }

}

#[tracing::instrument(
    name = "Saving new subscriber details in the database.",
    skip(f_form, f_pool)
)]
pub async fn insert_subscriber(
    f_pool: &PgPool,
    f_form: &FormData,
) -> Result<(), sqlx::Error> {

    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        f_form.email,
        f_form.name,
        Utc::now()
    )
    .execute(f_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}