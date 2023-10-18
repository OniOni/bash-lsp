use std::io::{self, Write};

pub fn err_msg(mut message: String) -> io::Result<()> {
    let stderr = io::stderr();
    let mut handle = stderr.lock();

    message.push_str("\n");
    handle.write_all(message.as_ref());
    handle.flush();

    Ok(())
}
