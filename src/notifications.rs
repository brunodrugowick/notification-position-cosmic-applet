use anyhow::{Context, Result};
use cosmic::cosmic_config::{Config, CosmicConfigEntry};
use cosmic_notifications_config::{Anchor, ID, NotificationsConfig};

pub fn load() -> Result<NotificationsConfig> {
    let helper =
        Config::new(ID, NotificationsConfig::VERSION).context("failed to open COSMIC config")?;

    Ok(NotificationsConfig::get_entry(&helper)
        .map(|config| config)
        .unwrap_or_else(|(_errors, config)| config))
}

pub fn apply_top_center() -> Result<NotificationsConfig> {
    let mut config = load()?;
    config.anchor = Anchor::Top;
    save(&config)?;
    Ok(config)
}

pub fn reset_defaults() -> Result<NotificationsConfig> {
    let config = NotificationsConfig::default();
    save(&config)?;
    Ok(config)
}

pub fn is_top_center(config: &NotificationsConfig) -> bool {
    matches!(config.anchor, Anchor::Top)
}

pub fn anchor_name(anchor: &Anchor) -> &'static str {
    match anchor {
        Anchor::Top => "Top",
        Anchor::Bottom => "Bottom",
        Anchor::Right => "Right",
        Anchor::Left => "Left",
        Anchor::TopLeft => "TopLeft",
        Anchor::TopRight => "TopRight",
        Anchor::BottomLeft => "BottomLeft",
        Anchor::BottomRight => "BottomRight",
    }
}

fn save(config: &NotificationsConfig) -> Result<()> {
    let helper =
        Config::new(ID, NotificationsConfig::VERSION).context("failed to open COSMIC config")?;

    config
        .write_entry(&helper)
        .context("failed to write COSMIC notifications config")
}
