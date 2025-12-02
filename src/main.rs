mod app;
mod editor;
mod search;

fn main() -> Result<(), druid::PlatformError> {
    app::run()
}
