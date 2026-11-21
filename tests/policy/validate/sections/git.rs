use dcd::policy::sections::git::Repo;
use dcd::policy::types::interval::Interval;
use dcd::policy::validate::sections::git::check;

fn run(url: &str, branch: &str, secs: u64) -> Vec<String> {
    let repo = Repo {
        url: url.to_owned(),
        branch: branch.to_owned(),
        poll_interval: Interval::from_secs(secs),
    };
    let mut issues = Vec::new();
    check(&repo, &mut issues);
    issues
}

#[test]
fn valid_repo_has_no_issues() {
    assert!(run("https://example.com/r.git", "main", 60).is_empty());
}

#[test]
fn empty_url_is_rejected() {
    assert!(run("", "main", 60)[0].starts_with("git.url"));
}

#[test]
fn blank_branch_is_rejected() {
    assert!(run("https://example.com/r.git", "  ", 60)[0].starts_with("git.branch"));
}

#[test]
fn zero_poll_interval_is_rejected() {
    assert!(run("https://example.com/r.git", "main", 0)[0].starts_with("git.poll_interval"));
}
