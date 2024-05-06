{
  description =
    "A todo list made in rust! This flake was made using the template from zero-to-nix";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      overlays = [
        (import rust-overlay)
        (self: super: { rustToolchain = super.rust-bin.stable.latest.default; })
      ];

      allSystems = [
        "x86_64-linux" # 64-bit Intel/AMD Linux
        "aarch64-linux" # 64-bit ARM Linux
        "x86_64-darwin" # 64-bit Intel macOS
        "aarch64-darwin" # 64-bit ARM macOS
      ];

      forAllSystems = f:
        nixpkgs.lib.genAttrs allSystems
        (system: f { pkgs = import nixpkgs { inherit overlays system; }; });
    in {
      # Development environment output
      devShells = forAllSystems ({ pkgs }: {
        default = pkgs.mkShell {
          # The Nix packages provided in the environment
          packages = (with pkgs; [ tmux rustToolchain ])
            ++ pkgs.lib.optionals pkgs.stdenv.isDarwin
            (with pkgs; [ libiconv ]);
          shellHook = ''
            tmux
          '';
        };
      });

      packages = forAllSystems ({ pkgs }: {
        default = let
          rustPlatform = pkgs.makeRustPlatform {
            cargo = pkgs.rustToolchain;
            rustc = pkgs.rustToolchain;
          };
        in rustPlatform.buildRustPackage {
          name = "todo-list";
          src = ./src;
          cargoLock.lockFile = ./Cargo.lock;
        };
      });
    };
}
