# Contribution and Maintenance notes

## Contributing

Pull Requests and issue reports are very welcome!

If you are unsure whether something would make a good contribution (or you aren't sure how to
tackle something), feel free to either open an issue to solicit feedback or chat with us at
[`#nrf-rs:matrix.org`]. This is preferred over opening a big PR that might then not get merged.

If a Pull Request has not been reviewed for a long time, feel free to ping the maintainers.

## Release process

In order to release a new version of the PACs, the following steps need to be performed:

* **Changelog**: Update [the changelog](./CHANGELOG.md) to list all notable changes under the `Unreleased`
  section.
  * You can use GitHub's "compare" feature to view all commits added since the last release. Go to
    <https://github.com/nrf-rs/nrf-hal/releases/>, select the latest release, and click the link
    "N commits to master since this release".
* **Version Bump**: Determine whether the next release contains breaking changes. This informs what
  kind of version bump is needed (minor vs. major). Then, bump the crate versions accordingly.
  Because of the large number of crates, we use [`cargo-xtask`]-based automation to bump all version
  numbers. Invoke it via `cargo xtask bump <new-version>` to update all version numbers to
  `<new-version>`.
* **Pull Request**: Open a Pull Request with the version bump and merge it once CI passes.
* **Tag**: Run `git pull`, and tag the release by running `git tag -a -m 'vX.Y.Z' 'vX.Y.Z'`,
  replacing `X.Y.Z` with the version you bumped to. Run `git push --tags origin`.
* **Publish**: Publish all HAL crates to crates.io, starting with `nrf-hal-common`. For that crate,
  you have to pass `--no-verify` to `cargo publish`, since the crate only builds with specific
  Cargo features. The examples should not be published.
* **GitHub Release**: Go to <https://github.com/nrf-rs/nrf-hal/releases> and click the tag you just
  pushed. Click *Edit Tag* and paste the changelog entries for the new version (no *Release Title*
  is necessary). Click *Publish Release*.
* **Announcement**: Post a link to the GitHub release in [`#nrf-rs:matrix.org`] and any other places
  you'd like to announce it.

[`cargo-xtask`]: https://github.com/matklad/cargo-xtask/
[`#nrf-rs:matrix.org`]: https://matrix.to/#/#nrf-rs:matrix.org
