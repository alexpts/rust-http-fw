use std::collections::HashMap;
use std::net::TcpStream;

pub struct EventMap {
    pub key: usize, // sequence event key
    pub map:  HashMap<usize, TcpStream>,
    pub server_key: usize, // tcp server event key
}

impl Iterator for EventMap {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.key += 1;
        return Some(self.key);
    }
}

impl EventMap {
    pub fn create_server_key(&mut self) -> usize {
        self.server_key = self.next().unwrap();
        return self.server_key;
    }
}