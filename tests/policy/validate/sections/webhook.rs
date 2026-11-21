use dcd::policy::sections::webhook::Webhook;
use dcd::policy::validate::sections::webhook::check;

fn run(secret_env: &str) -> Vec<String> {
    let hook = Webhook {
        listen: "127.0.0.1:9000".parse().unwrap(),
        secret_env: secret_env.to_owned(),
    };
    let mut issues = Vec::new();
    check(&hook, &mut issues);
    issues
}

#[test]
fn named_secret_is_valid() {
    assert!(run("DCD_SECRET").is_empty());
}

#[test]
fn empty_secret_name_is_rejected() {
    assert!(run("")[0].starts_with("webhook.secret_env"));
}
