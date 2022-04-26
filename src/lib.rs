pub use rocket;
pub use rocket_contrib;

pub trait Launch {
    fn new() -> Self;

    fn mount(&self, path: &str, app: rocket::Rocket) -> rocket::Rocket;
}