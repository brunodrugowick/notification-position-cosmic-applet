name := 'notification-position-cosmic'
appid := 'dev.drugo.NotificationPositionCosmicApplet'

rootdir := ''
prefix := '/usr'
cargo-target-dir := env('CARGO_TARGET_DIR', 'target')

appdata := appid + '.metainfo.xml'
desktop := appid + '.desktop'
icon-svg := appid + '.svg'

base-dir := absolute_path(clean(rootdir / prefix))
appdata-dst := base-dir / 'share' / 'appdata' / appdata
bin-dst := base-dir / 'bin' / name
desktop-dst := base-dir / 'share' / 'applications' / desktop
icons-dst := base-dir / 'share' / 'icons' / 'hicolor'
icon-svg-dst := icons-dst / 'scalable' / 'apps' / icon-svg

default: build-release

clean:
    cargo clean

build-debug *args:
    cargo build --locked {{args}}

build-release *args: (build-debug '--release' args)

check *args:
    cargo clippy --all-features --locked {{args}} -- -W clippy::pedantic

run *args:
    env RUST_BACKTRACE=full cargo run --locked {{args}}

install:
    install -Dm0755 {{ cargo-target-dir / 'release' / name }} {{bin-dst}}
    install -Dm0644 {{ 'resources' / desktop }} {{desktop-dst}}
    install -Dm0644 {{ 'resources' / appdata }} {{appdata-dst}}
    install -Dm0644 {{ 'resources' / 'icons' / 'hicolor' / 'scalable' / 'apps' / icon-svg }} {{icon-svg-dst}}

uninstall:
    rm {{bin-dst}} {{desktop-dst}} {{icon-svg-dst}}
