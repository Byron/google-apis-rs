# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 6.0.1 (2024-01-05)

Added another encoding/decoding wrapper for base64.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 100 calendar days.
 - 135 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Prepare changelog prior to google-apis-common release. ([`3e0829c`](https://github.com/Byron/google-apis-rs/commit/3e0829caf73a3bb2f7430bb2fc8109d43d5b61d9))
    - Merge pull request #464 from andrewbaxter/standard-base64 ([`23aecc3`](https://github.com/Byron/google-apis-rs/commit/23aecc38e8b03ab3d50641e5581aca63dde76cb3))
    - Attempt to add 'standard' base64 bytes support ([`ee4ed07`](https://github.com/Byron/google-apis-rs/commit/ee4ed07af102f7ae5f685b78327737b233a4023e))
</details>

## 6.0.0 (2023-08-23)

A maintenance release with updated yup-oauth2 dependency

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 126 calendar days.
 - 126 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release google-apis-common v6.0.0 ([`573d543`](https://github.com/Byron/google-apis-rs/commit/573d5437a379c19b9c13be1f8b1251ad135a2e9e))
    - Update changelog prior to common crate releases ([`0cf81f1`](https://github.com/Byron/google-apis-rs/commit/0cf81f1df4fa79b8eaffdb56d892d392dc580812))
    - Bump yup-oauth2 to the latest version, bringing in `hyper-rustls` v0.24. ([`58189b3`](https://github.com/Byron/google-apis-rs/commit/58189b31498fb324175721ab5bb46e8e12379636))
</details>

## 5.0.4 (2023-04-19)

### Bug Fixes

 - <csr-id-f5b37c3f71989c25bff9d7954ede21c3455e56cb/> pin `yup-oauth2` to the latest known working version of yup-oauth2.
   It comes with hyper-rustls 0.23 which the currently released batch of
   google crates requires.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release google-apis-common v5.0.4 ([`343b479`](https://github.com/Byron/google-apis-rs/commit/343b4792088e3eaa4562050ee5a7c56f9ed26268))
    - Pin `yup-oauth2` to the latest known working version of yup-oauth2. ([`f5b37c3`](https://github.com/Byron/google-apis-rs/commit/f5b37c3f71989c25bff9d7954ede21c3455e56cb))
</details>

## 5.0.3 (2023-04-19)

### Bug Fixes

 - <csr-id-0be367a4c116959ba9391679b0859d0b8985b7cc/> update `hyper-rustls` to latest version for compatibility with latest `yup-oauth2`.
   This works with `yup-oauth2` v8.2. Please let me know if it breaks again
   and we can keep following it with patch releases.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 55 calendar days.
 - 136 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release google-apis-common v5.0.3 ([`843f3dc`](https://github.com/Byron/google-apis-rs/commit/843f3dcf1f1fae352613d9a256f11e58e535498c))
    - Update `hyper-rustls` to latest version for compatibility with latest `yup-oauth2`. ([`0be367a`](https://github.com/Byron/google-apis-rs/commit/0be367a4c116959ba9391679b0859d0b8985b7cc))
    - Chore: update google-apis-common Cargo.toml ([`d9ce1ea`](https://github.com/Byron/google-apis-rs/commit/d9ce1eaf969d27a63c8d43aeabfebde54273733e))
    - Replace a few unwraps() with `?` where possible ([`6e0caa3`](https://github.com/Byron/google-apis-rs/commit/6e0caa3b220df3939d0b2cd2b4789219f9e8fcfd))
    - Fix clippy ([`ad51cb9`](https://github.com/Byron/google-apis-rs/commit/ad51cb96a5bbc72df38b3202b7a69aeed42bcb88))
</details>

## 5.0.2 (2022-12-03)

### Bug Fixes

 - <csr-id-a6574486ba7cae72951f9d8c2c555b38d7279306/> remove old time dependency from API.
   Chrono currently depends on an old version of time with a reported
   vulnerability: https://rustsec.org/advisories/RUSTSEC-2020-0159
   
   While it does not use any vulnerable code, the dependency may show
   up in code vulnerability scans, etc.
   
   This removes the "oldtime" feature from chrono, to remove that.
   Also removes the "std" feature because it doesn't seem to be in use
   in this code.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 3 calendar days.
 - 44 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release google-apis-common v5.0.2 ([`ce4d51f`](https://github.com/Byron/google-apis-rs/commit/ce4d51fb99cc957f58b9effce00072851497d1a9))
    - Remove old time dependency from API. ([`a657448`](https://github.com/Byron/google-apis-rs/commit/a6574486ba7cae72951f9d8c2c555b38d7279306))
    - Remove old time dependency from API. ([`80ba514`](https://github.com/Byron/google-apis-rs/commit/80ba5140331fecd75dc46debded09166aca812c4))
    - Merge branch 'update_yup_oauth' ([`5f601f8`](https://github.com/Byron/google-apis-rs/commit/5f601f89074d9f944aa1bc0db26ae14a0808d265))
    - Update yup-oauth2 to 8.0.0. ([`c6039c0`](https://github.com/Byron/google-apis-rs/commit/c6039c085db68835757bc5c9c09000ef5b18164a))
</details>

## 5.0.1 (2022-10-20)

### Documentation

 - <csr-id-4d30072c9152d105034eb121a89c79410933cc92/> min 1 try + retries in example

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 20 commits contributed to the release over the course of 2 calendar days.
 - 10 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release google-apis-common v5.0.1 ([`ca8ad69`](https://github.com/Byron/google-apis-rs/commit/ca8ad69d91f189355a6303730ac464501c945112))
    - Merge branch 'refactor' ([`d202c95`](https://github.com/Byron/google-apis-rs/commit/d202c95aa4b6ed4c159b4b3e2f754bf176234f5c))
    - Cargo fmt ([`4bdd77a`](https://github.com/Byron/google-apis-rs/commit/4bdd77a52ffa57c6cf6649f7973f5f1eee9e9d6e))
    - Impl std::fmt::Display for FieldMask ([`9285942`](https://github.com/Byron/google-apis-rs/commit/9285942f3d06039f85be8d60a19fcff12e5efdf2))
    - Cargo clippy --fix google-clis-common ([`f4317a2`](https://github.com/Byron/google-apis-rs/commit/f4317a2968238eb0837bb88e65de4cee387d500f))
    - Apply cargo clippy --fix ([`4cca633`](https://github.com/Byron/google-apis-rs/commit/4cca633f9236dc3cc1a799f562931c9f12026c9f))
    - Add FromStr impl for FieldMask ([`63793b5`](https://github.com/Byron/google-apis-rs/commit/63793b55a69a30aea17d87b29ab8b19d6d43ebdd))
    - Refactor Params into external struct ([`0ad3b12`](https://github.com/Byron/google-apis-rs/commit/0ad3b1258f55318cbfa9c4fc121291471cb65dca))
    - Add UploadProtocol enum to remove string types ([`cfa6958`](https://github.com/Byron/google-apis-rs/commit/cfa6958aa09b2e7ae9ad7da551abc011f38ccd79))
    - Merge pull request #384 from philippeitis/update-mime ([`b20e630`](https://github.com/Byron/google-apis-rs/commit/b20e63072317775e239b2400099ce1705f1c076c))
    - Make MultiPartReader::mime_type associated fn ([`f6b195d`](https://github.com/Byron/google-apis-rs/commit/f6b195df4f52bc13f711cadb236cba56890080ff))
    - Update mime ([`616b324`](https://github.com/Byron/google-apis-rs/commit/616b324a7796173534f97302fcdae6bae0381ad2))
    - Merge branch 'auth-refactor' ([`fb884e1`](https://github.com/Byron/google-apis-rs/commit/fb884e193a191a671602562ae8d82cd9fb6026ac))
    - Update documentation ([`7a114a6`](https://github.com/Byron/google-apis-rs/commit/7a114a6d1c8808abff3fc2dc9817d4ca89222450))
    - Min 1 try + retries in example ([`4d30072`](https://github.com/Byron/google-apis-rs/commit/4d30072c9152d105034eb121a89c79410933cc92))
    - Use Result<Option<_>, _> ([`a375b71`](https://github.com/Byron/google-apis-rs/commit/a375b710b1debaef08583bfc3160c120076d0ebe))
    - More correct GetToken docs ([`1132b54`](https://github.com/Byron/google-apis-rs/commit/1132b542d2590e0bccf588691d95e47148bc059d))
    - Make yup-oauth2 optional ([`7ca7f1c`](https://github.com/Byron/google-apis-rs/commit/7ca7f1cafda0292e888e8faff296ab1c3c4400bb))
    - Document auth.rs ([`50dd53a`](https://github.com/Byron/google-apis-rs/commit/50dd53a87716547ae6bf829e1baef5500b218cef))
    - Return Option<String> from GetToken::get_token instead of Result<...> ([`32110d6`](https://github.com/Byron/google-apis-rs/commit/32110d6970ae9c16d6889fc5c1fe65556fa3d469))
</details>

## 5.0.0 (2022-10-10)

### New Features

 - <csr-id-3625188eb5dac9f6dfe8e672a65e75dcb0810081/> apis-common for the 5.0 line of google crates.
   It aids with better type decoding support.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release over the course of 2 calendar days.
 - 8 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release google-apis-common v5.0.0 ([`1855e44`](https://github.com/Byron/google-apis-rs/commit/1855e44a9509f47ada83c49e0b1262fbb4e1eace))
    - Apis-common for the 5.0 line of google crates. ([`3625188`](https://github.com/Byron/google-apis-rs/commit/3625188eb5dac9f6dfe8e672a65e75dcb0810081))
    - Merge branch 'format_types' ([`ed5dab2`](https://github.com/Byron/google-apis-rs/commit/ed5dab2dbd1aa8d126b1253e8020bcf964f0ee34))
    - Use correct string impls for http headers ([`1c04f66`](https://github.com/Byron/google-apis-rs/commit/1c04f662d17fd131c9342bb4ccccb728e03b9376))
    - Support serde for arbitrary field types ([`f6cced9`](https://github.com/Byron/google-apis-rs/commit/f6cced960594d680c2335c5740c331b99e46bdcc))
    - Fix cargo check w.r.t. FieldMask ([`8cc2707`](https://github.com/Byron/google-apis-rs/commit/8cc27075638e005448defd31fbf8ec3e445dd7b0))
    - Add #[serde(default)] for Option parsing ([`ddac761`](https://github.com/Byron/google-apis-rs/commit/ddac761e06dd2bd6c544bfafc8f058871aa3ec04))
    - Add base64 round trip test ([`8809ec4`](https://github.com/Byron/google-apis-rs/commit/8809ec4807385e2c90fa2093885e67567f21a94f))
    - Add FieldMask and serde impl ([`afb96bd`](https://github.com/Byron/google-apis-rs/commit/afb96bd264c91372b9900360d9d060ea38c7a31e))
    - Add serde test cases ([`928c602`](https://github.com/Byron/google-apis-rs/commit/928c6027e65aed1fb71269571fecdf20e5b0a57c))
    - Serde cleanup ([`7662741`](https://github.com/Byron/google-apis-rs/commit/76627413a34ff335a74750ba3f2cbc8999d4c6f3))
    - Refactor serde functionality into separate module ([`5398dc6`](https://github.com/Byron/google-apis-rs/commit/5398dc6f7945ee08071c8131ed3fb02163342a60))
    - Fix type signatures ([`477be5d`](https://github.com/Byron/google-apis-rs/commit/477be5d76c1f33a1df6796684b4a29b59bd0ccfd))
    - Use chrono::Duration directly with serde attributes ([`05df68d`](https://github.com/Byron/google-apis-rs/commit/05df68de324561773b9867f143bf7dbb4381f603))
    - Add proper error handling for parsing Duration ([`444b610`](https://github.com/Byron/google-apis-rs/commit/444b610ddc62af31c9c95e6865b8e58452afb10f))
    - Clean up duration parsing code ([`fc78001`](https://github.com/Byron/google-apis-rs/commit/fc780014d4e94b7a01c5237feb21999c86474cc1))
    - Use appropriate types for date-time, duration, bytes ([`29aa8df`](https://github.com/Byron/google-apis-rs/commit/29aa8df15b0ee08b7dea83f9a25cd9d6c2304b99))
    - Add support for duration and base64 serde ([`66c535e`](https://github.com/Byron/google-apis-rs/commit/66c535e4d67b2ebcb726eaad4cb6daf24c650ecc))
</details>

## 4.0.1 (2022-10-02)

`+Send` trait bound for `GetToken`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 3 calendar days.
 - 6 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release google-apis-common v4.0.1 ([`93c8601`](https://github.com/Byron/google-apis-rs/commit/93c8601fdca62f92f164957d9e86220f4f1b10f7))
    - Prepare release of google-apis-common ([`dc0686e`](https://github.com/Byron/google-apis-rs/commit/dc0686ea0b4cf6984992930ba531407a3e9e2666))
    - Merge branch 'patch-2' ([`fe1e972`](https://github.com/Byron/google-apis-rs/commit/fe1e97299f5cb025d6e4d18a1864356bcb6f5244))
    - Merge branch 'patch-1' ([`6548d38`](https://github.com/Byron/google-apis-rs/commit/6548d384837316a6b40c2f02f581c89dc400181a))
    - Replace std::thread::sleep with tokio::thread::sleep ([`811ed3d`](https://github.com/Byron/google-apis-rs/commit/811ed3d016c7465e95628b91c0828837aeef2f88))
    - Make remove_json_null_values O(n) instead of O(n^2) ([`276324a`](https://github.com/Byron/google-apis-rs/commit/276324ae09f4fa60c65518991cf89ce6bd7900ac))
    - Add additional send/sync bounds ([`dad761d`](https://github.com/Byron/google-apis-rs/commit/dad761d3f8278e014cf51271ef0ab45495f5d86a))
    - Update lib.rs ([`5a5d476`](https://github.com/Byron/google-apis-rs/commit/5a5d476317d6491d9c23bc8c9e5740f392bec7c0))
    - Add Send bound to GetToken ([`b10bdda`](https://github.com/Byron/google-apis-rs/commit/b10bddab08e3ae42a23600235e09d35a71e547bd))
    - Merge branch 'common-cli-crate' ([`4c4d96d`](https://github.com/Byron/google-apis-rs/commit/4c4d96d3c2028497de2b2e86f94f79a9d6b371bb))
    - Fix compile warnings ([`3b4fae9`](https://github.com/Byron/google-apis-rs/commit/3b4fae994e82ba48c4cc809a668516f44a367e5e))
    - Run only tests that are actually used/implemented ([`a1e6496`](https://github.com/Byron/google-apis-rs/commit/a1e6496ccdeaa8a0d799c53ed3b95fca5b7fc7c9))
    - Move client.rs to common dependency ([`9142859`](https://github.com/Byron/google-apis-rs/commit/9142859612e00fd81a3644f587f6abb9a8d837a9))
</details>

## 4.0.0 (2022-09-25)

<csr-id-38f086ebb59b16d9e6be6747a38def03ae114aa0/>
<csr-id-f2363df5b5014e265647f33124ba4d0af6c7144a/>
<csr-id-4603769ca6fb33877dd15eedf5ea3dd1fbcdb152/>
<csr-id-b7d2e021f104a84914137e02ee1f0abf3ad082bd/>
<csr-id-7ef8049d2907b54f71566d5e8fb69538deed3a0c/>
<csr-id-a0d6551deab6017ef1904eaf48ced0627eb29ae2/>
<csr-id-9e92a3c1880d35e9da529e4cc44400d9d052a91f/>
<csr-id-9330ca140a82561da406d4f9ba1f4a4421d7e92c/>
<csr-id-eff8aac1edd20d83e7272e604440e978a3d1282b/>
<csr-id-d6d8d6037e853a2edd8358438de188ec91cdbdb0/>
<csr-id-deaf8f5049c5abadd855a7d9a9f4dd86bbc8e5b6/>
<csr-id-92c80e238ec3c3ab3b2e65944d755ed7cdc6949d/>
<csr-id-282eb1c41728d4a0628ef00aa257d91b3a1cdfd2/>
<csr-id-77c26b7e6c08a16814b547335995565aca25509f/>
<csr-id-f947a4552fad3d1c27b13cc3636922f6d170a9b2/>
<csr-id-1dd5eadac1cd9a8aea817067f6ef3c968feab794/>
<csr-id-fc4ff6fe8b31b2a6c49ab1de4f4e74e0675d3a81/>
<csr-id-9cff80836244ec600dafac1994943ed562107af0/>
<csr-id-83b916e8f94112effb3c213eaae38d78cb1aa291/>
<csr-id-d0eb8f3d9a8aa5cfdf9527a7989cc457f9d69612/>
<csr-id-87c15a7406bb84d71255942a7d47cdf34fcecfc6/>
<csr-id-d18714e9b2a76cebbe8c6004cddb95e77b9e04be/>
<csr-id-6cad082b7632393cf1415c48023a4733534482cb/>
<csr-id-364fc9eb185c8c2887cf9dd87bd1ce7a11efb575/>
<csr-id-26f57948a6631d42570f96498c2418ab457fcf4b/>
<csr-id-1756d7dec47a5080cd90169b36ee226ea5f4a0e4/>
<csr-id-0e6ecb997c60ef6927bbb514f783a2f30eef3952/>
<csr-id-8876209143c06bb164bdf4d297667ab0467de161/>
<csr-id-d3c1faf1f4900373f50004a9388200e96c1a06c9/>
<csr-id-fd6815997d9429af2a744b87eaea8e316e4e7708/>
<csr-id-7a4d59d19768377a7e14cc90b66674293bd1a0a8/>
<csr-id-ab4410d9130ef50df7fe100c2b7d04420b189fd3/>
<csr-id-ac325e4de06bbab638b90a1698c028f25569a4ec/>
<csr-id-629b1c21438b31976f20e2761eb6f9b30abaa036/>
<csr-id-64be0168811a42bc9820b44aed8f88b7a60524ad/>
<csr-id-7fe6e698ba0de70aa96cffa803cdb7f705fc9843/>
<csr-id-73038b2c668dbbbe1a1b8e82dd1246d031ec0b35/>
<csr-id-75316f4c8400bd778050c44b1574babb074fa82c/>
<csr-id-52a1dd23c12cadc809297e2561bf9212954ea0c1/>
<csr-id-684233ccee8ef89e17332f2992b80c5ffbc1bf4a/>
<csr-id-a0a264f4c36da6e26515b1399bf75ca199c14316/>
<csr-id-ad919460cdcadfa4c812bee54563138c21da554f/>
<csr-id-06caa1de02a9a6b8b8d843bb45bad02efd7a9790/>
<csr-id-e04b6d023dcc906b9135037355ae05e9ef043e1b/>
<csr-id-d9970513cbf27f2a2128cac843abbff350ecc9c8/>
<csr-id-7cf4034a415b14195d54e79cd2781e7f175cdbad/>
<csr-id-dd63b0fa672b106890a2086f0d997cd965517fc9/>
<csr-id-c3bd076c0faf39e2b07662ce2407c4cb61931a35/>
<csr-id-0d3adb658b47bb9e1bdd6b370e738aad06e238d4/>
<csr-id-4258fd559035952b9a7c9e1f3292833fdb03557c/>
<csr-id-96e07a35da8e06253228f16f473f571d80acd112/>
<csr-id-43d028950e8f3b1d1b7f001161de66feab314bc8/>
<csr-id-73f0e83086ba168f354a3395911409b4f2b91856/>
<csr-id-bc582e57612821e71ec056d4f213db3a4ff2bb83/>
<csr-id-8f59b9ba79668223214a7f0ec6362b17e09c760e/>
<csr-id-36db66bf3ce1881b4e352bcc934917d4e4926b4e/>
<csr-id-4a92a47bed1f61d52453192e6ab2c823b860f17b/>
<csr-id-569e8029e1dd9c7c2d45325ad6f034955534d8e6/>
<csr-id-93d053b2d6d7a51db7ee2e2c53b66ecf6f45f1dd/>
<csr-id-c4c49015f507ee0d80afef3cdd02dd44751c1dcd/>
<csr-id-d2495405c5cfcf2d761bfc95b697789bbcc7774b/>
<csr-id-8a20d778a93258a08f2f062bec9e281ee402876a/>
<csr-id-45d86f31f2f8aaf3a67f2c39c928435407f27ddc/>
<csr-id-13ed4eaecb827099d8000dc9733e39709ef02342/>
<csr-id-33771a6dc7137be3aa47cd4329859814afaa7cd0/>
<csr-id-df2dc4784aaa988c7c1e2bc2598b538fa33e8aec/>
<csr-id-527ffa35c23186a1253365448469030aae313b0c/>
<csr-id-2a2e7bfc9a6ae870acc2901527fb69c41895fb08/>
<csr-id-38c3d9a34dcdb947324b02876a5d2f3ef4e614e2/>
<csr-id-2b37fc4d353f8a6f139eef70cc5254140675facc/>
<csr-id-b9f237eec04ca2547cc8c53deb149e88aea1d2cb/>
<csr-id-e7721ce53bafc70ea4de3d14920739649c06c492/>
<csr-id-ae276438ae7dbc4cb141c8f216834435976e80f0/>
<csr-id-091d3f7e81de1f4ca2a6e07da836667c972b052d/>
<csr-id-5d1039e85fc1e8edada512ec16969e5fec901bb1/>
<csr-id-c6f92057582c576129db6f8ca27ee824df201c5d/>
<csr-id-864fe8424d83889ebe3aff69f0ada27c43a5bbc3/>
<csr-id-e5dc49f87411377a6b655efdffde14159cea5fc8/>
<csr-id-3e2216c4454d34a6ef080c5bb7d64b0b451bdecd/>
<csr-id-95e9187c842c8c301bfff8a3c228e9da189f580a/>
<csr-id-320d769c6f97460547395990efc07a441be9a927/>
<csr-id-8d7a49891f8e6db1c528ce4b212029612c077472/>
<csr-id-ca5dca7af93f7feef1b81237a9c7c1d5b07e1577/>
<csr-id-8aefeb37d98e49be7f2ad88597f5e93087d99aa6/>
<csr-id-e0de1b4c108ebfd37f6b7ebea4bf1109445d2661/>
<csr-id-930ce6d5c2bf9776b4fdcb8c8dd00753cf7aa47d/>
<csr-id-d2c12c296410be706c66bab824543f32be38da2d/>
<csr-id-a25b593969c972586a71101f1e6866b22db6c89e/>
<csr-id-df301c1c75fd60239fc93c50e6a6a4dd9d627f9d/>
<csr-id-b3fd15edec924c844f02aea4adfdcbe57d06681f/>
<csr-id-b0c0196f502a93d42053f9e6d54999e648631ac6/>
<csr-id-5094f61c883cb05034b3d95171bc63af027e97b5/>
<csr-id-be0faf0e1dd02c2efe4e8f903b948ae38596f04a/>
<csr-id-ab672c41f97ee1aa5e1ef69c68d3cfeebf9a9774/>
<csr-id-426b096ef8ebea1bf34b4b7bdbb2660ef7e7f921/>
<csr-id-52e58154a2c5d151871894ca86e67743aa5d0cc8/>
<csr-id-8732b5f8694a7c1f489347d12b79d72f4c68882e/>
<csr-id-1e5a6bbee8b461595269dc60b9cd92a6e2fc606d/>
<csr-id-900c2bfde917383295761e6b743ce465e242333a/>
<csr-id-75076acf16fc7263bea4b6b4fe063ef38cd6a44c/>
<csr-id-5cba22f0c6717574af77756324eae08431dd3cdb/>
<csr-id-06e1d4bff6311cda4cd7feb2e94a921bf26cd5e5/>
<csr-id-8295bf3c2dc310aeae293dc7287e2efe8d651abd/>
<csr-id-e3f4fcadad5310ad79d91a1ce449e1ee9f3d0c74/>
<csr-id-e1772d979d98c7d0d5c7fe14f0e81d6b5cc7ee18/>
<csr-id-6a7654d433acd39b26ada579664a3833ef28a708/>
<csr-id-5612d004b9fab6be352b4013f882a7a1dee2eb6b/>
<csr-id-862842f62166f2c84efa24cf1e09ef644e3ceacf/>
<csr-id-a8d333f91681010f9c5a52c50339d9d36ea49dc3/>
<csr-id-aecda18821cb39edc8ccb6dd3f286b1c4ce626bc/>
<csr-id-08d65ba62b025fac9c2db3db3842b86510c03d51/>
<csr-id-62b63b251ac13fb945c444bd100b97fc7846a3c8/>
<csr-id-e46535917ceeb9ba8cce2cbab39ae1e3f82942ac/>
<csr-id-defbeaa630661400e212f598c98bc60596afa5fb/>
<csr-id-6393bbf7f69f63e1291dc5b2cbd71a79bfdac2cc/>
<csr-id-337f167e6c9cfd464231bbbac9ea4503f9c77e00/>
<csr-id-7d58d66025ea58088950a0914bfb910a35a16748/>
<csr-id-1180314275b62979376ba5ebacf34763ef6ca610/>
<csr-id-31f22b1535feaa031586bdb6d16e2a306fd62a38/>
<csr-id-d0fb7a5ccc5729303b5aca2419ac06abf12e0133/>
<csr-id-d1cf8360f979df5c53fe32a34c0cddc246a072f1/>
<csr-id-3484fecf9cf19c0a09f6241f8beea2d62d637cf7/>
<csr-id-d6ddff240d1fefac28549efad78648d98e4ed9a4/>
<csr-id-1f9dc06a57f45c8be216602661b68a0adce5beca/>
<csr-id-ed0debe999c201a82be5134b3dbec7bd5400085e/>
<csr-id-e434563215b6b8cddc0aaa6a6c5ef48d6e7aedbb/>
<csr-id-5e5f0dcc662160a378387a91a0719407dde503c9/>
<csr-id-296debda85e0afb22261b61ab671325f539e01ff/>
<csr-id-9ba25af48b8c7794bb8f28475b821fdf957a0efd/>
<csr-id-23119a06d263f130d9797f73c12c34eed3ba5102/>
<csr-id-ad6dd7758e08e72cf3df3cee87df41286e9d5f0a/>
<csr-id-1d44d794eba6d7c371c10205e6f1d8b416748035/>
<csr-id-3e70a896742dd682a55394be6936811eaad27111/>
<csr-id-9e6c9537a527528debd0c68a5fe4494291facdbd/>
<csr-id-8375dd0508c6e09761f79f8b47379cc240a0c7c4/>
<csr-id-294da41a308e4f4125db876c5b24084ae8cfcb5f/>
<csr-id-362781e6014a2b050f69f5487b33aba8434c14a0/>
<csr-id-5ebb34ed5839c6c9f588a95e5661b68d07c02d4d/>
<csr-id-ab2290192ea790744851deb60e396ca908b6d190/>
<csr-id-fb80b056ac6cc8eee1972eec979a7f810861f8b8/>
<csr-id-d8acd607aea2e4bed548695e46fe541cb72ee904/>
<csr-id-cc1bfd19c8aecb4a62bf68f3bf7db650eb8fc29d/>
<csr-id-5165ff68dfe3d67f3bdcf8fee0a0d3aedee0fa33/>
<csr-id-caaf62e51dae4b3f5a405008958a60a83c9aed43/>
<csr-id-4bfdc9fd015b95bf9f3bcd311818de6cee342c9e/>
<csr-id-9abf0eb64a96544b6db313690e4b3267c6987df7/>
<csr-id-2c2585d16d5dd346fb16809db05495be93ab571b/>
<csr-id-0bc6d216c3be583cfc58143e136a16e573d7277f/>
<csr-id-9377220c59cf9a8a29720b0528b9263e9e947580/>
<csr-id-7b81646f43c1c4de5165c5b3e9e7ea5c836eb664/>
<csr-id-874cfb6f680cd601271befa66e86a54c59e3618b/>
<csr-id-8ba6acb88bf889d41560ccc2c16f5e884af68b9c/>
<csr-id-819e1ccce5c503329bf6ed5dd9078553a48997c5/>
<csr-id-d202f9792ba0aea107213ad33ce5e7da06145ef1/>
<csr-id-2740810b2aa40517f7492f7c67dca7a08b017600/>
<csr-id-aaac92bad68634923c554f4f2e8bc28769a47e84/>
<csr-id-c4e363d94ce1715f3ecfe6fd6ee56b93c670bfb2/>
<csr-id-0337435cd44105749cb219cc75d61da6895d5d8a/>
<csr-id-d1ebc0ff0be566c749651321394ffb955633286c/>
<csr-id-cc30a2e20b697ca318bd3b54e5b94f6935eaadd2/>
<csr-id-6279fd8f5df3ca9c9013635c36041e89df902428/>
<csr-id-09805e59adba84d83184ef02f27abbb054359c45/>
<csr-id-fad9d3b0ca3f588f65faf6ec46caf51a7ca1c239/>
<csr-id-fdc0141fbcabf68ed5d715314d483469e7a7ef14/>
<csr-id-d6accb8f6194bac7f982ee93409821436dd8beed/>
<csr-id-fc34337ee4ba708f63e3d2f164660edd5ffe5614/>
<csr-id-f31ef51a61cf8e1343e4ab956d8be29271976e59/>
<csr-id-850e115e33e5da9fe6266718e4cf04c23e554d2b/>
<csr-id-87dcf06eacc4fef9ed5bdec99fbb589c3d81666f/>
<csr-id-b35a1d6732022a41b846d6a8bcee8ecae940d260/>
<csr-id-495ecef8c8fcda27b08833df9fcfca503fa65002/>
<csr-id-da78e9fa4d68d772323c7a2927b8004b8ac5d1a8/>
<csr-id-7754a160c9fff6fb3796982cb2cc284c033d1008/>
<csr-id-152cdd848a41109819d890560d26270bd08c12ae/>
<csr-id-6e669ced2aca094b246c2c0eb805b362924112b1/>
<csr-id-129fd38e003d2ab23bad2ceb84f59bb74b4ae45b/>
<csr-id-e92f440d9b980c80c31c04752a8fe0c21fa36585/>
<csr-id-69b12104a9f9579773553825f63c321e7d1a6899/>
<csr-id-e86e55cae788506a2280816009b8620bad091477/>
<csr-id-5894c8163afa9f9d9bed592e7e41912c77cf993d/>
<csr-id-26314e743e2c4f38eb6c5824bf51209099000f9f/>
<csr-id-0d655411ab9c81808f683c0ad26a1eb927cdde46/>
<csr-id-05b442d5893ef46ddaf52306aef5807776bc1d05/>
<csr-id-89343654018fb1a2fb3f6955f5f0e1c3eb4fe0bd/>
<csr-id-d0b69af41390df40f5a11d44e08d1b67167a969a/>
<csr-id-464394af22714fee650ca3e310336584666f921a/>
<csr-id-f83dff672bc5a739f1a4b76333e25d40523fbe2c/>
<csr-id-5c284e1c418d93bca7da4a29c4f8feaf5800c1ce/>
<csr-id-15daf311ea79a95baf5b28760c88fbbff63a450b/>
<csr-id-2485343caa621bed4cca0df329abda7e61df813d/>
<csr-id-ef63790422db56158e2e1a6d651e329e14cd7ec0/>
<csr-id-f1fe6bac018c2268d10233ec1635f0273f1192dc/>
<csr-id-bbab1f2e38f4445179e7385a9507098d6ff15cbf/>
<csr-id-f7740ad149d78b4642670ff35deb6163ab56be22/>
<csr-id-137ba8caf3b9bad5bb7d8e4a9fb236e9988476f2/>
<csr-id-76827ff6659d33b7b9430e4971a7189fa0d23798/>
<csr-id-5b5ad43bfa06f5c525a7e00b537381cefe6b7aa4/>
<csr-id-a2550d11811de9f9ee51652975363d0f24b8d032/>
<csr-id-331ecf87a76189b10672770377d36877dbd7f53a/>
<csr-id-1dc168497ee180fa3728be290c65535ba16117e2/>
<csr-id-f1b99af5dcca4e169463a8932fcf217f9cace8c6/>
<csr-id-5bc4141fa584a247d507660bcbe46551789ad04a/>
<csr-id-c9c3ad011fdb4ae693ddeef436f7a14de35ad7b0/>
<csr-id-544be6d2a27423865d09f355785310368c4c42ed/>

The initial release of the crate that is shared among all google-api crates. This allows
trait-implementations to be re-used as they are not unique to their respective crate.

If the google-api crate you are looking at doesn't seem to use it, make sure you
have `4.0.2` or higher, or ask in the source repository for a release of the crates
you need.

### Chore

 - <csr-id-38f086ebb59b16d9e6be6747a38def03ae114aa0/> clean up after failed wget
   Wget leaves empty files when getting a 404. This causes JSON parse errors later on instead of the expected file not found message.
 - <csr-id-f2363df5b5014e265647f33124ba4d0af6c7144a/> add photoslibrary v1 code
 - <csr-id-4603769ca6fb33877dd15eedf5ea3dd1fbcdb152/> Add support for adding unlisted APIs
   The photoslibrary API is not listed however is still available.
   This adds a method of manually adding APIs to the shared.yaml and adds the photoslibary API info.
 - <csr-id-b7d2e021f104a84914137e02ee1f0abf3ad082bd/> Update virtualenv
   Update virtualenv from 12.0.7 to 16.0.0
 - <csr-id-7ef8049d2907b54f71566d5e8fb69538deed3a0c/> Pin hyper and mime crates to fix compilation.
 - <csr-id-a0d6551deab6017ef1904eaf48ced0627eb29ae2/> adds all missing apis
   With recent changes in the generator and or rust a bunch of api are now
   working that were blacklisted. This commit adds the generated files for
   those apis.
 - <csr-id-9e92a3c1880d35e9da529e4cc44400d9d052a91f/> remove working apis from blacklist
   With recent changes to rust and or the generator a bunch of api are now
   working. This commit removes them from the blacklist.
 - <csr-id-9330ca140a82561da406d4f9ba1f4a4421d7e92c/> added compute1
 - <csr-id-eff8aac1edd20d83e7272e604440e978a3d1282b/> remove compute1 from blacklist
 - <csr-id-d6d8d6037e853a2edd8358438de188ec91cdbdb0/> added sheets
 - <csr-id-deaf8f5049c5abadd855a7d9a9f4dd86bbc8e5b6/> removed sheets from blacklist
 - <csr-id-92c80e238ec3c3ab3b2e65944d755ed7cdc6949d/> regenerate apis without leading slashes
 - <csr-id-282eb1c41728d4a0628ef00aa257d91b3a1cdfd2/> v1.0.5
 - <csr-id-77c26b7e6c08a16814b547335995565aca25509f/> regen all source
   [skip ci]
 - <csr-id-f947a4552fad3d1c27b13cc3636922f6d170a9b2/> upgrade patch level
   It seems crates.io has inconsistent version information,
   making a few CLIs pull outdated crates.
   
   [skip CI]
 - <csr-id-1dd5eadac1cd9a8aea817067f6ef3c968feab794/> all api+cli
   However, it appears some APIs changed without updating their respective
   version number. Thus newer CLIs pull outdated APIs?
   Something is wrong.
   
   [skip CI]
 - <csr-id-fc4ff6fe8b31b2a6c49ab1de4f4e74e0675d3a81/> Update examples
 - <csr-id-9cff80836244ec600dafac1994943ed562107af0/> update all apis
   And bring in 5 new ones, including cloudkms.
   Also update dependencies to make things compile again.
 - <csr-id-83b916e8f94112effb3c213eaae38d78cb1aa291/> remaining publishes
 - <csr-id-d0eb8f3d9a8aa5cfdf9527a7989cc457f9d69612/> more crates published
 - <csr-id-87c15a7406bb84d71255942a7d47cdf34fcecfc6/> intermediate publish
   cargo cannot connect to github anymore for some reason
   
   [skip ci]
 - <csr-id-d18714e9b2a76cebbe8c6004cddb95e77b9e04be/> regen all apis/clis for v1.0.4
 - <csr-id-6cad082b7632393cf1415c48023a4733534482cb/> v1.0.4 - serde upgrade
 - <csr-id-364fc9eb185c8c2887cf9dd87bd1ce7a11efb575/> remove local override
   Should not have been committed in the first place
 - <csr-id-26f57948a6631d42570f96498c2418ab457fcf4b/> one step closer to getting it compiled
 - <csr-id-1756d7dec47a5080cd90169b36ee226ea5f4a0e4/> fix error handling
 - <csr-id-0e6ecb997c60ef6927bbb514f783a2f30eef3952/> latest release
 - <csr-id-8876209143c06bb164bdf4d297667ab0467de161/> regen all cli for new version
 - <csr-id-d3c1faf1f4900373f50004a9388200e96c1a06c9/> patch bump to v1.0.3
   To bring in support for hyper v0.10
 - <csr-id-fd6815997d9429af2a744b87eaea8e316e4e7708/> Update hyper for generated crates to ^0.10
   You guessed it, again related to the openssl upgrade in
   dermesser/yup-oauth2#51. As long as the API crates depend on openssl 0.7
   via hyper 0.9, any client using the APIs won't build :( For example,
   examples/drive_example/ in dermesser/yup-oauth2.
   
   Please regenerate and publish afterwards, if possible.
 - <csr-id-7a4d59d19768377a7e14cc90b66674293bd1a0a8/> published all v1.0.2
   For the sole purpose of getting the documentation onto
   docs.rs after applying a bugfix.
 - <csr-id-ab4410d9130ef50df7fe100c2b7d04420b189fd3/> v1.0.2
 - <csr-id-ac325e4de06bbab638b90a1698c028f25569a4ec/> v1.0.2
 - <csr-id-629b1c21438b31976f20e2761eb6f9b30abaa036/> cli
 - <csr-id-64be0168811a42bc9820b44aed8f88b7a60524ad/> all api
 - <csr-id-7fe6e698ba0de70aa96cffa803cdb7f705fc9843/> update code to latest version
   required before publishing
 - <csr-id-73038b2c668dbbbe1a1b8e82dd1246d031ec0b35/> update
 - <csr-id-75316f4c8400bd778050c44b1574babb074fa82c/> api-cli lock-step; depend on specific version
   As they are usually meant to work hand-in-hand anyway.
   This simplifies the way this works a lot, and is probably
   more correct as well.
 - <csr-id-52a1dd23c12cadc809297e2561bf9212954ea0c1/> v1.0.1
   * cli: now refers to just version 1 of API
   * api: updates the documentation URLs in cargo.toml
 - <csr-id-684233ccee8ef89e17332f2992b80c5ffbc1bf4a/> DS_Store
   [skip ci]
 - <csr-id-a0a264f4c36da6e26515b1399bf75ca199c14316/> get fixes into README
   This will provide a proper link to the readme.
 - <csr-id-ad919460cdcadfa4c812bee54563138c21da554f/> re-publish as much as possible
 - <csr-id-06caa1de02a9a6b8b8d843bb45bad02efd7a9790/> fetch latest json and re-gen all code
 - <csr-id-e04b6d023dcc906b9135037355ae05e9ef043e1b/> remove .DS_Store files
 - <csr-id-d9970513cbf27f2a2128cac843abbff350ecc9c8/> update
 - <csr-id-7cf4034a415b14195d54e79cd2781e7f175cdbad/> all CLIs and APIs are available in v1.0 now!
 - <csr-id-dd63b0fa672b106890a2086f0d997cd965517fc9/> all clis except for one
   google-serviceregistryalpha for some reason can't be found
   in version 1.0.0 even though it is there.
 - <csr-id-c3bd076c0faf39e2b07662ce2407c4cb61931a35/> ignore cloudtrace
 - <csr-id-0d3adb658b47bb9e1bdd6b370e738aad06e238d4/> keep state
   It uses timestamp files to remember which crates have already been
   published.
 - <csr-id-4258fd559035952b9a7c9e1f3292833fdb03557c/> add api publish notes
   That way already published crates will not be retried.
 - <csr-id-96e07a35da8e06253228f16f473f571d80acd112/> keep things stable in v1.0
   I think the current API is quite useable, therefore there is
   no need to keep it below v1 artificially.
 - <csr-id-43d028950e8f3b1d1b7f001161de66feab314bc8/> allow failure on nightly
   It appears someone in the dependency chain is pulling in a
   failing aster. Ideally, we review this or try to make a PR
   to fix this in our upstream dependencies.
   
   For now though, I want this badge green.
 - <csr-id-73f0e83086ba168f354a3395911409b4f2b91856/> use serde_derive
 - <csr-id-bc582e57612821e71ec056d4f213db3a4ff2bb83/> use yup-oauth2 1.0
   Celebrations !
 - <csr-id-8f59b9ba79668223214a7f0ec6362b17e09c760e/> use latest of yup-oauth2
   Seems to work just fine.
 - <csr-id-36db66bf3ce1881b4e352bcc934917d4e4926b4e/> Regenerate APIs
 - <csr-id-4a92a47bed1f61d52453192e6ab2c823b860f17b/> publish remaining cli
   Seems there are a few inconsistencies that needed to be
   ironed out manually.
 - <csr-id-569e8029e1dd9c7c2d45325ad6f034955534d8e6/> all cli
   Some could not be uploaded as we went out-of-memory.
   Will redo those on a stronger system
 - <csr-id-93d053b2d6d7a51db7ee2e2c53b66ecf6f45f1dd/> prepare cli for publish
 - <csr-id-c4c49015f507ee0d80afef3cdd02dd44751c1dcd/> latest APIs
   The only difference to the previous version is that they
   are referring to the latest versions of serde.
 - <csr-id-d2495405c5cfcf2d761bfc95b697789bbcc7774b/> specify version to allow cli publishing
   Let's see if we can actually get away with a '*' ... .
 - <csr-id-8a20d778a93258a08f2f062bec9e281ee402876a/> publish all APIs
   only compute1 didn't publish due to the typical
   ring-buffer error that really wants a dynamic buffer,
   instead of a static one.
 - <csr-id-45d86f31f2f8aaf3a67f2c39c928435407f27ddc/> update to latest version
   Which is to be published
 - <csr-id-13ed4eaecb827099d8000dc9733e39709ef02342/> to latest
   Using `make update-json`, all json descriptions have been update.
   Quite interesting to see that there are plenty of new ones which
   are giving 404 when queried. An actual bug, or something I should
   look into ?
 - <csr-id-33771a6dc7137be3aa47cd4329859814afaa7cd0/> version-update
   Use latest serde to make nightly builds work.
 - <csr-id-df2dc4784aaa988c7c1e2bc2598b538fa33e8aec/> one more down
   Even though docs work on this one, compilation does not.
   
   [skip ci]
 - <csr-id-527ffa35c23186a1253365448469030aae313b0c/> v0.3.6
   With simpler authentication flow.
 - <csr-id-2a2e7bfc9a6ae870acc2901527fb69c41895fb08/> rustc version no longer needed
   It now just works, and hopefully will keep doing so for a while.
   
   [skip ci]
 - <csr-id-38c3d9a34dcdb947324b02876a5d2f3ef4e614e2/> update to latest version
   [skip ci]
 - <csr-id-2b37fc4d353f8a6f139eef70cc5254140675facc/> update with latest troublemakers
   Those don't compile, usually for trivial reason, which means
   they come up with duplicate types, or have name-clashes.
   
   If there is the need, this can be fixed.
 - <csr-id-b9f237eec04ca2547cc8c53deb149e88aea1d2cb/> latest version to crates.io
   Also update the latest source-code, which is just a cleanup.
 - <csr-id-e7721ce53bafc70ea4de3d14920739649c06c492/> remove workaround marker
   ... and some left-over comments.
   
   The workaround code is actually more readable than the previous version,
   so it may as well stay.
 - <csr-id-ae276438ae7dbc4cb141c8f216834435976e80f0/> update code to latest version
 - <csr-id-091d3f7e81de1f4ca2a6e07da836667c972b052d/> increment versions ...
   ... in preparation for new publish.
   Latest flows, and all should work out of the box
   with the latest serde.
 - <csr-id-5d1039e85fc1e8edada512ec16969e5fec901bb1/> license year
   Wow, didn't know the makefile could do that :) !
 - <csr-id-c6f92057582c576129db6f8ca27ee824df201c5d/> `make regen-apis`

 - <csr-id-864fe8424d83889ebe3aff69f0ada27c43a5bbc3/> split doc and test to handle features
   Previously it would fail as the built-in doc target in make
   doesn't handle features at all.
   Now these need to be taken into consideration though for it
   to have a chance.
 - <csr-id-e5dc49f87411377a6b655efdffde14159cea5fc8/> add last known working rustc version
   For now, just nighly.
 - <csr-id-3e2216c4454d34a6ef080c5bb7d64b0b451bdecd/> use features for cli-dependency
   That way, we respect the API features when pulling it in
   via the CLI. Also make it compatible to the latest serde-json
   version.
 - <csr-id-95e9187c842c8c301bfff8a3c228e9da189f580a/> let's be sure to get the matrix right
   It's also done that way in yup-oauth2, and I forgot it
   previously. It's like a pre-emptive fix.
 - <csr-id-320d769c6f97460547395990efc07a441be9a927/> Travis CI support
   Travis should now be able to use nightly as well.
 - <csr-id-8d7a49891f8e6db1c528ce4b212029612c077472/> update to latest version + nightly support
   Nightly is now supported, in theory, to allow not to use serde_codegen,
   which currently has trouble to build thanks to an assertion error.
   
   Nightly on the other hand suffers from being build with incorrect
   feature-flags, which makes quasi_macros fail to build ... .
 - <csr-id-ca5dca7af93f7feef1b81237a9c7c1d5b07e1577/> pin `url` crate
   The latest one needs some modifications, that will be done in
   time.
 - <csr-id-8aefeb37d98e49be7f2ad88597f5e93087d99aa6/> publish state
   This helps make keep track of what was successfully published
   to crates.io
 - <csr-id-e0de1b4c108ebfd37f6b7ebea4bf1109445d2661/> to latest schema version
 - <csr-id-930ce6d5c2bf9776b4fdcb8c8dd00753cf7aa47d/> all jsons; version-up
   As we are now back to serde 0.6, the patch-level was upped too.
 - <csr-id-d2c12c296410be706c66bab824543f32be38da2d/> use compatible yup-oauth
   It seems all these serde versions interact with each other
   in unforseen ways, so they will have to be in sync for it
   to work. Its a shaky card-house I am building here,
   and I don't like it at all.
 - <csr-id-a25b593969c972586a71101f1e6866b22db6c89e/> first bunch of publishes
   Many don't work due to https://github.com/serde-rs/syntex/issues/33
 - <csr-id-df301c1c75fd60239fc93c50e6a6a4dd9d627f9d/> update after version-up
 - <csr-id-b3fd15edec924c844f02aea4adfdcbe57d06681f/> api version 0.1.12
 - <csr-id-b0c0196f502a93d42053f9e6d54999e648631ac6/> update code matching latest jsons
   This was particularly interesting as APIs went away
 - <csr-id-5094f61c883cb05034b3d95171bc63af027e97b5/> fetch latest json
   api-list.yaml was updated manually to push out APIs which are just
   empty or plain incompatible with what we are doing
 - <csr-id-be0faf0e1dd02c2efe4e8f903b948ae38596f04a/> upgrade to 0.7
   Desired feature: ignore unknown fields, which is now on by default.
 - <csr-id-ab672c41f97ee1aa5e1ef69c68d3cfeebf9a9774/> added custom client secret to allow operation
 - <csr-id-426b096ef8ebea1bf34b4b7bdbb2660ef7e7f921/> update generated code
   This seems to be just a remainder, two APIs were missed last time I
   updated the code. I guess.
 - <csr-id-52e58154a2c5d151871894ca86e67743aa5d0cc8/> remove unimplemented option
 - <csr-id-8732b5f8694a7c1f489347d12b79d72f4c68882e/> update all crates
 - <csr-id-1e5a6bbee8b461595269dc60b9cd92a6e2fc606d/> update remaining APIs
   Missed them in the first run
 - <csr-id-900c2bfde917383295761e6b743ce465e242333a/> update
 - <csr-id-75076acf16fc7263bea4b6b4fe063ef38cd6a44c/> updated to latest state
 - <csr-id-5cba22f0c6717574af77756324eae08431dd3cdb/> increment versions of API + CLI
 - <csr-id-06e1d4bff6311cda4cd7feb2e94a921bf26cd5e5/> update with latest changes
 - <csr-id-8295bf3c2dc310aeae293dc7287e2efe8d651abd/> no wildcards in dependencies
   Also version specifications in dependencies were chosen to provide
   maximum flexibility for users of the libraries.
   
   CLIs on the other hand specify last known-to-work major and minor versions
   to prevent breakage
 - <csr-id-e3f4fcadad5310ad79d91a1ce449e1ee9f3d0c74/> support for latest hyper
 - <csr-id-e1772d979d98c7d0d5c7fe14f0e81d6b5cc7ee18/> test on nightly, and run cargo test
   related to #132
 - <csr-id-6a7654d433acd39b26ada579664a3833ef28a708/> of latest API versions ... no functional change
 - <csr-id-5612d004b9fab6be352b4013f882a7a1dee2eb6b/> update everything to latest google API versions
 - <csr-id-862842f62166f2c84efa24cf1e09ef644e3ceacf/> update to latest version
 - <csr-id-a8d333f91681010f9c5a52c50339d9d36ea49dc3/> code compiles with rust 1.3/serde 0.6
 - <csr-id-aecda18821cb39edc8ccb6dd3f286b1c4ce626bc/> re-publish lib crates at latest version

 - <csr-id-08d65ba62b025fac9c2db3db3842b86510c03d51/> add source at latest version
 - <csr-id-62b63b251ac13fb945c444bd100b97fc7846a3c8/> latest json files
   This also adds a usable version of the genomics API, which was empty
   previously (and if I recall correctly).
 - <csr-id-e46535917ceeb9ba8cce2cbab39ae1e3f82942ac/> upgrade to latest hyper+clap
 - <csr-id-defbeaa630661400e212f598c98bc60596afa5fb/> configure to use wait-lock
   This allows to build everything concurrently without failure provided
   the latest cargo is used.
 - <csr-id-6393bbf7f69f63e1291dc5b2cbd71a79bfdac2cc/> latest version of v0.3.1 code
   * also fixed OSX deployment utility to deal with target folder
     as primary location for build-artifacts
   
   [skip ci]
 - <csr-id-337f167e6c9cfd464231bbbac9ea4503f9c77e00/> CLI v0.3.1
 - <csr-id-7d58d66025ea58088950a0914bfb910a35a16748/> update json and regen all code
 - <csr-id-1180314275b62979376ba5ebacf34763ef6ca610/> adjust to build.target-dir
   Thanks to the latest cargo 0.3.0, it's possible to keep everything
   in the stanard doc output directory, which essentially collects
   everything for us.
   
   This creatly reduces the space required to hold all documentation, and
   is in fact quite beatiful.
 - <csr-id-31f22b1535feaa031586bdb6d16e2a306fd62a38/> one target dir for all projects
   Starting from cargo 0.3.0, one can override the target-directory
   to be an absolute path, forcing all output to be dumped into
   one and the same target dir.
   
   That way, all dependencies are shared among the projects, saving
   4 to 7 minutes per project in debug and release mode respectively.
 - <csr-id-d0fb7a5ccc5729303b5aca2419ac06abf12e0133/> cli code update
 - <csr-id-d1cf8360f979df5c53fe32a34c0cddc246a072f1/> published latest versions
 - <csr-id-3484fecf9cf19c0a09f6241f8beea2d62d637cf7/> added latest version of api+cli
   APIs have additional files thanks to the build-script
   requirement.
   CLI has just seen minor changes though, making it
   usable with a stable compiler.
 - <csr-id-d6ddff240d1fefac28549efad78648d98e4ed9a4/> api+cli increment
   CLI was incremented to 0.3.0, just to signal usage of the latest clap-rs
   as well as the update of the used API implememtation.
   
   In that moment we also got rid of the json-tools dependency - it
   required unstable features, and I was not willing to enforce making
   it stable just yet.
 - <csr-id-1f9dc06a57f45c8be216602661b68a0adce5beca/> explicitly use stable rust
   This would be the default, but I want to be sure everyone sees
   stable is what we need.
 - <csr-id-ed0debe999c201a82be5134b3dbec7bd5400085e/> bump hyper dep to 0.5.0
   google-apis-rs no longer builds with hyper 0.4.0, due to the use of a
   now-undefined macro
 - <csr-id-e434563215b6b8cddc0aaa6a6c5ef48d6e7aedbb/> compilation without local overrides
   Also checked in code for groupsmigration to allow others to test it
   simply by checking out the right commit.
 - <csr-id-5e5f0dcc662160a378387a91a0719407dde503c9/> fix tar-handling
   Previously it would fail as '*' was in fact not substituted by the
   subshell. Now I just take the brutal route, using find.
   
   [skip ci]
 - <csr-id-296debda85e0afb22261b61ab671325f539e01ff/> typo + fixed yaml references
   [skip ci]
 - <csr-id-9ba25af48b8c7794bb8f28475b821fdf957a0efd/> change wording
   [skip ci]
 - <csr-id-23119a06d263f130d9797f73c12c34eed3ba5102/> updated for cli v0.2.0
   [skip ci]
 - <csr-id-ad6dd7758e08e72cf3df3cee87df41286e9d5f0a/> move all scripts into src/bash
   That way, they are more official than hidden scripts dumped in the
   project root.
   
   [skip ci]
 - <csr-id-1d44d794eba6d7c371c10205e6f1d8b416748035/> script to deploy for download
   A fully untested utility script allows to unpack a tar file previously
   created from `*-depoly.sh` scripts to a suitable location to be
   compatible with the downloads links we generate in the documentation
   index.
   
   Related to #107
   [skip ci]
 - <csr-id-3e70a896742dd682a55394be6936811eaad27111/> publish APIs @v0.1.7
   * keep track of publish through version files
   * updated clog configuration and changelog (automatic)
   
   [skip ci]
 - <csr-id-9e6c9537a527528debd0c68a5fe4494291facdbd/> latest version of all code
 - <csr-id-8375dd0508c6e09761f79f8b47379cc240a0c7c4/> remove special clap configuration
   In latest clap, it's all fixed
 - <csr-id-294da41a308e4f4125db876c5b24084ae8cfcb5f/> special clap configuration
   Turn off default features to disable overly red first version
   of otherwise very promising ascii-coloring support.
   
   It's good to see that thanks to yaml, that flexibility is easily
   achieved without altering any generator code.
 - <csr-id-362781e6014a2b050f69f5487b33aba8434c14a0/> added dev diary episode 2 link
 - <csr-id-5ebb34ed5839c6c9f588a95e5661b68d07c02d4d/> added information about imp
 - <csr-id-ab2290192ea790744851deb60e396ca908b6d190/> add support for 'imp'
   'imp' will resolve to 'improvements' in the changelog.
   A new feature supported in clog v0.5.0
 - <csr-id-fb80b056ac6cc8eee1972eec979a7f810861f8b8/> added changelog
   Powered by [clog](http://goo.gl/QUpyeL)
   
   [skip ci]
 - <csr-id-d8acd607aea2e4bed548695e46fe541cb72ee904/> disabled rust linter,but configured it
   It's usually too slow to run it, but besides that it's very practical
   to have !
   
   [skip ci]
 - <csr-id-cc1bfd19c8aecb4a62bf68f3bf7db650eb8fc29d/> to reflect recent changes
   And to be sure we don't forget to publish new crates when the new CLI
   hits the road.
 - <csr-id-5165ff68dfe3d67f3bdcf8fee0a0d3aedee0fa33/> using docker
   Allow building all dependencies using docker
 - <csr-id-caaf62e51dae4b3f5a405008958a60a83c9aed43/> prepare dep generation to use suffix
   That way, we can have multiple deps files, one per program 'type'
 - <csr-id-4bfdc9fd015b95bf9f3bcd311818de6cee342c9e/> make publish
   We will now keep marker files to remember which crates we have published
   successfully. These files are checked in to make this information
   persist.
 - <csr-id-9abf0eb64a96544b6db313690e4b3267c6987df7/> add latest version
   * json takes delegate errors
 - <csr-id-2c2585d16d5dd346fb16809db05495be93ab571b/> removed generated go file
   It shouldn't have been committed - after all, these are just there for
   my own reference.
 - <csr-id-0bc6d216c3be583cfc58143e136a16e573d7277f/> generate doc index
   It contains links to all generated API docs, in a style similar to the
   standard rust API docs. Thank you, shared CSS !
 - <csr-id-9377220c59cf9a8a29720b0528b9263e9e947580/> api-list is now in separte file
   This file is completely generated, and allows us to easily bring in
   new versions after each json update.
   
   To make that work, we simple merge all data handed to mako-render,
   inside of it. That way, we can put 'api/list' data in any yaml.
 - <csr-id-7b81646f43c1c4de5165c5b3e9e7ea5c836eb664/> update-json and all APIs
   It's about time we finish up this part, to make it even easier to test
   against more APIs, and keep them up-to-date
 - <csr-id-874cfb6f680cd601271befa66e86a54c59e3618b/> cargo calls for any API
   That way, it's so much easier to parallelize doc and test generation,
   just to be sure it's truly working.

### Documentation

<csr-id-4f98fc175ea274d3854929c05637c337f1a6aaa7/>
<csr-id-12743f543d6021322915bfdd9b4d5ef6b88f72de/>
<csr-id-6f5c1599168524c0df0b47713ea4eb1a00d049f5/>
<csr-id-944e04bd12f6415e3818f444d2604fc103ec162b/>
<csr-id-fbec9bdbba375037ec3ac5886bb86390622194fd/>
<csr-id-b64722cca85a0396cc1389da694e7abd2338ff2e/>
<csr-id-47f9ca8b209e0f2453dad6d8c121e60e138f511c/>
<csr-id-683cbbdd753611c6f09e5111bb1aa3c29e6b909d/>
<csr-id-92b1ef7476d0adb9168c94b8d9bb1097ad682fbc/>
<csr-id-6d3bbcea5713a7a868ba7e8def00ed18fda83b64/>
<csr-id-24e053718a6960466d4da69ba4113fc341646b69/>
<csr-id-c004840d5bbb5b5196a68b67f709008d055d496a/>
<csr-id-c65a8a6bdf9296721f21f86266f744d656f00ee9/>
<csr-id-334061a5e20846cf4f21847c1950f58ca4f9c87e/>
<csr-id-49c4a4101e73b516b67f66779072efe13a624ba6/>
<csr-id-74bb79d6b4b73b0031ee233cf9a1667f7fdb8070/>
<csr-id-c1d09e6d576b6f6bb1245af6e0b9b166c5f69b2f/>
<csr-id-aadf37004ee609d940674f6f30ae4c942ba522c8/>
<csr-id-2f293f5e1bc14b8189d38424ef24d829fedd8743/>
<csr-id-bec5cd5e5c12a38168e0a117adccccd6e3407e9f/>
<csr-id-6800edb4dd9b3655da231ef483780144c2b52884/>
<csr-id-4cf365d0263b66ee538eb5e31144469a3018d856/>
<csr-id-206ccadbb3849c27247d3670c3bf4591636b66d0/>
<csr-id-9bcb3f8ba900e313bea4fd4203177851e6e86f9a/>
<csr-id-ac35432b3f200a02a1272b3e295dcf6029e8b441/>
<csr-id-4b12da4a12927f363f9ce2e208a1c92f05bbda2f/>
<csr-id-182d0c6facbc80cf30c072abd930aa15a1898123/>
<csr-id-a7f93a93b62a908f470cc0de1164551786d1b96a/>
<csr-id-0ff1e07e534e33d0815676270c90109a0195ff82/>
<csr-id-42ae75c1a1a2bfa148a6c52884c88ac71bcf93c0/>
<csr-id-4e8872b37af5bbefcbec6db8f9192d0fbf180eeb/>
<csr-id-bfa20a18c8138ddd7c76a2fcdeb43d40bc884b3d/>
<csr-id-74aa7bba2d9e5e5375d15ee2500e385d4b33415b/>

 - <csr-id-a2f5625fe36c5c257d2dc2a4483e5685f8848518/> add lang attribute to docs index
   Use the W3 standard "lang" attribute on the "html" tag to provide context for browsers and screen
   readers.
 - <csr-id-c4055af677f0d7041c90a2bd34f4ff45607a43ba/> fix build errors
   Rustdoc assumes indented code is Rust code, but when it isn't, the docs
   fail to build. The Sheets API has Java, Objective-C, and JavaScript
   examples in its descripiton. Wrap these examples in language-tagged
   triple-backticks to prevent rustdoc from trying to compile them.
 - <csr-id-43b953445e4352d20c671950c8538b04059661ac/> fix typo
 - <csr-id-7a041ecf2d62d35383e82432e6322e05c74d07d9/> Add dependencies to README
   In the section that describes setting up the Cargo.toml, add the
   required dependencies, and note that `hyper` and `hyper-rustls` are not
   the latest versions. This helps new users get started quicker and
   easier.
 - <csr-id-8f0435ae4f8b1cfca65ec6374c60ced8b5080a18/> fix body and footer headers
 - <csr-id-5c798d5fb621f042a8851339df04709831f04ca5/> information about unstable rustc
   Explicitly state that a nightly or beta rust toolchain is required
   for successful builds.
 - <csr-id-a66f1ad728980d431a1ceb1ae0163afd6949d983/> added dev-diary episode 3
   [skip ci]
 - <csr-id-383595c44e9b2aafbece20eb60a3ff36c1f88d81/> added Download information
   That way, it's easy to obtain the respective precompiled binary, as
   well as seeing the source-code.
   
   Overall, it makes promoting the tools easier as the CLI docs can be
   linked directly.
 - <csr-id-6bca4b75d9b1abbe882f5c1bb7ab27232046f89d/> detailed deployment instructions
   Related to #107
   [skip ci]
 - <csr-id-fff466f6bcc1ff2dae882d5b1c29b0ff844e46fb/> `after_help` url for method scmds
   That way, whenever the extended help of a method (e.g. apis get-rest)
   is queried, you can easily jump to the online docs to get details about
   required data-structures or parameters for instance.
   
   [skip ci]
 - <csr-id-bd27046cc8cd5ccf515355a5d810dace168a7db3/> update STRUCT_FLAG and UPLOAD flags
   * adjust documentation to resemble actual upload flag semantics. It was
   still using the one previously used in docopt.
* Make -m <mime-type> optional, defaulting to 'application/octet-stream'
* put program type inforamtion into shared.yaml to allow accessing it
     from the index.html.mako template.
* Instead of writing pod-types, we generate a random value of the
     required type.
* Fully document how cursors can be set, which is all that's usually
     demonstrated in more complex dynamic structure documentation
* just for show, use absolute cursors in the top-level structure
* indicate you are setting an array or hashmap in the details
* add method listing for various categories, like 'downloads' and
     'uploads'
* add general information on how to do downloads and uploads using
     various protocols

### New Features

<csr-id-96415d17ca383ba0653fb4df23df1ebe27d57f55/>
<csr-id-75b80de3c644a1487358561810c7c56bad8cca1d/>
<csr-id-894b5b5ec7bf7cb027ba31bf83c40f27e0ab51bd/>
<csr-id-656fcae2b481ac90254bf5e3081d2bbd659d5232/>
<csr-id-db4624b46728379393372be40b1ce731fe8f28b4/>
<csr-id-b39bc3a9cd165db8f9ea3fa536697ca80d36628e/>
<csr-id-988d37f0dfaf8a1725bf92364e965c1f32e6802f/>
<csr-id-36513f101e0c3299513fe1bf542c7fc7c492e771/>
<csr-id-c248301951cc1266136e2ab7b6c6f5cc54d86164/>
<csr-id-a3289420337c7f607f4393fcf0832167267cc473/>
<csr-id-e42f6fbedb0a2e609c6d1363a5c0eaa5b7967863/>
<csr-id-b830c1c6decea4d5b3a16712b31daaa544cc837b/>
<csr-id-c14ef9afc86a17b5bc3952882f98fc7bf7a2ced8/>
<csr-id-03f35bd4f547da5843fab755ca678c01800aabed/>
<csr-id-159c65916f0fb4d0136a8cc622919daf60a7ecfd/>
<csr-id-f5f12c559448f73a08a812f4ac40bfc6dafcbabb/>
<csr-id-15b78cd1ff148a20006e92fd9210e93f01d9f366/>
<csr-id-1dd1fcf4b80e9554bac430326fa668b18cd9c678/>
<csr-id-6119bfb7627c7e238a5641b0781bfca3689e8a36/>
<csr-id-9eed4056e53d71f2b8165fd4099fda6fc0d0798a/>
<csr-id-36a7cb239a2717b54500ed41a346a382b092f76a/>
<csr-id-6ae6ee88a05d8d8c76f69c4bff2c37684b3d81ad/>
<csr-id-3f49f50ac21fb921b61c1170c633214782f39cc7/>
<csr-id-c3a9f1e8e594172ac783f0b9c76e093a534674ee/>
<csr-id-e34e24e04943e6cce8564295587bbf426c58077f/>
<csr-id-d6919f1eb65c7e29527360739555fce4a254d9e8/>
<csr-id-8afc76a7fe50ba8171f1e2045d989162c9864395/>
<csr-id-f71c2862851f98c00fb893fa3b940a912b893845/>
<csr-id-7dc9972445593f592f369759b9839a3dedf8d12c/>
<csr-id-be228f19940d38e484809116c1bd84bb8edf5ee8/>
<csr-id-4548644cb1498f4c7769d8e98cc7ddf8c0e4f47b/>
<csr-id-5799d44fceb537f8f82ae4919682c9189a172792/>
<csr-id-ca8e8c06220f858424c8c1b799b1f00bd89e9bb2/>
<csr-id-310c81f19cbfb8e1fc7d7f3766492c002a340761/>
<csr-id-3cef120c58d304e120ba5e86a1717f1c47452452/>
<csr-id-c78ea5381aeeb7c97ce4fc35e0c9da40a7022423/>
<csr-id-39253d988af3d7795b2167edb3a54b8988dda00c/>
<csr-id-f527c8202b961d3dcb4c30a13e3c28a650fb144c/>
<csr-id-390354bd08b429fb438d60c54e2a36756e086c3c/>
<csr-id-d1c97912cbebf8df3f2817b04b15a78d952b092a/>
<csr-id-cefd606b538ed86d7b659f83b64ee2b14f71fc3b/>
<csr-id-be7d8214c16287fb245918c38561544245a0aa1d/>
<csr-id-29ee94b4c04f72d2676a98dda6632a06c5b8ba54/>
<csr-id-065753cc3a56227c2e87fbcc8b36121dc3bb1ab6/>
<csr-id-42a76e465549beadd3080c36f68922d8e44fba54/>
<csr-id-d26cf7740614134e97f1b6add19c3b91242fc994/>
<csr-id-8ad316bda3fd5eaa7e9a993ff1a9120e71022365/>
<csr-id-57e0f0658379db524f1a964232a3fa39111be626/>
<csr-id-ffef7dda57c8f3f14d86712107416eaffe4c1bfc/>
<csr-id-9ea85273cd18798c7f0c523a45de1f25c0648c92/>
<csr-id-307d3f487c6b35f42be643505a4e65c6ce04e6ec/>
<csr-id-0823dec75cc89b8e0a87a41ab2dcd1d5a405a24e/>
<csr-id-8bb2166da0a11db45a68e53518e94119b6d5a3b3/>
<csr-id-bb75c5b69871ec88c888618d0c3292741c9cffff/>
<csr-id-55978ff9a2fe332c5ed46476af4f921a72999e5c/>
<csr-id-9f719dd9287ee112fa6c3ebb6be64e9793da8a81/>
<csr-id-d3bb130be0b25f984c75ab125d2b344929865213/>
<csr-id-265b448297493afe11c38ac751376c67907e84da/>
<csr-id-3a1543033949b8f25e2e3cd888c9f43029b4de3d/>
<csr-id-508d14eafbca167f9801a2ca7ff9a1ae922be734/>
<csr-id-02d7a06fdff10d54c93d00fa18e0330e1f536162/>
<csr-id-4a27ac7e1d14207645915637c4817a17f10916b9/>
<csr-id-cb5a0a35bc36cbf234e2ac5d2cec0b2c14ac1d2f/>
<csr-id-9d401f5486b447ea0fc43cb0d4bb84fac3329357/>
<csr-id-224af64068c60649266aff7cc06abd001053015b/>
<csr-id-b127df17b02a4823e74a5125961bdfa23f77f7a0/>
<csr-id-8db346b8b01f003fed24d202822c398fa0994443/>
<csr-id-71c827b3067131a150bfd4a3503a61b836ec39b5/>
<csr-id-fc589cb965848332dd944a790cafd7d4745d9fc7/>
<csr-id-3ea5e194859749e05632edcfd35cc21db8cf53ff/>
<csr-id-b0a1f518e957c96a0f5b5b2297a738cb42032e87/>
<csr-id-7cfb5afd394041019899ca4cdcf10c9187204409/>
<csr-id-d2bf24ca859b945e1f5ee64dc5ccdf7357d01184/>
<csr-id-1fee21de24eee4fd62151595ef7915987f7a39db/>
<csr-id-54eb784a550a619b3773e44fc2ddd0b2a58ffcd2/>
<csr-id-64219e7e7eed42f7491a2aba80f5e8fd7567385e/>
<csr-id-d758f410f68b84cb635a6a0633bb09b147939397/>
<csr-id-60d953a3428d11591954e7488bc46078d4765b1f/>
<csr-id-354370705dd317b9839cf9a6ad34e22b9efe12dc/>
<csr-id-33e85ddd29db5a75ce49718d850652c36ad7ce25/>
<csr-id-79cbf3ee3fccdbfadcb1176ebc319f8bbabb8b68/>
<csr-id-60adacf8d47eb43a0f82642a69c5216e79285dbc/>
<csr-id-eef1471357e7a16f7501575bcca1d17cddf05515/>
<csr-id-2c79f6e3cfbf7044a061eef1ddfb6fadac19401d/>
<csr-id-9a58b0badd0fea4220cccb953f6deb00c8edbaaa/>
<csr-id-7f33cf22a5c22e3cc50dcc199604af78ba8e13fa/>
<csr-id-dd0772f1d7e1330229bb36040686f91e088befd2/>
<csr-id-c0a247605890be6553fa4709074b4c4ca4a199a9/>
<csr-id-9a17ab9e4e98d8797a9912d3d5094c0e2bf9716f/>
<csr-id-664d8225d2d5275148395828af02c0bc54b7ee24/>
<csr-id-b8956103d9460c73956dbc28ca2f1684ba8b853c/>
<csr-id-f27fda8f34e084e1532f4e6528b93e156f062503/>
<csr-id-ac8c41530d082203f93d81851682d02ed5c98d9a/>
<csr-id-712fed578a377c27bd6153b098ee4b3244b0355e/>
<csr-id-2d036b6623a6f21e7d5706b382e2bc1e28dac87c/>
<csr-id-bb76832b2f317501d398f5ea9fe8ea6b12dacf7b/>
<csr-id-e1b7a63f0660682a1680d9651cd5c3e784b12030/>
<csr-id-aabed3858143bcd28d4b95e3831c408d3120719b/>
<csr-id-da300e035ebc92728c5566071c26505a38b409f6/>
<csr-id-7c6f7d5e97344e7df0f397c65209795e5b8515bc/>
<csr-id-6c4166094358fd236490239d12235a80b738f34f/>
<csr-id-432faa275f89bb1c3ab00b60ff07225eec5a4489/>
<csr-id-678b6929ca7bffb4e4495272330aac02a082dbcd/>
<csr-id-5b2d8a77a3cf17a1c5989e856b1ae2dc77613264/>
<csr-id-de0c7a4ae049b6f7fbc256d64bc363ebd8de2101/>
<csr-id-66f3ae14e5f088828d6c9d772643889366934fac/>
<csr-id-020300af15022124cfa0d3e1722d45ff371f924d/>
<csr-id-48d40d45c5ee2b8dce689eb0a0457e0364246899/>
<csr-id-693b5c8f6a556941fcbfaf6b58f0d0dd00053a66/>
<csr-id-582aca32494bf938889b04c60c5d3cec81872f77/>
<csr-id-942cbe18f1f237fe8efacde93fd121879924d619/>
<csr-id-01db89057deca47d86355e35c86b4fb88c218db0/>
<csr-id-e96260bacc959aee2d3baa1353d48087637f3df9/>
<csr-id-615a12465415cfa155271ce2fb94be9faa7405db/>
<csr-id-f1d95822f784bce84927c2a9d4134d5477495217/>
<csr-id-e164cf73667a6b64908a1dd41c5adf91191a5237/>
<csr-id-c1eeee0591f96e2865db1ed13900ba7b59475ac9/>
<csr-id-ba98bee62fa2e067e9bc18f6f52db8be1da35161/>
<csr-id-d8edf1dcd46c6f7ae27e6f61b8aa1dea071a44a0/>
<csr-id-a5e675e7a958327938a31ec38ddebfaf58af9f42/>
<csr-id-475163ec29e5d20e74141de76f38b88a51bfbd06/>
<csr-id-fc15a7030f81658663ff416a86880bfde01f23f0/>
<csr-id-3670e4f6c98d1b04a618fa9c14d5470a7a6765b7/>
<csr-id-4e5f2c05d93dd2f4cbf7472a8911fbd7e0463d9d/>
<csr-id-e3b6aee6d631c589cb277b999583aa460631c34d/>
<csr-id-2298601165f5b65f76c86f4542139965c2486e58/>
<csr-id-be938255bd14202cc77c6bc543c6e92060a7ccb0/>
<csr-id-2d77857aaf9b6a7e1a5dc7a3f77349a3662f8c7c/>
<csr-id-087a0762ac936f40bc4cec6f2281db34d9cab95b/>
<csr-id-30041e9c7da099c4843cd987ff34349394d8613d/>
<csr-id-20410adb786a1f35e870b38fc3b5b3140b626708/>
<csr-id-c0bfeabbc39cd7449f59c8e1fd1fe9e5abba315a/>
<csr-id-0812068c905463c10352ac194f44c9a317352647/>
<csr-id-0c2f149b1e168497a376ce48105fa4d4089612e6/>
<csr-id-f13c2960ab8b3441a32bde892a8ee53f8497b987/>
<csr-id-1980f76c3240b44c306158df30793ca20ffc9461/>
<csr-id-eebcf549295fe5b0521092bd0c79d83c416d351d/>
<csr-id-aaf432fb545b47a64692dda0296414edbf3017b6/>
<csr-id-24a727fdea7c2ae47dd23b7ff571cd717ec4d870/>
<csr-id-67b052c5f376c85ceb2f3e94e676e4906df9fd10/>
<csr-id-dda847607fc88ab6bb6d9646d52cd9795f7af0b3/>

 - <csr-id-c5ff239961df59cae23a7dd609d69b629f31e7fa/> Support custom connectors
   Switch the constraints on Hub types to use public traits based on
   tower::service, as recommended by Hyper. This enables support for
   custom connectors beyond hyper_rustls::HttpsConnector
 - <csr-id-cc57c6a93d1f791adb2aa80161af50858d69d61d/> bump yup-oauth2 to next major version
 - <csr-id-9ffa241f37fb1a8524cd15525944e253ad07db1f/> Allow overriding rootUrl and baseUrl
   Allow the hub to override `rootUrl` and `baseUrl` for the service.
   This is useful for pointing at localhost for testing or a proxy.
 - <csr-id-e646898137de3897cdabdb65d2ad553e45c772c1/> remove download information
 - <csr-id-91a657b8cfc4769c69acaaefffa88a5960cd4b9a/> can now be published
   This works as the API version is now explicitly specified,
   allowing cargo-publish to work as usual.
 - <csr-id-d37bb19df2bb4b274ee69c8ed3e85056c216e8e0/> Use flow for installed apps
   That way, more complex APIs like drive and calendars
   will work without any (sometimes non-existing) workarounds.
 - <csr-id-8f01e8e91837b76092507b9313d914dce4fb1c49/> updated API descriptions
 - <csr-id-ab1aa55d395286e96a6508a6afcc5b8d723572f5/> clap-rs v1.5 -> 2.0
 - <csr-id-ca36dbc50595f116952a95395d802d5be313b614/> improved structure setter code
   We save about 30% of CLI code just because we offload the work of
   settings structures into serde, building a generic `json::Value` to
   contain all the data, and then let serde do the deserialization for us.
   
   All we need for that is some information we let the generator provide
   and translate it into the runtime.
 - <csr-id-a2dd71451deaf49e2bc4bb8de68a4e4cc87ec8a9/> basis for simplified value setting
   Previously we would set static structures manully, using complex cases
   and utility functions. Now we setup the foundation to allow setting
   a generic `json::value::Value` instead, which can later be deserialized
   into the target structure.
   
   Related to #111
 - <csr-id-52027c6db59c2952f61ee03204fd947277d0cc62/> added download links (osx,ubuntu)
   All assets are configured via shared.yaml and are located elsewhere in
   the web. This could lead to broken assets at some point, but I am just
   risking it for know, knowing that it's easily done to have local
   resources.
 - <csr-id-0e6605d7a4ee59e16d52fd93e037b5608fd5f61f/> added back-link to crates.io
   * url is created per-API and features a nice crates image coming
   from githubusercontent.
* functionality is cursor-aware, and fixes the actual string the user
     passed in. That way, it is made very clear how the suggested value
     is to be used.
* it's a known weakness of the implementation that it operates on a
     flattened list of field names, and thus may make nonsensical
     suggestions.
* added punctuation to all errors
* As `possible_values()` applies to all arguments, we cannot use it
     anymore but have to check the UploadProtocol type ourselves.
     Besides that, switching to the latest `clap` simplified our lives
     a little.
* ajusted docs to not enforce using `-r` all the time
* More detailed error type for JsonTokenStorage
* removed all traces of rustc_serialize
* use pretty-printers everywhere to allow writing human-readable json
     files for secretes and for tokens
* added simple script to build tar archive with all debug/release
     binaries.
* slightly improved docker script, even though it would need additional
     work. For now, I use the cloud VM anyway
* with native support for type conversion and error handling
* improved hash-map key-value parsing to at least state that it knows
     it's dealing with a hashmap. Error text is still not what it should
     be because we don't know at runtime (initially) what type we handle.
* Seem to work for docopt, mkdocs and code itself
* mkdocs now show type of required params
* some code which deals with converting elements to their
     target types is totally untested right now.
* Allow to see all authentication related communication, similar to
     --debug flag otherwise.
* fixed broken generator when handling request value parsing.
* If `--debug` is set, we will output all server communication to
     stderr. That way, we can compare our requests to what is expected by
     ush based on official docs.
* `discovery` now doesn't use the API key anymore - this is specified
      using a custom override.
* set globally shared parameters (which includes 'alt')
* track if 'alt' is set to 'media' at runtime to do the right thing when
     outputting the result. There is still an issue to be fixed though
* support for encoding response schemas to json
* support for simple downloads (without alt=media)
* improved documentation about error handling, it's less verbose yet
     explains what you can do.
* ArgumentError -> ClIError - seems more fitting
* if there is no secret file in json format, we write a default one
     that we will then read in a second iteration of the loop.
     That way, the user has an example of how such a file must look like.
* Only supports one level of directory
* full error handling, and uses memory efficiently
* allow usage of cmn.rs for common types (like Error types)
* instantiate an engine and handle errors, in an initial quick and dirty
     way.
* allow to rename executables, for now just brute-force using a boolean
     flag. If we have more binaries at some point, we might want to be more
     elaborate.
* everything related to docopts functionality is now in the docopts
     module.
   
     Related to #45
* crate version: code gen version
* +<revision> (build-metadata): exact version of API schema
* MultiPartReader is using match to handle state, reducing unnecessary
     calls to 0 in that regard.
* Fixed seek() calls on readers, assuring they are reset to start each
     time the loop is done.
* both media-parameters now use `ReadSeek` streams.
* Use `seek()` to figure out size, simplifying the interface.
* reserve_exact(X) where possible (params, multi-part-reader)
* `if let` used whereever possible to prevent duplicate checks
* outer frame of `MultiPartReader` to allow using it in `doit()`
* restructured `doit()` to get content-types right
* Added filters for rust doc string
* fixed .PHONY
* multiple outputs per template/command invocation
* NICE embedding of code (like GSL can)
* CI
* documentation
* basic cargo

### Bug Fixes

<csr-id-1323d0dccbb2ed7570e59b9b125f7b4a97ef7575/>
<csr-id-99789de208609f0b8ca39852492fe0bbc54689ba/>
<csr-id-082e51e16e9deb01e9c82ea7c8ac9be5bee80c79/>
<csr-id-b68b2a6bf5786327afad1d95bceffa1111400e0a/>
<csr-id-5e28a06dc0dfafd414765738fff35d019a903cab/>
<csr-id-f3d0ef45d26baaafa3b9120bebe371bce424309c/>
<csr-id-0ba9535a1150750b80e862c8fc197819f0f25954/>
<csr-id-0f14aa966e5e878612111709568b13e7a2c70345/>
<csr-id-292dd2f34f04a2376c3d44990111d4a0fc2c400e/>
<csr-id-b6f5fc6eb3e6be21d22fb667b541f13ee3881df1/>
<csr-id-dc367c34751e04036e56a4d984d6b7f8f92cef4d/>
<csr-id-e9fe17ee3b5df32de65ed4017c65748eb8388a29/>
<csr-id-3921b6a5a071ec0dc9d803b0ae809a348c34f87f/>
<csr-id-5ca021727511b8265da1abadc48eb241ca24e3c5/>
<csr-id-bed46ba2414167fcf6563ac1766f3239765f4800/>
<csr-id-cb6679cb2b45022162a7e6a1b5de39b1fbbcf870/>
<csr-id-065cfdd22f974afe9d8071e0227929464c1df796/>
<csr-id-9e8a047ebfddd7a94226b8d559b03325abf7ab54/>
<csr-id-33f281360a0a5dfa93cd7e6f4f345689e86bcc3f/>
<csr-id-4bb7a33e9370f520b985f96aa8229b659320ff1d/>
<csr-id-a2c6b58d5b8525110a5386e93c2de4f6851b95c6/>
<csr-id-ef9e7f1bae2bff1629530fde14ca19ad424fc653/>
<csr-id-b54acb7c96c842228a7ec65ff6b6edaf2b19b0bd/>
<csr-id-78c7d46f9ddb7b102fd59135cac5d1033f090b0a/>
<csr-id-8179f3bf89991d83f6cb5689618f8ee90b3f9a5b/>
<csr-id-9a2d2b576c84536a7a93deedcba68544bf4a10eb/>
<csr-id-0bd7f2004843b4e9dcd8af366e7ffc6632fb9e41/>
<csr-id-53c27da2e786e12a29037addde15d571c3b53b39/>
<csr-id-8dab8c01249a9f54e43aebe8a009f60935279de8/>
<csr-id-8ab4fd0bd4b64eb76c77adc82aff99df17a1070c/>
<csr-id-be894becc38296a62760a0724ea1310081e713de/>
<csr-id-e129a7d3ae878a9ee78ea21fe1c0aa8b5671a5e0/>
<csr-id-0f61fa4c95c25c0e9f30cc10b6aa3b005d26e3ca/>
<csr-id-615ac64ec1d86c1c00ff05a4d2f6065c866330a7/>
<csr-id-d0491a4950f657c55dfbf6a16a16a64c72b9077c/>
<csr-id-27fdd8ee0c19dda409b6ca5a804edf23b8555ff3/>
<csr-id-a566b702738c4b470988645f2867966d1d288370/>
<csr-id-62db3ae87c1ad71082566a2e195a1e5d2cb7219f/>
<csr-id-a9e0be6583fd92b9a171091b70e81bdba4ad4aa2/>
<csr-id-2ad8d887cda32214dc520af5a9621366f4522fdf/>
<csr-id-5483e328320412c39798ba15f26d02b90dd7591d/>
<csr-id-b0a41c4e788fd95f9ace6823c2e52d18f33195c9/>
<csr-id-d9ed001b46cc6e510ea2267fa205abd036be10b6/>
<csr-id-bcf90cbcc8f625b787596fb95eda4355e35b403b/>
<csr-id-2cc48072344715c428c204e98ab991c8133cf4c2/>
<csr-id-2ca05292971d50adb267305492a8c703b929e99f/>
<csr-id-ee84fefb4a46bf816fd6fcedebaa1428d12969a5/>
<csr-id-9e64d1bd10f0cd68c8519954bda14ce784805a7a/>
<csr-id-be117767a11962e330568d1dd98035e7b142b910/>
<csr-id-3efa4f2b12219412cdabf8535e03974b94f71af5/>
<csr-id-3fe2732a01371ededca2c35fe7499a4bbe63c318/>
<csr-id-9274938f9f69ecab2e8cb975467860f41466ad1d/>
<csr-id-b27c990db8a8701e2814e77136a34689be56c623/>
<csr-id-d2a4e2ff8b16cb848869cc07b6c5a9107fb0a929/>
<csr-id-89432cc64600ba0711e412c6cf6b1e06e2f11102/>
<csr-id-c346645fc96abf9831ce723bb56e26f95e3c5b45/>
<csr-id-bf6a2ba60c364e7c30de198d335e481c0b3206f0/>
<csr-id-153324ebccf8a7846d9669f16c8f3ea52f0ec810/>
<csr-id-94c821e09d2b75756dd3dfa2d5f508b079413cf1/>
<csr-id-2f200217f942aa0317186811dbbe95d675a17ab0/>
<csr-id-fac50418a7156b1b2fa958008691dbb2f6cbb756/>
<csr-id-d46c083975201a6a4804fde9d4cec6ae0fc29479/>
<csr-id-4115d50ca795ec2a2958f5f75b7681cb9f84720b/>
<csr-id-d0ce221ba39db621b969b8c1faad358c775502a5/>
<csr-id-b039b382446f450a58c12d2d881dbcd00b96928a/>
<csr-id-7a38f7e4d5dea97b5bd2cbe6b10e4619b3b45b12/>
<csr-id-63e23dd48f7fb80268eb3bc95380b77b233de62a/>
<csr-id-5320a48e68c0ee4457455c5caa5c01f322fc6c7e/>
<csr-id-bac4e1a82fa331370c20a7c4843989f11974600c/>
<csr-id-feaa3a06ed53ae039750e2d420817116b1140984/>
<csr-id-02a41296628eb0cbc0c8b7b2e86b06678e8db084/>
<csr-id-1aff3135d97435632599bf39cf5e8c5de9d773a8/>
<csr-id-8ac8d3b1cb59249d492a657fa8cd39fbe3fd99a7/>
<csr-id-9a8ae4b7d66ec1b6a74316fceeccbf04a2f77469/>
<csr-id-57808cf92adf7ff4dd65664a4a4ed3a361b60c6e/>
<csr-id-de85fb43e53723d1d38d0b6e8746acc962035233/>
<csr-id-c2dd9c7a020e0367bc87b20fa8054c85f48b71c1/>
<csr-id-4e275eaaddfd7a86ed42d04df24113015c6ea099/>
<csr-id-607ba745d140e5d2567a715c6ddaa775d2cf0d99/>
<csr-id-b6a48bdcd5fb215e94a00a69d11ce0ac007c2df3/>
<csr-id-2f3b2d24ce2367356698b902becabb40b8636ab6/>
<csr-id-0bb30da78244abcf09c7d04571515e6584ccb4a3/>
<csr-id-be7ccb085cb5ea908fb75d0ae7cb6c91ded33bd4/>
<csr-id-6befdbc6fa730fc4a5513d2cad9e1784c580e516/>
<csr-id-f8689be4515d5693004da17bb2244a385ac1e794/>
<csr-id-845a568b25f387c58a17752852aed63e7305c7b1/>
<csr-id-6d84ef906e6b9ff344fd7acac3140bdad3d48e78/>
<csr-id-e523ddb6ec9f1e9e8bcc51fbec02e364dbddaa72/>
<csr-id-797f289886d899a7e1b21216ee46218d179e38bf/>
<csr-id-a4b73cc1c4e3919cf8bf2f782d598d0840c4922f/>
<csr-id-e730281003b4a4caad0d48c2712e5d1433848bd7/>
<csr-id-d8fdf9df9f41719f6acb9bf3750aa8069cfab675/>
<csr-id-9ea3fea7750bce93c531f99b13c747c78a806b59/>
<csr-id-4cf0720ef1e025737416ad5fe07eff2389c86ad8/>
<csr-id-fa278a99c769e99727176f4faae081cc2d219342/>
<csr-id-bf22bef77ae62d06209c70d273ecccef29a4268a/>
<csr-id-bf37e515d2b5affec6296c34fbfa68fa89f7d4b9/>
<csr-id-4b87d909f21daff696dd81da463fae3b14e59725/>
<csr-id-306852d5147d7083ff011f990c5feedcf3e338bb/>
<csr-id-830529c40b6ab01381fe36f27753047a2b03244f/>
<csr-id-fa011315c31815cf283ecff18e245553378f3cb9/>
<csr-id-76841da09801f23abef4955d76430ce1191c0b77/>
<csr-id-e45eb053d52db016342bd568d10bc368495dad86/>
<csr-id-5b4f18d341cbd8f87d3e3792b1dfa803f7849015/>
<csr-id-51ddcf74a6d1cf204156c6a018ced2f2d85c9352/>
<csr-id-acd42dfccc87f49cf5c9bf51a206da8bed9c02c2/>
<csr-id-3e0a24db0d8d25fec9457364d49106c22aee3c23/>
<csr-id-75e73d56d95dc4126ef39f0ae60d901a32af9954/>
<csr-id-6d3dc77635724602a89026477bfc0f8f785968ba/>
<csr-id-b9a469c0a4e655da54940dc2876559f573c88c08/>
<csr-id-2e74d9141313da1cc6a26149650ee59c43047f06/>
<csr-id-6db733274d65f10a213612561a5771bf4b7b8316/>
<csr-id-34d0a7aad3b139c71b4d0dd7ca4e10c1336ebb8f/>
<csr-id-a399488c2799e1acca0961f80a6c116a3330190c/>
<csr-id-191e822c5a93771e32e85bc5c00ef450c6719fb6/>
<csr-id-6f2149b7d49ee693cc616b92f9de79f220ce6e2d/>
<csr-id-9dbdcc465f45c13faa85e5489073e7b7f5e18133/>
<csr-id-dd1d191966aa41ec66c5a4baba5ebd43771c3a05/>
<csr-id-3403bd1c5cec379cd2ad98040cca0ec6a4eef4a3/>
<csr-id-99f8b65f75822d54f32100655d0b5678f43a8478/>
<csr-id-91861dcb71b371e8ec5511ddedee0ae45cee9af0/>
<csr-id-919ae4d8ae85f35f54c69c8c222ba43ba304e263/>
<csr-id-a2ca1cb28ec1ce9f5f381f55ea78aa59a56ea915/>
<csr-id-c7fb7c409343f19e26f1c3d488718decec7990b0/>
<csr-id-e953535473429b01293d679e23337b74645e0c18/>
<csr-id-d1c5bf1e4ab2a91c30d2bcbd1e08a1a02c73ad41/>
<csr-id-e5b013e97c56040dba266a43a8308448a32645eb/>
<csr-id-fca1b24cd186b090f75e35f362c8bbb2754e3e4d/>
<csr-id-ea161897f5fe25e024292755c753f2410211bea1/>
<csr-id-6ad0c2ef79a634d4cb631a36eb92b2cf82b59121/>
<csr-id-04f4c95688f2cef0866ce07da68ae9d710596c7c/>
<csr-id-3bc930ae47c2544de4825ecec5346f53626a75e2/>
<csr-id-cd1ff18ba94966088a779b26347dc683f1f0c2d3/>
<csr-id-556906ca60a90fc6eb34917d42813daf9792fbcb/>
<csr-id-3a9aa519496be9da6283b847f38d9a2deaf682aa/>
<csr-id-030c40d2699196e29d1c8606d042403df52a7534/>
<csr-id-4bf280079ed5cf33c4ed2617c3aa62151ec0dcd0/>
<csr-id-98f4bbab4774fb166936c60cbe8eee2302f35052/>
<csr-id-80161f72be1aa7f7551603c90752793c84eedb6d/>
<csr-id-0152138e0c019575caa3e40f87f19382d92a63ac/>
<csr-id-5ff22851faec165258e5c3ff9c6eed58df3efee3/>
<csr-id-8d9f175f917ec19e4752c5c3806f6f5624e066e2/>
<csr-id-10dfeeb1aa5a1de2919e9753444e8e63855d1285/>
<csr-id-97da926e28d7ad7ed90d12b7ff48477bcf67ee68/>
<csr-id-ff385e5cacb43d173912243fc033578b0c0b0f63/>
<csr-id-cfb8faefb8545114ddadea59871214b35e515d5a/>
<csr-id-b9a81a900ec054b102ce045cf25a4348c297f260/>
<csr-id-b6ebb1ec371c833ef7386264ed9522b880586316/>
<csr-id-a05426e79b8c0773dbb219b327539431e4d1fdfc/>
<csr-id-6b2301351f6792fb37b7dfec6c1f0592fdc6b9cc/>
<csr-id-e53e23a893ce6d59777b8b53f94770d5c3c86b9c/>
<csr-id-29d9e45c9fc8bbdbed23d3d5a9be20f8023bb22d/>
<csr-id-b90a1916889b2d1cc6c595c3cd121739223db345/>
<csr-id-863a98c0d7932475dc207d204ec91c26ddec326c/>
<csr-id-63997910decf909a8242a8a7f16f6a4c276e1d67/>
<csr-id-79879daf1b2a52593d2bc9b51ba244bfaddcf1f0/>
<csr-id-91f69ffd6ed85790d8b6d1c8b5b63d7f4c7e6259/>
<csr-id-1349c786b7e986511e4c2ca058d45bebb7f458dd/>
<csr-id-814c9c9ffab64a7607f4056fbad4203ea8f19991/>
<csr-id-876772cf2296c4b7c80c2f828e245c903da67802/>
<csr-id-31efbf4fb0033b9f1fdfae0054ece1717ec05b79/>
<csr-id-4c657ac9d132257a392bfbf2ed861142b6baf36a/>
<csr-id-a87fbdf0a86cfa410c79671aee931e3bf95fab11/>
<csr-id-51d05d6db01edb4f78159c3c07d77d0aceb85b89/>
<csr-id-5fd7cb511407de7176dc07c1443ef07075c063a4/>
<csr-id-8006bb8ca910b14ece8dee6230d476a361c7c163/>
<csr-id-b43eb0e301c068500777fe580c1bd1017d0819b1/>
<csr-id-6167dc07fc63cec22a8d2b01fe69f05f03ac3f9a/>
<csr-id-c8061ebe2fbe97274c68b7af6e5a8d08c0245139/>
<csr-id-4b9dbb28ff474661855f53143862b621e650f157/>
<csr-id-97b2649094cc225d0cfc42857140f0d245e11352/>
<csr-id-7758f99ff2e19c3518eddcfca2e1adeee12e0659/>
<csr-id-4f794ef5ff7b5a068a568056d2bfd7372ec9b57c/>
<csr-id-7e243936f226f6e26d2b551765b62cddc866776b/>
<csr-id-00de2b187d74fd78f049a13d1517fc91d218da71/>
<csr-id-e3ab233a6cee8482c1c98b1e2c759e7a17cceab9/>
<csr-id-de40a8bd1ee8759287cd2a489cc5d995c296a07e/>
<csr-id-614539a925c5e64508fa28506b1c6db3ccd96882/>
<csr-id-32145e645ea29ff43c451530906356564e12f817/>
<csr-id-538120f7d1425e026220211857658a775c958577/>
<csr-id-dfcd554faa36cbcdf18ab985c2aed744dd45dc6d/>
<csr-id-da57505567a58b59f320016d92b50f1ea248067c/>
<csr-id-9b308bb6ddebe979abca6f46da131c822f95c639/>
<csr-id-54540e695a9b246ca3d412ab62e843e4dd7974d0/>
<csr-id-50fa189a715332a7ce49fc7a9c95e5a1ef22b81f/>
<csr-id-a268be27d2123a77259fa1d7d1f831c7e72c4459/>
<csr-id-5d563c88a8e3ccb33ebe381b47beb6ecfd4444fc/>
<csr-id-559cb8fe458e18fec05d0ca3cd2847fb981f2da0/>
<csr-id-bfc392291666a40cf3fbe4db3dfeda69d23018fa/>
<csr-id-efe56ad25081b632f1e65fd8292e9c4d535659bc/>
<csr-id-cf258bf4e5148723940cc757ec032b5aff814f1e/>
<csr-id-d99ba9c5b3c5f73ad148679a866698c811eec495/>
<csr-id-df9f0299bf5db0b7affdd90b4dfb331c74f543f2/>
<csr-id-c7e169dff3712ff5f73497d2d9cba3303a83277a/>
<csr-id-7816cc81455c1c7a48e84289e176baf25e8480e2/>
<csr-id-1e332ddb91540c19586e6d85869c8e54c47552b0/>
<csr-id-92d8fa76d0f419738e2efa7df3deebb974c1e0cf/>
<csr-id-ff5cbb3bf410276fbe5af8cc966ac363e448970c/>
<csr-id-2531011fc579df4edc38b15de459c135975fa077/>
<csr-id-35bd1c3e9c8a6ab52068e279d8f925eea8af055d/>
<csr-id-3b7e63f28675ea2646c88dfa16c62c063e076b96/>
<csr-id-6d2b0fc2649bc5203c07c29dd020b50550d15746/>
<csr-id-28878e0618cbb5632a1353ceb2048a913e9355d2/>
<csr-id-1423e46210d95d823ff9bee9896cf407b0e9f0cc/>
<csr-id-baea071a6f1c52410c0ca79cf24ab325f6efa586/>
<csr-id-6fad7600a03f2f6a3964f309fc8e277b34f8aa60/>
<csr-id-0d9f6363eb271f95624559b06cfd07ab6b5bc9b5/>
<csr-id-4bdee961d19fc6fc6cb3cf322dfb85d2769bbcee/>
<csr-id-fad0a7177aa296aa777b45d0001effa36332d24e/>
<csr-id-a3206abc92d7bc9d829a1e2e00dbd299c379f2ab/>
<csr-id-9cbb2adc5a65bece45e524a71f2d66160f7aa133/>
<csr-id-f2dda421e64e9164557d5b3b94604bcb2be49254/>
<csr-id-70ea612f19fbe7e1ef0a01b0d399fb357a46c390/>
<csr-id-452b658c27e265c6a2df90ea56502db338957154/>
<csr-id-8746f5e0e20297ce58203da01638fafad155132c/>
<csr-id-8dc5e2a53dbe4d620e97089e2af9e3a94a82a4a4/>
<csr-id-f4030f02841521220fa52856fa733b828a59ab6b/>
<csr-id-bb04b60dc405d74765161bc75e35b4de72c5dcc4/>
<csr-id-ddb48a4303a7a0653898e9eea69b3d358a14fa0c/>
<csr-id-49c2ffb8e0f02698657aba46a7b34981258c6e35/>
<csr-id-317554aff398a823beae63fa09a6014ee1508f4b/>
<csr-id-11b6fe212ff33c1b2378997411cb11524d73a81c/>
<csr-id-c3d399e91a6fea7a09316f018865815214a14be8/>
<csr-id-179c64c5e74c7a783a3dc4ef68e900440e587c83/>
<csr-id-e06738a7bd49538d402f8c995710cf231d47221d/>
<csr-id-f2ca8c3fb79e482ca39d3aeb40be9b8c7f9c58d8/>
<csr-id-e081017cb3631df007937fe4bce09c554e8c58c0/>
<csr-id-e83b063f0527d7e5253f14a22c90fd3b4197584a/>
<csr-id-e0724fb56f4a49fc5da4d6b5ea75dd1029ee9a44/>
<csr-id-143aa6fd8638b3541d71954c6e3493bc961813dd/>
<csr-id-d4869cfefc58db4580e98e8dd1ae040c81083ba9/>

 - <csr-id-499416c01101c162da133613d5c855912b17eb3e/> teach remove_json_null_values arrays
   change `remove_json_null_values()` to properly remove nulls from and recurse in to arrays
   google_firestore1_beta1's `CommitRequest` contains an array of `Write` objects which can ultimately
   contain `Value` members that need to have nulls removed to avoid sending multiple types of values
   which generates a 400 response
   
   fixes calls to google_firestore1_beta1's `hub.projects().databases_documents_commit()`
 - <csr-id-d042fcf1a7e50666e1a5090680d4d1ff6081d695/> iteration over dicts with 'values' key
   Sheets api has a 'values' key in resources.spreadsheets.resources which
   collides with values().
 - <csr-id-f835835100f7fc744dd323dac9cbacb4b37ef726/> strip leading slashes from urls
 - <csr-id-d8e94b5a5dc53624511c77c7e8ee08f4e9bb6c07/> kgsearch doesn't work out of the box
   [skip CI]
 - <csr-id-4660d2367606641578bbbbfbe0e0e77bd29a9b72/> Example now uses hyper_rustls
   It's already done by the CLI, but the docs still showed
   code that would only work in older hyper versions that still
   shipped with HTTPS.
 - <csr-id-ad2748a691e0fc173e426f26c8f8142141c9c663/> for now allow nightly to fail
   I don't know what it is except for it pulling a more recent version
   of serde when in nighly mode. I don't even know if I set it up that
   way, but would say that is not too relevant right now.
 - <csr-id-dd4bfe3de0ffbdce07a763bfc7d8fa237a322e68/> more idiomatic swapping of values
 - <csr-id-c6d67daa813db9d7d85537df0554b0ff03d47ab1/> use latest gen/ from origin/master
   that fixes a conflict when merging
 - <csr-id-ef070eef59b2eb3e54b77b5fe600e6f6c900c8dd/> Added an assert to detect when docs need updating
 - <csr-id-de6528be98503c07d04fbf42f740b6da91902672/> Finished adjustments to index.html template (fixes #166)
   Summary of changes:
   
   - Converted from using span + br tags for formatting to using tables
* `SubCommand::new(...)` was renamed to `SubCommand::with_name(...)`
     which actually is now consistent with everything else
     (e.g. `Arg::with_name(...)`)
* Signature of `client::Response` changed and now requires a
     `hyper::Url` as well.
* Mime crate must be used in the same version hyper uses
* made attempted move a borrow
* Vec::add was removed ... which forces me to write 4 lines instead of
     one very readable one :(.
     Not everything is to the better here, even though I can imagine they
     did it to prevent people from thinking this is a cheap operation.
* Use `Result` everywhere, instead of Option or tuples
* Properly handle error occurring after the dry-run. We do it in an
     extensible way, in case we need to do more than handle invalid output
     files at some point. Output files that could not be opened will now
     result in a nice error message with all the information we have.
* CallType now represents either Upload or Standard calls, whereas
     the Upload variant is represented by the UploadProtocol enum.
     That way it's clear what happens, and we don't mix orthogonal concepts
     in one enumeration just for convenience.
* upload
* download
* request structures
* parameters
* scopes
* config-dir
* debug[-auth]
* mkdoc docs grammar is now hierarchical, making the command structure
     more obvious and easier to understand. It's a nice addition to the
     auto-generated, hierachical usage of clap.
* UploadProtocol enum is now CallType, to ease handling the different
     ways the Call has to be executed. It looks quite clean, even though
     combining upload protocols and the calltype is a bit hacky.
* `--version` now includes the API revision we embody
     (using crate_version())
* Allow multiple scopes to be specified, instead of just one. Previously
     this was problemantic due to argument parsing of docopt being greedy.
     However, this also means we have to specify the `-r` flag for each
     invocation. See https://github.com/kbknapp/clap-rs/issues/89 .
* Adapted to new signature of `Arg::possible_values()` and used the
     previously orphaned `UploadProtocol` enum.
* Deduplicated code a little by adding the new `opt_values()` generator
     function.
* also includes publishing tag files
* now the cursor will only be set permanently if the -r flag is used in
     'cursor' mode. In 'cursor=value' mode, the cursor change doesn't
     persist among the flags. That way, one can easily distinguish
     between setting the cursor, and setting a field. However,
     '...sublevel.level=value' will still work as it did previously, yet
     the cursor change will not persist.
* Documentation was adjusted to represent the new cursor style.
* fixed boundary syntax of multi-part message. Was --BOUNDARY, now is
     --BOUNDARY--
* Fixed ContentRange parsing and serialization. We actually managed
     to break it last time we tried to update it to match the Go
     implementation.
* fixed uploadType header parameter. It's based on chosen protocol and
     whether or not the method supports multipart operation for the given
     protocol.
* implement custom scopes - previously they could be set, but were
     ignored during the API call
* api-overrides are not yaml files for convenience. Existing ones were
     updated as needed.
* add new APIs
* remove old ones
* add latest json files
* updated all json API descriptions
* enabled 'pretty' printing of response structures. However, currently
     there is no way to get rid of all the NULL fields without external
     filtering
* all structure fields are now optional - there seems to be no way
     around it.
* exclude dataflow API - it doesn't have a single method as long as
     it's in B4. See https://github.com/Byron/google-apis-rs/issues/78
* assure ARRAY branch can be hit
* API-docs now adjust depending on where 'alt' is set (either as global
     parameter, or as method-parameter)
* CLI: download tracking now works for 'alt' as method-parameter
* CLI: global parameter remapping allows them to be named consistently,
     but map to the name required by the google API.
* Thanks to a generic function, we save a lot of code within main.rs
* more effcient signature for ParseError
* refactored errors into a hierarchy
* implemented `Display` trait for all error types, including some
     'hierarchy-aware' printing.
* in APIs, scopes will now be per-method, and if no scope is given,
     we will assume only the API key has to be set. Previously there was
     a wild mix between globally mentioned scopes and method scopes.
* assure CLI generation works so far, for all avaialable APIs
* update all APIs to contain said change. It's not worth a republish
     though.
* catchier title for dev diary episode 1
* fixed target name for clean, which was 'clean-api', but should have
     been 'clean-all-api'
* fix documentation link in Cargo.toml
* adjust to latest hyper. It's not even out yet, but people
     can't build the APIs anyway.
* deal with hyper client not using a type-parameter anymore
* fix incorrect documentation link (use '_' instead of '-')
* added crate publish tag files
* `docs` is `docs-all` now. On travis, this should only build one API
* macro 'alias' was renamed to 'rename'
* fixed `cargo test` on main project
* keywords are no longer than 20 characters, which is a restriction
     cargo imposes
* don't use 'homepage' link in cargo.toml unless the homepage is
     non-empty
* Added all publish-results to mark the respective crate version
* builds with latest beta/nightly
* using std::convert
* update to latest hyper (and other dependencies)
* `ResourceMethodsBuilder` -> `MethodsBuilder`. This is now precise
      enough. Previously it was just to similar to what's now a
      `CallBuilder`
* Fixed whitespace issue in `doit()`
* Added all APIs to source control
* upped crate version
* renamed `*MethodsBuilder` type to `*Methods` type
* renamed `*CallBuilder` type to `*Call` type
* greatly simplified `doit()` signature if uploads are involved
* pass `auth` to upload helper
* do not emit unused types. Sometimes though, rustc doesn't seem to
     detect that attributses are actually used
* ToParts trait is used and implemented only when needed.
* complete list of reserved words (keywords in Rust)
* use namespace for otherwise clashing types in cmn::, io::

### Other

 - <csr-id-8ba6acb88bf889d41560ccc2c16f5e884af68b9c/> Make new_context dict-compatible
   This is an incremental change towards a strongly-typed util module,
   aimed at reducing dependency on the DictObject class. The rough idea is
   to annotate everything as Dict, add some tests to codify the existing
   behavior, and then start defining dataclasses for the dischovery schema.
   
   We also remove some unused logic & params.
 - <csr-id-819e1ccce5c503329bf6ed5dd9078553a48997c5/> :iter not needed
 - <csr-id-d202f9792ba0aea107213ad33ce5e7da06145ef1/> blacklist versions that do not exist
 - <csr-id-2740810b2aa40517f7492f7c67dca7a08b017600/> Fix version of Python used on Travis
   mkdocs depends on tornado that fails to compile on Python before 2.7.9.
   When running in Travis not using the Python language a very old version
   of Python is used.
   
   This commit adds pyenv and uses it to ensure Travis Python is viable and
   stable.
 - <csr-id-aaac92bad68634923c554f4f2e8bc28769a47e84/> update toc
   [skip CI]
 - <csr-id-c4e363d94ce1715f3ecfe6fd6ee56b93c670bfb2/> remove obsolete notes about linux
   We use cargo now as installation method, no need
   to provide binaries anymore.
   
   [skip ci]
 - <csr-id-0337435cd44105749cb219cc75d61da6895d5d8a/> upgrade to v0.9
   This provides proc macros, greatly simplifying the build
   projects.
 - <csr-id-d1ebc0ff0be566c749651321394ffb955633286c/> use hyper-rustls instead of openssl
   The only openssl dependency left would be coming from yup-oauth2!
 - <csr-id-cc30a2e20b697ca318bd3b54e5b94f6935eaadd2/> don't use relative links
   Instead we link to the absolute location.
   
   tech debt: we now use http://byron.github.io/google-apis-rs
   multiple times and thus duplicate that information.
 - <csr-id-6279fd8f5df3ca9c9013635c36041e89df902428/> improve UX
   Better help alert when copying an installation script to clipboard.
   Better looks.
 - <csr-id-09805e59adba84d83184ef02f27abbb054359c45/> better install script + blacklist
   We now consider the blacklist, which is probably what the previous
   implementation achieved as it checked for existence of files on disk.
   We do the same, but more directly.
   
   A complete installation script is provided for those who don't yet
   have rustup installed.
 - <csr-id-fad9d3b0ca3f588f65faf6ec46caf51a7ca1c239/> link to doc.rs for APIs
   We also link more specifically to crates.io.
   
   Some debt was taken on as the build_version is special and
   duplicated right now.
 - <csr-id-fdc0141fbcabf68ed5d715314d483469e7a7ef14/> button to copy install-script
 - <csr-id-d6accb8f6194bac7f982ee93409821436dd8beed/> remove all download links
   Instead refer to cargo install for installation.
   
   [skip ci]
 - <csr-id-fc34337ee4ba708f63e3d2f164660edd5ffe5614/> use docs.rs for library documentation
   We will still need to host the CLI docs though.
 - <csr-id-f31ef51a61cf8e1343e4ab956d8be29271976e59/> multirust is deprecated - use rustup :)
 - <csr-id-850e115e33e5da9fe6266718e4cf04c23e554d2b/> badges for issue stats
   [skip ci]
 - <csr-id-87dcf06eacc4fef9ed5bdec99fbb589c3d81666f/> inform about nightly builds
   [skip ci]
 - <csr-id-b35a1d6732022a41b846d6a8bcee8ecae940d260/> need Rust 1.6 now
   [skip ci]
 - <csr-id-495ecef8c8fcda27b08833df9fcfca503fa65002/> rever to multirust-rs
 - <csr-id-da78e9fa4d68d772323c7a2927b8004b8ac5d1a8/> fix error from no trailing newline
 - <csr-id-7754a160c9fff6fb3796982cb2cc284c033d1008/> stackshare.io badge
   I like it ! It's super useful, especially when deciding which tools to
   use in a new project.
 - <csr-id-152cdd848a41109819d890560d26270bd08c12ae/> pretty-print errors in debug mode
 - <csr-id-6e669ced2aca094b246c2c0eb805b362924112b1/> update info about rust stable
   Yes, it's fully supported now.
 - <csr-id-129fd38e003d2ab23bad2ceb84f59bb74b4ae45b/> disallow empty values explicitly
   [skip ci]
 - <csr-id-e92f440d9b980c80c31c04752a8fe0c21fa36585/> download links to `tar.gz` files
   That way, we save bandwidth and preserve the executable bit of the
   respective program.
 - <csr-id-69b12104a9f9579773553825f63c321e7d1a6899/> DL title contains os-name
   That way, it's clearer, besides the icons themselves, which OS you are
   downloading for.
   
   Related to #106
   [skip ci]
 - <csr-id-e86e55cae788506a2280816009b8620bad091477/> improved display of BadRequest
   Previously you would only see "BadRequest ... " without the information
   that would actually help you to understand what the cause of the issue
   is.
   Now we will print all the information we have, accordingly, which
   greatly improves usability.
 - <csr-id-5894c8163afa9f9d9bed592e7e41912c77cf993d/> remove null in pretty-printed json
   Without all that clutter, it's so much more enjoyable to read the
   output.
   
   The implementation is based on a suggestion of @erickt, which is
   converts into a json::Value (able to represent any json structure),
   on which the filtering is applied.
   
   If we should ever implement pretty-printing in json-tools, we might
   still consider using these capabilities instead, as we would avoid
   building potentially large datastructures, all we would need is
   a sufficiently large destination buffer which is a single alloc and
   a consecutive region in memory.
 - <csr-id-26314e743e2c4f38eb6c5824bf51209099000f9f/> faster null-value removal
   Previously reserialization of token streams with removed null values
   was performed on a byte-per-byte basis, which was quite inefficient
   to say the least.
   
   Now it uses `io::copy` to copy in chunks of 65kb, which makes out
   our throughput and should deliver about 150MB/s at least.

### Refactor

 - <csr-id-0d655411ab9c81808f683c0ad26a1eb927cdde46/> Added bootstrap to make things pretty
 - <csr-id-05b442d5893ef46ddaf52306aef5807776bc1d05/> Cleaned up a lot of the link logic
 - <csr-id-89343654018fb1a2fb3f6955f5f0e1c3eb4fe0bd/> Converted data to a table
 - <csr-id-d0b69af41390df40f5a11d44e08d1b67167a969a/> OK version of json value setter
   However, we don't set the correct field names yet, and are lacking
   a remapping of CLI field names to struct field names before any
   testing makes sense.
 - <csr-id-464394af22714fee650ca3e310336584666f921a/> handle recursive mut json values
   * recurively drill down a mutable, recursive enumeration, without borrow
     checker issues. The obvious solution doesn't work, but should.
     Stackoverflow ?
   * infrastructure to set actual value, with support for ararys, pods and
     hashmaps
 - <csr-id-f83dff672bc5a739f1a4b76333e25d40523fbe2c/> bring in all required field data
   Previously we only knew the type as string, now we have enums and
   additional type information, like whether or not it's a POD.
   
   However, borrow-checker doesn't like the current code, will need more
   work.
 - <csr-id-5c284e1c418d93bca7da4a29c4f8feaf5800c1ce/> non-redundant data access
   Previously we would define information about the program types
   in two places, once for the index, and once per program type.
   Now within the index.html, we just load the respective program type
   information to have access to the latest at all times.
 - <csr-id-15daf311ea79a95baf5b28760c88fbbff63a450b/> use `json_tools::IteratorExt`
   That way, we can invert the flow and produce more idiomatic code.
   [skip ci]
 - <csr-id-2485343caa621bed4cca0df329abda7e61df813d/> use `arg_enum!` clap-rs macro
   That way, we get a better (case-insensitive) implementation of `FromStr`
   which reduces the amount of code we have to maintain.
 - <csr-id-ef63790422db56158e2e1a6d651e329e14cd7ec0/> better vector building
   Instead of using multiple lines to add vectors up involving iterators,
   I just add slices together. This should produce less, and possibly
   faster machine code, with less ascii code.
   
   Downloads tested with drive2, and are verified to be working !
 - <csr-id-f1fe6bac018c2268d10233ec1635f0273f1192dc/> move global params to runtime
   Global params were repeated per method, even though they were global,
   per API. Now they are kept in vectors and used at runtime, accordingly.
   We save a little bit of code, have simple matches, and less repition.
   
   It's unclear if this reduces the size of the binary though ... or the
   compile times, as the extra loop is an extra loop after all ;).
   
   Still need to test the download mode using drive1
   
   Related to #97
   [skip ci]
 - <csr-id-bbab1f2e38f4445179e7385a9507098d6ff15cbf/> use raw strings for argparser
   That way, we should be save from contained '"' characters, and whatever
   else.
 - <csr-id-f7740ad149d78b4642670ff35deb6163ab56be22/> request-value parsing;Default
   * Default::default() optimized to use T::default() if possible
   * deduplicated special type handling ('Count' strings -> int64)
   * put request value parsing into own private function
 - <csr-id-137ba8caf3b9bad5bb7d8e4a9fb236e9988476f2/> put API relevant stuff into subdir
   This is the first of many changes to come.
   We try to leverage our ability to merge multiple data source into one
   to abstract away what we are actually doing, and of course, to allow
   sharing the majority of the code, were applicable.
 - <csr-id-76827ff6659d33b7b9430e4971a7189fa0d23798/> remove map!, better dlg call
   * map! wasn't used.
   * improved delegate calls, using `match` or `delegate.is_some()` to
     get the nicest looking, shortest-possible code
 - <csr-id-5b5ad43bfa06f5c525a7e00b537381cefe6b7aa4/> deduplicate object creation code
   Everything is still working ... maybe it will all work now ?
 - <csr-id-a2550d11811de9f9ee51652975363d0f24b8d032/> into own def
   Was rather easy, and shows that plenty of complexity arose from the
   usage example.
   
   Implementing the action will probably be quite something ... .
   Can't wait to have an auto-generated sample program !
 - <csr-id-331ecf87a76189b10672770377d36877dbd7f53a/> methods useful for mbuild as too
   It's vital to be able to traverse parameters easily and consistently :)
 - <csr-id-1dc168497ee180fa3728be290c65535ba16117e2/> new _setter method
   Just to make things a little easier to read. Don't expect it to
   grow much larger though.
 - <csr-id-f1b99af5dcca4e169463a8932fcf217f9cace8c6/> move resource builder into own lib
   As the code is likely to grow far more complex, it's required to move
   it as long as it is easy to do so.

### Style

 - <csr-id-5bc4141fa584a247d507660bcbe46551789ad04a/> Fixed up indenting and line length

### Test

 - <csr-id-c9c3ad011fdb4ae693ddeef436f7a14de35ad7b0/> initial test
   Implementation has to follow next

### Refactor (BREAKING)

 - <csr-id-544be6d2a27423865d09f355785310368c4c42ed/> remove various errors structs


### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release google-apis-common v4.0.0 ([`64705a7`](https://github.com/Byron/google-apis-rs/commit/64705a75b6ece818c1a5c3c55ff686ea155c856b))
    - Prepare changelog ([`ecb10a2`](https://github.com/Byron/google-apis-rs/commit/ecb10a2ff500a1d10add9be336393a10e51ec050))
    - Merge branch 'common-crate' ([`96b3d72`](https://github.com/Byron/google-apis-rs/commit/96b3d728a3b3d76c64fa0e48198d09b2d3c023bd))
    - Prepare google-apis-common for release ([`716c4c2`](https://github.com/Byron/google-apis-rs/commit/716c4c263a278c334feacf57c3eabbed09251a9e))
    - Rename `google-api-client` to `google-apis-common` ([`8d7309b`](https://github.com/Byron/google-apis-rs/commit/8d7309b78c3bc909b794d447115328cfb0f41649))
</details>

