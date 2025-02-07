# uwu-tray

[fastest uwuifier in the west](https://github.com/Daniel-Liu-c0deb0t/uwu)... now in your system tray for even faster uwufication!

## FAQ

### Which platforms are supported?

 - [x] Windows
 - [x] Linux
 - [x] MacOS (Intel only because sse is only available on x86 processors)

### How do I use it?

 1. Run the executable found in the [Releases](https://github.com/Olaren15/uwu-tray/releases/latest)
 2. Click on the tray icon
 3. Choose "uwuify"
 4. Black magic
 5. Text in the clipboard is now uwuified

### I want to know more about the uwuification process

Great! I used the [uwuify](https://crates.io/crates/uwuify) crate. The Readme on [Github](https://github.com/Daniel-Liu-c0deb0t/uwu) contains all the details!

### Where are the UwUs???? All I'm reading rn is normal text!!!! ヽ（≧□≦）ノ

I have a version of this Readme [here](WEADME.md)

### Will there be updates in the future?

Maybe, maybe not, who knows ¯\\\_(ツ)_/¯

### Can I use or redistribute your code?

Yes, it's licensed under the MIT license

## Building

### Rust

This app is built with rust so make sure to have it installed on your system

### Dependencies

Since this app uses system APIs some dependencies need to be installed before building the app

#### Windows

- Windows SDK (Can be installed via Visual Studio Installer)

#### Linux

- GTK 3
- ayatana-appindicator

##### Install on Arch linux

```shell
sudo pacman -S gtk3 libayatana-appindicator
```

##### Install on Ubuntu / Debian

```shell
sudo apt install libgtk-3-dev libayatana-appindicator3-dev
```

##### Install on Fedora

```shell
sudo dnf install gtk3-devel libayatana-appindicator-gtk3-devel
```

#### MacOS

- Xcode command line tools

```shell
xcode-select --install
```

### Creating the executable

Once the dependencies are installed building is a simple as running

```shell
cargo build
```

---
Made with ❤️ by a friend of Blåhaj
