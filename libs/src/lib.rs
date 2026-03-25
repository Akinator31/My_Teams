use std::ffi::CString;

mod bindings;

pub struct ClientLog;

impl ClientLog {
    fn to_cstr(s: String) -> CString {
        CString::new(s).expect("CString::new failed")
    }

    pub fn client_event_logged_in(user_uuid: String, user_name: String) -> i32 {
        let user_uuid_cstr = Self::to_cstr(user_uuid);
        let user_name_cstr = Self::to_cstr(user_name);
        unsafe { bindings::client_event_logged_in(user_uuid_cstr.as_ptr(), user_name_cstr.as_ptr()) }
    }

    pub fn client_event_logged_out(user_uuid: String, user_name: String) -> i32 {
        let user_uuid_cstr = Self::to_cstr(user_uuid);
        let user_name_cstr = Self::to_cstr(user_name);
        unsafe { bindings::client_event_logged_out(user_uuid_cstr.as_ptr(), user_name_cstr.as_ptr()) }
    }

    pub fn client_event_private_message_received(user_uuid: String, message_body: String) -> i32 {
        let user_uuid_cstr = Self::to_cstr(user_uuid);
        let message_body_cstr = Self::to_cstr(message_body);
        unsafe { bindings::client_event_private_message_received(user_uuid_cstr.as_ptr(), message_body_cstr.as_ptr()) }
    }

    pub fn client_event_thread_reply_received(team_uuid: String, thread_uuid: String, user_uuid: String, reply_body: String) -> i32 {
        let team_uuid_cstr = Self::to_cstr(team_uuid);
        let thread_uuid_cstr = Self::to_cstr(thread_uuid);
        let user_uuid_cstr = Self::to_cstr(user_uuid);
        let reply_body_cstr = Self::to_cstr(reply_body);
        unsafe { bindings::client_event_thread_reply_received(team_uuid_cstr.as_ptr(), thread_uuid_cstr.as_ptr(), user_uuid_cstr.as_ptr(), reply_body_cstr.as_ptr()) }
    }

    pub fn client_event_team_created(team_uuid: String, team_name: String, team_description: String) -> i32 {
        let team_uuid_cstr = Self::to_cstr(team_uuid);
        let team_name_cstr = Self::to_cstr(team_name);
        let team_description_cstr = Self::to_cstr(team_description);
        unsafe { bindings::client_event_team_created(team_uuid_cstr.as_ptr(), team_name_cstr.as_ptr(), team_description_cstr.as_ptr()) }
    }

    pub fn client_event_channel_created(channel_uuid: String, channel_name: String, channel_description: String) -> i32 {
        let channel_uuid_cstr = Self::to_cstr(channel_uuid);
        let channel_name_cstr = Self::to_cstr(channel_name);
        let channel_description_cstr = Self::to_cstr(channel_description);
        unsafe { bindings::client_event_channel_created(channel_uuid_cstr.as_ptr(), channel_name_cstr.as_ptr(), channel_description_cstr.as_ptr()) }
    }

    pub fn client_event_thread_created(thread_uuid: String, user_uuid: String, thread_timestamp: i64, thread_title: String, thread_body: String) -> i32 {
        let thread_uuid_cstr = Self::to_cstr(thread_uuid);
        let user_uuid_cstr = Self::to_cstr(user_uuid);
        let thread_title_cstr = Self::to_cstr(thread_title);
        let thread_body_cstr = Self::to_cstr(thread_body);
        unsafe { bindings::client_event_thread_created(thread_uuid_cstr.as_ptr(), user_uuid_cstr.as_ptr(), thread_timestamp, thread_title_cstr.as_ptr(), thread_body_cstr.as_ptr()) }
    }

    pub fn client_print_users(user_uuid: String, user_name: String, user_status: i32) -> i32 {
        let user_uuid_cstr = Self::to_cstr(user_uuid);
        let user_name_cstr = Self::to_cstr(user_name);
        unsafe { bindings::client_print_users(user_uuid_cstr.as_ptr(), user_name_cstr.as_ptr(), user_status) }
    }

    pub fn client_print_teams(team_uuid: String, team_name: String, team_description: String) -> i32 {
        let team_uuid_cstr = Self::to_cstr(team_uuid);
        let team_name_cstr = Self::to_cstr(team_name);
        let team_description_cstr = Self::to_cstr(team_description);
        unsafe { bindings::client_print_teams(team_uuid_cstr.as_ptr(), team_name_cstr.as_ptr(), team_description_cstr.as_ptr()) }
    }

