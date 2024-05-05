{
  description = "WASM-powered personal website";
  inputs = {
    nixpkgs = {url = "github:nixos/nixpkgs/nixos-unstable";};
    rust-overlay = {url = "github:oxalica/rust-overlay";};
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  }: let
    system = "x86_64-linux";
  in {
    devShell.${system} = let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [rust-overlay.overlay];
      };
    in (({pkgs, ...}:
      pkgs.mkShell {
        buildInputs = with pkgs; [
          rust-analyzer
          cargo
          nodePackages.sass
          trunk
          wasm-bindgen-cli
          wasm-pack
          (rust-bin.nightly.latest.default.override {
            extensions = ["rust-src"];
            targets = ["wasm32-unknown-unknown"];
          })
        ];

        shellHook = "";
      }) {pkgs = pkgs;});
  };
}
