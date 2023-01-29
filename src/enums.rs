use diesel::deserialize::{self, FromSql};
use diesel::mysql::Mysql;
use diesel::serialize::{self, IsNull, Output, ToSql};
/// We define the enums used in the database here.
/// The abominations below are necessary to cast the rust enum type to the database enum.
/// Advantage: We cannot possibly misuse it in our code.
///
/// If you ever plan on changing the database, you've got to change this file.
/// If you are somewhat confident in rust and diesel, go for it.
/// While at it - change this to diesel enum derive - I did not get it to run automatically, so I manually mapped it.
/// Otherwise: DON'T. It took me an afternoon and an (example for postgres)[https://github.com/l4l/diesel-custom-types] and a lot of nerves to get this to work.
use diesel::{backend, AsExpression, FromSqlRow, SqlType};
use std::io::Write;

///The custom mapped types for our enums, ready to use in the database schema (schema.rs)
pub mod exports {
    pub use super::ChannelType;
    pub use super::ResponseType;
    pub use super::SectionType;
}

/// ONLY FOR USE IN DATABASE SCHEMA - use ```enums::Response``` in business logic!
///
/// This represents all possible responses to an event - except for null.
#[derive(SqlType)]
#[diesel(mysql_type(name = "String"))]
pub struct ResponseType;

/// This represents all possible responses to an event -
/// except for null, which you can use via wrapping this in an ```Option```.
///
/// This is the enum to use in business logic.
/// For use in the database schema, use the ResponseType struct via ```enums::exports::ResponseType```
#[derive(AsExpression, Clone, Copy, Debug, FromSqlRow, PartialEq, PartialOrd)]
#[diesel(sql_type = ResponseType)]
pub enum Response {
    Yes,
    No,
    Maybe,
}

/// Converts the rust enum options into SQL enum options.
impl ToSql<ResponseType, Mysql> for Response {
    fn to_sql(&self, out: &mut Output<Mysql>) -> serialize::Result {
        match *self {
            Response::Yes => out.write_all(b"yes")?,
            Response::No => out.write_all(b"no")?,
            Response::Maybe => out.write_all(b"maybe")?,
        }
        Ok(IsNull::No)
    }
}

/// Converts the SQL enum options into rust enum options
impl FromSql<ResponseType, Mysql> for Response {
    fn from_sql(value: backend::RawValue<'_, Mysql>) -> deserialize::Result<Self> {
        let bytes = value.as_bytes();

        match bytes {
            b"yes" => Ok(Response::Yes),
            b"no" => Ok(Response::No),
            b"maybe" => Ok(Response::Maybe),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

/// ONLY FOR USE IN DATABASE SCHEMA - use ```enums::Channel``` in business logic!
///
/// This represents all possible communication channels.
#[derive(SqlType)]
#[diesel(mysql_type(name = "String"))]
pub struct ChannelType;

/// This represents all possible communication channels for a message -
/// none is an enum type, no ```Option<T>``` wrapping required (legacy).
///
/// This is the enum to use in business logic.
/// For use in the database schema, use the ChannelType struct via ```enums::exports::ChannelType```
#[derive(AsExpression, Clone, Copy, Debug, FromSqlRow, PartialEq, PartialOrd)]
#[diesel(sql_type = ChannelType)]
pub enum Channel {
    None,
    Email,
    Push,
    Telegram,
}

/// Converts the rust enum options into SQL enum options.
impl ToSql<ChannelType, Mysql> for Channel {
    fn to_sql(&self, out: &mut Output<Mysql>) -> serialize::Result {
        match *self {
            Channel::None => out.write_all(b"none")?,
            Channel::Email => out.write_all(b"email")?,
            Channel::Push => out.write_all(b"push")?,
            Channel::Telegram => out.write_all(b"telegram")?,
        }
        Ok(IsNull::No)
    }
}

/// Converts the SQL enum options into rust enum options
impl FromSql<ChannelType, Mysql> for Channel {
    fn from_sql(value: backend::RawValue<'_, Mysql>) -> deserialize::Result<Self> {
        let bytes = value.as_bytes();

        match bytes {
            b"none" => Ok(Channel::None),
            b"email" => Ok(Channel::Email),
            b"push" => Ok(Channel::Push),
            b"telegram" => Ok(Channel::Telegram),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

/// ONLY FOR USE IN DATABASE SCHEMA - use ```enums::Section``` in business logic!
///
/// This represents all possible sections a user can be part of.
/// A user only has one section (legacy).
#[derive(SqlType)]
#[diesel(mysql_type(name = "String"))]
pub struct SectionType;

/// This represents all possible sections a user can be part of.
/// A user is part of exactly one section (legacy).
///
/// This is the enum to use in business logic.
/// For use in the database schema, use the ChannelType struct via ```enums::exports::SectionType```
#[derive(AsExpression, Clone, Copy, Debug, FromSqlRow, PartialEq, PartialOrd)]
#[diesel(sql_type = SectionType)]
pub enum Section {
    Sopran,
    Alt,
    Tenor,
    Bass,
    Dirigent,
    Instrument,
}

/// Converts the rust enum options into SQL enum options.
impl ToSql<SectionType, Mysql> for Section {
    fn to_sql(&self, out: &mut Output<Mysql>) -> serialize::Result {
        match *self {
            Section::Sopran => out.write_all(b"Sopran")?,
            Section::Alt => out.write_all(b"Alt")?,
            Section::Tenor => out.write_all(b"Tenor")?,
            Section::Bass => out.write_all(b"Bass")?,
            Section::Dirigent => out.write_all(b"Dirigent")?,
            Section::Instrument => out.write_all(b"Instrument")?,
        }
        Ok(IsNull::No)
    }
}

/// Converts the SQL enum options into rust enum options
impl FromSql<SectionType, Mysql> for Section {
    fn from_sql(value: backend::RawValue<'_, Mysql>) -> deserialize::Result<Self> {
        let bytes = value.as_bytes();

        match bytes {
            b"Sopran" => Ok(Section::Sopran),
            b"Alt" => Ok(Section::Alt),
            b"Tenor" => Ok(Section::Tenor),
            b"Bass" => Ok(Section::Bass),
            b"Dirigent" => Ok(Section::Dirigent),
            b"Instrument" => Ok(Section::Instrument),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