    pub fn client_team_print_channels(channel_uuid: String, channel_name: String, channel_description: String) -> i32 {
        let channel_uuid_cstr = Self::to_cstr(channel_uuid);
        let channel_name_cstr = Self::to_cstr(channel_name);
        let channel_description_cstr = Self::to_cstr(channel_description);
        unsafe { bindings::client_team_print_channels(channel_uuid_cstr.as_ptr(), channel_name_cstr.as_ptr(), channel_description_cstr.as_ptr()) }
    }

    pub fn client_channel_print_threads(thread_uuid: String, user_uuid: String, thread_timestamp: i64, thread_title: String, thread_body: String) -> i32 {
        let thread_uuid_cstr = Self::to_cstr(thread_uuid);
        let user_uuid_cstr = Self::to_cstr(user_uuid);
        let thread_title_cstr = Self::to_cstr(thread_title);
        let thread_body_cstr = Self::to_cstr(thread_body);
        unsafe { bindings::client_channel_print_threads(thread_uuid_cstr.as_ptr(), user_uuid_cstr.as_ptr(), thread_timestamp, thread_title_cstr.as_ptr(), thread_body_cstr.as_ptr()) }
    }

    pub fn client_thread_print_replies(thread_uuid: String, user_uuid: String, reply_timestamp: i64, reply_body: String) -> i32 {
        let thread_uuid_cstr = Self::to_cstr(thread_uuid);
        let user_uuid_cstr = Self::to_cstr(user_uuid);
        let reply_body_cstr = Self::to_cstr(reply_body);
        unsafe { bindings::client_thread_print_replies(thread_uuid_cstr.as_ptr(), user_uuid_cstr.as_ptr(), reply_timestamp, reply_body_cstr.as_ptr()) }
    }

    pub fn client_private_message_print_messages(sender_uuid: String, message_timestamp: i64, message_body: String) -> i32 {
        let sender_uuid_cstr = Self::to_cstr(sender_uuid);
        let message_body_cstr = Self::to_cstr(message_body);
        unsafe { bindings::client_private_message_print_messages(sender_uuid_cstr.as_ptr(), message_timestamp, message_body_cstr.as_ptr()) }
    }

    pub fn client_error_unknown_team(team_uuid: String) -> i32 {
        let team_uuid_cstr = Self::to_cstr(team_uuid);
        unsafe { bindings::client_error_unknown_team(team_uuid_cstr.as_ptr()) }
    }

    pub fn client_error_unknown_channel(channel_uuid: String) -> i32 {
        let channel_uuid_cstr = Self::to_cstr(channel_uuid);
        unsafe { bindings::client_error_unknown_channel(channel_uuid_cstr.as_ptr()) }
    }

    pub fn client_error_unknown_thread(thread_uuid: String) -> i32 {
        let thread_uuid_cstr = Self::to_cstr(thread_uuid);
        unsafe { bindings::client_error_unknown_thread(thread_uuid_cstr.as_ptr()) }
    }

    pub fn client_error_unknown_user(user_uuid: String) -> i32 {
        let user_uuid_cstr = Self::to_cstr(user_uuid);
        unsafe { bindings::client_error_unknown_user(user_uuid_cstr.as_ptr()) }
    }

    pub fn client_error_unauthorized() -> i32 {
        unsafe { bindings::client_error_unauthorized() }
    }

    pub fn client_error_already_exist() -> i32 {
        unsafe { bindings::client_error_already_exist() }
    }

    pub fn client_print_user(user_uuid: String, user_name: String, user_status: i32) -> i32 {
        let user_uuid_cstr = Self::to_cstr(user_uuid);
        let user_name_cstr = Self::to_cstr(user_name);
        unsafe { bindings::client_print_user(user_uuid_cstr.as_ptr(), user_name_cstr.as_ptr(), user_status) }
    }

    pub fn client_print_team(team_uuid: String, team_name: String, team_description: String) -> i32 {
        let team_uuid_cstr = Self::to_cstr(team_uuid);
        let team_name_cstr = Self::to_cstr(team_name);
        let team_description_cstr = Self::to_cstr(team_description);
        unsafe { bindings::client_print_team(team_uuid_cstr.as_ptr(), team_name_cstr.as_ptr(), team_description_cstr.as_ptr()) }
    }

