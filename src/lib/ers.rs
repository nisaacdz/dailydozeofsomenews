#[derive(Debug)]
pub enum MyErrs {
    CannotFindEnv,
    ConnectionFailed,
    AsyncFailed,
    JsonParseFailed(reqwest::Error),
    UrlParseFailed,
    CannotReadPage,
    BadFeedback,

    //TEMP
    UreqError,
    JsonParseErr,
}