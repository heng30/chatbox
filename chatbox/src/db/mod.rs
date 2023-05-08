pub mod data;
pub mod session;
pub mod archive;

pub fn init() {
    session::init().unwrap();
}
