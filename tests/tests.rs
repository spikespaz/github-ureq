use github_ureq::{utils::RepositoryNamespace, *};
use test_case::test_case;

static API_TOKEN: &str = env!("GITHUB_API_TOKEN");
static TEST_REPOSITORIES: [&str; 13] = [
    "https://github.com/obsproject/obs-studio",
    "https://github.com/microsoft/vscode",
    "https://github.com/facebook/react-native",
    "https://github.com/graalvm/graalvm-ce-builds",
    "https://github.com/VSCodium/vscodium",
    "https://github.com/rust-lang/rust",
    "https://github.com/rust-lang/rustfmt",
    "https://github.com/zadam/trilium",
    "https://github.com/ldc-developers/ldc",
    "https://github.com/archlinux/archinstall",
    "https://github.com/microsoft/TypeScript",
    "https://github.com/nodejs/node",
    "https://github.com/twbs/bootstrap",
];

#[test_case(TEST_REPOSITORIES[0]; "get_repository_0")]
#[test_case(TEST_REPOSITORIES[1]; "get_repository_1")]
#[test_case(TEST_REPOSITORIES[2]; "get_repository_2")]
#[test_case(TEST_REPOSITORIES[3]; "get_repository_3")]
#[test_case(TEST_REPOSITORIES[4]; "get_repository_4")]
#[test_case(TEST_REPOSITORIES[5]; "get_repository_5")]
#[test_case(TEST_REPOSITORIES[6]; "get_repository_6")]
#[test_case(TEST_REPOSITORIES[7]; "get_repository_7")]
#[test_case(TEST_REPOSITORIES[8]; "get_repository_8")]
#[test_case(TEST_REPOSITORIES[9]; "get_repository_9")]
#[test_case(TEST_REPOSITORIES[10]; "get_repository_10")]
#[test_case(TEST_REPOSITORIES[11]; "get_repository_11")]
#[test_case(TEST_REPOSITORIES[12]; "get_repository_12")]
fn get_repository(url: &str) {
    let RepositoryNamespace { owner, repository } = utils::namespace(url).unwrap();
    let result = github_ureq::get_repository(owner, repository, Some(API_TOKEN));

    println!("{}\n{:#?}", url, result);
    assert!(result.is_ok());
}

#[test_case(TEST_REPOSITORIES[0]; "get_repository_releases_0")]
#[test_case(TEST_REPOSITORIES[1]; "get_repository_releases_1")]
#[test_case(TEST_REPOSITORIES[2]; "get_repository_releases_2")]
#[test_case(TEST_REPOSITORIES[3]; "get_repository_releases_3")]
#[test_case(TEST_REPOSITORIES[4]; "get_repository_releases_4")]
#[test_case(TEST_REPOSITORIES[5]; "get_repository_releases_5")]
#[test_case(TEST_REPOSITORIES[6]; "get_repository_releases_6")]
#[test_case(TEST_REPOSITORIES[7]; "get_repository_releases_7")]
#[test_case(TEST_REPOSITORIES[8]; "get_repository_releases_8")]
#[test_case(TEST_REPOSITORIES[9]; "get_repository_releases_9")]
#[test_case(TEST_REPOSITORIES[10]; "get_repository_releases_10")]
#[test_case(TEST_REPOSITORIES[11]; "get_repository_releases_11")]
#[test_case(TEST_REPOSITORIES[12]; "get_repository_releases_12")]
fn get_repository_releases(url: &str) {
    let RepositoryNamespace { owner, repository } = utils::namespace(url).unwrap();
    let result = github_ureq::get_repository_releases(owner, repository, Some(API_TOKEN));

    println!("{}\n{:#?}", url, result);
    assert!(result.is_ok());
}
