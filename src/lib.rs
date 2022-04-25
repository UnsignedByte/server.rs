pub use rocket;

pub trait Launch {
    fn new() -> Self;

    fn mount(&self, path: &str, app: rocket::Rocket) -> rocket::Rocket;
}