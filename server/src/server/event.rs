use crate::clients::client::EventType;
use crate::server::data::channel::ChannelCreatedEvent;
use crate::server::data::thread::{ReplyCreatedEvent, ThreadCreatedEvent};
use crate::server::data::user::{UserLoggedInEvent, UserSubscribedEvent, UserUnsubscribedEvent};
use crate::server::server::MyTeamsServer;

macro_rules! impl_from_event {
    ($variant:ident, $type:ty) => {
        impl From<$type> for GlobalEvent {
            fn from(value: $type) -> Self {
                GlobalEvent::$variant(value)
            }
        }
    };
}

#[derive(Clone, Debug)]
enum GlobalEvent {
    UserLoggedIn(UserLoggedInEvent),
    ChannelCreated(ChannelCreatedEvent),
    ThreadCreated(ThreadCreatedEvent),
    ReplyCreated(ReplyCreatedEvent),
    UserSubscribed(UserSubscribedEvent),
    UserUnsubscribed(UserUnsubscribedEvent),
}

impl_from_event!(ChannelCreated, ChannelCreatedEvent);
impl_from_event!(ThreadCreated, ThreadCreatedEvent);
impl_from_event!(ReplyCreated, ReplyCreatedEvent);
impl_from_event!(UserSubscribed, UserSubscribedEvent);
impl_from_event!(UserUnsubscribed, UserUnsubscribedEvent);

impl From<GlobalEvent> for EventType {
    fn from(value: GlobalEvent) -> Self {
        match value {
            GlobalEvent::UserLoggedIn(ctx) => EventType::UserLoggedIn(ctx),
            GlobalEvent::ChannelCreated(ctx) => EventType::ChannelCreated(ctx),
            GlobalEvent::ThreadCreated(ctx) => EventType::ThreadCreated(ctx),
            GlobalEvent::ReplyCreated(ctx) => EventType::ReplyCreated(ctx),
            GlobalEvent::UserSubscribed(ctx) => EventType::UserSubscribed(ctx),
            GlobalEvent::UserUnsubscribed(ctx) => EventType::UserUnsubscribed(ctx),
        }
    }
}

impl TryFrom<EventType> for GlobalEvent {
    type Error = EventType;

    fn try_from(value: EventType) -> Result<Self, Self::Error> {
        match value {
            EventType::ChannelCreated(ctx) => Ok(GlobalEvent::ChannelCreated(ctx)),
            EventType::ThreadCreated(ctx) => Ok(GlobalEvent::ThreadCreated(ctx)),
            EventType::ReplyCreated(ctx) => Ok(GlobalEvent::ReplyCreated(ctx)),
            EventType::UserSubscribed(ctx) => Ok(GlobalEvent::UserSubscribed(ctx)),
            EventType::UserUnsubscribed(ctx) => Ok(GlobalEvent::UserUnsubscribed(ctx)),
            other => Err(other),
        }
    }
}

impl MyTeamsServer {
    fn send_event_to_all_clients(&mut self, event: EventType) {
        for client in &mut self.client_manager.clients {
            if client.uuid.is_some() {
                client.send_event(event.clone())
            }
        }
    }

    fn send_event_to_all_subscribed_clients(
        &mut self,
        global_event: GlobalEvent,
        team_uuid: String,
    ) {
        let Some(team_index) = self.data.find_teams_by_uuid(team_uuid) else {
            println!("Team no longer exists!");
            return;
        };

        for client_uuid in &mut self.data.teams[team_index].subscribed {
            let Some(client_index) = self.client_manager.is_client_logged_in(client_uuid.clone())
            else {
                continue;
            };

            self.client_manager.clients[client_index].send_event(global_event.clone().into());
        }
    }

    pub fn send_global_event(&mut self, event: EventType, team_uuid: Option<String>) {
        if event.is_global() {
            return self.send_event_to_all_clients(event);
        }

        match (GlobalEvent::try_from(event), team_uuid) {
            (Ok(global_event), Some(team_uuid)) => {
                self.send_event_to_all_subscribed_clients(global_event, team_uuid);
            }
            (Err(event), _) => {
                println!("This event is not global! {:?}", event);
            }
            (Ok(event), None) => {
                println!("A global event need a team_uuid! {:?}", event);
            }
        }
    }
}