    pub fn client_print_channel(channel_uuid: String, channel_name: String, channel_description: String) -> i32 {
        let channel_uuid_cstr = Self::to_cstr(channel_uuid);
        let channel_name_cstr = Self::to_cstr(channel_name);
        let channel_description_cstr = Self::to_cstr(channel_description);
        unsafe { bindings::client_print_channel(channel_uuid_cstr.as_ptr(), channel_name_cstr.as_ptr(), channel_description_cstr.as_ptr()) }
    }

    pub fn client_print_thread(thread_uuid: String, user_uuid: String, thread_timestamp: i64, thread_title: String, thread_body: String) -> i32 {
        let thread_uuid_cstr = Self::to_cstr(thread_uuid);
        let user_uuid_cstr = Self::to_cstr(user_uuid);
        let thread_title_cstr = Self::to_cstr(thread_title);
        let thread_body_cstr = Self::to_cstr(thread_body);
        unsafe { bindings::client_print_thread(thread_uuid_cstr.as_ptr(), user_uuid_cstr.as_ptr(), thread_timestamp, thread_title_cstr.as_ptr(), thread_body_cstr.as_ptr()) }
    }

    pub fn client_print_team_created(team_uuid: String, team_name: String, team_description: String) -> i32 {
        let team_uuid_cstr = Self::to_cstr(team_uuid);
        let team_name_cstr = Self::to_cstr(team_name);
        let team_description_cstr = Self::to_cstr(team_description);
        unsafe { bindings::client_print_team_created(team_uuid_cstr.as_ptr(), team_name_cstr.as_ptr(), team_description_cstr.as_ptr()) }
    }

    pub fn client_print_channel_created(channel_uuid: String, channel_name: String, channel_description: String) -> i32 {
        let channel_uuid_cstr = Self::to_cstr(channel_uuid);
        let channel_name_cstr = Self::to_cstr(channel_name);
        let channel_description_cstr = Self::to_cstr(channel_description);
        unsafe { bindings::client_print_channel_created(channel_uuid_cstr.as_ptr(), channel_name_cstr.as_ptr(), channel_description_cstr.as_ptr()) }
    }

    pub fn client_print_thread_created(thread_uuid: String, user_uuid: String, thread_timestamp: i64, thread_title: String, thread_body: String) -> i32 {
        let thread_uuid_cstr = Self::to_cstr(thread_uuid);
        let user_uuid_cstr = Self::to_cstr(user_uuid);
        let thread_title_cstr = Self::to_cstr(thread_title);
        let thread_body_cstr = Self::to_cstr(thread_body);
        unsafe { bindings::client_print_thread_created(thread_uuid_cstr.as_ptr(), user_uuid_cstr.as_ptr(), thread_timestamp, thread_title_cstr.as_ptr(), thread_body_cstr.as_ptr()) }
    }

    pub fn client_print_reply_created(thread_uuid: String, user_uuid: String, reply_timestamp: i64, reply_body: String) -> i32 {
        let thread_uuid_cstr = Self::to_cstr(thread_uuid);
        let user_uuid_cstr = Self::to_cstr(user_uuid);
        let reply_body_cstr = Self::to_cstr(reply_body);
        unsafe { bindings::client_print_reply_created(thread_uuid_cstr.as_ptr(), user_uuid_cstr.as_ptr(), reply_timestamp, reply_body_cstr.as_ptr()) }
    }

    pub fn client_print_subscribed(user_uuid: String, team_uuid: String) -> i32 {
        let user_uuid_cstr = Self::to_cstr(user_uuid);
        let team_uuid_cstr = Self::to_cstr(team_uuid);
        unsafe { bindings::client_print_subscribed(user_uuid_cstr.as_ptr(), team_uuid_cstr.as_ptr()) }
    }

    pub fn client_print_unsubscribed(user_uuid: String, team_uuid: String) -> i32 {
        let user_uuid_cstr = Self::to_cstr(user_uuid);
        let team_uuid_cstr = Self::to_cstr(team_uuid);
        unsafe { bindings::client_print_unsubscribed(user_uuid_cstr.as_ptr(), team_uuid_cstr.as_ptr()) }
    }
}

pub struct ServerLog;

impl ServerLog {
    fn to_cstr(s: String) -> CString {
        CString::new(s).expect("CString::new failed")
    }

    pub fn server_event_team_created(team_uuid: String, team_name: String, user_uuid: String) -> i32 {
        let team_uuid_cstr = Self::to_cstr(team_uuid);
        let team_name_cstr = Self::to_cstr(team_name);
        let user_uuid_cstr = Self::to_cstr(user_uuid);
        unsafe { bindings::server_event_team_created(team_uuid_cstr.as_ptr(), team_name_cstr.as_ptr(), user_uuid_cstr.as_ptr()) }
    }

