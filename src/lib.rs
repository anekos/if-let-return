
#[macro_export]
macro_rules! if_let_some {
    ($var:pat = $value:expr, $else_value:expr) => {
        #[allow(clippy::if_let_some_result)]
        let $var = if let Some(it) = $value {
            it
        } else {
            return $else_value;
        };
    }
}

#[macro_export]
macro_rules! if_let_ok {
    ($var:pat = $value:expr, $else_value:expr) => {
        let $var = match $value {
            Ok(it) => it,
            Err(err) => return $else_value(err),
        };
    }
}

