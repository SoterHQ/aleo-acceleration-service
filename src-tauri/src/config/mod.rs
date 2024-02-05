pub mod consts;

use std::{
    env,
    sync::{Arc, Mutex},
};

use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use sha2::{Digest, Sha256};

use crate::tls;

lazy_static! {
    static ref CONFIG: Mutex<Config> = Mutex::new(Config::new());
    pub static ref PROGRAM_ID: Mutex<String> = Mutex::new("".to_string());
}

#[tauri::command]
pub async fn input_password(password: String) -> Result<(), String> {
    let mut config = CONFIG.lock().unwrap();
    config
        .decrypt_config(password.as_str())
        .context("wrong password")
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn set_password(password: String) -> Result<(), String> {
    let mut config = CONFIG.lock().unwrap();

    config
        .set_password(password.as_str())
        .context("failed to set password")
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn has_password() -> Result<bool, String> {
    let config = CONFIG.lock().unwrap();

    config.has_password().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn try_password() -> Result<bool, String> {
    let mut config = CONFIG.lock().unwrap();
    if config.password.is_some() {
        return Ok(true);
    }
    Ok(config.decrypt_config("").is_ok())
}

#[tauri::command]
pub fn set_proxy(proxy: String) {
    if let Ok(()) = Config::get_config().set_proxy(&proxy) {
        set_proxy_env(&proxy)
    }
}

#[tauri::command]
pub fn get_proxy() -> Result<Option<String>, String> {
    let proxy = Config::get_config()
        .get_proxy()
        .map_err(|e| e.to_string())?;
    Ok(proxy)
}

#[tauri::command]
pub async fn test_proxy(proxy: String) -> Result<(), String> {
    let mut easy = curl::easy::Easy::new();
    easy.proxy(&proxy)
        .map_err(|e| format!("failed to set proxy: {}", e))?;
    _ = easy.url("https://cp.cloudflare.com");

    easy.perform()
        .map_err(|e| format!("failed to perform request: {}", e))?;
    return Ok(());
}

#[derive(Clone)]
pub struct Config {
    pub db: Option<Arc<rocksdb::DB>>,
    pub password: Option<secure_string::SecureString>,
}

const PASSWORD_TEST: &str = "hello world!";
const PASSWORD_TEST_KEY: &str = "password_test";
impl Config {
    pub fn new() -> Self {
        let db = match Self::create_db() {
            Ok(v) => v,
            Err(e) => {
                log::error!("cant create config database: {:#?}", e);
                return Self {
                    db: None,
                    password: None,
                };
            }
        };
        Self {
            db: Some(Arc::new(db)),
            password: None,
        }
    }

    pub fn decrypt_config(&mut self, password: &str) -> Result<()> {
        let db = self.db.clone().context("cant get db")?;
        let pass_test = db.get(PASSWORD_TEST_KEY)?.context("password not set")?;
        match password {
            "" => {
                if pass_test != PASSWORD_TEST.as_bytes() {
                    return Err(anyhow!("password is wrong"));
                }
            }
            _ => {
                let encrypt_key = hash(password);
                let decrypted =
                    tls::aes::aes_decode(&encrypt_key, &pass_test).context("cant decode db")?;
                if decrypted != PASSWORD_TEST.as_bytes() {
                    return Err(anyhow!("password is wrong"));
                }
            }
        };
        self.password = Some(secure_string::SecureString::from(password.to_string()));
        Ok(())
    }

    pub fn set_password(&mut self, password: &str) -> Result<()> {
        let db = self.db.clone().context("cant get db")?;
        self.password = Some(secure_string::SecureString::from(password.to_string()));
        self.create_secret_key().context("create server secret")?;
        match password {
            "" => {
                db.put(PASSWORD_TEST_KEY, PASSWORD_TEST)?;
            }
            _ => {
                let encrypt_key = hash(password);
                let encrypted = tls::aes::aes_encode(&encrypt_key, PASSWORD_TEST.as_bytes())?;
                db.put(PASSWORD_TEST_KEY, &encrypted)?;
            }
        };

        Ok(())
    }

    pub fn has_password(&self) -> Result<bool> {
        let db = self.db.clone().context("cant get db")?;
        Ok(db.key_may_exist(PASSWORD_TEST_KEY))
    }

    pub fn get_config() -> Config {
        let config = CONFIG.lock().unwrap();
        config.clone()
    }

    fn create_db() -> Result<rocksdb::DB> {
        let dbdir = dirs_next::data_dir()
            .context("cant find data_dir!")?
            .join("aleo-acc-service");
        let mut opts = rocksdb::Options::default();
        opts.create_if_missing(true);
        let db = rocksdb::DB::open(&opts, dbdir).context("cant open db")?;
        Ok(db)
    }

    pub fn create_secret_key(&self) -> Result<Vec<u8>> {
        let db = self.db.clone().context("cant get db")?;
        let secret_key = tls::generate_p256_secret().context("generate secret")?;

        match &self.password {
            Some(v) => {
                if v.unsecure() == "" {
                    db.put("secret_key", &secret_key)
                        .context("cant write to db")?;
                } else {
                    let encrypt_key = hash(v.unsecure());
                    let encrypted = tls::aes::aes_encode(&encrypt_key, &secret_key)?;
                    db.put("secret_key", &encrypted)
                        .context("cant write to db")?;
                }
            }

            None => {
                return Err(anyhow!("no password set!"));
            }
        }

        Ok(secret_key)
    }

    pub fn get_secret_key(&self) -> Result<Vec<u8>> {
        let db = self.db.clone().context("cant get db")?;
        let value = db
            .get("secret_key")
            .context("cant read db")?
            .context("secret not set!")?;

        match self
            .password
            .clone()
            .context("database not decrypted")?
            .unsecure()
        {
            "" => return Ok(value),
            _ => {
                let encrypt_key = hash(self.password.clone().context("no password")?.unsecure());
                let decrypted = tls::aes::aes_decode(&encrypt_key, &value)?;
                return Ok(decrypted);
            }
        }
    }

    pub fn set_proxy(&self, proxy: &str) -> Result<()> {
        let db = self.db.clone().context("cant get db")?;
        if proxy.is_empty() {
            return db.delete("proxy").context("cant delete proxy");
        }
        db.put("proxy", proxy).context("cant write to db")
    }

    pub fn get_proxy(&self) -> Result<Option<String>> {
        let db = self.db.clone().context("cant get db")?;
        match db.get("proxy").context("cant read db")? {
            Some(v) => Ok(Some(String::from_utf8(v).unwrap_or_default())),
            None => Ok(None),
        }
    }
}

pub fn hash(str: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(str);
    let result = hasher.finalize();
    result.to_vec()
}

pub async fn init() -> Result<()> {
    if let Ok(proxy) = Config::get_config().get_proxy() {
        if proxy.is_some() {
            set_proxy_env(proxy.unwrap().as_str());
        }
    }
    Ok(())
}

fn set_proxy_env(proxy: &str) {
    env::set_var("http_proxy", &proxy);
    env::set_var("https_proxy", &proxy);
    env::set_var("all_proxy", &proxy);
}

#[cfg(test)]
mod test {
    use std::{env, time::Duration};

    use tokio::time::sleep;

    use super::set_proxy_env;

    #[test]
    fn test_tokio_env() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let h = rt.spawn(async {
            sleep(Duration::from_secs(1)).await;
            let envvar = env::var("http_proxy").unwrap();
            println!("env: {}", envvar);
            assert_eq!(envvar, "&proxy");
        });
        env::set_var("http_proxy", "&proxy");

        rt.block_on(h).unwrap();
    }

    #[test]
    fn test_proxy() {
        let proxy = "http://127.0.0.1:7890";
        set_proxy_env(proxy);
        let mut easy = curl::easy::Easy::new();
        _ = easy.url("https://www.google.com/generate_204");

        easy.perform()
            .map_err(|e| format!("failed to perform request: {}", e))
            .unwrap();
    }
}
