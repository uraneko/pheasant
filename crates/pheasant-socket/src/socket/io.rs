pub mod recv {
    #[derive(Debug, Clone, Copy)]
    pub enum RecvFlag {
        DontWait = 64,
        ErrorQueue = 8192,
        OutOfBand = 1,
        Peek = 2,
        Truncate = 32,
        WaitAll = 256,
    }

    impl From<RecvFlag> for u32 {
        fn from(flag: RecvFlag) -> u32 {
            match flag {
                RecvFlag::DontWait => 64,
                RecvFlag::ErrorQueue => 8192,
                RecvFlag::OutOfBand => 1,
                RecvFlag::Peek => 2,
                RecvFlag::Truncate => 32,
                RecvFlag::WaitAll => 256,
            }
        }
    }

    impl RecvFlag {
        pub fn union(flags: &[Self]) -> u32 {
            flags
                .into_iter()
                .fold(0u32, |acc, f| acc | <RecvFlag as Into<u32>>::into(*f))
        }
    }

    pub struct RecvFlags(u32);

    impl RecvFlags {
        pub fn new() -> Self {
            Self(0)
        }

        pub fn dont_wait(mut self, dont_wait: bool) -> Self {
            let flag: u32 = RecvFlag::DontWait.into();
            if dont_wait {
                self.0 |= flag;
            } else {
                self.0 ^= flag;
            }

            self
        }

        pub fn error_queue(mut self, error_queue: bool) -> Self {
            let flag: u32 = RecvFlag::ErrorQueue.into();
            if error_queue {
                self.0 |= flag;
            } else {
                self.0 ^= flag;
            }

            self
        }

        pub fn out_of_band(mut self, out_of_band: bool) -> Self {
            let flag: u32 = RecvFlag::OutOfBand.into();
            if out_of_band {
                self.0 |= flag;
            } else {
                self.0 ^= flag;
            }

            self
        }

        pub fn peek(mut self, peek: bool) -> Self {
            let flag: u32 = RecvFlag::Peek.into();
            if peek {
                self.0 |= flag;
            } else {
                self.0 ^= flag;
            }

            self
        }

        pub fn trunc(mut self, trunc: bool) -> Self {
            let flag: u32 = RecvFlag::Truncate.into();
            if trunc {
                self.0 |= flag;
            } else {
                self.0 ^= flag;
            }

            self
        }

        pub fn wait_all(mut self, wait_all: bool) -> Self {
            let flag: u32 = RecvFlag::WaitAll.into();
            if wait_all {
                self.0 |= flag;
            } else {
                self.0 ^= flag;
            }

            self
        }
    }

    impl From<RecvFlags> for i32 {
        fn from(flags: RecvFlags) -> i32 {
            flags.0 as i32
        }
    }
}

pub mod send {
    #[derive(Debug, Clone, Copy)]
    pub enum SendFlag {
        Confirm = 2048,
        DontRoute = 4,
        DontWait = 64,
        EndOfRecord = 128,
        More = 32768,
        NoSignal = 16384,
        OutOfBand = 1,
    }

    impl From<SendFlag> for u32 {
        fn from(flag: SendFlag) -> u32 {
            match flag {
                SendFlag::Confirm => 2048,
                SendFlag::DontRoute => 4,
                SendFlag::DontWait => 64,
                SendFlag::EndOfRecord => 128,
                SendFlag::More => 32768,
                SendFlag::NoSignal => 16384,
                SendFlag::OutOfBand => 1,
            }
        }
    }

    impl SendFlag {
        pub fn union(flags: &[Self]) -> u32 {
            flags
                .into_iter()
                .fold(0u32, |acc, f| acc | <SendFlag as Into<u32>>::into(*f))
        }
    }

    pub struct SendFlags(u32);

    impl SendFlags {
        pub fn new() -> Self {
            Self(0)
        }

        pub fn confirm(mut self, confirm: bool) -> Self {
            let flag: u32 = SendFlag::Confirm.into();
            if confirm {
                self.0 |= flag;
            } else {
                self.0 ^= flag;
            }

            self
        }

        pub fn dont_route(mut self, dont_route: bool) -> Self {
            let flag: u32 = SendFlag::DontRoute.into();
            if dont_route {
                self.0 |= flag;
            } else {
                self.0 ^= flag;
            }

            self
        }

        pub fn out_of_band(mut self, out_of_band: bool) -> Self {
            let flag: u32 = SendFlag::OutOfBand.into();
            if out_of_band {
                self.0 |= flag;
            } else {
                self.0 ^= flag;
            }

            self
        }

        pub fn dont_wait(mut self, dont_wait: bool) -> Self {
            let flag: u32 = SendFlag::DontWait.into();
            if dont_wait {
                self.0 |= flag;
            } else {
                self.0 ^= flag;
            }

            self
        }

        pub fn more(mut self, more: bool) -> Self {
            let flag: u32 = SendFlag::More.into();
            if more {
                self.0 |= flag;
            } else {
                self.0 ^= flag;
            }

            self
        }

        pub fn no_signal(mut self, no_signal: bool) -> Self {
            let flag: u32 = SendFlag::NoSignal.into();
            if no_signal {
                self.0 |= flag;
            } else {
                self.0 ^= flag;
            }

            self
        }
    }

    impl From<SendFlags> for i32 {
        fn from(flags: SendFlags) -> i32 {
            flags.0 as i32
        }
    }
}
