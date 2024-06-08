{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.SDL2
    pkgs.pkg-config
  ];

  shellHook = ''
    export PKG_CONFIG_PATH=${pkgs.SDL2}/lib/pkgconfig
  '';
}
