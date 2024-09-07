{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-24.05";

  outputs =
    { self, nixpkgs }:
    let
      eachSystem =
        f:
        nixpkgs.lib.genAttrs [
          "x86_64-linux"
          "aarch64-linux"
        ] f;
    in
    {
      devShells = eachSystem (
        system: with nixpkgs.legacyPackages.${system}; {
          default = mkShell {
            packages = [
              cargo
              cargo-expand
              rust-analyzer
              rustfmt
              just
            ];
          };
        }
      );
      packages = eachSystem (system: {
        default =
          let
            cargo = (nixpkgs.lib.importTOML ./Cargo.toml).package;
          in
            nixpkgs.legacyPackages.${system}.pkgsStatic.rustPlatform.buildRustPackage {
              pname = cargo.name;
              version = cargo.version;

              src = ./.;

              cargoLock.lockFile = ./Cargo.lock;

              meta = {
                license = nixpkgs.lib.licenses.gpl3Only;
                maintainers = with nixpkgs.lib.maintainers; [ samuel-martineau ];
              };
          };
      });
    };
}
