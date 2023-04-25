pub mod data;
pub mod session;

pub fn init() {
    session::init().unwrap();
}
