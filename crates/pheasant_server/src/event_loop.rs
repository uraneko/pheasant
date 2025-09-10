pub trait EventLoop: Fn(Thread) -> Result<Lock, Error> {}