    pub fn server_event_channel_created(team_uuid: String, channel_uuid: String, channel_name: String) -> i32 {
        let team_uuid_cstr = Self::to_cstr(team_uuid);
        let channel_uuid_cstr = Self::to_cstr(channel_uuid);
        let channel_name_cstr = Self::to_cstr(channel_name);
        unsafe { bindings::server_event_channel_created(team_uuid_cstr.as_ptr(), channel_uuid_cstr.as_ptr(), channel_name_cstr.as_ptr()) }
    }

    pub fn server_event_thread_created(channel_uuid: String, thread_uuid: String, user_uuid: String, thread_title: String, thread_body: String) -> i32 {
        let channel_uuid_cstr = Self::to_cstr(channel_uuid);
        let thread_uuid_cstr = Self::to_cstr(thread_uuid);
        let user_uuid_cstr = Self::to_cstr(user_uuid);
        let thread_title_cstr = Self::to_cstr(thread_title);
        let thread_body_cstr = Self::to_cstr(thread_body);
        unsafe { bindings::server_event_thread_created(channel_uuid_cstr.as_ptr(), thread_uuid_cstr.as_ptr(), user_uuid_cstr.as_ptr(), thread_title_cstr.as_ptr(), thread_body_cstr.as_ptr()) }
    }

    pub fn server_event_reply_created(thread_uuid: String, user_uuid: String, reply_body: String) -> i32 {
        let thread_uuid_cstr = Self::to_cstr(thread_uuid);
        let user_uuid_cstr = Self::to_cstr(user_uuid);
        let reply_body_cstr = Self::to_cstr(reply_body);
        unsafe { bindings::server_event_reply_created(thread_uuid_cstr.as_ptr(), user_uuid_cstr.as_ptr(), reply_body_cstr.as_ptr()) }
    }

    pub fn server_event_user_subscribed(team_uuid: String, user_uuid: String) -> i32 {
        let team_uuid_cstr = Self::to_cstr(team_uuid);
        let user_uuid_cstr = Self::to_cstr(user_uuid);
        unsafe { bindings::server_event_user_subscribed(team_uuid_cstr.as_ptr(), user_uuid_cstr.as_ptr()) }
    }

    pub fn server_event_user_unsubscribed(team_uuid: String, user_uuid: String) -> i32 {
        let team_uuid_cstr = Self::to_cstr(team_uuid);
        let user_uuid_cstr = Self::to_cstr(user_uuid);
        unsafe { bindings::server_event_user_unsubscribed(team_uuid_cstr.as_ptr(), user_uuid_cstr.as_ptr()) }
    }

    pub fn server_event_user_created(user_uuid: String, user_name: String) -> i32 {
        let user_uuid_cstr = Self::to_cstr(user_uuid);
        let user_name_cstr = Self::to_cstr(user_name);
        unsafe { bindings::server_event_user_created(user_uuid_cstr.as_ptr(), user_name_cstr.as_ptr()) }
    }

    pub fn server_event_user_loaded(user_uuid: String, user_name: String) -> i32 {
        let user_uuid_cstr = Self::to_cstr(user_uuid);
        let user_name_cstr = Self::to_cstr(user_name);
        unsafe { bindings::server_event_user_loaded(user_uuid_cstr.as_ptr(), user_name_cstr.as_ptr()) }
    }

    pub fn server_event_user_logged_in(user_uuid: String) -> i32 {
        let user_uuid_cstr = Self::to_cstr(user_uuid);
        unsafe { bindings::server_event_user_logged_in(user_uuid_cstr.as_ptr()) }
    }

    pub fn server_event_user_logged_out(user_uuid: String) -> i32 {
        let user_uuid_cstr = Self::to_cstr(user_uuid);
        unsafe { bindings::server_event_user_logged_out(user_uuid_cstr.as_ptr()) }
    }

    pub fn server_event_private_message_sended(sender_uuid: String, receiver_uuid: String, message_body: String) -> i32 {
        let sender_uuid_cstr = Self::to_cstr(sender_uuid);
        let receiver_uuid_cstr = Self::to_cstr(receiver_uuid);
        let message_body_cstr = Self::to_cstr(message_body);
        unsafe { bindings::server_event_private_message_sended(sender_uuid_cstr.as_ptr(), receiver_uuid_cstr.as_ptr(), message_body_cstr.as_ptr()) }
    }
}
