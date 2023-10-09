use std::fmt::Display;

pub(crate) fn unwrap_result<T, E>(res: Result<T, E>, error: &str) -> T
where
    E: Display,
{
    res.or_else(|e| {
        log::error!("{error} : {e}");
        Err(error)
    })
    .unwrap()
}

pub(crate) fn unwrap_option<T>(res: Option<T>, error: &str) -> T {
    if res.is_some() {
        res.unwrap()
    } else {
        log::error!("{error}");
        panic!("{error}")
    }
}
