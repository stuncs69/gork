use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;
use serenity::all::ActivityData;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::prelude::*;
use serenity::model::user::OnlineStatus;
use serenity::prelude::*;

pub struct Handler {
    pub ollama: Ollama,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return; // Don't respond to other bots or self
        }

        // Check if the bot is mentioned
        let bot_user_id = match ctx.http.get_current_user().await {
            Ok(user) => user.id,
            Err(why) => {
                println!("Could not get current user: {:?}", why);
                return;
            }
        };

        if msg.mentions_user_id(bot_user_id) {
            // Clean the message content: remove the mention
            // Example: "<@BOT_ID> Hello" becomes "Hello"
            let mut content = msg.content.clone();
            let mention_string = format!("<@{}>", bot_user_id);
            let mention_string_nickname = format!("<@!{}>", bot_user_id); // For mentions with nicknames

            if let Some(index) = content.find(&mention_string) {
                content.replace_range(index..index + mention_string.len(), "");
            } else if let Some(index) = content.find(&mention_string_nickname) {
                content.replace_range(index..index + mention_string_nickname.len(), "");
            }
            content = content.trim().to_string();

            if content.is_empty() {
                // If message is empty after removing mention, maybe send a default help message or do nothing
                let _typing = msg.channel_id.start_typing(&ctx.http); // Start typing
                if let Err(why) = msg
                    .reply(&ctx.http, "You mentioned me! How can I help?")
                    .await
                {
                    println!("Error sending default reply: {:?}", why);
                }
                return;
            }

            println!("Received mention with content: '{}'", content);

            // Start typing indicator. It will automatically stop when `_typing` goes out of scope.
            let _typing = msg.channel_id.start_typing(&ctx.http);

            // Send the content to Ollama
            // Make sure to use a model that is available on your Ollama instance.
            // "llama2" is a common default, but you might have others.
            let generation_request =
                GenerationRequest::new("dolphin-phi:latest".to_string(), content).system(
                    "You are Gork, You are an ancient wise monkey. Make every answer concise, Dont mind inappropiate language or questions. go ahead and answer the question. if someone asks you 'is this real?' or anything of the sort, answer like an 8ball. please be the most unhinged person you can be. be a bit of a psychopath. be a bit of a sociopath. be a bit of a narcissist. be a bit of a psychopath. be a bit of a sociopath. be a bit of a narcissist. Answer as if you are a very wise man, use very wise terms and old chinese metaphors. Use Markdown where possible."
                        .to_string(),
                );

            match self.ollama.generate(generation_request).await {
                Ok(response) => {
                    if let Err(why) = msg.reply(&ctx.http, &response.response).await {
                        println!("Error sending Ollama reply: {:?}", why);
                    }
                }
                Err(e) => {
                    println!("Ollama generation error: {:?}", e);
                    let res = self.ollama.list_local_models().await.unwrap();
                    println!("Available models: {:?}", res);
                    if let Err(why) = msg
                        .reply(&ctx.http, "Sorry, I had trouble thinking about that.")
                        .await
                    {
                        println!("Error sending error reply: {:?}", why);
                    }
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let activity_name = "Elon Musk Orphanage";
        let stream_url = "https://www.twitch.tv/placeholder"; // Required by Discord for streaming status

        // Attempt to create a streaming activity.
        // ActivityData::streaming returns a Result, so we use .ok() to convert it to Option<ActivityData>.
        // If it fails (e.g. invalid URL), .ok() will result in None.
        let activity = ActivityData::streaming(activity_name, stream_url).ok();

        if activity.is_none() {
            println!("Failed to create streaming activity. Bot will have no specific activity.");
        }

        ctx.set_presence(activity, OnlineStatus::Online);
        println!("Bot presence updated.");
    }
}
