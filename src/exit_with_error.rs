use std::process::exit;

pub trait ExitWithError {
    fn message(&self, message: &str) -> !;
}

pub struct ExitWithErrorImpl();

impl ExitWithError for ExitWithErrorImpl {
    fn message(&self, message: &str) -> ! {
        println!("{}", message);
        exit(1);
    }
}
