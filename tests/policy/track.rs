use dcd::policy::types::track::Track;

fn parse(yaml: &str) -> Result<Track, serde_norway::Error> {
    serde_norway::from_str(yaml)
}

#[test]
fn accepts_tag_only() {
    assert_eq!(
        parse("tag: latest").unwrap(),
        Track::Tag("latest".to_owned())
    );
}

#[test]
fn accepts_semver_only() {
    assert_eq!(
        parse("semver: \"^1.2\"").unwrap(),
        Track::Semver("^1.2".to_owned())
    );
}

#[test]
fn accepts_digest_only() {
    let track = parse("digest: sha256:abc").unwrap();
    assert_eq!(track, Track::Digest("sha256:abc".to_owned()));
}

#[test]
fn rejects_empty_mapping() {
    let message = parse("{}").unwrap_err().to_string();
    assert!(message.contains("нужно указать"));
}

#[test]
fn rejects_two_fields() {
    let message = parse("tag: a\nsemver: b").unwrap_err().to_string();
    assert!(message.contains("только одно"));
}

#[test]
fn rejects_unknown_field() {
    let message = parse("branch: main").unwrap_err().to_string();
    assert!(message.contains("branch"));
}
