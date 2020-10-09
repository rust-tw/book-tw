// ANCHOR: here
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --省略--
    // ANCHOR_END: here
    Ok(())
    // ANCHOR: here
}

fn function2() -> io::Result<()> {
    // --省略--
    // ANCHOR_END: here
    Ok(())
    // ANCHOR: here
}
// ANCHOR_END: here
