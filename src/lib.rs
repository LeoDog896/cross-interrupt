use std::io::Result;
use std::process::Child;

/// Gracefully shutdowns the child process, if it is still running.
/// If it's not, `Ok(())` is returned.
///
/// The mapping to [`ErrorKind`]s is not part of the compatibility contract of the function.
///
/// This is equivalent to sending a SIGINT on Unix platforms.
pub fn interrupt(child: &mut Child) -> Result<()> {
    if child.try_wait()?.is_some() {
        Ok(())
    } else {
        let pid = child.id();

        #[cfg(unix)]
        cvt::cvt(unsafe { libc::kill(pid as i32, libc::SIGINT) }).map(drop)?;

        #[cfg(not(unix))]
        {
            use std::io::Error;
            use windows_sys::Win32::System::Console::{GenerateConsoleCtrlEvent, CTRL_C_EVENT};

            // Sends a CTRL C event to the PID process
            if unsafe { GenerateConsoleCtrlEvent(CTRL_C_EVENT, pid) } == 0 {
                return Err(Error::last_os_error());
            }
        }

        child.wait()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*;
    use std::process::Command;
    use std::{thread, time};
    use uuid::Uuid;

    use super::*;

    #[test]
    fn it_works() {
        let uuid = Uuid::new_v4().to_string();

        let mut command = Command::cargo_bin("dummy")
            .unwrap()
            .arg(uuid.clone())
            .spawn()
            .expect("Failed to spawn process");

        // We need to wait for a brief period of time to ensure the process has time to hook the signal handler
        thread::sleep(time::Duration::from_millis(100));

        interrupt(&mut command).unwrap();

        // check if the file "signature.txt" exists and the content is the same as the uuid
        let path = std::env::current_dir().unwrap();
        let path = path.join("signature.txt");

        assert!(path.exists());
        assert_eq!(uuid, std::fs::read_to_string(path).unwrap());
    }
}
