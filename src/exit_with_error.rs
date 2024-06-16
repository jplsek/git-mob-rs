use std::process::exit;

pub trait ExitWithError {
    fn message<S: AsRef<str>>(&self, message: S) -> !;
}

pub struct ExitWithErrorImpl();

impl ExitWithError for ExitWithErrorImpl {
    fn message<S: AsRef<str>>(&self, message: S) -> ! {
        println!("{}", message.as_ref());
        exit(1);
    }
}
