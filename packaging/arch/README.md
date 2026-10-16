# Arch Linux package

`PKGBUILD` builds tuicli v0.1.3 from its tagged source. To build and install it locally:

```sh
makepkg -si
```

To prepare an AUR submission, generate `.SRCINFO` from this directory with:

```sh
makepkg --printsrcinfo > .SRCINFO
```

The AUR submission itself belongs in the separate `tuicli` repository on AUR. `yay` and other AUR helpers can install it after that submission is published; they are clients for AUR, not separate package repositories.
