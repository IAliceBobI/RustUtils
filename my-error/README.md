# my-error

Forked from `https://github.com/FindoraNetwork/RUC`,
and depended on `anyhow`.

## Example

```rust
use my_error::{bail, d, ge, pnk, se, MyResultTrait, Result};

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
        match e.get_root_error().downcast_ref::<std::io::Error>() {
            Some(e) => {
                println!("this is an io error. {}", e);
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

```

**Output:**

```txt
Error: hello At: my-error/tests/demo1.rs Line: 10 Column: 38
Caused By: "aaa3" At: my-error/tests/demo1.rs Line: 8 Column: 16
 Caused By: "aaa2" At: my-error/tests/demo1.rs Line: 7 Column: 16
   Caused By: "aaa1" At: my-error/tests/demo1.rs Line: 6 Column: 16
     Caused By: Numerical argument out of domain (os error 33) At: _.rs Line: 0 Column: 0
this is an io error. Numerical argument out of domain (os error 33)
```
