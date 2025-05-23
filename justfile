dev:
    sudo chown -R enzo rudeus

update-config: dev
    cp -r ~/.config/alacritty/* rudeus/etc/rudeus/default/alacritty/.config/alacritty
    cp -r ~/.config/eww/* rudeus/etc/rudeus/default/eww/.config/eww

    cp ~/.config/fish/config.fish rudeus/etc/rudeus/default/fish/.config/fish/
    cp -r ~/.config/niri/* rudeus/etc/rudeus/default/niri/.config/niri
    cp ~/.config/starship.toml rudeus/etc/rudeus/default/starship/.config/
    cp -r ~/.config/wofi/* rudeus/etc/rudeus/default/wofi/.config/wofi
    cp -r ~/.config/yazi/* rudeus/etc/rudeus/default/yazi/.config/yazi
    cp -r ~/.config/zed/* rudeus/etc/rudeus/default/zed/.config/zed

deb-build: update-config
    sudo chown -R root:root rudeus
    dpkg-deb --build rudeus rudeus_amd64.deb

install-dependencies:
    sudo apt install -y cmake g++ pkg-config libfreetype6-dev libfontconfig1-dev libxcb-xfixes0-dev libxkbcommon-dev python3
    sudo apt install -y gcc clang libudev-dev libgbm-dev libxkbcommon-dev libegl1-mesa-dev libwayland-dev libinput-dev libdbus-1-dev libsystemd-dev libseat-dev libpipewire-0.3-dev libpango1.0-dev libdisplay-info-dev
    sudo apt install -y libxcb-cursor-dev
    sudo apt install -y 7zip
    sudo apt install -y libgtk-3-dev libdbusmenu-gtk3-dev libgtk-layer-shell-dev

clone-artifacts-repo:
    cd artifacts && git clone https://github.com/alacritty/alacritty
    cd artifacts && git clone https://github.com/YaLTeR/niri/
    cd artifacts && git clone https://github.com/Supreeeme/xwayland-satellite
    cd artifacts && git clone https://github.com/sxyazi/yazi.git
    cd artifacts && git clone https://github.com/elkowar/eww

build-artifacts:
    cd artifacts/alacritty && cargo build --release
    cd artifacts/niri && cargo build --release

    cargo build --release -p rudeus-niri

    cd artifacts/xwayland-satellite && cargo build --release
    cd artifacts/yazi && cargo build --release --locked
    cd artifacts/eww && cargo build --release --no-default-features --features=wayland

move-artifacts:
    cp artifacts/alacritty/target/release/alacritty rudeus/usr/local/bin
    cp artifacts/niri/target/release/niri rudeus/usr/local/bin

    cp target/release/rudeus-niri rudeus/usr/local/bin

    cp artifacts/xwayland-satellite/target/release/xwayland-satellite rudeus/usr/local/bin
    cp artifacts/yazi/target/release/yazi rudeus/usr/local/bin
    cp artifacts/eww/target/release/eww rudeus/usr/local/bin

upload-artifacts version:
    gh release create {{version}} rudeus_amd64.deb --title "{{version}}" --notes "Rudeus release {{version}}"
