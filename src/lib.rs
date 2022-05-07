use std::path::Path;
pub use rocket;
pub use rocket_contrib;

pub trait Launch : Sized {
    /// Create a new launcher on a root path `root`.
    fn new <P: AsRef<Path>> (root: P) -> std::io::Result<Self>;

    /// mount the launcher onto an app `app` at subpath `path` (consuming).
    fn mount<P: AsRef<Path>> (self, path: P, app: rocket::Rocket) -> rocket::Rocket;
}