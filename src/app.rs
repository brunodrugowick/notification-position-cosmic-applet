use crate::fl;
use crate::notifications;
use cosmic::app::Task;
use cosmic::iced::core::Rectangle;
use cosmic::iced::{Limits, Subscription, window::Id};
use cosmic::prelude::*;
use cosmic::surface::action::{app_popup, destroy_popup};
use cosmic::widget;
use cosmic_notifications_config::NotificationsConfig;

#[derive(Default)]
pub struct Applet {
    core: cosmic::Core,
    popup: Option<Id>,
    notifications_config: NotificationsConfig,
    status: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    PopupClosed(Id),
    Surface(cosmic::surface::Action),
    ConfigChanged(NotificationsConfig),
    ApplyTopCenter,
    ResetDefaults,
    ConfigSaved(Result<NotificationsConfig, String>),
}

impl cosmic::Application for Applet {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = "dev.drugo.NotificationPositionCosmicApplet";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    fn init(
        core: cosmic::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<Self::Message>) {
        let notifications_config = notifications::load().unwrap_or_default();
        let applet = Self {
            core,
            popup: None,
            status: status_for(&notifications_config),
            notifications_config,
        };

        (applet, Task::none())
    }

    fn on_close_requested(&self, id: Id) -> Option<Self::Message> {
        Some(Message::PopupClosed(id))
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let have_popup = self.popup;
        let button = self
            .core
            .applet
            .icon_button("preferences-system-notifications-symbolic")
            .on_press_with_rectangle(move |offset, bounds| {
                if let Some(id) = have_popup {
                    Message::Surface(destroy_popup(id))
                } else {
                    Message::Surface(app_popup::<Self>(
                        move |state: &mut Self| {
                            let new_id = Id::unique();
                            state.popup = Some(new_id);

                            let mut popup_settings = state.core.applet.get_popup_settings(
                                state.core.main_window_id().unwrap(),
                                new_id,
                                None,
                                None,
                                None,
                            );
                            popup_settings.positioner.size_limits = Limits::NONE
                                .min_width(280.0)
                                .max_width(420.0)
                                .min_height(160.0)
                                .max_height(600.0);
                            popup_settings.positioner.anchor_rect = Rectangle {
                                x: (bounds.x - offset.x) as i32,
                                y: (bounds.y - offset.y) as i32,
                                width: bounds.width as i32,
                                height: bounds.height as i32,
                            };
                            popup_settings
                        },
                        Some(Box::new(|state: &Self| {
                            let content = widget::column::with_capacity(7)
                                .spacing(8)
                                .push(widget::text::title4(fl!("popup-title")))
                                .push(widget::text(fl!(
                                    "current-anchor",
                                    anchor = notifications::anchor_name(
                                        &state.notifications_config.anchor
                                    )
                                )))
                                .push(widget::text(fl!("top-center-note")))
                                .push(
                                    widget::button::text(fl!("apply-center"))
                                        .on_press(Message::ApplyTopCenter),
                                )
                                .push(
                                    widget::button::text(fl!("reset-defaults"))
                                        .on_press(Message::ResetDefaults),
                                )
                                .push(widget::divider::horizontal::default())
                                .push(widget::text(state.status.as_str()));

                            Element::from(state.core.applet.popup_container(
                                widget::container(content).padding(8),
                            ))
                                .map(cosmic::Action::App)
                        })),
                    ))
                }
            });

        button.into()
    }

    fn view_window(&self, _id: Id) -> Element<'_, Self::Message> {
        widget::text("").into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        self.core()
            .watch_config::<NotificationsConfig>(cosmic_notifications_config::ID)
            .map(|update| Message::ConfigChanged(update.config))
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::TogglePopup => {}
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
            Message::Surface(action) => {
                return cosmic::task::message(cosmic::Action::Cosmic(
                    cosmic::app::Action::Surface(action),
                ));
            }
            Message::ConfigChanged(config) => {
                self.notifications_config = config;
                self.status = status_for(&self.notifications_config);
            }
            Message::ApplyTopCenter => {
                return Task::perform(
                    async { notifications::apply_top_center().map_err(|err| err.to_string()) },
                    |result| cosmic::Action::App(Message::ConfigSaved(result)),
                );
            }
            Message::ResetDefaults => {
                return Task::perform(
                    async { notifications::reset_defaults().map_err(|err| err.to_string()) },
                    |result| cosmic::Action::App(Message::ConfigSaved(result)),
                );
            }
            Message::ConfigSaved(result) => match result {
                Ok(config) => {
                    self.notifications_config = config;
                    self.status = status_for(&self.notifications_config);
                }
                Err(err) => {
                    self.status = fl!("status-error", error = err);
                }
            },
        }

        Task::none()
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }
}

fn status_for(config: &NotificationsConfig) -> String {
    if notifications::is_top_center(config) {
        fl!("status-center")
    } else {
        fl!("status-not-center")
    }
}
