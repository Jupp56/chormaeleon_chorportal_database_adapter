use chrono::{NaiveDate, NaiveTime};

pub use crate::enums::*;

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub birthday: Option<NaiveDate>,
    pub is_admin: bool,
    pub section: Section,
    pub image_file_path: String,
    pub street: Option<String>,
    pub number: Option<String>,
    pub zip_code: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub registration_date: NaiveDate,
    pub number_private: Option<String>,
    pub number_mobile: Option<String>,
    pub number_business: Option<String>,
    pub is_active: bool,
    pub channel_event_reminder: Channel,
    pub channel_event_change: Channel,
    pub channel_songbook_change: Channel,
    pub channel_album_change: Channel,
    pub password_reset_key: Option<String>,
    pub password_reset_date: Option<NaiveDate>
}

#[derive(Debug, Queryable)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub date: NaiveDate,
    pub start: NaiveTime,
    pub date_end: Option<NaiveDate>,
    pub end: NaiveTime,
    pub location: Option<String>,
    pub is_regular_practice: bool
}

#[derive(Debug, Queryable)]
pub struct UserEvent {
    pub id: i32,
    pub user_id: i32,
    pub event_id: i32,
    pub response: Response,
    pub comment: Option<String>
}

#[derive(Debug, Queryable)]
pub struct UserPushConfig {
    pub id: i32,
    pub user_id: i32,
    pub device_name: String,
    pub date: NaiveDate,
    pub push_config: String
}
