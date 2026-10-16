# Additional package distribution

The v0.1.3 source contains package definitions for Arch Linux, Homebrew on Linux, and Nix:

- `packaging/arch/PKGBUILD` — source build for `makepkg`; submit the generated `.SRCINFO` and `PKGBUILD` to a separate AUR repository before advertising `yay -S tuicli`.
- `packaging/homebrew/tuicli.rb` — Homebrew formula for Linux. To offer `brew install qalmurri/tap/tuicli`, publish this formula in a separate `homebrew-tap` repository.
- `flake.nix` — Nix flake package for `x86_64-linux` and `aarch64-linux`.

These definitions do not publish packages by themselves. Validate each package on its target platform, create and check in `flake.lock` with a supported Nix installation, then publish the AUR and Homebrew tap repositories. Do not announce those install commands as available until their package definitions are published.
