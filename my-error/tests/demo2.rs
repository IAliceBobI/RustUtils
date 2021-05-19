use anyhow::Result;

fn get_info() -> Result<i32> {
    let k = std::io::ErrorKind::WouldBlock;
    let e = std::io::Error::from(k);
    Err(e)?;
    Ok(33)
}

#[test]
fn run() {
    if let Err(e) = get_info() {
        match e.downcast_ref::<std::io::Error>() {
            Some(e) => {
                println!("this is a io error. {}", e);
            }
            None => {
                println!("other")
            }
        }
    }
}
