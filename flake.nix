{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = {
    nixpkgs,
    fenix,
    ...
  }: let
    system = "x86_64-linux";
    toolchain = "stable";
    overlays = [fenix.overlays.default];
    pkgs = import nixpkgs {
      inherit system overlays;
    };
    rustPkg = pkgs.fenix."${toolchain}".withComponents [
      "cargo"
      "clippy"
      "rust-src"
      "rustc"
      "rustfmt"
    ];
  in {
    devShells.x86_64-linux.default = pkgs.mkShell {
      nativeBuildInputs = with pkgs; [
        gdb
        rustPkg
        rust-analyzer-nightly
      ];
    };
  };
}
