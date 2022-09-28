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
    <https://github.com/nrf-rs/nrf-pacs/releases/>, select the latest release, and click the link
    "N commits to master since this release".
* **Version Bump**: Determine whether the next release contains breaking changes. This informs what
  kind of version bump is needed (minor vs. major). Then, bump the crate version in
  `Cargo.toml` accordingly. Do not commit the result, that will be done in the next step.
* **Publish**: Run `cargo xtask publish`. This will:
  * Bump the changelog, replacing the "Unreleased" section with one for the current version.
  * Create a commit and tag for the release.
  * Run `cargo publish` on each PAC.
* **Pull Request**: Open a Pull Request with the changes and merge it once CI passes.
* **GitHub Release**: Go to <https://github.com/nrf-rs/nrf-pacs/releases> and click the tag you just
  pushed. Click *Edit Tag* and paste the changelog entries for the new version (no *Release Title*
  is necessary). Click *Publish Release*.
* **Announcement**: Post a link to the GitHub release in [`#nrf-rs:matrix.org`] and any other places
  you'd like to announce it.

[`#nrf-rs:matrix.org`]: https://matrix.to/#/#nrf-rs:matrix.org
