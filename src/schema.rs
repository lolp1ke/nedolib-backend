// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "genre"))]
    pub struct Genre;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "status"))]
    pub struct Status;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "t_type"))]
    pub struct TType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tag"))]
    pub struct Tag;
}

diesel::table! {
    admins (id) {
        id -> Uuid,
    }
}

diesel::table! {
    artists (id) {
        id -> Uuid,
        name -> Nullable<Text>,
        bio -> Nullable<Text>,
        profile_picture_path -> Nullable<Text>,
    }
}

diesel::table! {
    authors (id) {
        id -> Uuid,
        name -> Nullable<Text>,
        bio -> Nullable<Text>,
        profile_picture_path -> Nullable<Text>,
    }
}

diesel::table! {
    profiles (id) {
        id -> Uuid,
        user_id -> Nullable<Uuid>,
        translator_id -> Nullable<Uuid>,
        admin_id -> Nullable<Uuid>,
        name -> Nullable<Text>,
        bio -> Nullable<Text>,
        profile_picture_path -> Nullable<Text>,
        banner_picture_path -> Nullable<Text>,
    }
}

diesel::table! {
    sessions (id) {
        id -> Uuid,
        user_id -> Uuid,
        ip -> Text,
        created -> Timestamp,
        altered -> Nullable<Timestamp>,
    }
}

diesel::table! {
    teams (id) {
        id -> Uuid,
        owner -> Uuid,
        name -> Text,
        bio -> Nullable<Text>,
        profile_picture_path -> Nullable<Text>,
        banner_picture_path -> Nullable<Text>,
        created -> Timestamp,
        altered -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Status;
    use super::sql_types::Tag;
    use super::sql_types::Genre;

    titles (id) {
        id -> Uuid,
        author_id -> Uuid,
        artist_id -> Uuid,
        name -> Text,
        alternative_names -> Array<Nullable<Text>>,
        release -> Int4,
        status -> Status,
        tags -> Array<Nullable<Tag>>,
        genres -> Array<Nullable<Genre>>,
        created -> Timestamp,
        altered -> Nullable<Timestamp>,
    }
}

diesel::table! {
    titles_teams (title_id, team_id) {
        title_id -> Uuid,
        team_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TType;

    translators (id) {
        id -> Uuid,
        team_id -> Uuid,
        t_type -> Array<Nullable<TType>>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        hash -> Text,
        email -> Text,
        created -> Timestamp,
        altered -> Nullable<Timestamp>,
    }
}

diesel::joinable!(profiles -> admins (admin_id));
diesel::joinable!(profiles -> translators (translator_id));
diesel::joinable!(profiles -> users (user_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(titles -> artists (artist_id));
diesel::joinable!(titles -> authors (author_id));
diesel::joinable!(titles_teams -> teams (team_id));
diesel::joinable!(titles_teams -> titles (title_id));

diesel::allow_tables_to_appear_in_same_query!(
    admins,
    artists,
    authors,
    profiles,
    sessions,
    teams,
    titles,
    titles_teams,
    translators,
    users,
);
