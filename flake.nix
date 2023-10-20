{
  description = "Great wall mind wallet";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { flake-utils, nixpkgs, self }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs { inherit system; }; in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            rust-analyzer
            rustc
            rustfmt
          ];

          RUST_SRC_PATH = pkgs.rust.packages.stable.rustPlatform.rustLibSrc;
        };
        formatter = pkgs.nixpkgs-fmt;
      }
    );
}
