# Notification Position COSMIC

Minimal COSMIC panel applet for changing notification placement.

Its popup controls the existing `cosmic-notifications` config and sets notifications to the top-center position.

## Behavior

Top-center is represented by:

- `Anchor::Top`

So this applet applies `com.system76.CosmicNotifications.anchor = Top`.

## Development

```sh
cargo check
cargo run
```

## Install

```sh
just build-release
sudo just install
```
