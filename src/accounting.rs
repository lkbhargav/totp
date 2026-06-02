use crate::filehandler::FileHandler;
use std::error::Error;

pub fn save_account(acc: &str, token: &str) -> Result<(), Box<dyn Error>> {
    let mut fh = FileHandler::init();
    fh.load_account_file()?;
    // check if account exists before adding.
    fh.add_account(acc.trim_end().to_owned(), token.trim_end().to_owned());
    fh.save_account_file();
    Ok(())
}

pub fn load_token(acc: &str) -> Result<String, Box<dyn Error>> {
    let mut fh = FileHandler::init();
    fh.load_account_file()?;
    let t = fh.get_token(acc.trim_end().to_owned());
    Ok(t?)
}

pub fn delete_account(acc: &str) -> Result<(), Box<dyn Error>> {
    let mut fh = FileHandler::init();
    fh.load_account_file()?;
    // check if account exists before adding.
    fh.delete_account(acc.trim_end().to_owned())?;
    fh.save_account_file();
    Ok(())
}
