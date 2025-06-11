pub type Result<T> = core::result::Result<T, Error>;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

mod consts {
    pub const MODEL: &str = "llama3.2:1b";

    pub const DEFAULT_SYSTEM: &str =
        "Make every answer concise and to the point. Speak like you are a japanese yakuza pirate.
    ";
}
