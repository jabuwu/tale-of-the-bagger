# Tale of the Bagger: A Love Story

Remake of [Tale of the Bagger](https://www.newgrounds.com/portal/view/715392) in [Bevy](https://bevyengine.org/).

## Build Commands

### Local Development

```
cargo run --features dev
```

### Web Development

First, install [Trunk](https://trunkrs.dev/).

```
trunk serve
```

### iOS Development

First, install [cargo-xcodebuild](https://github.com/Gordon-F/cargo-xcodebuild#setup), then uncomment the `lib` lines in `Cargo.toml`.

```
cargo xcodebuild run --features embedded_assets
```

## License

Code is licensed under dual MIT / Apache-2.0 but with no attribution necessary. All contributions must agree to this licensing.
