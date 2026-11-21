use dcd::policy::sections::registry::{Kind, Registry};
use dcd::policy::validate::sections::registry::check;

fn registry(name: &str, kind: Kind, host: Option<&str>, auth: (bool, bool)) -> Registry {
    Registry {
        name: name.to_owned(),
        kind,
        host: host.map(str::to_owned),
        username_env: auth.0.then(|| "USER".to_owned()),
        token_env: auth.1.then(|| "TOKEN".to_owned()),
    }
}

fn run(registries: &[Registry]) -> Vec<String> {
    let mut issues = Vec::new();
    check(registries, &mut issues);
    issues
}

#[test]
fn hub_with_credentials_is_valid() {
    assert!(run(&[registry("hub", Kind::Dockerhub, None, (true, true))]).is_empty());
}

#[test]
fn public_access_without_credentials_is_valid() {
    assert!(run(&[registry("hub", Kind::Ghcr, None, (false, false))]).is_empty());
}

#[test]
fn gitea_with_host_is_valid() {
    assert!(
        run(&[registry(
            "g",
            Kind::Gitea,
            Some("git.example.com"),
            (true, true)
        )])
        .is_empty()
    );
}

#[test]
fn gitea_without_host_is_rejected() {
    let issues = run(&[registry("g", Kind::Gitea, None, (true, true))]);
    assert!(issues[0].contains("host"));
}

#[test]
fn forgejo_with_blank_host_is_rejected() {
    let issues = run(&[registry("f", Kind::Forgejo, Some(" "), (true, true))]);
    assert!(issues[0].contains("host"));
}

#[test]
fn duplicate_names_are_rejected() {
    let one = registry("same", Kind::Ghcr, None, (false, false));
    assert!(run(&[one.clone(), one])[0].contains("уже используется"));
}

#[test]
fn empty_name_is_rejected() {
    assert!(run(&[registry("", Kind::Ghcr, None, (false, false))])[0].contains(".name"));
}

#[test]
fn username_without_token_is_rejected() {
    assert!(run(&[registry("r", Kind::Ghcr, None, (true, false))])[0].contains("вместе"));
}

#[test]
fn token_without_username_is_rejected() {
    assert!(run(&[registry("r", Kind::Ghcr, None, (false, true))])[0].contains("вместе"));
}
