use crate::utils::parsing::parse_quoted_args;

pub mod commands;
pub mod create;
pub mod info;
pub mod list;
pub mod login;
pub mod logout;
pub mod messages;
pub mod send;
pub mod set_context;
pub mod subscribe;
pub mod subscribed;
pub mod unsubscribe;
pub mod user;
pub mod users;

pub trait FromArgs: Sized {
    const COUNT: usize;
    fn from_args(args: Vec<String>) -> Option<Self>;
}

impl FromArgs for () {
    const COUNT: usize = 0;
    fn from_args(args: Vec<String>) -> Option<Self> {
        if args.len() == Self::COUNT {
            Some(())
        } else {
            None
        }
    }
}

impl FromArgs for String {
    const COUNT: usize = 1;
    fn from_args(mut args: Vec<String>) -> Option<Self> {
        if args.len() == Self::COUNT {
            Some(args.remove(0))
        } else {
            None
        }
    }
}

impl FromArgs for (String, String) {
    const COUNT: usize = 2;
    fn from_args(mut args: Vec<String>) -> Option<Self> {
        if args.len() == Self::COUNT {
            Some((args.remove(0), args.remove(0)))
        } else {
            None
        }
    }
}

impl FromArgs for (String, String, String) {
    const COUNT: usize = 3;
    fn from_args(mut args: Vec<String>) -> Option<Self> {
        if args.len() == Self::COUNT {
            Some((args.remove(0), args.remove(0), args.remove(0)))
        } else {
            None
        }
    }
}

pub fn check_command_format<T: FromArgs>(command_args: &str) -> Option<T> {
    let args = parse_quoted_args(command_args);

    T::from_args(args)
}
