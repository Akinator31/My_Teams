use crate::client::io_manager::IoManager;
use crate::utils::help::show_help;

pub fn help_cmd(_io_manager: &mut IoManager, _args: &str) {
    show_help();
}
