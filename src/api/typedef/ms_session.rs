pub struct MicrosoftSession {
    uuid: Box<str>,
    username: Box<str>,
    access_token: Box<str>,
    minecraft_token: Box<str>
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

    pub fn get_minecraft_token(&self) -> &str {
        self.minecraft_token.as_ref()
    }
}
