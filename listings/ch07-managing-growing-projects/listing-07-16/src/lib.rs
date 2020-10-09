// ANCHOR: here
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --省略--
    // ANCHOR_END: here
    Ok(())
    // ANCHOR: here
}

fn function2() -> IoResult<()> {
    // --省略--
    // ANCHOR_END: here
    Ok(())
    // ANCHOR: here
}
// ANCHOR_END: here
