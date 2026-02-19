use std::sync::OnceLock;
use reqwest::Client;

static CLIENT: OnceLock<Client> = OnceLock::new();

pub fn init() {
    CLIENT.set(Client::new()).unwrap();
}

pub fn client() -> &'static Client {
    CLIENT.get().unwrap()
}