let
  pkgs = import <nixpkgs> { };
in
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    openssl
    pkg-config
    wayland
    wayland-protocols
    wayland-utils
    vulkan-loader
    mesa
    squashfsTools
    alsa-lib
    alsa-utils
    clang
    libclang
    clang-tools
    libxcb
    libxkbcommon
    libx11
    luajit
  ];
  buildInputs = with pkgs; [
    libclang
    clang
    alsa-lib
    alsa-utils
    pkg-config
    libxcb
    libxkbcommon
    libx11
  ];
  LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
    pkgs.wayland
    pkgs.libxkbcommon
    pkgs.vulkan-loader
    pkgs.mesa
  ];
}
