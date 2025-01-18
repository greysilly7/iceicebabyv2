use chorus::errors::ChorusResult;
use chorus::instance::{ChorusUser, Instance};
use chorus::types::Guild;

#[derive(Debug, Clone)]
pub enum Message {
    Login,
    Register,
    MainView,
    UsernameUpdate(String),
    PasswordUpdate(String),
    InstanceUrlUpdate(String),
    SubmitLogin,
    LoginSuccess(Instance, ChorusUser),
    Logout,
    SwitchChannel(String),
    MessageInputUpdate(String),
    SendMessage,

    GuildsFetched(ChorusResult<Vec<Guild>>),
    FetchGuilds
}
