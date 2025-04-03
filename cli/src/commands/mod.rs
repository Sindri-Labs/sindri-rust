pub mod clone;
pub use clone::clone;

pub mod deploy;
pub use deploy::deploy;

use console::style;

pub fn handle_operation_error(command: &str, message: &str) -> ! {
    eprintln!("{}", style(format!("{} failed ❌", command)).bold());
    eprintln!("{}", style(message).red());
    std::process::exit(1);
}
