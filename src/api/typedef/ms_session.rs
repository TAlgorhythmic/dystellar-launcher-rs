#[derive(Debug)]
pub struct MicrosoftSession {
    pub uuid: Box<str>,
    pub username: Box<str>,
    pub access_token: Box<str>,
    pub refresh_token: Box<str>,
    pub minecraft_token: Box<str>
}

impl MicrosoftSession {
    pub fn get_uuid(&self) -> &str {
        self.uuid.as_ref()
    }

    pub fn get_username(&self) -> &str {
        self.username.as_ref()
    }

    pub fn get_access_token(&self) -> &str {
        self.access_token.as_ref()
    }

    pub fn get_refresh_token(&self) -> &str {
        self.refresh_token.as_ref()
    }

    pub fn get_minecraft_token(&self) -> &str {
        self.minecraft_token.as_ref()
    }
}

#[derive(Debug)]
pub struct ErrorData {
    pub title: &'static str,
    pub description: Box<str>
}
