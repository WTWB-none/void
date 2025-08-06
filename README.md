![alt](https://github.com/WTWB-none/mindbreaker/blob/main/src-tauri/icons/128x128@2x.png?raw=true)
# 🧠 VOID 🧠

I could never find an app that combined all my needs as a second-brain tool. And that's how the idea for this project was born!

## 🔑 Key differences from popular second-brain apps

* **🔮 Rich functionality**: The application was conceived as a hybrid of Notion and Obsidian. I hope I can satisfy all your (and my)) needs!
* **🔐 Local-first**: Your data is stored locally! No third-party servers! Your secrets remain yours!
* **⚓️‍💥 Open-source**: The code is fully open and available to everyone on GitHub!
* **✨ Cross-platform**: If your platform is not yet supported, you can simply rebuild it from source!
* **🛠️ Easy to customize**: The functionality of the app can be easily configured and extended without any programming skills!
* **💪 Flexible plugin system**: A wide range of pre-installed plugins, and if that’s not enough, you can easily create your own!
* **🚀 Fast and efficient**: The app is built with Rust and focused on performance!

## 📝 How to install and run VOID

> 🛠 **Note**: Rust is required for all systems. 
> WARNING gcc-14 required for build. If you use gcc-15 or version below 14 you need to downgrade/update your gcc up to 14 version. This limitation caused by surrealdb and cannot be bypassed untill surrealdb update their version of librocksdb-sys. Thanks for understanding.

### 1. Clone the repository

```bash
git clone https://github.com/WTWB-none/void.git
cd void
```

### 2. Install dependencies

```bash
npm install
```

### 3. Install system requirements

<details>
<summary>Windows</summary>

* Microsoft C++ Build Tools
* WebView2
* Node.js

</details>

<details>
<summary>Debian / Ubuntu</summary>

```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

</details>

<details>
<summary>Arch Linux</summary>

```bash
sudo pacman -Syu
sudo pacman -S --needed \
  webkit2gtk-4.1 \
  base-devel \
  curl \
  wget \
  file \
  openssl \
  appmenu-gtk-module \
  libappindicator-gtk3 \
  librsvg \
  xdotool
```

</details>

<details>
<summary>Fedora</summary>

```bash
sudo dnf check-update
sudo dnf install webkit2gtk4.1-devel \
  openssl-devel \
  curl \
  wget \
  file \
  libappindicator-gtk3-devel \
  librsvg2-devel \
  libxdo-devel
sudo dnf group install "c-development"
```

</details>

<details>
<summary>Gentoo</summary>

```bash
sudo emerge --ask \
  net-libs/webkit-gtk:4.1 \
  dev-libs/libappindicator \
  net-misc/curl \
  net-misc/wget \
  sys-apps/file
```

</details>

<details>
<summary>openSUSE</summary>

```bash
sudo zypper up
sudo zypper in webkit2gtk3-devel \
  libopenssl-devel \
  curl \
  wget \
  file \
  libappindicator3-1 \
  librsvg-devel
sudo zypper in -t pattern devel_basis
```

</details>

<details>
<summary>Alpine Linux</summary>

```bash
sudo apk add \
  build-base \
  webkit2gtk \
  curl \
  wget \
  file \
  openssl \
  libayatana-appindicator-dev \
  librsvg
```

</details>

<details>
<summary>NixOS</summary>

```nix
# shell.nix
let
  pkgs = import <nixpkgs> { };
in
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
    gobject-introspection
    cargo
    cargo-tauri
    nodejs
  ];

  buildInputs = with pkgs; [
    at-spi2-atk
    atkmm
    cairo
    gdk-pixbuf
    glib
    gtk3
    harfbuzz
    librsvg
    libsoup_3
    pango
    webkitgtk_4_1
    openssl
    gcc
    llvmPackages.clang
    zlib
    bzip2
    lz4
    zstd
    rocksdb
  ];

  LIBCLANG_PATH="${pkgs.libclang.lib}/lib";

  ROCKSDB_LIB_DIR="${pkgs.rocksdb}/lib";
  ROCKSDB_INCLUDE_DIR="${pkgs.rocksdb}/include";
  ROCKSDB_LINK_TYPE="static";
}
```

</details>

<details>
<summary>macOS</summary>

* Xcode (via App Store or Xcode Command Line Tools)

</details>

### 4. Run the app in development mode

```bash
npm run tauri dev
```

## 👨‍💻 Tech Stack

* **Frontend**: Vue.js, TypeScript
* **Backend**: Rust, Tauri
* **Database**: SurrealDB (for configuration and possibly user databases)

## 🗐️ Roadmap

Still under development, I'll share plans soon.

## 📝 Contact the Developer

If you have any suggestions, feel free to contact me on Telegram [@iomanip](https://t.me/GhostOfTranshumanist).

## 💵 Support the Project

You can support the project on [Boosty](https://boosty.to/void_project) and on [Patreon](https://patreon.com/c/transhuman1st)

If this project can replace my job, I can devote more time to its development and release it faster.
Everyone who supports the project will be mentioned on the official site and in the "Sponsors" section of the app.

## Socials
You can check our updates in 
[Telegram](https://t.me/void_project_tg)
[Reddit](https://reddit.com/r/void_foss)
[X](https://x.com/void_project_x)

## 📄 License

This project is licensed under the [Apache License 2.0](./LICENSE).
