use actix_web::{web, HttpResponse};
use sqlx::{PgPool};
use uuid::Uuid;
use chrono::Utc;
use unicode_segmentation::UnicodeSegmentation;

use crate::domain::{NewSubscriber, SubscriberName, SubscriberEmail};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(Self { email, name })
    }
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

    let new_subscriber = match f_form.0.try_into() {
        Ok(form) => form,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };	

    match insert_subscriber(&f_pool, &new_subscriber).await
    {
        Ok(_) => HttpResponse::Ok ().finish(),
        Err(_) => HttpResponse:: InternalServerError().finish()
    }

}

#[tracing::instrument(
    name = "Saving new subscriber details in the database.",
    skip(f_new_subscriber, f_pool)
)]
pub async fn insert_subscriber(
    f_pool: &PgPool,
    f_new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {

    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        f_new_subscriber.email.as_ref(),
        f_new_subscriber.name.as_ref(),
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