use anyhow::Error;
use backtrace::Backtrace;

pub fn recover_error(err: Box<dyn std::any::Any + Send>) -> Error {
    let mut buf = String::new();
    buf.push_str("[panic]\n");
    if let Some(err_str) = err.downcast_ref::<String>() {
        buf.push_str(err_str);
    } else if let Some(err_str) = err.downcast_ref::<&str>() {
        buf.push_str(err_str);
    } else {
        buf.push_str("[Unknown error]");
    }

    let backtrace = Backtrace::new();
    buf.push_str(format!("\n{:?}", backtrace).as_str());

    anyhow::Error::msg(buf)
}
