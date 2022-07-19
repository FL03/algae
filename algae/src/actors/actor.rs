/*
   Appellation: actor <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/

type ActorError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub trait TActor {
    type Agent;
    type Config;
    type Context;
    type Data;

    fn configure(settings: Self::Config) -> Result<Self::Config, config::ConfigError>;
    fn constructor(&self) -> Result<Self, ActorError>
    where
        Self: Sized;
}
