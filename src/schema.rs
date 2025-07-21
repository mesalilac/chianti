// @generated automatically by Diesel CLI.

diesel::table! {
    channels (id) {
        id -> Text,
        name -> Text,
        url -> Text,
        is_subscribed -> Bool,
        subscribers_count -> BigInt,
        added_at -> BigInt,
    }
}

diesel::table! {
    tags (id) {
        id -> Text,
        name -> Text,
        added_at -> BigInt,
    }
}

diesel::table! {
    video_tags (video_id, tag_id) {
        video_id -> Text,
        tag_id -> Text,
    }
}

diesel::table! {
    videos (id) {
        id -> Text,
        channel_id -> Text,
        url -> Text,
        title -> Text,
        description -> Text,
        watch_counter -> BigInt,
        duration_seconds -> BigInt,
        likes_count -> BigInt,
        view_count -> BigInt,
        comments_count -> BigInt,
        published_at -> BigInt,
        added_at -> BigInt,
    }
}

diesel::table! {
    watch_history (id) {
        id -> Text,
        video_id -> Text,
        channel_id -> Text,
        watch_duration_seconds -> BigInt,
        session_start_date -> BigInt,
        session_end_date -> BigInt,
        added_at -> BigInt,
    }
}

diesel::joinable!(video_tags -> tags (tag_id));
diesel::joinable!(video_tags -> videos (video_id));
diesel::joinable!(videos -> channels (channel_id));
diesel::joinable!(watch_history -> channels (channel_id));
diesel::joinable!(watch_history -> videos (video_id));

diesel::allow_tables_to_appear_in_same_query!(
    channels,
    tags,
    video_tags,
    videos,
    watch_history,
);
