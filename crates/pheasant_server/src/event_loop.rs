use crate::Socket;

pub trait EventLoop {
    /// handles the socket to socket communication
    fn message(&mut self);

    /// contains the logic for the repeating event loop
    /// usually should contain a while, for or loop block
    fn event_loop(&mut self);
}

impl EventLoop for Socket {}
