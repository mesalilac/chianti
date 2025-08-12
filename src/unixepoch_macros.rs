// Unix epoch functions
//
// Example:
//
// query = query.filter(year_unix!(channels_dsl::added_at).eq("2024"));
//

#[macro_export]
macro_rules! year_unix {
    ($ts:expr) => {
        strftime("%Y", $ts, "unixepoch")
    };
}

#[macro_export]
macro_rules! month_unix {
    ($ts:expr) => {
        strftime("%m", $ts, "unixepoch")
    };
}

#[macro_export]
macro_rules! day_unix {
    ($ts:expr) => {
        strftime("%d", $ts, "unixepoch")
    };
}
