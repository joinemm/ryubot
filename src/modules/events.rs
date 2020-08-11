use log::{error, info};
use serenity::{
    async_trait,
    framework::standard::{macros::hook, CommandError, DispatchError},
    model::channel::Message,
    model::gateway::Ready,
    model::prelude::Permissions,
    prelude::*,
};

pub struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }
}

#[hook]
pub async fn after_hook(ctx: &Context, msg: &Message, _cmd_name: &str, error: Result<(), CommandError>) {
    //  Print out an error if it happened
    if let Err(why) = error {
        error!(
            "Command error [ {}#{} {} ]: {:?}",
            msg.author.name, msg.author.discriminator, msg.content, why
        );
        let _ = msg.channel_id.say(&ctx, format!("```rs\n{:?}\n```", why)).await;
    // no error, just log the command usage
    } else {
        info!(
            "{}#{} used {}",
            msg.author.name, msg.author.discriminator, msg.content
        )
    }
}

#[hook]
pub async fn before_hook(_ctx: &Context, _msg: &Message, cmd_name: &str) -> bool {
    info!("Running command {}", cmd_name);
    true
}

#[hook]
pub async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    match error {
        DispatchError::LackingPermissions(Permissions::ADMINISTRATOR) => {
            let _ = msg
                .channel_id
                .say(
                    ctx,
                    "You need to be an **Administrator** to execute this command!",
                )
                .await;
        }
        DispatchError::LackingPermissions(Permissions::MANAGE_MESSAGES) => {
            let _ = msg
                .channel_id
                .say(
                    ctx,
                    "You require **Manage messages** permission to execute this command!",
                )
                .await;
        }
        DispatchError::NotEnoughArguments { min, given } => {
            let _ = msg
                .channel_id
                .say(
                    ctx,
                    format!(
                        "Not enough arguments! ({}) This command requires at least {}",
                        given, min
                    ),
                )
                .await;
        }
        DispatchError::OnlyForOwners => {
            let _ = msg
                .channel_id
                .say(ctx, "Only the bot owner is able to use this!")
                .await;
        }
        _ => println!("Unhandled dispatch error: {:?}", error),
    }
}
