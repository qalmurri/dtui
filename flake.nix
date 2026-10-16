{
  description = "Lightweight terminal desktop environment for Linux TTY";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
    in {
      packages = forAllSystems (system:
        let pkgs = import nixpkgs { inherit system; };
        in {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "tuicli";
            version = "0.1.3";
            src = self;
            cargoLock.lockFile = ./Cargo.lock;

            meta = {
              description = "Lightweight terminal desktop environment for Linux TTY";
              homepage = "https://github.com/qalmurri/tuicli";
              license = pkgs.lib.licenses.mit;
              mainProgram = "tuicli";
              platforms = systems;
            };
          };
        });

      apps = forAllSystems (system: {
        default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/tuicli";
        };
      });
    };
}
