use my_error::{bail, d, ge, pnk, se, MyResultTrait, Result};
use thiserror::Error;

fn xxx() -> Result<i32> {
    let a = std::result::Result::Err(std::io::Error::from_raw_os_error(33));

    let msg1 = d!(@"aaa1");
    let msg2 = d!(@"aaa2");
    let msg3 = d!(@"aaa3");

    se!(a).c(msg1).c(msg2).c(msg3).c(d!("hello"))?;
    Ok(0)
}

// run with: `cargo test -p my-error -- --nocapture`
#[test]
fn demo() {
    if let Err(e) = xxx() {
        println!("{}", e);
        match e.get_root_error().unwrap().downcast_ref::<std::io::Error>() {
            Some(e) => {
                println!("this is an io error. {}", e);
            }
            None => {
                println!("other")
            }
        }
    }
}

#[derive(Error, Debug)]
enum BizError {
    #[error("{0}")]
    Code(i32),
}

fn xxx2() -> Result<i32> {
    let abc = BizError::Code(44);
    let a = bail!(abc);

    let msg1 = d!(@"aaa1");
    let msg2 = d!(@"aaa2");
    let msg3 = d!(@"aaa3");

    a.c(msg1).c(msg2).c(msg3).c(d!("hello"))?;
    Ok(0)
}

#[test]
fn demo4() {
    if let Err(e) = xxx2() {
        println!("{}", e);
        match e.get_root_error().unwrap().downcast_ref::<BizError>() {
            Some(e) => {
                println!("this is BizError {}", e);
            }
            None => {
                println!("other")
            }
        }
    }
}

#[test]
fn demo2() {
    let result: Result<i32> = bail!("hello {} !!", 44);
    println!("{:?}", result);
    pnk!(Result::<i32>::Ok(0));
}

#[test]
fn demo3() {
    let result: Result<i32> = Err(ge!("hello {} !!", 33));
    println!("{:?}", result);
}
