use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};
use super::is_in_incorrect_channel;

#[command]
#[aliases("paste", "code")]
#[description("Teches users how to paste code neatly")]
#[usage("")]
#[example = ""]
#[help_available]
#[bucket = "potentially_big_output"]
pub(crate) async fn codeblock(ctx: &Context, msg: &Message) -> CommandResult {
    if is_in_incorrect_channel(ctx,msg).await {
        return Ok(());
    }
    msg.channel_id
        .say(
            &ctx.http,
            "**Use Codeblocks To Paste Your Code**

              to do a codeblock, simply do 

              \``` scripting language 
                  //your code here
              \```",
        )
        .await?;
    Ok(())
}
#[command]
#[aliases("NO!", "BROKEN", "PLEASE_FIX")]
#[description("Links the site to place bug reports")]
#[usage("")]
#[example = ""]
#[help_available]
#[bucket = "potentially_big_output"]
