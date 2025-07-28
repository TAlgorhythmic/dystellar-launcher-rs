use std::{error::Error, str::from_utf8, sync::LazyLock};

use magic_crypt::{new_magic_crypt, MagicCrypt256, MagicCryptTrait};
use sled::{open, Db, Tree};

use crate::api::{control::dir_provider::get_data_dir, typedef::ms_session::MicrosoftSession};

static KEY: &str = env!("CRYPT_PASS");
static CRYPT: LazyLock<MagicCrypt256> = LazyLock::new(|| new_magic_crypt!(KEY, 256));
static STORAGE: LazyLock<Db> = LazyLock::new(|| {
    open(format!("{}{}storage", get_data_dir().to_str().expect("Failed to get data dir"), std::path::MAIN_SEPARATOR)).expect("Failed to open storage")
});

static SESSION_TREE: LazyLock<Tree> = LazyLock::new(|| STORAGE.open_tree("session").expect("Failed to open session tree"));

pub fn store_session(access_token: &str, refresh_token: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let crypt = &CRYPT;
    let tree = &SESSION_TREE;
    let encrypted_access_token = crypt.encrypt_to_bytes(access_token);
    let encrypted_refresh_token = crypt.encrypt_to_bytes(refresh_token);

    tree.insert(b"access_token", encrypted_access_token)?;
    tree.insert(b"refresh_token", encrypted_refresh_token)?;

    Ok(())
}

pub fn retrieve_session() -> Result<Option<(Box<str>, Box<str>)>, Box<dyn Error + Send + Sync>> {
    let tree = &SESSION_TREE;

    let encrypted_access_token = tree.get(b"access_token")?;
    let encrypted_refresh_token = tree.get(b"refresh_token")?;

    if encrypted_access_token.is_none() || encrypted_refresh_token.is_none() {
        return Ok(None);
    }

    let crypt = &CRYPT;
    let decrypt_access_token: Box<str> = from_utf8(&crypt.decrypt_bytes_to_bytes(&encrypted_access_token.unwrap())?)?.into();
    let decrypt_refresh_token: Box<str> = from_utf8(&crypt.decrypt_bytes_to_bytes(&encrypted_refresh_token.unwrap())?)?.into();

    Ok(Some((decrypt_access_token, decrypt_refresh_token)))
}
