table! {
    events (id) {
        id -> Integer,
        name -> Varchar,
        description -> Nullable<Text>,
        date -> Date,
        start -> Time,
        dateEnd -> Nullable<Date>,
        end -> Time,
        location -> Nullable<Varchar>,
        isRegularPractice -> Nullable<Bool>,
    }
}

table! {
    userevents (id) {
        id -> Integer,
        userId -> Integer,
        eventId -> Integer,
        response -> Nullable<crate::enums::exports::ResponseType>,
        comment -> Nullable<Varchar>,
    }
}

table! {
    userpushconfig (id) {
        id -> Integer,
        userId -> Integer,
        deviceName -> Varchar,
        date -> Datetime,
        pushConfig -> Varchar,
    }
}

table! {
    users (id) {
        id -> Integer,
        email -> Varchar,
        password -> Varchar,
        firstName -> Varchar,
        lastName -> Varchar,
        birthday -> Nullable<Date>,
        isAdmin -> Bool,
        section -> crate::enums::exports::SectionType,
        imageFilePath -> Varchar,
        street -> Nullable<Varchar>,
        number -> Nullable<Varchar>,
        zipCode -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
        country -> Nullable<Varchar>,
        registrationDate -> Date,
        numberPrivate -> Nullable<Varchar>,
        numberMobile -> Nullable<Varchar>,
        numberBusiness -> Nullable<Varchar>,
        isActive -> Bool,
        channelEventReminder -> crate::enums::exports::ChannelType,
        channelEventChange -> crate::enums::exports::ChannelType,
        channelSongbookChange -> crate::enums::exports::ChannelType,
        channelAlbumChange -> crate::enums::exports::ChannelType,
        passwordResetKey -> Nullable<Varchar>,
        passwordResetDate -> Nullable<Date>,
    }
}

joinable!(userevents -> events (eventId));
joinable!(userevents -> users (userId));
joinable!(userpushconfig -> users (userId));

allow_tables_to_appear_in_same_query!(
    events,
    userevents,
    userpushconfig,
    users,
);
