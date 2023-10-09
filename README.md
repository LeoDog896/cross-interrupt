# cross-interrupt

cross-platform graceful program killer

```rs
use std::process::Command;
use std::{thread, time};
use cross_interrupt::interrupt;

let mut command = Command::new("node");
if let Ok(mut child) = command.spawn() {
    // give the process a chance to start
    thread::sleep(time::Duration::from_millis(100));

    // interrupt twice for our "press Ctrl+C again" message
    interrupt(&mut child).expect("command couldn't be interrupted");
    interrupt(&mut child).expect("command couldn't be interrupted");

    // wait for the command to finish
    child.wait().expect("command wasn't running");
} else {
    println!("node command didn't start");
}
```
