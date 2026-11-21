use dcd::policy::types::track::Track;
use dcd::policy::validate::stacks::track::check;

fn run(track: &Track) -> Vec<String> {
    let mut issues = Vec::new();
    check("p", track, &mut issues);
    issues
}

fn digest(value: &str) -> Track {
    Track::Digest(value.to_owned())
}

#[test]
fn tag_is_valid() {
    assert!(run(&Track::Tag("latest".to_owned())).is_empty());
}

#[test]
fn empty_tag_is_rejected() {
    assert!(run(&Track::Tag(String::new()))[0].starts_with("p.tag"));
}

#[test]
fn semver_is_valid() {
    assert!(run(&Track::Semver("^1.2".to_owned())).is_empty());
}

#[test]
fn blank_semver_is_rejected() {
    assert!(run(&Track::Semver(" ".to_owned()))[0].starts_with("p.semver"));
}

#[test]
fn proper_digest_is_valid() {
    assert!(run(&digest(&format!("sha256:{}", "a".repeat(64)))).is_empty());
}

#[test]
fn digest_with_wrong_prefix_is_rejected() {
    assert!(run(&digest(&format!("md5:{}", "a".repeat(64))))[0].starts_with("p.digest"));
}

#[test]
fn short_digest_is_rejected() {
    assert!(run(&digest("sha256:abc"))[0].starts_with("p.digest"));
}

#[test]
fn uppercase_digest_is_rejected() {
    assert!(run(&digest(&format!("sha256:{}", "A".repeat(64))))[0].starts_with("p.digest"));
}
