# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 7.0.0 (2024-10-15)

A maintenance release with various updated dependencies.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release.
 - 419 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update changelogs ([`9648129`](https://github.com/Byron/google-apis-rs/commit/9648129be46b2a0f96b599a7a21ae98ba1639938))
    - Set next major version for google-(apis|clis)-common, and the crates ([`cdacfdf`](https://github.com/Byron/google-apis-rs/commit/cdacfdf7a7b17b58dbcf5fc271a005715d6f0aa4))
    - Merge pull request #524 from IvanUkhov/yup-oauth2 ([`450748c`](https://github.com/Byron/google-apis-rs/commit/450748cf02173188a5731156f7694239899901e5))
    - Sort out new lines and trailing whitespace ([`8e80440`](https://github.com/Byron/google-apis-rs/commit/8e80440fef232c7cda0e95524878e0c75c59e3f4))
    - Start to update the CLIs ([`93a30e0`](https://github.com/Byron/google-apis-rs/commit/93a30e09e937efdc9e91217a2c83301aa470536f))
    - Update google-clis-common ([`b70df88`](https://github.com/Byron/google-apis-rs/commit/b70df8876e05acac59b16550fa248375809bb76a))
    - Merge pull request #509 from M4SS-Code/strsim ([`09580c6`](https://github.com/Byron/google-apis-rs/commit/09580c6d23d1f986ad3abb50ce4b9865af11613e))
    - Bump strsim to v0.11 and drop it from generated crates ([`3fbba31`](https://github.com/Byron/google-apis-rs/commit/3fbba3193fc451105ca4db07af409ceb3339506a))
    - Merge pull request #498 from OMGeeky/fix-clippy-to-string-display ([`f5e545f`](https://github.com/Byron/google-apis-rs/commit/f5e545fa22503d7c80647e5a7cbc1de4e8a14afa))
    - Implement Display instead of ToString ([`70e5c03`](https://github.com/Byron/google-apis-rs/commit/70e5c03a670513dc48206791d8de0047e69b0acd))
    - Merge pull request #491 from IvanUkhov/oauth2 ([`6f44ce8`](https://github.com/Byron/google-apis-rs/commit/6f44ce870262de0536879d1fb93f66faa78da95c))
    - Update yup-oauth2 to version 9 ([`8d47fd5`](https://github.com/Byron/google-apis-rs/commit/8d47fd501085f07cbf99419b527968a612657fc2))
    - Merge pull request #482 from IvanUkhov/yup-oauth-hyper-rustls ([`cf7f3fa`](https://github.com/Byron/google-apis-rs/commit/cf7f3fa860fb068caa2d720aeacf61909987751e))
    - Update yup-oauth2 to 8.3.3 and hyper-rustls to 0.25 ([`edd572c`](https://github.com/Byron/google-apis-rs/commit/edd572cc7b2e6746dbe6924dbf6e2dbfab4c960b))
</details>

## 6.0.0 (2023-08-23)

A maintenance release that bumps the yup-oauth2 crate's version.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release google-clis-common v6.0.0 ([`a27eb96`](https://github.com/Byron/google-apis-rs/commit/a27eb966cb9d001b33f0bcb990421002bec32c92))
    - Update changelog prior to common crate releases ([`0cf81f1`](https://github.com/Byron/google-apis-rs/commit/0cf81f1df4fa79b8eaffdb56d892d392dc580812))
    - Bump yup-oauth2 to the latest version, bringing in `hyper-rustls` v0.24. ([`58189b3`](https://github.com/Byron/google-apis-rs/commit/58189b31498fb324175721ab5bb46e8e12379636))
    - Merge branch 'update_yup_oauth' ([`5f601f8`](https://github.com/Byron/google-apis-rs/commit/5f601f89074d9f944aa1bc0db26ae14a0808d265))
    - Update yup-oauth2 to 8.0.0. ([`c6039c0`](https://github.com/Byron/google-apis-rs/commit/c6039c085db68835757bc5c9c09000ef5b18164a))
</details>

## 5.0.0 (2022-10-20)

### Breaking

Update `mime` dependency to 0.3 (from 0.2).

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 21 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release google-clis-common v5.0.0 ([`83072ad`](https://github.com/Byron/google-apis-rs/commit/83072adb69c576571672b310c5935d3ee9aa14e5))
    - Update apis-common crate changelog prior to release ([`065bd8d`](https://github.com/Byron/google-apis-rs/commit/065bd8d1b19f1450ff96a49981243843f8dfa2a0))
    - Merge branch 'refactor' ([`d202c95`](https://github.com/Byron/google-apis-rs/commit/d202c95aa4b6ed4c159b4b3e2f754bf176234f5c))
    - Add default impl to InvalidOptionsError ([`b706de7`](https://github.com/Byron/google-apis-rs/commit/b706de7a9569f182d022b097f8eb42677815808a))
    - Cargo clippy --fix google-clis-common ([`f4317a2`](https://github.com/Byron/google-apis-rs/commit/f4317a2968238eb0837bb88e65de4cee387d500f))
    - Update mime dependency in google-clis-common ([`85e0d28`](https://github.com/Byron/google-apis-rs/commit/85e0d284d1551d1809e6883a3a18b8ce62f7b831))
</details>

## 4.0.0 (2022-09-29)

The first release to allow CLIs to share code in a backwards compatible fashion.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release google-clis-common v4.0.0 ([`cd640a9`](https://github.com/Byron/google-apis-rs/commit/cd640a94a0f74a41982cc721d39e7681b4dc7037))
    - Explicitly version strsim dependency to allow publish ([`b8555d6`](https://github.com/Byron/google-apis-rs/commit/b8555d6982e92af21e2bb38d2de7b2f636d7b3cc))
    - Prepare changelog ([`6aab198`](https://github.com/Byron/google-apis-rs/commit/6aab1989e113a089e2c10fda48174dc9200cea8e))
    - Merge branch 'common-cli-crate' ([`4c4d96d`](https://github.com/Byron/google-apis-rs/commit/4c4d96d3c2028497de2b2e86f94f79a9d6b371bb))
    - Run only tests that are actually used/implemented ([`a1e6496`](https://github.com/Byron/google-apis-rs/commit/a1e6496ccdeaa8a0d799c53ed3b95fca5b7fc7c9))
    - Add google-clis-common crate ([`322ea69`](https://github.com/Byron/google-apis-rs/commit/322ea698c365f1a713bc54d5aae9ffb92d9e646b))
</details>

