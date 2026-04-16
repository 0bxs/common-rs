use reqwest::Client;
use std::sync::OnceLock;

static CLIENT: OnceLock<Client> = OnceLock::new();

pub fn init() {
    CLIENT.set(Client::new()).unwrap();
}

pub fn client() -> &'static Client {
    CLIENT.get().unwrap()
}
