mod app;
mod notifications;

fn main() -> cosmic::iced::Result {
    cosmic::applet::run::<app::Applet>(())
}
