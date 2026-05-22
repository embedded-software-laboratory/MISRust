fn returns_err() -> Result<(),()>{
    Err(())
}


fn main() -> Result<(), ()> {
    // ? operator propagates error, if the Result does yield an Err
    let res = returns_err()?;
    // direct unwrap of Result, if it yields Err, this panics
    let res2 = returns_err().unwrap();
    if let Ok(val) = returns_err() {
        // handle only Ok(()), missing Error handling
    }
    Ok(())
}