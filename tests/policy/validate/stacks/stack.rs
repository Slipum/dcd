use dcd::policy::load::from_str;
use dcd::policy::validate::stacks::stack::check;

const OK: &str = "  - name: blog\n    compose_file: c.yaml\n    services:\n      web:\n        image: ghcr.io/me/blog\n        track: { tag: latest }\n";

fn issues_for(stacks_yaml: &str) -> Vec<String> {
    let policy = from_str(&format!("version: 1\nstacks:\n{stacks_yaml}")).unwrap();
    let mut issues = Vec::new();
    check(&policy.stacks, false, &mut issues);
    issues
}

#[test]
fn valid_stack_has_no_issues() {
    assert!(issues_for(OK).is_empty());
}

#[test]
fn uppercase_stack_name_is_rejected() {
    assert!(issues_for(&OK.replace("name: blog", "name: Blog"))[0].contains(".name"));
}

#[test]
fn name_starting_with_dash_is_rejected() {
    assert!(issues_for(&OK.replace("name: blog", "name: -blog"))[0].contains(".name"));
}

#[test]
fn duplicate_stack_names_are_rejected() {
    let issues = issues_for(&format!("{OK}{OK}"));
    assert!(issues[0].contains("уже используется"));
}

#[test]
fn invalid_project_name_is_rejected() {
    let yaml = OK.replace("    compose_file", "    project: Bad\n    compose_file");
    assert!(issues_for(&yaml)[0].contains(".project"));
}

#[test]
fn empty_compose_file_is_rejected() {
    let yaml = OK.replace("compose_file: c.yaml", "compose_file: \"\"");
    assert!(issues_for(&yaml)[0].contains(".compose_file"));
}

#[test]
fn stack_without_services_is_rejected() {
    let issues = issues_for("  - name: blog\n    compose_file: c.yaml\n");
    assert!(issues[0].contains("хотя бы один сервис"));
}

#[test]
fn service_name_with_space_is_rejected() {
    let yaml = OK.replace("      web:", "      \"bad name\":");
    assert!(issues_for(&yaml)[0].contains("имя может содержать"));
}

#[test]
fn service_issues_carry_stack_and_service_path() {
    let issues = issues_for(&OK.replace("ghcr.io/me/blog", "app:1"));
    assert!(issues[0].starts_with("stacks[blog].services.web.image"));
}
