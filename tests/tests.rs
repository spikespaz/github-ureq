use test_case::test_case;
use url::Url;

use github_ureq::*;

static TEST_REPOSITORIES: [&str; 12] = [
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
];

#[test_case(TEST_REPOSITORIES[0] ; "get_repository_0")]
#[test_case(TEST_REPOSITORIES[1] ; "get_repository_1")]
#[test_case(TEST_REPOSITORIES[2] ; "get_repository_2")]
#[test_case(TEST_REPOSITORIES[3] ; "get_repository_3")]
#[test_case(TEST_REPOSITORIES[4] ; "get_repository_4")]
#[test_case(TEST_REPOSITORIES[5] ; "get_repository_5")]
#[test_case(TEST_REPOSITORIES[6] ; "get_repository_6")]
#[test_case(TEST_REPOSITORIES[7] ; "get_repository_7")]
#[test_case(TEST_REPOSITORIES[8] ; "get_repository_8")]
#[test_case(TEST_REPOSITORIES[9] ; "get_repository_9")]
#[test_case(TEST_REPOSITORIES[10] ; "get_repository_10")]
#[test_case(TEST_REPOSITORIES[11] ; "get_repository_11")]
pub fn get_repository(url: &str) {
    let url = Url::parse(url).unwrap();
    let (owner, repository) = url
        .path()
        .strip_prefix('/')
        .unwrap()
        .split_once('/')
        .unwrap();

    let endpoint = format!("https://api.github.com/repos/{}/{}", owner, repository);

    let response = ureq::get(&endpoint)
        .call()
        .unwrap_or_else(|_| panic!("request to '{}' failed", endpoint));

    let repository: Repository = response
        .into_json()
        .unwrap_or_else(|_| panic!("converting response from '{}' failed", endpoint));

    println!("{:#?}", repository);
}

#[test_case(TEST_REPOSITORIES[0] ; "get_releases_0")]
#[test_case(TEST_REPOSITORIES[1] ; "get_releases_1")]
#[test_case(TEST_REPOSITORIES[2] ; "get_releases_2")]
#[test_case(TEST_REPOSITORIES[3] ; "get_releases_3")]
#[test_case(TEST_REPOSITORIES[4] ; "get_releases_4")]
#[test_case(TEST_REPOSITORIES[5] ; "get_releases_5")]
#[test_case(TEST_REPOSITORIES[6] ; "get_releases_6")]
#[test_case(TEST_REPOSITORIES[7] ; "get_releases_7")]
#[test_case(TEST_REPOSITORIES[8] ; "get_releases_8")]
#[test_case(TEST_REPOSITORIES[9] ; "get_releases_9")]
#[test_case(TEST_REPOSITORIES[10] ; "get_releases_10")]
#[test_case(TEST_REPOSITORIES[11] ; "get_releases_11")]
pub fn get_releases(url: &str) {
    let url = Url::parse(url).unwrap();
    let (owner, repository) = url
        .path()
        .strip_prefix('/')
        .unwrap()
        .split_once('/')
        .unwrap();

    let endpoint = format!(
        "https://api.github.com/repos/{}/{}/releases",
        owner, repository
    );

    let response = ureq::get(&endpoint)
        .call()
        .unwrap_or_else(|_| panic!("request to '{}' failed", endpoint));

    let releases: Vec<Release> = response
        .into_json()
        .unwrap_or_else(|_| panic!("converting response from '{}' failed", endpoint));

    println!("{:#?}", releases);
}
