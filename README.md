![alt](https://github.com/WTWB-none/mindbreaker/blob/main/src-tauri/icons/128x128@2x.png?raw=true)

# 🧠 VOID 🧠

Я очень долго не мог найти приложение которое смогло бы сочетать в себе все мои потребности в качестве second-brain приложения. Таким образом и родилась идея этого проекта!

## 🔑 Ключевые отличия от популярных second-brain приложений

- **🔮 Богатый функционал**: Приложение задумывалось как симбиоз notion и obsidian. Надеюсь у меня получится удовлетворить все ваши(и мои)) потребности!
- **🔒 Local-first**: Ваши данные храняться локально! Никаких сторонних серверов! Ваши секреты остануться вашими!
- **⛓️‍💥 Open-source**: Код проекта полностью открыт и доступен для всех желающих на GitHub!
- **✨ Кроссплатформенность**: Если ваша платформа пока не поддерживается, вы можете просто пересобрать его из исходников!
- **🛠️ Легкость в кастомизации**: Функционал приложения можно легко настраивать и расширять, не требуя навыков программирования!
- **💪 Гибкая система плагинов**: Широкий выбор предустановленных плагинов, однако если вам их не хватает вы с легкостью можете создать свой!
- **🚀 Быстрый и эффективный**: Приложение разработано на Rust с фокусом на производительность!

## 📝 Как установить и запустить VOID 

На данный момент (на полпути к альфе):

- склонировать данный репозиторий

```
git clone https://github.com/WTWB-none/void.git
```

- установить зависимости
  для всех систем обязательно нужен Rust
  
  -  windows
      Microsoft C++ Build Tools
      WebView2
      node
  - linux
    
    ``` 
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
    ``` arch
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
    ``` fedora
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
    ``` gentoo
    sudo emerge --ask \
      net-libs/webkit-gtk:4.1 \
      dev-libs/libappindicator \
      net-misc/curl \
      net-misc/wget \
      sys-apps/file
    ```
    ``` openSUSE
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
    ``` Alpine
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
    ``` NixOS
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
    
      buildInputs = with pkgs;[
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
      ];
    }
    ```
  - mac os
    - xcode
```
cd void && npm install
```

- запустить проект в dev режиме

```
npm run tauri dev
```

## 🧑‍💻 Используемый стек

- **Frontend**: Vue.js, TypeScript
- **Backend**: Rust, Tauri
- **Database**: SurrealDB(конфигурация и возможно пользовательские бд)

## 🗺️ Roadmap

Пока что в процессе разработки, о планах расскажу в ближайшее время)

## 📝 Связь с разработчиком

Если у вас есть какие-либо предложения, пожалуйста, свяжитесь со мной в телеграмм [iomanip](https://t.me/GhostOfTranshumanist).

## 💵 Вы можете поддержать проект на [Boosty](https://boosty.to/transhumanistdream) и в будущем на ~~[Patreon](...)~~

Если этот проект сможет заменить мне работу то я смогу больше времени уделять его разработке и он выйдет гораздо быстрее)
Каждый человек поддержавший проект будет указан на официальном сайте проекта а также в разделе "Спонсоры" в приложении.

