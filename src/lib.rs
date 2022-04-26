use std::path::Path;
pub use rocket;
pub use rocket_contrib;

pub trait Launch {
    /// Create a new launcher on a root path `root`.
    fn new <P: AsRef<Path>> (root: P) -> Self;

    /// mount the launcher onto an app `app` at subpath `path` (consuming).
    fn mount(self, path: &str, app: rocket::Rocket) -> rocket::Rocket;
}