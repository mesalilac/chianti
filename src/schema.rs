// @generated automatically by Diesel CLI.

diesel::table! {
    channels (id) {
        id -> Text,
        name -> Text,
        url -> Text,
        subscribers_count -> BigInt,
        added_at -> BigInt,
    }
}

diesel::table! {
    videos (id) {
        id -> Text,
        channel_id -> Text,
        url -> Text,
        title -> Text,
        watch_counter -> BigInt,
        watch_duration_seconds -> BigInt,
        duration_seconds -> BigInt,
        view_count -> BigInt,
        published_at -> BigInt,
        session_start_date -> BigInt,
        session_end_date -> BigInt,
        added_at -> BigInt,
    }
}

diesel::table! {
    watch_history (id) {
        id -> Text,
        video_id -> Text,
        channel_id -> Text,
        added_at -> BigInt,
    }
}

diesel::joinable!(videos -> channels (channel_id));
diesel::joinable!(watch_history -> channels (channel_id));
diesel::joinable!(watch_history -> videos (video_id));

diesel::allow_tables_to_appear_in_same_query!(
    channels,
    videos,
    watch_history,
);
