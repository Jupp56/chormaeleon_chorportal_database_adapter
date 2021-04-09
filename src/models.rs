use chrono::{NaiveDate, NaiveTime};

use crate::enums;
use crate::enums::Response;

#[derive(Debug, Queryable)]
pub struct User {
    id: i32,
    email: String,
    password: String,
    first_name: Option<String>,
    last_name: Option<String>,
    birthday: Option<NaiveDate>,
    is_admin: bool,
    section: enums::Section,
    image_file_path: String,
    street: Option<String>,
    number: Option<String>,
    zip_code: Option<String>,
    city: Option<String>,
    country: Option<String>,
    registration_date: NaiveDate,
    number_private: Option<String>,
    number_mobile: Option<String>,
    number_business: Option<String>,
    is_active: bool,
    channel_event_reminder: enums::Channel,
    channel_event_change: enums::Channel,
    channel_songbook_change: enums::Channel,
    channel_album_change: enums::Channel,
    password_reset_key: Option<String>,
    password_reset_date: Option<NaiveDate>
}

#[derive(Debug, Queryable)]
pub struct Event {
    id: i32,
    name: String,
    description: Option<String>,
    date: NaiveDate,
    start: NaiveTime,
    date_end: Option<NaiveDate>,
    end: NaiveTime,
    location: Option<String>,
    is_regular_practice: bool
}

#[derive(Debug, Queryable)]
pub struct UserEvent {
    id: i32,
    user_id: i32,
    event_id: i32,
    response: Response,
    comment: Option<String>
}

#[derive(Debug, Queryable)]
pub struct UserPushConfig {
    id: i32,
    user_id: i32,
    device_name: String,
    date: NaiveDate,
    push_config: String
}
