{ nixpkgs, pkgs, lib, config, inputs, ... }:

{
languages.rust = {
  channel = "nightly";
  components = [
    "cargo"
    "rust-src"
    "rustc"
  ];
  enable = true;
};

packages = [pkgs.gcc];

env.LD_LIBRARY_PATH = "${nixpkgs.lib.makeLibraryPath [
          # pkgs.xorg.libX11
          # pkgs.xorg.libXcursor
          # pkgs.xorg.libxcb
          # pkgs.xorg.libXi
          pkgs.libxkbcommon
          pkgs.libGL
          pkgs.libxkbcommon
          pkgs.wayland
        ]}";
}
