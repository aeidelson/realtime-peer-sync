
// This is the actual client, which is wrapped in the c interface in `lib.rs`.
pub struct Client {
    internal_val: i32,
}

impl Client {
    pub fn new() -> Client {
        return Client {
            internal_val: 99,
        };
    }

    pub fn dbg(&self) {
        println!("Debug value: {}", self.internal_val);
    }
}
