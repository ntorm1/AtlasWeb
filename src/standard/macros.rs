#[macro_export]
macro_rules! handle_result {
    ($result:expr, $error_message:expr) => {
        if let Err(ref error) = $result {
            let formatted_error_message = format!("{}", $error_message);
            let full_error_message = format!("{}: {}", formatted_error_message, error);
            eprintln!("{}", full_error_message);
            assert!(false);
        }
    };
}
