use my_error::{d, MyResultTrait, Result};

fn xxx(_arg: Option<i32>) -> Result<i32> {
    let a = std::result::Result::Err(std::io::Error::from_raw_os_error(33));

    let msg1 = d!(@"aaa1");
    let msg2 = d!(@"aaa2");
    let msg3 = d!(@"aaa3");

    a.c(msg1).c(msg2).c(msg3).c(d!("hello"))?;
    Ok(0)
}

// run with: `cargo test -p my-error -- --nocapture`
#[test]
fn demo() {
    if let Err(e) = xxx(None) {
        println!("{}", e);
    }
    if let Err(e) = xxx(None) {
        println!("{}", e);
    }
}
