# APT repository publishing

The release workflow attaches an `amd64` `.deb` package to every GitHub Release. A second workflow publishes a signed APT repository to GitHub Pages.

## One-time GitHub setup

1. In **Settings → Pages**, set the build and deployment source to **GitHub Actions**.
2. Generate a dedicated signing key on a trusted machine:

   ```bash
   gpg --quick-generate-key "tuicli APT Archive" rsa4096 sign 0
   gpg --list-secret-keys --keyid-format long
   ```

3. Add these repository Actions secrets:
   - `APT_GPG_PRIVATE_KEY`: the ASCII-armored output of `gpg --armor --export-secret-keys <fingerprint>`.
   - `APT_GPG_PASSPHRASE`: the signing key passphrase.
4. Add the repository Actions variable `ENABLE_APT_REPOSITORY` with value `true`.

Keep the private key and passphrase out of the repository. The workflow publishes only the exported public key. Only maintainers allowed to create release tags should be allowed to run workflows that can use these secrets.

## Publishing a release

Push a version tag such as `v0.1.2`. The release workflow builds the binary and `.deb`; the APT workflow builds and signs repository metadata, then deploys it to GitHub Pages. The first Pages deployment requires the one-time setup above.

The current repository publishes `amd64` packages to the `stable` distribution. Packages are built on Ubuntu 22.04 and require glibc 2.35 or newer, so the documented minimum is Debian 12 or Ubuntu 22.04. APT clients use:

```text
https://qalmurri.github.io/tuicli
```

The repository is rebuilt from each release and contains the package for that release. If older versions need to remain installable from APT, the repository publishing workflow should retain prior `.deb` files when it is expanded to multiple-version retention.
