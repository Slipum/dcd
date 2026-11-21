use dcd::policy::validate::stacks::image::check;

fn run(image: &str) -> Vec<String> {
    let mut issues = Vec::new();
    check("p", image, &mut issues);
    issues
}

#[test]
fn plain_name_is_valid() {
    assert!(run("nginx").is_empty());
}

#[test]
fn name_with_host_and_path_is_valid() {
    assert!(run("ghcr.io/me/blog").is_empty());
}

#[test]
fn registry_port_is_not_a_tag() {
    assert!(run("localhost:5000/app").is_empty());
}

#[test]
fn empty_image_is_rejected() {
    assert!(run("")[0].contains("пустым"));
}

#[test]
fn whitespace_is_rejected() {
    assert!(run("my app")[0].contains("пробелы"));
}

#[test]
fn tag_is_rejected() {
    assert!(run("app:1.2")[0].contains("без тега"));
}

#[test]
fn tag_after_path_is_rejected() {
    assert!(run("ghcr.io/me/blog:latest")[0].contains("без тега"));
}

#[test]
fn digest_is_rejected_with_a_single_message() {
    let issues = run("app@sha256:abc");
    assert_eq!(issues.len(), 1);
    assert!(issues[0].contains("без digest"));
}
