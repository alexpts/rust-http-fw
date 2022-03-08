use std::collections::HashMap;
use std::io;
use std::io::Error;
use std::net::{TcpListener, TcpStream};

use polling::{Event, Poller};

use super::event_map::EventMap;

pub struct Server {
    poller: Poller,
    listener: Option<TcpListener>,
    incoming_handler: fn(&TcpStream, &mut [u8; 1024]) -> Result<usize, io::Error>,
    event_map: EventMap,
}

pub fn new(incoming_handler: fn(&TcpStream, &mut [u8; 1024]) -> Result<usize, Error>) -> Server {
    return Server {
        event_map: EventMap {
            key: 4,
            server_key: 4,
            map: HashMap::new(),
        },
        poller: Poller::new().unwrap(),
        listener: None,
        incoming_handler,
    };
}

impl Server {
    pub fn bind(&mut self, address: &str) {
        let listener = TcpListener::bind(address).unwrap();
        listener.set_nonblocking(true).unwrap();

        let key = self.event_map.create_server_key();
        self.poller.add(&listener, Event::readable(key)).unwrap();
        self.listener = Some(listener);
    }

    fn accept(&mut self) {
        let listener: &TcpListener = self.listener.as_ref().unwrap();
        let (client, _address) = listener.accept().unwrap();

        client.set_nonblocking(true).unwrap();

        self.poller
            .modify(listener, Event::readable(self.event_map.server_key)).unwrap(); // re-event

        let event_id = self.event_map.next().unwrap();
        self.poller
            .add(&client, Event::readable(event_id))
            .expect("Can`t` add socket to epoll");
        self.event_map.map.insert(event_id, client);
    }

    fn incoming_message(&self, event: &Event) {
        let socket = self.event_map.map.get(&event.key).unwrap();
        let mut buffer = [0_u8; 1024];

        if (self.incoming_handler)(socket, &mut buffer).is_ok() {
            self.poller
                .modify(socket, Event::readable(event.key))
                .expect("Can`t` add socket to epoll");
        }
    }

    pub fn start(&mut self) {
        let mut events = Vec::new();

        loop {
            self.poller.wait(&mut events, None).unwrap();

            for ev in &events {
                if ev.key == self.event_map.server_key {
                    self.accept();
                    continue;
                }

                self.incoming_message(ev);
            }

            // Wait for at least one I/O event.
            events.clear();
        }
    }
}