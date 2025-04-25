use core::fmt::Debug;

/// If the result is error, print it
pub fn print_error<T, Error: Debug>(result: Result<T, Error>) {
    if let Err(err) = result {
        println!("{:?}", err);
    }
}