pub mod socket;

#[repr(C)]
pub struct linger {
    // bool active or not
    l_onoff: i32,
    // in seconds
    l_linger: i32,
}

impl linger {
    /// duration is in seconds
    pub fn new(active: bool, duration: i32) -> Self {
        Self {
            l_onoff: if active { 1 } else { 0 },
            l_linger: duration,
        }
    }
}
