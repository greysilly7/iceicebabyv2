use chorus::instance::{ChorusUser, Instance};
use chorus::types::{GatewayReady, Guild, MessageCreate, Snowflake};

#[derive(Debug, Clone)]
pub enum Message {
    Login,
    Register,
    MainView,
    UsernameUpdate(String),
    PasswordUpdate(String),
    InstanceUrlUpdate(String),
    SubmitLogin,
    LoginSuccess(Instance, ChorusUser, Option<Vec<Guild>>),
    LogoutCleanup,
    Logout,
    SwitchGuild(Snowflake),
    SwitchChannel(Snowflake),
    MessageInputUpdate(String),
    SendMessage,
    MessageSent,
    UpdateGuilds(Vec<Guild>),
    UpdateChannels(Vec<Snowflake>),
    UpdateMessages(Vec<chorus::types::Message>),
    ReadyReceived(GatewayReady),
    MessageCreateReceived(MessageCreate),
}