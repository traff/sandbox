extern crate semver;

use semver::Version;

fn main() {
    assert!(Version::parse("1.2.3") == Ok(Version {
        major: 1u64,
        minor: 2u64,
        patch: 3u64,
        pre: vec!(),
        build: vec!(),
    }));

    println!("Versions compared successfully!");
}
