pub mod session;
pub mod data;

pub fn init() {
    session::init().unwrap();
}
