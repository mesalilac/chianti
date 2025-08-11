#[macro_export]
macro_rules! apply_sort {
    ($query:expr, $column:expr, $order:expr) => {
        match $order {
            Some(SortOrder::Asc) => $query.order($column.asc()),
            Some(SortOrder::Desc) => $query.order($column.desc()),
            None => $query,
        }
    };
}
