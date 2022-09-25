# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

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
   
   Should have been fixed alongside of #81
 - <csr-id-4f98fc175ea274d3854929c05637c337f1a6aaa7/> visual gap between cursor and kv
   Previously, the space was barely visible, confusing even myself :).
   Now it's clear, using 4 spaces, that there is a cursor invocation
   followed by a key-value pair.
 - <csr-id-12743f543d6021322915bfdd9b4d5ef6b88f72de/> add link to general documentation
   [skip ci]
 - <csr-id-6f5c1599168524c0df0b47713ea4eb1a00d049f5/> request values are moved, not borrowed
   [skip ci]
 - <csr-id-944e04bd12f6415e3818f444d2604fc103ec162b/> filled README.md
   All possible documentation was added in a quality sufficient for
   a first release. After all, everything there is is documented.
 - <csr-id-fbec9bdbba375037ec3ac5886bb86390622194fd/> integrate different program types
   * put program type inforamtion into shared.yaml to allow accessing it
     from the index.html.mako template.
 - <csr-id-b64722cca85a0396cc1389da694e7abd2338ff2e/> random values + cursor information
   * Instead of writing pod-types, we generate a random value of the
     required type.
   * Fully document how cursors can be set, which is all that's usually
     demonstrated in more complex dynamic structure documentation
 - <csr-id-47f9ca8b209e0f2453dad6d8c121e60e138f511c/> absolute top-level cursor + details
   * just for show, use absolute cursors in the top-level structure
   * indicate you are setting an array or hashmap in the details
 - <csr-id-683cbbdd753611c6f09e5111bb1aa3c29e6b909d/> relative cursor positioning
   It would still be nice though to show absolute positioning as well.
 - <csr-id-92b1ef7476d0adb9168c94b8d9bb1097ad682fbc/> dynamic absolute cursor position example
   We build all required -r flags using absolute cursor positions only.
   The next step should be to use relative ones, and of course be more
   verbose about how this should be interpreted (sequential).
 - <csr-id-6d3bbcea5713a7a868ba7e8def00ed18fda83b64/> upload and output flag
   We are already there, except for documenting the request value type,
   which definitely deserves a separate issue.
 - <csr-id-24e053718a6960466d4da69ba4113fc341646b69/> optional paramters
   Added documentation for flags setting all kinds of optional parameters.
 - <csr-id-c004840d5bbb5b5196a68b67f709008d055d496a/> inforamtion about setting structs
   For now we just have a 'dum' example, but once we are there, we shall
   make the example and documentation based on the actual request value.
   
   This requires some additional work, which fortunately has to be done
   in python only.
 - <csr-id-c65a8a6bdf9296721f21f86266f744d656f00ee9/> add required scalar arguments
 - <csr-id-334061a5e20846cf4f21847c1950f58ca4f9c87e/> name default scope in API docs
 - <csr-id-49c4a4101e73b516b67f66779072efe13a624ba6/> added CLI scope documentation
   In addition to that, they can now be set as well.
   Unified generation of the 'default' scope.
 - <csr-id-74bb79d6b4b73b0031ee233cf9a1667f7fdb8070/> update to include CLI targets
 - <csr-id-c1d09e6d576b6f6bb1245af6e0b9b166c5f69b2f/> minor phrasing changes
   Also removed superfluous 'extern' for tests
 - <csr-id-aadf37004ee609d940674f6f30ae4c942ba522c8/> deal with 'virtual' methods resource
   We assure to know about it, instead of writing nonsense about that
   'methods' resources which does not actually exist.
   
   I am relatively sure to have found all the spots.
 - <csr-id-2f293f5e1bc14b8189d38424ef24d829fedd8743/> method features and general info
   * add method listing for various categories, like 'downloads' and
     'uploads'
   * add general information on how to do downloads and uploads using
     various protocols
 - <csr-id-bec5cd5e5c12a38168e0a117adccccd6e3407e9f/> add build instructions
   These should help people to get started on their own.
 - <csr-id-6800edb4dd9b3655da231ef483780144c2b52884/> initial version
   It's still rather simple, but a basis for further improvements
 - <csr-id-4cf365d0263b66ee538eb5e31144469a3018d856/> result handling and remaining todos
   Basically there is no todo left, which puts us in a good position for
   implementing more features, and get some feedback in the meanwhile.
 - <csr-id-206ccadbb3849c27247d3670c3bf4591636b66d0/> bigger font for doc-index
 - <csr-id-9bcb3f8ba900e313bea4fd4203177851e6e86f9a/> for additional parameters
   Based on the parameters suitable for the entire API. One could also
   make them available in the builder ... .
 - <csr-id-ac35432b3f200a02a1272b3e295dcf6029e8b441/> cross linking of resources/activities
   This makes it so much easier to get to the example call you are
   interested in.
   It's getting there, slowly ;)
 - <csr-id-4b12da4a12927f363f9ce2e208a1c92f05bbda2f/> docs for terms.upload methods
   Also fs::File is now used with prefix, to prevent clashes.
 - <csr-id-182d0c6facbc80cf30c072abd930aa15a1898123/> scope docs for method builders
 - <csr-id-a7f93a93b62a908f470cc0de1164551786d1b96a/> fixed spacing
   Also, the `do()` implementation was moved into it's own def, even
   though it's still quite empty.
 - <csr-id-0ff1e07e534e33d0815676270c90109a0195ff82/> improved spacing
 - <csr-id-42ae75c1a1a2bfa148a6c52884c88ac71bcf93c0/> added info about settable parts
   It's not as good as the parts info on the website, but it's something !
   At least people don't have to read the text, but find this information
   in all the spots that are relevant to this.
 - <csr-id-4e8872b37af5bbefcbec6db8f9192d0fbf180eeb/> more information, nicer visuals
 - <csr-id-bfa20a18c8138ddd7c76a2fcdeb43d40bc884b3d/> method builder call example
   With nearly fully randomized examples to show how it can be done.
   It's quite nice to see actual calls, using everything required to get
   a call. The only thing the user has to manage is to fill in actual
   values.
   
   But, it also shows that our builder pattern doesn't work yet due to ...
   you guessed it ... lifetime issues :D
 - <csr-id-74aa7bba2d9e5e5375d15ee2500e385d4b33415b/> library overview as far as possible
   Everything we have, feature wise, is now documented in a first version
   at least.
   
   We shall keep this uptodate with what we are implementing, which also
   helps figuring out a good api.

### New Features

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
 - <csr-id-96415d17ca383ba0653fb4df23df1ebe27d57f55/> did you mean for struct values
   * functionality is cursor-aware, and fixes the actual string the user
     passed in. That way, it is made very clear how the suggested value
     is to be used.
   * it's a known weakness of the implementation that it operates on a
     flattened list of field names, and thus may make nonsensical
     suggestions.
   * added punctuation to all errors
 - <csr-id-75b80de3c644a1487358561810c7c56bad8cca1d/> `-u <mode> <file>` parsing
   * As `possible_values()` applies to all arguments, we cannot use it
     anymore but have to check the UploadProtocol type ourselves.
     Besides that, switching to the latest `clap` simplified our lives
     a little.
   * ajusted docs to not enforce using `-r` all the time
 - <csr-id-894b5b5ec7bf7cb027ba31bf83c40f27e0ab51bd/> adjust to serde usage in `yup-oauth`
   * More detailed error type for JsonTokenStorage
   * removed all traces of rustc_serialize
   * use pretty-printers everywhere to allow writing human-readable json
     files for secretes and for tokens
 - <csr-id-656fcae2b481ac90254bf5e3081d2bbd659d5232/> implement -u as good as possible
   We can't have the `-u <mode> <file> <mime>` style yet, but
   https://github.com/kbknapp/clap-rs/issues/88 might help with that
   at some point.
   
   Related to #92 and #81
 - <csr-id-db4624b46728379393372be40b1ce731fe8f28b4/> parse structure and build App
   We are currently setting everything up at runtime, and manage to get
   nearly all information into it, except for the more complex
   `-u (simple|resumable) <file> <mime>` flag.
 - <csr-id-b39bc3a9cd165db8f9ea3fa536697ca80d36628e/> initial version of command generation
   It compiles and works, even though there are many things we want to
   improve.
   
   One big question is how to define multi-arguments, like -u foo bar baz.
 - <csr-id-988d37f0dfaf8a1725bf92364e965c1f32e6802f/> setup infrastructure
   This allows us to setup clap and see if it compiles, which is the prime
   goal of the current workflow step.
   
   Related to #81
 - <csr-id-36513f101e0c3299513fe1bf542c7fc7c492e771/> simple linux deployment script
   It's made for a linux machine, not for docker
 - <csr-id-c248301951cc1266136e2ab7b6c6f5cc54d86164/> simple osx deploy script
   * added simple script to build tar archive with all debug/release
     binaries.
   * slightly improved docker script, even though it would need additional
     work. For now, I use the cloud VM anyway
 - <csr-id-a3289420337c7f607f4393fcf0832167267cc473/> improved error handling
   We are now able to decode detailed errors and pass them on. This allows
   the CLI to provide more useful error responses.
   Additionally, the CLI will only print debug responses in --debug mode.
 - <csr-id-e42f6fbedb0a2e609c6d1363a5c0eaa5b7967863/> per-API-credentials with default
   That way, we can provide better service, as CLIs that consume a lot of
   quota can easily have their own app credentials, and with it, their
   own quota.
   
   The fallback will be a project that allows to use all possible
   google APIs.
   
   The user can always put in his own application secret to use his own
   quota or even paid services.
 - <csr-id-b830c1c6decea4d5b3a16712b31daaa544cc837b/> hashmap handling
   * with native support for type conversion and error handling
   * improved hash-map key-value parsing to at least state that it knows
     it's dealing with a hashmap. Error text is still not what it should
     be because we don't know at runtime (initially) what type we handle.
 - <csr-id-c14ef9afc86a17b5bc3952882f98fc7bf7a2ced8/> repeated required args
   * Seem to work for docopt, mkdocs and code itself
   * mkdocs now show type of required params
   * some code which deals with converting elements to their
     target types is totally untested right now.
   
   Related to #77
 - <csr-id-03f35bd4f547da5843fab755ca678c01800aabed/> --debug-auth flag
   * Allow to see all authentication related communication, similar to
     --debug flag otherwise.
   * fixed broken generator when handling request value parsing.
 - <csr-id-159c65916f0fb4d0136a8cc622919daf60a7ecfd/> --debug flag to output traffix
   * If `--debug` is set, we will output all server communication to
     stderr. That way, we can compare our requests to what is expected by
     ush based on official docs.
   * `discovery` now doesn't use the API key anymore - this is specified
      using a custom override.
   
   Nice, we are totally ready to test and fix all API features.
   
   Related to #70
 - <csr-id-f5f12c559448f73a08a812f4ac40bfc6dafcbabb/> added first versions of all CLI
   That way, changes can be tracked.
   Also, we make it official.
   
   Future checkins will only be made if major changes were done,
   similar to how the APIs are handled.
   
   Related to #64
 - <csr-id-15b78cd1ff148a20006e92fd9210e93f01d9f366/> struct value parsing
   This works already for simple request values, but doens't generate
   compiling code for structures with Parts in them.
   Nonetheless, it's a big step towards finishing the overall issue.
   
   Related to #64
 - <csr-id-1dd1fcf4b80e9554bac430326fa668b18cd9c678/> field cursor complete and untested
   Tests just need to be run, and of course, the impementation might need
   fixing.
   
   Related to #64
 - <csr-id-6119bfb7627c7e238a5641b0781bfca3689e8a36/> make respective uppload_call
   Now we actually provide the information required to upload data in a
   simple or resumable fashion.
 - <csr-id-9eed4056e53d71f2b8165fd4099fda6fc0d0798a/> upload flag parsing
   We handle errors gracefully with costum types and minimal amount of
   code. Unfortunately, Mime type parsing is very 'flexible', allowing
   nonesense types to be passed easily.
   
   Related to #62
 - <csr-id-36a7cb239a2717b54500ed41a346a382b092f76a/> global optional parameters+DL tracking
   * set globally shared parameters (which includes 'alt')
   * track if 'alt' is set to 'media' at runtime to do the right thing when
     outputting the result. There is still an issue to be fixed though
   
   Related to #61
 - <csr-id-6ae6ee88a05d8d8c76f69c4bff2c37684b3d81ad/> parse method parameters and set them
   It's implemented in a working fashion, except that the default value
   is not currently set to something sensible, causing duplicate errors in
   case the key-value syntax is wrong.
   
   Related to #61
 - <csr-id-3f49f50ac21fb921b61c1170c633214782f39cc7/> handle output json encoding and ostreams
   * support for encoding response schemas to json
   * support for simple downloads (without alt=media)
 - <csr-id-c3a9f1e8e594172ac783f0b9c76e093a534674ee/> interpret output arguments
   For now we don't properly handle errors when opening files, but the
   code is there.
   Will panic in next commit.
   
   Related to #63
 - <csr-id-e34e24e04943e6cce8564295587bbf426c58077f/> required arg parsing + first doit() call
   We are parsing required scalar values and handle parse-errors correctly,
   to the point were we make a simple, non-upload doit() call.
   
   It shows that we seem to build invalid calls, for now,but that's nothing
   we can't fix once the time is ripe.
   
   Next goals will be related to finalizing the argument parsing code.
 - <csr-id-d6919f1eb65c7e29527360739555fce4a254d9e8/> infrastructure for call and dry-run
   Now we are able to cleanly handle our arguments on a per-method basis.
   The generated code won't clutter our design as we put the details into
   their own methods.
 - <csr-id-8afc76a7fe50ba8171f1e2045d989162c9864395/> Implementation of JsonTokenStorage
   It's also used by the code, replacing the previous standing,
   MemoryStorage.
 - <csr-id-f71c2862851f98c00fb893fa3b940a912b893845/> init hub + refactor for dry-run mode
   The hub is just using preset types - we will have to implement our own
   storage and auth-delegate, as well as a Hub delegate at some point.
   
   Dry run mode allows us to check for errors and use a call builder
   using the very same code.
 - <csr-id-7dc9972445593f592f369759b9839a3dedf8d12c/> Display + Error traits for Error struct
   * improved documentation about error handling, it's less verbose yet
     explains what you can do.
 - <csr-id-be228f19940d38e484809116c1bd84bb8edf5ee8/> engine checks resource and method args
   We are now at a spot where we can actually start parsing arguments.
   
   * ArgumentError -> ClIError - seems more fitting
 - <csr-id-4548644cb1498f4c7769d8e98cc7ddf8c0e4f47b/> write default and read app-secret
   * if there is no secret file in json format, we write a default one
     that we will then read in a second iteration of the loop.
     That way, the user has an example of how such a file must look like.
   
   Next step is to cleanup the error type and implement the Error trait.
 - <csr-id-5799d44fceb537f8f82ae4919682c9189a172792/> create config directory, if possible
   * Only supports one level of directory
   * full error handling, and uses memory efficiently
 - <csr-id-ca8e8c06220f858424c8c1b799b1f00bd89e9bb2/> infrastructure
   * allow usage of cmn.rs for common types (like Error types)
   * instantiate an engine and handle errors, in an initial quick and dirty
     way.
   
    Fixes #52
 - <csr-id-310c81f19cbfb8e1fc7d7f3766492c002a340761/> generate complete docopts grammar
   Grammar is laid out per method, providing general purpose arguments
   only as needed/supported.
   
   All details will be contained in the markdown documentation.
   
   Related to #45
 - <csr-id-3cef120c58d304e120ba5e86a1717f1c47452452/> per-method-markdown-files
   That way, all information can be placed within a single markdown file
   per method call. This will keep loading times low while maximizing
   usability.
   
   That way, it's comparable to the API documentation, which is most
   detailed on a per-method basis as well.
 - <csr-id-c78ea5381aeeb7c97ce4fc35e0c9da40a7022423/> cli postprocessing support
   That way, a single huge markdown file containing documentation for
   commands and methods can be split up into multiple files for
   individual inclusion in mkdocs.
   
   It's done by a post-processor which is loaded by mako-render, providing
   access to the entire context. Said processor may also drop results
   altogether and thus prevent files to be written that have been split up
   by it.
 - <csr-id-39253d988af3d7795b2167edb3a54b8988dda00c/> docopt subcommands
   Setup command/subcommand pattern.
   Next will be the infrastucture for documenting these, using mkdocs
   and markdown.
 - <csr-id-f527c8202b961d3dcb4c30a13e3c28a650fb144c/> bin renaming + docopt infrastructure
   * allow to rename executables, for now just brute-force using a boolean
     flag. If we have more binaries at some point, we might want to be more
     elaborate.
   * everything related to docopts functionality is now in the docopts
     module.
   
     Related to #45
 - <csr-id-390354bd08b429fb438d60c54e2a36756e086c3c/> basic usage of docopts
   For now we just show it works within our generator.
   Next step is to actually generate docopts grammar.
 - <csr-id-d1c97912cbebf8df3f2817b04b15a78d952b092a/> mkdocs generator works now
   It can be selected for each type of program we want to build, and makes
   sense for everything that is not a library.
   
   We also tried to unify names and folders a bit more, even though there
   certainly is more work to be done to be fully non-redundant.
 - <csr-id-cefd606b538ed86d7b659f83b64ee2b14f71fc3b/> cli depends on API, generically
   This allows us to build efficiently. CLI programs can now have their
   own cmn.rs implementation, which we can test standalone with
   `cargo test`.
   
   The primary makefile currently just explicitly pulls in the type-*.yaml,
   one day we could possibly put it into a loop.
 - <csr-id-be7d8214c16287fb245918c38561544245a0aa1d/> api generation works once again
   With the new structure, it should be easy to add CLI programs with
   proper dependencies accordingly.
 - <csr-id-29ee94b4c04f72d2676a98dda6632a06c5b8ba54/> Resumable upload implemented
   With all bells and whisles. For now, we don't have a good return value
   to indicate that the operation was cancelled, which needs fixing.
 - <csr-id-065753cc3a56227c2e87fbcc8b36121dc3bb1ab6/> implement query_transfer_status()
   The delegate logic is implemented and seems sound.
   It's somewhat funny that after all this back and forth, all we get
   is a valid start position for the upload.
 - <csr-id-42a76e465549beadd3080c36f68922d8e44fba54/> ContentRange header (parse and format)
   Now we are able to send the transfer-update requests and implement the
   actual chunk logic.
 - <csr-id-d26cf7740614134e97f1b6add19c3b91242fc994/> use of oauth2::Scheme
   That way, we improved our API, reduced code bloat, and are very clear
   about the what we do for Authorization.
 - <csr-id-8ad316bda3fd5eaa7e9a993ff1a9120e71022365/> crate version +<revision>
   That way, crate names reveal exact inforamtion about the contained
   API revision.
   
   * crate version: code gen version
   * +<revision> (build-metadata): exact version of API schema
 - <csr-id-57e0f0658379db524f1a964232a3fa39111be626/> check upload size against max-size

 - <csr-id-ffef7dda57c8f3f14d86712107416eaffe4c1bfc/> make actual `store_upload_url()` call
   We also assure to call only as often as we have to, keeping some state
   between the loops accordingly.
 - <csr-id-9ea85273cd18798c7f0c523a45de1f25c0648c92/> improved delegate calls
   The delegate will be asked for an upload URL, that he may store during
   yet another call.
 - <csr-id-307d3f487c6b35f42be643505a4e65c6ce04e6ec/> resumable-upload infrastructure
   Layout the `ResumableUploadHelper` and implement the entire logic
   with the mbuild renerator.
   
   All that's left to be done is to implement the 'chunked upload' method.
   
   The borrow checker helped me to prevent a bug as well.
 - <csr-id-0823dec75cc89b8e0a87a41ab2dcd1d5a405a24e/> don't crash if json decode fails.
   Instead, tell the delegate about it and return the error.
 - <csr-id-8bb2166da0a11db45a68e53518e94119b6d5a3b3/> mark unused types with marker trait
   For some reason, some google APIs define types they never use. We now
   mark them, just because we can, to show our superiority ;) ;) ;) :D .
 - <csr-id-bb75c5b69871ec88c888618d0c3292741c9cffff/> support for 'variant' schema
   Documentation links, at one spot, have been updated as well.
   The variant schema is represented natively as enum, it all looks
   very good.
   
   Json has been taken care of as well ... .
 - <csr-id-55978ff9a2fe332c5ed46476af4f921a72999e5c/> Option<_> in schema only if needed
   This means that only part fields will be optional.
 - <csr-id-9f719dd9287ee112fa6c3ebb6be64e9793da8a81/> added field aliases, were needed
   This makes sure our fields can properly be decoded.
 - <csr-id-d3bb130be0b25f984c75ab125d2b344929865213/> use serge instead of serialize
   However, for some reason, the `Serialize/Deserialize` macros don't work
   for me, even though they work just fine in the respective tests of
   the serge crate. What am I possibly doing wrong ?
 - <csr-id-265b448297493afe11c38ac751376c67907e84da/> simplify delegate calls
   Now we use the DefaultDelegate as standin in case there is user-delgate.
   That way, we save plenty of complexity as no additional
   `if let Some(ref mut dlg) = delegate` is necesary.
 - <csr-id-3a1543033949b8f25e2e3cd888c9f43029b4de3d/> prevent duplicate schema types
   These could clash with types we import from Cmn. When that happens,
   just a single list must be adjusted for a fix, see
   `unique_type_name`
 - <csr-id-508d14eafbca167f9801a2ca7ff9a1ae922be734/> begin()/finished() calls
   During `begin()`, the delegate receives additional information about the
   current call, which can be useful for performance tracking, among
   other things.
 - <csr-id-02d7a06fdff10d54c93d00fa18e0330e1f536162/> alt 'media' handling to allow dls
   This also includes documentation to state which methods actually support
   media download, and how to achieve that.
   
   Added TODO to not forget we should tell the user how to achieve these
   kinds of things.
 - <csr-id-4a27ac7e1d14207645915637c4817a17f10916b9/> crates with 'google-' prefix

 - <csr-id-cb5a0a35bc36cbf234e2ac5d2cec0b2c14ac1d2f/> allow to set user-agent

 - <csr-id-9d401f5486b447ea0fc43cb0d4bb84fac3329357/> optimizations and simplification; seek
   * MultiPartReader is using match to handle state, reducing unnecessary
     calls to 0 in that regard.
   * Fixed seek() calls on readers, assuring they are reset to start each
     time the loop is done.
   * both media-parameters now use `ReadSeek` streams.
   * Use `seek()` to figure out size, simplifying the interface.
 - <csr-id-224af64068c60649266aff7cc06abd001053015b/> optimized memory allocation and options
   * reserve_exact(X) where possible (params, multi-part-reader)
   * `if let` used whereever possible to prevent duplicate checks
   
   This increases the possible performance, and makes for more readable,
   concise code.
 - <csr-id-b127df17b02a4823e74a5125961bdfa23f77f7a0/> multibytereader single byte test
   It shows that we actually don't handle our state correctly.
   The first test which reads to string obviously uses a big-enough buffer.
 - <csr-id-8db346b8b01f003fed24d202822c398fa0994443/> MultiPartReader is working.
   Something that is missing is a single-byte read test
 - <csr-id-71c827b3067131a150bfd4a3503a61b836ec39b5/> initial part writing
   We are a state-machine, and handle parts of it correctly.
   However, we don't yet write the boundary at all, and could improve
   our use of match.
 - <csr-id-fc589cb965848332dd944a790cafd7d4745d9fc7/> multi-part mime-type and add_parts()
   Next we will implement the actual Read method
 - <csr-id-3ea5e194859749e05632edcfd35cc21db8cf53ff/> handle 'alt' param
   It's conditionally set to json, if we expect a response value.
 - <csr-id-b0a1f518e957c96a0f5b5b2297a738cb42032e87/> more multipart infrastructure
   * outer frame of `MultiPartReader` to allow using it in `doit()`
   * restructured `doit()` to get content-types right
   
   There is more work to do, as it currently doesn't compile, nor
   do we deal with our streams correctly.
   
   But we are on a good way.
 - <csr-id-7cfb5afd394041019899ca4cdcf10c9187204409/> improve body infrastructure
   This will support choosing custom readers at runtime, depending on
   whether we have a resumable or simple media upload.
 - <csr-id-d2bf24ca859b945e1f5ee64dc5ccdf7357d01184/> simplify URL_ENCODE handling
   More maintainable template code, with less redundancy.
 - <csr-id-1fee21de24eee4fd62151595ef7915987f7a39db/> uri-template handling complete
   We now handle url-encoding for the parameters that would require it,
   and can deal with repeated params that will match '/param*'.
 - <csr-id-54eb784a550a619b3773e44fc2ddd0b2a58ffcd2/> uri-template generation works
   This doesn't mean it's correctly implemented, but we are on our way.
   It does compile, at least
 - <csr-id-64219e7e7eed42f7491a2aba80f5e8fd7567385e/> repeated types in examples
   Made sure usage examples know how to use repeated types.
 - <csr-id-d758f410f68b84cb635a6a0633bb09b147939397/> repeatable parameters working
   The code dealing with them currently assumes they are "/" separated.
 - <csr-id-60d953a3428d11591954e7488bc46078d4765b1f/> intermed. support for 'methods'
   These 'methods' have no resources, and need slightly special handling.
   This version at least makes the generator work, even though
   it produces duplicates.
   
   However, as it is so ugly, I'd rather consider to change it
   substantially ... this feature should just come naturally.
 - <csr-id-354370705dd317b9839cf9a6ad34e22b9efe12dc/> partial implementation of url expr
   URL expressions allow to substitute values within the URL with
   parameters. However, this is not only a simple key-value replacement,
   but supports expressions that need a parser.
   
   This one will have to be implemented next.
 - <csr-id-33e85ddd29db5a75ce49718d850652c36ad7ce25/> set upload media type
   Related to #17
 - <csr-id-79cbf3ee3fccdbfadcb1176ebc319f8bbabb8b68/> add more obvious crate and api version

 - <csr-id-60adacf8d47eb43a0f82642a69c5216e79285dbc/> pre-request delegate call.
   This one is likely to change the further we advance in the upload-media
   implementation.
 - <csr-id-eef1471357e7a16f7501575bcca1d17cddf05515/> json decode and delegation
   Now json errors are handled and delegated with the option to retry,
   and all other values are just decoded according to plan.
   
   For now, I brutally unwrap json values assuming this will work, because
   it really should work.
 - <csr-id-2c79f6e3cfbf7044a061eef1ddfb6fadac19401d/> authentication with and without scopes
   It's quite rough around the edges, but has a slight chance to work.
   Will still to handle return values accordingly.
 - <csr-id-9a58b0badd0fea4220cccb953f6deb00c8edbaaa/> attempt to send json-encoded request
   This doesn't work yet, as I am unable to unwrap the client properly.
   It's a refcell that contains a BorrowMut to a hyper::Client, and
   lets just, it's complicated.
 - <csr-id-7f33cf22a5c22e3cc50dcc199604af78ba8e13fa/> add cargo.toml dependency information

 - <csr-id-dd0772f1d7e1330229bb36040686f91e088befd2/> docs and tests of youtube3 on travis
   This might already bring it close to 7 minutes runtime, which seems
   like providing us with a buffer big enough for when it is
   feature-complete.
 - <csr-id-c0a247605890be6553fa4709074b4c4ca4a199a9/> update-json using discovery API
   Instead of depending on the google go client API repository, I now
   use the original data source, namely the discovery API.
 - <csr-id-9a17ab9e4e98d8797a9912d3d5094c0e2bf9716f/> full usage example on landing page
   Related to #4
 - <csr-id-664d8225d2d5275148395828af02c0bc54b7ee24/> oauth22 -> oauth2_v2
   Related to #3
 - <csr-id-b8956103d9460c73956dbc28ca2f1684ba8b853c/> improved library names
   Related to #3
 - <csr-id-f27fda8f34e084e1532f4e6528b93e156f062503/> new github-pages target
   For import of all docs to the github
 - <csr-id-ac8c41530d082203f93d81851682d02ed5c98d9a/> now we pre-generate nested schemas
   Into a complete, global list of schemas, with additional meta-data.
   
   However, it's currently not complete, as $refs are missing.
   There is some resemblance to to_rust_type(...), which worries me
   slightly
 - <csr-id-712fed578a377c27bd6153b098ee4b3244b0355e/> part 1 to implement 'any' type
   It is a Json object, with a schema as defined elsewhere. It's quite
   cool to see this (nearly) working already. However, it will require
   us to transitively assign the required markers which is based
   on information we don't currently have.
   
   Maybe implementing this could also help to simplify name-clash checks
   or make them better at least ?
 - <csr-id-2d036b6623a6f21e7d5706b382e2bc1e28dac87c/> build all apis, were possible
   Now there is a blacklist feature, allowing to list apis we can't yet
   handle for whichever reason.
 - <csr-id-bb76832b2f317501d398f5ea9fe8ea6b12dacf7b/> new Scope enum type
   For use in all places where scopes are desired. It will also be made
   available for adding scopes by the user.
 - <csr-id-e1b7a63f0660682a1680d9651cd5c3e784b12030/> scope as property ...
   ... however, it will become an enumeration, as I don't like people
   putting in strings all by themselves. This also means we have to
   generate good enums ourselves.
 - <csr-id-aabed3858143bcd28d4b95e3831c408d3120719b/> query string setup
   It works for uploads as well as for others.
   
   Next up is to setup the head and authentication. It will be as simple
   as calling and handling `GetToken`, even though I think that there
   needs to be better support for the scope that is asked for ... .
 - <csr-id-da300e035ebc92728c5566071c26505a38b409f6/> generic result type
   ... and we actually add additional fields to our fields list.
 - <csr-id-7c6f7d5e97344e7df0f397c65209795e5b8515bc/> additional fields and Result type
   Now query params are handled completely, including clash check.
   Additionally, there is a new result type which encapsulates everything.
   
   It must be typed to the actual result type though, which can be a
   request result
 - <csr-id-6c4166094358fd236490239d12235a80b738f34f/> put all fields onto a list
   Also handle the case when the 'part' field is generated from the
   request. Additional params still need work
 - <csr-id-432faa275f89bb1c3ab00b60ff07225eec5a4489/> spike to see how delegate can be work
   To avoid an additional type parameter, we will use dynamic dispatch
   for the delegate.
   
   Having function overrides at some point seems like an excercise better
   left for version 1.1 ;)
 - <csr-id-678b6929ca7bffb4e4495272330aac02a082dbcd/> first attempt to get it to work
   With a big but ! The most simple thing to do it was to just add
   additional type parameters to the respective method.
   
   Now the type cannot be inferred, which means type-hints must be added.
   This should be easy enough, but ... has to be done somehow.
 - <csr-id-5b2d8a77a3cf17a1c5989e856b1ae2dc77613264/> media-upload doit() methods
   It's just a first step, and even though the generation works well,
   I am still missing the right Rust code. Will have to simplify ...
 - <csr-id-de0c7a4ae049b6f7fbc256d64bc363ebd8de2101/> `param()` to set any parameter
   That way, things like drive.files.insert alt=media has a chance to work.
   We should actually check for this to support various 'alt' values
 - <csr-id-66f3ae14e5f088828d6c9d772643889366934fac/> added gogole drive API
   Just to have another, different set of api information to deal with,
   and not accidentally hard-code things to work with youtube only.
   
   Prepared dealing with media uploads, and it turns out to be best to
   adjust the 'doit()' to take the respective type parameter.
   
   We also have to think about downloads, like the ones for google drive,
   which requires custom query parameters.
 - <csr-id-020300af15022124cfa0d3e1722d45ff371f924d/> ground work for upload media
   This might mean we need additional type parameters, but I will see how
   it's going to work out.
   
   In theory, we could define a new trait for Seek+Read, but this would
   mean that we couldn't contain owned streams.
   
   For max flexibility, it's better to have additional type parameters
   and use BorrowMut to allow ownership, and borrow.
 - <csr-id-48d40d45c5ee2b8dce689eb0a0457e0364246899/> request type handling part 1
   Now we will generate proper resoure methods builder calls to instaniate
   the more or less valid method builders.
   
   However, it doesn't compile yet, and the 'to_parts()' method on
   resources is still missing.
 - <csr-id-693b5c8f6a556941fcbfaf6b58f0d0dd00053a66/> build insert/update ... methods
   It's just the first version which defaults everything.
   Required parameter lists still have to be built.
   
   It's not going to be a problem at all.
 - <csr-id-582aca32494bf938889b04c60c5d3cec81872f77/> properties and setters for mbuilder
   This includes descriptions, of course, and generally seems to look
   quite neat. For now, we brutally consume all input to own it,
   but in future we might be able to put in Borrow to support them all.
 - <csr-id-942cbe18f1f237fe8efacde93fd121879924d619/> infrastructure for method builders
   Now comes the actual work of setting them up.
   Additionally, the docs were decluttered to show comments only
   were necessary. Now the code path to getting the hub is as concise as
   possible.
 - <csr-id-01db89057deca47d86355e35c86b4fb88c218db0/> Partial MethodBuilder impl
   Including documentation at least on the method builder part. The
   great thing is that fully working examples are now included on
   every type !
   
   Now more involved part starts ... namely setting up the individual call
   method signatures.
 - <csr-id-e96260bacc959aee2d3baa1353d48087637f3df9/> defs are now more readable
   This works with a new `indent` and `unindent` filters respectively.
   There are a few things to consider, but I have understood how it works
   and can handle it.
   There is some overhead just to give me nicer visuals ... might choose
   a different route, like annotations.
 - <csr-id-615a12465415cfa155271ce2fb94be9faa7405db/> generate hub implementation and docs
   This includes docs for the library usage.
   It's totally great to be able to paste example code right were it
   belongs, and also put the same elsewhere to compose more complex docs.
 - <csr-id-f1d95822f784bce84927c2a9d4134d5477495217/> def for DO NOT EDIT comments
   A note like that is now added to all files we generated, commented out
   depending on the file type.
   
   Quite neat, except that for filtering, I always have to use blocks.
 - <csr-id-e164cf73667a6b64908a1dd41c5adf91191a5237/> Traits now show up as part of lib
   Previously, they were in an extra, oddly named crate.
   Now we just make it a part of our generated codebase.
   
   That way, traits, and common code, shows up as part of the library.
   Fair enough.
   
   This also means that the types ar not reusable.
   Maybe a mixed-mode can be used if that is desired.
 - <csr-id-c1eeee0591f96e2865db1ed13900ba7b59475ac9/> add marker traits to schema types
   Based on their involvement in activities.
   It nearly works perfectly.
 - <csr-id-ba98bee62fa2e067e9bc18f6f52db8be1da35161/> LUTs and context to make better docs
   Now a context is passed to utility functions, which contains the state
   these may be interested in. This keeps it clean from global state.
   
   With the lookup tables, it is possible to figure out relations between
   types and document them accordingly.
 - <csr-id-d8edf1dcd46c6f7ae27e6f61b8aa1dea071a44a0/> first generated result ...
   ... just to keep track on how it changes over time.
 - <csr-id-a5e675e7a958327938a31ec38ddebfaf58af9f42/> generating valid rust from schemas
   It's very nice, even though there is some more work to be done here.
   It's just the beginning ... .
 - <csr-id-475163ec29e5d20e74141de76f38b88a51bfbd06/> now sets up entire project structure
   That way, we have a common library to pull in from the main repository,
   and a space for testing new code (in a partial implementation).
   
   Next there will be generated object structures.
 - <csr-id-fc15a7030f81658663ff416a86880bfde01f23f0/> improved license information
   ... and readme, and looks of author listing.
   Slowly getting into the flow, possibilities seem thrilling.
 - <csr-id-3670e4f6c98d1b04a618fa9c14d5470a7a6765b7/> LICENSE + README.md
   Readme is very initial, but the architecture is set to evolve it to
   something no less than beatiful.
 - <csr-id-4e5f2c05d93dd2f4cbf7472a8911fbd7e0463d9d/> mako-render generates output dirs
   That way, the makefile doesn't need to know that much anymore, and
   gets simpler/less verbose.
   
   \# Also
   * Added filters for rust doc string
   * fixed .PHONY
 - <csr-id-e3b6aee6d631c589cb277b999583aa460631c34d/> apis target - make all apis
 - <csr-id-2298601165f5b65f76c86f4542139965c2486e58/> can now use custom libraries in pycode
   Namespaces can exclusively be used during rendering, which is fine if
   you remind yourself of the newline rules.
   However, I also need some utiltiies that convert input data. These
   are now within their own libraries, which can be used from python blocks
   like the ordinary python functions they are.
   
   Quite neat.
   In future, most of the functionality will be in separate namespaces,
   the top-level will just assemble the main library file, usnig the
   provided %defs. That way, the main file is kept clean.
 - <csr-id-be938255bd14202cc77c6bc543c6e92060a7ccb0/> cargo.toml template
   It's quite final, and super easy to change and to read.
   
   It seems we want to use namespaces/shared implementations soon to allow
   using defs. In our case, we transform the version in a particular way,
   which is easy enough, yet I'd like to use it to make the system more
   powerful.
 - <csr-id-2d77857aaf9b6a7e1a5dc7a3f77349a3662f8c7c/> generic source/output mappings
   This includes proper handling of dependencies.
   The code is concise, pythonic and quite 'cody', but does the job just
   fine.
 - <csr-id-087a0762ac936f40bc4cec6f2281db34d9cab95b/> multiple input-outputs per call
   That way, we read the data files only once, but produce all the outputs
   we need. Together with a powerful makefile, we have a multi-invocation
   with proper depedency tracking.
   Everything will be regenerated though, even though just a single input
   template file changed.
   
   The alternative would be to have one dependency and invocation per
   input dependency, but that will read the entire json each time.
   
   Let's see what's faster/more useful during development.
 - <csr-id-30041e9c7da099c4843cd987ff34349394d8613d/> api deps generation works
   It's very pleasant to use, and worth the slightly greater effort.
 - <csr-id-20410adb786a1f35e870b38fc3b5b3140b626708/> mako autosetup and improved executable
   Now we can write mako templates, with a similar feature set as
   pyratemp. Except that its syntax is nicer, allows to do everything
   and that there is syntax highlight support.
   
   Let's see how it fares
 - <csr-id-c0bfeabbc39cd7449f59c8e1fd1fe9e5abba315a/> successfully generating make deps
   After minor modifications to pyratemp, it certainly does the job.
   
   What it **does NOT** do:
   
   * multiple outputs per template/command invocation
   * NICE embedding of code (like GSL can)
   
   It will do the job nonetheless, but mako might be worth a look
 - <csr-id-0812068c905463c10352ac194f44c9a317352647/> my first gsl program ...
   And it crashes on linux and on osx.
   What am I doing wrong ?
 - <csr-id-0c2f149b1e168497a376ce48105fa4d4089612e6/> unified make based build system
   Added all prerequisite programs in binary for easier use.
   Make is now implemented top-level, and is not expected to do too much
   work actually. It will, however, keep track of all required
   gsl invocation and make sure calls are efficient by not having
   to rebuild everything every time. That's what make does, anyway ;)
 - <csr-id-f13c2960ab8b3441a32bde892a8ee53f8497b987/> added authenticator arg
   That will allow interaction between client and authentication attempts.
   It also shows how cumbersome it is to deal with all these
   generics ... but hey, you gotta do what you gotta do.
   
   If boxes of pointers would be used, it would be easier to handle, but
   enforces a certain memory model. That, of course, is not desired.
 - <csr-id-1980f76c3240b44c306158df30793ca20ffc9461/> makefile for handling json-to-xml
   That way, it will remain clearly documented how to do this, and allow
   for efficient calling of gsl as well, at some point.
   
   Of course it will be a little more difficult for us to know all
   dependencies, but gsl could generate these as well for us, I suppose.
 - <csr-id-eebcf549295fe5b0521092bd0c79d83c416d351d/> add conversion tool and youtube api
   The json file needs to be converted to valid XML, which should be
   done by a soon-to-be-modified xml2json tool.
 - <csr-id-aaf432fb545b47a64692dda0296414edbf3017b6/> first primitive types and api
   Now it should be possible to implement first version of actual
   insert handling, with everything there is about it.
   
   That should eventually help to generalize it, as I am definitely
   not going to hand-implemented these protocols ... .
   
   The great thing is, that if done right, one will be able to truly be
   first and make an impact !
 - <csr-id-24a727fdea7c2ae47dd23b7ff571cd717ec4d870/> improved module layout
   As there will be plenty of types, it will be better to split it up.
   Also learned something about self::<submodule> :).
   
   Insert and and update should be hand-implemented just to see how it's
   working. Then there should be some investment to auto-generate this
   with `gsl`. Once the latter works ... I could auto-generate all apis,
   or adjust the go generator to create rust instead.
   
   Depends on what will be faster ... .
 - <csr-id-67b052c5f376c85ceb2f3e94e676e4906df9fd10/> figure out ownership model
   There is a central `YouTube` type which helps constructing various
   sub-builders, which in turn provide individual functions.
   
   Architecturally, it's very similar to the go implementation, but
   more efficient memory wise.
 - <csr-id-dda847607fc88ab6bb6d9646d52cd9795f7af0b3/> initial commit
   Base project with everything it will need to grow:
   * CI
   * documentation
   * basic cargo

### Bug Fixes

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
   - Added Bootstrap stylesheet
   - Refactored a lot of the logic which was being done in the html ${...}
     tags out into a block which gets run at the start of each api version.
     (hopefully this will make the template easier to maintain in the long
     run)
   
   Possible issue:
   
   I swapped from looping over each key in `tc.keys()` to assuming the keys
   will only ever be ["api", "cli"]. This hard codes the keys instead of
   getting them dynamically, but makes it easier to format as a table and
   lets you pull a lot of the logic out of the template and into a single
   block before each table row.
   
   If the types of application in `tc.keys()` ever changes then this
   template will need to be updated accordingly!
 - <csr-id-1323d0dccbb2ed7570e59b9b125f7b4a97ef7575/> use new serde map implementation
   No fun, this one.
 - <csr-id-99789de208609f0b8ca39852492fe0bbc54689ba/> build better data
   Really just what is needed right now to make it work.
   
   [skip ci]
 - <csr-id-082e51e16e9deb01e9c82ea7c8ac9be5bee80c79/> make cli publishing work
   It really needs allow-dirty.
   Let's hope that won't publish too much.
 - <csr-id-b68b2a6bf5786327afad1d95bceffa1111400e0a/> try to depend on major version of api
   Previously that didn't work due to a bug in carg,
   but should work now.
 - <csr-id-5e28a06dc0dfafd414765738fff35d019a903cab/> cli + api use a single base version
   That way we get rid of the duplication at least.
   Probably it would be enough to just refer to version 1 of the
   library respectively, and let semver do the rest.
 - <csr-id-f3d0ef45d26baaafa3b9120bebe371bce424309c/> correct link to license on github
   [skip ci]
 - <csr-id-0ba9535a1150750b80e862c8fc197819f0f25954/> handle discovery urls with $
   Some google discovery URLs contain `$discovery` or other variants,
   causing the calls to wget to interpret `$d` as an environment variable
   instead of a literal. An example is:
   `https://logging.googleapis.com/$discovery/rest?version=v2`.
   
   To fix this, the `$` has been escaped so that wget fetches the URL as
   expected.
 - <csr-id-0f14aa966e5e878612111709568b13e7a2c70345/> Add an unused field to empty API types.
   Null structs (struct Foo;) cause the following error when trying to
   deserialize an empty JSON object `{}` into them:
   
   `JsonDecodeError("{}\n", Syntax(InvalidType(Map), 1, 1))` (also known as
   `invalid type: map at line 1 column 1: {}`). The optional struct member
   prevents this error.
 - <csr-id-292dd2f34f04a2376c3d44990111d4a0fc2c400e/> URL-encoding '/' in URLs is not accepted by Google APIs.
 - <csr-id-b6f5fc6eb3e6be21d22fb667b541f13ee3881df1/> use redirect flow
   The interactive flow requires to paste a code back into the
   command-line, which does only work when it's cat'ed, but not
   if it is pasted.
   
   This should let it handle everything internally, which is
   way more user-friendly.
 - <csr-id-dc367c34751e04036e56a4d984d6b7f8f92cef4d/> relative path for custom target dir
   Using a shared target-dir is important to keep
   disk-space usage in check and speed up builds.
 - <csr-id-e9fe17ee3b5df32de65ed4017c65748eb8388a29/> don't fail by default on non-nightly
 - <csr-id-3921b6a5a071ec0dc9d803b0ae809a348c34f87f/> use working version of serde-codegen
   This update fixes the build on stable, and allows builds
   on nightly as usual.
   
   The trick is to use the latest version of serde-codegen,
   which keeps the syntex version internal, preventing clashes
   between libraries that might have different requirements.
 - <csr-id-5ca021727511b8265da1abadc48eb241ca24e3c5/> as learned from yup-oauth
   That way, there is no redudancny anymore.
 - <csr-id-bed46ba2414167fcf6563ac1766f3239765f4800/> work with latest serde
   `cargo test` will work now.
   We now use the latest serde once again, which should
   make everything better.
 - <csr-id-cb6679cb2b45022162a7e6a1b5de39b1fbbcf870/> remove cargo/config
   It seems due to a so far possibly unfiled bug, cargo fails to
   get it's CWDs right.
   
   Last verified with cargo 0.11.0-nightly (42bce5c 2016-05-17).
   
   To reproduce, just put the deleted file back and run a build command,
   such as
   
   ```bash
   make drive3-cli-cargo ARGS=build --no-default-features --features=nightly
   ```
 - <csr-id-065cfdd22f974afe9d8071e0227929464c1df796/> use hyper Bearer header style
   Considering we kind-of hardcoded this authentication type anyway,
   we now use the Auth-types provided by hyper 0.8.
   
   The incentive here was the compiler telling us that there the
   yup-oauth::Scheme type doesn't implement the hyper::authorization::Scheme
   anymore, even though that clearly was the case. Also it couldn't be
   reproduced in yup-oauth itself.
   
   This will need some work to get correct again, so this is just a crude
   patch to make it work again.
 - <csr-id-9e8a047ebfddd7a94226b8d559b03325abf7ab54/> compatibility with serde 0.6
   0.7 has a weird assertion error that might have happened
   if files get too large.
 - <csr-id-33f281360a0a5dfa93cd7e6f4f345689e86bcc3f/> choose serde-version which works
   Everything newer than the ones we see here will cause
   the error described in #148.
 - <csr-id-4bb7a33e9370f520b985f96aa8229b659320ff1d/> use venv-python to run any utility
   Previously the yaml version generation could fail if your system-python
   didn't have yaml installed. Now the virtual env is used, which is
   guaranteed to support yaml.
 - <csr-id-a2c6b58d5b8525110a5386e93c2de4f6851b95c6/> use latest oauth2 lib
   It enables using std::time::Duration natively
 - <csr-id-ef9e7f1bae2bff1629530fde14ca19ad424fc653/> use new discoveryRestUrl field for json download
 - <csr-id-b54acb7c96c842228a7ec65ff6b6edaf2b19b0bd/> use std::Thread::sleep
   However, in sibling libraries, we still use time::Duration, which
   now is a part of std::time::Duration.
   These should be adjusted, to make the usage of
   sleep(Duration::from_millis(d.num_milliseconds() as u64)) into sleep(d)
 - <csr-id-78c7d46f9ddb7b102fd59135cac5d1033f090b0a/> improve handling of error code if stable is tested
 - <csr-id-8179f3bf89991d83f6cb5689618f8ee90b3f9a5b/> get cmn compiling on nightly rust

 - <csr-id-9a2d2b576c84536a7a93deedcba68544bf4a10eb/> assure license can be generated
 - <csr-id-0bd7f2004843b4e9dcd8af366e7ffc6632fb9e41/> use PYTHONPATH for mako invocation
   That way, it will find its resources.
 - <csr-id-53c27da2e786e12a29037addde15d571c3b53b39/> improve version and library name handling
   We can now deal with versions having the 'alpha' or 'beta' suffix.
   It's rather hard-coded, but solves the problem for now.
   
   Related to #126
 - <csr-id-8dab8c01249a9f54e43aebe8a009f60935279de8/> update to latest serde/rust
 - <csr-id-8ab4fd0bd4b64eb76c77adc82aff99df17a1070c/> update to serde 0.5.0
   Serde move all json code into a separate crate that we are now using
   as well.
 - <csr-id-be894becc38296a62760a0724ea1310081e713de/> use clap 1.0.3
   * `SubCommand::new(...)` was renamed to `SubCommand::with_name(...)`
     which actually is now consistent with everything else
     (e.g. `Arg::with_name(...)`)
 - <csr-id-e129a7d3ae878a9ee78ea21fe1c0aa8b5671a5e0/> compatibility with hyper 0.6.4
   * Signature of `client::Response` changed and now requires a
     `hyper::Url` as well.
   
    Closes #123
 - <csr-id-0f61fa4c95c25c0e9f30cc10b6aa3b005d26e3ca/> adjust linux script to target dir
   Previously it attempted to find build-artifacts in
   the 'gen' directory, now these are all found in
   'target', provided cargo 0.3.0 is used.
   
   [skip ci]
 - <csr-id-615ac64ec1d86c1c00ff05a4d2f6065c866330a7/> flush output stream on CLI output
   For some reason, this is now a requirement - previously this didn't
   seem to be necessary.
   
   Don't know what changed there ... and it's odd it doesn't flush
   when the process is going down or the handle is destroyed.
 - <csr-id-d0491a4950f657c55dfbf6a16a16a64c72b9077c/> work with hyper v0.6.0
   Currently the latter actually fails to link on OSX, and requires a local
   override with [this fix](https://goo.gl/OTExmN).
 - <csr-id-27fdd8ee0c19dda409b6ca5a804edf23b8555ff3/> type-inference fails on empty vec
   Previously this wasn't the case, as the type could be inferred by the
   type of the parent-vector to extend.
   
   Apparently this feature was removed, probably for good reason.
 - <csr-id-a566b702738c4b470988645f2867966d1d288370/> make statement shell compatible
   The previous one actually required bash, instead of sh
 - <csr-id-62db3ae87c1ad71082566a2e195a1e5d2cb7219f/> add type annotation
   It seems to be required when building with an older rustc version.
   This did work in nightly, and just seems to be some sort of limiation
   in stable.
 - <csr-id-a9e0be6583fd92b9a171091b70e81bdba4ad4aa2/> work on stable
   CLI was slightly adjusted to not use unstable features.
   Fortunately, there is no serde magic happening, which allows
   us to keep it simple without using a build script.
 - <csr-id-2ad8d887cda32214dc520af5a9621366f4522fdf/> minor fixes
   * Mime crate must be used in the same version hyper uses
   * made attempted move a borrow
 - <csr-id-5483e328320412c39798ba15f26d02b90dd7591d/> expanded header implementation
   Now it compiles to the point where `Mime` appears as duplicate type,
   for some reason.
 - <csr-id-b0a41c4e788fd95f9ace6823c2e52d18f33195c9/> first big step towards syntex
   Even though there is a bug that caues {} to be used in stead of
   (),
   when exanding macros, which causes syntax errors that we have to
   workaround, it's not a real issue.
   
   What's happening additionally is missing hyper macros, which
   now have to be expanded manually. Shouldn't be a problem,
   pretty-printing when compiling is made for just that ;).
   
   No, it's sad that `include!()` works so badly, it makes
   using serde so difficult ... it's no fun i must say.
   
   Just for stable ... I am not sure if it is worth it."
 - <csr-id-d9ed001b46cc6e510ea2267fa205abd036be10b6/> clean was depending on unknown targets
   There are no per-program-type docs clean, just made it depend on
   docs-all-clean.
   
   Also added the `docs-api|cli` target to the generated per-program-type
   make help. It was just missing, even though it existed.
 - <csr-id-bcf90cbcc8f625b787596fb95eda4355e35b403b/> fix clean target for docs/cli
   clean-all-docs and clean-all-cli aren't valid targets. The current mako
   template causes `make clean` to abend reporting that it can't make these
   targets.
 - <csr-id-2cc48072344715c428c204e98ab991c8133cf4c2/> URL substitution handling
   Previously we would remove the wrong parameters when attempting to
   remove only those parameters that have been used in the URL
   substitution.
   
   The code we have now is more idiomatic and appears to be removing the
   correct parameters.
 - <csr-id-2ca05292971d50adb267305492a8c703b929e99f/> dc630d01e 2015-05-09
   * Vec::add was removed ... which forces me to write 4 lines instead of
     one very readable one :(.
     Not everything is to the better here, even though I can imagine they
     did it to prevent people from thinking this is a cheap operation.
   
   [skip ci]
 - <csr-id-ee84fefb4a46bf816fd6fcedebaa1428d12969a5/> deal with rustc lifetime issue
   Related to #109
 - <csr-id-9e64d1bd10f0cd68c8519954bda14ce784805a7a/> limit tar.gz to executable
   Previously it could re-pack tar-files and mess everything up.
   
   [skip ci]
 - <csr-id-be117767a11962e330568d1dd98035e7b142b910/> osx-tar files without directory
   Previously, they contained the parent directory, which wasn't intended
   and was different from the plain-layout dictated by the linux version
   of the script.
   
   [skip ci]
 - <csr-id-3efa4f2b12219412cdabf8535e03974b94f71af5/> filter null values of requrest structs
   Some servers, like youtube, reject null values possibly thanks to
   the reliance on parts. Now we are filtering them (in a very inefficient,
   but working way), which seems to be fine with the servers.
   
   Effectively, we seem to be able now to upload videos ... .
   
   More testing required !
 - <csr-id-3fe2732a01371ededca2c35fe7499a4bbe63c318/> upgrade to hyper v0.4.0
   It was basically just a find-and-replace to adapt to the changed names
   of Error and Result types.
 - <csr-id-9274938f9f69ecab2e8cb975467860f41466ad1d/> completed list of parameter names
   Previously the 'did-you-mean' functionality only knew the global
   paramters, but not the method-local ones.
 - <csr-id-b27c990db8a8701e2814e77136a34689be56c623/> simplified call to form_urlencode
   It now supports more generic inputs, as suggested in a lenghty
   dialog on a corresponding github issue.
   
   Required to build with >=0.2.33
 - <csr-id-d2a4e2ff8b16cb848869cc07b6c5a9107fb0a929/> added latest reference CLI code
   Just to have something to link to
 - <csr-id-89432cc64600ba0711e412c6cf6b1e06e2f11102/> gate usage of `upload_media_params`
   Previously the local stack variable would be used even though it
   wasn't initialized as there were no upload flags. Now this only
   happens if there are media params.
   
   [skip ci]
 - <csr-id-c346645fc96abf9831ce723bb56e26f95e3c5b45/> let delegate forget uploaded urls
   When uploading using the resumable protocol, we are now telling the
   delegate to forget the previously stored URL after successful upload.
   Previously it would have tried to return such a URL and thus made
   the system retry uploading a file that was already uploaded.
 - <csr-id-bf6a2ba60c364e7c30de198d335e481c0b3206f0/> handle repeated required strings
   In a single case we wouldn't properly pass on string arguments that
   were repeated. Now we handle them with a nice one-liner.
 - <csr-id-153324ebccf8a7846d9669f16c8f3ea52f0ec810/> 'about()' text for main commands
   It shows up in the help, and makes it easier to navigate the command
   tree without bringing up the html documentation.
 - <csr-id-94c821e09d2b75756dd3dfa2d5f508b079413cf1/> adjust `JsonTokenStorage` to yup-oauth
   Signature of `set()` changed to return a `Result<(), _>` instead of
   an `Option<_>`.
   
   Related to https://github.com/Byron/yup-oauth2/issues/5
   [skip ci]
 - <csr-id-2f200217f942aa0317186811dbbe95d675a17ab0/> unified error handling
   * Use `Result` everywhere, instead of Option or tuples
   * Properly handle error occurring after the dry-run. We do it in an
     extensible way, in case we need to do more than handle invalid output
     files at some point. Output files that could not be opened will now
     result in a nice error message with all the information we have.
 - <csr-id-fac50418a7156b1b2fa958008691dbb2f6cbb756/> escape subcommand descriptions
   Otherwise, we could have had invalid rust strings.
   
   [skip ci]
 - <csr-id-d46c083975201a6a4804fde9d4cec6ae0fc29479/> remove unused std_misc feature
   Hopefully this will not trigger errors elsewhere, but we will
   just find out I guess ;)
 - <csr-id-4115d50ca795ec2a2958f5f75b7681cb9f84720b/> adjust to latest hyper header macros
 - <csr-id-d0ce221ba39db621b969b8c1faad358c775502a5/> re-introduce UploadProtocol,fix CallType
   * CallType now represents either Upload or Standard calls, whereas
     the Upload variant is represented by the UploadProtocol enum.
     That way it's clear what happens, and we don't mix orthogonal concepts
     in one enumeration just for convenience.
   
   All tested APIs seem to build, verified
   
   * upload
   * download
   * request structures
   * parameters
   * scopes
   * config-dir
   * debug[-auth]
 - <csr-id-b039b382446f450a58c12d2d881dbcd00b96928a/> update docs and fix calltype handling
   * mkdoc docs grammar is now hierarchical, making the command structure
     more obvious and easier to understand. It's a nice addition to the
     auto-generated, hierachical usage of clap.
   * UploadProtocol enum is now CallType, to ease handling the different
     ways the Call has to be executed. It looks quite clean, even though
     combining upload protocols and the calltype is a bit hacky.
 - <csr-id-7a38f7e4d5dea97b5bd2cbe6b10e4619b3b45b12/> various fixes and improvements
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
   
    Related to #81
 - <csr-id-63e23dd48f7fb80268eb3bc95380b77b233de62a/> print usage if command is missing
   Also, fixed config-dir substitution in flag's help message
 - <csr-id-5320a48e68c0ee4457455c5caa5c01f322fc6c7e/> tweaks to make youtube3 work
   Mainly minor cleanup, and handling of generator branches that
   didn't show up in smaller APIs that were used during the first steps.
   
   related to #81
 - <csr-id-bac4e1a82fa331370c20a7c4843989f11974600c/> adjust option usage to changed API
   Discovery API now builds and seems to work even ! More testing
   will have to be done though to be sure.
 - <csr-id-feaa3a06ed53ae039750e2d420817116b1140984/> handle apis without media upload
   We are annotating the type of the optional protocols if that shall be
   required.
 - <csr-id-02a41296628eb0cbc0c8b7b2e86b06678e8db084/> call `iter()` directly
   As IntoIter is only implemented for slices up a sice of 32.
   DFAReporting though will reach 55, at least.
   
   Also added dfareporting-cli code to show how stackoverflow issues can be
   circumvented efficiently.
 - <csr-id-1aff3135d97435632599bf39cf5e8c5de9d773a8/> commit before un-using UploadProtocol
   We will try to wait for https://github.com/kbknapp/clap-rs/issues/87
   to allow us to use the enumeration instead of strings, as well as
   an iterator, which will look more idiomatic in the end.
 - <csr-id-8ac8d3b1cb59249d492a657fa8cd39fbe3fd99a7/> generate command data structure
   We do this in the hopes to circumvent a stack overflow.
   This means we will setup the parser entirely at runtime, which actually
   saves a little bit of code.
 - <csr-id-9a8ae4b7d66ec1b6a74316fceeccbf04a2f77469/> upload some code to help debugging
   We get a stack-overflow when trying to run the dfa-reporting program,
   and right now I don't know how to workaround it.
   
   This could be preventing us from using clap.
 - <csr-id-57808cf92adf7ff4dd65664a4a4ed3a361b60c6e/> make it work with latest hyper
   This is known to work with the master of hyper. It's probably OK
   to keep it, preparing for the next release and under the assupmtion
   that I will not be releasing binaries for a while.
 - <csr-id-de85fb43e53723d1d38d0b6e8746acc962035233/> exclude cloudsearch from build
   It doesn't have a single method, and thus is useless
 - <csr-id-c2dd9c7a020e0367bc87b20fa8054c85f48b71c1/> code updated to v0.1.6, latest CLI
   * also includes publishing tag files
 - <csr-id-4e275eaaddfd7a86ed42d04df24113015c6ea099/> CLI + API release preps
 - <csr-id-607ba745d140e5d2567a715c6ddaa775d2cf0d99/> update changed `url` crate imports
 - <csr-id-b6a48bdcd5fb215e94a00a69d11ce0ac007c2df3/> request value cursor handling and docs
   * now the cursor will only be set permanently if the -r flag is used in
     'cursor' mode. In 'cursor=value' mode, the cursor change doesn't
     persist among the flags. That way, one can easily distinguish
     between setting the cursor, and setting a field. However,
     '...sublevel.level=value' will still work as it did previously, yet
     the cursor change will not persist.
   * Documentation was adjusted to represent the new cursor style.
 - <csr-id-2f3b2d24ce2367356698b902becabb40b8636ab6/> simple and resumable upload works
   * fixed boundary syntax of multi-part message. Was --BOUNDARY, now is
     --BOUNDARY--
   * Fixed ContentRange parsing and serialization. We actually managed
     to break it last time we tried to update it to match the Go
     implementation.
   * fixed uploadType header parameter. It's based on chosen protocol and
     whether or not the method supports multipart operation for the given
     protocol.
   
   Related to #76
 - <csr-id-0bb30da78244abcf09c7d04571515e6584ccb4a3/> use only one request structure
   This works as we can just put all request-structure parsing to the top
   of the function.
   That way, we don't put the request struture twice.
 - <csr-id-be7ccb085cb5ea908fb75d0ae7cb6c91ded33bd4/> set request value to call
   Previously, even though the request was passed by reference, it was
   copied and thus our changes never arrived in the call.
   
   Now the API makes this clear by taking ownership, and the CLI code
   sets the Request value lateron, explicitly.
   
   Related to #76
 - <csr-id-6befdbc6fa730fc4a5513d2cad9e1784c580e516/> verified download works
   * implement custom scopes - previously they could be set, but were
     ignored during the API call
   * api-overrides are not yaml files for convenience. Existing ones were
     updated as needed.
 - <csr-id-f8689be4515d5693004da17bb2244a385ac1e794/> update all code to latest version
   * add new APIs
   * remove old ones
   * add latest json files
 - <csr-id-845a568b25f387c58a17752852aed63e7305c7b1/> response value json decoding
   * updated all json API descriptions
   * enabled 'pretty' printing of response structures. However, currently
     there is no way to get rid of all the NULL fields without external
     filtering
   * all structure fields are now optional - there seems to be no way
     around it.
 - <csr-id-6d84ef906e6b9ff344fd7acac3140bdad3d48e78/> implement deletion of tokens
   Previously this case was entirely uncovered.
   Interesting note: when a token is revoked, existing auth-tokens will
   still work. However, you may not refresh them in case permissions
   have been revoked. It's good as there is only one code-path to deal
   with (and we verified it to be working), and bad for the user as
   malicious software can keep using an account for certain time until
   the token expires.
 - <csr-id-e523ddb6ec9f1e9e8bcc51fbec02e364dbddaa72/> adapt to changed yup-oauth2 API
   The latter changed a lot, to the better, and we handle the new
   return types accordingly.
   
   Related to #74
 - <csr-id-797f289886d899a7e1b21216ee46218d179e38bf/> resolve generator issues
   * exclude dataflow API - it doesn't have a single method as long as
     it's in B4. See https://github.com/Byron/google-apis-rs/issues/78
   * assure ARRAY branch can be hit
 - <csr-id-a4b73cc1c4e3919cf8bf2f782d598d0840c4922f/> update make target
   Also, generate CLI. Probably there is not enough time to build it.
 - <csr-id-e730281003b4a4caad0d48c2712e5d1433848bd7/> README info + fix author email
   Please note that docker build script is still in debug mode, this
   issue will remind me about it: #72
 - <csr-id-d8fdf9df9f41719f6acb9bf3750aa8069cfab675/> scopes were used illegally
   Which caused a compile error. This was fixed by assuring the code
   uses the same function to determine whether or not scopes are present
   per method.
   
   [skip ci]
 - <csr-id-9ea3fea7750bce93c531f99b13c747c78a806b59/> (abf0548b5 2015-04-15) (built 2015-04-15)
 - <csr-id-4cf0720ef1e025737416ad5fe07eff2389c86ad8/> latest version of all APIs
   Now CLI and API and the same level
 - <csr-id-fa278a99c769e99727176f4faae081cc2d219342/> request value parsing compiles and inits
   Therefore, you can now actually make the call and the program will not
   crash due to uninitialized Options.
 - <csr-id-bf22bef77ae62d06209c70d273ecccef29a4268a/> struct access compiles ...
   ... but currently wouldn't run as we don't initialize the optional sub-
   structures at all.
 - <csr-id-bf37e515d2b5affec6296c34fbfa68fa89f7d4b9/> corrected cursor handling in mkdocs
   The trick was to use an actual list of cursor tokens that is consumed
   on use. That way, we don't loose track of were we are in the
   structure.
   
   Related to #64
 - <csr-id-4b87d909f21daff696dd81da463fae3b14e59725/> NULL default values instead of randoms
   Instead of generating nonesense random values, we just map defaults
   that represent the respective NULL value of a given type.
 - <csr-id-306852d5147d7083ff011f990c5feedcf3e338bb/> alt-media handling in CLI+API-docs
   * API-docs now adjust depending on where 'alt' is set (either as global
     parameter, or as method-parameter)
   * CLI: download tracking now works for 'alt' as method-parameter
   * CLI: global parameter remapping allows them to be named consistently,
     but map to the name required by the google API.
 - <csr-id-830529c40b6ab01381fe36f27753047a2b03244f/> optional parameter default handling
   Now we provide a matching default for each paramter, thus alleviating
   the previous issue of unncecessary follow-up errors.
 - <csr-id-fa011315c31815cf283ecff18e245553378f3cb9/> add rustc_serialize to test-crate
   A top-level `cargo test` didn't work anymore thanks to a missing
   mention of rustc_serialize.
   
   [skip ci]
 - <csr-id-76841da09801f23abef4955d76430ce1191c0b77/> optimze argument handling and conversion
   * Thanks to a generic function, we save a lot of code within main.rs
   * more effcient signature for ParseError
 - <csr-id-e45eb053d52db016342bd568d10bc368495dad86/> Display for Errors + refactor
   * refactored errors into a hierarchy
   * implemented `Display` trait for all error types, including some
     'hierarchy-aware' printing.
 - <csr-id-5b4f18d341cbd8f87d3e3792b1dfa803f7849015/> improved scope handling; fix CLI
   * in APIs, scopes will now be per-method, and if no scope is given,
     we will assume only the API key has to be set. Previously there was
     a wild mix between globally mentioned scopes and method scopes.
   * assure CLI generation works so far, for all avaialable APIs
   
   Related to #48
 - <csr-id-51ddcf74a6d1cf204156c6a018ced2f2d85c9352/> add commands.yml.mako
   It was previously hidden thanks to .gitignore.
   
   Good I made a fresh clone to see if make really really works.
 - <csr-id-acd42dfccc87f49cf5c9bf51a206da8bed9c02c2/> dependencies are now per-program-type
   Previously we put cli.py into the common lib folder, which caused the
   API to be regenerated and rebuilt whenever we changed code that will
   only affect the CLI, causing terrible turnaround times.
   
   Now the dependency is fixed.
 - <csr-id-3e0a24db0d8d25fec9457364d49106c22aee3c23/> 'bytes ...' -> 'bytes=...'
   * update all APIs to contain said change. It's not worth a republish
     though.
 - <csr-id-75e73d56d95dc4126ef39f0ae60d901a32af9954/> better subtext + rename target
   * catchier title for dev diary episode 1
   * fixed target name for clean, which was 'clean-api', but should have
     been 'clean-all-api'
 - <csr-id-6d3dc77635724602a89026477bfc0f8f785968ba/> one folder per API docs
   Otherwise, it would overwrite its search index, effectively breaking
   the search field.
   
   We might run into space issues on github, as the generated docs are
   duplicating each other and use a lot of disk-space.
 - <csr-id-b9a469c0a4e655da54940dc2876559f573c88c08/> use bytes=... when sending as well
   Previously, `bytes=` was just parsed, but not sent to the server.
   This change is motivated by a similar change in this commit:
   http://goo.gl/AvyvLb
 - <csr-id-2e74d9141313da1cc6a26149650ee59c43047f06/> fix dependencies
   That way, we don't build documentation unless this is truly necessary
 - <csr-id-6db733274d65f10a213612561a5771bf4b7b8316/> add publish state v0.1.5
 - <csr-id-34d0a7aad3b139c71b4d0dd7ca4e10c1336ebb8f/> corrected absolute links
   This only affected links in readme files, not the relative ones
   in the actual documentation
 - <csr-id-a399488c2799e1acca0961f80a6c116a3330190c/> v0.1.5
   * fix documentation link in Cargo.toml
   * adjust to latest hyper. It's not even out yet, but people
     can't build the APIs anyway.
 - <csr-id-191e822c5a93771e32e85bc5c00ef450c6719fb6/> adjust to hyper client
   * deal with hyper client not using a type-parameter anymore
   * fix incorrect documentation link (use '_' instead of '-')
 - <csr-id-6f2149b7d49ee693cc616b92f9de79f220ce6e2d/> v0.1.4
   * added crate publish tag files
 - <csr-id-9dbdcc465f45c13faa85e5489073e7b7f5e18133/> adjust invalid make target
   * `docs` is `docs-all` now. On travis, this should only build one API
 - <csr-id-dd1d191966aa41ec66c5a4baba5ebd43771c3a05/> v0.1.4
   * macro 'alias' was renamed to 'rename'
   * fixed `cargo test` on main project
   
   The latter pointed me to the serde issue, which would have made
   everything fail when actually used to communicate with google servers.
 - <csr-id-3403bd1c5cec379cd2ad98040cca0ec6a4eef4a3/> v0.1.3
   * keywords are no longer than 20 characters, which is a restriction
     cargo imposes
   * don't use 'homepage' link in cargo.toml unless the homepage is
     non-empty
   * Added all publish-results to mark the respective crate version
   
   Related to #46
 - <csr-id-99f8b65f75822d54f32100655d0b5678f43a8478/> version 0.1.3
   * builds with latest beta/nightly
 - <csr-id-91861dcb71b371e8ec5511ddedee0ae45cee9af0/> rustc (be9bd7c93 2015-04-05)
   * using std::convert
   * update to latest hyper (and other dependencies)
   
   Related to #46
 - <csr-id-919ae4d8ae85f35f54c69c8c222ba43ba304e263/> github-pages index generation
   Previously, we forgot to pull in the new type-specific dataset, which
   caused the index.html.mako file to fail.
 - <csr-id-a2ca1cb28ec1ce9f5f381f55ea78aa59a56ea915/> check-in of latest sources
   This also includes crate files to remember which
   crates we have published already.
   
   Related to #44
 - <csr-id-c7fb7c409343f19e26f1c3d488718decec7990b0/> set the API version to 0.1.2
 - <csr-id-e953535473429b01293d679e23337b74645e0c18/> incl. `Result` conform to standards
   Related to #44
 - <csr-id-d1c5bf1e4ab2a91c30d2bcbd1e08a1a02c73ad41/> remove newlines interpreted as test
   When documenting mandatory arguments of a methods builder, it was
   possible to document 'parts', which have a long multi-line description
   with empty lines inbetween. This caused the indentation to be funny
   and tricked rustdoc into thinking these are indented doc-tests.
   
   Now we remove these empty lines, also hoping we dont encounter lines
   with just whitespace in them. The latter would require a tiny update
   of our regex.
 - <csr-id-e5b013e97c56040dba266a43a8308448a32645eb/> remove custom Result Enum
   Instead, we just use a customized `Result` tyoe and thus stick to
   common Rust conventions.
 - <csr-id-fca1b24cd186b090f75e35f362c8bbb2754e3e4d/> update json files from discovery API
 - <csr-id-ea161897f5fe25e024292755c753f2410211bea1/> typo fixes and misc. improvements

 - <csr-id-6ad0c2ef79a634d4cb631a36eb92b2cf82b59121/> whitespace and trait rename
   * `ResourceMethodsBuilder` -> `MethodsBuilder`. This is now precise
      enough. Previously it was just to similar to what's now a
      `CallBuilder`
   * Fixed whitespace issue in `doit()`
 - <csr-id-04f4c95688f2cef0866ce07da68ae9d710596c7c/> upload size now taken properly
   Previously, it would query the size from the wrong dict and obtain
   the value 0 all the time. This would have made every upload fail with
   `UploadSizeLimitExeeded`.
   Now we obtain the actual size limit, and will ignore it if unset/0
   for some reason.
   
   Patch += 1
 - <csr-id-3bc930ae47c2544de4825ecec5346f53626a75e2/> 0.1.0 release
   * Added all APIs to source control
   * upped crate version
 - <csr-id-cd1ff18ba94966088a779b26347dc683f1f0c2d3/> upload() return value handling
   Now deals with Cancellation and non-OK status codes correctly.
 - <csr-id-556906ca60a90fc6eb34917d42813daf9792fbcb/> re-export types used by delegate
   Otherwise, delegate implementation might not actually be possible.
 - <csr-id-3a9aa519496be9da6283b847f38d9a2deaf682aa/> better introduction and version handling
   Make it cristal clear what the crate version means, and what version of
   the documentation you are looking at. Also do this in the README file.
   
   Assure that 'Google' is capitalized.
 - <csr-id-030c40d2699196e29d1c8606d042403df52a7534/> repository/source-code link
   Previously it pointed to a timestamp file. Unified repository
   source code link generation, and simplified 'deps.mako'.
   
   Related to #38
 - <csr-id-4bf280079ed5cf33c4ed2617c3aa62151ec0dcd0/> simplification and cleanup
   * renamed `*MethodsBuilder` type to `*Methods` type
   * renamed `*CallBuilder` type to `*Call` type
   * greatly simplified `doit()` signature if uploads are involved
   * pass `auth` to upload helper
 - <csr-id-98f4bbab4774fb166936c60cbe8eee2302f35052/> schema_markers() accessed map incorrectly
 - <csr-id-80161f72be1aa7f7551603c90752793c84eedb6d/> prune unused and ToParts trait
   * do not emit unused types. Sometimes though, rustc doesn't seem to
     detect that attributses are actually used
   * ToParts trait is used and implemented only when needed.
   
   Linters are back to 'normal'.
 - <csr-id-0152138e0c019575caa3e40f87f19382d92a63ac/> pretty names for methods and resources
   Previously, it was possible for methods to have '.' which showed up
   in the documentation. Now these are replaced with ' '.
 - <csr-id-5ff22851faec165258e5c3ff9c6eed58df3efee3/> exclude those with recursive schemas
   They currently don't compile as Box 'serde' is not supported.
   See https://github.com/erickt/rust-serde/issues/45.
   
   Related to #34.
 - <csr-id-8d9f175f917ec19e4752c5c3806f6f5624e066e2/> make recursive types possible
   Must be `Option<Box<T>>` now, whereas a simple `Box<T>` worked
   previously. Anyway, serde can't decode/encode Boxes yet, so
   plus1 was removed from the list of APIs to build.
   
   Related to #34
 - <csr-id-10dfeeb1aa5a1de2919e9753444e8e63855d1285/> MethodBuilder -> CallBuilder
   Find-and-replace. It seems to build and work correctly, still
 - <csr-id-97da926e28d7ad7ed90d12b7ff48477bcf67ee68/> improved markdown for library overview
   And names of free methods, which previously contained '.'. These are
   now spaces.
 - <csr-id-ff385e5cacb43d173912243fc033578b0c0b0f63/> just add latest youtube code
   It's good to see what actually changed in the json realm.
 - <csr-id-cfb8faefb8545114ddadea59871214b35e515d5a/> Vec/HashMap are Optionals
   That assures that we can decode partial server responses, and send
   partial structures as well.
 - <csr-id-b9a81a900ec054b102ce045cf25a4348c297f260/> serde cleanup;JsonError pub fields
 - <csr-id-b6ebb1ec371c833ef7386264ed9522b880586316/> prevent type-clash with `Result`
   This should have been fixed in previous commit, but wasn't.
   Actually a change that fixed one API, broke the other, and vice-versa.
   
   It's kind of a hack, because it's tailored to fix particular cases only,
   yet I believe it's contained enough to say 'good enough'.
 - <csr-id-a05426e79b8c0773dbb219b327539431e4d1fdfc/> some links pointed to old doc name
   With one of the recent changes, the crate name was changed to be
   different from the library name. However, there were still plenty of
   places that would refer to the library name instead of the new crate
   name.
   
   That way, links in the README.md as well as index/index.html still
   pointed to the old location.
 - <csr-id-6b2301351f6792fb37b7dfec6c1f0592fdc6b9cc/> MultiPartReader test case
   Simple fixes, required as its API changed
 - <csr-id-e53e23a893ce6d59777b8b53f94770d5c3c86b9c/> MultiPartReader now works correctly
   The state-handling was incorrect, causing it to not handle small reads
   correctly.
   However, this is working nicely now.
 - <csr-id-29d9e45c9fc8bbdbed23d3d5a9be20f8023bb22d/> fix lifetime issues
   Those were totally real, actually, and I am happy the borrow checker
   exists !
   
   Only one weirdness happened with RefCell<BorrowMut<C>>, but it could be
   fixed by checking actual types using `let foo: () = something_nasty()`.
 - <csr-id-b90a1916889b2d1cc6c595c3cd121739223db345/> repeated params string addition
   It seems odd that String + &str is required.
   In any way, previously it would try to add i32 to a string.
 - <csr-id-863a98c0d7932475dc207d204ec91c26ddec326c/> repeated parameters docs improvement
   Previously, it said it would 'set' the property, which is not the case
   after all.
 - <csr-id-63997910decf909a8242a8a7f16f6a4c276e1d67/> regenerate .api.deps less often
   It took too long to do it, so the 'MAKO_LIB_FILES' dependency was
   removed. It can be re-added if needed.
 - <csr-id-79879daf1b2a52593d2bc9b51ba244bfaddcf1f0/> decent solution for free methods
   Now I just add a 'virtual' resource, which is called 'methods'.
   The good thing about this is that it works well standalone, or
   in conjunction with actual resources.
   
   Also the system works with it just with minimal changes
 - <csr-id-91f69ffd6ed85790d8b6d1c8b5b63d7f4c7e6259/> unit-tests work once again
   Added missing Result cases to make match exhaustive
 - <csr-id-1349c786b7e986511e4c2ca058d45bebb7f458dd/> remove BorrowMut until it's cleared
   See stackoverflow at http://goo.gl/f27zJkj.
   
   Now we can actually call out client and move on with handling the result
 - <csr-id-814c9c9ffab64a7607f4056fbad4203ea8f19991/> user lower-case library names,always

 - <csr-id-876772cf2296c4b7c80c2f828e245c903da67802/> force python2.7 in virtualenv
   force the usage of python2.7 on systems where /usr/bin/python points to python3.x
   
   fixes issue #12
 - <csr-id-31efbf4fb0033b9f1fdfae0054ece1717ec05b79/> incorrectly capitalized cargo.toml
   This caused cargo on a case-sensitive file-system not to find the
   cargo file, which made it to look upwards in the directory structure
   to find the correctly named Cargo.toml fo the 'cmn' development
   project.
 - <csr-id-4c657ac9d132257a392bfbf2ed861142b6baf36a/> explicit subshell for cargo-doc
   Previously, it was only executing for cargo $(ARGS)
 - <csr-id-a87fbdf0a86cfa410c79671aee931e3bf95fab11/> try using a subshell for cargo cmd
   Apparently travis doesn't execute cargo in the right sub-directory.
   Might be a difference in the way make works
   
   Related to #8
 - <csr-id-51d05d6db01edb4f78159c3c07d77d0aceb85b89/> fixed dependency to wrong target
   Which caused the cmn.rs to be missing, and the build to fail.
 - <csr-id-5fd7cb511407de7176dc07c1443ef07075c063a4/> install virtualenv automatically
   The only dependency we really have is python, and wget.
   Pip is not needed !
 - <csr-id-8006bb8ca910b14ece8dee6230d476a361c7c163/> fully qualified activity names

 - <csr-id-b43eb0e301c068500777fe580c1bd1017d0819b1/> Do not generate docs !
   Previously, travis would continuously overwrite my combined docs with
   the ones from the dev-project, and make them useless.
   
   This has been driving me nuts ! Good to have it fixed !
 - <csr-id-6167dc07fc63cec22a8d2b01fe69f05f03ac3f9a/> added milestone link
   It's important to the project, and should thus be listed there
 - <csr-id-c8061ebe2fbe97274c68b7af6e5a8d08c0245139/> use function to make links correctly
   It will automatically handle rust-doc mode, and use relative or absolute
   links respectively.
 - <csr-id-4b9dbb28ff474661855f53143862b621e650f157/> assured it handles '0' correctly
   Previously, it could possibly take a '0' away from the start of a
   version. Now this is definitely not possible anymore.
 - <csr-id-97b2649094cc225d0cfc42857140f0d245e11352/> make 'regen-apis' work
   Thanks to changes in mako libraries, it won't work anymore without
   the template directory set
 - <csr-id-7758f99ff2e19c3518eddcfca2e1adeee12e0659/> typo
 - <csr-id-4f794ef5ff7b5a068a568056d2bfd7372ec9b57c/> fix incorrect nested type names
   There was a name-duplication which led to un-inmplemented types.
   
   The good thing is that this was the last issue that kept all 72
   APIs from compiling.
 - <csr-id-7e243936f226f6e26d2b551765b62cddc866776b/> finally, we pick up all types
   HashMap types were missing previously, but now it seems to be picked
   up quite nicely.
   Would this mean we do the type-setup correctly, everywhere ?
 - <csr-id-00de2b187d74fd78f049a13d1517fc91d218da71/> transitive, minimal traits for types
   Previously, I would just assign all useful traits to all types, no
   matter on how they were actually used.
   Now it builds all dependnecies and considers them when assigning
   traits, which is as precise as we need it.
   
   This is important to us as the `Json` type is just encodable, but
   not decodable. Fortunately, we just have to encode it, but in theory
   this makes it hard to embed any json in a known structure.
 - <csr-id-e3ab233a6cee8482c1c98b1e2c759e7a17cceab9/> no unused types anymore
   Due to shared global state which was altered, we got wrong results.
   This is fixed now, thanks to a deepcopy. Amazing, how altering global
   state is always biting you, even though you are convinced it's safe
   to do in just this case !
   General rule: Just don't do it, no matter what !
 - <csr-id-de40a8bd1ee8759287cd2a489cc5d995c296a07e/> improved camelCasing
   Previously, it was possible to get types like Foo_bar, which is not
   desireable.
   Now it is totally impossible to see such blasphemy ;)
 - <csr-id-614539a925c5e64508fa28506b1c6db3ccd96882/> protect from nested-type-clash
   It was possible for a nested type to be generated with a name that in
   fact CLASHED with an existing schema type. What are the odds !
   
   The clash-check added will just verify against clashes with schema
   types, which seems to be doing it for now.
 - <csr-id-32145e645ea29ff43c451530906356564e12f817/> nested type names are consistent now
   At least so it appears.
   The implementation doesn't look totally clean to me, as it seems
   similar concerns are in different portions of the code, which was
   merely tuned to work together.
   
   It could break appart if someone - me - wants to change it sometime
 - <csr-id-538120f7d1425e026220211857658a775c958577/> scope -> add_scope
   This is not only more precisely saying what it does, but also doesn't
   clash with scope parameters on resources ;) (happened in dfareporting)
 - <csr-id-dfcd554faa36cbcdf18ab985c2aed744dd45dc6d/> improved nested array type handling
   It needs deduplication though, coming up next
 - <csr-id-da57505567a58b59f320016d92b50f1ea248067c/> prevent struct recursion issue
   This works by just boxing types which are nested within themselves,
   breaking the recursion.
 - <csr-id-9b308bb6ddebe979abca6f46da131c822f95c639/> nicer code and identifiers
 - <csr-id-54540e695a9b246ca3d412ab62e843e4dd7974d0/> nested types work for arrays
   Thanks to removed code which made no sense to me, I put in a bug.
   Now the code is back, beta than ever, and documented as well :).
 - <csr-id-50fa189a715332a7ce49fc7a9c95e5a1ef22b81f/> now deals with non-objects
   These are arrays or HashMaps, which are nested types too. This is used
   to have custom types of standard vectors or hashmaps, which resolve
   to NewTypes in Rust.
 - <csr-id-a268be27d2123a77259fa1d7d1f831c7e72c4459/> optionals are working once again
   A bug was introduced which caused nested-types not to be optional
   in situations were they should.
 - <csr-id-5d563c88a8e3ccb33ebe381b47beb6ecfd4444fc/> nested type resolution and hashes
   It seems we do it better than the actual Go implementation, which fails
   to detect that scopes are actually having a string member.
   
   However, there still is an issue, as it's as hashmap for us, but just
   a member for go ... lets see ...
   https://developers.google.com/discovery/v1/reference/apis#resource
   shows that we implement it correctly :) !!
 - <csr-id-559cb8fe458e18fec05d0ca3cd2847fb981f2da0/> remove compiler warnings.
   Also, a build issue was fixed when schemas were no objects.
   However, I think I will have to check this one anyway
 - <csr-id-bfc392291666a40cf3fbe4db3dfeda69d23018fa/> no compiler warnings
   This involves disabling the dead-code lint, which is just to ease
   debugging, currently there is a lot of dead code as 'hub' is never used.
   
   Soon, this will change, so the lint will be enabled again.
 - <csr-id-efe56ad25081b632f1e65fd8292e9c4d535659bc/> deepcopy dicts instead
   It was possible for writes to happen in nested dicts, causing global
   data to change and confuse the system.
   Not that I wouldn't be aware of that danger, but apparently I didn't
   see the recursiveness of the call tree :).
 - <csr-id-cf258bf4e5148723940cc757ec032b5aff814f1e/> fixes to help more projects to build
   Involving
   * complete list of reserved words (keywords in Rust)
   * use namespace for otherwise clashing types in cmn::, io::
 - <csr-id-d99ba9c5b3c5f73ad148679a866698c811eec495/> fix name clashes
   Scopes could be invalid, previosly, and the hub type could clash
   with other types provided as Schema.
   
   Also, we used reserved identifiers
 - <csr-id-df9f0299bf5db0b7affdd90b4dfb331c74f543f2/> deal with missing auth information
   Now all APIs can be built successfully, which should help to
   prevent things from getting hardcoded in any way.
 - <csr-id-c7e169dff3712ff5f73497d2d9cba3303a83277a/> resource-to-category map
   It allows to obtain category, which we previously dropped
 - <csr-id-7816cc81455c1c7a48e84289e176baf25e8480e2/> do not degenerate during activity_split
   First step, next one will actually be keeping that data ...
 - <csr-id-1e332ddb91540c19586e6d85869c8e54c47552b0/> asssure candidate is in mapping
   It seems nearly nothing can be taken for granted ;).
   It's best to just run against a big set of APIs and fix issues as they
   arise though.
   
   More flexibility means more maintenance, after all.
 - <csr-id-92d8fa76d0f419738e2efa7df3deebb974c1e0cf/> intermediate improvements ...
   ... it shows that the override I used previously won't work for `admin`.
   Therefore we have to keep the actual value, instead of degenrating it.
   
   Makes sense ... it's interesting how much one tends to hard-code things
   to work just for a few cases, unless you opt in to see the whole picture
 - <csr-id-ff5cbb3bf410276fbe5af8cc966ac363e448970c/> ignore beta/alpha,assure latest
   There were a few bugs in the generator program, which caused old
   versions to be picked up, and alphas/betas
 - <csr-id-2531011fc579df4edc38b15de459c135975fa077/> now with flattened activities
   That way, we don't have recursive method builders, but instead
   flatten the hierarchy. In our case, this is easier and means
   less to type. E.g.
   
   `hub.user().message().import(...)` now is
   `hub.user().message_import(...)`
   
   In go it would look like this though
   `hub.user.messages.import(...)
   which is neater.
   
   We will never have that though, as it will initialize a massive amount
   of data right on the stack, even though only some of it is ever used
   ... .
 - <csr-id-35bd1c3e9c8a6ab52068e279d8f925eea8af055d/> first recursive resource support
   However, this also means we need recursive builders, which is tottally
   unsupported for now ... .
   
   This means we have to generalize rbuild generation ... could be easy.
   Lets see
 - <csr-id-3b7e63f28675ea2646c88dfa16c62c063e076b96/> make scope gen work with gmail
 - <csr-id-6d2b0fc2649bc5203c07c29dd020b50550d15746/> scopes are sorted Strings now
   That way, we make retrieved tokens independent of the order scopes
   were passed in. Additionally, we can pass any scopes, just in case
   someone uses one token for multiple APIs.
   Let's keep it flexible.
 - <csr-id-28878e0618cbb5632a1353ceb2048a913e9355d2/> Manual scope parameter ...
   ... however, it should better be a set, and there must be a way to
   control certain global names using the configuration :)
 - <csr-id-1423e46210d95d823ff9bee9896cf407b0e9f0cc/> it now works in every which way
   Custom property annotations have been very useful, to steer very special
   cases.
   
   It's also good that now there is enough infrastructure to deal with
   any amount of additional type parameters.
 - <csr-id-baea071a6f1c52410c0ca79cf24ab325f6efa586/> added size and mime type support
   This information must be provided, I just forgot it
 - <csr-id-6fad7600a03f2f6a3964f309fc8e277b34f8aa60/> doit() call with enum type annotation
   It's syntax I never used before, but ... works !
   Now lets try to get the BorrowMut back
 - <csr-id-0d9f6363eb271f95624559b06cfd07ab6b5bc9b5/> recursion for nested types
   Drive has recursive nested types, which were not handled preeviously.
 - <csr-id-4bdee961d19fc6fc6cb3cf322dfb85d2769bbcee/> examples section in mbuilder got lost
 - <csr-id-fad0a7177aa296aa777b45d0001effa36332d24e/> filter request value props by parts
   Previously, it would just show all parts.
   It's still not correct though as this isn't necessarily the parts used
   in the request value, but only the ones in the response value.
   
   It's as good as it gets though, that's all the information contained
   in the json.
 - <csr-id-a3206abc92d7bc9d829a1e2e00dbd299c379f2ab/> method builder examples work now
   It was easier than expected, but in the end needs quite some custom
   code and handling. Good to have mako (and python !!!)
 - <csr-id-9cbb2adc5a65bece45e524a71f2d66160f7aa133/> have to handle required/optionals vals
   Of course, it's ok to do that, but ... of course it doesn't make things
   easier. However, I want these examples to be representing the real thing
 - <csr-id-f2dda421e64e9164557d5b3b94604bcb2be49254/> remove empty '/// # ' lines
   They seem to make cargo fail to build valid doctests. Might be worth
   a ticket !
 - <csr-id-70ea612f19fbe7e1ef0a01b0d399fb357a46c390/> fixed part handling,it compiles now
   What's missing is docs, which will see some work now.
   I guess it will be best to hide all the prelude from the user, to allow
   him to focus on what's important here.
 - <csr-id-452b658c27e265c6a2df90ea56502db338957154/> setters now copy copyables
   Previously, they would take everything as reference and clone
   everything unconditionally. Now we do it only as we need to do it,
   no extra work incurred.
 - <csr-id-8746f5e0e20297ce58203da01638fafad155132c/> using visual markers now
   Makes everything evaluate faster, and is good enough as well.
   Besides, you don't have to think about whitespace too much, keeping
   things simpler is usually better
 - <csr-id-8dc5e2a53dbe4d620e97089e2af9e3a94a82a4a4/> perfected trait recognition.
   However, they are not listed as traits of the youtube api. What we
   really want is to list common implementation types as part of ourselves.
   
   This doesn't work though as long as we don't have the common impl
   as part of our sources.
 - <csr-id-f4030f02841521220fa52856fa733b828a59ab6b/> now the map is complete
   It's quite nice - next up is marker traits !
 - <csr-id-bb04b60dc405d74765161bc75e35b4de72c5dcc4/> dependency handling:dirs with timestamp
   That way, make will not regenerate unnecessarily
 - <csr-id-ddb48a4303a7a0653898e9eea69b3d358a14fa0c/> make all pods optionals.
   That way, json conversions will always work, which is probably what
   we desire (especially when handling server answers).
 - <csr-id-49c2ffb8e0f02698657aba46a7b34981258c6e35/> now docs look good too
 - <csr-id-317554aff398a823beae63fa09a6014ee1508f4b/> unify generated constants
   like library-name. That way, they are always the same, even if I
   change my mind.
   
   Good coding style is easy, using the current setup.
 - <csr-id-11b6fe212ff33c1b2378997411cb11524d73a81c/> mv youtube-rs to google-apis-rs
 - <csr-id-c3d399e91a6fea7a09316f018865815214a14be8/> handle whitespace and add GENINFO
   Also, remove obsolete pyra files
 - <csr-id-179c64c5e74c7a783a3dc4ef68e900440e587c83/> is now self-contained
   A little more than the promised 500 lines of code though ;).
 - <csr-id-e06738a7bd49538d402f8c995710cf231d47221d/> removed gsl, added pyratemp
   As GSL failed in my first attempt to get the example program going,
   it might be better to try something else before too much time is spend.
   
   Fortunately, pyratemp **seems** to be something usable, and even if not,
   it might be possible to make it usable as it's just a 'simple'
   python script that I might be able to understand, if need be.
 - <csr-id-f2ca8c3fb79e482ca39d3aeb40be9b8c7f9c58d8/> fixed dependencies
   The make deps generator should only care about the shared xml
 - <csr-id-e081017cb3631df007937fe4bce09c554e8c58c0/> forgot to add shared.xml
   As XML files are ignored, I didn't see that.
 - <csr-id-e83b063f0527d7e5253f14a22c90fd3b4197584a/> works exactly as needed.
   Producing non-malformed pretty xml
 - <csr-id-e0724fb56f4a49fc5da4d6b5ea75dd1029ee9a44/> xml.tostring works now ...
   ... but it still generates invalid output due to scopes.
   Should be an easy fix
 - <csr-id-143aa6fd8638b3541d71954c6e3493bc961813dd/> make it handle top-level keys
   It can now handle multiple of them.
   
   However, conversion fails, as the bloody xml converter can't handle
   booleans ??? WTF
 - <csr-id-d4869cfefc58db4580e98e8dd1ae040c81083ba9/> make sure we get correct openssl vers.

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

 - 2863 commits contributed to the release over the course of 2766 calendar days.
 - 642 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 7 unique issues were worked on: [#173](https://github.com/Byron/google-apis-rs/issues/173), [#269](https://github.com/Byron/google-apis-rs/issues/269), [#271](https://github.com/Byron/google-apis-rs/issues/271), [#281](https://github.com/Byron/google-apis-rs/issues/281), [#296](https://github.com/Byron/google-apis-rs/issues/296), [#328](https://github.com/Byron/google-apis-rs/issues/328), [#357](https://github.com/Byron/google-apis-rs/issues/357)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#173](https://github.com/Byron/google-apis-rs/issues/173)**
    - Remove reference to 'intentionally using an old version of hyper' ([`823b1a2`](https://github.com/Byron/google-apis-rs/commit/823b1a270a4b568d8f9cce40cf917f29dab28a25))
 * **[#269](https://github.com/Byron/google-apis-rs/issues/269)**
    - Don't re-borrow a refcell while it is still in use ([`9e0133c`](https://github.com/Byron/google-apis-rs/commit/9e0133c1b4bc35d9a3e35ee587533ee0f5ee57c5))
 * **[#271](https://github.com/Byron/google-apis-rs/issues/271)**
    - Assuure Delegate is Send ([`3b087bf`](https://github.com/Byron/google-apis-rs/commit/3b087bf3e284165df4b084b9360077cd8f1022e1))
 * **[#281](https://github.com/Byron/google-apis-rs/issues/281)**
    - Also run 'docs-all' on CI to assure CLI docs keep working ([`18c4ea9`](https://github.com/Byron/google-apis-rs/commit/18c4ea9ec93df7552bc0f25977bec8afa39d4796))
 * **[#296](https://github.com/Byron/google-apis-rs/issues/296)**
    - Explicitly specify `Send` requirement where dyn traits are used ([`74621c9`](https://github.com/Byron/google-apis-rs/commit/74621c9c372af3d0417c8a0b701d84686faa36ec))
 * **[#328](https://github.com/Byron/google-apis-rs/issues/328)**
    - regenerate bigquery 2 ([`77f097d`](https://github.com/Byron/google-apis-rs/commit/77f097dda2e1dcb41eb40391e5a59d48ad2341b1))
    - Make tuple-structs with public members ([`bfdb767`](https://github.com/Byron/google-apis-rs/commit/bfdb767a06f90c26512ec9c6a5e2d1939d7db007))
 * **[#357](https://github.com/Byron/google-apis-rs/issues/357)**
    - regenerate all APIs and CLIs ([`7abe6a3`](https://github.com/Byron/google-apis-rs/commit/7abe6a3de2c3de713d9d4754c880897f85a84847))
 * **Uncategorized**
    - Merge branch 'common-crate' ([`96b3d72`](https://github.com/Byron/google-apis-rs/commit/96b3d728a3b3d76c64fa0e48198d09b2d3c023bd))
    - prepare google-apis-common for release ([`716c4c2`](https://github.com/Byron/google-apis-rs/commit/716c4c263a278c334feacf57c3eabbed09251a9e))
    - rename `google-api-client` to `google-apis-common` ([`8d7309b`](https://github.com/Byron/google-apis-rs/commit/8d7309b78c3bc909b794d447115328cfb0f41649))
    - Remove obsolete code for `client.rs`, replace it with a note. ([`013dc54`](https://github.com/Byron/google-apis-rs/commit/013dc54ac8903faebf4818c3edd8eb461cb7e7f4))
    - Update common crate ([`1cc9571`](https://github.com/Byron/google-apis-rs/commit/1cc9571b1f279718d14b4d3b2130707a5ad72238))
    - Remove yup-oauth2 from API crates ([`3fc9a07`](https://github.com/Byron/google-apis-rs/commit/3fc9a077d298edd259827820fd33bff6c28ce9c7))
    - Split out common api client code into separate crate ([`e6be1ed`](https://github.com/Byron/google-apis-rs/commit/e6be1edb45d67c71f1b9552a728a2fe49924dbee))
    - Merge branch 'generic_oauth2_clean' ([`5e9fbf1`](https://github.com/Byron/google-apis-rs/commit/5e9fbf1f7fb813bb49592f1ed1caaea9745252e5))
    - update changelog ([`8e899ae`](https://github.com/Byron/google-apis-rs/commit/8e899ae23a540871eaeff78c7a682db2817c0ef8))
    - Replace () with documented NoToken for clarity ([`e61090c`](https://github.com/Byron/google-apis-rs/commit/e61090cc75586f7bdcb78b78b849ce642922363c))
    - Update root Cargo.toml file ([`586a7d3`](https://github.com/Byron/google-apis-rs/commit/586a7d32648fe86cea48462c9a9599dd61f4c0d6))
    - Update mkdocs.yml.mako ([`b339790`](https://github.com/Byron/google-apis-rs/commit/b3397902c029db8debb482c1b378938fe7ccac3c))
    - Use key-values for nav ([`49ff295`](https://github.com/Byron/google-apis-rs/commit/49ff295ab9cee7c4f0163fff4571eedde2f53618))
    - See if this fixes CI by naively swapping 'pages' with 'nav'. ([`fedf484`](https://github.com/Byron/google-apis-rs/commit/fedf48445febb012d50286c9d73808cbf4cc636e))
    - Add yup-oauth2 error to auth error handling ([`7c045d7`](https://github.com/Byron/google-apis-rs/commit/7c045d7a259396bf1c4ce2b85415d1d3f7b858fa))
    - Remove breaking changes, add TODOs ([`cac4666`](https://github.com/Byron/google-apis-rs/commit/cac4666204691332812b8f22ddc82782ad662e70))
    - Use generic Authy trait in place of yup_oauth2 ([`7078038`](https://github.com/Byron/google-apis-rs/commit/70780388579b0362c09b76783f17a44fecf64f84))
    - Merge pull request #361 from Byron/dependabot/pip/mako-1.2.2 ([`f279d82`](https://github.com/Byron/google-apis-rs/commit/f279d821e1367d7b8026bed4a43c5dac44a63e99))
    - Bump mako from 1.1.4 to 1.2.2 ([`d87ab9d`](https://github.com/Byron/google-apis-rs/commit/d87ab9df2315c4bd6b0a9a06b28bc667f15be13c))
    - more publish markers ([`0998b28`](https://github.com/Byron/google-apis-rs/commit/0998b2810e91ab7b76847c859f339ed14e57d014))
    - Release google-youtubereporting1 v4.0.1+20220305 ([`be6cdf5`](https://github.com/Byron/google-apis-rs/commit/be6cdf5e7ccaaea1c7762a1b7b558e3d310b54d8))
    - Release google-youtube3 v4.0.1+20220303 ([`e9258a8`](https://github.com/Byron/google-apis-rs/commit/e9258a899cfe9ce7ea5eb13a34a6d0fd2d1fba46))
    - Release google-workflows1 v4.0.1+20220223 ([`78a9207`](https://github.com/Byron/google-apis-rs/commit/78a9207a5b76d22b80494c335917c7e06bc28b75))
    - Release google-workflowexecutions1 v4.0.1+20220222 ([`b25d079`](https://github.com/Byron/google-apis-rs/commit/b25d0796049e5f35005494945ad1bbb468ef79dd))
    - Release google-webrisk1 v4.0.1+20220226 ([`22df78b`](https://github.com/Byron/google-apis-rs/commit/22df78b3912e05e14610592e6d023969ab8109c8))
    - Release google-webmasters3 v4.0.1+20190428 ([`ef4e1b6`](https://github.com/Byron/google-apis-rs/commit/ef4e1b67ab80ca445f2dc6a6cc03eea0f3f142b3))
    - Release google-webfonts1 v4.0.1+20220215 ([`0cc0f01`](https://github.com/Byron/google-apis-rs/commit/0cc0f017bf9f35f2f326b15c9a05d35246afac77))
    - Release google-vmmigration1 v4.0.1+20220225 ([`228d4cb`](https://github.com/Byron/google-apis-rs/commit/228d4cbf9d0312a2d5f0ba4c558ac7a73f6722d9))
    - Release google-videointelligence1_beta1 v4.0.1+20171122 ([`a99d6d7`](https://github.com/Byron/google-apis-rs/commit/a99d6d7bd7097b3936288d89d349a7eeb5200335))
    - Release google-videointelligence1 v4.0.1+20220225 ([`638d6bc`](https://github.com/Byron/google-apis-rs/commit/638d6bcda7b5fabe693c5cc21e3e7fcabb81cd80))
    - Release google-versionhistory1 v4.0.1+20220307 ([`306b3c9`](https://github.com/Byron/google-apis-rs/commit/306b3c9b8e8be72b5b1f48aebf46c4efe2207a53))
    - Release google-verifiedaccess1 v4.0.1+20220215 ([`78301df`](https://github.com/Byron/google-apis-rs/commit/78301dfc3afe2b1cd0a2e87a8145c2e3a4456b2e))
    - Release google-vectortile1 v4.0.1+20210331 ([`be1aec8`](https://github.com/Byron/google-apis-rs/commit/be1aec8f17a37c8ac6aadab0bdbb225c2841561f))
    - Release google-vault1 v4.0.1+20220222 ([`3c6b568`](https://github.com/Byron/google-apis-rs/commit/3c6b5682c01e4575c3eef667ba9483e3aa0683fd))
    - Release google-urlshortener1 v4.0.1+20150519 ([`d0bee95`](https://github.com/Byron/google-apis-rs/commit/d0bee955444333345fc3c0f2c9f2bb88eb8d4c6d))
    - Release google-translate3 v4.0.1+20220121 ([`0af6438`](https://github.com/Byron/google-apis-rs/commit/0af64384e95758ddf96d6e4c3ecd516afcb4698f))
    - Release google-translate2 v4.0.1+20170525 ([`17de7ca`](https://github.com/Byron/google-apis-rs/commit/17de7ca335c8f0d86b20aaf742f3d1b3beff8c41))
    - Release google-transcoder1 v4.0.1+20220201 ([`9df6b21`](https://github.com/Byron/google-apis-rs/commit/9df6b21d079f543dfb6350e0465e261c245a6072))
    - Release google-transcoder1_beta1 v4.0.1+20210323 ([`84b0e87`](https://github.com/Byron/google-apis-rs/commit/84b0e873dce0e59b712966d57dace83ca2bc605a))
    - Release google-tpu1_alpha1 v4.0.1+20220301 ([`0bc4f9b`](https://github.com/Byron/google-apis-rs/commit/0bc4f9bc15602f187fca6c6c1eacaa6f72f63189))
    - Release google-tpu1 v4.0.1+20220301 ([`f42b771`](https://github.com/Byron/google-apis-rs/commit/f42b7719849a20602946a1e4aae67b8cd220b99a))
    - Release google-texttospeech1 v4.0.1+20220228 ([`b083998`](https://github.com/Byron/google-apis-rs/commit/b0839988bec06576e0e664fa4cae73b825d58c1e))
    - Release google-testing1 v4.0.1+20220301 ([`09a1aa5`](https://github.com/Byron/google-apis-rs/commit/09a1aa5b02fbb4e17ab96db995188c9a236eb6eb))
    - Release google-tasks1 v4.0.1+20220305 ([`8f7b83e`](https://github.com/Byron/google-apis-rs/commit/8f7b83eeb69178e34bebc22df24922254f32a5e3))
    - Release google-taskqueue1_beta2 v4.0.1+20160428 ([`a787681`](https://github.com/Byron/google-apis-rs/commit/a787681788f0f4b04d20307ed7b65f347a60c207))
    - Release google-tagmanager2 v4.0.1+20220301 ([`83e8e41`](https://github.com/Byron/google-apis-rs/commit/83e8e41d1ebe983935595045d79aeed6b645c8b1))
    - Release google-tagmanager1 v4.0.1+20220301 ([`8ac7479`](https://github.com/Byron/google-apis-rs/commit/8ac7479819628c84981b2d58945b05ead4e97cdc))
    - Release google-surveys2 v4.0.1+20180508 ([`1f52f48`](https://github.com/Byron/google-apis-rs/commit/1f52f48908ba6e8ba0d58a3390015a2c8ad1f94c))
    - Release google-sts1 v4.0.1+20220227 ([`ea7885d`](https://github.com/Byron/google-apis-rs/commit/ea7885d8508014f4438d7f3e5c30278db77bd000))
    - Release google-storagetransfer1 v4.0.1+20220223 ([`f9dd0be`](https://github.com/Byron/google-apis-rs/commit/f9dd0be0fc91e88d611efef8fa14cec4115ba45c))
    - Release google-storage1 v4.0.1+20220228 ([`bdce0ae`](https://github.com/Byron/google-apis-rs/commit/bdce0ae01e2dbcfb11593b65318ef91f111eaff1))
    - Release google-sqladmin1 v4.0.1+20220226 ([`97aa9cf`](https://github.com/Byron/google-apis-rs/commit/97aa9cf959d3ea2260e7402a4bcdda7f5b9d58dc))
    - Release google-sqladmin1_beta4 v4.0.1+20220226 ([`4921b3a`](https://github.com/Byron/google-apis-rs/commit/4921b3a3fbdab06badddab50888259e4fed8e39f))
    - Release google-sql1_beta4 v4.0.1+20200331 ([`f0a2658`](https://github.com/Byron/google-apis-rs/commit/f0a26582d50dd70edc29e78185f2d0a6c05d2b22))
    - Release google-speech1_beta1 v4.0.1+20181005 ([`d1c304c`](https://github.com/Byron/google-apis-rs/commit/d1c304c54dff2aa6a2c16b33be67cfb8755f5226))
    - Release google-speech1 v4.0.1+20220221 ([`c4b5dd7`](https://github.com/Byron/google-apis-rs/commit/c4b5dd78646330965e0afe42c09bf0350f490f84))
    - Release google-spectrum1_explorer v4.0.1+20170306 ([`9936c8d`](https://github.com/Byron/google-apis-rs/commit/9936c8de81f8d2a2467628ddcdd81bdb10caf048))
    - Release google-sourcerepo1 v4.0.1+20220217 ([`e925cc0`](https://github.com/Byron/google-apis-rs/commit/e925cc0191f5e19ecfb953551bce992b68080019))
    - Release google-smartdevicemanagement1 v4.0.1+20220302 ([`d053a9b`](https://github.com/Byron/google-apis-rs/commit/d053a9bbe69bc44a9d549c5a648df7d2e184161f))
    - Release google-siteverification1 v4.0.1+20191119 ([`3ca4aed`](https://github.com/Byron/google-apis-rs/commit/3ca4aedd93cdba77b04eafee589d3dc706115048))
    - Release google-serviceregistryalpha v4.0.1+20160401 ([`3a1e23c`](https://github.com/Byron/google-apis-rs/commit/3a1e23c76458f259cfab8297c8affa1ac2afa683))
    - Release google-servicedirectory1_beta1 v4.0.1+20220224 ([`c0bddc9`](https://github.com/Byron/google-apis-rs/commit/c0bddc90bb9a2c0dd6ecb8d8df625dc9a886d258))
    - Release google-servicedirectory1 v4.0.1+20220224 ([`d61a0b2`](https://github.com/Byron/google-apis-rs/commit/d61a0b2228ac8ee809bdde0ad6c2d26b34ed56e0))
    - Release google-servicecontrol2 v4.0.1+20220227 ([`c86b0dd`](https://github.com/Byron/google-apis-rs/commit/c86b0dd9ac296fed44cb4733ec6bc0310e913f29))
    - Release google-servicecontrol1 v4.0.1+20220227 ([`9d1fe7b`](https://github.com/Byron/google-apis-rs/commit/9d1fe7b0010c963e0a84fb1d6cd27de5edba3c01))
    - Release google-servicebroker1 v4.0.1+20190624 ([`ee243cc`](https://github.com/Byron/google-apis-rs/commit/ee243cc75b8845f6fd04ae198a2ec08c832775c4))
    - Release google-securitycenter1 v4.0.1+20220224 ([`a4774f4`](https://github.com/Byron/google-apis-rs/commit/a4774f44eed2ffe371364d7c20c3c42eb87a622b))
    - Release google-secretmanager1_beta1 v4.0.1+20220226 ([`bec45a8`](https://github.com/Byron/google-apis-rs/commit/bec45a87b5d26e8aaa767b92f551a689a58dad9c))
    - Release google-secretmanager1 v4.0.1+20220226 ([`208ff79`](https://github.com/Byron/google-apis-rs/commit/208ff7982baeaacd64c4d15339cd59c092b65a73))
    - Release google-searchconsole1 v4.0.1+20220305 ([`e599ab8`](https://github.com/Byron/google-apis-rs/commit/e599ab8140cf0f6766c2159ebdd834d53f021f6e))
    - Release google-sasportal1_alpha1 v4.0.1+20220301 ([`36aa8ae`](https://github.com/Byron/google-apis-rs/commit/36aa8ae71fcbfadbd0fb39dce73855ff11e4a046))
    - Release google-safebrowsing4 v4.0.1+20220305 ([`ece59a9`](https://github.com/Byron/google-apis-rs/commit/ece59a976a876022535ced88856b31b9b0ea4b3a))
    - Release google-runtimeconfig1_beta1 v4.0.1+20220228 ([`d46aaf7`](https://github.com/Byron/google-apis-rs/commit/d46aaf78c9f2395672c97db476fb9df7ccf3f164))
    - Release google-runtimeconfig1 v4.0.1+20220228 ([`2081142`](https://github.com/Byron/google-apis-rs/commit/2081142087756be8e109635ff290f05ed9dc9a0d))
    - Release google-run2 v4.0.1+20220225 ([`ad6314a`](https://github.com/Byron/google-apis-rs/commit/ad6314aceadd3900b829501c971a2bf0c4197dff))
    - Release google-run1 v4.0.1+20220225 ([`e79bf2f`](https://github.com/Byron/google-apis-rs/commit/e79bf2ff6b25578c071e8a536dee5c0c9e3e0006))
    - Release google-retail2 v4.0.1+20220224 ([`657bf99`](https://github.com/Byron/google-apis-rs/commit/657bf991e64a0af07613b58cb631b6722f3ec1db))
    - Release google-resourceviews1_beta2 v4.0.1+20160512 ([`1ce524d`](https://github.com/Byron/google-apis-rs/commit/1ce524d1a094f8d356ce1713fd4fe6090de4a7d0))
    - Release google-resourcesettings1 v4.0.1+20220305 ([`2aaec7f`](https://github.com/Byron/google-apis-rs/commit/2aaec7fc357005ead24cf2c17edd5a921eda26a8))
    - Release google-reseller1_sandbox v4.0.1+20160329 ([`7c804f3`](https://github.com/Byron/google-apis-rs/commit/7c804f3e3ec4f964d423749003d76d1c4e932aee))
    - Release google-replicapoolupdater1_beta1 v4.0.1+20161003 ([`844a441`](https://github.com/Byron/google-apis-rs/commit/844a4410bd6c2efea32bf9cc3c6056ea88b80ad4))
    - Release google-replicapool1_beta2 v4.0.1+20160512 ([`0072be5`](https://github.com/Byron/google-apis-rs/commit/0072be5feb725a5deb0161fff68a16808c6cf29f))
    - Release google-remotebuildexecution2 v4.0.1+20210329 ([`6de5dc5`](https://github.com/Byron/google-apis-rs/commit/6de5dc51e287dc70ffa834b5e2759e9f9bca7df5))
    - Release google-redis1 v4.0.1+20220301 ([`327b69c`](https://github.com/Byron/google-apis-rs/commit/327b69c3ebfed6cf5005065b5172a702b8b99b96))
    - Release google-recommender1_beta1 v4.0.1+20220228 ([`5ae9aff`](https://github.com/Byron/google-apis-rs/commit/5ae9affb548f47188f887afeded4d48c931454b9))
    - Release google-recommender1 v4.0.1+20220228 ([`a1340b4`](https://github.com/Byron/google-apis-rs/commit/a1340b45e1f0db4ce0f4d47c1b65c730f9be6955))
    - Release google-recommendationengine1_beta1 v4.0.1+20220224 ([`ba7311d`](https://github.com/Byron/google-apis-rs/commit/ba7311d00841aed395c0398f4163d5cd44a768e5))
    - Release google-recaptchaenterprise1 v4.0.1+20220226 ([`16f63ae`](https://github.com/Byron/google-apis-rs/commit/16f63aeff2215c09a79f27c7f68d6a72e935c06b))
    - Release google-realtimebidding1 v4.0.1+20220307 ([`156b9bf`](https://github.com/Byron/google-apis-rs/commit/156b9bf77d6dfadff4696e0bb9448b94c919cddc))
    - Release google-qpxexpress1 v4.0.1+20160708 ([`75125d9`](https://github.com/Byron/google-apis-rs/commit/75125d9ed1eefabbe7e2d543e32921447226bb4b))
    - Release google-pubsublite1 v4.0.1+20220301 ([`07ab8f7`](https://github.com/Byron/google-apis-rs/commit/07ab8f704b5f100186ccf84aec47cc850d5de945))
    - Release google-pubsub1_beta2 v4.0.1+20220221 ([`2ab214f`](https://github.com/Byron/google-apis-rs/commit/2ab214ff90cc72d9ad25980581a5dae56edf43d4))
    - Release google-pubsub1 v4.0.1+20220221 ([`20fa835`](https://github.com/Byron/google-apis-rs/commit/20fa835f0139b73cbcd04985e93905a1c2fc449e))
    - Release google-proximitybeacon1_beta1 v4.0.1+20200127 ([`84d1a18`](https://github.com/Byron/google-apis-rs/commit/84d1a1839c009af58d938666fcaae861a407b57d))
    - Release google-prod_tt_sasportal1_alpha1 v4.0.1+20220303 ([`26499c1`](https://github.com/Byron/google-apis-rs/commit/26499c137206958832e01c90ce2b9e08f357c3e2))
    - Release google-privateca1 v4.0.1+20220209 ([`9c39486`](https://github.com/Byron/google-apis-rs/commit/9c394867e20b40e8eb6cf541e0114ffc85ed7c31))
    - Release google-privateca1_beta1 v4.0.1+20220209 ([`ef17716`](https://github.com/Byron/google-apis-rs/commit/ef177167369bf68041cc57d98543cf793278cf64))
    - Release google-prediction1d6 v4.0.1+20160511 ([`4b55534`](https://github.com/Byron/google-apis-rs/commit/4b5553464edb2ef02590d36127873291fe115fe0))
    - Release google-policytroubleshooter1 v4.0.1+20220227 ([`61dd320`](https://github.com/Byron/google-apis-rs/commit/61dd320848d687a9121f551a9b4d6b3b82e65c26))
    - Release google-policysimulator1 v4.0.1+20220227 ([`7881bc5`](https://github.com/Byron/google-apis-rs/commit/7881bc564f994854f90ea83c9553c155291f60b3))
    - Release google-policyanalyzer1 v4.0.1+20220227 ([`b3f7459`](https://github.com/Byron/google-apis-rs/commit/b3f7459020073b63321a3e7c8fe10c3bd847319c))
    - Release google-plusdomains1 v4.0.1+20190616 ([`8b3683d`](https://github.com/Byron/google-apis-rs/commit/8b3683d6f3870433b7f286c06a4a80e577047517))
    - Release google-plus1 v4.0.1+20190616 ([`624c298`](https://github.com/Byron/google-apis-rs/commit/624c29814d25321e9969a2fddb777b7aad7d0373))
    - Release google-playmoviespartner1 v4.0.1+20170919 ([`a0d6cab`](https://github.com/Byron/google-apis-rs/commit/a0d6cab29734100529e25c7bec089c011a86ea14))
    - Release google-playintegrity1 v4.0.1+20220305 ([`3f133ba`](https://github.com/Byron/google-apis-rs/commit/3f133ba480d90666ae5d35fdfb708efe19bf25c5))
    - Release google-playcustomapp1 v4.0.1+20220305 ([`1ea72f1`](https://github.com/Byron/google-apis-rs/commit/1ea72f1c495f977cacfab7940fc5bff061b9a4a1))
    - Release google-playablelocations3 v4.0.1+20200707 ([`34e7e38`](https://github.com/Byron/google-apis-rs/commit/34e7e38414536fb3ee21f30073844d4b35a01367))
    - Release google-photoslibrary1 v4.0.1+20220303 ([`ba6e204`](https://github.com/Byron/google-apis-rs/commit/ba6e20417b1952fd8f27a43416919f66cf472053))
    - Release google-people1 v4.0.1+20220303 ([`1b8b22f`](https://github.com/Byron/google-apis-rs/commit/1b8b22f652aeb93b00f72fde1ab879f2fb942a99))
    - Release google-paymentsresellersubscription1 v4.0.1+20220307 ([`604506b`](https://github.com/Byron/google-apis-rs/commit/604506be0f04b9aa06f65cbbce7bc4b2a688c203))
    - Release google-partners2 v4.0.1+20180925 ([`b76f28c`](https://github.com/Byron/google-apis-rs/commit/b76f28c71fd0e2c322ca4eef3b15cf8d1a3e11a9))
    - Release google-pagespeedonline5 v4.0.1+20220302 ([`2bd487f`](https://github.com/Byron/google-apis-rs/commit/2bd487f3232a888dcb43d4a77f038c5f5716e158))
    - Release google-pagespeedonline4 v4.0.1+20191206 ([`6dcfd6c`](https://github.com/Byron/google-apis-rs/commit/6dcfd6cf655bc1dc14a8bf7ec39b7ead8dda0ecc))
    - Release google-pagespeedonline2 v4.0.1+20191206 ([`d3308d3`](https://github.com/Byron/google-apis-rs/commit/d3308d379141811513dc3808f04a6042bccac338))
    - Release google-oslogin1_beta v4.0.1+20220228 ([`e60a246`](https://github.com/Byron/google-apis-rs/commit/e60a246e47951fdf847244e8effbf61a12bcf7a2))
    - Release google-oslogin1 v4.0.1+20220228 ([`9ec9302`](https://github.com/Byron/google-apis-rs/commit/9ec9302b563096f99890fd6441852104b5033aa0))
    - Release google-orgpolicy2 v4.0.1+20220305 ([`1158f06`](https://github.com/Byron/google-apis-rs/commit/1158f0695b438753524c0e319974d9ef562d934f))
    - Release google-ondemandscanning1 v4.0.1+20220228 ([`32b9880`](https://github.com/Byron/google-apis-rs/commit/32b9880b201303e9e84c5aa74a77dd67efb94de2))
    - Release google-notebooks1 v4.0.1+20220224 ([`6f834a8`](https://github.com/Byron/google-apis-rs/commit/6f834a8cb8d1d58b7f5ff7538b7ab4e20b9dbe68))
    - Release google-networkservices1 v4.0.1+20220222 ([`49e4286`](https://github.com/Byron/google-apis-rs/commit/49e4286234a387761a45e84f2e4abe70f315992a))
    - Release google-networksecurity1 v4.0.1+20220223 ([`b299391`](https://github.com/Byron/google-apis-rs/commit/b299391dd31ae48998a7c9b331fb705a3c6541cf))
    - Release google-networkmanagement1 v4.0.1+20220223 ([`338067a`](https://github.com/Byron/google-apis-rs/commit/338067a51c5e78bb87ef3d29361ae679a95162b2))
    - Release google-networkconnectivity1 v4.0.1+20220210 ([`6e2ec2e`](https://github.com/Byron/google-apis-rs/commit/6e2ec2effb5a8cee047f926476b77b64fad8b342))
    - Release google-networkconnectivity1_alpha1 v4.0.1+20220210 ([`d65844e`](https://github.com/Byron/google-apis-rs/commit/d65844ec97df7adcdd162320790a345e5d8f45a6))
    - Release google-mybusinessverifications1 v4.0.1+20220305 ([`fe66a9d`](https://github.com/Byron/google-apis-rs/commit/fe66a9de01b341a3323c2a1b7d442fa4573ed68a))
    - Release google-mybusinessplaceactions1 v4.0.1+20220305 ([`2cbd391`](https://github.com/Byron/google-apis-rs/commit/2cbd3912c9398b2bf7c19fbfb996d53887129cfc))
    - Release google-mybusinessnotifications1 v4.0.1+20220305 ([`d762625`](https://github.com/Byron/google-apis-rs/commit/d762625e8e39b667615ce9805a76e00db2ffe6c8))
    - Release google-mybusinesslodging1 v4.0.1+20220305 ([`7b7f158`](https://github.com/Byron/google-apis-rs/commit/7b7f158c06895d12e09fa6a53ada24dacf1a84cd))
    - Release google-mybusinessbusinessinformation1 v4.0.1+20220305 ([`e88682d`](https://github.com/Byron/google-apis-rs/commit/e88682d1261e87707c1f4704930049d9cda030c0))
    - Release google-mybusinessbusinesscalls1 v4.0.1+20220305 ([`dce3f62`](https://github.com/Byron/google-apis-rs/commit/dce3f624e7f0a4c007b9c18a4da3ccc4f9ca644e))
    - Release google-mybusinessaccountmanagement1 v4.0.1+20220305 ([`ffa7ca6`](https://github.com/Byron/google-apis-rs/commit/ffa7ca638205545ff72b80da661f8457e1b1c8d8))
    - Release google-mybusiness4 v4.0.1+0 ([`6b69504`](https://github.com/Byron/google-apis-rs/commit/6b69504adc449f5754803ee81d31724dabdca87a))
    - Release google-monitoring3 v4.0.1+20220218 ([`8feb1a0`](https://github.com/Byron/google-apis-rs/commit/8feb1a0b25afe2eb82d6659966a8bea71f08b8ad))
    - Release google-ml1 v4.0.1+20220212 ([`1e2d670`](https://github.com/Byron/google-apis-rs/commit/1e2d670309f6aa029c38564e8a665d3e5be24e9b))
    - Release google-mirror1 v4.0.1+20190424 ([`3e74722`](https://github.com/Byron/google-apis-rs/commit/3e7472284922f49a2352aa89a884a66846b0a384))
    - Release google-metastore1_beta v4.0.1+20220222 ([`936c5cd`](https://github.com/Byron/google-apis-rs/commit/936c5cd2ee69b363d40e6d49df40aca676d91876))
    - Release google-memcache1_beta2 v4.0.1+20220224 ([`b3bfa1e`](https://github.com/Byron/google-apis-rs/commit/b3bfa1edcd6cb4f81de60cb973e3a75c8ed2f1a1))
    - Release google-memcache1 v4.0.1+20220224 ([`ee007f6`](https://github.com/Byron/google-apis-rs/commit/ee007f6b7c786e4964508be03225984c21cc4891))
    - Release google-manufacturers1 v4.0.1+20220303 ([`18baa15`](https://github.com/Byron/google-apis-rs/commit/18baa151fa78900201890e78b0e2ea0643eed8ab))
    - Release google-manager1_beta2 v4.0.1+20140915 ([`ba3b612`](https://github.com/Byron/google-apis-rs/commit/ba3b612df7aa7f09a81be50d964b1e0228190248))
    - Release google-managedidentities1 v4.0.1+20220216 ([`edddc90`](https://github.com/Byron/google-apis-rs/commit/edddc9010beb0d2154044026b0c1349a49a36cd1))
    - Release google-logging2_beta1 v4.0.1+20190325 ([`069ca3f`](https://github.com/Byron/google-apis-rs/commit/069ca3f8fe439e185544840d018ba710d4b783a9))
    - Release google-logging2 v4.0.1+20220225 ([`bd7bd36`](https://github.com/Byron/google-apis-rs/commit/bd7bd3653bf3d366dd2741521c84d39d6530c579))
    - Release google-localservices1 v4.0.1+20220305 ([`3736dfe`](https://github.com/Byron/google-apis-rs/commit/3736dfe0b2eae971bb720ca923b89b325176a8dd))
    - Release google-lifesciences2_beta v4.0.1+20220211 ([`d424f41`](https://github.com/Byron/google-apis-rs/commit/d424f41c2012b38b29ab2c6fb9f75ef3d1877050))
    - Release google-licensing1 v4.0.1+20220305 ([`e7c00b0`](https://github.com/Byron/google-apis-rs/commit/e7c00b01663560fd582c450a1104a4d693f4ca31))
    - Release google-libraryagent1 v4.0.1+20220305 ([`e631e60`](https://github.com/Byron/google-apis-rs/commit/e631e60f3daaf98e83a6985609709cc5a42d441a))
    - Release google-language1_beta1 v4.0.1+20220218 ([`d77b434`](https://github.com/Byron/google-apis-rs/commit/d77b434da4af27bb87617ba2ad3f7372a5f5b4ab))
    - Release google-language1 v4.0.1+20220218 ([`7e72561`](https://github.com/Byron/google-apis-rs/commit/7e7256145002f9fae935810a0a09a098896ac500))
    - Release google-keep1 v4.0.1+20220301 ([`bff4b3f`](https://github.com/Byron/google-apis-rs/commit/bff4b3ff16ef1b9de539f99e707e4581f96471fb))
    - Release google-jobs4 v4.0.1+20220211 ([`2c1f6dd`](https://github.com/Byron/google-apis-rs/commit/2c1f6dd46472c4f685227a43a8c0afaca4107783))
    - Release google-jobs3 v4.0.1+20220211 ([`429fc0a`](https://github.com/Byron/google-apis-rs/commit/429fc0afce079e505d83391154360ec900b5aeba))
    - Release google-indexing3 v4.0.1+20220126 ([`5fa9cd2`](https://github.com/Byron/google-apis-rs/commit/5fa9cd231a57eb9c833ddfa7dce10ed49195e6d1))
    - Release google-ids1 v4.0.1+20220221 ([`d9b3c6a`](https://github.com/Byron/google-apis-rs/commit/d9b3c6ab12ff232807d4b6d4262f457d215330ec))
    - Release google-identitytoolkit3 v4.0.1+20180723 ([`7d130d3`](https://github.com/Byron/google-apis-rs/commit/7d130d379b15c059e39edff224b76938d9e1031a))
    - Release google-ideahub1_beta v4.0.1+20220305 ([`894822a`](https://github.com/Byron/google-apis-rs/commit/894822a77ae9ea7ab1fc38c291c7f2c723313f93))
    - Release google-iap1_beta1 v4.0.1+20220225 ([`1d59776`](https://github.com/Byron/google-apis-rs/commit/1d59776842b854c7d65681e71b5a4b55024c2a04))
    - Release google-iap1 v4.0.1+20220225 ([`a14150d`](https://github.com/Byron/google-apis-rs/commit/a14150d8eced7a89f669488d92b70b3e8317e7e2))
    - Release google-iamcredentials1 v4.0.1+20220225 ([`465c2d8`](https://github.com/Byron/google-apis-rs/commit/465c2d87f64e5c6dd4713335a2bbefa9c9ee286b))
    - Release google-iam1 v4.0.1+20220224 ([`e2e9b4d`](https://github.com/Byron/google-apis-rs/commit/e2e9b4d1fa201fe5fe209456b2ab9277b506eb6f))
    - Release google-healthcare1_beta1 v4.0.1+20220223 ([`1e735ca`](https://github.com/Byron/google-apis-rs/commit/1e735cae28691f6a1639ff3de6fe003c60ea2208))
    - Release google-healthcare1 v4.0.1+20220223 ([`b30b198`](https://github.com/Byron/google-apis-rs/commit/b30b198cd1e3ab545a2369a49d8e8189031a7dd6))
    - Release google-groupssettings1 v4.0.1+20220224 ([`c8d24b3`](https://github.com/Byron/google-apis-rs/commit/c8d24b3861a7fa8570d17e481d78d9c7c3ea1cf8))
    - Release google-groupsmigration1 v4.0.1+20220226 ([`f42de03`](https://github.com/Byron/google-apis-rs/commit/f42de032f5b144e694622048f84997c2b9b05187))
    - Release google-gmailpostmastertools1_beta1 v4.0.1+20220305 ([`0426db8`](https://github.com/Byron/google-apis-rs/commit/0426db8e70e07f232af45d0358ad300afe481de0))
    - Release google-gmailpostmastertools1 v4.0.1+20220305 ([`bdbd544`](https://github.com/Byron/google-apis-rs/commit/bdbd54473d70a32a2c9bc19a6bb21de6aa1035d1))
    - Release google-gmail1 v4.0.1+20220228 ([`a504177`](https://github.com/Byron/google-apis-rs/commit/a504177984446c85839e3961cc0a3ea74b39b6f7))
    - Release google-gkehub1 v4.0.1+20220211 ([`4105689`](https://github.com/Byron/google-apis-rs/commit/4105689f059f4fb8a824744781e5aff9904897e1))
    - Release google-genomics1 v4.0.1+20210324 ([`3f77be1`](https://github.com/Byron/google-apis-rs/commit/3f77be1fabf215a66b1c797b08ce294c25a5c8ae))
    - Release google-gan1_beta1 v4.0.1+20130205 ([`dfbb0a0`](https://github.com/Byron/google-apis-rs/commit/dfbb0a0a5cb8c9464d8b5b1da2a21610c3e08352))
    - Release google-gamesmanagement1_management v4.0.1+20220217 ([`c0b7888`](https://github.com/Byron/google-apis-rs/commit/c0b78881c79018b00ad7d7575d18212fccc4757f))
    - Release google-gameservices1 v4.0.1+20220223 ([`f4ff6d8`](https://github.com/Byron/google-apis-rs/commit/f4ff6d873d0dffc7b8552ac9b71e62b6d9428e90))
    - Release google-gamesconfiguration1_configuration v4.0.1+20220217 ([`c344d00`](https://github.com/Byron/google-apis-rs/commit/c344d004ac6ac2f732455944af0ad878a5bceef3))
    - Release google-games1 v4.0.1+20220217 ([`712a76c`](https://github.com/Byron/google-apis-rs/commit/712a76c855ee35e2b9b961a4c55778a700f313b5))
    - Release google-fusiontables2 v4.0.1+20171117 ([`933eee4`](https://github.com/Byron/google-apis-rs/commit/933eee44920a328d4a99e15e1f5dd7ce20a45feb))
    - Release google-fitness1 v4.0.1+20220302 ([`11ffeae`](https://github.com/Byron/google-apis-rs/commit/11ffeae143f45edeeb4aeb177368356a9e290fda))
    - Release google-firestore1_beta1 v4.0.1+20220221 ([`b45c23f`](https://github.com/Byron/google-apis-rs/commit/b45c23f92693405e95954bab975297df3d500335))
    - Release google-firestore1 v4.0.1+20220221 ([`551c96a`](https://github.com/Byron/google-apis-rs/commit/551c96a1c199276007e5b349f6dfeb97a089837d))
    - Release google-firebasestorage1_beta v4.0.1+20220218 ([`d6d735d`](https://github.com/Byron/google-apis-rs/commit/d6d735dd17f342e90c30e757ec0ee822b78a22e2))
    - Release google-firebaseremoteconfig1 v4.0.1+20171129 ([`abaa926`](https://github.com/Byron/google-apis-rs/commit/abaa92657f5779f08b0f0f8a075c66226b79e292))
    - Release google-firebaseml1 v4.0.1+20220302 ([`7015887`](https://github.com/Byron/google-apis-rs/commit/70158870dad5a24e665757fb13b4fc4bec035442))
    - Release google-firebasehosting1_beta1 v4.0.1+20220212 ([`fe2985d`](https://github.com/Byron/google-apis-rs/commit/fe2985d31ff50a362c9f18bd99b8773e7ffa137c))
    - Release google-firebasehosting1 v4.0.1+20220212 ([`2c5396d`](https://github.com/Byron/google-apis-rs/commit/2c5396d5700ef83e93df39a820085201ed1e4970))
    - Release google-firebasedynamiclinks1 v4.0.1+20220228 ([`a381823`](https://github.com/Byron/google-apis-rs/commit/a381823bf9eeede132baa2c6f3f876b78003939a))
    - Release google-firebasedatabase1_beta v4.0.1+20220304 ([`9335c95`](https://github.com/Byron/google-apis-rs/commit/9335c950295a12725fbbe3773311838ff6f79a3c))
    - Release google-firebaseappcheck1_beta v4.0.1+20220225 ([`c09f95e`](https://github.com/Byron/google-apis-rs/commit/c09f95e76f5ba480d90f7e763f940c436b99210a))
    - Release google-firebase1_beta1 v4.0.1+20220304 ([`7ebe6bb`](https://github.com/Byron/google-apis-rs/commit/7ebe6bb5bb572757f30cda05d7f71ad042e1b760))
    - Release google-file1_beta1 v4.0.1+20220214 ([`bb21b5a`](https://github.com/Byron/google-apis-rs/commit/bb21b5a32788687cce25188642d45cc4187390db))
    - Release google-file1 v4.0.1+20220214 ([`07bb59b`](https://github.com/Byron/google-apis-rs/commit/07bb59b368d039af57a09b88bdf66234466204e8))
    - Release google-fcmdata1_beta1 v4.0.1+20220305 ([`bbaa2f7`](https://github.com/Byron/google-apis-rs/commit/bbaa2f7896316014179da56d9f4a8eaa749bab8b))
    - Release google-fcm1 v4.0.1+20220228 ([`bada56e`](https://github.com/Byron/google-apis-rs/commit/bada56e80709d6844c63eb4c198cf6a2b52d766e))
    - Release google-factchecktools1_alpha1 v4.0.1+20220305 ([`363f156`](https://github.com/Byron/google-apis-rs/commit/363f1569a89b985065d0251591b24426f25de780))
    - Release google-eventarc1 v4.0.1+20220301 ([`8d8ab5a`](https://github.com/Byron/google-apis-rs/commit/8d8ab5a0d4769903d51fcf42269486ddfc09b727))
    - Release google-essentialcontacts1 v4.0.1+20220227 ([`aa56c4b`](https://github.com/Byron/google-apis-rs/commit/aa56c4b34fb4d11a818eabbcd517dc2e979786dd))
    - Release google-driveactivity2 v4.0.1+20220301 ([`04894af`](https://github.com/Byron/google-apis-rs/commit/04894af8b159cd284e3d40888e45b49c7bb3754c))
    - Release google-drive2 v4.0.1+20220225 ([`ec93daf`](https://github.com/Byron/google-apis-rs/commit/ec93daf40693c76aa6dfeb7805d1c777a9824c88))
    - Release google-doubleclicksearch2 v4.0.1+20220301 ([`2c8a8c0`](https://github.com/Byron/google-apis-rs/commit/2c8a8c09d3863fed8cc34e64a1a19a44fc29296a))
    - Release google-doubleclickbidmanager1d1 v4.0.1+20220302 ([`4a3c758`](https://github.com/Byron/google-apis-rs/commit/4a3c75884a79d9844cc11c4da628bd1e56f15787))
    - Release google-doubleclickbidmanager1 v4.0.1+20210323 ([`eb04dac`](https://github.com/Byron/google-apis-rs/commit/eb04dac9ac8170dbb35c5da908870c34d5cb701a))
    - Release google-domainsrdap1 v4.0.1+20220307 ([`0d4f651`](https://github.com/Byron/google-apis-rs/commit/0d4f651b5f3f3a42fef5c9dacf5bc2bdff75983b))
    - Release google-domains1 v4.0.1+20220128 ([`99ca0f2`](https://github.com/Byron/google-apis-rs/commit/99ca0f2e0523185e5aa1aa5c5f7daef34269c229))
    - Release google-domains1_beta1 v4.0.1+20220128 ([`149c4f6`](https://github.com/Byron/google-apis-rs/commit/149c4f686f1fd05f68003210d3137d947fa3ee77))
    - Release google-documentai1_beta2 v4.0.1+20220226 ([`7b2f075`](https://github.com/Byron/google-apis-rs/commit/7b2f07582dc1988c895e1933017f1b32e36c95e4))
    - Release google-documentai1 v4.0.1+20220226 ([`2c883e0`](https://github.com/Byron/google-apis-rs/commit/2c883e002dfc326ab806f61f1e25557be053ee08))
    - Release google-dns2 v4.0.1+20220217 ([`9bae6f5`](https://github.com/Byron/google-apis-rs/commit/9bae6f52049636c5c92b4b4c9561e4a0b6288067))
    - update docs1 to latest version ([`1738a9e`](https://github.com/Byron/google-apis-rs/commit/1738a9eeac86e9dfbb5d8fec0af52ece1945d15c))
    - updated publishing markers ([`4f8c853`](https://github.com/Byron/google-apis-rs/commit/4f8c85310a5fc7e928fd2915be78a21940caced7))
    - Release google-dns1 v4.0.1+20220217 ([`b60c170`](https://github.com/Byron/google-apis-rs/commit/b60c17056cd576b56045e52ac1fdb232c55389e8))
    - Release google-dlp2_beta1 v4.0.1+20171205 ([`750ff37`](https://github.com/Byron/google-apis-rs/commit/750ff3786049d5f4febea93c77885abccb88f07e))
    - Release google-dlp2 v4.0.1+20220227 ([`5f75a89`](https://github.com/Byron/google-apis-rs/commit/5f75a891a4bdc3c64bd5f359af443e7406acb628))
    - Release google-displayvideo1 v4.0.1+20220303 ([`b30fc12`](https://github.com/Byron/google-apis-rs/commit/b30fc1246488f057327415ba5ab5be512c621c6e))
    - Release google-discovery1 v4.0.1+20200806 ([`c1fb304`](https://github.com/Byron/google-apis-rs/commit/c1fb30427d341b5e173921275f496e46ad65ab09))
    - Release google-digitalassetlinks1 v4.0.1+20220301 ([`9fa9442`](https://github.com/Byron/google-apis-rs/commit/9fa9442af601ef2d2e6cf7d326c1f97d1725ddc2))
    - Release google-dialogflow3 v4.0.1+20220228 ([`2e11eaf`](https://github.com/Byron/google-apis-rs/commit/2e11eaf844c450a265167fc3ce6507dd699e648f))
    - Release google-dialogflow2_beta1 v4.0.1+20220228 ([`d7ae63f`](https://github.com/Byron/google-apis-rs/commit/d7ae63fc6e2901d1df20d146b2586a6864412adc))
    - Release google-dialogflow2 v4.0.1+20220228 ([`ba172a2`](https://github.com/Byron/google-apis-rs/commit/ba172a2094c275b0bd2a4d6f0ab37acac9e011ca))
    - Release google-dfareporting3d5 v4.0.1+20220104 ([`bfbaf83`](https://github.com/Byron/google-apis-rs/commit/bfbaf835311603afa02054e8bdef8815798106f1))
    - Release google-dfareporting3d4 v4.0.1+20220104 ([`7207dec`](https://github.com/Byron/google-apis-rs/commit/7207decaf8f52b2a97571a3a2438380b5b6985a8))
    - Release google-dfareporting3d3 v4.0.1+20220104 ([`636162d`](https://github.com/Byron/google-apis-rs/commit/636162df2da66259dea5023153ec38959bfe4bc6))
    - Release google-dfareporting3d2 v4.0.1+20190531 ([`e39b99e`](https://github.com/Byron/google-apis-rs/commit/e39b99eb9edc22dd5121b206f5faaeb7feeb5d8d))
    - Release google-dfareporting3 v4.0.1+20180830 ([`2d1db01`](https://github.com/Byron/google-apis-rs/commit/2d1db01f82f938dedf6686de8bfde97c6d5f2acd))
    - Release google-dfareporting2d8 v4.0.1+20180830 ([`c672f31`](https://github.com/Byron/google-apis-rs/commit/c672f31a6b3601703e968e3a02c93db236452d09))
    - Release google-deploymentmanager2_beta2 v4.0.1+20160201 ([`5552494`](https://github.com/Byron/google-apis-rs/commit/555249426c2de98ab3cb2fb56d5f6aae9f08c83e))
    - Release google-deploymentmanager2 v4.0.1+20220225 ([`135aceb`](https://github.com/Byron/google-apis-rs/commit/135aceb6932b6b99f0101880248e096aa71fe275))
    - Release google-datastream1 v4.0.1+20220207 ([`07d07ce`](https://github.com/Byron/google-apis-rs/commit/07d07ce72f54784aa0b7e3090985dc08ac49fb88))
    - Release google-datastore1_beta3 v4.0.1+20220221 ([`7ac902c`](https://github.com/Byron/google-apis-rs/commit/7ac902c822684cdb315f8ebbc764c2b792ed9c3b))
    - Release google-datastore1 v4.0.1+20220221 ([`f28b257`](https://github.com/Byron/google-apis-rs/commit/f28b25753ce9ecdaaa21f169c577a6c176af9e76))
    - Release google-dataproc1 v4.0.1+20220224 ([`23c9cc5`](https://github.com/Byron/google-apis-rs/commit/23c9cc5b557f510d24833e1c362087a40f8c1585))
    - Release google-dataplex1 v4.0.1+20220223 ([`26109c3`](https://github.com/Byron/google-apis-rs/commit/26109c300394862dc93645efaee9937b23f240ce))
    - Release google-datapipelines1 v4.0.1+20220218 ([`f7fd54a`](https://github.com/Byron/google-apis-rs/commit/f7fd54ae322cfe4a5070e69461c27fd413c771c7))
    - Release google-datamigration1 v4.0.1+20220216 ([`a12166c`](https://github.com/Byron/google-apis-rs/commit/a12166c18edb8c18966e433eca7e8524a48c7307))
    - Release google-datalabeling1_beta1 v4.0.1+20220301 ([`efb9d4b`](https://github.com/Byron/google-apis-rs/commit/efb9d4bc8eb99d2fdca93337f469263169b51fdd))
    - Release google-datafusion1_beta1 v4.0.1+20211028 ([`9e3caeb`](https://github.com/Byron/google-apis-rs/commit/9e3caeb25949ed9eecb6293171b5fd367b877497))
    - Release google-datafusion1 v4.0.1+20211028 ([`df17229`](https://github.com/Byron/google-apis-rs/commit/df172290bd310265e41a37d5e3f17b57bbf86350))
    - Release google-datacatalog1 v4.0.1+20220224 ([`2fe5dc7`](https://github.com/Byron/google-apis-rs/commit/2fe5dc7602cd19e2d9950a07c8a7495245e24916))
    - Release google-datacatalog1_beta1 v4.0.1+20220224 ([`52adbba`](https://github.com/Byron/google-apis-rs/commit/52adbbaeec4e1d30b83bba6503e5180a4a9637e2))
    - Release google-customsearch1 v4.0.1+20220305 ([`edf425c`](https://github.com/Byron/google-apis-rs/commit/edf425cdbace1d3fbf5326394f7f7e6bd468e9c3))
    - Release google-coordinate1 v4.0.1+20150811 ([`e603e9e`](https://github.com/Byron/google-apis-rs/commit/e603e9e095aee0dbf9943786d014b11d586cf102))
    - Release google-content2_sandbox v4.0.1+20181009 ([`2e81431`](https://github.com/Byron/google-apis-rs/commit/2e81431368774e22c8b6e52c7770fc13511068c7))
    - Release google-content2 v4.0.1+20220303 ([`f03716e`](https://github.com/Byron/google-apis-rs/commit/f03716eacc22a72c04577650f187b3f92f669ab5))
    - Release google-containeranalysis1 v4.0.1+20220225 ([`8b32133`](https://github.com/Byron/google-apis-rs/commit/8b32133cbc9fa444caacc4e8fbf9f4bacc46e62f))
    - Release google-containeranalysis1_beta1 v4.0.1+20220225 ([`9ea3940`](https://github.com/Byron/google-apis-rs/commit/9ea3940f21432b7cfd4d1974bd6489d99b01a89c))
    - Release google-container1 v4.0.1+20220215 ([`5383f84`](https://github.com/Byron/google-apis-rs/commit/5383f840950c436823df37cada5d08faca9874d8))
    - Release google-contactcenterinsights1 v4.0.1+20220227 ([`2802b8b`](https://github.com/Byron/google-apis-rs/commit/2802b8ba6c56ce396edc26116f9c769c5d0529d9))
    - Release google-consumersurveys2 v4.0.1+20170407 ([`594bf45`](https://github.com/Byron/google-apis-rs/commit/594bf456f72f278115f7a03528e4c83e9c0f1274))
    - Release google-connectors1 v4.0.1+20220214 ([`2b4c6ba`](https://github.com/Byron/google-apis-rs/commit/2b4c6ba2b32fda887ebd8db2dac29385e15fecb7))
    - Release google-compute1 v4.0.1+20220224 ([`0d1e07a`](https://github.com/Byron/google-apis-rs/commit/0d1e07a4c10aaa686c2f6ea0f552fa8aace39e3d))
    - Release google-composer1 v4.0.1+20220224 ([`b35aa4e`](https://github.com/Byron/google-apis-rs/commit/b35aa4ec2e68e545f2771ff759556c0b6a0b2059))
    - Release google-commentanalyzer1_alpha1 v4.0.1+20200405 ([`887a269`](https://github.com/Byron/google-apis-rs/commit/887a269ca93130752f2c7197df647960c1156a67))
    - Release google-clouduseraccountsvm_beta v4.0.1+20160316 ([`fd4e7e8`](https://github.com/Byron/google-apis-rs/commit/fd4e7e8e6efc87304aaea708907db00f214ad71d))
    - Release google-cloudtrace2 v4.0.1+20220224 ([`91e09d2`](https://github.com/Byron/google-apis-rs/commit/91e09d2486090b6e9b42a13b294a95144cb18d35))
    - Release google-cloudtrace1 v4.0.1+20220224 ([`ff10792`](https://github.com/Byron/google-apis-rs/commit/ff10792d45cf2de47681a3959fe749b257734644))
    - Release google-cloudtasks2_beta3 v4.0.1+20220212 ([`350a45b`](https://github.com/Byron/google-apis-rs/commit/350a45bc3fb8381874959a59a2fff7e568e4f436))
    - Release google-cloudtasks2_beta2 v4.0.1+20220212 ([`9e9faa7`](https://github.com/Byron/google-apis-rs/commit/9e9faa7e25e20f3237fbf88495991f8b8e23721a))
    - Release google-cloudtasks2 v4.0.1+20220212 ([`8bc7487`](https://github.com/Byron/google-apis-rs/commit/8bc7487d95297698612c0b9b11507b389a1fa48e))
    - Release google-cloudsupport2_beta v4.0.1+20220305 ([`e34eec1`](https://github.com/Byron/google-apis-rs/commit/e34eec1eca46ee52de1330dd9c1aab109bcc0013))
    - Release google-cloudshell1 v4.0.1+20220301 ([`3dd49d6`](https://github.com/Byron/google-apis-rs/commit/3dd49d64be444ea2e9953bfe5a884264a3c8d73b))
    - Release google-cloudscheduler1_beta1 v4.0.1+20220212 ([`3ab2b1e`](https://github.com/Byron/google-apis-rs/commit/3ab2b1e9590aa29cbfa35f9556b1c590266d0a9c))
    - Release google-cloudscheduler1 v4.0.1+20220212 ([`6585203`](https://github.com/Byron/google-apis-rs/commit/6585203ce67454ba28413024732dc4a83514ac16))
    - Release google-cloudresourcemanager3 v4.0.1+20220306 ([`8035da9`](https://github.com/Byron/google-apis-rs/commit/8035da90181ddfe5b83fcedbc5a3569b84594f5a))
    - Release google-cloudresourcemanager2 v4.0.1+20220306 ([`1cfdd9e`](https://github.com/Byron/google-apis-rs/commit/1cfdd9ead88824f5899338289a436669750450a4))
    - Release google-cloudresourcemanager1_beta1 v4.0.1+20220306 ([`6df382e`](https://github.com/Byron/google-apis-rs/commit/6df382e06630cb9f2641ed3cbdf6963c800518e8))
    - Release google-cloudresourcemanager1 v4.0.1+20220306 ([`5cc11a9`](https://github.com/Byron/google-apis-rs/commit/5cc11a978c1fc9b744ac3ee6d2bee574ee77079b))
    - Release google-cloudprofiler2 v4.0.1+20220228 ([`6a05f83`](https://github.com/Byron/google-apis-rs/commit/6a05f83f0d21eb0b371172229feeab64c8595f30))
    - Release google-cloudprivatecatalogproducer1_beta1 v4.0.1+20200405 ([`91f5a7a`](https://github.com/Byron/google-apis-rs/commit/91f5a7a62d7e12184606e81cc153b2381764507e))
    - Release google-cloudprivatecatalog1_beta1 v4.0.1+20200405 ([`9e1525f`](https://github.com/Byron/google-apis-rs/commit/9e1525fc826cc06bcaec8ab678d13e8246f60dd4))
    - Release google-cloudmonitoring2_beta2 v4.0.1+20170501 ([`2f038f4`](https://github.com/Byron/google-apis-rs/commit/2f038f48fab0f4ce7cc876daa7d8434b685863ad))
    - Release google-cloudlatencytest2 v4.0.1+20160309 ([`b31a43a`](https://github.com/Byron/google-apis-rs/commit/b31a43a93e1ec0486aaab802cfcf90f54eda529e))
    - Release google-cloudkms1_beta1 v4.0.1+20170515 ([`b752af3`](https://github.com/Byron/google-apis-rs/commit/b752af3ea7ad86872e81595c12c1b638579bfa54))
    - Release google-cloudkms1 v4.0.1+20220225 ([`f4f8c7d`](https://github.com/Byron/google-apis-rs/commit/f4f8c7d280ad292fa9feba91110f7a42a1eecdda))
    - Release google-cloudiot1 v4.0.1+20220131 ([`56c5226`](https://github.com/Byron/google-apis-rs/commit/56c5226fd1855fd184cd7c4e902ecaab32ea10ed))
    - Release google-cloudidentity1 v4.0.1+20220301 ([`ab6c8f6`](https://github.com/Byron/google-apis-rs/commit/ab6c8f69cc8b4baec9b95b55c7c60b696f3d28a1))
    - Release google-cloudfunctions1 v4.0.1+20220224 ([`a0765c1`](https://github.com/Byron/google-apis-rs/commit/a0765c161eaa00e418544a2b5b8e10d2b2993279))
    - Release google-clouderrorreporting1_beta1 v4.0.1+20220302 ([`7b4e8b0`](https://github.com/Byron/google-apis-rs/commit/7b4e8b07116d3db350d308165ea44400d444077d))
    - Release google-clouddeploy1 v4.0.1+20220223 ([`effe102`](https://github.com/Byron/google-apis-rs/commit/effe1028b446e1deaf4039ad0c49a8b493fb7038))
    - Release google-clouddebugger2 v4.0.1+20220225 ([`c1fcb8d`](https://github.com/Byron/google-apis-rs/commit/c1fcb8ddf612910b2e8bcca0d20c7baf06b86bd5))
    - Release google-cloudchannel1 v4.0.1+20220303 ([`d5a93bc`](https://github.com/Byron/google-apis-rs/commit/d5a93bca05f5a7b338a61498d24a1b0f061e7d6b))
    - Release google-cloudbuild1 v4.0.1+20220218 ([`be0216c`](https://github.com/Byron/google-apis-rs/commit/be0216c9145c79538337386b7097f88bb2f834e9))
    - Release google-cloudbilling1 v4.0.1+20220305 ([`478911b`](https://github.com/Byron/google-apis-rs/commit/478911b30653f6317667de2447e9a090187e85c2))
    - Release google-cloudasset1_beta1 v4.0.1+20220225 ([`1f5966f`](https://github.com/Byron/google-apis-rs/commit/1f5966f69d1e03a6218358f4a4169b102a6dc969))
    - Release google-cloudasset1 v4.0.1+20220225 ([`68c4452`](https://github.com/Byron/google-apis-rs/commit/68c4452a1e9b54cea64fcf8231e7a50ef9190e7b))
    - Release google-classroom1 v4.0.1+20220224 ([`8e0e777`](https://github.com/Byron/google-apis-rs/commit/8e0e777a21c7c47de53a0dc4e3f49f5fd3fb8b5e))
    - Release google-chromeuxreport1 v4.0.1+20220302 ([`c4c24d8`](https://github.com/Byron/google-apis-rs/commit/c4c24d8b38e3899380eed23875f22f5f952904c7))
    - Release google-chromepolicy1 v4.0.1+20220305 ([`4da7138`](https://github.com/Byron/google-apis-rs/commit/4da7138250f3e05372073b2369a8beb35b803761))
    - Release google-chromemanagement1 v4.0.1+20220305 ([`c2f0307`](https://github.com/Byron/google-apis-rs/commit/c2f030748ebffb2fffe3716ff09168cf31a98e95))
    - Release google-certificatemanager1 v4.0.1+20220214 ([`fa2409d`](https://github.com/Byron/google-apis-rs/commit/fa2409d1de8eaa543dc2cb5abc68819d7c8d0fad))
    - Release google-calendar3 v4.0.1+20220217 ([`a254518`](https://github.com/Byron/google-apis-rs/commit/a254518cf991f643fc2dcaae3b4d080fde844b44))
    - Release google-books1 v4.0.1+20220301 ([`631d99d`](https://github.com/Byron/google-apis-rs/commit/631d99d52c07bc3530b236700f06e852c37a114b))
    - Release google-blogger3 v4.0.1+20220305 ([`9dcf4b9`](https://github.com/Byron/google-apis-rs/commit/9dcf4b930469cfe2b40b9080f1ad2eb43257690a))
    - Release google-binaryauthorization1_beta1 v4.0.1+20220225 ([`ba9cd50`](https://github.com/Byron/google-apis-rs/commit/ba9cd504bd4509a0c0785327b1fe652450fbca90))
    - Release google-binaryauthorization1 v4.0.1+20220225 ([`36b7d5a`](https://github.com/Byron/google-apis-rs/commit/36b7d5a0aa9a1878e0bc95df87dcde691cab9fee))
    - Release google-billingbudgets1_beta1 v4.0.1+20220227 ([`c5ee46e`](https://github.com/Byron/google-apis-rs/commit/c5ee46e0125d6cabda42d196c6bed18fa11a0ed8))
    - Release google-billingbudgets1 v4.0.1+20220227 ([`966f9ad`](https://github.com/Byron/google-apis-rs/commit/966f9adf6b42797bbc1855a850d6d3234c523197))
    - Release google-bigtableadmin2 v4.0.1+20220222 ([`b090dad`](https://github.com/Byron/google-apis-rs/commit/b090dad301534ea4ba6c3cf124bdce4055e3662d))
    - Release google-bigqueryreservation1 v4.0.1+20220226 ([`09864d1`](https://github.com/Byron/google-apis-rs/commit/09864d14dff622f6f929b6129714d09326f5f19d))
    - Release google-bigquerydatatransfer1 v4.0.1+20220225 ([`e68dbe4`](https://github.com/Byron/google-apis-rs/commit/e68dbe42469e81c1de584bebea0db7ee2a6e1760))
    - Release google-bigqueryconnection1_beta1 v4.0.1+20220226 ([`909fcfa`](https://github.com/Byron/google-apis-rs/commit/909fcfa43077916f0a855158a5e40e825658a50f))
    - Release google-bigquery2 v4.0.1+20220222 ([`d09cc22`](https://github.com/Byron/google-apis-rs/commit/d09cc220b92821f862ed3cfcc879ddcd01b22e1b))
    - Release google-baremetalsolution2 v4.0.1+20220209 ([`79a7ada`](https://github.com/Byron/google-apis-rs/commit/79a7ada20304db07d400701adb06890b1985d453))
    - Release google-autoscaler1_beta2 v4.0.1+20150629 ([`71c9382`](https://github.com/Byron/google-apis-rs/commit/71c9382637f63dc682f931e9ea95cc0633a80542))
    - Release google-authorizedbuyersmarketplace1 v4.0.1+20220307 ([`ebb1c35`](https://github.com/Byron/google-apis-rs/commit/ebb1c35a8c7413a984bcefc789bded742f492650))
    - Release google-assuredworkloads1 v4.0.1+20220224 ([`76f4bd4`](https://github.com/Byron/google-apis-rs/commit/76f4bd4231d9c2d7832323a5de6c88409323682d))
    - Release google-artifactregistry1_beta1 v4.0.1+20220225 ([`720e332`](https://github.com/Byron/google-apis-rs/commit/720e3327abd36257fa5b967a1224855d2d9b0de0))
    - Release google-artifactregistry1 v4.0.1+20220225 ([`29f891e`](https://github.com/Byron/google-apis-rs/commit/29f891e67f1661878832bcc2e3182f9cca512bd1))
    - Release google-area120tables1_alpha1 v4.0.1+20220301 ([`56079ac`](https://github.com/Byron/google-apis-rs/commit/56079acd70acc445c552fb488b9544b22bd4d423))
    - Release google-appstate1 v4.0.1+20190627 ([`bb959c7`](https://github.com/Byron/google-apis-rs/commit/bb959c7d2dc3cc2f336ae35ec06483cdca64c143))
    - Release google-appsactivity1 v4.0.1+20200628 ([`457f085`](https://github.com/Byron/google-apis-rs/commit/457f0856b4c13748b88f30881ba75b14c4143f46))
    - Release google-appengine1_beta5 v4.0.1+20181005 ([`38ddb45`](https://github.com/Byron/google-apis-rs/commit/38ddb450743ac6a6a80b38b15b141be17fb06435))
    - Release google-appengine1_beta4 v4.0.1+20181005 ([`1fad9d3`](https://github.com/Byron/google-apis-rs/commit/1fad9d377c626ce35a017d6c747e1d5e0289234a))
    - Release google-appengine1 v4.0.1+20220226 ([`2f49f3b`](https://github.com/Byron/google-apis-rs/commit/2f49f3b2e7c6b7a93295bce2f0471e92ca83df4c))
    - Release google-apikeys2 v4.0.1+20220305 ([`efb0762`](https://github.com/Byron/google-apis-rs/commit/efb07623ffb456667850563f6234948c11fb73b8))
    - Release google-apigee1 v4.0.1+20220301 ([`4ff7f06`](https://github.com/Byron/google-apis-rs/commit/4ff7f066e26254a16dd4e3af79e5aa83211ea75b))
    - Release google-apigateway1 v4.0.1+20220223 ([`2767b2f`](https://github.com/Byron/google-apis-rs/commit/2767b2fe09b2b837b87ea7205281c339d1c46cda))
    - Release google-androidpublisher3 v4.0.1+20220307 ([`a63195e`](https://github.com/Byron/google-apis-rs/commit/a63195eb349131f325ff0412ced890c829c016cf))
    - Release google-androidpublisher2 v4.0.1+20200331 ([`eb9df71`](https://github.com/Byron/google-apis-rs/commit/eb9df71b9ab4e2af456ab2478aa56673b9f94cf4))
    - Release google-androidmanagement1 v4.0.1+20220302 ([`409cafc`](https://github.com/Byron/google-apis-rs/commit/409cafc5104ef253e778521784da0581810a5959))
    - Release google-androidenterprise1 v4.0.1+20220303 ([`c0dbcb8`](https://github.com/Byron/google-apis-rs/commit/c0dbcb864ffe47167e95f3c049e32455258cd9ac))
    - Release google-androiddeviceprovisioning1 v4.0.1+20220305 ([`1953427`](https://github.com/Byron/google-apis-rs/commit/1953427962473f764d208811cfa86bab883f7c33))
    - Release google-analyticsreporting4 v4.0.1+20220215 ([`d9d2a1b`](https://github.com/Byron/google-apis-rs/commit/d9d2a1b3b69cec4752f5a41e7729d4b9a45fb3fb))
    - Release google-analyticsdata1_beta v4.0.1+20220303 ([`e6acf0a`](https://github.com/Byron/google-apis-rs/commit/e6acf0a64e0b1057cf161fbe3c0d5e0a494348a7))
    - Release google-analyticsadmin1_alpha v4.0.1+20220307 ([`dbf1115`](https://github.com/Byron/google-apis-rs/commit/dbf1115f59b5c6a951c765115652466baa1a3507))
    - Release google-analytics3 v4.0.1+20190807 ([`101b433`](https://github.com/Byron/google-apis-rs/commit/101b433ed65377c0acd14049627954ecb72e40cb))
    - Release google-alertcenter1_beta1 v4.0.1+20220221 ([`ff5ee4a`](https://github.com/Byron/google-apis-rs/commit/ff5ee4ae51d15d831d8f15286ff7582658e659d5))
    - Release google-adsensehost4d1 v4.0.1+20200930 ([`c040795`](https://github.com/Byron/google-apis-rs/commit/c040795242a083e44461c45e51e49c639debc8c8))
    - Release google-adsense2 v4.0.1+20220304 ([`0c5190b`](https://github.com/Byron/google-apis-rs/commit/0c5190b5044bf2c1da3f73868bf98a66be0f2035))
    - Release google-adsense1d4 v4.0.1+20201002 ([`68805fb`](https://github.com/Byron/google-apis-rs/commit/68805fbd2543b78b49c7c3db755c5651c89cc6bb))
    - Release google-admob1 v4.0.1+20220303 ([`16519ba`](https://github.com/Byron/google-apis-rs/commit/16519ba82d8fa503c3d47e1059f644ee046fba46))
    - Release google-adexperiencereport1 v4.0.1+20220303 ([`6a79f58`](https://github.com/Byron/google-apis-rs/commit/6a79f5828b130282279615eaefbb0148a764f760))
    - Release google-adexchangeseller2 v4.0.1+20171101 ([`729a064`](https://github.com/Byron/google-apis-rs/commit/729a0644ca3918f918fe8b44115ff1b88b8717ba))
    - Release google-adexchangebuyer2_v2_beta1 v4.0.1+20220307 ([`fc3cf0b`](https://github.com/Byron/google-apis-rs/commit/fc3cf0b8c719ae78507bbd57ad66b2abfc6d30b0))
    - Release google-adexchangebuyer1d4 v4.0.1+20210330 ([`6e7e9dd`](https://github.com/Byron/google-apis-rs/commit/6e7e9ddd328a1476f80aa540d0338609329b76e1))
    - Release google-adexchangebuyer1d3 v4.0.1+20210330 ([`ec55c00`](https://github.com/Byron/google-apis-rs/commit/ec55c000c92e9e7a33dfc058fb965e0d057e49e0))
    - Release google-accesscontextmanager1_beta v4.0.1+20220301 ([`c608b9b`](https://github.com/Byron/google-apis-rs/commit/c608b9b8d40beff8252fc223fee33f14dd0ab5c4))
    - Release google-accesscontextmanager1 v4.0.1+20220301 ([`4017dae`](https://github.com/Byron/google-apis-rs/commit/4017daef000b56b81a9e0d4e09926493d92cb3fc))
    - Release google-accessapproval1_beta1 v4.0.1+20200708 ([`edb330c`](https://github.com/Byron/google-apis-rs/commit/edb330c7560eb011b6b2315cf4a32048351024d2))
    - Release google-accessapproval1 v4.0.1+20220225 ([`ae924ac`](https://github.com/Byron/google-apis-rs/commit/ae924ac4927d6a43f0b96da464efc81b98f1d752))
    - Release google-acceleratedmobilepageurl1 v4.0.1+20220305 ([`6eb1cd8`](https://github.com/Byron/google-apis-rs/commit/6eb1cd898a5a9ebb2ef7a6d0b645a8f009b6cb8e))
    - Release google-abusiveexperiencereport1 v4.0.1+20220303 ([`f0371b5`](https://github.com/Byron/google-apis-rs/commit/f0371b582118740dec8bc21184b35aab1f335698))
    - publish sheets 4 ([`d2022d0`](https://github.com/Byron/google-apis-rs/commit/d2022d0b65139fdc841d6c392e700cf73e05cd0f))
    - Release google-sheets4 v4.0.1+20220221 ([`8e4ec82`](https://github.com/Byron/google-apis-rs/commit/8e4ec82be7c7425924cb1cf4d3b36070b3424fc8))
    - Merge branch 'python_fixes' ([`0e011fc`](https://github.com/Byron/google-apis-rs/commit/0e011fcbe2382e40c94a5349e4f41cbd14420cd9))
    - Remove dynamic imports in mako-render ([`3483ab0`](https://github.com/Byron/google-apis-rs/commit/3483ab0fb6630f9aa9cd8d6d1fe7a6fee2a0896b))
    - generator lib: Initial tests ([`a60caa2`](https://github.com/Byron/google-apis-rs/commit/a60caa2690ec45014898d866213020c973d02cee))
    - Make new_context dict-compatible ([`8ba6acb`](https://github.com/Byron/google-apis-rs/commit/8ba6acb88bf889d41560ccc2c16f5e884af68b9c))
    - Restructure `src` dir ([`08552c4`](https://github.com/Byron/google-apis-rs/commit/08552c43644ee953f1c69b4e5c018d35d1f865f6))
    - pyright type checking: Initial setup ([`5e1c0c8`](https://github.com/Byron/google-apis-rs/commit/5e1c0c857e00e6da37257d0bc68c263008c702d0))
    - track vision framework ([`4cdef0b`](https://github.com/Byron/google-apis-rs/commit/4cdef0be06d6c0b247c68cd6edd23bff9c2d2628))
    - Release google-vision1 v4.0.1+20220225 ([`e7f565f`](https://github.com/Byron/google-apis-rs/commit/e7f565fe6ae36d9e3b706606121eb972afc0257b))
    - regenerate vision1 framework prior to release ([`8a16ffb`](https://github.com/Byron/google-apis-rs/commit/8a16ffb8cb9e058a7bca9dbaed818442031261a0))
    - Release google-drive3-cli v4.0.1+20220225 ([`37c5f85`](https://github.com/Byron/google-apis-rs/commit/37c5f85f2ed77c29cbfe7bf1dd530da551c6cde2))
    - Release google-drive3 v4.0.1+20220225 ([`9d4cb8e`](https://github.com/Byron/google-apis-rs/commit/9d4cb8e027e97f00ca547d04ba135c3a6649aafd))
    - Merge branch 'http2_timeout_panic' ([`53f5b32`](https://github.com/Byron/google-apis-rs/commit/53f5b320832ecc6429c43238b117bb6c61db468e))
    - regen drive3 with v4.0.1 for release prep ([`1582289`](https://github.com/Byron/google-apis-rs/commit/1582289b677d7fa7d0f32ae95b409a0ac29e4cc0))
    - reset changes in gen/ to previous state. ([`da030c0`](https://github.com/Byron/google-apis-rs/commit/da030c0fffebb6e95b8cf1c22e8a81eb9ad26bd1))
    - Resolves 'Panic on http2 timeout ' ([`6d0a974`](https://github.com/Byron/google-apis-rs/commit/6d0a9740dabdd865707f15367cbdf42a8b9370d0))
    - Merge pull request #341 from humb1t/patch-1 ([`0a8b3ca`](https://github.com/Byron/google-apis-rs/commit/0a8b3ca4e465a5fc9a099682d4b074255912c204))
    - Update README.md ([`7202684`](https://github.com/Byron/google-apis-rs/commit/72026848021d34615bbb23333e2f59df9b85dd41))
    - keep publishing marker ([`f00ca37`](https://github.com/Byron/google-apis-rs/commit/f00ca37fa4e96657f051292fc8ac610b80373c65))
    - Fix the client source to pass `cargo test` ([`7293ff5`](https://github.com/Byron/google-apis-rs/commit/7293ff5b10349281ed1c5416e79c2dc20524a738))
    - Restrict pyyaml version to avoid error ([`1a52ec5`](https://github.com/Byron/google-apis-rs/commit/1a52ec5801b31e0cb5a27c0d3986fe60ff1a94fe))
    - Fix doc-test preamble to use new builder API ([`c8bd7f7`](https://github.com/Byron/google-apis-rs/commit/c8bd7f77d8414465dac6283ff86405752bc32084))
    - Release google-drive3-cli v4.0.0+20220225 ([`acbcbfe`](https://github.com/Byron/google-apis-rs/commit/acbcbfe48d81ad36c692934166a81274efbf2650))
    - regen drive3 CLI ([`8e7a1ed`](https://github.com/Byron/google-apis-rs/commit/8e7a1ed7cf4ea3e228814d558e37d66e2f8e3d44))
    - Release google-drive3 v4.0.0+20220225 ([`0b2a9b3`](https://github.com/Byron/google-apis-rs/commit/0b2a9b3e08a9f78e47f218673c5a2479ce34d66f))
    - Regen drive3 4.0 ([`a83f75d`](https://github.com/Byron/google-apis-rs/commit/a83f75df88a70503e1e7a22d9ade63bfbdfcfcab))
    - bump major version of all crates to represent major structural changes ([`cbfe145`](https://github.com/Byron/google-apis-rs/commit/cbfe14524b0f923db8491e250c8f9b43c910d1ff))
    - Update to latest released yup-oauth2 v7 ([`5c22221`](https://github.com/Byron/google-apis-rs/commit/5c22221f77f81280f1ddbe3e8af58be8d28cf7eb))
    - Support custom connectors ([`c5ff239`](https://github.com/Byron/google-apis-rs/commit/c5ff239961df59cae23a7dd609d69b629f31e7fa))
    - markers for publishing APIs and CLIs ([`0b52431`](https://github.com/Byron/google-apis-rs/commit/0b5243162b46fac9f63cffc6a8fce9f906d2094b))
    - Release google-youtubereporting1-cli v3.1.0+20220305 ([`bd46e48`](https://github.com/Byron/google-apis-rs/commit/bd46e4849a9ef5498f602081aa97e306bfcf6985))
    - Release google-youtube3-cli v3.1.0+20220303 ([`e1904ec`](https://github.com/Byron/google-apis-rs/commit/e1904ec2e164f99a1f0ce96d5c14dca6e97d7692))
    - Release google-workflows1-cli v3.1.0+20220223 ([`c31dbb9`](https://github.com/Byron/google-apis-rs/commit/c31dbb92325673873b1550376d95ca389e31b514))
    - Release google-workflowexecutions1-cli v3.1.0+20220222 ([`c49a8c9`](https://github.com/Byron/google-apis-rs/commit/c49a8c97ccb696695506393ab906efccd90deea3))
    - Release google-webrisk1-cli v3.1.0+20220226 ([`22c43a4`](https://github.com/Byron/google-apis-rs/commit/22c43a46523877c95dd75c4e0529ffca42d0d96d))
    - Release google-webmasters3-cli v3.1.0+20190428 ([`6a15750`](https://github.com/Byron/google-apis-rs/commit/6a15750da9a96f3a3ea0ec0e0e5d777c0eeb6d8e))
    - Release google-webfonts1-cli v3.1.0+20220215 ([`054465f`](https://github.com/Byron/google-apis-rs/commit/054465fc691edd4509a37d3fe1a1d190d8bfbf26))
    - Release google-vmmigration1-cli v3.1.0+20220225 ([`8f08c40`](https://github.com/Byron/google-apis-rs/commit/8f08c401ae888890a0a6c7d66c3863f7f790cb52))
    - Release google-vision1-cli v3.1.0+20220225 ([`f3f831d`](https://github.com/Byron/google-apis-rs/commit/f3f831d10b8550e3fc0e6948eeaaa644caa2808a))
    - Release google-videointelligence1_beta1-cli v3.1.0+20171122 ([`dff5b0a`](https://github.com/Byron/google-apis-rs/commit/dff5b0ad3f7b64adc9fa50055ad48251188f62c8))
    - Release google-videointelligence1-cli v3.1.0+20220225 ([`b8d8d6f`](https://github.com/Byron/google-apis-rs/commit/b8d8d6fbf87a16501afd918fef0c2c49b9fe93e8))
    - Release google-versionhistory1-cli v3.1.0+20220307 ([`920960a`](https://github.com/Byron/google-apis-rs/commit/920960af19279efba26e92bbcee0e6d0ef33380d))
    - Release google-verifiedaccess1-cli v3.1.0+20220215 ([`bcf9b14`](https://github.com/Byron/google-apis-rs/commit/bcf9b14952fb9c724cc53f77f223aeb39037ad3f))
    - Release google-vectortile1-cli v3.1.0+20210331 ([`870d59b`](https://github.com/Byron/google-apis-rs/commit/870d59b7f4a2d5b9bfb01d7f3533ab4ee8b49f39))
    - Release google-vault1-cli v3.1.0+20220222 ([`eb51bb0`](https://github.com/Byron/google-apis-rs/commit/eb51bb0af9ef6ae289cc5e6718c68be916680a69))
    - Release google-urlshortener1-cli v3.1.0+20150519 ([`f92498c`](https://github.com/Byron/google-apis-rs/commit/f92498ccdbf9a181c5619ae7b5da9fd7e347992d))
    - Release google-translate3-cli v3.1.0+20220121 ([`2da2a2a`](https://github.com/Byron/google-apis-rs/commit/2da2a2ad855167103a359a004e95135f8986e02d))
    - Release google-translate2-cli v3.1.0+20170525 ([`fbc2516`](https://github.com/Byron/google-apis-rs/commit/fbc2516539c21297b8f77925b3685d271f42ee24))
    - Release google-transcoder1-cli v3.1.0+20220201 ([`c9d2ea3`](https://github.com/Byron/google-apis-rs/commit/c9d2ea33385d200efbd935baf3a0858289be3123))
    - Release google-transcoder1_beta1-cli v3.1.0+20210323 ([`3983ebc`](https://github.com/Byron/google-apis-rs/commit/3983ebc572af2ac54094a6e57b0225ba0d175dd0))
    - Release google-tpu1_alpha1-cli v3.1.0+20220301 ([`000be32`](https://github.com/Byron/google-apis-rs/commit/000be32c63a1eacb87da73ed0754d318e9b364f6))
    - Release google-tpu1-cli v3.1.0+20220301 ([`f95a9f1`](https://github.com/Byron/google-apis-rs/commit/f95a9f1c724effb37038465608387d316a80a3c2))
    - Release google-texttospeech1-cli v3.1.0+20220228 ([`fdb2c0f`](https://github.com/Byron/google-apis-rs/commit/fdb2c0f3ebaf02bce14aa48411e3b12065afbdb4))
    - Release google-testing1-cli v3.1.0+20220301 ([`ac8926c`](https://github.com/Byron/google-apis-rs/commit/ac8926c1935ccd2f7a73c152aab9d0e8f4d8658d))
    - Release google-tasks1-cli v3.1.0+20220305 ([`1bda825`](https://github.com/Byron/google-apis-rs/commit/1bda8252a5767a4ad08f020c91156681c92c9235))
    - Release google-taskqueue1_beta2-cli v3.1.0+20160428 ([`081e108`](https://github.com/Byron/google-apis-rs/commit/081e108c19d0390e4eff0deaa86c5e22ff4e5c15))
    - Release google-tagmanager2-cli v3.1.0+20220301 ([`bec1274`](https://github.com/Byron/google-apis-rs/commit/bec127467503eb3c3978224beab33947bcb0eaf1))
    - Release google-tagmanager1-cli v3.1.0+20220301 ([`15d89b2`](https://github.com/Byron/google-apis-rs/commit/15d89b21274db9b25611f4ad171dd40af63e0d15))
    - Release google-surveys2-cli v3.1.0+20180508 ([`9585dba`](https://github.com/Byron/google-apis-rs/commit/9585dba0b2e582f3a98e450805ede661110a5466))
    - Release google-sts1-cli v3.1.0+20220227 ([`a3014c1`](https://github.com/Byron/google-apis-rs/commit/a3014c1aef4711fd5abbb657532addf4f5358b18))
    - Release google-storagetransfer1-cli v3.1.0+20220223 ([`92eaec6`](https://github.com/Byron/google-apis-rs/commit/92eaec6f5e8c3d367a8e5919bfb0c8ad85c7243f))
    - Release google-storage1-cli v3.1.0+20220228 ([`4797e51`](https://github.com/Byron/google-apis-rs/commit/4797e51a79e155c623cb737a4314b7a698d87108))
    - Release google-sqladmin1-cli v3.1.0+20220226 ([`7da0ced`](https://github.com/Byron/google-apis-rs/commit/7da0cedbfd6e493d02deb85c14c51d4d8266f9ed))
    - Release google-sqladmin1_beta4-cli v3.1.0+20220226 ([`c6c966b`](https://github.com/Byron/google-apis-rs/commit/c6c966b2a6e4963e37d985da0c4bf110a72fa9a5))
    - Release google-sql1_beta4-cli v3.1.0+20200331 ([`0dde9af`](https://github.com/Byron/google-apis-rs/commit/0dde9afa7d5dbc9a0338c6b6514faade24bef9ae))
    - Release google-speech1_beta1-cli v3.1.0+20181005 ([`39e2dad`](https://github.com/Byron/google-apis-rs/commit/39e2dadc396cc53b9d906fb9114cc2c17142a0cd))
    - Release google-speech1-cli v3.1.0+20220221 ([`2672550`](https://github.com/Byron/google-apis-rs/commit/26725502fe3e35bb6ddda0857823a7f0436ac92c))
    - Release google-spectrum1_explorer-cli v3.1.0+20170306 ([`168197a`](https://github.com/Byron/google-apis-rs/commit/168197a7e585ac97f2ec576dd42e407efad90a88))
    - Release google-sourcerepo1-cli v3.1.0+20220217 ([`353f334`](https://github.com/Byron/google-apis-rs/commit/353f334d0ebd73f36282474e476aaeaf111577e5))
    - Release google-smartdevicemanagement1-cli v3.1.0+20220302 ([`ac5782a`](https://github.com/Byron/google-apis-rs/commit/ac5782a2a15d9ef5daf4b822d436aab83aacb05a))
    - Release google-siteverification1-cli v3.1.0+20191119 ([`6cac79e`](https://github.com/Byron/google-apis-rs/commit/6cac79e30b6b0614025d9c3a1d12b134db4d1dc7))
    - Release google-sheets4-cli v3.1.0+20220221 ([`0538ff0`](https://github.com/Byron/google-apis-rs/commit/0538ff075470d3f56d7250247f15e5f4a9709ac9))
    - Release google-serviceregistryalpha-cli v3.1.0+20160401 ([`7ea3669`](https://github.com/Byron/google-apis-rs/commit/7ea3669c09c4418f97b700888e12fd9d50c583bc))
    - Release google-servicedirectory1_beta1-cli v3.1.0+20220224 ([`6620b88`](https://github.com/Byron/google-apis-rs/commit/6620b88e634dfda52218c76b8f5f554e61c0160a))
    - Release google-servicedirectory1-cli v3.1.0+20220224 ([`55f6233`](https://github.com/Byron/google-apis-rs/commit/55f6233fb29790e2365a5cebde83bf372b581fa7))
    - Release google-servicecontrol2-cli v3.1.0+20220227 ([`fe9d202`](https://github.com/Byron/google-apis-rs/commit/fe9d20214b18788d6e461c5426ab536a59198841))
    - Release google-servicecontrol1-cli v3.1.0+20220227 ([`f25b9d4`](https://github.com/Byron/google-apis-rs/commit/f25b9d47b56a489b8f3f0307fbfcd1d1bf2fb3e3))
    - Release google-servicebroker1-cli v3.1.0+20190624 ([`34d241a`](https://github.com/Byron/google-apis-rs/commit/34d241a4c2c9b117efcb3f01160248ca75021a30))
    - Release google-securitycenter1-cli v3.1.0+20220224 ([`877e97f`](https://github.com/Byron/google-apis-rs/commit/877e97f1bb78d6c1dba3efaa1bb491e7feceb10c))
    - Release google-secretmanager1_beta1-cli v3.1.0+20220226 ([`c9d0473`](https://github.com/Byron/google-apis-rs/commit/c9d0473b5201e6c17dd867081bf2cbad7ecc2f65))
    - Release google-secretmanager1-cli v3.1.0+20220226 ([`6cb1135`](https://github.com/Byron/google-apis-rs/commit/6cb11351e5d6041a39ce1cb90294b5ab4fa05af9))
    - Release google-searchconsole1-cli v3.1.0+20220305 ([`5d6a2c6`](https://github.com/Byron/google-apis-rs/commit/5d6a2c69f508a730cbfa02da6fd5697c93b4fe7a))
    - Release google-sasportal1_alpha1-cli v3.1.0+20220301 ([`5eca6eb`](https://github.com/Byron/google-apis-rs/commit/5eca6ebfbc2d79fdc9a56c03d9010cd5e389c987))
    - Release google-safebrowsing4-cli v3.1.0+20220305 ([`f491723`](https://github.com/Byron/google-apis-rs/commit/f4917237b9ebe12325505238a724b67101dd15cf))
    - Release google-runtimeconfig1_beta1-cli v3.1.0+20220228 ([`aee0299`](https://github.com/Byron/google-apis-rs/commit/aee02998a1ce5c6386fea729a1d177fc32eaf47a))
    - Release google-privateca1_beta1-cli v3.1.0+20220209 ([`464be47`](https://github.com/Byron/google-apis-rs/commit/464be47b72ab9d3bc3b7b05788ffdcab72bac270))
    - Release google-prediction1d6-cli v3.1.0+20160511 ([`6865645`](https://github.com/Byron/google-apis-rs/commit/6865645dc2a410a6b7318d03a236fa41bce62f8f))
    - Release google-policytroubleshooter1-cli v3.1.0+20220227 ([`e528454`](https://github.com/Byron/google-apis-rs/commit/e528454c890077516a9314fe1193f487ec456ed6))
    - Release google-policysimulator1-cli v3.1.0+20220227 ([`b8c1e72`](https://github.com/Byron/google-apis-rs/commit/b8c1e72099da18e8cd71e6bf1ef7b9b60126c44a))
    - Release google-policyanalyzer1-cli v3.1.0+20220227 ([`32ef61c`](https://github.com/Byron/google-apis-rs/commit/32ef61ca583443ddbdcd60408ba01560870bfb1d))
    - Release google-plusdomains1-cli v3.1.0+20190616 ([`022efc5`](https://github.com/Byron/google-apis-rs/commit/022efc5145523da61e21fe0345934f7077f74d4a))
    - Release google-plus1-cli v3.1.0+20190616 ([`f372491`](https://github.com/Byron/google-apis-rs/commit/f372491b470ff0916ad1f54c75fcf57a5f18dbbb))
    - Release google-playmoviespartner1-cli v3.1.0+20170919 ([`5c9e2cf`](https://github.com/Byron/google-apis-rs/commit/5c9e2cf6eaca981af25e316f3ed5ceae95ddb86b))
    - Release google-playintegrity1-cli v3.1.0+20220305 ([`5b7f1f1`](https://github.com/Byron/google-apis-rs/commit/5b7f1f1b9d72153d389d40a36b2b19f44d3eb86d))
    - Release google-playcustomapp1-cli v3.1.0+20220305 ([`3e8bfb6`](https://github.com/Byron/google-apis-rs/commit/3e8bfb643caeabf6627ec6c4424e1144a4bb0e7d))
    - Release google-playablelocations3-cli v3.1.0+20200707 ([`c307a93`](https://github.com/Byron/google-apis-rs/commit/c307a936addd5af77c8267b240861d3fe04834ff))
    - Release google-photoslibrary1-cli v3.1.0+20220303 ([`659c1a1`](https://github.com/Byron/google-apis-rs/commit/659c1a14f6c724a9f9e5106b568f06aadd3404a6))
    - Release google-people1-cli v3.1.0+20220303 ([`6dd8e1b`](https://github.com/Byron/google-apis-rs/commit/6dd8e1b9714546e322b1d32b26bda3f9b7cd1cc3))
    - Release google-paymentsresellersubscription1-cli v3.1.0+20220307 ([`93cf931`](https://github.com/Byron/google-apis-rs/commit/93cf93177317d441eebcb7e9f0e0a7fc24666204))
    - Release google-partners2-cli v3.1.0+20180925 ([`1b0387a`](https://github.com/Byron/google-apis-rs/commit/1b0387a8f0c6b2ba88dbb6c0bfb2398b83ad28b1))
    - Release google-pagespeedonline5-cli v3.1.0+20220302 ([`06da688`](https://github.com/Byron/google-apis-rs/commit/06da6889005c0bdcb309157d92a779457860cb0e))
    - Release google-pagespeedonline4-cli v3.1.0+20191206 ([`6940601`](https://github.com/Byron/google-apis-rs/commit/6940601f9f4b000bae30d1515509bde472fae113))
    - Release google-pagespeedonline2-cli v3.1.0+20191206 ([`85b8802`](https://github.com/Byron/google-apis-rs/commit/85b880296c7100508dcac5b9266b959659a60215))
    - Release google-oslogin1_beta-cli v3.1.0+20220228 ([`ee7dca0`](https://github.com/Byron/google-apis-rs/commit/ee7dca057ee79ecaa608cbb30924823eddbf1ea5))
    - Release google-oslogin1-cli v3.1.0+20220228 ([`a6e6e78`](https://github.com/Byron/google-apis-rs/commit/a6e6e78e4573a81f3d01d0a55ced65e864203daa))
    - Release google-orgpolicy2-cli v3.1.0+20220305 ([`ab3b98d`](https://github.com/Byron/google-apis-rs/commit/ab3b98dbc39340f3934da8005cda236138c6ce21))
    - Release google-ondemandscanning1-cli v3.1.0+20220228 ([`1c56210`](https://github.com/Byron/google-apis-rs/commit/1c562104f6ff7bde29d245fec7326e5ae97a0c42))
    - Release google-notebooks1-cli v3.1.0+20220224 ([`f3b8649`](https://github.com/Byron/google-apis-rs/commit/f3b86495448ca3ec1e12fd7bcadd7609400ee872))
    - Release google-networkservices1-cli v3.1.0+20220222 ([`5c616dc`](https://github.com/Byron/google-apis-rs/commit/5c616dcfbaebdacb2d3c1b48db6eaa798d0b3927))
    - Release google-networksecurity1-cli v3.1.0+20220223 ([`0c117cd`](https://github.com/Byron/google-apis-rs/commit/0c117cd2954c9172f2e3a4fabfdefe4e01056fc8))
    - Release google-networkmanagement1-cli v3.1.0+20220223 ([`c1d1b0f`](https://github.com/Byron/google-apis-rs/commit/c1d1b0f2c8a278f7168d8e727c3f6a79f2531f89))
    - Release google-networkconnectivity1-cli v3.1.0+20220210 ([`71464a8`](https://github.com/Byron/google-apis-rs/commit/71464a8d7910b727e8f87935ae082274d1f9969b))
    - Release google-networkconnectivity1_alpha1-cli v3.1.0+20220210 ([`0c0e755`](https://github.com/Byron/google-apis-rs/commit/0c0e75579939f58178c6e86fea9c425f83494fc7))
    - Release google-mybusinessverifications1-cli v3.1.0+20220305 ([`44642ed`](https://github.com/Byron/google-apis-rs/commit/44642ed4da5d0f4f8f20d9c422b8510220968b07))
    - Release google-mybusinessplaceactions1-cli v3.1.0+20220305 ([`e3ae08f`](https://github.com/Byron/google-apis-rs/commit/e3ae08f928312a65abeaf86a42b81b133477b3a9))
    - Release google-mybusinessnotifications1-cli v3.1.0+20220305 ([`840824b`](https://github.com/Byron/google-apis-rs/commit/840824bd7eb1eea939e17f838855da61dfab046c))
    - Release google-mybusinesslodging1-cli v3.1.0+20220305 ([`7f1de21`](https://github.com/Byron/google-apis-rs/commit/7f1de21a1888fbf42f83fa774283047502f6abb7))
    - Release google-mybusinessbusinessinformation1-cli v3.1.0+20220305 ([`79f4a2a`](https://github.com/Byron/google-apis-rs/commit/79f4a2a679cb08831542457e23e282c55a10c9ba))
    - Release google-mybusinessbusinesscalls1-cli v3.1.0+20220305 ([`c9d2ab3`](https://github.com/Byron/google-apis-rs/commit/c9d2ab3416cc971349d0d6a5da402289e6cefd23))
    - Release google-mybusinessaccountmanagement1-cli v3.1.0+20220305 ([`9c7b7f6`](https://github.com/Byron/google-apis-rs/commit/9c7b7f626b244d5d742db0023be8f6dbb5102cf8))
    - Release google-mybusiness4-cli v3.1.0+0 ([`8a95583`](https://github.com/Byron/google-apis-rs/commit/8a95583d05a72c520f6085b9331016f124161a39))
    - Release google-monitoring3-cli v3.1.0+20220218 ([`14da499`](https://github.com/Byron/google-apis-rs/commit/14da499b282d9cd4afc3fa416e8564942e4d66bb))
    - Release google-ml1-cli v3.1.0+20220212 ([`f4c352d`](https://github.com/Byron/google-apis-rs/commit/f4c352d89721e5c58b6a6ee8322c5e74171f441a))
    - Release google-mirror1-cli v3.1.0+20190424 ([`5e06d0e`](https://github.com/Byron/google-apis-rs/commit/5e06d0e1814290a29008888f0a3045a1ee04979b))
    - Release google-metastore1_beta-cli v3.1.0+20220222 ([`7d9d63f`](https://github.com/Byron/google-apis-rs/commit/7d9d63febc0cb85b5dd5756e5fff472e774baf24))
    - Release google-memcache1_beta2-cli v3.1.0+20220224 ([`17b5a18`](https://github.com/Byron/google-apis-rs/commit/17b5a1895232f81abe2b522b81a2887d1fed594f))
    - Release google-memcache1-cli v3.1.0+20220224 ([`2a5160c`](https://github.com/Byron/google-apis-rs/commit/2a5160cd8837459282b158edfad60e1a99331624))
    - Release google-manufacturers1-cli v3.1.0+20220303 ([`f649250`](https://github.com/Byron/google-apis-rs/commit/f6492508ab04c8c1344d463a0f67ab38d5119677))
    - Release google-manager1_beta2-cli v3.1.0+20140915 ([`aa67d9e`](https://github.com/Byron/google-apis-rs/commit/aa67d9e47b0f11589b8e66bc138a5f61d71388bc))
    - Release google-managedidentities1-cli v3.1.0+20220216 ([`661eadf`](https://github.com/Byron/google-apis-rs/commit/661eadf80be6a963ca29abb3b85fe161d754faf7))
    - Release google-logging2_beta1-cli v3.1.0+20190325 ([`379db4c`](https://github.com/Byron/google-apis-rs/commit/379db4c16b87f1415f92fe7d22c6b1c093f3c526))
    - Release google-logging2-cli v3.1.0+20220225 ([`266da77`](https://github.com/Byron/google-apis-rs/commit/266da7741d46328888c78856a9f39a39ae3accd3))
    - Release google-localservices1-cli v3.1.0+20220305 ([`0f438ce`](https://github.com/Byron/google-apis-rs/commit/0f438ce692d5105bd7bb2f1b3bca99f805fef568))
    - Release google-lifesciences2_beta-cli v3.1.0+20220211 ([`42d1e12`](https://github.com/Byron/google-apis-rs/commit/42d1e12822aefaef270d751ace0adbd86b7765ce))
    - Release google-licensing1-cli v3.1.0+20220305 ([`cfbf86b`](https://github.com/Byron/google-apis-rs/commit/cfbf86b5967902bba6dbbd41dfff7f9ba8663079))
    - Release google-libraryagent1-cli v3.1.0+20220305 ([`1074002`](https://github.com/Byron/google-apis-rs/commit/1074002e283006ce682187b940cbc6beea120817))
    - Release google-language1_beta1-cli v3.1.0+20220218 ([`a254fbe`](https://github.com/Byron/google-apis-rs/commit/a254fbe0f5b8231040be895283925e49f5bff1f3))
    - Release google-language1-cli v3.1.0+20220218 ([`5319b2d`](https://github.com/Byron/google-apis-rs/commit/5319b2df8a3cbc8e61884c292195117a08120a23))
    - Release google-keep1-cli v3.1.0+20220301 ([`cc6d2d0`](https://github.com/Byron/google-apis-rs/commit/cc6d2d08d7166f1347fff53bfa6b9a66a788f6fe))
    - Release google-jobs4-cli v3.1.0+20220211 ([`940f355`](https://github.com/Byron/google-apis-rs/commit/940f3553d55d63d2097c149b7f175aaaf9488064))
    - Release google-jobs3-cli v3.1.0+20220211 ([`76c015c`](https://github.com/Byron/google-apis-rs/commit/76c015cbc309c878b97c74e6ed03154a97ef1826))
    - Release google-indexing3-cli v3.1.0+20220126 ([`5b80ca9`](https://github.com/Byron/google-apis-rs/commit/5b80ca9edf50d611716d796601707e11ce1deb9a))
    - Release google-ids1-cli v3.1.0+20220221 ([`c5e034d`](https://github.com/Byron/google-apis-rs/commit/c5e034d1cd13ea4769ba57b79903013a1a7d56e4))
    - Release google-identitytoolkit3-cli v3.1.0+20180723 ([`4b33ec7`](https://github.com/Byron/google-apis-rs/commit/4b33ec7f8d429a2d6e525616377ff1bd07bd3ff2))
    - Release google-ideahub1_beta-cli v3.1.0+20220305 ([`527b4bc`](https://github.com/Byron/google-apis-rs/commit/527b4bc78c7c425ad4d013f7982dfd14ffabd2ba))
    - Release google-iap1_beta1-cli v3.1.0+20220225 ([`eafa95a`](https://github.com/Byron/google-apis-rs/commit/eafa95ace29de14bf411cb96d1ee0c82fcf35906))
    - Release google-iap1-cli v3.1.0+20220225 ([`88db424`](https://github.com/Byron/google-apis-rs/commit/88db42463ce38334ae6415fd161a6b29cd6986a4))
    - Release google-iamcredentials1-cli v3.1.0+20220225 ([`f5325b0`](https://github.com/Byron/google-apis-rs/commit/f5325b028b571cccb1d0fe4f781750112ea5d210))
    - Release google-iam1-cli v3.1.0+20220224 ([`4d4aec3`](https://github.com/Byron/google-apis-rs/commit/4d4aec383be6c19ca28fe2816aeb6e1ed0719b90))
    - Release google-healthcare1_beta1-cli v3.1.0+20220223 ([`600eb58`](https://github.com/Byron/google-apis-rs/commit/600eb58c51d6022882d01f0aa4e4a0c18d74a40d))
    - Release google-healthcare1-cli v3.1.0+20220223 ([`60f0b5d`](https://github.com/Byron/google-apis-rs/commit/60f0b5d1a55169999fff811d926625f36b844b0b))
    - Release google-groupssettings1-cli v3.1.0+20220224 ([`260202c`](https://github.com/Byron/google-apis-rs/commit/260202c16e6d71b1130e468ffa9d1a2357b7c08f))
    - Release google-groupsmigration1-cli v3.1.0+20220226 ([`a066e51`](https://github.com/Byron/google-apis-rs/commit/a066e512efc0c59a19c2c68f22489d99f03db1fd))
    - Release google-gmailpostmastertools1_beta1-cli v3.1.0+20220305 ([`f15beda`](https://github.com/Byron/google-apis-rs/commit/f15bedaa59ca05b6641174aac61be7c5079d27b3))
    - Release google-gmailpostmastertools1-cli v3.1.0+20220305 ([`549165f`](https://github.com/Byron/google-apis-rs/commit/549165ff21cc38fc4371177ed661fd7cbd4e9aa9))
    - Release google-gmail1-cli v3.1.0+20220228 ([`c16fb58`](https://github.com/Byron/google-apis-rs/commit/c16fb586b396c3ab23cc55a3e10fef97a20bda7b))
    - Release google-gkehub1-cli v3.1.0+20220211 ([`eff59ec`](https://github.com/Byron/google-apis-rs/commit/eff59ec5e27497e77d92d17faae8b5118f9014e0))
    - Release google-genomics1-cli v3.1.0+20210324 ([`d82e9fe`](https://github.com/Byron/google-apis-rs/commit/d82e9fe1e77b25e4ecf3f9289b40b73404b0b741))
    - Release google-gan1_beta1-cli v3.1.0+20130205 ([`ce04b63`](https://github.com/Byron/google-apis-rs/commit/ce04b6396690fc568e7a2d340a3313714ad3e3dd))
    - Release google-gamesmanagement1_management-cli v3.1.0+20220217 ([`e84a315`](https://github.com/Byron/google-apis-rs/commit/e84a315a0bf41c6f8d66c10814975ed1bd51c098))
    - Release google-gameservices1-cli v3.1.0+20220223 ([`40efce3`](https://github.com/Byron/google-apis-rs/commit/40efce33b97a087cea70f0d11b9e9a97bc736780))
    - Release google-gamesconfiguration1_configuration-cli v3.1.0+20220217 ([`7bbd167`](https://github.com/Byron/google-apis-rs/commit/7bbd16788fc2f2ede39e3a1c2a5af3a098012dcb))
    - Release google-games1-cli v3.1.0+20220217 ([`e5c18dc`](https://github.com/Byron/google-apis-rs/commit/e5c18dc8bc6bb39824437b0dae79c2fa52199f03))
    - Release google-fusiontables2-cli v3.1.0+20171117 ([`2559463`](https://github.com/Byron/google-apis-rs/commit/255946365cceaf73b6b37a4524f77dca265b5546))
    - Release google-fitness1-cli v3.1.0+20220302 ([`9c97427`](https://github.com/Byron/google-apis-rs/commit/9c974276997839e291b1050ae4bb63540ca90e10))
    - Release google-firestore1_beta1-cli v3.1.0+20220221 ([`5749938`](https://github.com/Byron/google-apis-rs/commit/5749938d52fa4671e61088a37631f625b03143c2))
    - Release google-firestore1-cli v3.1.0+20220221 ([`5f63a79`](https://github.com/Byron/google-apis-rs/commit/5f63a792faedd43be163c25133572d11fd636a67))
    - Release google-firebasestorage1_beta-cli v3.1.0+20220218 ([`25abb41`](https://github.com/Byron/google-apis-rs/commit/25abb41716c51a0d9f49ec2e5cca6fb292ef5e93))
    - Release google-firebaseremoteconfig1-cli v3.1.0+20171129 ([`b5b3e72`](https://github.com/Byron/google-apis-rs/commit/b5b3e7203cf94a05427eae3aaf8bc23d7745ecff))
    - Release google-firebaseml1-cli v3.1.0+20220302 ([`150b422`](https://github.com/Byron/google-apis-rs/commit/150b422a789c0bd39484a7f352230d1e3d3d8a4d))
    - Release google-firebasehosting1_beta1-cli v3.1.0+20220212 ([`b3b67d1`](https://github.com/Byron/google-apis-rs/commit/b3b67d106d8af763a26808c0e3dbaaf246ad8937))
    - Release google-firebasehosting1-cli v3.1.0+20220212 ([`6456dcd`](https://github.com/Byron/google-apis-rs/commit/6456dcd41c5b88c7877e03e6a6d0148104d567c8))
    - Release google-firebasedynamiclinks1-cli v3.1.0+20220228 ([`a57c952`](https://github.com/Byron/google-apis-rs/commit/a57c952de4e8d69263995005e1f34bdb228c94ec))
    - Release google-firebasedatabase1_beta-cli v3.1.0+20220304 ([`0e5c683`](https://github.com/Byron/google-apis-rs/commit/0e5c6839802cf542f1d9fad7b019ba9869296018))
    - Release google-firebaseappcheck1_beta-cli v3.1.0+20220225 ([`14eba3a`](https://github.com/Byron/google-apis-rs/commit/14eba3a9914c1131f68b6cf21a733c1cec3dc224))
    - Release google-firebase1_beta1-cli v3.1.0+20220304 ([`5204a0d`](https://github.com/Byron/google-apis-rs/commit/5204a0dae0053d3c9d870b3c296783b075e8dcdd))
    - Release google-file1_beta1-cli v3.1.0+20220214 ([`59277cd`](https://github.com/Byron/google-apis-rs/commit/59277cdf373e08b8b33ac69105e8abdb7ec2a647))
    - Release google-file1-cli v3.1.0+20220214 ([`2445f49`](https://github.com/Byron/google-apis-rs/commit/2445f49cb4ec3c52d1963cd82bbbce4e1ba30403))
    - Release google-fcmdata1_beta1-cli v3.1.0+20220305 ([`ec55291`](https://github.com/Byron/google-apis-rs/commit/ec55291574118d5fd633b17676248728cece854c))
    - Release google-fcm1-cli v3.1.0+20220228 ([`0790df3`](https://github.com/Byron/google-apis-rs/commit/0790df3a8fc41d35113527cf75faa189b5a4dafa))
    - Release google-factchecktools1_alpha1-cli v3.1.0+20220305 ([`5978329`](https://github.com/Byron/google-apis-rs/commit/597832922c080bdc0775a1c10f64cb02350b5c9a))
    - Release google-eventarc1-cli v3.1.0+20220301 ([`9b2a560`](https://github.com/Byron/google-apis-rs/commit/9b2a56050d8f71a316fc27d4ec3c32d0084b3206))
    - Release google-essentialcontacts1-cli v3.1.0+20220227 ([`2921806`](https://github.com/Byron/google-apis-rs/commit/29218067bd647a2076079b1c47696f19ef451bb4))
    - Release google-driveactivity2-cli v3.1.0+20220301 ([`7361ff3`](https://github.com/Byron/google-apis-rs/commit/7361ff3e3f4139d0b9387df65e7e449ac9794153))
    - Release google-drive3-cli v3.1.0+20220225 ([`0cf8e9e`](https://github.com/Byron/google-apis-rs/commit/0cf8e9e91ed89edc52a908291829d993b94260da))
    - Release google-drive2-cli v3.1.0+20220225 ([`d9e0ff9`](https://github.com/Byron/google-apis-rs/commit/d9e0ff9ea86411f24d6a460009a3582c55657126))
    - Release google-doubleclicksearch2-cli v3.1.0+20220301 ([`b9b9aaf`](https://github.com/Byron/google-apis-rs/commit/b9b9aaf2ce13877813bfe5610f0f8b07463635eb))
    - Release google-doubleclickbidmanager1d1-cli v3.1.0+20220302 ([`23a0215`](https://github.com/Byron/google-apis-rs/commit/23a0215b959d9685f7832cfac91860ec1df507bf))
    - Release google-doubleclickbidmanager1-cli v3.1.0+20210323 ([`14c5045`](https://github.com/Byron/google-apis-rs/commit/14c5045e6102ba305347a63b0771a7c4c29258fd))
    - Release google-domainsrdap1-cli v3.1.0+20220307 ([`41efb87`](https://github.com/Byron/google-apis-rs/commit/41efb8751e5ad8681dc6956ea9e21dbe476174b8))
    - Release google-domains1-cli v3.1.0+20220128 ([`b453b0e`](https://github.com/Byron/google-apis-rs/commit/b453b0e6a1e71ed40fd68ba6887dbe8122ece44d))
    - Release google-domains1_beta1-cli v3.1.0+20220128 ([`1d80714`](https://github.com/Byron/google-apis-rs/commit/1d8071422fa9da3974d661cc467b11148c9423f7))
    - Release google-documentai1_beta2-cli v3.1.0+20220226 ([`b693066`](https://github.com/Byron/google-apis-rs/commit/b693066954e7122674b55c8355f63c5ea9669930))
    - Release google-documentai1-cli v3.1.0+20220226 ([`d3a4fe1`](https://github.com/Byron/google-apis-rs/commit/d3a4fe1c7d227835714497164ca0e47fa9c2744b))
    - Release google-docs1-cli v3.1.0+20220301 ([`524e69d`](https://github.com/Byron/google-apis-rs/commit/524e69df2354042d0a2ac13fa07f461bb8a0b145))
    - Release google-dns2-cli v3.1.0+20220217 ([`75c82fc`](https://github.com/Byron/google-apis-rs/commit/75c82fce3bc3e791d14f3f06840fd8adc6836d4c))
    - Release google-dns1-cli v3.1.0+20220217 ([`df94fb8`](https://github.com/Byron/google-apis-rs/commit/df94fb8ad14fd9c4598f8a576613ac86768f1952))
    - Release google-dlp2_beta1-cli v3.1.0+20171205 ([`67296e5`](https://github.com/Byron/google-apis-rs/commit/67296e509aaaf50ae8529bb797ae0164a8193e32))
    - Release google-dlp2-cli v3.1.0+20220227 ([`5213159`](https://github.com/Byron/google-apis-rs/commit/52131590d9b1117b88180164cee78eb0da4b0863))
    - Release google-displayvideo1-cli v3.1.0+20220303 ([`4ce0479`](https://github.com/Byron/google-apis-rs/commit/4ce04790680446024189df768d874c7b180cf87b))
    - Release google-discovery1-cli v3.1.0+20200806 ([`858c5d4`](https://github.com/Byron/google-apis-rs/commit/858c5d42dfd5ae0efe9fef56a8173ffb0ac582f3))
    - Release google-digitalassetlinks1-cli v3.1.0+20220301 ([`5def045`](https://github.com/Byron/google-apis-rs/commit/5def045623fcaedf7c038c1cc11637fa39945f8a))
    - Release google-dialogflow3-cli v3.1.0+20220228 ([`28e242f`](https://github.com/Byron/google-apis-rs/commit/28e242ff3ff61ea62a2d291b2c7eac45134e3314))
    - Release google-dialogflow2_beta1-cli v3.1.0+20220228 ([`7f5c04e`](https://github.com/Byron/google-apis-rs/commit/7f5c04ec4d01601e078c7cdbc3ca8e93355762d9))
    - Release google-dialogflow2-cli v3.1.0+20220228 ([`5cac160`](https://github.com/Byron/google-apis-rs/commit/5cac160fd1a6f3cb285e7cbb59a19f19b347161e))
    - Release google-dfareporting3d5-cli v3.1.0+20220104 ([`ea055e1`](https://github.com/Byron/google-apis-rs/commit/ea055e1cdef3390571bf0c3583fe946d427aad55))
    - Release google-dfareporting3d4-cli v3.1.0+20220104 ([`3cabd2d`](https://github.com/Byron/google-apis-rs/commit/3cabd2db2448ba6030df7dfbe4ba7ca74ca076b4))
    - Release google-dfareporting3d3-cli v3.1.0+20220104 ([`4390400`](https://github.com/Byron/google-apis-rs/commit/43904001638869d7dad1a4a8a00cf10830d4c878))
    - Release google-dfareporting3d2-cli v3.1.0+20190531 ([`fe868bf`](https://github.com/Byron/google-apis-rs/commit/fe868bfae5c9efb3ec19f321a505ddbc579a28a0))
    - Release google-dfareporting3-cli v3.1.0+20180830 ([`0a4360e`](https://github.com/Byron/google-apis-rs/commit/0a4360e1c98367dfbf43e773bbe8a6e0f4a0666e))
    - Release google-dfareporting2d8-cli v3.1.0+20180830 ([`fcea7a7`](https://github.com/Byron/google-apis-rs/commit/fcea7a79437000f8389a8849c0fe79600ab073b5))
    - Release google-deploymentmanager2_beta2-cli v3.1.0+20160201 ([`a22dd7f`](https://github.com/Byron/google-apis-rs/commit/a22dd7fa67a5a9dda234b6310e12465fd2220679))
    - Release google-deploymentmanager2-cli v3.1.0+20220225 ([`93ae481`](https://github.com/Byron/google-apis-rs/commit/93ae48133af7874944ad50c713660c60b18e692d))
    - Release google-datastream1-cli v3.1.0+20220207 ([`3734a27`](https://github.com/Byron/google-apis-rs/commit/3734a27dbde91125c5fec697198434348c60c2f5))
    - Release google-datastore1_beta3-cli v3.1.0+20220221 ([`bbe255d`](https://github.com/Byron/google-apis-rs/commit/bbe255d9a0db4484608d6ca9076d76339c7381e9))
    - Release google-datastore1-cli v3.1.0+20220221 ([`c24fdee`](https://github.com/Byron/google-apis-rs/commit/c24fdee702af66e54260a7568edd56b863aea31f))
    - Release google-dataproc1-cli v3.1.0+20220224 ([`3736202`](https://github.com/Byron/google-apis-rs/commit/373620255f6e4a9d3610953970d5bbe6d662a202))
    - Release google-dataplex1-cli v3.1.0+20220223 ([`7ccdd96`](https://github.com/Byron/google-apis-rs/commit/7ccdd968da83ec8abae1d298a447537749deba6d))
    - Release google-datapipelines1-cli v3.1.0+20220218 ([`8568153`](https://github.com/Byron/google-apis-rs/commit/8568153e3fe82b4a41fa23ded80ce66dbf5c1be3))
    - Release google-datamigration1-cli v3.1.0+20220216 ([`1ba922b`](https://github.com/Byron/google-apis-rs/commit/1ba922bcf191a69d0838cadbce95b2c50497bcc0))
    - Release google-datalabeling1_beta1-cli v3.1.0+20220301 ([`c6385e1`](https://github.com/Byron/google-apis-rs/commit/c6385e142647c00e3484520b9ba16fc565537056))
    - Release google-datafusion1_beta1-cli v3.1.0+20211028 ([`004a4b4`](https://github.com/Byron/google-apis-rs/commit/004a4b4c13a512466c6322b0be01320e4685eb17))
    - Release google-datafusion1-cli v3.1.0+20211028 ([`91747a0`](https://github.com/Byron/google-apis-rs/commit/91747a05009a6c964d26de12a62721d736b9c774))
    - Release google-datacatalog1-cli v3.1.0+20220224 ([`16d73b0`](https://github.com/Byron/google-apis-rs/commit/16d73b05a9d3c951749a89702f83d120a89ade26))
    - Release google-datacatalog1_beta1-cli v3.1.0+20220224 ([`30a5746`](https://github.com/Byron/google-apis-rs/commit/30a5746f62b4f60e13c592547ff81e215c2d23c5))
    - Release google-customsearch1-cli v3.1.0+20220305 ([`1911ef8`](https://github.com/Byron/google-apis-rs/commit/1911ef83a2eca6a5abe5119f857d4b9ef1b69200))
    - Release google-coordinate1-cli v3.1.0+20150811 ([`0432986`](https://github.com/Byron/google-apis-rs/commit/0432986a0ae7f1f48ef66b95ce43fabfb1be06c1))
    - Release google-content2_sandbox-cli v3.1.0+20181009 ([`be85d57`](https://github.com/Byron/google-apis-rs/commit/be85d579d4ef443300a4e27287f47ee2863aa2e6))
    - Release google-content2-cli v3.1.0+20220303 ([`8e32396`](https://github.com/Byron/google-apis-rs/commit/8e3239655483d649f54d83d2c8d813a9d12959e6))
    - Release google-containeranalysis1-cli v3.1.0+20220225 ([`664366a`](https://github.com/Byron/google-apis-rs/commit/664366ac8a448d1a076c520b1ba6f4ef134b7ba7))
    - Release google-containeranalysis1_beta1-cli v3.1.0+20220225 ([`3a2b90f`](https://github.com/Byron/google-apis-rs/commit/3a2b90f536b494ba2593592995506e0ccab243db))
    - Release google-container1-cli v3.1.0+20220215 ([`29f9d3a`](https://github.com/Byron/google-apis-rs/commit/29f9d3a8f31f6b98562e3a78f0759baed7a1d8cd))
    - Release google-contactcenterinsights1-cli v3.1.0+20220227 ([`4c6dffb`](https://github.com/Byron/google-apis-rs/commit/4c6dffb8aa07541ecc7c7ffd7478b8864cb3bc70))
    - Release google-consumersurveys2-cli v3.1.0+20170407 ([`7baca37`](https://github.com/Byron/google-apis-rs/commit/7baca3761dd605f63220834df329aca6d384e4c6))
    - Release google-connectors1-cli v3.1.0+20220214 ([`9afc978`](https://github.com/Byron/google-apis-rs/commit/9afc9789d9dd6fdeb63867004597730719d29068))
    - Release google-compute1-cli v3.1.0+20220224 ([`5ba990d`](https://github.com/Byron/google-apis-rs/commit/5ba990d38645e5767e27f1112b463a8a7ca2d4af))
    - Release google-composer1-cli v3.1.0+20220224 ([`a48c4a1`](https://github.com/Byron/google-apis-rs/commit/a48c4a1da2274e94296a02302566a5fff2efcb0b))
    - Release google-commentanalyzer1_alpha1-cli v3.1.0+20200405 ([`23de84f`](https://github.com/Byron/google-apis-rs/commit/23de84f239cb942a75956df5122ab3a3133d08a5))
    - Release google-clouduseraccountsvm_beta-cli v3.1.0+20160316 ([`7c86a08`](https://github.com/Byron/google-apis-rs/commit/7c86a0871ca40187b0852834cd7288d00005e85c))
    - Release google-cloudtrace2-cli v3.1.0+20220224 ([`fd76a32`](https://github.com/Byron/google-apis-rs/commit/fd76a32db35b6fbab16bd82044c86bb3eee71020))
    - Release google-cloudtrace1-cli v3.1.0+20220224 ([`8c7f848`](https://github.com/Byron/google-apis-rs/commit/8c7f848fb927159639efa5e17754824ee47fb238))
    - Release google-cloudtasks2_beta3-cli v3.1.0+20220212 ([`738a2fe`](https://github.com/Byron/google-apis-rs/commit/738a2fe2e69da214c87e873b453bf07c1bd87034))
    - Release google-cloudtasks2_beta2-cli v3.1.0+20220212 ([`f0fc22a`](https://github.com/Byron/google-apis-rs/commit/f0fc22a59733bd0fd78448aec360b32720db3418))
    - Release google-cloudtasks2-cli v3.1.0+20220212 ([`25f52e3`](https://github.com/Byron/google-apis-rs/commit/25f52e3f798cf57fca4facb06115b10bba6f829f))
    - Release google-cloudsupport2_beta-cli v3.1.0+20220305 ([`4a9cea5`](https://github.com/Byron/google-apis-rs/commit/4a9cea566814f4d50680b922ba7886ae49bc9e8a))
    - Release google-cloudshell1-cli v3.1.0+20220301 ([`f1a38fc`](https://github.com/Byron/google-apis-rs/commit/f1a38fc591aa40857a797f2a170864bdaf48d0e0))
    - Release google-cloudscheduler1_beta1-cli v3.1.0+20220212 ([`5941b76`](https://github.com/Byron/google-apis-rs/commit/5941b7689fc34f6a2745d60fd1f451d03dc1f29a))
    - Release google-cloudscheduler1-cli v3.1.0+20220212 ([`04bdd30`](https://github.com/Byron/google-apis-rs/commit/04bdd30c4f2885cb03f00074b18393fc0d8d7b10))
    - Release google-cloudresourcemanager3-cli v3.1.0+20220306 ([`b9841a5`](https://github.com/Byron/google-apis-rs/commit/b9841a503611f3f7fb4f03b3d092c7ed8593bd09))
    - Release google-cloudresourcemanager2-cli v3.1.0+20220306 ([`5022041`](https://github.com/Byron/google-apis-rs/commit/502204190644e95dec6cdc9a25030ce31da4af9f))
    - Release google-cloudresourcemanager1_beta1-cli v3.1.0+20220306 ([`f300ca0`](https://github.com/Byron/google-apis-rs/commit/f300ca0f7111abfeb4c69d55c801aa4c61ee8c82))
    - Release google-cloudresourcemanager1-cli v3.1.0+20220306 ([`09fa4ed`](https://github.com/Byron/google-apis-rs/commit/09fa4edd25504d5508c3e92a909600bcb1d9ca96))
    - Release google-cloudprofiler2-cli v3.1.0+20220228 ([`1ffd4d4`](https://github.com/Byron/google-apis-rs/commit/1ffd4d42684f12fbf28d6ed8e6c1d5429b569471))
    - Release google-cloudprivatecatalogproducer1_beta1-cli v3.1.0+20200405 ([`a0ec65b`](https://github.com/Byron/google-apis-rs/commit/a0ec65bf03f1308a8a303dc9aa25f78f7803bc47))
    - Release google-cloudprivatecatalog1_beta1-cli v3.1.0+20200405 ([`9db9124`](https://github.com/Byron/google-apis-rs/commit/9db9124c47501d879061e300b836b293e1b90dd6))
    - Release google-cloudmonitoring2_beta2-cli v3.1.0+20170501 ([`e206a04`](https://github.com/Byron/google-apis-rs/commit/e206a044fd2f95f1c7f934ebbaefb68efda8e8ae))
    - Release google-cloudlatencytest2-cli v3.1.0+20160309 ([`2462c96`](https://github.com/Byron/google-apis-rs/commit/2462c96ff7192cdb69e936568aa971d635597898))
    - Release google-cloudkms1_beta1-cli v3.1.0+20170515 ([`f9281d2`](https://github.com/Byron/google-apis-rs/commit/f9281d29d078fcbff8fd98e0ee99565230f87d77))
    - Release google-cloudkms1-cli v3.1.0+20220225 ([`a850e74`](https://github.com/Byron/google-apis-rs/commit/a850e74ec45d660d513e1c20f57d4a782e21f238))
    - Release google-cloudiot1-cli v3.1.0+20220131 ([`08f6d87`](https://github.com/Byron/google-apis-rs/commit/08f6d878492cfd43c5d758ad42d398d0f5d8ea44))
    - Release google-cloudidentity1-cli v3.1.0+20220301 ([`29cf87d`](https://github.com/Byron/google-apis-rs/commit/29cf87dbdc27468967bcc1c544b960e4234cf947))
    - Release google-cloudfunctions1-cli v3.1.0+20220224 ([`9c86bda`](https://github.com/Byron/google-apis-rs/commit/9c86bda05a5d897612e81dad9dfa9c6a0abfb772))
    - Release google-clouderrorreporting1_beta1-cli v3.1.0+20220302 ([`fe6c0d0`](https://github.com/Byron/google-apis-rs/commit/fe6c0d0e0f3f97d0041def940a5734b05ac8ef26))
    - Release google-clouddeploy1-cli v3.1.0+20220223 ([`5c3ac73`](https://github.com/Byron/google-apis-rs/commit/5c3ac731e127b36a4dde3bcab4a3c778c23ac506))
    - Release google-clouddebugger2-cli v3.1.0+20220225 ([`aff11bf`](https://github.com/Byron/google-apis-rs/commit/aff11bf80d1a8f2c1d4f985f78f000fdd6c9faee))
    - Release google-cloudchannel1-cli v3.1.0+20220303 ([`7ed9528`](https://github.com/Byron/google-apis-rs/commit/7ed9528dd24e993476e7a4487de7f35e996528c7))
    - Release google-cloudbuild1-cli v3.1.0+20220218 ([`49bf5ea`](https://github.com/Byron/google-apis-rs/commit/49bf5ea598d49cfac0690878544239f742651fd2))
    - Release google-cloudbilling1-cli v3.1.0+20220305 ([`04b1732`](https://github.com/Byron/google-apis-rs/commit/04b17328e74ce50d23899b13dcd7aa91a76d062b))
    - Release google-cloudasset1_beta1-cli v3.1.0+20220225 ([`62a0f9a`](https://github.com/Byron/google-apis-rs/commit/62a0f9a18cac7a27f576d613ccf4d3ee9b057563))
    - Release google-cloudasset1-cli v3.1.0+20220225 ([`2095af9`](https://github.com/Byron/google-apis-rs/commit/2095af92636fa089ee87e561cbf92f28b5caa348))
    - Release google-classroom1-cli v3.1.0+20220224 ([`75a9b1b`](https://github.com/Byron/google-apis-rs/commit/75a9b1b4b4ff464d01dcaa00a0affc8ebd57b7c9))
    - Release google-chromeuxreport1-cli v3.1.0+20220302 ([`6a0001b`](https://github.com/Byron/google-apis-rs/commit/6a0001b9658a6b129df768b8acb06a46ce3dc5aa))
    - Release google-chromepolicy1-cli v3.1.0+20220305 ([`49d1c09`](https://github.com/Byron/google-apis-rs/commit/49d1c09ec5194586efe81eb5582111fd111019ad))
    - Release google-chromemanagement1-cli v3.1.0+20220305 ([`fa1fe02`](https://github.com/Byron/google-apis-rs/commit/fa1fe02d5fecc2f78b29ede8487504b567d6e94f))
    - Release google-certificatemanager1-cli v3.1.0+20220214 ([`2d2b095`](https://github.com/Byron/google-apis-rs/commit/2d2b095a6a63fc17bbe87f70b74d162fdcd43eee))
    - Release google-calendar3-cli v3.1.0+20220217 ([`7211431`](https://github.com/Byron/google-apis-rs/commit/721143123d2814f188f05b285556139a2114eca2))
    - Release google-books1-cli v3.1.0+20220301 ([`818c845`](https://github.com/Byron/google-apis-rs/commit/818c845943bb9c67a1bae8364fdf0cb89b3b9dd0))
    - Release google-blogger3-cli v3.1.0+20220305 ([`e51d342`](https://github.com/Byron/google-apis-rs/commit/e51d3423c923cc3a3d59c552b8c8b076c95c8b91))
    - Release google-binaryauthorization1_beta1-cli v3.1.0+20220225 ([`453185b`](https://github.com/Byron/google-apis-rs/commit/453185bbf66d9f283687a49bf6087fbe27cf8c09))
    - Release google-binaryauthorization1-cli v3.1.0+20220225 ([`1e53920`](https://github.com/Byron/google-apis-rs/commit/1e53920185e2de593ebffa207457803bf23bd8f6))
    - Release google-billingbudgets1_beta1-cli v3.1.0+20220227 ([`c270ac8`](https://github.com/Byron/google-apis-rs/commit/c270ac85cf56fdca00bb298e8f19732776169da5))
    - Release google-billingbudgets1-cli v3.1.0+20220227 ([`cd9751c`](https://github.com/Byron/google-apis-rs/commit/cd9751c1c6459dd26fc0887798d583396a531096))
    - Release google-bigtableadmin2-cli v3.1.0+20220222 ([`31ad894`](https://github.com/Byron/google-apis-rs/commit/31ad89454ad1b398fc856392ad2153919b4f754e))
    - Release google-bigqueryreservation1-cli v3.1.0+20220226 ([`1ea267b`](https://github.com/Byron/google-apis-rs/commit/1ea267b3d3e06c3daa2c7dac9c2ace90b58d3125))
    - Release google-bigquerydatatransfer1-cli v3.1.0+20220225 ([`c6abf78`](https://github.com/Byron/google-apis-rs/commit/c6abf78a2310eff4d293ad07e56343b95dbd2dfd))
    - Release google-bigqueryconnection1_beta1-cli v3.1.0+20220226 ([`778eeeb`](https://github.com/Byron/google-apis-rs/commit/778eeeb5c96932d933215eb27f095d79f73a2a49))
    - Release google-bigquery2-cli v3.1.0+20220222 ([`11751b1`](https://github.com/Byron/google-apis-rs/commit/11751b1cc957221fa1f5b46e5330084e58125127))
    - Release google-baremetalsolution2-cli v3.1.0+20220209 ([`4a1017b`](https://github.com/Byron/google-apis-rs/commit/4a1017bebfd94b45ab82c8084782f733f67f91a5))
    - Release google-autoscaler1_beta2-cli v3.1.0+20150629 ([`4bdfd50`](https://github.com/Byron/google-apis-rs/commit/4bdfd50662e84c11b7fbd4ac735ebf9645f28256))
    - Release google-authorizedbuyersmarketplace1-cli v3.1.0+20220307 ([`a422619`](https://github.com/Byron/google-apis-rs/commit/a422619f3be5ee24abe530c3e990d35b20ef7334))
    - Release google-assuredworkloads1-cli v3.1.0+20220224 ([`882ac63`](https://github.com/Byron/google-apis-rs/commit/882ac636a3b89b7973f1bdd7ee50621488868448))
    - Release google-artifactregistry1_beta1-cli v3.1.0+20220225 ([`dbdd194`](https://github.com/Byron/google-apis-rs/commit/dbdd194dbedfe4d5696818422b88c9fb4e2a5bb4))
    - Release google-artifactregistry1-cli v3.1.0+20220225 ([`704c1b3`](https://github.com/Byron/google-apis-rs/commit/704c1b33792b913168fa957714eede2661b26a27))
    - Release google-area120tables1_alpha1-cli v3.1.0+20220301 ([`cedc357`](https://github.com/Byron/google-apis-rs/commit/cedc357971e7a2c9a0bc4bdcb72fed5235ad9c23))
    - Release google-appstate1-cli v3.1.0+20190627 ([`cc2c733`](https://github.com/Byron/google-apis-rs/commit/cc2c73339bc25327a5ce979051c0a444e3775645))
    - Release google-appsactivity1-cli v3.1.0+20200628 ([`e2a6a92`](https://github.com/Byron/google-apis-rs/commit/e2a6a92588a5746201e79ae2ce0edd7a14464c51))
    - Release google-appengine1_beta5-cli v3.1.0+20181005 ([`201ed02`](https://github.com/Byron/google-apis-rs/commit/201ed0265d801619195badd82eb0dccea402fca1))
    - Release google-appengine1_beta4-cli v3.1.0+20181005 ([`8aafb88`](https://github.com/Byron/google-apis-rs/commit/8aafb884536a7cce4fc84f35d4ac950a5b95dc62))
    - Release google-appengine1-cli v3.1.0+20220226 ([`c7bd5ca`](https://github.com/Byron/google-apis-rs/commit/c7bd5ca817a0bca50f0ebc7ae2d8c0f272efe9d9))
    - Release google-apikeys2-cli v3.1.0+20220305 ([`30fdee7`](https://github.com/Byron/google-apis-rs/commit/30fdee7085a1d86985ff34e51c3a3831d68fb07e))
    - Release google-apigee1-cli v3.1.0+20220301 ([`de63c56`](https://github.com/Byron/google-apis-rs/commit/de63c5624266251e76e288e52b15bd44bed125a8))
    - Release google-apigateway1-cli v3.1.0+20220223 ([`5728655`](https://github.com/Byron/google-apis-rs/commit/5728655e62fa5605fa89843044b522fda9b6b9b3))
    - Release google-androidpublisher3-cli v3.1.0+20220307 ([`7a49cfe`](https://github.com/Byron/google-apis-rs/commit/7a49cfec537282697b4f477afc93df8dc95db0ce))
    - Release google-androidpublisher2-cli v3.1.0+20200331 ([`4af2858`](https://github.com/Byron/google-apis-rs/commit/4af2858077dcbc33b045d0efd35af3726b7757bf))
    - Release google-androidmanagement1-cli v3.1.0+20220302 ([`a540884`](https://github.com/Byron/google-apis-rs/commit/a540884e1e1adfecc9b4c03f3653ce31bc8b20e3))
    - Release google-androidenterprise1-cli v3.1.0+20220303 ([`1200c63`](https://github.com/Byron/google-apis-rs/commit/1200c6301981cc42e21298b345d1d55b6797363f))
    - Release google-androiddeviceprovisioning1-cli v3.1.0+20220305 ([`ed71b23`](https://github.com/Byron/google-apis-rs/commit/ed71b234d150ade1a7a6d4c62875ee4f9415f038))
    - Release google-analyticsreporting4-cli v3.1.0+20220215 ([`93b3968`](https://github.com/Byron/google-apis-rs/commit/93b39683ba97eb52d8c9a3ac8a25ab697fb3104f))
    - Release google-analyticsdata1_beta-cli v3.1.0+20220303 ([`9714faf`](https://github.com/Byron/google-apis-rs/commit/9714fafe272e8bf3185265a134ffac0b5bf0628f))
    - Release google-analyticsadmin1_alpha-cli v3.1.0+20220307 ([`86b29b2`](https://github.com/Byron/google-apis-rs/commit/86b29b2ce6415e8476f50334938e8d07f2aef39f))
    - Release google-analytics3-cli v3.1.0+20190807 ([`dbaf9fe`](https://github.com/Byron/google-apis-rs/commit/dbaf9fe3d6ad7a20822285e5f1ddfc540754172d))
    - Release google-alertcenter1_beta1-cli v3.1.0+20220221 ([`5ee8eff`](https://github.com/Byron/google-apis-rs/commit/5ee8eff05d83d3aa563b76b80afe665368ffa9bb))
    - Release google-adsensehost4d1-cli v3.1.0+20200930 ([`71ac27d`](https://github.com/Byron/google-apis-rs/commit/71ac27dd97da9eaecbbdde0f90cb3e308b762bc4))
    - Release google-adsense2-cli v3.1.0+20220304 ([`79f4267`](https://github.com/Byron/google-apis-rs/commit/79f4267468146fc076a68eea2546d41c56e99ed3))
    - Release google-adsense1d4-cli v3.1.0+20201002 ([`92bdbe5`](https://github.com/Byron/google-apis-rs/commit/92bdbe59ea0799eafdb3ae31d28493bae1d3b19a))
    - Release google-admob1-cli v3.1.0+20220303 ([`cb842db`](https://github.com/Byron/google-apis-rs/commit/cb842dbf101a85d87b6ed2fd2fa6cc81d95abd06))
    - Release google-adexperiencereport1-cli v3.1.0+20220303 ([`2aaa6b7`](https://github.com/Byron/google-apis-rs/commit/2aaa6b7b9f1bff777a6b22fcf2fd538be5a2104d))
    - Release google-adexchangeseller2-cli v3.1.0+20171101 ([`d9143df`](https://github.com/Byron/google-apis-rs/commit/d9143dff2ba02ccdfbbba52c694d4f52147113ad))
    - Release google-adexchangebuyer2_v2_beta1-cli v3.1.0+20220307 ([`764dabe`](https://github.com/Byron/google-apis-rs/commit/764dabeb2dd31f306f35586ac786d470d8cef6fe))
    - Release google-adexchangebuyer1d4-cli v3.1.0+20210330 ([`14e3007`](https://github.com/Byron/google-apis-rs/commit/14e30070490ecb5214ff6fab01e5aa8269a6b3fe))
    - Release google-adexchangebuyer1d3-cli v3.1.0+20210330 ([`e9d64f8`](https://github.com/Byron/google-apis-rs/commit/e9d64f81ad7207a3303cfa0f1a8eaf00d578d2be))
    - Release google-accesscontextmanager1_beta-cli v3.1.0+20220301 ([`9e9c986`](https://github.com/Byron/google-apis-rs/commit/9e9c986ae25ba7f90febc6b7cdb59c03d4041cec))
    - Release google-accesscontextmanager1-cli v3.1.0+20220301 ([`7b4fe69`](https://github.com/Byron/google-apis-rs/commit/7b4fe6960af8622fb31d00c26539ee0e4b7a050e))
    - Release google-accessapproval1_beta1-cli v3.1.0+20200708 ([`b761aeb`](https://github.com/Byron/google-apis-rs/commit/b761aeb7615922bb4748d85c4169e1793de87fdf))
    - Release google-accessapproval1-cli v3.1.0+20220225 ([`0a4a3ab`](https://github.com/Byron/google-apis-rs/commit/0a4a3ab876cbac5f7bde1508d08315bddc23051c))
    - Release google-acceleratedmobilepageurl1-cli v3.1.0+20220305 ([`eaa6153`](https://github.com/Byron/google-apis-rs/commit/eaa615346f30ae47edd501134b1aaac8b00765fc))
    - Release google-abusiveexperiencereport1-cli v3.1.0+20220303 ([`46c223f`](https://github.com/Byron/google-apis-rs/commit/46c223f113745cce9d08f27364694aa15f1e4b70))
    - Release google-youtubereporting1 v3.1.0+20220305 ([`52b653d`](https://github.com/Byron/google-apis-rs/commit/52b653d1f975ad3ef6412bb7629b764798fa29af))
    - Release google-workflows1 v3.1.0+20220223 ([`fd8b290`](https://github.com/Byron/google-apis-rs/commit/fd8b2909c4c739f3d30383775f2695ff75202c81))
    - Release google-workflowexecutions1 v3.1.0+20220222 ([`95249ea`](https://github.com/Byron/google-apis-rs/commit/95249ea976d5b15ef72a91045d48153b94a15402))
    - Release google-webrisk1 v3.1.0+20220226 ([`6523405`](https://github.com/Byron/google-apis-rs/commit/6523405405219f972de0c24c7841dd0937ab2fe4))
    - Release google-webmasters3 v3.1.0+20190428 ([`4a0b08b`](https://github.com/Byron/google-apis-rs/commit/4a0b08b4f89c08c718cd22e34eac1213d53b7219))
    - Release google-webfonts1 v3.1.0+20220215 ([`667dba7`](https://github.com/Byron/google-apis-rs/commit/667dba722cf5101d485e56f9cd70c326a10cc264))
    - Release google-vmmigration1 v3.1.0+20220225 ([`2adfa61`](https://github.com/Byron/google-apis-rs/commit/2adfa61a246838d9f13ee05f73c8cb86af5365a3))
    - Release google-vision1 v3.1.0+20220225 ([`ae6bb41`](https://github.com/Byron/google-apis-rs/commit/ae6bb41f0016aab43363ec61fb691e03c44e31f5))
    - Release google-videointelligence1_beta1 v3.1.0+20171122 ([`49d00d9`](https://github.com/Byron/google-apis-rs/commit/49d00d96e6ba3b9c46fb2136cb488117c6efa95d))
    - Release google-videointelligence1 v3.1.0+20220225 ([`b34b319`](https://github.com/Byron/google-apis-rs/commit/b34b319cb3c5473e7f8c40773a118da4f14eaaa3))
    - Release google-versionhistory1 v3.1.0+20220307 ([`060a6fd`](https://github.com/Byron/google-apis-rs/commit/060a6fd73984315d085e9d792509df8aacb10242))
    - Release google-verifiedaccess1 v3.1.0+20220215 ([`2574ace`](https://github.com/Byron/google-apis-rs/commit/2574ace617d0e88833adf55a1b6e3f4a27d89e26))
    - Release google-vectortile1 v3.1.0+20210331 ([`bfc5579`](https://github.com/Byron/google-apis-rs/commit/bfc557954edc6f62d4a0b13bd56c893cd921067c))
    - Release google-vault1 v3.1.0+20220222 ([`e7a0c0d`](https://github.com/Byron/google-apis-rs/commit/e7a0c0da0cffe939d7a56f7fc6526fe2594a3ae3))
    - Release google-urlshortener1 v3.1.0+20150519 ([`1478b55`](https://github.com/Byron/google-apis-rs/commit/1478b55b5bf32329b8123bc27855647b04f69297))
    - Release google-translate3 v3.1.0+20220121 ([`c0974c4`](https://github.com/Byron/google-apis-rs/commit/c0974c4465efc8816d14c195ad2978e1892c86ef))
    - Release google-translate2 v3.1.0+20170525 ([`c63ebb7`](https://github.com/Byron/google-apis-rs/commit/c63ebb7d50b9953e83833c8993e2dcc31ff9cea1))
    - Release google-transcoder1 v3.1.0+20220201 ([`34ba786`](https://github.com/Byron/google-apis-rs/commit/34ba78691fb69fbb81e2ce3dcacba55a01922458))
    - Release google-transcoder1_beta1 v3.1.0+20210323 ([`04db803`](https://github.com/Byron/google-apis-rs/commit/04db803e21830dc2dfbaad979a5a63de66efe4f1))
    - Release google-tpu1_alpha1 v3.1.0+20220301 ([`bb985a7`](https://github.com/Byron/google-apis-rs/commit/bb985a709da8dde5ea833b1ee518c0552ceed84d))
    - Release google-tpu1 v3.1.0+20220301 ([`0b18729`](https://github.com/Byron/google-apis-rs/commit/0b187291da0463dc2662e353accc120229a17994))
    - Release google-testing1 v3.1.0+20220301 ([`e3f825e`](https://github.com/Byron/google-apis-rs/commit/e3f825e75c6b959aae5e8699dfe24640673259cb))
    - Release google-tasks1 v3.1.0+20220305 ([`92bf24a`](https://github.com/Byron/google-apis-rs/commit/92bf24a2f30580209da749a9efc43f47d55d189c))
    - Release google-taskqueue1_beta2 v3.1.0+20160428 ([`895989d`](https://github.com/Byron/google-apis-rs/commit/895989d38371173af965e6d516e225087aab1ce9))
    - Release google-tagmanager2 v3.1.0+20220301 ([`e6dc9f4`](https://github.com/Byron/google-apis-rs/commit/e6dc9f4d487cadb5a8e8724d9458cd5149ae6df7))
    - Release google-tagmanager1 v3.1.0+20220301 ([`14baf81`](https://github.com/Byron/google-apis-rs/commit/14baf8141a001050f0b427a5220b28675e67b98a))
    - Release google-surveys2 v3.1.0+20180508 ([`33ea099`](https://github.com/Byron/google-apis-rs/commit/33ea09911db9ca117696300f91342afdbb12833b))
    - Release google-sts1 v3.1.0+20220227 ([`6316311`](https://github.com/Byron/google-apis-rs/commit/631631121477005bf6c6ebc6fc4bfccc9422c276))
    - Release google-storagetransfer1 v3.1.0+20220223 ([`682dd5e`](https://github.com/Byron/google-apis-rs/commit/682dd5e3dd02d55770fb9dd075ffffebff6a89a0))
    - Release google-storage1 v3.1.0+20220228 ([`23813b3`](https://github.com/Byron/google-apis-rs/commit/23813b346a912a1c2646698d1e116e53fadf9b60))
    - Release google-sqladmin1 v3.1.0+20220226 ([`dce4a54`](https://github.com/Byron/google-apis-rs/commit/dce4a54d7dbc3b86614d301d0bae913e6282779a))
    - Release google-sqladmin1_beta4 v3.1.0+20220226 ([`e63c50e`](https://github.com/Byron/google-apis-rs/commit/e63c50e0ef7febf23743b8c5e646b8049cf18c4b))
    - Release google-sql1_beta4 v3.1.0+20200331 ([`20d1cc0`](https://github.com/Byron/google-apis-rs/commit/20d1cc0ae65b83730af652c4a5332a3998bb95da))
    - Release google-speech1_beta1 v3.1.0+20181005 ([`aec17d7`](https://github.com/Byron/google-apis-rs/commit/aec17d700b4c904c4020a17341777996aadcc6e9))
    - Release google-speech1 v3.1.0+20220221 ([`841f3b5`](https://github.com/Byron/google-apis-rs/commit/841f3b5ef2af96d3f69022f622273e104fc207cd))
    - Release google-spectrum1_explorer v3.1.0+20170306 ([`523e373`](https://github.com/Byron/google-apis-rs/commit/523e373340691e7fab783f12fb125dfa02c15097))
    - Release google-sourcerepo1 v3.1.0+20220217 ([`0b8e0e4`](https://github.com/Byron/google-apis-rs/commit/0b8e0e412b02ddba15b77480a116ed04cf8ce17a))
    - Release google-smartdevicemanagement1 v3.1.0+20220302 ([`cb363e1`](https://github.com/Byron/google-apis-rs/commit/cb363e1ad7a27b0a1dd5cfdb730dc49963b97f02))
    - Release google-siteverification1 v3.1.0+20191119 ([`4bc5098`](https://github.com/Byron/google-apis-rs/commit/4bc5098b0660c25e96048bb1abb6e3ab2a7019a2))
    - Release google-serviceregistryalpha v3.1.0+20160401 ([`d6d05e8`](https://github.com/Byron/google-apis-rs/commit/d6d05e8fbdc1c28055adb39d2e23cf8009f44d10))
    - Release google-servicedirectory1_beta1 v3.1.0+20220224 ([`d49a05a`](https://github.com/Byron/google-apis-rs/commit/d49a05ad494a3c22d65bb17ec988c8b5da2b872a))
    - Release google-servicedirectory1 v3.1.0+20220224 ([`91052c0`](https://github.com/Byron/google-apis-rs/commit/91052c0fe2ba44d3505051dad037baf755944a8b))
    - Release google-servicecontrol2 v3.1.0+20220227 ([`5831809`](https://github.com/Byron/google-apis-rs/commit/5831809bd0fc2709be719f66c78551d03c1c7afa))
    - Release google-servicecontrol1 v3.1.0+20220227 ([`2e90459`](https://github.com/Byron/google-apis-rs/commit/2e90459cf0e48f4570db6f7bf26ebe28aafddc7d))
    - Release google-servicebroker1 v3.1.0+20190624 ([`25541c0`](https://github.com/Byron/google-apis-rs/commit/25541c025fe8c6eb42782e71ae1d1cdb56db35cd))
    - Release google-securitycenter1 v3.1.0+20220224 ([`a7384ef`](https://github.com/Byron/google-apis-rs/commit/a7384efbd99b896f0a838d0aee9f7dfb229af258))
    - Release google-secretmanager1_beta1 v3.1.0+20220226 ([`5783519`](https://github.com/Byron/google-apis-rs/commit/578351976d0186ccf0ec62af1a1a68318b80fc04))
    - Release google-secretmanager1 v3.1.0+20220226 ([`1b291bc`](https://github.com/Byron/google-apis-rs/commit/1b291bcca3108b8ad2dc0b7dbe134d615cb92920))
    - Release google-searchconsole1 v3.1.0+20220305 ([`ab0f063`](https://github.com/Byron/google-apis-rs/commit/ab0f063d13c7787c1ccc61e0e5c55a0050e373da))
    - Release google-sasportal1_alpha1 v3.1.0+20220301 ([`bc13b61`](https://github.com/Byron/google-apis-rs/commit/bc13b61b0b70fc6c27019642e0bff8c8b36ec32c))
    - Release google-safebrowsing4 v3.1.0+20220305 ([`67649a3`](https://github.com/Byron/google-apis-rs/commit/67649a370dcb7e82974c2b2a64ae2e2ad0742e2b))
    - Release google-runtimeconfig1_beta1 v3.1.0+20220228 ([`5d4c0f6`](https://github.com/Byron/google-apis-rs/commit/5d4c0f66652cb117be2c7c802f485646ca72326e))
    - Release google-runtimeconfig1 v3.1.0+20220228 ([`4973361`](https://github.com/Byron/google-apis-rs/commit/49733618681735cf98e776b7464dcab908f9d573))
    - Release google-run2 v3.1.0+20220225 ([`d19f5c5`](https://github.com/Byron/google-apis-rs/commit/d19f5c58d90773edf5b7f68e30404faa4842645c))
    - Release google-run1 v3.1.0+20220225 ([`7ae675f`](https://github.com/Byron/google-apis-rs/commit/7ae675f575a9b008a10fdd4614b62f1c25e65b8d))
    - Release google-retail2 v3.1.0+20220224 ([`215582b`](https://github.com/Byron/google-apis-rs/commit/215582bc91ab373708bba17e2768d4075402b5b7))
    - Release google-resourceviews1_beta2 v3.1.0+20160512 ([`c6b60d9`](https://github.com/Byron/google-apis-rs/commit/c6b60d9cad1e587583b7276a78082238a628193b))
    - Release google-resourcesettings1 v3.1.0+20220305 ([`105279e`](https://github.com/Byron/google-apis-rs/commit/105279ebf3c0caae1835f3a41ba3e999d9b74daa))
    - Release google-reseller1_sandbox v3.1.0+20160329 ([`bd700d7`](https://github.com/Byron/google-apis-rs/commit/bd700d7db47c817e0e504269eea4037e81e78d96))
    - Release google-replicapoolupdater1_beta1 v3.1.0+20161003 ([`377ef2c`](https://github.com/Byron/google-apis-rs/commit/377ef2c97a6b732594a706824d23fe13b3308300))
    - Release google-replicapool1_beta2 v3.1.0+20160512 ([`4e341c7`](https://github.com/Byron/google-apis-rs/commit/4e341c73a04e2428efffba0405ddb8fed298db69))
    - Release google-remotebuildexecution2 v3.1.0+20210329 ([`70d5b8a`](https://github.com/Byron/google-apis-rs/commit/70d5b8a02483c8731a86ee0d0e3d391a01672aef))
    - Release google-redis1 v3.1.0+20220301 ([`721e7c4`](https://github.com/Byron/google-apis-rs/commit/721e7c4c002e819bd6de2d04ab5bf71efb083068))
    - Release google-recommender1_beta1 v3.1.0+20220228 ([`4bc3a02`](https://github.com/Byron/google-apis-rs/commit/4bc3a021f19c34fbb25609c532d27153beda1abb))
    - Release google-recommender1 v3.1.0+20220228 ([`1af9c88`](https://github.com/Byron/google-apis-rs/commit/1af9c88e5ab91a2246025c88260305ae432b06c0))
    - Release google-recommendationengine1_beta1 v3.1.0+20220224 ([`c52c21d`](https://github.com/Byron/google-apis-rs/commit/c52c21da70c71e9fc9964dfdfd1f6d2219b11527))
    - Release google-recaptchaenterprise1 v3.1.0+20220226 ([`01d2497`](https://github.com/Byron/google-apis-rs/commit/01d2497a994bfa2ff3a4dc3e2880184f6d907a67))
    - Release google-realtimebidding1 v3.1.0+20220307 ([`e9f0626`](https://github.com/Byron/google-apis-rs/commit/e9f0626abc8b18ee46102187a83de282647c9dae))
    - Release google-qpxexpress1 v3.1.0+20160708 ([`b4d0c50`](https://github.com/Byron/google-apis-rs/commit/b4d0c500128a069864e601fc3af876701c2dae89))
    - Release google-pubsublite1 v3.1.0+20220301 ([`f77cc43`](https://github.com/Byron/google-apis-rs/commit/f77cc43e25ece68d39aa45d9902d40cdf002b819))
    - Release google-pubsub1_beta2 v3.1.0+20220221 ([`1cd85cf`](https://github.com/Byron/google-apis-rs/commit/1cd85cfcd9e1f81c586fb0c9593b6c26135a015d))
    - Release google-pubsub1 v3.1.0+20220221 ([`2b25813`](https://github.com/Byron/google-apis-rs/commit/2b25813f79a0cc4d3f0ccb989f746668b0894e1a))
    - Release google-proximitybeacon1_beta1 v3.1.0+20200127 ([`763a896`](https://github.com/Byron/google-apis-rs/commit/763a896c3691b7b59f7998b353113af2c4b91b6f))
    - Release google-prod_tt_sasportal1_alpha1 v3.1.0+20220303 ([`028b27d`](https://github.com/Byron/google-apis-rs/commit/028b27d305f46564f386f759dcab392de4449d3f))
    - Release google-privateca1 v3.1.0+20220209 ([`1fd06f1`](https://github.com/Byron/google-apis-rs/commit/1fd06f1bf7687638c2662be0b0fa9b2fda8882ff))
    - Release google-privateca1_beta1 v3.1.0+20220209 ([`77d1e4c`](https://github.com/Byron/google-apis-rs/commit/77d1e4c415fa32e82bf56a7ba246c54a3fdc80df))
    - Release google-prediction1d6 v3.1.0+20160511 ([`9aae85c`](https://github.com/Byron/google-apis-rs/commit/9aae85c0d2b4129a992b53db3a1d4cc137706c7c))
    - Release google-policytroubleshooter1 v3.1.0+20220227 ([`456a1bc`](https://github.com/Byron/google-apis-rs/commit/456a1bccf3ce1c43c01803adc54bfab19f562e84))
    - Release google-policysimulator1 v3.1.0+20220227 ([`e4171ba`](https://github.com/Byron/google-apis-rs/commit/e4171ba41843a2214466c5fd470319b99f0af261))
    - Release google-policyanalyzer1 v3.1.0+20220227 ([`9f64d74`](https://github.com/Byron/google-apis-rs/commit/9f64d74bc7aeb0c5663369a4d2774fb386db0d4f))
    - Release google-plusdomains1 v3.1.0+20190616 ([`a392185`](https://github.com/Byron/google-apis-rs/commit/a39218558580e28e6beab95cc0323b53f47855f8))
    - Release google-plus1 v3.1.0+20190616 ([`5858d0d`](https://github.com/Byron/google-apis-rs/commit/5858d0d1d8dfb57698e152d4f6ebf747f8d30b01))
    - Release google-playmoviespartner1 v3.1.0+20170919 ([`d2f3447`](https://github.com/Byron/google-apis-rs/commit/d2f3447d630c93b028fbb4ec7f6e76cffb6f53d2))
    - Release google-playintegrity1 v3.1.0+20220305 ([`15ca475`](https://github.com/Byron/google-apis-rs/commit/15ca4757c3a94a7b7914af0783b8df4cf6b51806))
    - Release google-playcustomapp1 v3.1.0+20220305 ([`b4475c9`](https://github.com/Byron/google-apis-rs/commit/b4475c9b7df853a0da10738b099941ab9bcc6279))
    - Release google-playablelocations3 v3.1.0+20200707 ([`1fdb8bf`](https://github.com/Byron/google-apis-rs/commit/1fdb8bfe9ba1041eb9af8b5b3d104e00a0a207be))
    - Release google-photoslibrary1 v3.1.0+20220303 ([`c728819`](https://github.com/Byron/google-apis-rs/commit/c7288195615c9b5e124908bf40c896e4827a0814))
    - Release google-people1 v3.1.0+20220303 ([`beb6f41`](https://github.com/Byron/google-apis-rs/commit/beb6f41573c0073f9bc284e7d0eff232e17f107f))
    - Release google-paymentsresellersubscription1 v3.1.0+20220307 ([`601c8f5`](https://github.com/Byron/google-apis-rs/commit/601c8f5b7550d2a661ef23bdf5dbd310673f223f))
    - Release google-partners2 v3.1.0+20180925 ([`e2fe706`](https://github.com/Byron/google-apis-rs/commit/e2fe706169c67234c57cc26317c4dd1c8b0becc5))
    - Release google-pagespeedonline5 v3.1.0+20220302 ([`4e976b9`](https://github.com/Byron/google-apis-rs/commit/4e976b93a75b6d0722924a1c66b596eadf9299a6))
    - Release google-pagespeedonline4 v3.1.0+20191206 ([`74028ac`](https://github.com/Byron/google-apis-rs/commit/74028ac41403958d5503cb72d522e0722d2e5131))
    - Release google-pagespeedonline2 v3.1.0+20191206 ([`a955ab9`](https://github.com/Byron/google-apis-rs/commit/a955ab9f6f6b6a913c905e0d1ef6394417f42c12))
    - Release google-oslogin1_beta v3.1.0+20220228 ([`73bfb9d`](https://github.com/Byron/google-apis-rs/commit/73bfb9d7f4237a756d466ea52dd2dc7e24275c87))
    - Release google-oslogin1 v3.1.0+20220228 ([`dad4c8f`](https://github.com/Byron/google-apis-rs/commit/dad4c8f5d6a7cf4dc89b148f768d262837f6b17a))
    - Release google-orgpolicy2 v3.1.0+20220305 ([`efc2be2`](https://github.com/Byron/google-apis-rs/commit/efc2be27c65a746589a8a0eca2f703a03d6aee62))
    - Release google-ondemandscanning1 v3.1.0+20220228 ([`dd33358`](https://github.com/Byron/google-apis-rs/commit/dd33358c8458b3eaa00b21e1cb85c330414d7cd4))
    - Release google-notebooks1 v3.1.0+20220224 ([`eaf8709`](https://github.com/Byron/google-apis-rs/commit/eaf8709744cc502aa30a7da3a2f075ee8d3662f5))
    - Release google-networkservices1 v3.1.0+20220222 ([`8a91b71`](https://github.com/Byron/google-apis-rs/commit/8a91b71b97381eec7b83c3843b33cbe209f33a33))
    - Release google-networksecurity1 v3.1.0+20220223 ([`4d583d8`](https://github.com/Byron/google-apis-rs/commit/4d583d8adaac0abe65341af6fc6eb0f6e4db27c8))
    - Release google-networkmanagement1 v3.1.0+20220223 ([`9cf58de`](https://github.com/Byron/google-apis-rs/commit/9cf58dee0842ff1d07fa5374004fa4ef576e1f82))
    - Release google-networkconnectivity1 v3.1.0+20220210 ([`2a743c2`](https://github.com/Byron/google-apis-rs/commit/2a743c22c94b5275e5babfd3feda7177210c67cd))
    - Release google-networkconnectivity1_alpha1 v3.1.0+20220210 ([`35e8255`](https://github.com/Byron/google-apis-rs/commit/35e8255bdb236138a252dbe7b54dc7d037cd4433))
    - Release google-mybusinessverifications1 v3.1.0+20220305 ([`71dcd81`](https://github.com/Byron/google-apis-rs/commit/71dcd81e7d1420456a80ee860568e732ab12b168))
    - Release google-mybusinessplaceactions1 v3.1.0+20220305 ([`713d22b`](https://github.com/Byron/google-apis-rs/commit/713d22b6ffea241503ef6caef43a2eeb9269431b))
    - Release google-mybusinessnotifications1 v3.1.0+20220305 ([`3294f57`](https://github.com/Byron/google-apis-rs/commit/3294f57211079b90f0ebc389834bd64ef2e9743f))
    - Release google-mybusinesslodging1 v3.1.0+20220305 ([`050d805`](https://github.com/Byron/google-apis-rs/commit/050d805cf9daf90d66b03332cdf98e7232a393c8))
    - Release google-mybusinessbusinessinformation1 v3.1.0+20220305 ([`4a4fef3`](https://github.com/Byron/google-apis-rs/commit/4a4fef383dd83bb6dc7a6f2347e37be039854ade))
    - Release google-mybusinessbusinesscalls1 v3.1.0+20220305 ([`e3ccb43`](https://github.com/Byron/google-apis-rs/commit/e3ccb43c359be89621ed6481f9b5161e60f2d7e5))
    - Release google-mybusinessaccountmanagement1 v3.1.0+20220305 ([`81b14e0`](https://github.com/Byron/google-apis-rs/commit/81b14e0b0a0c612968c9a14ed67558f15e492aab))
    - Release google-mybusiness4 v3.1.0+0 ([`0707804`](https://github.com/Byron/google-apis-rs/commit/070780415cb6fc843d9c8f7cbea641b9363a3436))
    - Release google-monitoring3 v3.1.0+20220218 ([`f68e7d1`](https://github.com/Byron/google-apis-rs/commit/f68e7d1e5f32a7f6be217fed67aa91089cae96f2))
    - Release google-ml1 v3.1.0+20220212 ([`ae81350`](https://github.com/Byron/google-apis-rs/commit/ae8135031dc709c32e150eb1dbf4c875f68c95e2))
    - Release google-mirror1 v3.1.0+20190424 ([`c5d745f`](https://github.com/Byron/google-apis-rs/commit/c5d745fc1e7564178000b63a90f722cd29f40689))
    - Release google-metastore1_beta v3.1.0+20220222 ([`f1d0465`](https://github.com/Byron/google-apis-rs/commit/f1d046516409a5a669798c61d2112eb49acb2470))
    - Release google-memcache1_beta2 v3.1.0+20220224 ([`90aa96d`](https://github.com/Byron/google-apis-rs/commit/90aa96db8d68e13cc48e8b3e555bf72a9dfccb06))
    - Release google-memcache1 v3.1.0+20220224 ([`39d7b3f`](https://github.com/Byron/google-apis-rs/commit/39d7b3f02214d55dfe621fa82390865b1b1b079f))
    - Release google-manufacturers1 v3.1.0+20220303 ([`d49a2b4`](https://github.com/Byron/google-apis-rs/commit/d49a2b4f0d546e9a416be091c0298f7260528131))
    - Release google-manager1_beta2 v3.1.0+20140915 ([`df0630d`](https://github.com/Byron/google-apis-rs/commit/df0630d9a1c7d391e953eba43cb4439a7874f503))
    - Release google-managedidentities1 v3.1.0+20220216 ([`dcb1ff1`](https://github.com/Byron/google-apis-rs/commit/dcb1ff1a570436aab2fb521a9a9268586d7ba376))
    - Release google-logging2_beta1 v3.1.0+20190325 ([`1fc1ce6`](https://github.com/Byron/google-apis-rs/commit/1fc1ce638d43e781efc8da525731e70828cd190c))
    - Release google-logging2 v3.1.0+20220225 ([`7c19d5e`](https://github.com/Byron/google-apis-rs/commit/7c19d5ef06c31182aeadde8f67af77147621904f))
    - Release google-localservices1 v3.1.0+20220305 ([`b9a87d8`](https://github.com/Byron/google-apis-rs/commit/b9a87d812f9af9f193cd2f64d65d024cc807c867))
    - Release google-lifesciences2_beta v3.1.0+20220211 ([`f4ef927`](https://github.com/Byron/google-apis-rs/commit/f4ef927d82d040d34971fdc7d7c92dfdc0841222))
    - Release google-licensing1 v3.1.0+20220305 ([`7a28b34`](https://github.com/Byron/google-apis-rs/commit/7a28b3403ca515b19ba7e5d822169577c9d0a8bf))
    - Release google-libraryagent1 v3.1.0+20220305 ([`a39f4d1`](https://github.com/Byron/google-apis-rs/commit/a39f4d1cc888f9bbdfd68e8b766c2241c19b3e6a))
    - Release google-language1_beta1 v3.1.0+20220218 ([`84f2327`](https://github.com/Byron/google-apis-rs/commit/84f23274ba72072310346fd3d77a3b4c37e4155c))
    - Release google-language1 v3.1.0+20220218 ([`f12de6e`](https://github.com/Byron/google-apis-rs/commit/f12de6e95a267cc936c5949448ec6a0b866ab629))
    - Release google-keep1 v3.1.0+20220301 ([`e421c56`](https://github.com/Byron/google-apis-rs/commit/e421c564a3f7d2560735f4ce0229b74821a8b7f9))
    - Release google-jobs4 v3.1.0+20220211 ([`f3fb7d5`](https://github.com/Byron/google-apis-rs/commit/f3fb7d5bf5150619c9d47fe18287665621490733))
    - Release google-jobs3 v3.1.0+20220211 ([`b62eff4`](https://github.com/Byron/google-apis-rs/commit/b62eff4f7776462225e8190dd6b82a805ed26caa))
    - Release google-indexing3 v3.1.0+20220126 ([`9fccecc`](https://github.com/Byron/google-apis-rs/commit/9fccecc3f370aae0cfb18303bba946d10b8a2ef4))
    - Release google-ids1 v3.1.0+20220221 ([`98bed3a`](https://github.com/Byron/google-apis-rs/commit/98bed3ab827066355a932102f11d08ce0a2fdb5a))
    - Release google-identitytoolkit3 v3.1.0+20180723 ([`68c5099`](https://github.com/Byron/google-apis-rs/commit/68c509963100dc7778fe8a46cc93edd873a5919a))
    - Release google-ideahub1_beta v3.1.0+20220305 ([`e644ed5`](https://github.com/Byron/google-apis-rs/commit/e644ed568cd38cbc7a115aa15369e08a1e602c32))
    - Release google-iap1_beta1 v3.1.0+20220225 ([`7c76aba`](https://github.com/Byron/google-apis-rs/commit/7c76aba1e81c4730d294906e296f41642dbb35b9))
    - Release google-iap1 v3.1.0+20220225 ([`eaa51f4`](https://github.com/Byron/google-apis-rs/commit/eaa51f4fe5ede91594e1cb5d4ead52399970f639))
    - Release google-iamcredentials1 v3.1.0+20220225 ([`6a01e60`](https://github.com/Byron/google-apis-rs/commit/6a01e60d857d064118cdbdeec07de4cc412f590a))
    - Release google-iam1 v3.1.0+20220224 ([`81e56d1`](https://github.com/Byron/google-apis-rs/commit/81e56d18ae0c398041aff81768352f6b7b45fc28))
    - Release google-healthcare1_beta1 v3.1.0+20220223 ([`2c1795d`](https://github.com/Byron/google-apis-rs/commit/2c1795da0de43987a203771d912ccbbbf7b337c9))
    - Release google-healthcare1 v3.1.0+20220223 ([`7afd89a`](https://github.com/Byron/google-apis-rs/commit/7afd89a54c7d96da0580b91a8fbcf8fa9ae72d49))
    - Release google-groupssettings1 v3.1.0+20220224 ([`267246b`](https://github.com/Byron/google-apis-rs/commit/267246b39b525b24367f29178e79db9fa64bcb24))
    - Release google-groupsmigration1 v3.1.0+20220226 ([`7d9b04e`](https://github.com/Byron/google-apis-rs/commit/7d9b04e2d32458b9576478bf4105f0903b93db53))
    - Release google-gmailpostmastertools1_beta1 v3.1.0+20220305 ([`8ddc466`](https://github.com/Byron/google-apis-rs/commit/8ddc466dc5dd9e88fc6f0a18bd75643f961891c9))
    - Release google-gmailpostmastertools1 v3.1.0+20220305 ([`90e05e5`](https://github.com/Byron/google-apis-rs/commit/90e05e5627bd8b3f35758c0c94f11856bca4bd5d))
    - Release google-gmail1 v3.1.0+20220228 ([`e2fd104`](https://github.com/Byron/google-apis-rs/commit/e2fd104d815131da8ae9f7e062060d7e31aa281c))
    - Release google-gkehub1 v3.1.0+20220211 ([`8a2712a`](https://github.com/Byron/google-apis-rs/commit/8a2712ade39bda965aff7e4aac814689cf733d1a))
    - Release google-genomics1 v3.1.0+20210324 ([`9ea7e9b`](https://github.com/Byron/google-apis-rs/commit/9ea7e9bdc6d519ef4a12811d3ae1f58aec44a15c))
    - Release google-gan1_beta1 v3.1.0+20130205 ([`7268326`](https://github.com/Byron/google-apis-rs/commit/7268326e8e1d1e21555cefa017c755842d7fbfbe))
    - Release google-gamesmanagement1_management v3.1.0+20220217 ([`073a0b2`](https://github.com/Byron/google-apis-rs/commit/073a0b23e2cd2d12774dbcc00da132f2afa9ce4c))
    - Release google-gameservices1 v3.1.0+20220223 ([`a2628ea`](https://github.com/Byron/google-apis-rs/commit/a2628ea809132bd3f9f3dd422f75befbb3edcbad))
    - Release google-gamesconfiguration1_configuration v3.1.0+20220217 ([`d4b05b2`](https://github.com/Byron/google-apis-rs/commit/d4b05b2c1f07e205b5bfc4ebd495a09b26bfd252))
    - Release google-games1 v3.1.0+20220217 ([`52ca373`](https://github.com/Byron/google-apis-rs/commit/52ca3734de13b9334268c59e224077676ddf24f3))
    - Release google-fusiontables2 v3.1.0+20171117 ([`779f0b3`](https://github.com/Byron/google-apis-rs/commit/779f0b3a7aacb14863d73eec783471e66d8484e4))
    - Release google-fitness1 v3.1.0+20220302 ([`f19e553`](https://github.com/Byron/google-apis-rs/commit/f19e5534dfc42f0e37577627c6a890c87150dd1f))
    - Release google-firestore1_beta1 v3.1.0+20220221 ([`b092840`](https://github.com/Byron/google-apis-rs/commit/b0928404fdff1d31d8bf8901e353907a6735736c))
    - Release google-firestore1 v3.1.0+20220221 ([`c422d45`](https://github.com/Byron/google-apis-rs/commit/c422d456b346f1671dccbaa21774b40eeb516ed9))
    - Release google-firebasestorage1_beta v3.1.0+20220218 ([`5b5c6a2`](https://github.com/Byron/google-apis-rs/commit/5b5c6a29975642fffcabe3b2d4135822484c5320))
    - Release google-firebaseremoteconfig1 v3.1.0+20171129 ([`78160b3`](https://github.com/Byron/google-apis-rs/commit/78160b3625994ad53c0c5bae4bd5d58e3b1965f7))
    - Release google-firebaseml1 v3.1.0+20220302 ([`3240f0b`](https://github.com/Byron/google-apis-rs/commit/3240f0b60f1529d69ca968a0cad7c57c598f6775))
    - Release google-firebasehosting1_beta1 v3.1.0+20220212 ([`cc0787e`](https://github.com/Byron/google-apis-rs/commit/cc0787e6179c03ddaa0bb2ebfa30b08df1f82bbc))
    - Release google-firebasehosting1 v3.1.0+20220212 ([`7e2d6b8`](https://github.com/Byron/google-apis-rs/commit/7e2d6b886cd06f00b3e510d30b30082431e15b08))
    - Release google-firebasedynamiclinks1 v3.1.0+20220228 ([`22e812d`](https://github.com/Byron/google-apis-rs/commit/22e812d8fceaf4e6b3386f31247314ba806e12b9))
    - Release google-firebasedatabase1_beta v3.1.0+20220304 ([`544e5e1`](https://github.com/Byron/google-apis-rs/commit/544e5e12e439fe44de3e0c553eb4c9a2a18df1b5))
    - Release google-firebaseappcheck1_beta v3.1.0+20220225 ([`fab142a`](https://github.com/Byron/google-apis-rs/commit/fab142a038ebe3d3d202e653fc43abd1dfa4eed5))
    - Release google-firebase1_beta1 v3.1.0+20220304 ([`3b3ab0d`](https://github.com/Byron/google-apis-rs/commit/3b3ab0dbab80106f7e38e42dee7fd956580e9278))
    - Release google-file1_beta1 v3.1.0+20220214 ([`d81c13b`](https://github.com/Byron/google-apis-rs/commit/d81c13becfa8d0c9be23039f69c6b0407df9e793))
    - Release google-file1 v3.1.0+20220214 ([`cf3e269`](https://github.com/Byron/google-apis-rs/commit/cf3e269ac71bac20bdbcaf6cec62e85f2a74409a))
    - Release google-fcmdata1_beta1 v3.1.0+20220305 ([`d758c44`](https://github.com/Byron/google-apis-rs/commit/d758c44090d1c7dd1b76557a42e352721b244bba))
    - Release google-fcm1 v3.1.0+20220228 ([`e30f214`](https://github.com/Byron/google-apis-rs/commit/e30f214a040944a93b0fe8a26aaabd1850088470))
    - Release google-factchecktools1_alpha1 v3.1.0+20220305 ([`ee2a3d7`](https://github.com/Byron/google-apis-rs/commit/ee2a3d73ec6e68341d14608ab08a57a6b8a59d8a))
    - Release google-eventarc1 v3.1.0+20220301 ([`285ecc0`](https://github.com/Byron/google-apis-rs/commit/285ecc07715da6919bedde8bfbbab85ea4dea686))
    - Release google-essentialcontacts1 v3.1.0+20220227 ([`2501dba`](https://github.com/Byron/google-apis-rs/commit/2501dbaa06f8b539d455ba251ac2c98b130bbbd7))
    - Release google-driveactivity2 v3.1.0+20220301 ([`d642eab`](https://github.com/Byron/google-apis-rs/commit/d642eaba103e8ceaa139206cade87f043bec194d))
    - Release google-drive2 v3.1.0+20220225 ([`1f7d821`](https://github.com/Byron/google-apis-rs/commit/1f7d8211d622c9e4ee0d739ef5ee42e83e82154c))
    - Release google-doubleclicksearch2 v3.1.0+20220301 ([`9a51265`](https://github.com/Byron/google-apis-rs/commit/9a5126542be29d679eacf36b07d2ac4b786dcbda))
    - Release google-doubleclickbidmanager1d1 v3.1.0+20220302 ([`0e7eea6`](https://github.com/Byron/google-apis-rs/commit/0e7eea68277bac7a3d1d03c7144b4a13d18f0a8e))
    - Release google-doubleclickbidmanager1 v3.1.0+20210323 ([`56c4a50`](https://github.com/Byron/google-apis-rs/commit/56c4a5067a9320bba3cf9d81931c6348b2936861))
    - Release google-domainsrdap1 v3.1.0+20220307 ([`e76d8bb`](https://github.com/Byron/google-apis-rs/commit/e76d8bbd05adb1a981c70c2b1dbd807cb17c663b))
    - Release google-domains1 v3.1.0+20220128 ([`3a85a14`](https://github.com/Byron/google-apis-rs/commit/3a85a145d4c7af921155a0758a6e2fb3786aaeff))
    - Release google-domains1_beta1 v3.1.0+20220128 ([`5c6cbf2`](https://github.com/Byron/google-apis-rs/commit/5c6cbf25367456f6b9711b1315562f665af1b3db))
    - Release google-documentai1_beta2 v3.1.0+20220226 ([`8fb3df2`](https://github.com/Byron/google-apis-rs/commit/8fb3df2f6961eefd5197c4b6f7974725f2c67e8c))
    - Release google-documentai1 v3.1.0+20220226 ([`baa3bb4`](https://github.com/Byron/google-apis-rs/commit/baa3bb416cd1aaa4f6c6e54569f0283b518f9b4d))
    - Release google-docs1 v3.1.0+20220301 ([`a8a3cbc`](https://github.com/Byron/google-apis-rs/commit/a8a3cbcb574a30a4482444b74c428e2bb38db64c))
    - Release google-dns2 v3.1.0+20220217 ([`9010ffe`](https://github.com/Byron/google-apis-rs/commit/9010ffe48ff539c5f67a5b605b70fcbfbd487851))
    - Release google-dns1 v3.1.0+20220217 ([`ddaac43`](https://github.com/Byron/google-apis-rs/commit/ddaac436e24140b75a249938fdb804b5efda26d5))
    - Release google-dlp2_beta1 v3.1.0+20171205 ([`1e32f46`](https://github.com/Byron/google-apis-rs/commit/1e32f466a41387ec136d05c5bd897e1d0e07774a))
    - Release google-dlp2 v3.1.0+20220227 ([`a66c3e8`](https://github.com/Byron/google-apis-rs/commit/a66c3e8093247c77c8749c6fd94d67167f7439e1))
    - Release google-displayvideo1 v3.1.0+20220303 ([`cf4a2e4`](https://github.com/Byron/google-apis-rs/commit/cf4a2e47a13fccb00c80e7e97b8768685eba280d))
    - Release google-discovery1 v3.1.0+20200806 ([`1354987`](https://github.com/Byron/google-apis-rs/commit/1354987497e25b9f80e2fca5e7113f52f602493d))
    - Release google-digitalassetlinks1 v3.1.0+20220301 ([`c54d018`](https://github.com/Byron/google-apis-rs/commit/c54d018c17bf8c72433d91e8f895dd5d450a2954))
    - Release google-dialogflow3 v3.1.0+20220228 ([`cb1ed77`](https://github.com/Byron/google-apis-rs/commit/cb1ed771f8914a7d9a1857ddd2481a92cd4e064e))
    - Release google-dialogflow2_beta1 v3.1.0+20220228 ([`69693f2`](https://github.com/Byron/google-apis-rs/commit/69693f2499c2475fcb102cb58fa5c279a6a2ddec))
    - Release google-dialogflow2 v3.1.0+20220228 ([`35fbcb8`](https://github.com/Byron/google-apis-rs/commit/35fbcb8df035ae91cc5e3b5d5b19ffc6a51f56ba))
    - Release google-dfareporting3d5 v3.1.0+20220104 ([`9dae1eb`](https://github.com/Byron/google-apis-rs/commit/9dae1ebd79d3ba78e7141e23d04581d7027ed4ee))
    - Release google-dfareporting3d4 v3.1.0+20220104 ([`8ff95c4`](https://github.com/Byron/google-apis-rs/commit/8ff95c41ded9d46e73b4ba2bb2ab88d2efb12a26))
    - Release google-dfareporting3d3 v3.1.0+20220104 ([`546deaa`](https://github.com/Byron/google-apis-rs/commit/546deaa513b2b0d403186ccd5856b591830b73ab))
    - Release google-dfareporting3d2 v3.1.0+20190531 ([`1f97e9e`](https://github.com/Byron/google-apis-rs/commit/1f97e9ed2a3110df4f6639240b65c9241d818ec2))
    - Release google-dfareporting3 v3.1.0+20180830 ([`d5be340`](https://github.com/Byron/google-apis-rs/commit/d5be340dc492709928f735822de13c7eca858b22))
    - Release google-dfareporting2d8 v3.1.0+20180830 ([`138fad0`](https://github.com/Byron/google-apis-rs/commit/138fad085bc5ef0ee26142a4c278a2c9f0589239))
    - Release google-deploymentmanager2_beta2 v3.1.0+20160201 ([`3587dae`](https://github.com/Byron/google-apis-rs/commit/3587dae940584aa09f6bc5691d8515153a00fe6d))
    - Release google-deploymentmanager2 v3.1.0+20220225 ([`090da14`](https://github.com/Byron/google-apis-rs/commit/090da143965a9158fb26b6013d610c1bcc523806))
    - Release google-datastream1 v3.1.0+20220207 ([`f6af866`](https://github.com/Byron/google-apis-rs/commit/f6af866f1706f0757f77f090a706faf6550b8e9d))
    - Release google-datastore1_beta3 v3.1.0+20220221 ([`8984f61`](https://github.com/Byron/google-apis-rs/commit/8984f6116a3a1280a43d218c1b3dc54be2fe1545))
    - Release google-datastore1 v3.1.0+20220221 ([`4830c43`](https://github.com/Byron/google-apis-rs/commit/4830c43da1a5e569858d78386d8c3ebabc17193a))
    - Release google-dataproc1 v3.1.0+20220224 ([`6e98dfb`](https://github.com/Byron/google-apis-rs/commit/6e98dfbc6fb58e179c333f19bd8737e298a683c3))
    - Release google-dataplex1 v3.1.0+20220223 ([`a050a8e`](https://github.com/Byron/google-apis-rs/commit/a050a8e9fd1c7f3097904a04e8f8404bd8a0ee3c))
    - Release google-datapipelines1 v3.1.0+20220218 ([`deea3a3`](https://github.com/Byron/google-apis-rs/commit/deea3a345f24d6aa18cb453b2bd2f1b8aecb4753))
    - Release google-datamigration1 v3.1.0+20220216 ([`4ab03a4`](https://github.com/Byron/google-apis-rs/commit/4ab03a4af3e304378462b9aaa6d6d45f8213e582))
    - Release google-datalabeling1_beta1 v3.1.0+20220301 ([`6ba067b`](https://github.com/Byron/google-apis-rs/commit/6ba067b403cf12fc4fd4fe6a2c224bb7883cfa59))
    - Release google-datafusion1_beta1 v3.1.0+20211028 ([`3733723`](https://github.com/Byron/google-apis-rs/commit/37337239e3b4260d8aaa56c62d25564b165a4767))
    - Release google-datafusion1 v3.1.0+20211028 ([`34e3938`](https://github.com/Byron/google-apis-rs/commit/34e3938b04b8b171f00d9392c1cf668e6fabe34f))
    - Release google-datacatalog1 v3.1.0+20220224 ([`a520e33`](https://github.com/Byron/google-apis-rs/commit/a520e3368fb5363c95fe939eff06874271c32f26))
    - Release google-datacatalog1_beta1 v3.1.0+20220224 ([`1104b58`](https://github.com/Byron/google-apis-rs/commit/1104b587e28baa11fb03534a131dcdc0e95552d9))
    - Release google-customsearch1 v3.1.0+20220305 ([`b925c49`](https://github.com/Byron/google-apis-rs/commit/b925c49cfe810a876a1fe9861fc5cf4a83685ab2))
    - Release google-coordinate1 v3.1.0+20150811 ([`40a735b`](https://github.com/Byron/google-apis-rs/commit/40a735bc61204bc83d6b2c9e70467b6bcdfcb5a9))
    - Release google-content2_sandbox v3.1.0+20181009 ([`a23c047`](https://github.com/Byron/google-apis-rs/commit/a23c047de7eba9e6a961e47f9a8a3a93f7d4ba11))
    - Release google-content2 v3.1.0+20220303 ([`72c0e55`](https://github.com/Byron/google-apis-rs/commit/72c0e55c8d6e4008dcdf31e31316706c6f584dfc))
    - Release google-containeranalysis1 v3.1.0+20220225 ([`7f2d8a4`](https://github.com/Byron/google-apis-rs/commit/7f2d8a4dc3b09332c7b2f17bc37b9b1f6e09e01b))
    - Release google-containeranalysis1_beta1 v3.1.0+20220225 ([`df61630`](https://github.com/Byron/google-apis-rs/commit/df6163034c091452eec1a33b2aaf66347f6a2399))
    - Release google-container1 v3.1.0+20220215 ([`0246719`](https://github.com/Byron/google-apis-rs/commit/0246719e4d9345ce3bc773967fbb262aedc09ecd))
    - Release google-contactcenterinsights1 v3.1.0+20220227 ([`84ffb00`](https://github.com/Byron/google-apis-rs/commit/84ffb00bc335cecc7fee7f568e24d1efb40dd81b))
    - Release google-consumersurveys2 v3.1.0+20170407 ([`2e63a0f`](https://github.com/Byron/google-apis-rs/commit/2e63a0f4cb8a018a92c7b6c91ddaff607556804a))
    - Release google-connectors1 v3.1.0+20220214 ([`63c5d4b`](https://github.com/Byron/google-apis-rs/commit/63c5d4b01bb3a18f59428f5b9774ae714f6eac87))
    - Release google-compute1 v3.1.0+20220224 ([`0fa368e`](https://github.com/Byron/google-apis-rs/commit/0fa368e56976d2161535a2d6c476ed10d3184ba4))
    - Release google-composer1 v3.1.0+20220224 ([`b7162cd`](https://github.com/Byron/google-apis-rs/commit/b7162cd8e6780103781837aef7994b29fa449c95))
    - Release google-commentanalyzer1_alpha1 v3.1.0+20200405 ([`5c2b9da`](https://github.com/Byron/google-apis-rs/commit/5c2b9da911f307e067ff02c1156be40ddc572c63))
    - Release google-clouduseraccountsvm_beta v3.1.0+20160316 ([`6af6cd5`](https://github.com/Byron/google-apis-rs/commit/6af6cd5dd4ba8a74f22361a73ec243394899b3bc))
    - Release google-cloudtrace2 v3.1.0+20220224 ([`7fd2b2d`](https://github.com/Byron/google-apis-rs/commit/7fd2b2daa6036fdda0198e1d2f9d1a71c2fb6cfd))
    - Release google-cloudtrace1 v3.1.0+20220224 ([`fddd9d4`](https://github.com/Byron/google-apis-rs/commit/fddd9d4d2cbdca8eb61abe25c0c681ab8bdc5a2e))
    - Release google-cloudtasks2_beta3 v3.1.0+20220212 ([`715da46`](https://github.com/Byron/google-apis-rs/commit/715da46f4f0fe2166d947ec4293038467e903646))
    - Release google-cloudtasks2_beta2 v3.1.0+20220212 ([`663e741`](https://github.com/Byron/google-apis-rs/commit/663e7416cc4063ce37b33319a05ad09d69702377))
    - Release google-cloudtasks2 v3.1.0+20220212 ([`1bf39fa`](https://github.com/Byron/google-apis-rs/commit/1bf39fa31dc95ea364b1d5201e4e603704dd68a8))
    - Release google-cloudsupport2_beta v3.1.0+20220305 ([`e00cfb6`](https://github.com/Byron/google-apis-rs/commit/e00cfb68bde9ba914f4fab706efdd2d2c568eebe))
    - Release google-cloudshell1 v3.1.0+20220301 ([`da92733`](https://github.com/Byron/google-apis-rs/commit/da92733395da502d2adbd5bcc394697285075f11))
    - Release google-cloudscheduler1_beta1 v3.1.0+20220212 ([`0f5cd27`](https://github.com/Byron/google-apis-rs/commit/0f5cd271203e735748dec3e02445e0f010179df4))
    - Release google-cloudscheduler1 v3.1.0+20220212 ([`dfa4f9f`](https://github.com/Byron/google-apis-rs/commit/dfa4f9f037c71465bf2840ad37dbebd9cb55224c))
    - Release google-cloudresourcemanager3 v3.1.0+20220306 ([`3e97848`](https://github.com/Byron/google-apis-rs/commit/3e978482fa5010ec40bc91c7563c7c20cd1bac85))
    - Release google-cloudresourcemanager2 v3.1.0+20220306 ([`df00857`](https://github.com/Byron/google-apis-rs/commit/df008576e52cc721656e8a92fc1ee0cd9df7a644))
    - Release google-cloudresourcemanager1_beta1 v3.1.0+20220306 ([`a848bc3`](https://github.com/Byron/google-apis-rs/commit/a848bc3fae436ec6933ec2fae232867af4b09198))
    - Release google-cloudresourcemanager1 v3.1.0+20220306 ([`9366d21`](https://github.com/Byron/google-apis-rs/commit/9366d21240536c63002e45995279c5fde23c8724))
    - Release google-cloudprofiler2 v3.1.0+20220228 ([`6011b86`](https://github.com/Byron/google-apis-rs/commit/6011b863890f0590e17b7c1c5088f21f74848842))
    - Release google-cloudprivatecatalogproducer1_beta1 v3.1.0+20200405 ([`28dd448`](https://github.com/Byron/google-apis-rs/commit/28dd4484340f1cf4d8fa7e948abdd84fc9a78ee6))
    - Release google-cloudprivatecatalog1_beta1 v3.1.0+20200405 ([`094b4d6`](https://github.com/Byron/google-apis-rs/commit/094b4d6ca3d5ed85436f27d0d964262504d1df93))
    - Release google-cloudmonitoring2_beta2 v3.1.0+20170501 ([`47aed21`](https://github.com/Byron/google-apis-rs/commit/47aed214c277230900c501fa794f88c25ccc8862))
    - Release google-cloudlatencytest2 v3.1.0+20160309 ([`6317c60`](https://github.com/Byron/google-apis-rs/commit/6317c603732099594cc1563f87b921dd0d47e6a6))
    - Release google-cloudkms1_beta1 v3.1.0+20170515 ([`c27ba6f`](https://github.com/Byron/google-apis-rs/commit/c27ba6f66f2dd12131ada9b3287148e720d6ab40))
    - Release google-cloudkms1 v3.1.0+20220225 ([`5b327fc`](https://github.com/Byron/google-apis-rs/commit/5b327fc1c6a7ffe119340ab5e10ed6d2e666b180))
    - Release google-cloudiot1 v3.1.0+20220131 ([`ee3a136`](https://github.com/Byron/google-apis-rs/commit/ee3a136bb31f97b644acb3ab751b5b5a8d1a48bf))
    - Release google-cloudidentity1 v3.1.0+20220301 ([`40346f6`](https://github.com/Byron/google-apis-rs/commit/40346f6b4852118c9291cfc478d060c6f288ab3d))
    - Release google-cloudfunctions1 v3.1.0+20220224 ([`2258ea9`](https://github.com/Byron/google-apis-rs/commit/2258ea9b182aed0d5cae022b373de969668cc4e6))
    - Release google-clouderrorreporting1_beta1 v3.1.0+20220302 ([`646f342`](https://github.com/Byron/google-apis-rs/commit/646f342da518c0b573a2a1636b36ff90495e2b45))
    - Release google-clouddeploy1 v3.1.0+20220223 ([`f0a6041`](https://github.com/Byron/google-apis-rs/commit/f0a6041132f1da2c1d7cf851fcd9ad0e448dd8e2))
    - Release google-clouddebugger2 v3.1.0+20220225 ([`71e7e91`](https://github.com/Byron/google-apis-rs/commit/71e7e9194dc598aec46e9e688f84a753055c9645))
    - Release google-cloudchannel1 v3.1.0+20220303 ([`955b2c1`](https://github.com/Byron/google-apis-rs/commit/955b2c1f395cec804b464a4b32d99f61cf48ec87))
    - Release google-cloudbuild1 v3.1.0+20220218 ([`1272e02`](https://github.com/Byron/google-apis-rs/commit/1272e024765dc24bda41a02ab9156d8ce5e5dbaa))
    - Release google-cloudbilling1 v3.1.0+20220305 ([`23653b2`](https://github.com/Byron/google-apis-rs/commit/23653b23a78c13ce05397e5ceadc056901678167))
    - Release google-cloudasset1_beta1 v3.1.0+20220225 ([`8911aaf`](https://github.com/Byron/google-apis-rs/commit/8911aafbe3d4f863aabade37251331d062c5e293))
    - Release google-cloudasset1 v3.1.0+20220225 ([`bc048fb`](https://github.com/Byron/google-apis-rs/commit/bc048fb2576418d6ce3ab9bcf6f751d196e1bdaa))
    - Release google-classroom1 v3.1.0+20220224 ([`ba6e5e2`](https://github.com/Byron/google-apis-rs/commit/ba6e5e2b49c7a45f1bf66b06df1ec3af3c624195))
    - Release google-chromeuxreport1 v3.1.0+20220302 ([`705658c`](https://github.com/Byron/google-apis-rs/commit/705658c23505ee1c100ea68b98abbdc4bcefa293))
    - Release google-chromepolicy1 v3.1.0+20220305 ([`655b08c`](https://github.com/Byron/google-apis-rs/commit/655b08c580518723be305a9f8c13d935d4f45172))
    - Release google-chromemanagement1 v3.1.0+20220305 ([`6d37957`](https://github.com/Byron/google-apis-rs/commit/6d379573177cd4907a6ce0216498d3f4756eb0d6))
    - Release google-certificatemanager1 v3.1.0+20220214 ([`70e5ef3`](https://github.com/Byron/google-apis-rs/commit/70e5ef3d73d6d41adda81d8b0f1f3d9df04ec817))
    - Release google-calendar3 v3.1.0+20220217 ([`9801169`](https://github.com/Byron/google-apis-rs/commit/9801169552dbcd20520920872f5a024451938d13))
    - Release google-books1 v3.1.0+20220301 ([`736dac1`](https://github.com/Byron/google-apis-rs/commit/736dac1a89218cd3d6963ea7f44459dc9ca27a83))
    - Release google-blogger3 v3.1.0+20220305 ([`84250f0`](https://github.com/Byron/google-apis-rs/commit/84250f0d00885a4880db309adb03e04ec4955e8d))
    - Release google-binaryauthorization1_beta1 v3.1.0+20220225 ([`9d5e81b`](https://github.com/Byron/google-apis-rs/commit/9d5e81babcb169e3a5977ab06014b7a2bb763403))
    - Release google-binaryauthorization1 v3.1.0+20220225 ([`137f1bf`](https://github.com/Byron/google-apis-rs/commit/137f1bf9888fded0fc57e0d6988f7efdaf27bc98))
    - Release google-billingbudgets1_beta1 v3.1.0+20220227 ([`202f19e`](https://github.com/Byron/google-apis-rs/commit/202f19ed881b54080652eb91b48e20d6d37acdeb))
    - Release google-billingbudgets1 v3.1.0+20220227 ([`8f17bc0`](https://github.com/Byron/google-apis-rs/commit/8f17bc06f640e3a415dcc7f98116d6004eb88939))
    - Release google-bigtableadmin2 v3.1.0+20220222 ([`a1929ff`](https://github.com/Byron/google-apis-rs/commit/a1929ffda5a8b94a5a44f6a339fef505065137fb))
    - Release google-bigqueryreservation1 v3.1.0+20220226 ([`59918ac`](https://github.com/Byron/google-apis-rs/commit/59918ac692ba05cea8d061c7edc1ee36c6fac90c))
    - Release google-bigquerydatatransfer1 v3.1.0+20220225 ([`d20dcf0`](https://github.com/Byron/google-apis-rs/commit/d20dcf049ecefd67f50992991885555f1c22ed69))
    - Release google-bigqueryconnection1_beta1 v3.1.0+20220226 ([`c80c208`](https://github.com/Byron/google-apis-rs/commit/c80c20852d2a75aab303fb917181af478f653df1))
    - Release google-bigquery2 v3.1.0+20220222 ([`3125f22`](https://github.com/Byron/google-apis-rs/commit/3125f22c48cd83a11e64b8a839108a976c643d83))
    - Release google-baremetalsolution2 v3.1.0+20220209 ([`6e4e405`](https://github.com/Byron/google-apis-rs/commit/6e4e405bc28ec9d2e88c163748480d7b0bea7132))
    - Release google-autoscaler1_beta2 v3.1.0+20150629 ([`5423f90`](https://github.com/Byron/google-apis-rs/commit/5423f90faf2704c63acc677adc4c62fabe60d997))
    - Release google-authorizedbuyersmarketplace1 v3.1.0+20220307 ([`6e3cb17`](https://github.com/Byron/google-apis-rs/commit/6e3cb17fcd1e4da2e581c4a43253841d033f9c20))
    - Release google-assuredworkloads1 v3.1.0+20220224 ([`232017b`](https://github.com/Byron/google-apis-rs/commit/232017b330c37d272ee587493453e7559433159f))
    - Release google-artifactregistry1_beta1 v3.1.0+20220225 ([`5cb8beb`](https://github.com/Byron/google-apis-rs/commit/5cb8beb8fb34ee41ebc7b199b75fce3401f6bb4b))
    - Release google-artifactregistry1 v3.1.0+20220225 ([`fe7cfa0`](https://github.com/Byron/google-apis-rs/commit/fe7cfa0ddc5a3f6cd2e44c14fe3091cdd62202f0))
    - Release google-area120tables1_alpha1 v3.1.0+20220301 ([`7f8245d`](https://github.com/Byron/google-apis-rs/commit/7f8245d92e28d58166b0c19e77af470d95f4a4fe))
    - Release google-appstate1 v3.1.0+20190627 ([`d8ab59e`](https://github.com/Byron/google-apis-rs/commit/d8ab59e3c4d6ca3e07309f81fff4d430550440e5))
    - Release google-appsactivity1 v3.1.0+20200628 ([`c5c6bc7`](https://github.com/Byron/google-apis-rs/commit/c5c6bc71e85cf5821b15427ef25b12f36b8716be))
    - Release google-appengine1_beta5 v3.1.0+20181005 ([`7a7b0ae`](https://github.com/Byron/google-apis-rs/commit/7a7b0aeb1176e5e93c34baee371c02e53b2836a7))
    - Release google-appengine1_beta4 v3.1.0+20181005 ([`7e78b4e`](https://github.com/Byron/google-apis-rs/commit/7e78b4e0a7a0c9f3e566bef351fc67b14719af89))
    - Release google-appengine1 v3.1.0+20220226 ([`14947ec`](https://github.com/Byron/google-apis-rs/commit/14947ec4ee3f37fa976c6448b5579f90ca378247))
    - Release google-apikeys2 v3.1.0+20220305 ([`36b9561`](https://github.com/Byron/google-apis-rs/commit/36b956187e4db7f2a35435046b5291f2131d2833))
    - Release google-apigee1 v3.1.0+20220301 ([`0b00130`](https://github.com/Byron/google-apis-rs/commit/0b00130bcc1ad6d5ce03f678cc374d3e6202cbb3))
    - Release google-apigateway1 v3.1.0+20220223 ([`ad0f587`](https://github.com/Byron/google-apis-rs/commit/ad0f5870edc4bfda1f2e9d77742a8a63bac28734))
    - Release google-androidpublisher3 v3.1.0+20220307 ([`1edd1c2`](https://github.com/Byron/google-apis-rs/commit/1edd1c2a4f5a64c2b115a78e16babff1a0870d88))
    - Release google-androidpublisher2 v3.1.0+20200331 ([`6dc4085`](https://github.com/Byron/google-apis-rs/commit/6dc408575b0aa711cdfb40f7b088f2d098cc4df2))
    - Release google-androidmanagement1 v3.1.0+20220302 ([`031cbda`](https://github.com/Byron/google-apis-rs/commit/031cbda4128e0f039bc040d715fba4f20761486d))
    - Release google-androidenterprise1 v3.1.0+20220303 ([`60c780f`](https://github.com/Byron/google-apis-rs/commit/60c780f9b7b87c4af0b8bfb1c379b59193a07378))
    - Release google-androiddeviceprovisioning1 v3.1.0+20220305 ([`b6d0645`](https://github.com/Byron/google-apis-rs/commit/b6d064576d13f7202420a4ba179f2cf73fd5de96))
    - Release google-analyticsreporting4 v3.1.0+20220215 ([`956a1bb`](https://github.com/Byron/google-apis-rs/commit/956a1bbb017d5ae7069ea8a2e3338565516ba190))
    - Release google-analyticsdata1_beta v3.1.0+20220303 ([`7056c33`](https://github.com/Byron/google-apis-rs/commit/7056c33676f78db8ff7e74261ad16590408d9f07))
    - Release google-analyticsadmin1_alpha v3.1.0+20220307 ([`db81ba3`](https://github.com/Byron/google-apis-rs/commit/db81ba3d296adf07bf3289e8433d7e747cf7a1ce))
    - Release google-analytics3 v3.1.0+20190807 ([`c6d47cc`](https://github.com/Byron/google-apis-rs/commit/c6d47cc3ef26b09e9b5da1806a3f125c9002d397))
    - Release google-alertcenter1_beta1 v3.1.0+20220221 ([`eec3c6a`](https://github.com/Byron/google-apis-rs/commit/eec3c6a4365bda983198c34c203bd4c1abf832a9))
    - Release google-adsensehost4d1 v3.1.0+20200930 ([`bd5f37c`](https://github.com/Byron/google-apis-rs/commit/bd5f37c9829293b75eb9b368a425d8e9e9fff2cb))
    - Release google-adsense2 v3.1.0+20220304 ([`68697b9`](https://github.com/Byron/google-apis-rs/commit/68697b98d8b818e8fde14dbc0638fb0961b93783))
    - Release google-adsense1d4 v3.1.0+20201002 ([`f1595e2`](https://github.com/Byron/google-apis-rs/commit/f1595e27708cb14dd46158a0d7842bc12dc0f1fd))
    - Release google-admob1 v3.1.0+20220303 ([`59949a1`](https://github.com/Byron/google-apis-rs/commit/59949a12b4bdb9ea60ed00e49f9f0e0c2a3b4722))
    - Release google-adexperiencereport1 v3.1.0+20220303 ([`23f3c81`](https://github.com/Byron/google-apis-rs/commit/23f3c8161c9d3f972af34dc9daae0af019c448a3))
    - Release google-adexchangeseller2 v3.1.0+20171101 ([`469be4e`](https://github.com/Byron/google-apis-rs/commit/469be4e60d6a06a877005a27d15d43ef53895239))
    - Release google-adexchangebuyer2_v2_beta1 v3.1.0+20220307 ([`d003621`](https://github.com/Byron/google-apis-rs/commit/d0036217c19bca7cd9d272242f8a654926647dcd))
    - Release google-adexchangebuyer1d4 v3.1.0+20210330 ([`bae805e`](https://github.com/Byron/google-apis-rs/commit/bae805e1633b1e8357219b3f2fa12e5f9f24fc7d))
    - Release google-adexchangebuyer1d3 v3.1.0+20210330 ([`122a51a`](https://github.com/Byron/google-apis-rs/commit/122a51a13f5756decfc7a29883d5a30a15fc89f5))
    - Release google-accesscontextmanager1_beta v3.1.0+20220301 ([`d1c3cd7`](https://github.com/Byron/google-apis-rs/commit/d1c3cd7fdef1895f4c60e372e7f231db47a9d8e0))
    - Release google-accesscontextmanager1 v3.1.0+20220301 ([`417431d`](https://github.com/Byron/google-apis-rs/commit/417431d8d88111e36a554a74d6b5447e8afff674))
    - Release google-accessapproval1_beta1 v3.1.0+20200708 ([`ec59fed`](https://github.com/Byron/google-apis-rs/commit/ec59fed4975959da3277f3f7b8adf94d11e22fb8))
    - Release google-accessapproval1 v3.1.0+20220225 ([`bf30007`](https://github.com/Byron/google-apis-rs/commit/bf30007c7bd40838808f0c243b8b9cd03297710a))
    - Release google-acceleratedmobilepageurl1 v3.1.0+20220305 ([`b154c01`](https://github.com/Byron/google-apis-rs/commit/b154c0121e642ee175fbe459d68248004298b35b))
    - Release google-abusiveexperiencereport1 v3.1.0+20220303 ([`eb2fb61`](https://github.com/Byron/google-apis-rs/commit/eb2fb61f0969c7f4e8c7fd6601c69b455210f69f))
    - Release google-texttospeech1 v3.1.0+20220228 ([`79fad37`](https://github.com/Byron/google-apis-rs/commit/79fad3762cd7f2d0d8622c62d9eca0d0cdca69f8))
    - regen all APIs and CLIs ([`f393030`](https://github.com/Byron/google-apis-rs/commit/f393030e84320eb41112b7d19a5bcb7e8cd05a24))
    - Release google-drive3 v3.1.0+20220225 ([`cc296a5`](https://github.com/Byron/google-apis-rs/commit/cc296a57b672e60dec3d0f3b134f452877b5dd26))
    - regen drive3 API prior to release ([`b4c019f`](https://github.com/Byron/google-apis-rs/commit/b4c019f726d359c55e3a92220cf7f59f0893d2ca))
    - Release google-youtube3 v3.1.0+20220303 ([`f08b907`](https://github.com/Byron/google-apis-rs/commit/f08b90760b5e3c26e25aa1e567722986026d0522))
    - regenerate youtube3 API ([`03b851e`](https://github.com/Byron/google-apis-rs/commit/03b851ea40c2b0cdce8292b68b63cb8f6342d17d))
    - Release google-sheets4 v3.1.0+20220221 ([`3173b95`](https://github.com/Byron/google-apis-rs/commit/3173b95c0549f93f4c8665e80e76c1e321de07b4))
    - Release google-sheets4-cli v3.1.0+20220221 ([`72007f7`](https://github.com/Byron/google-apis-rs/commit/72007f74a8104241c13030083e2fe646d7bae2a1))
    - regen sheets 4 to prepare for release ([`3c5c971`](https://github.com/Byron/google-apis-rs/commit/3c5c971b3bc15642759f9dc81da0608a52be9a27))
    - bump build version ([`493e723`](https://github.com/Byron/google-apis-rs/commit/493e723a7a23977a46795a44d508de374a7f0ee6))
    - fix build of CLI ([`815d772`](https://github.com/Byron/google-apis-rs/commit/815d772c2761c77c18764536d43dec77f634f0dc))
    - Bump hyper-rustls version ([`42090e2`](https://github.com/Byron/google-apis-rs/commit/42090e283dffd4e1b66ac922a6a549aace361700))
    - add release tracking file ([`7df7d34`](https://github.com/Byron/google-apis-rs/commit/7df7d346b8ea69af3ace8885fb252b68a3e994bd))
    - Release google-bigquery2 v3.0.2+20220222 ([`790b526`](https://github.com/Byron/google-apis-rs/commit/790b52671368862aa7a16df500df950fdd510c04))
    - regenerate bigquery ([`df94ed5`](https://github.com/Byron/google-apis-rs/commit/df94ed5bb208c5283298940de6f6ef30ec14bfa8))
    - bump shared patch level prior to bigquery2 release ([`84084a4`](https://github.com/Byron/google-apis-rs/commit/84084a465de50615797a9eb38a4d1ffc11fdf91c))
    - Release google-bigquery2 v3.0.1+20220222 ([`2c7e641`](https://github.com/Byron/google-apis-rs/commit/2c7e64134a93a1cae6ddd2608043eeb98e7fc8df))
    - Release google-bigquery2 v3.0.1+20220222 ([`c083b40`](https://github.com/Byron/google-apis-rs/commit/c083b40be111c740ca2273183ad6942506922cac))
    - regenerate biquery prior to release ([`3b38050`](https://github.com/Byron/google-apis-rs/commit/3b3805075e08a84dd824239775f431a9db260bf4))
    - bump patch level for re-releasing bigquery ([`b7d157f`](https://github.com/Byron/google-apis-rs/commit/b7d157fa1852098fb30d56c5c2916d1cb7ffa6c6))
    - add non-uploading job.insert rpc ([`beb110a`](https://github.com/Byron/google-apis-rs/commit/beb110a941186f938996b3e928824f04f59183d3))
    - upgrade top-level manifest's yup-oauth2 to avoid confusion ([`b92afd3`](https://github.com/Byron/google-apis-rs/commit/b92afd37c0e9d281613995623cc0dc3702bb0943))
    - publishing results ([`11d332e`](https://github.com/Byron/google-apis-rs/commit/11d332e69ccfd53ee2a4b18c5134dfb819a3ce28))
    - Release google-youtubereporting1-cli v3.0.0+20220305 ([`f910746`](https://github.com/Byron/google-apis-rs/commit/f9107460af1900d1dccc0257885fbdf2e504c723))
    - Release google-youtube3-cli v3.0.0+20220303 ([`5e8177e`](https://github.com/Byron/google-apis-rs/commit/5e8177e0d853961f17b4a28ad5f3b7ff0ce24fdf))
    - Release google-workflows1-cli v3.0.0+20220223 ([`67c685a`](https://github.com/Byron/google-apis-rs/commit/67c685ac7d1edd6fdd143dad1f7a3d88bf019a7e))
    - Release google-workflowexecutions1-cli v3.0.0+20220222 ([`3a716a1`](https://github.com/Byron/google-apis-rs/commit/3a716a1c86976ea460bda1d46d1976b52e3f1c2b))
    - Release google-webrisk1-cli v3.0.0+20220226 ([`03ed813`](https://github.com/Byron/google-apis-rs/commit/03ed813f5bd6f50e45bf6c9ae57d4539b5b7b020))
    - Release google-webmasters3-cli v3.0.0+20190428 ([`0166855`](https://github.com/Byron/google-apis-rs/commit/0166855eb86e3e82aef2d19fd3b00769de93fc85))
    - Release google-webfonts1-cli v3.0.0+20220215 ([`0284c76`](https://github.com/Byron/google-apis-rs/commit/0284c7615d84942deb7273079b927842403000cf))
    - Release google-vmmigration1-cli v3.0.0+20220225 ([`e5a6355`](https://github.com/Byron/google-apis-rs/commit/e5a6355f5c4d18f1e19186d481f1c96e7883442f))
    - Release google-vision1-cli v3.0.0+20220225 ([`a5c6bd9`](https://github.com/Byron/google-apis-rs/commit/a5c6bd9edfc7467b7b9ca9bac696fe8ddb7e394c))
    - Release google-videointelligence1_beta1-cli v3.0.0+20171122 ([`017af71`](https://github.com/Byron/google-apis-rs/commit/017af71c4cb69259ffd2f5f334810c267114fb80))
    - Release google-videointelligence1-cli v3.0.0+20220225 ([`dc19016`](https://github.com/Byron/google-apis-rs/commit/dc190161444261fc54cd4bc1fd04ed16f2b83145))
    - Release google-versionhistory1-cli v3.0.0+20220307 ([`322130e`](https://github.com/Byron/google-apis-rs/commit/322130e290b018f65bbb6910e6dde2458f1bba2f))
    - Release google-verifiedaccess1-cli v3.0.0+20220215 ([`5de1e2e`](https://github.com/Byron/google-apis-rs/commit/5de1e2e76e6f1f04bc1ec2bba60d093b09d38b05))
    - Release google-vectortile1-cli v3.0.0+20210331 ([`f368b3e`](https://github.com/Byron/google-apis-rs/commit/f368b3e2a244ed2be6f2486ce47394008454716f))
    - Release google-vault1-cli v3.0.0+20220222 ([`d748261`](https://github.com/Byron/google-apis-rs/commit/d748261e7b5f4284a829f1d91eabea3d1f2975e5))
    - Release google-urlshortener1-cli v3.0.0+20150519 ([`2819ef6`](https://github.com/Byron/google-apis-rs/commit/2819ef6b33f3d6187abf0f5f05e15152d7598049))
    - Release google-translate3-cli v3.0.0+20220121 ([`d5cc684`](https://github.com/Byron/google-apis-rs/commit/d5cc68468813b225588fc84e679d0df110b26fde))
    - Release google-translate2-cli v3.0.0+20170525 ([`5eb22d4`](https://github.com/Byron/google-apis-rs/commit/5eb22d4ef9cf629f8056d0f259e6b1295a25cd4c))
    - Release google-transcoder1-cli v3.0.0+20220201 ([`05a1938`](https://github.com/Byron/google-apis-rs/commit/05a193824b6ee11bf081dd27288519b0602ffb4e))
    - Release google-transcoder1_beta1-cli v3.0.0+20210323 ([`427e8fe`](https://github.com/Byron/google-apis-rs/commit/427e8fec653cbb8dcaabacf94f81becd9bae1440))
    - Release google-tpu1_alpha1-cli v3.0.0+20220301 ([`6949689`](https://github.com/Byron/google-apis-rs/commit/69496896c24bd8cfc1c2fe86cfc3ca2a5b4e6a8d))
    - Release google-tpu1-cli v3.0.0+20220301 ([`3a615f4`](https://github.com/Byron/google-apis-rs/commit/3a615f44ad42e184516a97b6a96ec1c872e68631))
    - Release google-texttospeech1-cli v3.0.0+20220228 ([`8ca0f28`](https://github.com/Byron/google-apis-rs/commit/8ca0f28376e35c0c882f9ecff90514b6a59e7d3f))
    - Release google-testing1-cli v3.0.0+20220301 ([`e4d969d`](https://github.com/Byron/google-apis-rs/commit/e4d969d86701092170b3f12786949a50470d64e1))
    - Release google-tasks1-cli v3.0.0+20220305 ([`1aba07c`](https://github.com/Byron/google-apis-rs/commit/1aba07ca8a3f34e4fd1e74c861ce54c8d9851c21))
    - Release google-taskqueue1_beta2-cli v3.0.0+20160428 ([`56f6bde`](https://github.com/Byron/google-apis-rs/commit/56f6bde6b1fdd1c62981dfd814bbd68850d7efed))
    - Release google-tagmanager2-cli v3.0.0+20220301 ([`5e0cdcf`](https://github.com/Byron/google-apis-rs/commit/5e0cdcf6c4afbafa458feef911051a4ec1b5a244))
    - Release google-tagmanager1-cli v3.0.0+20220301 ([`ab1a8ca`](https://github.com/Byron/google-apis-rs/commit/ab1a8cacf23e0e12c7c00962f0f52b13ae039c20))
    - Release google-surveys2-cli v3.0.0+20180508 ([`eeb00ad`](https://github.com/Byron/google-apis-rs/commit/eeb00ada3b5005ba444828c98f84ef097e9ed65c))
    - Release google-sts1-cli v3.0.0+20220227 ([`54e8b50`](https://github.com/Byron/google-apis-rs/commit/54e8b508e36c7b0fb7195e082721092d418995b3))
    - Release google-storagetransfer1-cli v3.0.0+20220223 ([`ef9feb7`](https://github.com/Byron/google-apis-rs/commit/ef9feb70de00c653dc1329281ecd2150daa23d16))
    - Release google-storage1-cli v3.0.0+20220228 ([`a656add`](https://github.com/Byron/google-apis-rs/commit/a656add05d099c66caf7997215cd0dd7b3da53ad))
    - Release google-sqladmin1-cli v3.0.0+20220226 ([`4e1daa2`](https://github.com/Byron/google-apis-rs/commit/4e1daa277409eb1fc1b0f79f29b45ec55bbad3c8))
    - Release google-sqladmin1_beta4-cli v3.0.0+20220226 ([`c84323d`](https://github.com/Byron/google-apis-rs/commit/c84323dcfbe5941e2d206e9f1e2459d99c4ba640))
    - Release google-sql1_beta4-cli v3.0.0+20200331 ([`e6f86f7`](https://github.com/Byron/google-apis-rs/commit/e6f86f729c50b288a10a1b47cc2ddbc39ead0ec4))
    - Release google-speech1_beta1-cli v3.0.0+20181005 ([`2777ef7`](https://github.com/Byron/google-apis-rs/commit/2777ef7c88e67947299ba79b89ba75b0eccf3790))
    - Release google-speech1-cli v3.0.0+20220221 ([`625a29a`](https://github.com/Byron/google-apis-rs/commit/625a29a51adc625ac35e59022f4ebf0c40201181))
    - Release google-spectrum1_explorer-cli v3.0.0+20170306 ([`694fafa`](https://github.com/Byron/google-apis-rs/commit/694fafa94d1d6e1a56231bed61d0daad3939af9d))
    - Release google-sourcerepo1-cli v3.0.0+20220217 ([`e809a7f`](https://github.com/Byron/google-apis-rs/commit/e809a7f71b1ae8c66d69fc2084b322821f9d7cea))
    - Release google-smartdevicemanagement1-cli v3.0.0+20220302 ([`c5436c7`](https://github.com/Byron/google-apis-rs/commit/c5436c7a6943457067a1a34183a605fc19876b91))
    - Release google-siteverification1-cli v3.0.0+20191119 ([`c624974`](https://github.com/Byron/google-apis-rs/commit/c624974e9421829ae6593f7ee9ba89fa536e194a))
    - Release google-sheets4-cli v3.0.0+20220221 ([`8490b1a`](https://github.com/Byron/google-apis-rs/commit/8490b1a243d814cd9de6d2b93b761221e82adde2))
    - Release google-serviceregistryalpha-cli v3.0.0+20160401 ([`b563268`](https://github.com/Byron/google-apis-rs/commit/b563268e7d118e0ec5e6219357ab0c9bc958207f))
    - Release google-servicedirectory1_beta1-cli v3.0.0+20220224 ([`5d3b20f`](https://github.com/Byron/google-apis-rs/commit/5d3b20f94c65ae9d865afb108495c280f3ef1a73))
    - Release google-servicedirectory1-cli v3.0.0+20220224 ([`6895ada`](https://github.com/Byron/google-apis-rs/commit/6895adaa5a9d9fa3a60bbf381599a8c61dc9a194))
    - Release google-servicecontrol2-cli v3.0.0+20220227 ([`00f383e`](https://github.com/Byron/google-apis-rs/commit/00f383ebb96c1607d0a339e915a19eb417a44258))
    - Release google-servicecontrol1-cli v3.0.0+20220227 ([`3d61303`](https://github.com/Byron/google-apis-rs/commit/3d61303e25268ab39642affd775fa3bac0426a44))
    - Release google-servicebroker1-cli v3.0.0+20190624 ([`65ae382`](https://github.com/Byron/google-apis-rs/commit/65ae3828883923604ff4c9d172e6adf7b68610a1))
    - Release google-securitycenter1-cli v3.0.0+20220224 ([`c8ae7a3`](https://github.com/Byron/google-apis-rs/commit/c8ae7a3efa749d8df64cc80c69214eb6cd93b823))
    - Release google-secretmanager1_beta1-cli v3.0.0+20220226 ([`1af2488`](https://github.com/Byron/google-apis-rs/commit/1af2488762cad296eb1a156b505f4a0b7b04b9da))
    - Release google-secretmanager1-cli v3.0.0+20220226 ([`29f4432`](https://github.com/Byron/google-apis-rs/commit/29f44327144f39e579a90fafebc12713781adeb6))
    - Release google-searchconsole1-cli v3.0.0+20220305 ([`2526e05`](https://github.com/Byron/google-apis-rs/commit/2526e05b29e6bcdad03cc8ca009a3cf21c3baa38))
    - Release google-sasportal1_alpha1-cli v3.0.0+20220301 ([`9bb333c`](https://github.com/Byron/google-apis-rs/commit/9bb333c2a3b49d091abdd2586472f5f6e89b0c41))
    - Release google-safebrowsing4-cli v3.0.0+20220305 ([`3530816`](https://github.com/Byron/google-apis-rs/commit/3530816517d697905bae6a30b7378f5973dfa17a))
    - Release google-runtimeconfig1_beta1-cli v3.0.0+20220228 ([`be8b5c2`](https://github.com/Byron/google-apis-rs/commit/be8b5c2539c41ca6deb0fdc2636ac5749ecebf51))
    - Release google-runtimeconfig1-cli v3.0.0+20220228 ([`2e931c9`](https://github.com/Byron/google-apis-rs/commit/2e931c955b998af0e45653e50960537ee02d1856))
    - Release google-run2-cli v3.0.0+20220225 ([`2e7820e`](https://github.com/Byron/google-apis-rs/commit/2e7820eed3c760769fb3ecf79878f7a7ddfe3fcf))
    - Release google-run1-cli v3.0.0+20220225 ([`3b324da`](https://github.com/Byron/google-apis-rs/commit/3b324da18ebc93230054fbdbc5be5679bc29a85c))
    - Release google-retail2-cli v3.0.0+20220224 ([`f342a29`](https://github.com/Byron/google-apis-rs/commit/f342a291b53565e1da5e00c9c499952edb81d757))
    - Release google-resourceviews1_beta2-cli v3.0.0+20160512 ([`0eeb371`](https://github.com/Byron/google-apis-rs/commit/0eeb3719f3f6d22c3abc00f057dc5248a768acb9))
    - Release google-resourcesettings1-cli v3.0.0+20220305 ([`2d12834`](https://github.com/Byron/google-apis-rs/commit/2d128342b615388b3d3cac2c86f0be7f58d04b8a))
    - Release google-reseller1_sandbox-cli v3.0.0+20160329 ([`79b8fa9`](https://github.com/Byron/google-apis-rs/commit/79b8fa916a9466b325978ec0ba40f2cd1d8abeab))
    - Release google-replicapoolupdater1_beta1-cli v3.0.0+20161003 ([`d2046b6`](https://github.com/Byron/google-apis-rs/commit/d2046b6f2062a83efc27805b84758505492dbb3f))
    - Release google-replicapool1_beta2-cli v3.0.0+20160512 ([`2fb2fc8`](https://github.com/Byron/google-apis-rs/commit/2fb2fc8dfdbb1d3d7260ea8941717b0a732decbe))
    - Release google-remotebuildexecution2-cli v3.0.0+20210329 ([`e12c129`](https://github.com/Byron/google-apis-rs/commit/e12c129738e1bb04088155187f5f6a065a9079d1))
    - Release google-redis1-cli v3.0.0+20220301 ([`ff3a104`](https://github.com/Byron/google-apis-rs/commit/ff3a104aa8651de032d067da964d3327877a12ce))
    - Release google-recommender1_beta1-cli v3.0.0+20220228 ([`fb404d8`](https://github.com/Byron/google-apis-rs/commit/fb404d8e4bb1629f232dd43bab196b6bd70d0572))
    - Release google-recommender1-cli v3.0.0+20220228 ([`a0396da`](https://github.com/Byron/google-apis-rs/commit/a0396da5e9e75fc92fb39aefbe739e6f10d4537d))
    - Release google-recommendationengine1_beta1-cli v3.0.0+20220224 ([`795c847`](https://github.com/Byron/google-apis-rs/commit/795c84751d4fc8b48df441bad120dd1e0b09d142))
    - Release google-recaptchaenterprise1-cli v3.0.0+20220226 ([`98e50fe`](https://github.com/Byron/google-apis-rs/commit/98e50fec9f77cfd4b49b2fb323570ddcbb17dc07))
    - Release google-realtimebidding1-cli v3.0.0+20220307 ([`042129b`](https://github.com/Byron/google-apis-rs/commit/042129b6d8377ac9926b20ecc517a8431cb3df91))
    - Release google-qpxexpress1-cli v3.0.0+20160708 ([`9d142c9`](https://github.com/Byron/google-apis-rs/commit/9d142c9ecd3d7840f55ff125994293dc720122b1))
    - Release google-pubsublite1-cli v3.0.0+20220301 ([`c14881e`](https://github.com/Byron/google-apis-rs/commit/c14881e09b87548d5ee1e6c4d6e0c803c944365e))
    - Release google-pubsub1_beta2-cli v3.0.0+20220221 ([`56c388d`](https://github.com/Byron/google-apis-rs/commit/56c388d3da0b9fab1f9e2faad376c47d3f6b655e))
    - Release google-pubsub1-cli v3.0.0+20220221 ([`2d219fe`](https://github.com/Byron/google-apis-rs/commit/2d219fe6d3f1f92149064d85cc9776bb9ec8130c))
    - Release google-proximitybeacon1_beta1-cli v3.0.0+20200127 ([`309030c`](https://github.com/Byron/google-apis-rs/commit/309030c2d631da74ae06565ab168159ef92e7593))
    - Release google-prod_tt_sasportal1_alpha1-cli v3.0.0+20220303 ([`fdcfbb0`](https://github.com/Byron/google-apis-rs/commit/fdcfbb02584e0a7cda2a5a8267ccb5b2477fa9ed))
    - Release google-privateca1-cli v3.0.0+20220209 ([`ebc2669`](https://github.com/Byron/google-apis-rs/commit/ebc2669d2e6b0c2d749eeef98fc5dadfd1b9ef9b))
    - Release google-privateca1_beta1-cli v3.0.0+20220209 ([`5427734`](https://github.com/Byron/google-apis-rs/commit/54277347dda68ad77867687783ea253ffc47dc71))
    - Release google-prediction1d6-cli v3.0.0+20160511 ([`ce35e89`](https://github.com/Byron/google-apis-rs/commit/ce35e8902224bc35a390d0eb2a330dcb15e26761))
    - Release google-policytroubleshooter1-cli v3.0.0+20220227 ([`9b7771d`](https://github.com/Byron/google-apis-rs/commit/9b7771ddbfd0242782c738177490a33ed2f6c556))
    - Release google-policysimulator1-cli v3.0.0+20220227 ([`b06d61b`](https://github.com/Byron/google-apis-rs/commit/b06d61bf2ec9c0c9da3cd84719b944191db683f3))
    - Release google-policyanalyzer1-cli v3.0.0+20220227 ([`e54b7d6`](https://github.com/Byron/google-apis-rs/commit/e54b7d69e303e472fadf02e4406d725b89f24d90))
    - Release google-plusdomains1-cli v3.0.0+20190616 ([`70aaf8d`](https://github.com/Byron/google-apis-rs/commit/70aaf8ddb7166413ee579221cbd8287032056b1a))
    - Release google-plus1-cli v3.0.0+20190616 ([`c19297a`](https://github.com/Byron/google-apis-rs/commit/c19297af48b5dcda5ac7ced79f037c7a0f5fdf78))
    - Release google-playmoviespartner1-cli v3.0.0+20170919 ([`32babcc`](https://github.com/Byron/google-apis-rs/commit/32babcc3141a1b45a33a81b026553d9b698a0de8))
    - Release google-playintegrity1-cli v3.0.0+20220305 ([`bea571f`](https://github.com/Byron/google-apis-rs/commit/bea571f4351e7b8e13b49f6ee6f6eb483e8554d1))
    - Release google-playcustomapp1-cli v3.0.0+20220305 ([`30bcba4`](https://github.com/Byron/google-apis-rs/commit/30bcba48ba62e9c1690ed8eaff2b87bf306f519a))
    - Release google-playablelocations3-cli v3.0.0+20200707 ([`a687442`](https://github.com/Byron/google-apis-rs/commit/a687442ab64f79e21fbabc271cb7d032be106a4e))
    - Release google-photoslibrary1-cli v3.0.0+20220303 ([`02fbb90`](https://github.com/Byron/google-apis-rs/commit/02fbb90f3f7adef14c488f4ad143fb41f0d1be7d))
    - Release google-people1-cli v3.0.0+20220303 ([`aea7b14`](https://github.com/Byron/google-apis-rs/commit/aea7b14f88a24ba1589e990700cacb3151f88052))
    - Release google-paymentsresellersubscription1-cli v3.0.0+20220307 ([`9aed517`](https://github.com/Byron/google-apis-rs/commit/9aed517a5e47e2d507c78bca7915f58a0a226157))
    - Release google-partners2-cli v3.0.0+20180925 ([`aeeebc1`](https://github.com/Byron/google-apis-rs/commit/aeeebc173e5179bc184c157229db731b6f0d0f25))
    - Release google-pagespeedonline5-cli v3.0.0+20220302 ([`6114d8f`](https://github.com/Byron/google-apis-rs/commit/6114d8f8fa2caf1a5ad333daceb6976a81aa8f5b))
    - Release google-pagespeedonline4-cli v3.0.0+20191206 ([`d76bfd1`](https://github.com/Byron/google-apis-rs/commit/d76bfd1301a8ecd8fef32fef1459ed1e1b1aff1a))
    - Release google-pagespeedonline2-cli v3.0.0+20191206 ([`85d7d9d`](https://github.com/Byron/google-apis-rs/commit/85d7d9de5f5fc8371675c2be68b4b230b534ff3c))
    - Release google-oslogin1_beta-cli v3.0.0+20220228 ([`d7c1e86`](https://github.com/Byron/google-apis-rs/commit/d7c1e864a96a52c286a21cbe4b67eaf1ebcfe75b))
    - Release google-oslogin1-cli v3.0.0+20220228 ([`32155b2`](https://github.com/Byron/google-apis-rs/commit/32155b2f8b558776fda54d22c37769c64ed9f33b))
    - Release google-orgpolicy2-cli v3.0.0+20220305 ([`65ba64f`](https://github.com/Byron/google-apis-rs/commit/65ba64f8cbc88f0694f87c0e0edf91ed711c941f))
    - Release google-ondemandscanning1-cli v3.0.0+20220228 ([`5a505e6`](https://github.com/Byron/google-apis-rs/commit/5a505e6654ce428364ad9f063a1cf90b7c151442))
    - Release google-notebooks1-cli v3.0.0+20220224 ([`3ea092d`](https://github.com/Byron/google-apis-rs/commit/3ea092d61c63ac61f1ecba0bd999a5edb020f1e3))
    - Release google-networkservices1-cli v3.0.0+20220222 ([`144bb11`](https://github.com/Byron/google-apis-rs/commit/144bb116801794627e0476f26f20447bcdcb3008))
    - Release google-networksecurity1-cli v3.0.0+20220223 ([`64460f7`](https://github.com/Byron/google-apis-rs/commit/64460f71d31aa3ae9e4d3d9ee3936f46cebdd33d))
    - Release google-networkmanagement1-cli v3.0.0+20220223 ([`8187bac`](https://github.com/Byron/google-apis-rs/commit/8187bac2e1b367abdf36c2c295dfb3140e479edc))
    - Release google-networkconnectivity1-cli v3.0.0+20220210 ([`9a7f91f`](https://github.com/Byron/google-apis-rs/commit/9a7f91fdac5ef350ce95b4573635240d4a7d6ad5))
    - Release google-networkconnectivity1_alpha1-cli v3.0.0+20220210 ([`67c678c`](https://github.com/Byron/google-apis-rs/commit/67c678c6e053ee20252049d032d1b476dd71969b))
    - Release google-mybusinessverifications1-cli v3.0.0+20220305 ([`c5bddd0`](https://github.com/Byron/google-apis-rs/commit/c5bddd09e265776ea4c1432ad73f1ad99db7c137))
    - Release google-mybusinessplaceactions1-cli v3.0.0+20220305 ([`cdbed41`](https://github.com/Byron/google-apis-rs/commit/cdbed4127de9ceeaf12346484cc3b020cc701090))
    - Release google-mybusinessnotifications1-cli v3.0.0+20220305 ([`f510703`](https://github.com/Byron/google-apis-rs/commit/f5107031baae0ede56bc7805a7c162238f82fb98))
    - Release google-mybusinesslodging1-cli v3.0.0+20220305 ([`30dbb3f`](https://github.com/Byron/google-apis-rs/commit/30dbb3f4a1e6c6291d9b9ac4f7af5b835c764f26))
    - Release google-mybusinessbusinessinformation1-cli v3.0.0+20220305 ([`289f1a2`](https://github.com/Byron/google-apis-rs/commit/289f1a234c3029109ec1d6247bed289e51ad3d27))
    - Release google-mybusinessbusinesscalls1-cli v3.0.0+20220305 ([`7d58527`](https://github.com/Byron/google-apis-rs/commit/7d5852760df590df81f2a6be2d0fb1b379653a40))
    - Release google-mybusinessaccountmanagement1-cli v3.0.0+20220305 ([`28df5de`](https://github.com/Byron/google-apis-rs/commit/28df5de530dde1eb8ff3249ccd4615e21999749a))
    - Release google-mybusiness4-cli v3.0.0+0 ([`27da416`](https://github.com/Byron/google-apis-rs/commit/27da4168363cd2bb257909858b5fd3ed41273b2b))
    - Release google-monitoring3-cli v3.0.0+20220218 ([`40bd1e0`](https://github.com/Byron/google-apis-rs/commit/40bd1e0579f4d08d6f8065a6d0c3bcc0c039fc3a))
    - Release google-ml1-cli v3.0.0+20220212 ([`71df6da`](https://github.com/Byron/google-apis-rs/commit/71df6dac924de4ec5360d9a832da70618497fd5a))
    - Release google-mirror1-cli v3.0.0+20190424 ([`5322b94`](https://github.com/Byron/google-apis-rs/commit/5322b943db8e666093a6f72c1d5a06878d475f3d))
    - Release google-metastore1_beta-cli v3.0.0+20220222 ([`53ad9c6`](https://github.com/Byron/google-apis-rs/commit/53ad9c6fce3236aa05aeac5b616845fff1e7ec27))
    - Release google-memcache1_beta2-cli v3.0.0+20220224 ([`36a5e3a`](https://github.com/Byron/google-apis-rs/commit/36a5e3abdb2caae44f897dcf6ba2ebf491a9b726))
    - Release google-memcache1-cli v3.0.0+20220224 ([`e312e11`](https://github.com/Byron/google-apis-rs/commit/e312e11841db85a6fc772831bd0403064b0d69b8))
    - Release google-manufacturers1-cli v3.0.0+20220303 ([`aec8611`](https://github.com/Byron/google-apis-rs/commit/aec86119c8960f132de579cb7196912eb45e67f1))
    - Release google-manager1_beta2-cli v3.0.0+20140915 ([`619a434`](https://github.com/Byron/google-apis-rs/commit/619a434eb6aa796f7d3f8288ea883099b05ff89e))
    - Release google-managedidentities1-cli v3.0.0+20220216 ([`199d7f7`](https://github.com/Byron/google-apis-rs/commit/199d7f7c63031de84a1bc0b3da85f8c3b224aefb))
    - Release google-logging2_beta1-cli v3.0.0+20190325 ([`735f637`](https://github.com/Byron/google-apis-rs/commit/735f6377ff0c518fb0aa31c5028414d5ba4e2323))
    - Release google-logging2-cli v3.0.0+20220225 ([`5d4314c`](https://github.com/Byron/google-apis-rs/commit/5d4314c088ef47450dda477a13c1c8d5faa273aa))
    - Release google-localservices1-cli v3.0.0+20220305 ([`8e5ed41`](https://github.com/Byron/google-apis-rs/commit/8e5ed41070e149fcbcf6ed871b4ba0e2337189aa))
    - Release google-lifesciences2_beta-cli v3.0.0+20220211 ([`8a4750c`](https://github.com/Byron/google-apis-rs/commit/8a4750c581ccae366c7654e7a3f80370bf9d4141))
    - Release google-licensing1-cli v3.0.0+20220305 ([`0865142`](https://github.com/Byron/google-apis-rs/commit/0865142e32383ac6adebc90c7e85998a607141e9))
    - Release google-libraryagent1-cli v3.0.0+20220305 ([`6f2ddcc`](https://github.com/Byron/google-apis-rs/commit/6f2ddcc5b4b1ce759bafa772dc18f477b5426a5a))
    - Release google-language1_beta1-cli v3.0.0+20220218 ([`7dccd9c`](https://github.com/Byron/google-apis-rs/commit/7dccd9c905c00fe380d3d14313f297f71a20df12))
    - Release google-language1-cli v3.0.0+20220218 ([`7143c71`](https://github.com/Byron/google-apis-rs/commit/7143c717815533aff0a2138f2e4e043f440912ee))
    - Release google-keep1-cli v3.0.0+20220301 ([`b411ccd`](https://github.com/Byron/google-apis-rs/commit/b411ccde3e51d5368e846c13bae6ec17cb7ce5f0))
    - Release google-jobs4-cli v3.0.0+20220211 ([`4a32fc5`](https://github.com/Byron/google-apis-rs/commit/4a32fc5402d832575fe98ce5d5d903ce206c8f74))
    - Release google-jobs3-cli v3.0.0+20220211 ([`52c179b`](https://github.com/Byron/google-apis-rs/commit/52c179ba4143eb95ac79320fba1ab22c85948aed))
    - Release google-indexing3-cli v3.0.0+20220126 ([`054b91b`](https://github.com/Byron/google-apis-rs/commit/054b91b5b9764383482dc7f86350696f447ab632))
    - Release google-ids1-cli v3.0.0+20220221 ([`dd56efd`](https://github.com/Byron/google-apis-rs/commit/dd56efd1302a4ca7c48e33d02ea2e1ee5b576e47))
    - Release google-identitytoolkit3-cli v3.0.0+20180723 ([`aa1fcce`](https://github.com/Byron/google-apis-rs/commit/aa1fcceee0c674b586d3f1e26f88613893cf98b6))
    - Release google-ideahub1_beta-cli v3.0.0+20220305 ([`2a433d5`](https://github.com/Byron/google-apis-rs/commit/2a433d583d4e68a026466cbd20ff502b838709a7))
    - Release google-iap1_beta1-cli v3.0.0+20220225 ([`fed1ea6`](https://github.com/Byron/google-apis-rs/commit/fed1ea606e6e0594a3584a978e8c0e63872971f9))
    - Release google-iap1-cli v3.0.0+20220225 ([`7eefb11`](https://github.com/Byron/google-apis-rs/commit/7eefb114cdb4643b591217fb30cce50713078e55))
    - Release google-iamcredentials1-cli v3.0.0+20220225 ([`1cbde2e`](https://github.com/Byron/google-apis-rs/commit/1cbde2eb8ec502c8256a5833244e7daaf682c85e))
    - Release google-iam1-cli v3.0.0+20220224 ([`38b4ebe`](https://github.com/Byron/google-apis-rs/commit/38b4ebe2f7c8d3d813369aa0e4f481e0f226a596))
    - Release google-healthcare1_beta1-cli v3.0.0+20220223 ([`9dbd012`](https://github.com/Byron/google-apis-rs/commit/9dbd01273b6815cf16fa4aa2249b3535773c159d))
    - Release google-healthcare1-cli v3.0.0+20220223 ([`39b3d46`](https://github.com/Byron/google-apis-rs/commit/39b3d46e8909c81119940c166007836f0a637ef0))
    - Release google-groupssettings1-cli v3.0.0+20220224 ([`55e9a67`](https://github.com/Byron/google-apis-rs/commit/55e9a67cc9754c4074f9022bcbef3582ec819876))
    - Release google-groupsmigration1-cli v3.0.0+20220226 ([`a097284`](https://github.com/Byron/google-apis-rs/commit/a097284ba7778ca8cbe28d247aba32ff175be3be))
    - Release google-gmailpostmastertools1_beta1-cli v3.0.0+20220305 ([`9901f8b`](https://github.com/Byron/google-apis-rs/commit/9901f8bbf79d2c3a19546d21e7e407964784901a))
    - Release google-gmailpostmastertools1-cli v3.0.0+20220305 ([`83f65f2`](https://github.com/Byron/google-apis-rs/commit/83f65f276ecad43c59c0ab0148eb4f25b6525119))
    - Release google-gmail1-cli v3.0.0+20220228 ([`45b7239`](https://github.com/Byron/google-apis-rs/commit/45b723938d0bac11a10425c93412077565ebf998))
    - Release google-gkehub1-cli v3.0.0+20220211 ([`6c96809`](https://github.com/Byron/google-apis-rs/commit/6c9680951e9c8b0d9d19e9ad83af342651984d9d))
    - Release google-genomics1-cli v3.0.0+20210324 ([`36e181c`](https://github.com/Byron/google-apis-rs/commit/36e181c544303f93028f57bb34730ab0119c4683))
    - Release google-gan1_beta1-cli v3.0.0+20130205 ([`2a81f19`](https://github.com/Byron/google-apis-rs/commit/2a81f19c692a5c3eccfde79739a55ba1e6559b0a))
    - Release google-gamesmanagement1_management-cli v3.0.0+20220217 ([`e675b10`](https://github.com/Byron/google-apis-rs/commit/e675b105e1f8afd0032ee11ebdaed776cc3fd736))
    - Release google-gameservices1-cli v3.0.0+20220223 ([`442eefd`](https://github.com/Byron/google-apis-rs/commit/442eefd5e5be42ba4c4c3f83480afafd6c5f8d64))
    - Release google-gamesconfiguration1_configuration-cli v3.0.0+20220217 ([`892beec`](https://github.com/Byron/google-apis-rs/commit/892beec884d549abc25a8fe0d613587e620a665f))
    - Release google-games1-cli v3.0.0+20220217 ([`1230bcc`](https://github.com/Byron/google-apis-rs/commit/1230bcc21ee01ca8c36ea50c7f3cb2de3e7beaa2))
    - Release google-fusiontables2-cli v3.0.0+20171117 ([`cb39969`](https://github.com/Byron/google-apis-rs/commit/cb39969846d8a41956077b17e85ee9d7b6d729d3))
    - Release google-fitness1-cli v3.0.0+20220302 ([`1032fe8`](https://github.com/Byron/google-apis-rs/commit/1032fe8b109fe0a0d4df8b799b64fff40b167449))
    - Release google-firestore1_beta1-cli v3.0.0+20220221 ([`7d7f945`](https://github.com/Byron/google-apis-rs/commit/7d7f945fdf54dc76b9d34c6f4547fb1acd5ea2d0))
    - Release google-firestore1-cli v3.0.0+20220221 ([`8453a09`](https://github.com/Byron/google-apis-rs/commit/8453a0963caf13cd7cf61c80095c7426eda59d47))
    - Release google-firebasestorage1_beta-cli v3.0.0+20220218 ([`46ea989`](https://github.com/Byron/google-apis-rs/commit/46ea98959ac5e1417232ca0acb4d899dcef2fc31))
    - Release google-firebaseremoteconfig1-cli v3.0.0+20171129 ([`74bb743`](https://github.com/Byron/google-apis-rs/commit/74bb7439ec22fb64fbf1397e536d15a4979cfe8a))
    - Release google-firebaseml1-cli v3.0.0+20220302 ([`30af8b7`](https://github.com/Byron/google-apis-rs/commit/30af8b78e0c763c01d6d7d299ff0e0b61fcff328))
    - Release google-firebasehosting1_beta1-cli v3.0.0+20220212 ([`92099c0`](https://github.com/Byron/google-apis-rs/commit/92099c0d85e7b7869078031026be802bf9024d9a))
    - Release google-firebasehosting1-cli v3.0.0+20220212 ([`ec6e4df`](https://github.com/Byron/google-apis-rs/commit/ec6e4dfeb3cfaaf40bca3d0f1769be405f5109a6))
    - Release google-firebasedynamiclinks1-cli v3.0.0+20220228 ([`ff21991`](https://github.com/Byron/google-apis-rs/commit/ff219912d6d793d60ee6f1e502918f487844b912))
    - Release google-firebasedatabase1_beta-cli v3.0.0+20220304 ([`ddf56e8`](https://github.com/Byron/google-apis-rs/commit/ddf56e8ee64b658e6366ef23efaf7006db47f0c1))
    - Release google-firebaseappcheck1_beta-cli v3.0.0+20220225 ([`ed1c70b`](https://github.com/Byron/google-apis-rs/commit/ed1c70b3900007b1623e1f36ec8cc55d0fa68be6))
    - Release google-firebase1_beta1-cli v3.0.0+20220304 ([`4d944ac`](https://github.com/Byron/google-apis-rs/commit/4d944acff8014fb0ee6fdb5d211bf60db8a7ff60))
    - Release google-file1_beta1-cli v3.0.0+20220214 ([`aaadccd`](https://github.com/Byron/google-apis-rs/commit/aaadccd79089c4f00a5497f176edf7cc8508de05))
    - Release google-file1-cli v3.0.0+20220214 ([`fc20c4d`](https://github.com/Byron/google-apis-rs/commit/fc20c4d612df7b5568df335edb5ad83f61dab521))
    - Release google-fcmdata1_beta1-cli v3.0.0+20220305 ([`09c481f`](https://github.com/Byron/google-apis-rs/commit/09c481fc0b5cf05a4a5eb2590c363841f9337498))
    - Release google-fcm1-cli v3.0.0+20220228 ([`45bf32c`](https://github.com/Byron/google-apis-rs/commit/45bf32c1d624e4a3c776d9f0745dfa3420da7917))
    - Release google-factchecktools1_alpha1-cli v3.0.0+20220305 ([`9caf13d`](https://github.com/Byron/google-apis-rs/commit/9caf13d58aa339cf720949c8cd6a3fe6d4e330cb))
    - Release google-eventarc1-cli v3.0.0+20220301 ([`da3cb38`](https://github.com/Byron/google-apis-rs/commit/da3cb38ca3096ebe2a4eceffcff6ad80be3059bd))
    - Release google-essentialcontacts1-cli v3.0.0+20220227 ([`b74ce2d`](https://github.com/Byron/google-apis-rs/commit/b74ce2dbe32e3113063cddc8ed5f2e379d10d01e))
    - Release google-driveactivity2-cli v3.0.0+20220301 ([`aca8dd6`](https://github.com/Byron/google-apis-rs/commit/aca8dd6c2702c6644bde2247320f4796bcdabced))
    - Release google-drive3-cli v3.0.0+20220225 ([`8fcf81e`](https://github.com/Byron/google-apis-rs/commit/8fcf81e04fd6457a8d57656481465ad4b3704a10))
    - Release google-drive2-cli v3.0.0+20220225 ([`ecbda30`](https://github.com/Byron/google-apis-rs/commit/ecbda303a20c154ba99b995c522b0726ac02eedb))
    - Release google-doubleclicksearch2-cli v3.0.0+20220301 ([`871ab7c`](https://github.com/Byron/google-apis-rs/commit/871ab7cae99260bd6ada254a16b6224b6db4fbc4))
    - Release google-doubleclickbidmanager1d1-cli v3.0.0+20220302 ([`d4f61dc`](https://github.com/Byron/google-apis-rs/commit/d4f61dcaee9468c16aca11a73c1d1dbba0691c25))
    - Release google-doubleclickbidmanager1-cli v3.0.0+20210323 ([`365d60f`](https://github.com/Byron/google-apis-rs/commit/365d60f073ac4da6fae16c27aafc56b987f843c4))
    - Release google-domainsrdap1-cli v3.0.0+20220307 ([`4fbdce0`](https://github.com/Byron/google-apis-rs/commit/4fbdce093bfddca47324373b6344778c3a6c7f1c))
    - Release google-domains1-cli v3.0.0+20220128 ([`dad5d0d`](https://github.com/Byron/google-apis-rs/commit/dad5d0d435d1fc866cd65c0c4ed90675634db55e))
    - Release google-domains1_beta1-cli v3.0.0+20220128 ([`65abd11`](https://github.com/Byron/google-apis-rs/commit/65abd11277607294addddab052ec31b433ec04a2))
    - Release google-documentai1_beta2-cli v3.0.0+20220226 ([`53b1bf6`](https://github.com/Byron/google-apis-rs/commit/53b1bf6da613eaaa0008dfb55b9c83b66e43a0d2))
    - Release google-documentai1-cli v3.0.0+20220226 ([`c838390`](https://github.com/Byron/google-apis-rs/commit/c838390a9263267c5e39abc8119789301c96bfa2))
    - Release google-docs1-cli v3.0.0+20220301 ([`29304bb`](https://github.com/Byron/google-apis-rs/commit/29304bb0d4c6b75fcdd89a3e0f676c4cc6fc43d6))
    - Release google-dns2-cli v3.0.0+20220217 ([`3fbfd44`](https://github.com/Byron/google-apis-rs/commit/3fbfd44d6d7c11a78ccc40651ddd1eef27e281af))
    - Release google-dns1-cli v3.0.0+20220217 ([`921e868`](https://github.com/Byron/google-apis-rs/commit/921e8685d6e333db1071b994d5f998efa7147ff3))
    - Release google-dlp2_beta1-cli v3.0.0+20171205 ([`4ef80c6`](https://github.com/Byron/google-apis-rs/commit/4ef80c619d51ff8d1506eef270390786f517f4c6))
    - Release google-dlp2-cli v3.0.0+20220227 ([`5cbc671`](https://github.com/Byron/google-apis-rs/commit/5cbc671ca80deeaac8c36823adb8187b5d6820f5))
    - Release google-displayvideo1-cli v3.0.0+20220303 ([`4721f90`](https://github.com/Byron/google-apis-rs/commit/4721f90089d5d6f9f872db9b3c16f402b5bfc3cd))
    - Release google-discovery1-cli v3.0.0+20200806 ([`41168fe`](https://github.com/Byron/google-apis-rs/commit/41168fea601aa90196530e1b080c2ed414ac07d9))
    - Release google-digitalassetlinks1-cli v3.0.0+20220301 ([`6f7c3c5`](https://github.com/Byron/google-apis-rs/commit/6f7c3c555782bacfc0e56eefd4de5d8a8a495ab4))
    - Release google-dialogflow3-cli v3.0.0+20220228 ([`933ed26`](https://github.com/Byron/google-apis-rs/commit/933ed26cf40f46686b7051318aaab72dee56070b))
    - Release google-dialogflow2_beta1-cli v3.0.0+20220228 ([`633ccc3`](https://github.com/Byron/google-apis-rs/commit/633ccc3b7abf68e8da336faf05db5173f84bf5e2))
    - Release google-dialogflow2-cli v3.0.0+20220228 ([`0f1ec58`](https://github.com/Byron/google-apis-rs/commit/0f1ec5838fd0471b9159922fab884058f7a73f5c))
    - Release google-dfareporting3d5-cli v3.0.0+20220104 ([`164e955`](https://github.com/Byron/google-apis-rs/commit/164e9555f782cc9c991c6c292b60448eddadeb25))
    - Release google-dfareporting3d4-cli v3.0.0+20220104 ([`c2860cc`](https://github.com/Byron/google-apis-rs/commit/c2860cc046911a2c9c5d36497fc20416f4724c23))
    - Release google-dfareporting3d3-cli v3.0.0+20220104 ([`6e55faa`](https://github.com/Byron/google-apis-rs/commit/6e55faa75f87571b985888551ad8480a68a04853))
    - Release google-dfareporting3d2-cli v3.0.0+20190531 ([`da72d9c`](https://github.com/Byron/google-apis-rs/commit/da72d9cd8b805fd5503f978132cd7487a90b48ad))
    - Release google-dfareporting3-cli v3.0.0+20180830 ([`245f4f6`](https://github.com/Byron/google-apis-rs/commit/245f4f6e89de2addf51f8ded4169d95558a4c519))
    - Release google-dfareporting2d8-cli v3.0.0+20180830 ([`e792124`](https://github.com/Byron/google-apis-rs/commit/e792124cc1300b2c42b5b2441f4e9d42a3b43731))
    - Release google-deploymentmanager2_beta2-cli v3.0.0+20160201 ([`c658fe9`](https://github.com/Byron/google-apis-rs/commit/c658fe97f6043b32db96e35ef0a0fdbc99b5e36b))
    - Release google-deploymentmanager2-cli v3.0.0+20220225 ([`d3fc904`](https://github.com/Byron/google-apis-rs/commit/d3fc9048c4e7a69743f0dab74dc012bf3c2af6fe))
    - Release google-datastream1-cli v3.0.0+20220207 ([`01f8a74`](https://github.com/Byron/google-apis-rs/commit/01f8a74232dc380db6e7677c0a03fd5befc321c8))
    - Release google-datastore1_beta3-cli v3.0.0+20220221 ([`94164f3`](https://github.com/Byron/google-apis-rs/commit/94164f3f9f3d914213eab7c5a29ac5c96230157c))
    - Release google-datastore1-cli v3.0.0+20220221 ([`d64a6d2`](https://github.com/Byron/google-apis-rs/commit/d64a6d2efc0a384dd48e1066740d14a64d9f5757))
    - Release google-dataproc1-cli v3.0.0+20220224 ([`60dd65a`](https://github.com/Byron/google-apis-rs/commit/60dd65a390a7b534dc563f99f1524d9cb2c6d3de))
    - Release google-dataplex1-cli v3.0.0+20220223 ([`e200216`](https://github.com/Byron/google-apis-rs/commit/e200216c355fe2549f9afc07720e7cedfe74d24a))
    - Release google-datapipelines1-cli v3.0.0+20220218 ([`6fc9a2c`](https://github.com/Byron/google-apis-rs/commit/6fc9a2c4eb18056a150a87d02cf7328a85580d71))
    - Release google-datamigration1-cli v3.0.0+20220216 ([`a5327d2`](https://github.com/Byron/google-apis-rs/commit/a5327d25ad083c4d5d34cbb5e37dd31a65a8896e))
    - Release google-datalabeling1_beta1-cli v3.0.0+20220301 ([`8f0f854`](https://github.com/Byron/google-apis-rs/commit/8f0f8544a113b95b90cfcf3720dcdb63c4191c3f))
    - Release google-datafusion1_beta1-cli v3.0.0+20211028 ([`06129f2`](https://github.com/Byron/google-apis-rs/commit/06129f2cd4343ab40859ca374e7d5c02dd1ff5f2))
    - Release google-datafusion1-cli v3.0.0+20211028 ([`872e8de`](https://github.com/Byron/google-apis-rs/commit/872e8de8656d4543453d458a7fcc75de2dc13cb1))
    - Release google-datacatalog1-cli v3.0.0+20220224 ([`36e5c48`](https://github.com/Byron/google-apis-rs/commit/36e5c4887f0a9b429afa8eac104812bd08b718fc))
    - Release google-datacatalog1_beta1-cli v3.0.0+20220224 ([`a5c9397`](https://github.com/Byron/google-apis-rs/commit/a5c93977f5afb1601cf5b2b6cc354c99c7a52952))
    - Release google-customsearch1-cli v3.0.0+20220305 ([`4ea0271`](https://github.com/Byron/google-apis-rs/commit/4ea0271c7212c607bd5302031162cbe84566005a))
    - Release google-coordinate1-cli v3.0.0+20150811 ([`006e60f`](https://github.com/Byron/google-apis-rs/commit/006e60f63342c6c345c2270c96711ade1c9d2a10))
    - Release google-content2_sandbox-cli v3.0.0+20181009 ([`851ac8c`](https://github.com/Byron/google-apis-rs/commit/851ac8c7c7a6c055e1d124ab810d24e81056ab37))
    - Release google-content2-cli v3.0.0+20220303 ([`73455ec`](https://github.com/Byron/google-apis-rs/commit/73455ec37e2590f863a7ae488b3448a22947a93c))
    - Release google-containeranalysis1-cli v3.0.0+20220225 ([`5ba9e19`](https://github.com/Byron/google-apis-rs/commit/5ba9e19afee7444177d36b724bd296613aac88fc))
    - Release google-containeranalysis1_beta1-cli v3.0.0+20220225 ([`79c20d2`](https://github.com/Byron/google-apis-rs/commit/79c20d2296262fac5985758a6e7ed7f7cd91e2d8))
    - Release google-container1-cli v3.0.0+20220215 ([`c807cac`](https://github.com/Byron/google-apis-rs/commit/c807cacef0590ba9d831c8c58374eecfb1350be8))
    - Release google-contactcenterinsights1-cli v3.0.0+20220227 ([`b4f1dd2`](https://github.com/Byron/google-apis-rs/commit/b4f1dd2c99e8d502dfd6fbeed28da94eb95aee41))
    - Release google-consumersurveys2-cli v3.0.0+20170407 ([`c3865bf`](https://github.com/Byron/google-apis-rs/commit/c3865bf18688f9b56701516b4b3f67592fb42872))
    - Release google-connectors1-cli v3.0.0+20220214 ([`84fbbc5`](https://github.com/Byron/google-apis-rs/commit/84fbbc5bb82f78ca217850b498f9ca5914670921))
    - Release google-compute1-cli v3.0.0+20220224 ([`be5b8a5`](https://github.com/Byron/google-apis-rs/commit/be5b8a51745defd1889d7b2ec6580761ad8393fb))
    - Release google-composer1-cli v3.0.0+20220224 ([`8539493`](https://github.com/Byron/google-apis-rs/commit/8539493c7a89552d252b643b8e42d5e1f49cdead))
    - Release google-commentanalyzer1_alpha1-cli v3.0.0+20200405 ([`a48f328`](https://github.com/Byron/google-apis-rs/commit/a48f328a4b3362389cd170a0e7e5428ccf9444bd))
    - Release google-clouduseraccountsvm_beta-cli v3.0.0+20160316 ([`410dd16`](https://github.com/Byron/google-apis-rs/commit/410dd16851a9653444b7404bd0780468359a4abd))
    - Release google-cloudtrace2-cli v3.0.0+20220224 ([`4170010`](https://github.com/Byron/google-apis-rs/commit/41700103d338444ccc7c28041744b32f2883b63b))
    - Release google-cloudtrace1-cli v3.0.0+20220224 ([`1b14aa4`](https://github.com/Byron/google-apis-rs/commit/1b14aa43e91f36004b45ec286a3fb977aa30910d))
    - Release google-cloudtasks2_beta3-cli v3.0.0+20220212 ([`2563163`](https://github.com/Byron/google-apis-rs/commit/25631637c299a9ff1d3b15e58c0e13b5383337ff))
    - Release google-cloudtasks2_beta2-cli v3.0.0+20220212 ([`0e696b6`](https://github.com/Byron/google-apis-rs/commit/0e696b631b214e2427233a9299b655469c2728d8))
    - Release google-cloudtasks2-cli v3.0.0+20220212 ([`563d98a`](https://github.com/Byron/google-apis-rs/commit/563d98a2f4284072640205ea034698e3d0c4dd37))
    - Release google-cloudsupport2_beta-cli v3.0.0+20220305 ([`162297b`](https://github.com/Byron/google-apis-rs/commit/162297b988c2709c67f599953664cf0aab94957a))
    - Release google-cloudshell1-cli v3.0.0+20220301 ([`9587351`](https://github.com/Byron/google-apis-rs/commit/9587351b4d43184566a28fa907d4b6714e951784))
    - Release google-cloudasset1_beta1-cli v3.0.0+20220225 ([`183ebd9`](https://github.com/Byron/google-apis-rs/commit/183ebd96573869d6ff46da50743ad5c09f4131c0))
    - Release google-cloudasset1-cli v3.0.0+20220225 ([`c067a92`](https://github.com/Byron/google-apis-rs/commit/c067a92ece35cbd6d38dd8a4268ab20dc31b6b08))
    - Release google-classroom1-cli v3.0.0+20220224 ([`1a0f398`](https://github.com/Byron/google-apis-rs/commit/1a0f398a8c287ea6d19d15f7065c73e9b4405d3f))
    - Release google-chromeuxreport1-cli v3.0.0+20220302 ([`cf3ccc6`](https://github.com/Byron/google-apis-rs/commit/cf3ccc6429ee8c0f991b765b063d49a2169328b3))
    - Release google-chromepolicy1-cli v3.0.0+20220305 ([`f8f5180`](https://github.com/Byron/google-apis-rs/commit/f8f518065d8fd0013b0992284f3ea0a5553e3a8c))
    - Release google-chromemanagement1-cli v3.0.0+20220305 ([`0f20009`](https://github.com/Byron/google-apis-rs/commit/0f20009397e45d9b64dead6f687a3cc98ebf9a67))
    - Release google-certificatemanager1-cli v3.0.0+20220214 ([`128dfb7`](https://github.com/Byron/google-apis-rs/commit/128dfb7acfc88138e5b096c3ec903f1f37847253))
    - Release google-calendar3-cli v3.0.0+20220217 ([`9d8f248`](https://github.com/Byron/google-apis-rs/commit/9d8f248f256a96cb6de77d2473eaa04dd85364f8))
    - Release google-books1-cli v3.0.0+20220301 ([`fd2b453`](https://github.com/Byron/google-apis-rs/commit/fd2b453b0b61ce95c60e5a026a5f4fcfc567a944))
    - Release google-blogger3-cli v3.0.0+20220305 ([`1d1e0d8`](https://github.com/Byron/google-apis-rs/commit/1d1e0d82cd7cb7ddeb3811f77e8ab22a4876dc3c))
    - Release google-binaryauthorization1_beta1-cli v3.0.0+20220225 ([`2a1f168`](https://github.com/Byron/google-apis-rs/commit/2a1f16837fa392c9f752dd62fabf648c8eaeab1b))
    - Release google-binaryauthorization1-cli v3.0.0+20220225 ([`a6018c2`](https://github.com/Byron/google-apis-rs/commit/a6018c22d83e0191d7e960183a87f7cef58bcfef))
    - Release google-billingbudgets1_beta1-cli v3.0.0+20220227 ([`70d2b67`](https://github.com/Byron/google-apis-rs/commit/70d2b67ce7b75f9dc699233ce6c9c1d7e6e04061))
    - Release google-billingbudgets1-cli v3.0.0+20220227 ([`4b741ad`](https://github.com/Byron/google-apis-rs/commit/4b741ad026f9306c4c534d6488ad72d8db963a26))
    - Release google-bigtableadmin2-cli v3.0.0+20220222 ([`c86e9a7`](https://github.com/Byron/google-apis-rs/commit/c86e9a752204051593e7f3a113f6c54084039a60))
    - Release google-bigqueryreservation1-cli v3.0.0+20220226 ([`939502c`](https://github.com/Byron/google-apis-rs/commit/939502cab459c0e695f308b3a3e7f2c32c3526f7))
    - Release google-bigquerydatatransfer1-cli v3.0.0+20220225 ([`28a1f9c`](https://github.com/Byron/google-apis-rs/commit/28a1f9cac5c3949983c8498aaeff7405b8e7f3cd))
    - Release google-bigqueryconnection1_beta1-cli v3.0.0+20220226 ([`1192068`](https://github.com/Byron/google-apis-rs/commit/1192068c4ddf6aeb04263379f99c2736b0574837))
    - Release google-bigquery2-cli v3.0.0+20220222 ([`f2667c9`](https://github.com/Byron/google-apis-rs/commit/f2667c95665f7f78febbeaa206252cfc3dc1b473))
    - Release google-baremetalsolution2-cli v3.0.0+20220209 ([`c86a530`](https://github.com/Byron/google-apis-rs/commit/c86a5306c9a3085ae228b9137fc86ae2f627a0aa))
    - Release google-autoscaler1_beta2-cli v3.0.0+20150629 ([`422782e`](https://github.com/Byron/google-apis-rs/commit/422782e1706f5f0f5c03efa9af079ca7bfcaabd9))
    - Release google-authorizedbuyersmarketplace1-cli v3.0.0+20220307 ([`18c345d`](https://github.com/Byron/google-apis-rs/commit/18c345db3f4a783f2c3004304017e382553c2fe9))
    - Release google-assuredworkloads1-cli v3.0.0+20220224 ([`042e22f`](https://github.com/Byron/google-apis-rs/commit/042e22fb7f49e8988dcc91e37f49106de6cfd4d4))
    - Release google-artifactregistry1_beta1-cli v3.0.0+20220225 ([`9cb256b`](https://github.com/Byron/google-apis-rs/commit/9cb256be924ddc883f4a564a0ff91aa3027f6530))
    - Release google-artifactregistry1-cli v3.0.0+20220225 ([`c9d2ca7`](https://github.com/Byron/google-apis-rs/commit/c9d2ca70346bae4decdd56bdc1173e9fb75cd760))
    - Release google-area120tables1_alpha1-cli v3.0.0+20220301 ([`56c5c4d`](https://github.com/Byron/google-apis-rs/commit/56c5c4d918eac86b39c31c9c72bfa937e7f4cce9))
    - Release google-appstate1-cli v3.0.0+20190627 ([`2d0ae73`](https://github.com/Byron/google-apis-rs/commit/2d0ae73b50131f361cf88da8dd98880dbf41d380))
    - Release google-appsactivity1-cli v3.0.0+20200628 ([`64f301a`](https://github.com/Byron/google-apis-rs/commit/64f301a42a95bcd0c328e3081cc38bfaa0dcc145))
    - Release google-appengine1_beta5-cli v3.0.0+20181005 ([`2c38d11`](https://github.com/Byron/google-apis-rs/commit/2c38d11a3efb3a6bdd8125ef9796790839daea25))
    - Release google-appengine1_beta4-cli v3.0.0+20181005 ([`17b63d3`](https://github.com/Byron/google-apis-rs/commit/17b63d3d579ecab85e9695d76b7a249ad0920aa1))
    - Release google-appengine1-cli v3.0.0+20220226 ([`854f6c2`](https://github.com/Byron/google-apis-rs/commit/854f6c261d8539fd2e3069166dba00db5d6874f4))
    - Release google-apikeys2-cli v3.0.0+20220305 ([`8a82ca8`](https://github.com/Byron/google-apis-rs/commit/8a82ca82bde2f10695fb4e0bf801cd73aecdaef2))
    - Release google-apigee1-cli v3.0.0+20220301 ([`65705b0`](https://github.com/Byron/google-apis-rs/commit/65705b03d8f8c0f84189db788430128e2f26ede7))
    - Release google-apigateway1-cli v3.0.0+20220223 ([`d811cdd`](https://github.com/Byron/google-apis-rs/commit/d811cdd73bf08605bc21de866ee6146fb77def00))
    - Release google-androidpublisher3-cli v3.0.0+20220307 ([`b9036ac`](https://github.com/Byron/google-apis-rs/commit/b9036accebd99881f55c4f7c0235cdb859a8b10d))
    - Release google-androidpublisher2-cli v3.0.0+20200331 ([`f6aa703`](https://github.com/Byron/google-apis-rs/commit/f6aa703354b524f7178fa94e617f36333e7bd6ee))
    - Release google-androidmanagement1-cli v3.0.0+20220302 ([`14126e6`](https://github.com/Byron/google-apis-rs/commit/14126e68615cdb427cb92dd729863c02ef7789f0))
    - Release google-androidenterprise1-cli v3.0.0+20220303 ([`774fb0a`](https://github.com/Byron/google-apis-rs/commit/774fb0a812cc5f3b93f05b494f3c856de055f271))
    - Release google-androiddeviceprovisioning1-cli v3.0.0+20220305 ([`b26a24d`](https://github.com/Byron/google-apis-rs/commit/b26a24de4e91a7b2dd35fd591a709321f035c49d))
    - Release google-analyticsreporting4-cli v3.0.0+20220215 ([`ab2daec`](https://github.com/Byron/google-apis-rs/commit/ab2daec176a47e1a0732e44227994bab545bd345))
    - Release google-analyticsdata1_beta-cli v3.0.0+20220303 ([`ac8485e`](https://github.com/Byron/google-apis-rs/commit/ac8485e79e1fcdf5e578e1a9a4ecaa36358270c6))
    - Release google-analyticsadmin1_alpha-cli v3.0.0+20220307 ([`3eff81c`](https://github.com/Byron/google-apis-rs/commit/3eff81cd1f1a1ab9aebb6c98e405227321ddfb93))
    - Release google-analytics3-cli v3.0.0+20190807 ([`51e8803`](https://github.com/Byron/google-apis-rs/commit/51e880306cf4b3f1296f75a8504e6e922f943078))
    - Release google-alertcenter1_beta1-cli v3.0.0+20220221 ([`fd88334`](https://github.com/Byron/google-apis-rs/commit/fd883349b15f13f53f0277231476b9356b4e3a76))
    - Release google-adsensehost4d1-cli v3.0.0+20200930 ([`31aa7ab`](https://github.com/Byron/google-apis-rs/commit/31aa7abbf532a35dc20e7e86a43f62a5074cc7fa))
    - Release google-adsense2-cli v3.0.0+20220304 ([`f189e35`](https://github.com/Byron/google-apis-rs/commit/f189e35184863df737bc3f7b6f7f671e70db0fee))
    - a bunch of publishes ([`36b7409`](https://github.com/Byron/google-apis-rs/commit/36b74097b68af7439a3c08e8fad422243e76126f))
    - Release google-adsense1d4-cli v3.0.0+20201002 ([`3bcceeb`](https://github.com/Byron/google-apis-rs/commit/3bcceebddab2c03c2f57fa292db441aba8e86885))
    - Release google-admob1-cli v3.0.0+20220303 ([`dbafd5f`](https://github.com/Byron/google-apis-rs/commit/dbafd5f10d50a08a2759c613b6d48fbb157cc0c7))
    - Release google-adexperiencereport1-cli v3.0.0+20220303 ([`9d596c2`](https://github.com/Byron/google-apis-rs/commit/9d596c2d09413924554a1766588c74feffd59f07))
    - Release google-adexchangeseller2-cli v3.0.0+20171101 ([`d0377e2`](https://github.com/Byron/google-apis-rs/commit/d0377e2201d2e7369709da0c116fefe9fb2a1982))
    - Release google-adexchangebuyer2_v2_beta1-cli v3.0.0+20220307 ([`85b44ff`](https://github.com/Byron/google-apis-rs/commit/85b44ff352b2bb350b2d7ff91b3af83e713b7517))
    - Release google-adexchangebuyer1d4-cli v3.0.0+20210330 ([`c9ef92f`](https://github.com/Byron/google-apis-rs/commit/c9ef92fe7c53bb459ff21610082d671f2e972f8b))
    - Release google-adexchangebuyer1d3-cli v3.0.0+20210330 ([`ada3573`](https://github.com/Byron/google-apis-rs/commit/ada35735c432afc67fba2e80c42464e181b04bc6))
    - Release google-accesscontextmanager1_beta-cli v3.0.0+20220301 ([`3457a58`](https://github.com/Byron/google-apis-rs/commit/3457a582f578241b82cf8c2927dc062413d12c5b))
    - Release google-accesscontextmanager1-cli v3.0.0+20220301 ([`db2b826`](https://github.com/Byron/google-apis-rs/commit/db2b826cf467a3bdec1938eb2bee9ec79baa1399))
    - Release google-accessapproval1_beta1-cli v3.0.0+20200708 ([`df27281`](https://github.com/Byron/google-apis-rs/commit/df27281218450029a36bb9dcba8c1b0a368366db))
    - Release google-accessapproval1-cli v3.0.0+20220225 ([`79a874b`](https://github.com/Byron/google-apis-rs/commit/79a874b56149ffe7d8d5d3f1ba78deb710e60e85))
    - Release google-acceleratedmobilepageurl1-cli v3.0.0+20220305 ([`70e9733`](https://github.com/Byron/google-apis-rs/commit/70e97331bba42368486fd2ed440c0a994ad05505))
    - Release google-abusiveexperiencereport1-cli v3.0.0+20220303 ([`264e855`](https://github.com/Byron/google-apis-rs/commit/264e855ed6cead23f7efc683d3b7e7325fc88734))
    - Release google-youtubereporting1 v3.0.0+20220305 ([`97afbd9`](https://github.com/Byron/google-apis-rs/commit/97afbd9fbd026bcb3c4e50c580c77ff62b5e5ee3))
    - Release google-youtube3 v3.0.0+20220303 ([`fa0e98a`](https://github.com/Byron/google-apis-rs/commit/fa0e98a15612df7cc2e58d3ceff9297343ce470a))
    - Release google-workflows1 v3.0.0+20220223 ([`dbeb920`](https://github.com/Byron/google-apis-rs/commit/dbeb920d9de49d4f21a7c40cb8dfa7e6285ac98a))
    - Release google-workflowexecutions1 v3.0.0+20220222 ([`2db607d`](https://github.com/Byron/google-apis-rs/commit/2db607dc685cb834af1b5c31bbd2a0a5965ba884))
    - Release google-webrisk1 v3.0.0+20220226 ([`9c8db56`](https://github.com/Byron/google-apis-rs/commit/9c8db56ddabb38aca5fb9f5bcd06ad97b87c8626))
    - Release google-webmasters3 v3.0.0+20190428 ([`2db2678`](https://github.com/Byron/google-apis-rs/commit/2db2678b038a013f7b157231a569b0ecf13d5ffb))
    - Release google-webfonts1 v3.0.0+20220215 ([`ac1c48f`](https://github.com/Byron/google-apis-rs/commit/ac1c48f2110b71562d8aba3d40035fdbaa44fb37))
    - Release google-vmmigration1 v3.0.0+20220225 ([`9548d51`](https://github.com/Byron/google-apis-rs/commit/9548d51831df0750cf516c30865d0cd0e58c5ea8))
    - Release google-vision1 v3.0.0+20220225 ([`9fca99a`](https://github.com/Byron/google-apis-rs/commit/9fca99a49fead8b5fac518e8c461f34e05c57abe))
    - Release google-videointelligence1_beta1 v3.0.0+20171122 ([`8b783b5`](https://github.com/Byron/google-apis-rs/commit/8b783b5cf4bb9d7f33c866580c46f1779216bd58))
    - Release google-videointelligence1 v3.0.0+20220225 ([`62383dd`](https://github.com/Byron/google-apis-rs/commit/62383ddbc7a3c3d8ad4779714bcd657147a1ffec))
    - Release google-versionhistory1 v3.0.0+20220307 ([`826baeb`](https://github.com/Byron/google-apis-rs/commit/826baeb0d5869b454d5d39d15df77fc42d41d818))
    - Release google-verifiedaccess1 v3.0.0+20220215 ([`1cffd41`](https://github.com/Byron/google-apis-rs/commit/1cffd41bde1ae2b1342b846a433406c24599d495))
    - Release google-vectortile1 v3.0.0+20210331 ([`5877dc9`](https://github.com/Byron/google-apis-rs/commit/5877dc911df9a90c974d0e4193b5ebe50d4892df))
    - Release google-vault1 v3.0.0+20220222 ([`ca81666`](https://github.com/Byron/google-apis-rs/commit/ca816660ba007916c33e037029c3548dfbe5a788))
    - Release google-urlshortener1 v3.0.0+20150519 ([`5b8a6e7`](https://github.com/Byron/google-apis-rs/commit/5b8a6e74a8d35b84a44c6ebce5a60a6b6dab3acd))
    - Release google-translate3 v3.0.0+20220121 ([`f89b59b`](https://github.com/Byron/google-apis-rs/commit/f89b59b48689e29887149162033f16939a3ce867))
    - Release google-translate2 v3.0.0+20170525 ([`5820adb`](https://github.com/Byron/google-apis-rs/commit/5820adbfb194a8233cb702fa365f49bacb93b8b8))
    - Release google-transcoder1 v3.0.0+20220201 ([`c789be4`](https://github.com/Byron/google-apis-rs/commit/c789be409550077a86b19dacfeb227453951fff0))
    - Release google-transcoder1_beta1 v3.0.0+20210323 ([`9ab8a6d`](https://github.com/Byron/google-apis-rs/commit/9ab8a6d29cc46ab413b278a8989a31e7ff8d4662))
    - Release google-tpu1_alpha1 v3.0.0+20220301 ([`c772b90`](https://github.com/Byron/google-apis-rs/commit/c772b902243acd7f047fabac64b5ff7ac249eb77))
    - Release google-tpu1 v3.0.0+20220301 ([`8722fd5`](https://github.com/Byron/google-apis-rs/commit/8722fd51d6bd7ab2e07b95e1c5e73bba8c2c85bb))
    - Release google-texttospeech1 v3.0.0+20220228 ([`99a84ad`](https://github.com/Byron/google-apis-rs/commit/99a84ad36eec6d5acc35c2484e4a58a9dd93587c))
    - Release google-testing1 v3.0.0+20220301 ([`51ef23b`](https://github.com/Byron/google-apis-rs/commit/51ef23b65243e07dd8a2e51431e71bd4815e903d))
    - Release google-tasks1 v3.0.0+20220305 ([`fc81df5`](https://github.com/Byron/google-apis-rs/commit/fc81df5e4b1dcb593b372b08a9a53968c918986e))
    - Release google-taskqueue1_beta2 v3.0.0+20160428 ([`5b90564`](https://github.com/Byron/google-apis-rs/commit/5b90564db685793f4b4abaecedf7ac99db07e474))
    - Release google-tagmanager2 v3.0.0+20220301 ([`2ee90e1`](https://github.com/Byron/google-apis-rs/commit/2ee90e1e8748d2b05cb729702845942da0043f05))
    - Release google-tagmanager1 v3.0.0+20220301 ([`fd3525e`](https://github.com/Byron/google-apis-rs/commit/fd3525ea0b44e074b5bb09c549babed52b359ed8))
    - Release google-surveys2 v3.0.0+20180508 ([`d07ffe7`](https://github.com/Byron/google-apis-rs/commit/d07ffe7832215b0ce8ccc50f579819a06027e3a8))
    - Release google-sts1 v3.0.0+20220227 ([`a74e9ac`](https://github.com/Byron/google-apis-rs/commit/a74e9ac1427db48bcd2d4c751a70cff589fbdc7c))
    - Release google-storagetransfer1 v3.0.0+20220223 ([`3fd2150`](https://github.com/Byron/google-apis-rs/commit/3fd2150aa2ae1a17223a9babf947fa77aa322b92))
    - Release google-storage1 v3.0.0+20220228 ([`84f1458`](https://github.com/Byron/google-apis-rs/commit/84f1458deca432426023906ec83f5e9d02d92347))
    - Release google-sqladmin1 v3.0.0+20220226 ([`4ba8deb`](https://github.com/Byron/google-apis-rs/commit/4ba8deb4d7b943d2db5c4f87e94b57d7a27f1ad2))
    - Release google-sqladmin1_beta4 v3.0.0+20220226 ([`f64525d`](https://github.com/Byron/google-apis-rs/commit/f64525d7965305be0fbe166f277df5f8323f51f0))
    - Release google-sql1_beta4 v3.0.0+20200331 ([`83ea447`](https://github.com/Byron/google-apis-rs/commit/83ea44721635ba8950fc2d380bf969491d674a19))
    - Release google-speech1_beta1 v3.0.0+20181005 ([`c99c0e9`](https://github.com/Byron/google-apis-rs/commit/c99c0e9f0322cfa358d8a6321d922e50c7594145))
    - Release google-speech1 v3.0.0+20220221 ([`4f70dc7`](https://github.com/Byron/google-apis-rs/commit/4f70dc7edbfd6220f50edc7d747dc948d0cb2c01))
    - Release google-spectrum1_explorer v3.0.0+20170306 ([`c18e8b2`](https://github.com/Byron/google-apis-rs/commit/c18e8b26b9cf1f2179cae1fa2594c8f62bf036a8))
    - Release google-sourcerepo1 v3.0.0+20220217 ([`08049bf`](https://github.com/Byron/google-apis-rs/commit/08049bf8d8c1c83a63b6e016def5125742870128))
    - Release google-smartdevicemanagement1 v3.0.0+20220302 ([`ba4675c`](https://github.com/Byron/google-apis-rs/commit/ba4675cff47bdf9d09a88cd0a65849e3bbb7e87e))
    - Release google-siteverification1 v3.0.0+20191119 ([`c7f93d2`](https://github.com/Byron/google-apis-rs/commit/c7f93d283adf82584193b1eb72166b842237a57e))
    - Release google-sheets4 v3.0.0+20220221 ([`89ce89d`](https://github.com/Byron/google-apis-rs/commit/89ce89d1148f6f42db43d5cded6a5c1aa06d5c68))
    - Release google-serviceregistryalpha v3.0.0+20160401 ([`e3c5528`](https://github.com/Byron/google-apis-rs/commit/e3c55280e0b0d34e0579c9b68405793e9cc03cac))
    - Release google-servicedirectory1_beta1 v3.0.0+20220224 ([`61f55f6`](https://github.com/Byron/google-apis-rs/commit/61f55f6cb0d1e2248de95a7bbf8fd7d6896c8230))
    - Release google-servicedirectory1 v3.0.0+20220224 ([`a038f55`](https://github.com/Byron/google-apis-rs/commit/a038f5542f70fa93cf4888aca2a4d5538d1ecab2))
    - Release google-servicecontrol2 v3.0.0+20220227 ([`df0feb7`](https://github.com/Byron/google-apis-rs/commit/df0feb728c308484fc43f0ca8a008969411efcc9))
    - Release google-servicecontrol1 v3.0.0+20220227 ([`e50ff8e`](https://github.com/Byron/google-apis-rs/commit/e50ff8e30278df188ef6a016efd451d3458aa5b5))
    - Release google-servicebroker1 v3.0.0+20190624 ([`1d44b28`](https://github.com/Byron/google-apis-rs/commit/1d44b28b6a903c04b3b77afeee3eb30e7ead5074))
    - Release google-securitycenter1 v3.0.0+20220224 ([`3fddea9`](https://github.com/Byron/google-apis-rs/commit/3fddea98b25a3c582d70ed54c22037aeb8104095))
    - Release google-secretmanager1_beta1 v3.0.0+20220226 ([`df79a70`](https://github.com/Byron/google-apis-rs/commit/df79a70e44bc9b1f889eed91584a60908be45a3b))
    - Release google-secretmanager1 v3.0.0+20220226 ([`fc85c95`](https://github.com/Byron/google-apis-rs/commit/fc85c95f024a64a9d687defb8a90859656ee6123))
    - Release google-searchconsole1 v3.0.0+20220305 ([`5c42540`](https://github.com/Byron/google-apis-rs/commit/5c425406beeda33cfb26094d18604eedbd16665c))
    - Release google-sasportal1_alpha1 v3.0.0+20220301 ([`38350a6`](https://github.com/Byron/google-apis-rs/commit/38350a633bea53bd7a4579624222c646dc4a87ed))
    - Release google-safebrowsing4 v3.0.0+20220305 ([`22306dd`](https://github.com/Byron/google-apis-rs/commit/22306dd9ad93d07040eb061696d3dae2875a176c))
    - Release google-runtimeconfig1_beta1 v3.0.0+20220228 ([`79c8402`](https://github.com/Byron/google-apis-rs/commit/79c840210bf3b8e68b682fcfe0bbe32bde40517b))
    - Release google-runtimeconfig1 v3.0.0+20220228 ([`f2bde98`](https://github.com/Byron/google-apis-rs/commit/f2bde988e33f50e50e65edb9e16c1bb9fa2d30bf))
    - Release google-run2 v3.0.0+20220225 ([`e3a3598`](https://github.com/Byron/google-apis-rs/commit/e3a35989642f1dda704ce4626a761f2c7183cce1))
    - Release google-run1 v3.0.0+20220225 ([`f401798`](https://github.com/Byron/google-apis-rs/commit/f401798e2b7894022b0fe3ede0474fe136ebd24e))
    - Release google-retail2 v3.0.0+20220224 ([`021195c`](https://github.com/Byron/google-apis-rs/commit/021195ca249370b82cd525d7b654b291883f21f5))
    - Release google-resourceviews1_beta2 v3.0.0+20160512 ([`ee4bbb0`](https://github.com/Byron/google-apis-rs/commit/ee4bbb02963a5b5af6f410a506208a7ca3605617))
    - Release google-resourcesettings1 v3.0.0+20220305 ([`b66ded0`](https://github.com/Byron/google-apis-rs/commit/b66ded0f696fd9812ebb206e8ca18cfb22a5e415))
    - Release google-reseller1_sandbox v3.0.0+20160329 ([`f6c62b4`](https://github.com/Byron/google-apis-rs/commit/f6c62b43eb7572beb2fd76942e94c8d39bc718bc))
    - Release google-replicapoolupdater1_beta1 v3.0.0+20161003 ([`818595a`](https://github.com/Byron/google-apis-rs/commit/818595a3647c6fd278f6da8fba5dedca7675ea0f))
    - Release google-replicapool1_beta2 v3.0.0+20160512 ([`d5224c7`](https://github.com/Byron/google-apis-rs/commit/d5224c7b7f2e3e7f5dc50e19e6d0f1aac5449a51))
    - Release google-remotebuildexecution2 v3.0.0+20210329 ([`3c8aab0`](https://github.com/Byron/google-apis-rs/commit/3c8aab0cd2541c47a13d2e31e6fdda94cd0f2b13))
    - Release google-redis1 v3.0.0+20220301 ([`53c8b27`](https://github.com/Byron/google-apis-rs/commit/53c8b270868bc8b5ec9d2190a477d7d5e0b0d3bc))
    - Release google-recommender1_beta1 v3.0.0+20220228 ([`d831765`](https://github.com/Byron/google-apis-rs/commit/d831765d97ea5b156a0dd6d4a2b1dd10d6d51f17))
    - Release google-recommender1 v3.0.0+20220228 ([`240a771`](https://github.com/Byron/google-apis-rs/commit/240a7712e5666e55cbc907a905113663ee64a854))
    - Release google-recommendationengine1_beta1 v3.0.0+20220224 ([`27c48f9`](https://github.com/Byron/google-apis-rs/commit/27c48f9e9f464a316e08956fec36b5175ec1e462))
    - Release google-recaptchaenterprise1 v3.0.0+20220226 ([`594a226`](https://github.com/Byron/google-apis-rs/commit/594a22672ccdf65f04f55da1aa9c43d4415b933f))
    - Release google-realtimebidding1 v3.0.0+20220307 ([`f2d1c45`](https://github.com/Byron/google-apis-rs/commit/f2d1c45011f11095207d739f2df5544c4bd7d176))
    - Release google-qpxexpress1 v3.0.0+20160708 ([`7b0d006`](https://github.com/Byron/google-apis-rs/commit/7b0d006b67637e934feb0a2d826e06ad58641494))
    - Release google-pubsublite1 v3.0.0+20220301 ([`f75ab8b`](https://github.com/Byron/google-apis-rs/commit/f75ab8bf4ec574a60cc9295c5de112c46160a5f6))
    - Release google-pubsub1_beta2 v3.0.0+20220221 ([`8948e2f`](https://github.com/Byron/google-apis-rs/commit/8948e2f82a91bd8076c6c190bf516f74b0edafc5))
    - Release google-pubsub1 v3.0.0+20220221 ([`9fd6e79`](https://github.com/Byron/google-apis-rs/commit/9fd6e793f78e6a14ed70374f9e172fbe969057f0))
    - Release google-proximitybeacon1_beta1 v3.0.0+20200127 ([`d2dffee`](https://github.com/Byron/google-apis-rs/commit/d2dffee4657b62b015cbb0dd18ce4ad019ff2804))
    - Release google-prod_tt_sasportal1_alpha1 v3.0.0+20220303 ([`c64e090`](https://github.com/Byron/google-apis-rs/commit/c64e0906bd5a45140bebb9a4ee5544bf266c6d81))
    - Release google-privateca1 v3.0.0+20220209 ([`fe7d41d`](https://github.com/Byron/google-apis-rs/commit/fe7d41d26ef5ce748fccd08bb7e46a6fc749917a))
    - Release google-privateca1_beta1 v3.0.0+20220209 ([`a101ed6`](https://github.com/Byron/google-apis-rs/commit/a101ed6ecea472d733264c146ed9c25fd7736f08))
    - Release google-prediction1d6 v3.0.0+20160511 ([`a01e9f1`](https://github.com/Byron/google-apis-rs/commit/a01e9f1aa59a02f6ff8e5b04c373858630be9d82))
    - Release google-policytroubleshooter1 v3.0.0+20220227 ([`cbc05b2`](https://github.com/Byron/google-apis-rs/commit/cbc05b256ecb92d08adb06b2e6a7b0616ad74ab8))
    - Release google-policysimulator1 v3.0.0+20220227 ([`933773b`](https://github.com/Byron/google-apis-rs/commit/933773bcc430e68414c84f62a4a42e52768cfca3))
    - Release google-policyanalyzer1 v3.0.0+20220227 ([`0582c12`](https://github.com/Byron/google-apis-rs/commit/0582c120a3e0e9a0a19333b77e4cd6321f2bd3e4))
    - Release google-plusdomains1 v3.0.0+20190616 ([`75fc1b7`](https://github.com/Byron/google-apis-rs/commit/75fc1b71f4be3fcbeaf53c25e91019a3ec1daaf3))
    - Release google-plus1 v3.0.0+20190616 ([`c76b368`](https://github.com/Byron/google-apis-rs/commit/c76b368329a990fab27d8fca11b6c687f7144201))
    - Release google-playmoviespartner1 v3.0.0+20170919 ([`75c4bc3`](https://github.com/Byron/google-apis-rs/commit/75c4bc3d314b954bf60699381fab0d4d1e0223aa))
    - Release google-playintegrity1 v3.0.0+20220305 ([`1bb9b42`](https://github.com/Byron/google-apis-rs/commit/1bb9b421f10b86c3ed888981e71db11a7bc0a51e))
    - Release google-playcustomapp1 v3.0.0+20220305 ([`86633f1`](https://github.com/Byron/google-apis-rs/commit/86633f154a782955a30863618524134088404d26))
    - Release google-playablelocations3 v3.0.0+20200707 ([`a50b31d`](https://github.com/Byron/google-apis-rs/commit/a50b31d7ce199e6db2ea4021e39b3a55f49aad68))
    - Release google-photoslibrary1 v3.0.0+20220303 ([`8c25549`](https://github.com/Byron/google-apis-rs/commit/8c255492a2ef984aa5491c811d95ccbe784d46f0))
    - Release google-people1 v3.0.0+20220303 ([`7d93c6b`](https://github.com/Byron/google-apis-rs/commit/7d93c6b4698090bd818563fdea34d12d84b0b053))
    - Release google-paymentsresellersubscription1 v3.0.0+20220307 ([`04c32e4`](https://github.com/Byron/google-apis-rs/commit/04c32e492c75ce49df4c49c26c5434b15e426640))
    - Release google-partners2 v3.0.0+20180925 ([`032247b`](https://github.com/Byron/google-apis-rs/commit/032247b8b15b39a8682e55e660194e0bc5d05f7b))
    - Release google-pagespeedonline5 v3.0.0+20220302 ([`58a2b22`](https://github.com/Byron/google-apis-rs/commit/58a2b22ec6f71852acf1978b8300fe87f3c14011))
    - Release google-pagespeedonline4 v3.0.0+20191206 ([`41ef1d7`](https://github.com/Byron/google-apis-rs/commit/41ef1d74e9da45eb2a7c2b023555d50e7d93d552))
    - Release google-pagespeedonline2 v3.0.0+20191206 ([`9998369`](https://github.com/Byron/google-apis-rs/commit/99983691d8404202d8a566bc645be594cf84d911))
    - Release google-oslogin1_beta v3.0.0+20220228 ([`8d5c4f9`](https://github.com/Byron/google-apis-rs/commit/8d5c4f91074bc832778cebcfa574abdbba7d4ebc))
    - Release google-oslogin1 v3.0.0+20220228 ([`29eba25`](https://github.com/Byron/google-apis-rs/commit/29eba25779a84adae6a5e84424f432ed2230c149))
    - Release google-orgpolicy2 v3.0.0+20220305 ([`839bce2`](https://github.com/Byron/google-apis-rs/commit/839bce29014e3f8408d67bd31f191ea247c3d265))
    - Release google-ondemandscanning1 v3.0.0+20220228 ([`92a007f`](https://github.com/Byron/google-apis-rs/commit/92a007fa72a9f88172e95dc3f2111445a596c5f1))
    - Release google-notebooks1 v3.0.0+20220224 ([`fc001eb`](https://github.com/Byron/google-apis-rs/commit/fc001eb82ed2c16ec7376f76bbd96c28d9ad99c4))
    - Release google-notebooks1 v3.0.0+20220224 ([`69c977e`](https://github.com/Byron/google-apis-rs/commit/69c977ef9fc7634f4f33030e1a62276bf20f9b90))
    - Release google-networkservices1 v3.0.0+20220222 ([`70b6e8d`](https://github.com/Byron/google-apis-rs/commit/70b6e8def2b6681247136290c88c52f3d4ea0494))
    - Release google-networksecurity1 v3.0.0+20220223 ([`333adad`](https://github.com/Byron/google-apis-rs/commit/333adadd66dfe40a2ba5aad42610e9015d03c70e))
    - Release google-networkmanagement1 v3.0.0+20220223 ([`d99eb38`](https://github.com/Byron/google-apis-rs/commit/d99eb38f310c12433e6f5276a0708022c9c522b8))
    - Release google-networkconnectivity1 v3.0.0+20220210 ([`933bd0d`](https://github.com/Byron/google-apis-rs/commit/933bd0d17ec8eb42d7707d6fea2bba2df37b2a57))
    - Release google-networkconnectivity1_alpha1 v3.0.0+20220210 ([`d13f13c`](https://github.com/Byron/google-apis-rs/commit/d13f13cba56666821b6cafe825d9412fa0006261))
    - Release google-mybusinessverifications1 v3.0.0+20220305 ([`e1a6719`](https://github.com/Byron/google-apis-rs/commit/e1a6719074ba5aa09a6f6df619affd4df3114978))
    - Release google-mybusinessplaceactions1 v3.0.0+20220305 ([`15fd7d2`](https://github.com/Byron/google-apis-rs/commit/15fd7d2d11d917a11cbc7e91c0340b6bf1773391))
    - Release google-mybusinessnotifications1 v3.0.0+20220305 ([`b323ff5`](https://github.com/Byron/google-apis-rs/commit/b323ff536bab7b3480b927eef508e270abf6055f))
    - Release google-mybusinesslodging1 v3.0.0+20220305 ([`8557a2f`](https://github.com/Byron/google-apis-rs/commit/8557a2f0f4c74e0fda6f8fa4b2aeea5dfec5e2fd))
    - Release google-mybusinessbusinessinformation1 v3.0.0+20220305 ([`af96af0`](https://github.com/Byron/google-apis-rs/commit/af96af04adc613ef76bc72970a2b573a5f043472))
    - Release google-mybusinessbusinesscalls1 v3.0.0+20220305 ([`f705bc7`](https://github.com/Byron/google-apis-rs/commit/f705bc77fa1120e13ee8d33031ea4483ee6a8077))
    - Release google-mybusinessaccountmanagement1 v3.0.0+20220305 ([`8ca9a9d`](https://github.com/Byron/google-apis-rs/commit/8ca9a9d73a5ea06162bab14e517b8636197782f4))
    - Release google-mybusiness4 v3.0.0+0 ([`bbba38d`](https://github.com/Byron/google-apis-rs/commit/bbba38d18d08395fabc561187d411e711558ff60))
    - Release google-monitoring3 v3.0.0+20220218 ([`02e67a0`](https://github.com/Byron/google-apis-rs/commit/02e67a0a7d937fd97ee5efc48bf2589e5e3e17b7))
    - Release google-ml1 v3.0.0+20220212 ([`c8a89b6`](https://github.com/Byron/google-apis-rs/commit/c8a89b658feb382e3d230cd7f4cefb0f1d3abd23))
    - Release google-mirror1 v3.0.0+20190424 ([`92d5c21`](https://github.com/Byron/google-apis-rs/commit/92d5c21988a91d3048ade4f6815fffe6e0ffdaf6))
    - Release google-metastore1_beta v3.0.0+20220222 ([`0ca4656`](https://github.com/Byron/google-apis-rs/commit/0ca465629f835103695fd40c1aadbf3b9cd0d91c))
    - Release google-memcache1_beta2 v3.0.0+20220224 ([`efbf54f`](https://github.com/Byron/google-apis-rs/commit/efbf54fa877f1128b2d1ea31efa9a318e031b1a7))
    - Release google-memcache1 v3.0.0+20220224 ([`51df0c0`](https://github.com/Byron/google-apis-rs/commit/51df0c073624781185a1202a498b339688036193))
    - Release google-manufacturers1 v3.0.0+20220303 ([`3207a5f`](https://github.com/Byron/google-apis-rs/commit/3207a5f25947de0fa7e0a19f1d6bf0d5a3b60001))
    - Release google-manager1_beta2 v3.0.0+20140915 ([`ad1a7d4`](https://github.com/Byron/google-apis-rs/commit/ad1a7d4ac9be8d653f899d2ff72a02b9bdad6bb9))
    - Release google-managedidentities1 v3.0.0+20220216 ([`a5fa8c7`](https://github.com/Byron/google-apis-rs/commit/a5fa8c7ece8804db635cb2fa09a6e35aa6d76bd3))
    - Release google-logging2_beta1 v3.0.0+20190325 ([`5f7f74b`](https://github.com/Byron/google-apis-rs/commit/5f7f74b4b0794e24da428efc1c01449256b8fedc))
    - Release google-logging2 v3.0.0+20220225 ([`7c11a16`](https://github.com/Byron/google-apis-rs/commit/7c11a16a774b68ccf1657af8c5782b97af71ef50))
    - Release google-localservices1 v3.0.0+20220305 ([`892fa29`](https://github.com/Byron/google-apis-rs/commit/892fa297e7174d9b659567374b125e551138b568))
    - Release google-lifesciences2_beta v3.0.0+20220211 ([`461c88a`](https://github.com/Byron/google-apis-rs/commit/461c88ad602a3b1d1349e08025dde2af4987a763))
    - Release google-licensing1 v3.0.0+20220305 ([`34d9ead`](https://github.com/Byron/google-apis-rs/commit/34d9ead30468a9a7c6eadd2ca2621de1ff7471b6))
    - Release google-libraryagent1 v3.0.0+20220305 ([`a6fc5b2`](https://github.com/Byron/google-apis-rs/commit/a6fc5b2d1d2d42f09bab9617cf53c0a4668ecd73))
    - Release google-language1_beta1 v3.0.0+20220218 ([`b3985b2`](https://github.com/Byron/google-apis-rs/commit/b3985b2b947f3c74c5dfa470e5012f8ac5039fc8))
    - Release google-language1 v3.0.0+20220218 ([`76cafe5`](https://github.com/Byron/google-apis-rs/commit/76cafe571f195d24e2a328768667fbda468cad98))
    - Release google-keep1 v3.0.0+20220301 ([`7c41f61`](https://github.com/Byron/google-apis-rs/commit/7c41f61bf0bf510e4215b149b3f99da904ca3615))
    - Release google-jobs4 v3.0.0+20220211 ([`20628f5`](https://github.com/Byron/google-apis-rs/commit/20628f5d5a0bfd624716d35726446fbde074799a))
    - Release google-jobs3 v3.0.0+20220211 ([`534e161`](https://github.com/Byron/google-apis-rs/commit/534e16147dc47b022c1993f17f0d8b9bbe952e4a))
    - Release google-indexing3 v3.0.0+20220126 ([`f3f8e3f`](https://github.com/Byron/google-apis-rs/commit/f3f8e3f6018a48aa81d95c6cb1dd4303d6a54c29))
    - Release google-ids1 v3.0.0+20220221 ([`929d419`](https://github.com/Byron/google-apis-rs/commit/929d419b6790bf692f7bf6ce3eed6be51d2ce4f3))
    - Release google-identitytoolkit3 v3.0.0+20180723 ([`c3ec6a9`](https://github.com/Byron/google-apis-rs/commit/c3ec6a98f9da708ab26d711c5e968f2e965b54fd))
    - Release google-ideahub1_beta v3.0.0+20220305 ([`65b9d57`](https://github.com/Byron/google-apis-rs/commit/65b9d5764a6997d6d19979bfdfa04d2a82c8596b))
    - Release google-iap1_beta1 v3.0.0+20220225 ([`bece166`](https://github.com/Byron/google-apis-rs/commit/bece166b63232fe0f59384b64643f2708584d56c))
    - Release google-iap1 v3.0.0+20220225 ([`368db12`](https://github.com/Byron/google-apis-rs/commit/368db12518ddcc1d8cb861cbb336a6a8c91cbf3d))
    - Release google-iamcredentials1 v3.0.0+20220225 ([`4680770`](https://github.com/Byron/google-apis-rs/commit/4680770b7e4a70a8534884628b8ea10cb97e718f))
    - Release google-iam1 v3.0.0+20220224 ([`1252be6`](https://github.com/Byron/google-apis-rs/commit/1252be6ab43c79adb88db3133d9aae0551e70b05))
    - Release google-healthcare1_beta1 v3.0.0+20220223 ([`93df7eb`](https://github.com/Byron/google-apis-rs/commit/93df7eb7b9e914a1bba44c1c3f6d45f94dea7dfd))
    - Release google-healthcare1 v3.0.0+20220223 ([`83e4275`](https://github.com/Byron/google-apis-rs/commit/83e4275f581cb817fea0a807b48873197781d61d))
    - Release google-groupssettings1 v3.0.0+20220224 ([`4db48f7`](https://github.com/Byron/google-apis-rs/commit/4db48f7563607b2a9086efa38b7b620dd647fadb))
    - Release google-groupsmigration1 v3.0.0+20220226 ([`59fb518`](https://github.com/Byron/google-apis-rs/commit/59fb518fdbb3cdefc020b27b627b26c617be7c61))
    - Release google-gmailpostmastertools1_beta1 v3.0.0+20220305 ([`b890012`](https://github.com/Byron/google-apis-rs/commit/b8900120c512a18dee659ab059ec579affdff869))
    - Release google-gmailpostmastertools1 v3.0.0+20220305 ([`e97c524`](https://github.com/Byron/google-apis-rs/commit/e97c52427232fb14d76f64dc6bbf4724c925594b))
    - Release google-gmail1 v3.0.0+20220228 ([`ba2f2e9`](https://github.com/Byron/google-apis-rs/commit/ba2f2e9a50864864206131f94f5e553d8b46c828))
    - Release google-gkehub1 v3.0.0+20220211 ([`500eb2a`](https://github.com/Byron/google-apis-rs/commit/500eb2a210501cdd813a2f26d4ffc912ba177d12))
    - Release google-genomics1 v3.0.0+20210324 ([`a5d9a63`](https://github.com/Byron/google-apis-rs/commit/a5d9a63e401d556e91f9ec68935e64643179d43c))
    - Release google-gan1_beta1 v3.0.0+20130205 ([`42b4a1f`](https://github.com/Byron/google-apis-rs/commit/42b4a1f005b28f91d65262efd4fdce8f566e788a))
    - Release google-gamesmanagement1_management v3.0.0+20220217 ([`cb64b01`](https://github.com/Byron/google-apis-rs/commit/cb64b01238d3d2fceb80fe0b427b6963ca054962))
    - Release google-gameservices1 v3.0.0+20220223 ([`b1740f2`](https://github.com/Byron/google-apis-rs/commit/b1740f263cc7a052fc9f4b5f68417ec992efe873))
    - Release google-gamesconfiguration1_configuration v3.0.0+20220217 ([`e2332b5`](https://github.com/Byron/google-apis-rs/commit/e2332b507b6f54d73fd78b578b2919ad2af8e783))
    - Release google-games1 v3.0.0+20220217 ([`4fddb19`](https://github.com/Byron/google-apis-rs/commit/4fddb19b619e9cadab45b9c84a4a373919db0fbd))
    - Release google-fusiontables2 v3.0.0+20171117 ([`a1cfea3`](https://github.com/Byron/google-apis-rs/commit/a1cfea3fe2ab750a2c954cd17464493891959753))
    - Release google-fitness1 v3.0.0+20220302 ([`675e061`](https://github.com/Byron/google-apis-rs/commit/675e0618a5a8393c2f26bf168425f183ea724d91))
    - Release google-firestore1_beta1 v3.0.0+20220221 ([`8538cea`](https://github.com/Byron/google-apis-rs/commit/8538cea71bf0eb4ef652f5ace09daf22297f604d))
    - Release google-firestore1 v3.0.0+20220221 ([`48c0b44`](https://github.com/Byron/google-apis-rs/commit/48c0b44d0db8a35512f1edadde71974b62509477))
    - Release google-firebasestorage1_beta v3.0.0+20220218 ([`7cbf0a0`](https://github.com/Byron/google-apis-rs/commit/7cbf0a084115aab8ca6f0eb7b7eb6eff3f060055))
    - Release google-firebaseremoteconfig1 v3.0.0+20171129 ([`2d5c111`](https://github.com/Byron/google-apis-rs/commit/2d5c11178158180eac3571b76ce402b6d6334f40))
    - Release google-firebaseml1 v3.0.0+20220302 ([`d2f320c`](https://github.com/Byron/google-apis-rs/commit/d2f320c65950e7865e1ecaa48143a22a97ea9003))
    - Release google-firebasehosting1_beta1 v3.0.0+20220212 ([`2bb9e18`](https://github.com/Byron/google-apis-rs/commit/2bb9e180ebf8ec39426ba9703eea42c8fd5463f9))
    - Release google-firebasehosting1 v3.0.0+20220212 ([`a1f7887`](https://github.com/Byron/google-apis-rs/commit/a1f78879ceebf19a08c32c0e09b88be2fbc34804))
    - Release google-firebasedynamiclinks1 v3.0.0+20220228 ([`682820a`](https://github.com/Byron/google-apis-rs/commit/682820aad7f80bd709db02197618a5abb0f39686))
    - Release google-firebasedatabase1_beta v3.0.0+20220304 ([`0f08833`](https://github.com/Byron/google-apis-rs/commit/0f08833f7998b2035c2d4e8280d227fab8446cdc))
    - Release google-firebaseappcheck1_beta v3.0.0+20220225 ([`17415da`](https://github.com/Byron/google-apis-rs/commit/17415da1c99d7d032d25e2dfb1916a4b26bb2ba3))
    - Release google-firebase1_beta1 v3.0.0+20220304 ([`b4177e5`](https://github.com/Byron/google-apis-rs/commit/b4177e5898893a7ec0459ead88c1f5d06bb983f3))
    - Release google-file1_beta1 v3.0.0+20220214 ([`6d25904`](https://github.com/Byron/google-apis-rs/commit/6d2590444d0cdf10a74772a410b6b5927e1b58a9))
    - Release google-file1 v3.0.0+20220214 ([`d9cdc05`](https://github.com/Byron/google-apis-rs/commit/d9cdc0517b4ceb4f025b16ad383379512b9c82df))
    - Release google-fcmdata1_beta1 v3.0.0+20220305 ([`e867d42`](https://github.com/Byron/google-apis-rs/commit/e867d4298912490608d1395a53725bc4e6606101))
    - Release google-fcm1 v3.0.0+20220228 ([`923477c`](https://github.com/Byron/google-apis-rs/commit/923477c1ed0c83d0a7387d691cbf1ff453439740))
    - Release google-factchecktools1_alpha1 v3.0.0+20220305 ([`ba9773f`](https://github.com/Byron/google-apis-rs/commit/ba9773f55e751fec9ac2ab2d509f5b05797d84ee))
    - Release google-eventarc1 v3.0.0+20220301 ([`113ae4f`](https://github.com/Byron/google-apis-rs/commit/113ae4f24086e6993e0670f38c3924ea3808c340))
    - Release google-essentialcontacts1 v3.0.0+20220227 ([`cc79327`](https://github.com/Byron/google-apis-rs/commit/cc7932731c9a323270efe6cb9e5d0f4d0ab925ac))
    - Release google-driveactivity2 v3.0.0+20220301 ([`4db13ee`](https://github.com/Byron/google-apis-rs/commit/4db13eed61581b64014ed7ce7799f57f7def8717))
    - Release google-drive3 v3.0.0+20220225 ([`fca1528`](https://github.com/Byron/google-apis-rs/commit/fca1528cb9b58d9e72ddde5f5da75d7f99132f09))
    - Release google-drive2 v3.0.0+20220225 ([`f469861`](https://github.com/Byron/google-apis-rs/commit/f4698611551bcce9265f5bf8d495f04c1188657f))
    - Release google-doubleclicksearch2 v3.0.0+20220301 ([`da4d941`](https://github.com/Byron/google-apis-rs/commit/da4d9416c2fbdb9b4bdb6fb9da3e8304c78b6dce))
    - Release google-doubleclickbidmanager1d1 v3.0.0+20220302 ([`e3172d1`](https://github.com/Byron/google-apis-rs/commit/e3172d144cc59d3a1eb16368a7cac2a08a3126d9))
    - Release google-doubleclickbidmanager1 v3.0.0+20210323 ([`1e45006`](https://github.com/Byron/google-apis-rs/commit/1e450060afe2b08f62974897286ce63b6c23e24b))
    - Release google-domainsrdap1 v3.0.0+20220307 ([`4755655`](https://github.com/Byron/google-apis-rs/commit/4755655b3b18debea0d7ea318706d72da52c3d8f))
    - Release google-domains1 v3.0.0+20220128 ([`d24f572`](https://github.com/Byron/google-apis-rs/commit/d24f5729d65f1da2be9411df97cee2a193dc6662))
    - Release google-domains1_beta1 v3.0.0+20220128 ([`67fd151`](https://github.com/Byron/google-apis-rs/commit/67fd15125b896e3d0e0f33659555bceb24f7d9f0))
    - Release google-documentai1_beta2 v3.0.0+20220226 ([`4ea58f4`](https://github.com/Byron/google-apis-rs/commit/4ea58f46910785927c2af12dcfb56045dc152579))
    - Release google-documentai1 v3.0.0+20220226 ([`408ed77`](https://github.com/Byron/google-apis-rs/commit/408ed77667390c5f181205515d8e0ca401e312a3))
    - Release google-docs1 v3.0.0+20220301 ([`7a829b5`](https://github.com/Byron/google-apis-rs/commit/7a829b563c2e009645b64b8ef9c5625609bdbe89))
    - Release google-dns2 v3.0.0+20220217 ([`a17ee81`](https://github.com/Byron/google-apis-rs/commit/a17ee81205d1b7acca623cb921f008ce66246d9a))
    - Release google-dns1 v3.0.0+20220217 ([`55def66`](https://github.com/Byron/google-apis-rs/commit/55def6641299a278afc690bd1bb0fb145c5409a2))
    - Release google-dlp2_beta1 v3.0.0+20171205 ([`b085ea4`](https://github.com/Byron/google-apis-rs/commit/b085ea484dede5b4a53f0ebb51d9d2dabfd0ef5b))
    - Release google-dlp2 v3.0.0+20220227 ([`69e3fd6`](https://github.com/Byron/google-apis-rs/commit/69e3fd64859ca7335ac1d990f50362a5e1b6157a))
    - Release google-displayvideo1 v3.0.0+20220303 ([`4064e54`](https://github.com/Byron/google-apis-rs/commit/4064e548eb68ab30dad50db37eb985f542165ffd))
    - Release google-discovery1 v3.0.0+20200806 ([`51c467c`](https://github.com/Byron/google-apis-rs/commit/51c467cea3ee8092e16768e9b40f48fefc3b5a38))
    - Release google-digitalassetlinks1 v3.0.0+20220301 ([`df4b715`](https://github.com/Byron/google-apis-rs/commit/df4b715cb795944485403d64fbb80e23d3be82ea))
    - Release google-dialogflow3 v3.0.0+20220228 ([`e155a35`](https://github.com/Byron/google-apis-rs/commit/e155a355d74c4a430207102eb3df1363f5a766e4))
    - Release google-dialogflow2_beta1 v3.0.0+20220228 ([`a6a08ec`](https://github.com/Byron/google-apis-rs/commit/a6a08ecedfdbf31191582f44713109b9ed42cff3))
    - Release google-dialogflow2 v3.0.0+20220228 ([`4a7391d`](https://github.com/Byron/google-apis-rs/commit/4a7391d7cb077153dd70c57205847f2909722c68))
    - Release google-dfareporting3d5 v3.0.0+20220104 ([`2a0c155`](https://github.com/Byron/google-apis-rs/commit/2a0c155e77308c7dbc302821dc8681f7c6b3dca0))
    - Release google-dfareporting3d4 v3.0.0+20220104 ([`36f8d33`](https://github.com/Byron/google-apis-rs/commit/36f8d3329ec898f73aa89302d9e449751308d221))
    - Release google-dfareporting3d3 v3.0.0+20220104 ([`12c4527`](https://github.com/Byron/google-apis-rs/commit/12c45273f51e61364997eb34ab3c9f0560f48f17))
    - Release google-dfareporting3d2 v3.0.0+20190531 ([`dcd7784`](https://github.com/Byron/google-apis-rs/commit/dcd778478d905f9a9136b346d1cdc0710d0e5280))
    - Release google-dfareporting3 v3.0.0+20180830 ([`515f29a`](https://github.com/Byron/google-apis-rs/commit/515f29a6d00ec3351571c29b32b49643e34e9b90))
    - Release google-dfareporting2d8 v3.0.0+20180830 ([`4dfae78`](https://github.com/Byron/google-apis-rs/commit/4dfae787e6167bc2e97f99c5d5d93e4942ddce76))
    - Release google-deploymentmanager2_beta2 v3.0.0+20160201 ([`4e7d5d3`](https://github.com/Byron/google-apis-rs/commit/4e7d5d3662386d4554e7bae922ecaf9951de68cd))
    - Release google-deploymentmanager2 v3.0.0+20220225 ([`72f26bf`](https://github.com/Byron/google-apis-rs/commit/72f26bfe27863bc592cdd845971dd0312b177cd4))
    - Release google-datastream1 v3.0.0+20220207 ([`4d9c4fb`](https://github.com/Byron/google-apis-rs/commit/4d9c4fb239e975b981c412c2d157db17efcff116))
    - Release google-datastore1_beta3 v3.0.0+20220221 ([`3b9e11f`](https://github.com/Byron/google-apis-rs/commit/3b9e11fd1fe8e0faa5c57f2bdf207dacfc870b43))
    - Release google-datastore1 v3.0.0+20220221 ([`2413d19`](https://github.com/Byron/google-apis-rs/commit/2413d19bf1d3e1d964dc07923070fba4836c679f))
    - Release google-dataproc1 v3.0.0+20220224 ([`dcd58bd`](https://github.com/Byron/google-apis-rs/commit/dcd58bd15ec343b926b620f5db19978119b71560))
    - Release google-dataplex1 v3.0.0+20220223 ([`e0574be`](https://github.com/Byron/google-apis-rs/commit/e0574bec936b524c52fa9d52784331d8a10f47d7))
    - Release google-datapipelines1 v3.0.0+20220218 ([`39050ba`](https://github.com/Byron/google-apis-rs/commit/39050bafd7ea08fcf35dd421dea1ac0bfc55bbce))
    - Release google-datamigration1 v3.0.0+20220216 ([`149a66e`](https://github.com/Byron/google-apis-rs/commit/149a66e8b6bd1198cced6b9e666b09ab49739167))
    - Release google-datalabeling1_beta1 v3.0.0+20220301 ([`c18cf88`](https://github.com/Byron/google-apis-rs/commit/c18cf887bb6d4613022cb80a092284d2cce14f0b))
    - Release google-datafusion1_beta1 v3.0.0+20211028 ([`a5c1fa0`](https://github.com/Byron/google-apis-rs/commit/a5c1fa0595012b84ba99a72939f12cef1f1287f2))
    - Release google-datafusion1 v3.0.0+20211028 ([`a519b07`](https://github.com/Byron/google-apis-rs/commit/a519b07b3faca453f6dd0a2f322abb6363c28b90))
    - Release google-datacatalog1 v3.0.0+20220224 ([`7972ef7`](https://github.com/Byron/google-apis-rs/commit/7972ef7092a688a7067018b42953efda3e647310))
    - Release google-datacatalog1_beta1 v3.0.0+20220224 ([`7b15161`](https://github.com/Byron/google-apis-rs/commit/7b1516166231a37c258ccf50d34ba6e1affd7673))
    - Release google-customsearch1 v3.0.0+20220305 ([`e90790e`](https://github.com/Byron/google-apis-rs/commit/e90790ec782037bb7a7befa8fffb62f8181bc977))
    - Release google-coordinate1 v3.0.0+20150811 ([`f5a9108`](https://github.com/Byron/google-apis-rs/commit/f5a9108041c1085c2d4b7da485761da2f4dec650))
    - Release google-content2_sandbox v3.0.0+20181009 ([`4426b32`](https://github.com/Byron/google-apis-rs/commit/4426b32bfae993ea472362c75e616f7cb464e83d))
    - Release google-content2 v3.0.0+20220303 ([`da06593`](https://github.com/Byron/google-apis-rs/commit/da0659369753241cba424597bd7e2f2ad9ca292e))
    - Release google-containeranalysis1 v3.0.0+20220225 ([`ce29be5`](https://github.com/Byron/google-apis-rs/commit/ce29be5a75581cca05725c891ede6f081dfce9c4))
    - Release google-containeranalysis1_beta1 v3.0.0+20220225 ([`dce684c`](https://github.com/Byron/google-apis-rs/commit/dce684c16a01b358076de825a452c5580dfc5ad2))
    - Release google-container1 v3.0.0+20220215 ([`9093112`](https://github.com/Byron/google-apis-rs/commit/90931126fb9751c81f22edf61b968924a836fce4))
    - Release google-contactcenterinsights1 v3.0.0+20220227 ([`e433ac5`](https://github.com/Byron/google-apis-rs/commit/e433ac5f40e76825992ca186fcc235096c9b7787))
    - Release google-consumersurveys2 v3.0.0+20170407 ([`7f7bb4b`](https://github.com/Byron/google-apis-rs/commit/7f7bb4b7a7e76191380c90d726e0775e301eb33d))
    - Release google-connectors1 v3.0.0+20220214 ([`9af3649`](https://github.com/Byron/google-apis-rs/commit/9af36490e69a143cf49f8b39eca5c9052d85ebee))
    - Release google-compute1 v3.0.0+20220224 ([`4a26680`](https://github.com/Byron/google-apis-rs/commit/4a26680fa972a5609f2fbfebf8e6c6b57a1827bb))
    - Release google-composer1 v3.0.0+20220224 ([`27789d4`](https://github.com/Byron/google-apis-rs/commit/27789d4bb405f48c59a5ef75af1064b5bf1199db))
    - Release google-commentanalyzer1_alpha1 v3.0.0+20200405 ([`4cd5d72`](https://github.com/Byron/google-apis-rs/commit/4cd5d72be724ffae153a3df94dceab3139e956fd))
    - Release google-clouduseraccountsvm_beta v3.0.0+20160316 ([`dc267c5`](https://github.com/Byron/google-apis-rs/commit/dc267c548c6f1772cfcfa7b81f7284ef44353409))
    - Release google-cloudtrace2 v3.0.0+20220224 ([`2565b9d`](https://github.com/Byron/google-apis-rs/commit/2565b9dcf6e73a440431cd197463e28ce02e5bc8))
    - Release google-cloudtrace1 v3.0.0+20220224 ([`d400a84`](https://github.com/Byron/google-apis-rs/commit/d400a8471fab4f891ecda5d8281277f7789878c1))
    - Release google-cloudtasks2_beta3 v3.0.0+20220212 ([`35361fd`](https://github.com/Byron/google-apis-rs/commit/35361fdec2452c85c195d62eb91216208a888e0c))
    - Release google-cloudtasks2_beta2 v3.0.0+20220212 ([`adf5837`](https://github.com/Byron/google-apis-rs/commit/adf5837debd13ef7a97fb8b639e7e2c820f0d302))
    - Release google-cloudtasks2 v3.0.0+20220212 ([`7044532`](https://github.com/Byron/google-apis-rs/commit/7044532bc6abd418bcc84d396861707c0436bbbb))
    - Release google-cloudsupport2_beta v3.0.0+20220305 ([`3c0a4ee`](https://github.com/Byron/google-apis-rs/commit/3c0a4ee3d2b77906408c6ae62bb1af97ef3bd7de))
    - Release google-cloudshell1 v3.0.0+20220301 ([`d655f7a`](https://github.com/Byron/google-apis-rs/commit/d655f7a9bad6f336186d859f1ce8c7b09d756208))
    - Release google-cloudscheduler1_beta1 v3.0.0+20220212 ([`1dc187f`](https://github.com/Byron/google-apis-rs/commit/1dc187f17b5b598c022de68d9b6e818f8651b3b7))
    - Release google-cloudscheduler1 v3.0.0+20220212 ([`e1dff07`](https://github.com/Byron/google-apis-rs/commit/e1dff07767972b3d28c0a11632f789345a1e4d96))
    - Release google-cloudresourcemanager3 v3.0.0+20220306 ([`8f469b1`](https://github.com/Byron/google-apis-rs/commit/8f469b1613939f8cb4c6ca965019196482b13c0a))
    - Release google-cloudresourcemanager2 v3.0.0+20220306 ([`58ac2ab`](https://github.com/Byron/google-apis-rs/commit/58ac2ab41c2fd66c9536a4e3fbe320eb2b04a8b5))
    - Release google-cloudresourcemanager1_beta1 v3.0.0+20220306 ([`65fa858`](https://github.com/Byron/google-apis-rs/commit/65fa8583593df9b7c232eb535a7678b201ecc567))
    - Release google-cloudresourcemanager1 v3.0.0+20220306 ([`4d44315`](https://github.com/Byron/google-apis-rs/commit/4d443157ebc4cdf2339e5b4c538068e20e9b3650))
    - Release google-cloudprofiler2 v3.0.0+20220228 ([`bccb177`](https://github.com/Byron/google-apis-rs/commit/bccb177e057cae4f8a60c1944500450f2766f0e8))
    - Release google-cloudprivatecatalogproducer1_beta1 v3.0.0+20200405 ([`07d6c36`](https://github.com/Byron/google-apis-rs/commit/07d6c3646a2d1702dae42026bd2adc5deedb8098))
    - Release google-cloudprivatecatalog1_beta1 v3.0.0+20200405 ([`5e268bf`](https://github.com/Byron/google-apis-rs/commit/5e268bf42d3c2c7d53811d4f06381f4b59c7216f))
    - Release google-cloudmonitoring2_beta2 v3.0.0+20170501 ([`3d608be`](https://github.com/Byron/google-apis-rs/commit/3d608be6c2fcc2005b31ab2528745b07b24edfba))
    - Release google-cloudlatencytest2 v3.0.0+20160309 ([`54a54e5`](https://github.com/Byron/google-apis-rs/commit/54a54e5e4b58604379093a6a6c30b1a37b584cf9))
    - Release google-cloudkms1_beta1 v3.0.0+20170515 ([`821d830`](https://github.com/Byron/google-apis-rs/commit/821d830aa34e52a90f9d0ab7f56581bf507f4950))
    - Release google-cloudkms1 v3.0.0+20220225 ([`600e360`](https://github.com/Byron/google-apis-rs/commit/600e360c43f9a1c5e691ae292ead8c64a1b3d26b))
    - Release google-cloudiot1 v3.0.0+20220131 ([`35da04d`](https://github.com/Byron/google-apis-rs/commit/35da04def706c9ade0ab447d3011e8d2b38d6900))
    - Release google-cloudidentity1 v3.0.0+20220301 ([`27fd9a1`](https://github.com/Byron/google-apis-rs/commit/27fd9a1aff48fd91600655d626e9d4dc093e3f12))
    - Release google-cloudfunctions1 v3.0.0+20220224 ([`d505205`](https://github.com/Byron/google-apis-rs/commit/d505205b20f4e6cd46dfbf9b12ebf053c572dcd3))
    - Release google-clouderrorreporting1_beta1 v3.0.0+20220302 ([`598d791`](https://github.com/Byron/google-apis-rs/commit/598d791e7245d2af2831e956b8bf021a5d460331))
    - Release google-clouddeploy1 v3.0.0+20220223 ([`16b4e21`](https://github.com/Byron/google-apis-rs/commit/16b4e21431e76813cd030aead168ca9dfed30430))
    - Release google-clouddebugger2 v3.0.0+20220225 ([`4a80b76`](https://github.com/Byron/google-apis-rs/commit/4a80b763088033f15b54507b37e22cfaf1a10a07))
    - Release google-cloudchannel1 v3.0.0+20220303 ([`cfb40cc`](https://github.com/Byron/google-apis-rs/commit/cfb40ccbd222781cbddb1fe9fd0afa50043221d0))
    - Release google-cloudbuild1 v3.0.0+20220218 ([`732b70b`](https://github.com/Byron/google-apis-rs/commit/732b70b7e92cce84ce959b14562e3bf667fbde9d))
    - Release google-cloudbilling1 v3.0.0+20220305 ([`89342ad`](https://github.com/Byron/google-apis-rs/commit/89342adecc2416b2d6102e882b62875864b2eab5))
    - Release google-cloudasset1_beta1 v3.0.0+20220225 ([`2a2b124`](https://github.com/Byron/google-apis-rs/commit/2a2b124573542270bd50bd60735da859dfad7d59))
    - Release google-cloudasset1 v3.0.0+20220225 ([`e3781fc`](https://github.com/Byron/google-apis-rs/commit/e3781fc623f6d3e631f7698b1efcb055c0384786))
    - Release google-classroom1 v3.0.0+20220224 ([`b28cd5a`](https://github.com/Byron/google-apis-rs/commit/b28cd5ac0a14b4fb82286d4a7ad8732c8614559c))
    - Release google-chromeuxreport1 v3.0.0+20220302 ([`8c2a506`](https://github.com/Byron/google-apis-rs/commit/8c2a5060c253fb40d885e7565d4cd75dffc9e3ae))
    - Release google-chromepolicy1 v3.0.0+20220305 ([`225e16c`](https://github.com/Byron/google-apis-rs/commit/225e16c4ef197db9bf6342db06c07a0e3d9319e1))
    - Release google-chromemanagement1 v3.0.0+20220305 ([`17bc736`](https://github.com/Byron/google-apis-rs/commit/17bc736bb60ac6626401bfb01c3d30e91e861085))
    - Release google-certificatemanager1 v3.0.0+20220214 ([`b6cba33`](https://github.com/Byron/google-apis-rs/commit/b6cba3356f037030a898fc7e2e8049358c359d72))
    - Release google-calendar3 v3.0.0+20220217 ([`b905edd`](https://github.com/Byron/google-apis-rs/commit/b905edddc31056096494336a740b3fe1e233879c))
    - Release google-books1 v3.0.0+20220301 ([`00b9958`](https://github.com/Byron/google-apis-rs/commit/00b9958c1a8f608caff1985075ad8eaa08c806ab))
    - Release google-blogger3 v3.0.0+20220305 ([`fcef78f`](https://github.com/Byron/google-apis-rs/commit/fcef78fdb844647b9ee707f17a3c5b1012bb7afb))
    - Release google-binaryauthorization1_beta1 v3.0.0+20220225 ([`b2a0f85`](https://github.com/Byron/google-apis-rs/commit/b2a0f85ed1d6bda21c4edf05c711906d92095196))
    - Release google-binaryauthorization1 v3.0.0+20220225 ([`2e964d6`](https://github.com/Byron/google-apis-rs/commit/2e964d62955d0488fb0600e8635628a1c42bb4ae))
    - Release google-billingbudgets1_beta1 v3.0.0+20220227 ([`6bc51fc`](https://github.com/Byron/google-apis-rs/commit/6bc51fce617a76bb5cc8dc2e6aedd9a03ff53391))
    - Release google-billingbudgets1 v3.0.0+20220227 ([`d13caf2`](https://github.com/Byron/google-apis-rs/commit/d13caf2ed471b584931a45ff0a1a48fb6854b6e3))
    - Release google-bigtableadmin2 v3.0.0+20220222 ([`26b8995`](https://github.com/Byron/google-apis-rs/commit/26b8995e6d7c02ffa61d2e758d1b7a3bbfdecd5a))
    - Release google-bigqueryreservation1 v3.0.0+20220226 ([`e162ea1`](https://github.com/Byron/google-apis-rs/commit/e162ea1059d636598965929ced7163dd30c0e76e))
    - Release google-bigquerydatatransfer1 v3.0.0+20220225 ([`df158d6`](https://github.com/Byron/google-apis-rs/commit/df158d663dbc9a1e0ce0b6f65273e4227277ac6c))
    - Release google-bigqueryconnection1_beta1 v3.0.0+20220226 ([`82bde01`](https://github.com/Byron/google-apis-rs/commit/82bde012c9b31a969c75495e477befb7465d7d6d))
    - Release google-bigquery2 v3.0.0+20220222 ([`22f0bcd`](https://github.com/Byron/google-apis-rs/commit/22f0bcd620f7cdf44863ec9a5b0d6e9e9e8cf088))
    - Release google-baremetalsolution2 v3.0.0+20220209 ([`32a5753`](https://github.com/Byron/google-apis-rs/commit/32a57530296267317786d6e314623ba996184b0f))
    - Release google-autoscaler1_beta2 v3.0.0+20150629 ([`ffe4fd0`](https://github.com/Byron/google-apis-rs/commit/ffe4fd0e324d108a47a2cefa4595c02fc5ea967d))
    - Release google-authorizedbuyersmarketplace1 v3.0.0+20220307 ([`f0e64f9`](https://github.com/Byron/google-apis-rs/commit/f0e64f914785c10c1e4ae3902a073f9764a25015))
    - Release google-assuredworkloads1 v3.0.0+20220224 ([`4547e93`](https://github.com/Byron/google-apis-rs/commit/4547e93c1d22780da40899d2103fa68c71ebe410))
    - Release google-artifactregistry1_beta1 v3.0.0+20220225 ([`be7cc02`](https://github.com/Byron/google-apis-rs/commit/be7cc02fd976d3361c2cf34bd6c6e7bb280b1939))
    - Release google-artifactregistry1 v3.0.0+20220225 ([`555c70e`](https://github.com/Byron/google-apis-rs/commit/555c70e456109b20d4a16cb9be358406d2d0bbdb))
    - Release google-area120tables1_alpha1 v3.0.0+20220301 ([`b82bc76`](https://github.com/Byron/google-apis-rs/commit/b82bc766eae0bf932f4a3bc95cd05370731b419d))
    - Release google-appstate1 v3.0.0+20190627 ([`581865d`](https://github.com/Byron/google-apis-rs/commit/581865dcef3ee066f590d1d1fd8cbd96ecaf2374))
    - Release google-appsactivity1 v3.0.0+20200628 ([`4cee937`](https://github.com/Byron/google-apis-rs/commit/4cee9377844a290514143e5cd08b2830e3720438))
    - Release google-appengine1_beta5 v3.0.0+20181005 ([`b13a6a7`](https://github.com/Byron/google-apis-rs/commit/b13a6a79b5ad09a16ce8332e393ba36295e195ca))
    - Release google-appengine1_beta4 v3.0.0+20181005 ([`b8ae1eb`](https://github.com/Byron/google-apis-rs/commit/b8ae1ebef85e4c915323a285dd35b2cf32323d2a))
    - Release google-appengine1 v3.0.0+20220226 ([`b33628f`](https://github.com/Byron/google-apis-rs/commit/b33628f2ebef812db1126c5dba771b3a1393eed5))
    - Release google-apikeys2 v3.0.0+20220305 ([`a70c833`](https://github.com/Byron/google-apis-rs/commit/a70c833a1af7415702f8d76f9681da303bf0f4df))
    - Release google-apigee1 v3.0.0+20220301 ([`085b3cb`](https://github.com/Byron/google-apis-rs/commit/085b3cbc1e50383ca8ff21ae38eda98e3322568e))
    - Release google-apigateway1 v3.0.0+20220223 ([`dbf73a6`](https://github.com/Byron/google-apis-rs/commit/dbf73a64da1a414807e5a1a6948c04567bb4cf62))
    - Release google-androidpublisher3 v3.0.0+20220307 ([`28f1863`](https://github.com/Byron/google-apis-rs/commit/28f186318766208039b5087c2c315e7d8c031a4b))
    - Release google-androidpublisher2 v3.0.0+20200331 ([`3e974f4`](https://github.com/Byron/google-apis-rs/commit/3e974f40cd0b51cc67effc902a6e7abc7ebd4069))
    - Release google-androidmanagement1 v3.0.0+20220302 ([`8052ef1`](https://github.com/Byron/google-apis-rs/commit/8052ef136665e9bebb1016b775c7451b93b19bb3))
    - Release google-androidenterprise1 v3.0.0+20220303 ([`a88248b`](https://github.com/Byron/google-apis-rs/commit/a88248b070427ba7b19cfd058fd9aff72f06978b))
    - Release google-androiddeviceprovisioning1 v3.0.0+20220305 ([`1e6126c`](https://github.com/Byron/google-apis-rs/commit/1e6126ccdcecb3999fd6ef29e5326e9dacfb4979))
    - Release google-analyticsreporting4 v3.0.0+20220215 ([`209267e`](https://github.com/Byron/google-apis-rs/commit/209267e7c03c2cd00bb013ec6484a61d164460c8))
    - Release google-analyticsdata1_beta v3.0.0+20220303 ([`a1e2b07`](https://github.com/Byron/google-apis-rs/commit/a1e2b073c5bf032fa254736e323673a272878820))
    - Release google-analyticsadmin1_alpha v3.0.0+20220307 ([`83e8f99`](https://github.com/Byron/google-apis-rs/commit/83e8f9932919b75472853072f36150c1c61d962f))
    - Release google-analytics3 v3.0.0+20190807 ([`ce606e8`](https://github.com/Byron/google-apis-rs/commit/ce606e8bb21a95c134e2418f9d66b750bf8a47f8))
    - Release google-alertcenter1_beta1 v3.0.0+20220221 ([`d567a75`](https://github.com/Byron/google-apis-rs/commit/d567a75babf055815bb198cf01a24f7e52c86891))
    - Release google-adsensehost4d1 v3.0.0+20200930 ([`ab18d32`](https://github.com/Byron/google-apis-rs/commit/ab18d3203df62372305aa287619b169967294b83))
    - Release google-adsense2 v3.0.0+20220304 ([`b08e5b1`](https://github.com/Byron/google-apis-rs/commit/b08e5b108c9debba47daa3facb54a50faba510f8))
    - Release google-adsense1d4 v3.0.0+20201002 ([`a374d9d`](https://github.com/Byron/google-apis-rs/commit/a374d9d29a2ba1638c8326a4908707e30b140a20))
    - Release google-admob1 v3.0.0+20220303 ([`ae8ff5b`](https://github.com/Byron/google-apis-rs/commit/ae8ff5b424deb7484c90f3e17663b4fabdc7b411))
    - Release google-adexperiencereport1 v3.0.0+20220303 ([`c67cee3`](https://github.com/Byron/google-apis-rs/commit/c67cee3ba427dafe2ab7042bf331de0484068fcc))
    - Release google-adexchangeseller2 v3.0.0+20171101 ([`c3dfe29`](https://github.com/Byron/google-apis-rs/commit/c3dfe297e64464f2aa415d308c7c9a6f2a77ce87))
    - Release google-adexchangebuyer2_v2_beta1 v3.0.0+20220307 ([`c304885`](https://github.com/Byron/google-apis-rs/commit/c30488568ff1422014a67ab65368f22ce629783b))
    - Release google-adexchangebuyer1d4 v3.0.0+20210330 ([`7836254`](https://github.com/Byron/google-apis-rs/commit/7836254d2b5df09ccedbfdebe1cc847a934b7f5f))
    - Release google-adexchangebuyer1d3 v3.0.0+20210330 ([`cf13a81`](https://github.com/Byron/google-apis-rs/commit/cf13a814ab405823b858581aee457369ed584f2a))
    - Release google-accesscontextmanager1_beta v3.0.0+20220301 ([`1677205`](https://github.com/Byron/google-apis-rs/commit/16772056dca24a676559989bb95046e3a26e31b9))
    - Release google-accesscontextmanager1 v3.0.0+20220301 ([`611b7b5`](https://github.com/Byron/google-apis-rs/commit/611b7b55d7211594af9a9c7f416995881acf79f8))
    - Release google-accessapproval1_beta1 v3.0.0+20200708 ([`5d8b68c`](https://github.com/Byron/google-apis-rs/commit/5d8b68c1d7128f14faf5fe292cf3de050cbae066))
    - Release google-accessapproval1 v3.0.0+20220225 ([`b98ff92`](https://github.com/Byron/google-apis-rs/commit/b98ff9210c402627283961cf51d86a2cb916e3d4))
    - Release google-acceleratedmobilepageurl1 v3.0.0+20220305 ([`0f9c1f7`](https://github.com/Byron/google-apis-rs/commit/0f9c1f740958feeb72a91c6a9e0b96e8a1e29319))
    - Release google-abusiveexperiencereport1 v3.0.0+20220303 ([`c5263ff`](https://github.com/Byron/google-apis-rs/commit/c5263ffeeb6b6db008656cd569e4008f1a945479))
    - regen all APIs and validate them ([`f2c1b82`](https://github.com/Byron/google-apis-rs/commit/f2c1b82112d8a3ee077e45475b7307141774d753))
    - Update all json files; make some fixes to make it work at all ([`743a56f`](https://github.com/Byron/google-apis-rs/commit/743a56f4b7415dd63ccb718578be8c287cbc6da0))
    - update version to 3.0 due to breaking changes in error management ([`d12cd32`](https://github.com/Byron/google-apis-rs/commit/d12cd3238efbe0c92c8b7fc4c81038bf839c1905))
    - remove various errors structs ([`544be6d`](https://github.com/Byron/google-apis-rs/commit/544be6d2a27423865d09f355785310368c4c42ed))
    - bump yup-oauth2 to next major version ([`cc57c6a`](https://github.com/Byron/google-apis-rs/commit/cc57c6a93d1f791adb2aa80161af50858d69d61d))
    - drive3 and youtube3 published ([`16724d2`](https://github.com/Byron/google-apis-rs/commit/16724d2d9be70b8f751d94402aabd01aba70baaf))
    - Release google-youtube3 v2.0.10+20210330 ([`67fee6d`](https://github.com/Byron/google-apis-rs/commit/67fee6d314ff796145d584a16a331a3fac6ab29e))
    - update youtube to latest version ([`151a355`](https://github.com/Byron/google-apis-rs/commit/151a355717797d6110ace03d93ebe1c3e444c248))
    - Keep the set version, and force buidling the API, during publish ([`022e65b`](https://github.com/Byron/google-apis-rs/commit/022e65bc5b52d08f75d2e996a0d80d62caf7ef92))
    - Release google-youtube3 v2.0.9+20210330 ([`86f055b`](https://github.com/Byron/google-apis-rs/commit/86f055be4aa862d084fed57b43a5eca21169771b))
    - Release google-drive3 v2.0.10+20210322 ([`d81ca34`](https://github.com/Byron/google-apis-rs/commit/d81ca34ae75bc23e0eb225b67b6bcc2db4b7aa30))
    - regenerate drive3 ([`98b8818`](https://github.com/Byron/google-apis-rs/commit/98b8818f7d4dd552a7a399448c33b8efef9bf3a5))
    - Bump build version for google drive ([`f1f9dd3`](https://github.com/Byron/google-apis-rs/commit/f1f9dd3f85460166ee16ceca561198dd18fd6fff))
    - fix #307 by reexporting publicly hyper and hyper_rustls ([`49f6651`](https://github.com/Byron/google-apis-rs/commit/49f6651d7e9685a44bb28cd523e4ad5905e94616))
    - Fix typo ([`fac4e09`](https://github.com/Byron/google-apis-rs/commit/fac4e09676f0effd82883da06b202ba62369183d))
    - Release google-gmail1 v2.0.9+20210322 ([`93de38d`](https://github.com/Byron/google-apis-rs/commit/93de38dddd36ab21eb99f782a3a6dbdc4173220f))
    - regenerate gmail1 for release ([`a130864`](https://github.com/Byron/google-apis-rs/commit/a1308641f22eab89878fe96c61786dbf5616e426))
    - Allow access to the client and the hub ([`08abd44`](https://github.com/Byron/google-apis-rs/commit/08abd44cce512c11698836988d679fd050485c4e))
    - Release google-drive3 v2.0.9+20210322 ([`96a2974`](https://github.com/Byron/google-apis-rs/commit/96a297478288cdf3bf043a3ea03d74ce5be4badd))
    - Release google-drive3 v2.0.9+20210322 ([`d8c768d`](https://github.com/Byron/google-apis-rs/commit/d8c768ddc25ce19a8bf63af82f4a828e5d459ada))
    - Release google-drive3 v2.0.9+20210322 ([`f7b6546`](https://github.com/Byron/google-apis-rs/commit/f7b654632c6d4e7fc9ff673742ded81a8e26094d))
    - Release google-drive3 v2.0.9+20210322 ([`c2a86c5`](https://github.com/Byron/google-apis-rs/commit/c2a86c5c5a9f3973e5a94cc693d6e0d5a2ec3774))
    - regen drive3 with oauth2 re-export ([`34d28a0`](https://github.com/Byron/google-apis-rs/commit/34d28a0ddf2be498233243de37e03832b598ca00))
    - adjust version to 2.0.9 for all crates: oauth2 re-export ([`3111593`](https://github.com/Byron/google-apis-rs/commit/31115938af763d79846178f849837693ce6d1e7f))
    - release without changelog by default ([`efcd11e`](https://github.com/Byron/google-apis-rs/commit/efcd11e7c18ef2ded266acf12ab2076963e4b430))
    - Move extern crate import with other extern crate imports ([`3837d05`](https://github.com/Byron/google-apis-rs/commit/3837d0559a0c47cbaf3433e28eb732c1ee98ce40))
    - Fix duplication of oauth2 definition ([`5a1a7b1`](https://github.com/Byron/google-apis-rs/commit/5a1a7b1fb28a5981a68401ff0bbec30b580b04d5))
    - Expose yup_oauth2 crate from inside the crates ([`861a9d8`](https://github.com/Byron/google-apis-rs/commit/861a9d8281388d6cf560eba61f3ebcf0f51a8451))
    - publish result ([`e6619c6`](https://github.com/Byron/google-apis-rs/commit/e6619c65929b2d4495d5a662fd7eec6b2b1befe4))
    - Release google-youtubereporting1 v2.0.8+20210329 ([`6def9f9`](https://github.com/Byron/google-apis-rs/commit/6def9f9e792a7fe74f7efeee9e18df21c3817867))
    - Release google-youtube3 v2.0.8+20210330 ([`0b87813`](https://github.com/Byron/google-apis-rs/commit/0b8781339b24212e224495313eb30ddf7afbe637))
    - Release google-workflows1 v2.0.8+20210318 ([`1cdb3bd`](https://github.com/Byron/google-apis-rs/commit/1cdb3bdce1a74019479a83f43be530b2f72ca957))
    - Release google-workflowexecutions1 v2.0.8+20210316 ([`0bcca36`](https://github.com/Byron/google-apis-rs/commit/0bcca36c86177983feb6c591eb275728406f7e4b))
    - Release google-webrisk1 v2.0.8+20210319 ([`6337956`](https://github.com/Byron/google-apis-rs/commit/633795605d1347df85c75f9e63117ab6a845b1d2))
    - Release google-webmasters3 v2.0.8+20190428 ([`7a2f863`](https://github.com/Byron/google-apis-rs/commit/7a2f86305935ae0731e4e1f5b2adb0cf05f1dc4a))
    - Release google-webfonts1 v2.0.8+20210326 ([`8471257`](https://github.com/Byron/google-apis-rs/commit/8471257a9c7fd1152012b3f974ea7d63a2ab2a5e))
    - Release google-vision1 v2.0.8+20210319 ([`c7214c4`](https://github.com/Byron/google-apis-rs/commit/c7214c40eee5abc1682b54f3259d889ad94f2cd1))
    - Release google-videointelligence1_beta1 v2.0.8+20171122 ([`87f2432`](https://github.com/Byron/google-apis-rs/commit/87f2432d95790c81470471b4d52b57fb5a7763f0))
    - Release google-videointelligence1 v2.0.8+20210325 ([`477b021`](https://github.com/Byron/google-apis-rs/commit/477b02191586b53c97b8242b3437774fb0aed592))
    - Release google-verifiedaccess1 v2.0.8+20210302 ([`61ca99e`](https://github.com/Byron/google-apis-rs/commit/61ca99e02a8032a51eafe5c39da5c84bbe290f9c))
    - Release google-vectortile1 v2.0.8+20210331 ([`0ca5d96`](https://github.com/Byron/google-apis-rs/commit/0ca5d96a02b5200f19c5e8360e540c59688b554b))
    - Release google-vault1 v2.0.8+20210316 ([`330ae4c`](https://github.com/Byron/google-apis-rs/commit/330ae4c39084b2642f790b991afa428ed1145967))
    - Release google-urlshortener1 v2.0.8+20150519 ([`a6ab757`](https://github.com/Byron/google-apis-rs/commit/a6ab757027f963eb27416cb5876946aee7b7bdb3))
    - Release google-translate3 v2.0.8+20210312 ([`7d5db98`](https://github.com/Byron/google-apis-rs/commit/7d5db982deda1ebc068e2680fc0a8d620a98ec23))
    - Release google-translate2 v2.0.8+20170525 ([`eafe2f0`](https://github.com/Byron/google-apis-rs/commit/eafe2f09ab9a421444eda031eeca3701d8416e57))
    - Release google-transcoder1_beta1 v2.0.8+20210323 ([`64579d9`](https://github.com/Byron/google-apis-rs/commit/64579d9c9106833e1ca0fe15bbe2cfc6a3beca7f))
    - Release google-tpu1_alpha1 v2.0.8+20210217 ([`b08d495`](https://github.com/Byron/google-apis-rs/commit/b08d495db2024508c31879937789f0f9e4a4d97e))
    - Release google-tpu1 v2.0.8+20210217 ([`38a9b98`](https://github.com/Byron/google-apis-rs/commit/38a9b98b6fcc5dc0cc2902c2ca9c110a23238e83))
    - Release google-texttospeech1 v2.0.8+20210326 ([`881de07`](https://github.com/Byron/google-apis-rs/commit/881de073c8a68e3af7f1cd62a298fe0ebb7bd432))
    - Release google-testing1 v2.0.8+20210326 ([`bd94c6f`](https://github.com/Byron/google-apis-rs/commit/bd94c6f1a97665dafe218cf861b512df537e8450))
    - Release google-tasks1 v2.0.8+20210329 ([`bb998e5`](https://github.com/Byron/google-apis-rs/commit/bb998e5e8d96e5ea38aebab6ed86d1d703d20853))
    - Release google-taskqueue1_beta2 v2.0.8+20160428 ([`6a691eb`](https://github.com/Byron/google-apis-rs/commit/6a691eb6b23c5edf8d05c20a8432d195750f7c11))
    - Release google-tagmanager2 v2.0.8+20210330 ([`4f25cbb`](https://github.com/Byron/google-apis-rs/commit/4f25cbb35b461a8be346c4685065ae0abf92daf3))
    - Release google-tagmanager1 v2.0.8+20210330 ([`b74a9d4`](https://github.com/Byron/google-apis-rs/commit/b74a9d49b0ee80ce4a1ba40c88e9e84d122b7fd6))
    - Release google-surveys2 v2.0.8+20180508 ([`d8121b3`](https://github.com/Byron/google-apis-rs/commit/d8121b3334b33c5e624ad63563bfb30be2a3eaf9))
    - Release google-sts1 v2.0.8+20210326 ([`9e1afd7`](https://github.com/Byron/google-apis-rs/commit/9e1afd75a400bea38c9f886c0afdaabd49d07fdf))
    - Release google-storagetransfer1 v2.0.8+20210316 ([`fd28ceb`](https://github.com/Byron/google-apis-rs/commit/fd28cebc0fc8347b3a0bb6eb43f65821edf98caf))
    - Release google-sqladmin1_beta4 v2.0.8+20210321 ([`828a704`](https://github.com/Byron/google-apis-rs/commit/828a704d408ad78473dc23598a4e0f62c76c597a))
    - Release google-sql1_beta4 v2.0.8+20200331 ([`1a46153`](https://github.com/Byron/google-apis-rs/commit/1a46153759fb817f07e0dccc2f24fceed390c5c7))
    - Release google-speech1_beta1 v2.0.8+20181005 ([`2d910ac`](https://github.com/Byron/google-apis-rs/commit/2d910acac234979209ee7bfb7f97b7e48551d5d1))
    - Release google-speech1 v2.0.8+20210325 ([`97c5b63`](https://github.com/Byron/google-apis-rs/commit/97c5b635eb50c42e987b48b9cbbd4a7a41ef2ad2))
    - Release google-spectrum1_explorer v2.0.8+20170306 ([`73af769`](https://github.com/Byron/google-apis-rs/commit/73af76975ab07425bfc94be056f8517fa9945080))
    - Release google-sourcerepo1 v2.0.8+20210125 ([`f9fa1ba`](https://github.com/Byron/google-apis-rs/commit/f9fa1bad30953b5ecc2023d9599b115b7576039a))
    - Release google-smartdevicemanagement1 v2.0.8+20210319 ([`9a0f58d`](https://github.com/Byron/google-apis-rs/commit/9a0f58de229ef0833522a055790ae786caf5e2c5))
    - Release google-siteverification1 v2.0.8+20191119 ([`730f6e9`](https://github.com/Byron/google-apis-rs/commit/730f6e90e10886277029a15261dcc760a736457c))
    - Release google-sheets4 v2.0.8+20210322 ([`e1b294c`](https://github.com/Byron/google-apis-rs/commit/e1b294c6b7630dc063e102a076634001cf696423))
    - Release google-serviceregistryalpha v2.0.8+20160401 ([`fd72452`](https://github.com/Byron/google-apis-rs/commit/fd7245209b6cfbea6d63103200db0f139eacb51c))
    - Release google-servicedirectory1_beta1 v2.0.8+20210316 ([`cdf7811`](https://github.com/Byron/google-apis-rs/commit/cdf78112146d54771dd6afc5db70de3ebed1350e))
    - Release google-servicedirectory1 v2.0.8+20210316 ([`1571779`](https://github.com/Byron/google-apis-rs/commit/15717799367789904aafbb33aff8f05c37298969))
    - Release google-servicecontrol2 v2.0.8+20210326 ([`bdb60bd`](https://github.com/Byron/google-apis-rs/commit/bdb60bdf4d5977fc0213054d940fd4a5d06c225d))
    - Release google-servicecontrol1 v2.0.8+20210326 ([`0dcfef8`](https://github.com/Byron/google-apis-rs/commit/0dcfef8025573bf941c242f652fd9ad810a5150c))
    - Release google-servicebroker1 v2.0.8+20190624 ([`ba3da92`](https://github.com/Byron/google-apis-rs/commit/ba3da9254aeca5cd6395ed0da2d96d89842451e4))
    - Release google-securitycenter1 v2.0.8+20210326 ([`20b0259`](https://github.com/Byron/google-apis-rs/commit/20b02590a625ae6cec9b18eb063fee1fb6e6ef41))
    - Release google-secretmanager1_beta1 v2.0.8+20210319 ([`d3efaeb`](https://github.com/Byron/google-apis-rs/commit/d3efaeb80c56b4c8d0d29e799e86ea07ef322081))
    - Release google-secretmanager1 v2.0.8+20210319 ([`dc7ef58`](https://github.com/Byron/google-apis-rs/commit/dc7ef5853ae440d48d3eeec89401e2e79c18034d))
    - Release google-searchconsole1 v2.0.8+20210325 ([`0c2b3a3`](https://github.com/Byron/google-apis-rs/commit/0c2b3a30d2c08cdf4d2c8190f7a4a9e5a43c1260))
    - Release google-sasportal1_alpha1 v2.0.8+20210330 ([`1dc8921`](https://github.com/Byron/google-apis-rs/commit/1dc8921fd8e5a7b9eea44951c220aa53f0f0feca))
    - Release google-safebrowsing4 v2.0.8+20210330 ([`3996d93`](https://github.com/Byron/google-apis-rs/commit/3996d9377e6b02a424093bc3683babcbaa464f89))
    - Release google-runtimeconfig1_beta1 v2.0.8+20210329 ([`2a990a2`](https://github.com/Byron/google-apis-rs/commit/2a990a29c4b1b2a2fdc132be316dd409ec9422e3))
    - Release google-runtimeconfig1 v2.0.8+20210329 ([`415e7d9`](https://github.com/Byron/google-apis-rs/commit/415e7d953a01b828b6802fcca8ad593b02e7bc48))
    - Release google-run1 v2.0.8+20210326 ([`da7f333`](https://github.com/Byron/google-apis-rs/commit/da7f333e1e5cc9d0c4b78bba301debbfb21bab34))
    - Release google-run1 v2.0.8+20210326 ([`71ddafd`](https://github.com/Byron/google-apis-rs/commit/71ddafd997b500e101e1691307698730edcebefa))
    - Release google-retail2 v2.0.8+20210319 ([`5b371a2`](https://github.com/Byron/google-apis-rs/commit/5b371a242bf73f228879e4a5b858c11376d30912))
    - Release google-resourceviews1_beta2 v2.0.8+20160512 ([`e1b9c46`](https://github.com/Byron/google-apis-rs/commit/e1b9c464714bed517341c0cbc0856cec8afe04cb))
    - Release google-reseller1_sandbox v2.0.8+20160329 ([`33ee0fc`](https://github.com/Byron/google-apis-rs/commit/33ee0fcdd1c854c2cc1819f36920315b0d79de57))
    - Release google-replicapoolupdater1_beta1 v2.0.8+20161003 ([`1d6c909`](https://github.com/Byron/google-apis-rs/commit/1d6c9094d492d91892370985ceffcd673c8f9951))
    - Release google-replicapool1_beta2 v2.0.8+20160512 ([`bee756a`](https://github.com/Byron/google-apis-rs/commit/bee756adae51fc9fc1fef44a15d1199e80a1f5e8))
    - Release google-remotebuildexecution2 v2.0.8+20210329 ([`e25c078`](https://github.com/Byron/google-apis-rs/commit/e25c078361ef0abf40691c765d0b213622660d82))
    - Release google-redis1 v2.0.8+20210325 ([`4a0ffd0`](https://github.com/Byron/google-apis-rs/commit/4a0ffd066bcff4e95676e83f3bdc78a77eb9557e))
    - Release google-recommender1_beta1 v2.0.8+20210319 ([`98d89b0`](https://github.com/Byron/google-apis-rs/commit/98d89b071269f3b9932c3d3b65a35d9021ae75db))
    - Release google-recommender1 v2.0.8+20210319 ([`d64e4ff`](https://github.com/Byron/google-apis-rs/commit/d64e4ff677e14ca302e7b700d1122bb86531b512))
    - Release google-recommendationengine1_beta1 v2.0.8+20210319 ([`3e247ba`](https://github.com/Byron/google-apis-rs/commit/3e247ba40edca958d31085ad54bd081ebf5b16da))
    - Release google-realtimebidding1 v2.0.8+20210331 ([`efd899e`](https://github.com/Byron/google-apis-rs/commit/efd899eae92426ba251a552b2efbe3f829fa861e))
    - Release google-qpxexpress1 v2.0.8+20160708 ([`77802b1`](https://github.com/Byron/google-apis-rs/commit/77802b1ee8131184b812470ed78f1ed7252d5626))
    - Release google-pubsublite1 v2.0.8+20210322 ([`fe10f41`](https://github.com/Byron/google-apis-rs/commit/fe10f4118effe15edda1407c0e8d3b9dd0ceaf7e))
    - Release google-pubsub1_beta2 v2.0.8+20210322 ([`e3b58c8`](https://github.com/Byron/google-apis-rs/commit/e3b58c86cd099d551e8f6c665b4688e099c508d1))
    - Release google-pubsub1 v2.0.8+20210322 ([`d418e13`](https://github.com/Byron/google-apis-rs/commit/d418e13e7e0206c5966344c2067b30ae332b5964))
    - Release google-proximitybeacon1_beta1 v2.0.8+20200127 ([`3e91417`](https://github.com/Byron/google-apis-rs/commit/3e914176ce9ad6d0c5035397fe9fb365f66007b9))
    - Release google-prod_tt_sasportal1_alpha1 v2.0.8+20210330 ([`6a128e0`](https://github.com/Byron/google-apis-rs/commit/6a128e070976a272fefd52d0cc4af934d99d47bf))
    - Release google-privateca1_beta1 v2.0.8+20210322 ([`aeb8428`](https://github.com/Byron/google-apis-rs/commit/aeb842861994d4beee5c3bd8a3f3938ff6689be3))
    - Release google-prediction1d6 v2.0.8+20160511 ([`5915c44`](https://github.com/Byron/google-apis-rs/commit/5915c44cfd668b487c5d4f8c3c6cfd2b5c63baa3))
    - Release google-policytroubleshooter1 v2.0.8+20210329 ([`b8a09a7`](https://github.com/Byron/google-apis-rs/commit/b8a09a782b4622592492353fe853212accb42dfd))
    - Release google-policysimulator1 v2.0.8+20210330 ([`cd3b113`](https://github.com/Byron/google-apis-rs/commit/cd3b1137e20dd8555195b7b1a33b97af31a984bb))
    - Release google-plusdomains1 v2.0.8+20190616 ([`b4c32f0`](https://github.com/Byron/google-apis-rs/commit/b4c32f045bea7a3f79dbfeff6aafd19867e31824))
    - Release google-plus1 v2.0.8+20190616 ([`5a9f499`](https://github.com/Byron/google-apis-rs/commit/5a9f4999fbb805e7acea3d3f4ee024461421963f))
    - Release google-playmoviespartner1 v2.0.8+20170919 ([`d6f8362`](https://github.com/Byron/google-apis-rs/commit/d6f8362e9183c5e371ad46053796e732b1e770c6))
    - Release google-playcustomapp1 v2.0.8+20210331 ([`1403d00`](https://github.com/Byron/google-apis-rs/commit/1403d00c11cd2825551734131b56a888d2431b03))
    - Release google-playablelocations3 v2.0.8+20200707 ([`e2cecc2`](https://github.com/Byron/google-apis-rs/commit/e2cecc2c6760473a80512254000ae078d8b440b9))
    - Release google-photoslibrary1 v2.0.8+20210331 ([`ea23d2a`](https://github.com/Byron/google-apis-rs/commit/ea23d2a43ef96077208fb1cdcf8a481f358cc867))
    - Release google-people1 v2.0.8+20210330 ([`408a22b`](https://github.com/Byron/google-apis-rs/commit/408a22b44712ddfc0e81b2b975232b9a751b6822))
    - Release google-partners2 v2.0.8+20180925 ([`eaa5057`](https://github.com/Byron/google-apis-rs/commit/eaa50576fcff55909227aa3fee660bec687c3395))
    - Release google-pagespeedonline5 v2.0.8+20210330 ([`2c40d24`](https://github.com/Byron/google-apis-rs/commit/2c40d2484d5a337caaf338b93d7a457ea1d4e685))
    - Release google-pagespeedonline4 v2.0.8+20191206 ([`1538d2a`](https://github.com/Byron/google-apis-rs/commit/1538d2aa4e63d1919f48fc72d11a5114d615d9da))
    - Release google-pagespeedonline2 v2.0.8+20191206 ([`a0bdf77`](https://github.com/Byron/google-apis-rs/commit/a0bdf778b28e2d9be55392588f6d5da6899a9fc6))
    - Release google-oslogin1_beta v2.0.8+20210316 ([`919a6e8`](https://github.com/Byron/google-apis-rs/commit/919a6e89d0afb69c51de4d005a0ac41679286a1f))
    - Release google-oslogin1 v2.0.8+20210316 ([`3302bd3`](https://github.com/Byron/google-apis-rs/commit/3302bd30ea101d395016481bc85bf4bb5a6d798c))
    - Release google-orgpolicy2 v2.0.8+20210330 ([`f15bd38`](https://github.com/Byron/google-apis-rs/commit/f15bd381ad564a370ade24a2931b8a3c47c5341f))
    - Release google-ondemandscanning1 v2.0.8+20210329 ([`a8ff594`](https://github.com/Byron/google-apis-rs/commit/a8ff594cde6e4aef0aac3e3fd698bf49b8d9bc64))
    - Release google-notebooks1 v2.0.8+20210305 ([`6d77567`](https://github.com/Byron/google-apis-rs/commit/6d7756702e0878216a0d27a42fae2d5c13173a18))
    - Release google-networkmanagement1 v2.0.8+20210325 ([`bf14bf3`](https://github.com/Byron/google-apis-rs/commit/bf14bf3c2b05ccd6f9eb661086f4b16eb1152706))
    - Release google-networkconnectivity1_alpha1 v2.0.8+20210324 ([`83539b5`](https://github.com/Byron/google-apis-rs/commit/83539b5a9a39ce7a9d60c8b99ce1f71621843af3))
    - Release google-mybusinessaccountmanagement1 v2.0.8+20210330 ([`8e8b3d3`](https://github.com/Byron/google-apis-rs/commit/8e8b3d39e61d3f7f754dd3d4f758abe39472cd13))
    - Release google-mybusiness4 v2.0.8+0 ([`52d2fa5`](https://github.com/Byron/google-apis-rs/commit/52d2fa5c1337386f7f7ef64cbfed65df58c52df8))
    - Release google-monitoring3 v2.0.8+20210322 ([`33be560`](https://github.com/Byron/google-apis-rs/commit/33be5604c933fcfbc73b2d00a39cc4ec777ca021))
    - Release google-ml1 v2.0.8+20210317 ([`e16a282`](https://github.com/Byron/google-apis-rs/commit/e16a28257144612c918c7c6f2fe7fb6e19643a57))
    - Release google-mirror1 v2.0.8+20190424 ([`004d458`](https://github.com/Byron/google-apis-rs/commit/004d458de354be370825d593d3a00099e203f3f7))
    - Release google-metastore1_beta v2.0.8+20210325 ([`5104529`](https://github.com/Byron/google-apis-rs/commit/51045294b3f31795dfc3a6d3da1e1249dcd9de76))
    - Release google-memcache1_beta2 v2.0.8+20210324 ([`9a85f21`](https://github.com/Byron/google-apis-rs/commit/9a85f213fb9444054c0d4c879e4d20069c41ce6c))
    - Release google-memcache1 v2.0.8+20210324 ([`4afd3c8`](https://github.com/Byron/google-apis-rs/commit/4afd3c82126975a08dddfc896028948baca77cd7))
    - Release google-manufacturers1 v2.0.8+20210325 ([`cad74c0`](https://github.com/Byron/google-apis-rs/commit/cad74c030455415f86a0a8acf943cf3c9b05642e))
    - Release google-manager1_beta2 v2.0.8+20140915 ([`8779768`](https://github.com/Byron/google-apis-rs/commit/8779768c6733bf0138e54a217848141e7a172a3a))
    - Release google-managedidentities1 v2.0.8+20210324 ([`dcfcdf4`](https://github.com/Byron/google-apis-rs/commit/dcfcdf42123c575d1174cf59986a23a384f06e28))
    - Release google-logging2_beta1 v2.0.8+20190325 ([`d249b4c`](https://github.com/Byron/google-apis-rs/commit/d249b4c87bd55360ec6f326e699caa62a777b116))
    - Release google-logging2 v2.0.8+20210325 ([`2960879`](https://github.com/Byron/google-apis-rs/commit/296087964690b709dddad548b9dd5990681afa28))
    - Release google-localservices1 v2.0.8+20210330 ([`9d14c14`](https://github.com/Byron/google-apis-rs/commit/9d14c14126b5720a694d00a3e36b7e854cf9f7ba))
    - Release google-lifesciences2_beta v2.0.8+20210319 ([`92bbb37`](https://github.com/Byron/google-apis-rs/commit/92bbb37cdc00397c4764e5c3c35958fdcac57cfa))
    - Release google-licensing1 v2.0.8+20210329 ([`a7c9e9b`](https://github.com/Byron/google-apis-rs/commit/a7c9e9b237ed3694a9c3826b2ab29eedd730d616))
    - Release google-libraryagent1 v2.0.8+20210330 ([`635c0a3`](https://github.com/Byron/google-apis-rs/commit/635c0a3e213bf43b7fada5db2163090e7d1de7c3))
    - Release google-language1_beta1 v2.0.8+20210326 ([`2f27342`](https://github.com/Byron/google-apis-rs/commit/2f2734258a2565c74eda6fd5453767543a9cbc26))
    - Release google-language1 v2.0.8+20210326 ([`4c747b0`](https://github.com/Byron/google-apis-rs/commit/4c747b0d3504299752f791afa66f00fa12e98188))
    - Release google-jobs4 v2.0.8+20210309 ([`086d66d`](https://github.com/Byron/google-apis-rs/commit/086d66d9813a1380e596c058f071c1d13441d265))
    - Release google-jobs3 v2.0.8+20210309 ([`cff50fc`](https://github.com/Byron/google-apis-rs/commit/cff50fc7f1b7032563ce1b5a69b0430148044537))
    - Release google-indexing3 v2.0.8+20210323 ([`700a782`](https://github.com/Byron/google-apis-rs/commit/700a782239433c490bf1c77f3d30e3b182c36290))
    - Release google-identitytoolkit3 v2.0.8+20180723 ([`7bbc1b7`](https://github.com/Byron/google-apis-rs/commit/7bbc1b723a692cda4eb50ffb86457593ce564c4c))
    - Release google-iap1_beta1 v2.0.8+20210326 ([`0aab50f`](https://github.com/Byron/google-apis-rs/commit/0aab50fbe29b9d4b6bbdf275735ea1dc0b493989))
    - Release google-iap1 v2.0.8+20210326 ([`3076410`](https://github.com/Byron/google-apis-rs/commit/30764100754a2cc40c3336966926888d802d0134))
    - Release google-iamcredentials1 v2.0.8+20210326 ([`b56fc2b`](https://github.com/Byron/google-apis-rs/commit/b56fc2b1b44d42571eff920a9713c89de33f78b1))
    - Release google-iam1 v2.0.8+20210325 ([`60f3c80`](https://github.com/Byron/google-apis-rs/commit/60f3c80cb429c25f435b45d78a947a2e099dc1bb))
    - Release google-healthcare1_beta1 v2.0.8+20210317 ([`7465865`](https://github.com/Byron/google-apis-rs/commit/746586514b310fe2bd92cc5cf492467eca449687))
    - Release google-healthcare1 v2.0.8+20210317 ([`d87ee49`](https://github.com/Byron/google-apis-rs/commit/d87ee49d51fd2440c4f7060eb30253746ba4b751))
    - Release google-groupssettings1 v2.0.8+20210325 ([`439dee4`](https://github.com/Byron/google-apis-rs/commit/439dee48ce331fa199d095ecef13f24d40eee6f1))
    - Release google-groupsmigration1 v2.0.8+20210318 ([`724d509`](https://github.com/Byron/google-apis-rs/commit/724d509384ae6b546ee6047af1374740322288ed))
    - Release google-gmailpostmastertools1_beta1 v2.0.8+20210330 ([`20a06b4`](https://github.com/Byron/google-apis-rs/commit/20a06b4276284f168dc27fa68e325a8971c489b4))
    - Release google-gmailpostmastertools1 v2.0.8+20210330 ([`3cd820e`](https://github.com/Byron/google-apis-rs/commit/3cd820ebd9b16edcca13d0081ded8ab15beaef24))
    - Release google-gmail1 v2.0.8+20210322 ([`5de1a3f`](https://github.com/Byron/google-apis-rs/commit/5de1a3f70b844abe36f69f75f51ed96078456ccb))
    - Release google-gkehub1 v2.0.8+20210322 ([`9ed1d72`](https://github.com/Byron/google-apis-rs/commit/9ed1d72b0d127246330c59f37a6dd97ede3386df))
    - Release google-genomics1 v2.0.8+20210324 ([`9f3f271`](https://github.com/Byron/google-apis-rs/commit/9f3f2718752a58e8ec008ae7b0f237f272f75b80))
    - Release google-gan1_beta1 v2.0.8+20130205 ([`595b513`](https://github.com/Byron/google-apis-rs/commit/595b513bdcab93566ecc31f674741a36cacbe8a1))
    - Release google-gamesmanagement1_management v2.0.8+20210325 ([`e5c99cf`](https://github.com/Byron/google-apis-rs/commit/e5c99cf237b85232ca4e9d5876423bfadfcdbb6d))
    - Release google-gameservices1 v2.0.8+20210312 ([`8354633`](https://github.com/Byron/google-apis-rs/commit/8354633f0ffe404393736808910faea68dd0218c))
    - Release google-gamesconfiguration1_configuration v2.0.8+20210325 ([`251b136`](https://github.com/Byron/google-apis-rs/commit/251b1361cbe7ba35e00b120f41dd65e5b1cb4e21))
    - Release google-games1 v2.0.8+20210325 ([`e7db793`](https://github.com/Byron/google-apis-rs/commit/e7db79337f09e4db21182b741ff3c7f341787ff2))
    - Release google-fusiontables2 v2.0.8+20171117 ([`49ac341`](https://github.com/Byron/google-apis-rs/commit/49ac3411b8705fa8d66304688c7179687ba1370a))
    - Release google-fitness1 v2.0.8+20210323 ([`edb2ab9`](https://github.com/Byron/google-apis-rs/commit/edb2ab9825372ba4f41ec227cdb16ea7e35cd4c4))
    - Release google-firestore1_beta1 v2.0.8+20210317 ([`b6dd1ac`](https://github.com/Byron/google-apis-rs/commit/b6dd1ace08d5eceb6a5eb10680cedeee1ccd3175))
    - Release google-firestore1 v2.0.8+20210317 ([`9ba3759`](https://github.com/Byron/google-apis-rs/commit/9ba375967d1308366bbf3664063d8b13b267761b))
    - Release google-firebasestorage1_beta v2.0.8+20210329 ([`aee6fb0`](https://github.com/Byron/google-apis-rs/commit/aee6fb0ffe819810a8c47dba13cc145074902b03))
    - Release google-firebaseremoteconfig1 v2.0.8+20171129 ([`162bc4f`](https://github.com/Byron/google-apis-rs/commit/162bc4f60553fbcb4772ffeef5f8128a51a1b728))
    - Release google-firebaseml1 v2.0.8+20210329 ([`f7b1de3`](https://github.com/Byron/google-apis-rs/commit/f7b1de3bf04d8957abf76cf7f11a7888749d252e))
    - Release google-firebasehosting1_beta1 v2.0.8+20210315 ([`d809b50`](https://github.com/Byron/google-apis-rs/commit/d809b5070200fe908e43aac1bd89b2c05bb0ffed))
    - Release google-firebasehosting1 v2.0.8+20210315 ([`787a976`](https://github.com/Byron/google-apis-rs/commit/787a976288ae1fde3f460942eb0983f613ed0916))
    - Release google-firebasedynamiclinks1 v2.0.8+20210329 ([`358c866`](https://github.com/Byron/google-apis-rs/commit/358c866234badcc6ec83f747340fdaed10e14bcb))
    - Release google-firebasedatabase1_beta v2.0.8+20210329 ([`4aeae84`](https://github.com/Byron/google-apis-rs/commit/4aeae84aa31f43de11d2285d773ac245fe913754))
    - Release google-firebase1_beta1 v2.0.8+20210329 ([`322fc83`](https://github.com/Byron/google-apis-rs/commit/322fc83255c8d7a4aff718ff6f121f197701aab7))
    - Release google-file1_beta1 v2.0.8+20210304 ([`2273714`](https://github.com/Byron/google-apis-rs/commit/2273714a80f2ace6a49136a1a7e7ad18f07ae75e))
    - Release google-file1 v2.0.8+20210323 ([`80f6b1d`](https://github.com/Byron/google-apis-rs/commit/80f6b1d0e96d0911366894ef345966a0848ab315))
    - Release google-fcm1 v2.0.8+20210329 ([`d998b31`](https://github.com/Byron/google-apis-rs/commit/d998b3114758b37654fbc27e350c19033b011826))
    - Release google-factchecktools1_alpha1 v2.0.8+20210330 ([`47d6afe`](https://github.com/Byron/google-apis-rs/commit/47d6afe80fd9e064b7e2a0bc09dcdbb7cf5d0453))
    - Release google-eventarc1 v2.0.8+20210325 ([`afe25b2`](https://github.com/Byron/google-apis-rs/commit/afe25b227654422518f7f0427219a38c17eaa3a3))
    - Release google-driveactivity2 v2.0.8+20210326 ([`5b9cdad`](https://github.com/Byron/google-apis-rs/commit/5b9cdad13240d35a4df6b90a89662fc1fefc4385))
    - Release google-drive2 v2.0.8+20210322 ([`f4bc1d5`](https://github.com/Byron/google-apis-rs/commit/f4bc1d52e3d1fc1c3952c02fca52f7361a3ae9df))
    - Release google-doubleclicksearch2 v2.0.8+20210323 ([`bd6956d`](https://github.com/Byron/google-apis-rs/commit/bd6956d4cfb5709d6c29ef460a49273e8a43b7a9))
    - Release google-doubleclickbidmanager1d1 v2.0.8+20210323 ([`74dc158`](https://github.com/Byron/google-apis-rs/commit/74dc1580385374b4d3be37b3733f099159b0db9a))
    - Release google-doubleclickbidmanager1 v2.0.8+20210323 ([`47e7aee`](https://github.com/Byron/google-apis-rs/commit/47e7aee19edda85dd9c8547a9a527b6487a77a34))
    - Release google-domainsrdap1 v2.0.8+20210331 ([`5bd763f`](https://github.com/Byron/google-apis-rs/commit/5bd763fc07f3e1b7980bee687b07bf4c9938ea75))
    - Release google-domains1_beta1 v2.0.8+20210216 ([`6a00b92`](https://github.com/Byron/google-apis-rs/commit/6a00b92d190f9942dd2474cc5d77f772de1c16eb))
    - Release google-documentai1_beta2 v2.0.8+20210329 ([`dcff8f9`](https://github.com/Byron/google-apis-rs/commit/dcff8f92097ec02b87d2397879dbc053fdaa4235))
    - Release google-documentai1 v2.0.8+20210329 ([`54c3394`](https://github.com/Byron/google-apis-rs/commit/54c3394f448c4e05382fa421e1ba1cf52a4768ef))
    - Release google-dns1 v2.0.8+20210319 ([`a98328f`](https://github.com/Byron/google-apis-rs/commit/a98328fba3e2227521fa5cc1c0bf75a4523b1509))
    - Add docs1 (things don't get regenerated automatically it seems) ([`3660439`](https://github.com/Byron/google-apis-rs/commit/36604391ab69eb1c59a70a035fa3f46ff2ba0ae3))
    - Release google-dlp2_beta1 v2.0.8+20171205 ([`ac4057d`](https://github.com/Byron/google-apis-rs/commit/ac4057d3832fa18dd05e8e4d472a0e27db861481))
    - Release google-dlp2 v2.0.8+20210326 ([`fd4a0b7`](https://github.com/Byron/google-apis-rs/commit/fd4a0b7aa455539fb733f823a8e462a6ad29c2a3))
    - Release google-displayvideo1 v2.0.8+20210325 ([`2765749`](https://github.com/Byron/google-apis-rs/commit/2765749fec2e047fda30f55896d253d04e4baa6b))
    - Release google-discovery1 v2.0.8+20200806 ([`f98d99c`](https://github.com/Byron/google-apis-rs/commit/f98d99c19e560477ba1ab400f5d8168f6bb0166d))
    - Release google-digitalassetlinks1 v2.0.8+20210322 ([`e8ee44d`](https://github.com/Byron/google-apis-rs/commit/e8ee44dc9af9c6514fe4fa7e1030a42f13c510b0))
    - Release google-dialogflow3 v2.0.8+20210329 ([`93a8cc7`](https://github.com/Byron/google-apis-rs/commit/93a8cc72444462eb379c118f475dcc70cb1bdc73))
    - Release google-dialogflow2_beta1 v2.0.8+20210329 ([`aed1f57`](https://github.com/Byron/google-apis-rs/commit/aed1f57a17f846d104ceca4b16c5f8751a8fd7b6))
    - Release google-dialogflow2 v2.0.8+20210329 ([`0d913fc`](https://github.com/Byron/google-apis-rs/commit/0d913fcb3bcde59ba290a8bb9c79a3a92e9c0548))
    - Release google-dfareporting3d4 v2.0.8+20210210 ([`f4b310f`](https://github.com/Byron/google-apis-rs/commit/f4b310f08482da11635214095b1d8c94951cbeda))
    - Release google-dfareporting3d3 v2.0.8+20210210 ([`62371ee`](https://github.com/Byron/google-apis-rs/commit/62371eeeb1f79f1acfa0ead9efcd72f4352b95e5))
    - Release google-dfareporting3d2 v2.0.8+20190531 ([`4db1b14`](https://github.com/Byron/google-apis-rs/commit/4db1b1458d60709d458e7593797b866842e2c96d))
    - Release google-dfareporting3 v2.0.8+20180830 ([`61953af`](https://github.com/Byron/google-apis-rs/commit/61953af2876c7414eb28cffe7dedbe452ef2153c))
    - Release google-dfareporting2d8 v2.0.8+20180830 ([`f93540c`](https://github.com/Byron/google-apis-rs/commit/f93540c7f641b0b323f9e27cdfe8a64126ae3599))
    - Release google-deploymentmanager2_beta2 v2.0.8+20160201 ([`1181b16`](https://github.com/Byron/google-apis-rs/commit/1181b1630059a99956f8bce6e2d5b496590c50fb))
    - Release google-deploymentmanager2 v2.0.8+20210320 ([`85204ad`](https://github.com/Byron/google-apis-rs/commit/85204ad91822777e5f345ecc56dcaecb8eddc908))
    - Release google-deploymentmanager2 v2.0.8+20210320 ([`3b04d43`](https://github.com/Byron/google-apis-rs/commit/3b04d431ca2024a55ca65a73966e9bd7ebc175cb))
    - Release google-datastore1_beta3 v2.0.8+20210317 ([`6334012`](https://github.com/Byron/google-apis-rs/commit/633401242a170e6fe4b2fcb253a075ac6c5ea78f))
    - Release google-datastore1 v2.0.8+20210317 ([`8fbc6ee`](https://github.com/Byron/google-apis-rs/commit/8fbc6ee223fe97d5cdf4f44a2835d18b3deede68))
    - Release google-dataproc1 v2.0.8+20210322 ([`90cf506`](https://github.com/Byron/google-apis-rs/commit/90cf506fc75ddeccdb142238b89cd8e80fa54482))
    - Release google-datamigration1 v2.0.8+20210317 ([`f001d7b`](https://github.com/Byron/google-apis-rs/commit/f001d7b352b81a5132d745fad5a708b9576dc527))
    - Release google-datalabeling1_beta1 v2.0.8+20210317 ([`f4eab3b`](https://github.com/Byron/google-apis-rs/commit/f4eab3bc8358e36ee11f8395a6a6a89320dbe40f))
    - Release google-datafusion1_beta1 v2.0.8+20210322 ([`f3365b1`](https://github.com/Byron/google-apis-rs/commit/f3365b16d37aa22a296e0dc4f3bbd7ecff081aa0))
    - Release google-datafusion1 v2.0.8+20210322 ([`3ba60d2`](https://github.com/Byron/google-apis-rs/commit/3ba60d26f2fb1ead2d35d404e5de7ef9e2939848))
    - Release google-datacatalog1_beta1 v2.0.8+20210316 ([`bf0c86c`](https://github.com/Byron/google-apis-rs/commit/bf0c86c88dba00e69788b4dab6c00b5f8e2c1024))
    - Release google-customsearch1 v2.0.8+20210330 ([`86d63bb`](https://github.com/Byron/google-apis-rs/commit/86d63bbfe73350877292316a9536512a487564b5))
    - Release google-coordinate1 v2.0.8+20150811 ([`769dd6e`](https://github.com/Byron/google-apis-rs/commit/769dd6e29afb1af8d278dd8993df46a9924c5a30))
    - Release google-content2_sandbox v2.0.8+20181009 ([`339798b`](https://github.com/Byron/google-apis-rs/commit/339798ba7361d0d42ddbb7cebb31b142b13af0c2))
    - Release google-content2 v2.0.8+20210325 ([`e73e0d7`](https://github.com/Byron/google-apis-rs/commit/e73e0d765bc35511fc69a4f99c8991265390224b))
    - Release google-containeranalysis1_beta1 v2.0.8+20210326 ([`4708f6c`](https://github.com/Byron/google-apis-rs/commit/4708f6ca0d4a76373a0be9857ace94c556d6928f))
    - Release google-container1 v2.0.8+20210314 ([`5ca2adc`](https://github.com/Byron/google-apis-rs/commit/5ca2adc4bdb39d8f2a12c824cef8001170fd7d42))
    - Release google-consumersurveys2 v2.0.8+20170407 ([`d68fa95`](https://github.com/Byron/google-apis-rs/commit/d68fa95e64f1fdb2d062dd54a5ca06d21da7910a))
    - Release google-compute1 v2.0.8+20210316 ([`5ef5114`](https://github.com/Byron/google-apis-rs/commit/5ef511468aeb0aa7e98033d5dbb9e8cbf6061d33))
    - Release google-composer1 v2.0.8+20210319 ([`40c9e85`](https://github.com/Byron/google-apis-rs/commit/40c9e85c44ccc582e78c4e2625b8954e534ad54d))
    - Release google-commentanalyzer1_alpha1 v2.0.8+20200405 ([`2792655`](https://github.com/Byron/google-apis-rs/commit/279265536ee3b98117d8072d5bf9db56615eb2d6))
    - Release google-clouduseraccountsvm_beta v2.0.8+20160316 ([`28211b3`](https://github.com/Byron/google-apis-rs/commit/28211b310130418a8688c7c680333d3a9c28e4ae))
    - Release google-cloudtrace2 v2.0.8+20210319 ([`e0a244e`](https://github.com/Byron/google-apis-rs/commit/e0a244e4d2e378038486ed505fb47029996996ca))
    - Release google-cloudtrace1 v2.0.8+20210319 ([`08dd143`](https://github.com/Byron/google-apis-rs/commit/08dd143e49695bfee582decab3a631be1dcdde58))
    - Release google-cloudtasks2_beta3 v2.0.8+20210315 ([`c315d3f`](https://github.com/Byron/google-apis-rs/commit/c315d3fee53b8fd39a906e613606866ef7fed00d))
    - Release google-cloudtasks2_beta2 v2.0.8+20210315 ([`26fc4d5`](https://github.com/Byron/google-apis-rs/commit/26fc4d57de526c4c1805aa72114de464a439174f))
    - Release google-cloudtasks2 v2.0.8+20210315 ([`a5600bb`](https://github.com/Byron/google-apis-rs/commit/a5600bb8b1101e532b67ff06dce9c830af34dfbe))
    - Release google-cloudshell1 v2.0.8+20210326 ([`45a10d2`](https://github.com/Byron/google-apis-rs/commit/45a10d2a775d67668361fb0f396c33f72edb0d24))
    - Release google-cloudscheduler1_beta1 v2.0.8+20210315 ([`165ae0a`](https://github.com/Byron/google-apis-rs/commit/165ae0ae55e6332621587b3eb6e5136b96042d21))
    - Release google-cloudscheduler1 v2.0.8+20210315 ([`5e2b103`](https://github.com/Byron/google-apis-rs/commit/5e2b10302da003469747756a528aac01439aff2f))
    - Release google-cloudresourcemanager3 v2.0.8+20210328 ([`5523b0c`](https://github.com/Byron/google-apis-rs/commit/5523b0ced54ce0d2c9d9ddd4cd0ce679b1f8bc42))
    - Release google-cloudresourcemanager2 v2.0.8+20210328 ([`ad752c9`](https://github.com/Byron/google-apis-rs/commit/ad752c9f20a34c08d86020ab2277aa10a3b2613d))
    - Release google-cloudresourcemanager1_beta1 v2.0.8+20210328 ([`369be76`](https://github.com/Byron/google-apis-rs/commit/369be76b30a545dfaf719300e4d38ed6944ff236))
    - Release google-cloudresourcemanager1 v2.0.8+20210328 ([`7fe34bd`](https://github.com/Byron/google-apis-rs/commit/7fe34bd832b8e76599ff8187fb300bdf7af8ad79))
    - Release google-cloudprofiler2 v2.0.8+20210326 ([`d23f0b0`](https://github.com/Byron/google-apis-rs/commit/d23f0b0c3becc4b8cfa46d186fc5145ec35419ee))
    - Release google-cloudprivatecatalogproducer1_beta1 v2.0.8+20200405 ([`52521c8`](https://github.com/Byron/google-apis-rs/commit/52521c8b0daf77bdc72f47298bae2e13dd1bc51f))
    - Release google-cloudprivatecatalog1_beta1 v2.0.8+20200405 ([`76d89d5`](https://github.com/Byron/google-apis-rs/commit/76d89d5abfb4f9050f88d15f84547be56fd7a1ef))
    - Release google-cloudmonitoring2_beta2 v2.0.8+20170501 ([`ee9178d`](https://github.com/Byron/google-apis-rs/commit/ee9178da7fb3344d4dcdd6ad58e6d2f61b73e9e5))
    - Release google-cloudlatencytest2 v2.0.8+20160309 ([`a902921`](https://github.com/Byron/google-apis-rs/commit/a9029214c1a147b85214522a26a9203a65ec17ae))
    - Release google-cloudkms1_beta1 v2.0.8+20170515 ([`dd7e58f`](https://github.com/Byron/google-apis-rs/commit/dd7e58f025fc23af029f4411fe8c65cb9fcf046a))
    - Release google-cloudkms1 v2.0.8+20210318 ([`ddcfbe7`](https://github.com/Byron/google-apis-rs/commit/ddcfbe764ec308117958e4e6c92b53aa0220525f))
    - Release google-cloudiot1 v2.0.8+20210323 ([`fa7b40e`](https://github.com/Byron/google-apis-rs/commit/fa7b40eceb0280502e015db10b4c1b2fa9ee980a))
    - Release google-cloudidentity1 v2.0.8+20210310 ([`d5c2aa8`](https://github.com/Byron/google-apis-rs/commit/d5c2aa83707be679ab8b262983a5d29fa406671d))
    - Release google-cloudfunctions1 v2.0.8+20210325 ([`dee139a`](https://github.com/Byron/google-apis-rs/commit/dee139a0cfb23793628da8095ae0684e06d1385a))
    - Release google-clouderrorreporting1_beta1 v2.0.8+20210323 ([`500924f`](https://github.com/Byron/google-apis-rs/commit/500924fd1401dcdd75333f8a67ecdd3fa5583ede))
    - Release google-clouddebugger2 v2.0.8+20210326 ([`8678e62`](https://github.com/Byron/google-apis-rs/commit/8678e626ed9ef9cc0b524b5a1f534888bc7576ff))
    - Release google-cloudchannel1 v2.0.8+20210326 ([`9e2a3d6`](https://github.com/Byron/google-apis-rs/commit/9e2a3d69a0d5683c18dff43fafa22bcec29d9698))
    - Release google-cloudbuild1 v2.0.8+20210329 ([`f9dc002`](https://github.com/Byron/google-apis-rs/commit/f9dc002174ec927dc0d215b8bfebab0107c2ad2c))
    - Release google-cloudbilling1 v2.0.8+20210322 ([`3e78705`](https://github.com/Byron/google-apis-rs/commit/3e7870526933ece333b5f232fc39495b30fe530f))
    - Release google-cloudasset1_beta1 v2.0.8+20210319 ([`0bd425d`](https://github.com/Byron/google-apis-rs/commit/0bd425ddcd95426a7bf15accbc927f211d7c9a04))
    - Release google-cloudasset1 v2.0.8+20210319 ([`fbb7f51`](https://github.com/Byron/google-apis-rs/commit/fbb7f51f1c4053d921b89ec717e4a496c7d864c7))
    - Release google-classroom1 v2.0.8+20210323 ([`cdf2c54`](https://github.com/Byron/google-apis-rs/commit/cdf2c549bb3027916068859551e35531fc12eea2))
    - Release google-chromeuxreport1 v2.0.8+20210330 ([`ad912ff`](https://github.com/Byron/google-apis-rs/commit/ad912ffaba095b4efad6b90f73a52eb3eb96434c))
    - Release google-chromepolicy1 v2.0.8+20210329 ([`1782c62`](https://github.com/Byron/google-apis-rs/commit/1782c62c60b240ca7f5bdf0c4633dc8063a8ed52))
    - Release google-chromemanagement1 v2.0.8+20210330 ([`4cf7cfb`](https://github.com/Byron/google-apis-rs/commit/4cf7cfb4cb55bfd55437b86d7778fd219a0c11bb))
    - Release google-chat1 v2.0.8+20210324 ([`ce7dd54`](https://github.com/Byron/google-apis-rs/commit/ce7dd54f07432969ce7049a3b964e3f6ac721cff))
    - Release google-calendar3 v2.0.8+20210327 ([`d7d0d5f`](https://github.com/Byron/google-apis-rs/commit/d7d0d5f3c4cbfae40d99e3bfb8b41996bbc5a511))
    - Release google-books1 v2.0.8+20210326 ([`b2ed52c`](https://github.com/Byron/google-apis-rs/commit/b2ed52c453ed2fcfd181c316e4683b2629a4bc2d))
    - Release google-blogger3 v2.0.8+20210331 ([`6d0df97`](https://github.com/Byron/google-apis-rs/commit/6d0df970f25895de38a4227d1e03bd37e7a89c00))
    - Release google-binaryauthorization1_beta1 v2.0.8+20210318 ([`ef4ef84`](https://github.com/Byron/google-apis-rs/commit/ef4ef842bb7d3af4d14ad018ce247626751ed328))
    - Release google-binaryauthorization1 v2.0.8+20210318 ([`12724ea`](https://github.com/Byron/google-apis-rs/commit/12724eadbade2d10bccabd26c32861f27c7ee2f6))
    - Release google-billingbudgets1_beta1 v2.0.8+20210326 ([`939f619`](https://github.com/Byron/google-apis-rs/commit/939f61998732d628f619e31eeb68cf86dc64d243))
    - Release google-billingbudgets1 v2.0.8+20210326 ([`715bd61`](https://github.com/Byron/google-apis-rs/commit/715bd6144f4842df3bdffa1771d0d823b455fbd9))
    - Release google-bigtableadmin2 v2.0.8+20210323 ([`05f9e33`](https://github.com/Byron/google-apis-rs/commit/05f9e33c0e64664b31e62963b5fda961a8e74e4d))
    - Release google-bigqueryreservation1 v2.0.8+20210324 ([`272d578`](https://github.com/Byron/google-apis-rs/commit/272d5785b1633766055bdd1f39ec48110a154b79))
    - Release google-bigquerydatatransfer1 v2.0.8+20210324 ([`60f55b8`](https://github.com/Byron/google-apis-rs/commit/60f55b8402266e653d172063fae937cc392933bb))
    - Release google-bigqueryconnection1_beta1 v2.0.8+20210326 ([`e729f80`](https://github.com/Byron/google-apis-rs/commit/e729f8008643e8c4e74f54e712d99b762c08d34d))
    - Release google-bigquery2 v2.0.8+20210327 ([`19459ba`](https://github.com/Byron/google-apis-rs/commit/19459bae6b7e8f1b01ba838c151fc517892fada8))
    - Release google-autoscaler1_beta2 v2.0.8+20150629 ([`b7f1f2d`](https://github.com/Byron/google-apis-rs/commit/b7f1f2df8c7a9aed47eac562e72f0d20259e6190))
    - Release google-assuredworkloads1 v2.0.8+20210325 ([`89186b6`](https://github.com/Byron/google-apis-rs/commit/89186b67850d0956685912893989136032902f18))
    - Release google-artifactregistry1_beta1 v2.0.8+20210324 ([`4cef926`](https://github.com/Byron/google-apis-rs/commit/4cef9266cd44b2bdfc2305e6662e3d51519db2e1))
    - Release google-artifactregistry1 v2.0.8+20210324 ([`56dc0ba`](https://github.com/Byron/google-apis-rs/commit/56dc0ba36001c226ff51b9bfb394df1452bbfa45))
    - Release google-area120tables1_alpha1 v2.0.8+20210330 ([`37fa928`](https://github.com/Byron/google-apis-rs/commit/37fa92889bc355a4e77efe7d46fcbc0aa2749ad2))
    - Release google-appstate1 v2.0.8+20190627 ([`31461ff`](https://github.com/Byron/google-apis-rs/commit/31461ffadac164ea0bb5e2e3ba4b18f1f118827d))
    - Release google-appsactivity1 v2.0.8+20200628 ([`dc37d5f`](https://github.com/Byron/google-apis-rs/commit/dc37d5f83dbbf77921592e4ca97eb90424f48001))
    - Release google-appengine1_beta5 v2.0.8+20181005 ([`cff7c29`](https://github.com/Byron/google-apis-rs/commit/cff7c292a3f6a10e0522d3e111652d7ea3e8fc68))
    - Release google-appengine1_beta4 v2.0.8+20181005 ([`a427784`](https://github.com/Byron/google-apis-rs/commit/a4277846be6e5872d5853bf783996e91f381081c))
    - Release google-appengine1 v2.0.8+20210324 ([`cb04dc6`](https://github.com/Byron/google-apis-rs/commit/cb04dc6da45dbfd145538b877c058add9e85b1ef))
    - Release google-apigee1 v2.0.8+20210319 ([`715b6a8`](https://github.com/Byron/google-apis-rs/commit/715b6a8b1075d6eb634a52752a96438805bf1a37))
    - Release google-apigateway1 v2.0.8+20210318 ([`eba2d92`](https://github.com/Byron/google-apis-rs/commit/eba2d92444cf558222731f43fea67b2977139b0c))
    - Release google-androidpublisher3 v2.0.8+20210401 ([`60fabff`](https://github.com/Byron/google-apis-rs/commit/60fabff03e683d2a5d8476827809bad761364098))
    - Release google-androidpublisher2 v2.0.8+20200331 ([`0a9d7fe`](https://github.com/Byron/google-apis-rs/commit/0a9d7fec43c27cbe20252f227bab4856296d9066))
    - Release google-androidmanagement1 v2.0.8+20210322 ([`3b9b7a8`](https://github.com/Byron/google-apis-rs/commit/3b9b7a813521b9550a40f65c2fd0befda2093955))
    - Release google-androidenterprise1 v2.0.8+20210324 ([`2c457de`](https://github.com/Byron/google-apis-rs/commit/2c457de7842a7d46a8f88fa73144da2fef1e9754))
    - Release google-androiddeviceprovisioning1 v2.0.8+20210331 ([`a79e68e`](https://github.com/Byron/google-apis-rs/commit/a79e68ea52e59e3a692c9a521913635c3e9ff445))
    - Release google-analyticsreporting4 v2.0.8+20210310 ([`59c9108`](https://github.com/Byron/google-apis-rs/commit/59c91086d12f82d2046ec7096afa6fc7898ea9f8))
    - Release google-analyticsdata1_beta v2.0.8+20210329 ([`1974eab`](https://github.com/Byron/google-apis-rs/commit/1974eab1729d4caed458d2b472dbe9bb07715b16))
    - Release google-analyticsadmin1_alpha v2.0.8+20210331 ([`a009dd5`](https://github.com/Byron/google-apis-rs/commit/a009dd50fb1a4c9a78783dd1aac18eed4c218812))
    - Release google-analytics3 v2.0.8+20190807 ([`c47ec83`](https://github.com/Byron/google-apis-rs/commit/c47ec83f5ef6e8fdd0c96725f0726fba40961bc2))
    - Release google-alertcenter1_beta1 v2.0.8+20210323 ([`549d832`](https://github.com/Byron/google-apis-rs/commit/549d832a10c3c4d8c092cfeca8c1d71a687fa71c))
    - Release google-adsensehost4d1 v2.0.8+20200930 ([`324a1a3`](https://github.com/Byron/google-apis-rs/commit/324a1a3339489f268c821bb88d249d0615260c79))
    - Release google-adsense1d4 v2.0.8+20201002 ([`49944d0`](https://github.com/Byron/google-apis-rs/commit/49944d06f2fc445097dea0b7fb76bcaf35fae29e))
    - Release google-admob1 v2.0.8+20210331 ([`880bbbe`](https://github.com/Byron/google-apis-rs/commit/880bbbe199d68aaf5f49aed999d3fd177cf08864))
    - Release google-adexperiencereport1 v2.0.8+20210329 ([`2e830dd`](https://github.com/Byron/google-apis-rs/commit/2e830dd86383306c2e81c7deb922d1aeb0f0507d))
    - Release google-adexchangeseller2 v2.0.8+20171101 ([`4429e20`](https://github.com/Byron/google-apis-rs/commit/4429e20c1858de218c8f554c6843f94a01e2f0a8))
    - Release google-adexchangebuyer2_v2_beta1 v2.0.8+20210331 ([`b663950`](https://github.com/Byron/google-apis-rs/commit/b6639500fd23e205726d8738a4d5cdce955ec42c))
    - Release google-adexchangebuyer1d4 v2.0.8+20210330 ([`d7548d4`](https://github.com/Byron/google-apis-rs/commit/d7548d41a47987c43117bb94375275aa78d02728))
    - Release google-adexchangebuyer1d3 v2.0.8+20210330 ([`b48295d`](https://github.com/Byron/google-apis-rs/commit/b48295de477e0283df79aee74ea7d5c847d86f00))
    - Release google-accesscontextmanager1_beta v2.0.8+20210319 ([`611b786`](https://github.com/Byron/google-apis-rs/commit/611b786bbf30a9374ff22dd76b17db25b5d19aba))
    - Release google-accesscontextmanager1 v2.0.8+20210319 ([`21e9651`](https://github.com/Byron/google-apis-rs/commit/21e96510987faa634e683babd0fd8e58bafd4300))
    - Release google-accessapproval1_beta1 v2.0.8+20200708 ([`2ff4a90`](https://github.com/Byron/google-apis-rs/commit/2ff4a90285e15f613ce65fdb872aa7e954cba9aa))
    - Release google-accessapproval1 v2.0.8+20210318 ([`e325909`](https://github.com/Byron/google-apis-rs/commit/e3259090b12ed9a428cbb7dcc74601ef3bed177c))
    - Release google-acceleratedmobilepageurl1 v2.0.8+20210330 ([`aa1f0bb`](https://github.com/Byron/google-apis-rs/commit/aa1f0bb235a90b5611a1308ad8a139e0c6963cfc))
    - Release google-abusiveexperiencereport1 v2.0.8+20210329 ([`de876c8`](https://github.com/Byron/google-apis-rs/commit/de876c8fea620c1ca4fb8aaa498a64b38a2b3836))
    - regen all APIs ([`d032b0c`](https://github.com/Byron/google-apis-rs/commit/d032b0c96131fea1b49f0bbbc40ac67a322a9ed8))
    - prevent publishing of top-level manifest ([`21914b2`](https://github.com/Byron/google-apis-rs/commit/21914b2c4d935ebb26ecfadb4ba924de5b1193fc))
    - publish tracker file - probably not really needed anymore ([`81380a1`](https://github.com/Byron/google-apis-rs/commit/81380a19883b16abcf15e66a158666df6aaf364a))
    - Release google-drive3 v2.0.8+20210322 ([`1c1e5dc`](https://github.com/Byron/google-apis-rs/commit/1c1e5dcb8207298b48b24bbb9658414872b702d1))
    - regen drive3 ([`96f24fc`](https://github.com/Byron/google-apis-rs/commit/96f24fc4b92e23dead82b76b28a497713be21701))
    - Release google-storage1 v2.0.8+20210330 ([`2fe4d1a`](https://github.com/Byron/google-apis-rs/commit/2fe4d1a8e010fdb98b4e19b9dac9fc9400d8667e))
    - regen storage 1 ([`a9dd6ec`](https://github.com/Byron/google-apis-rs/commit/a9dd6ecd181465bf179e21435752a3f266586467))
    - Bump patch of API version ([`bdfb522`](https://github.com/Byron/google-apis-rs/commit/bdfb522945104ec99e64289b014067f8fd18cddb))
    - keep pubsub marker file ([`b2274ca`](https://github.com/Byron/google-apis-rs/commit/b2274caac46234cafca08c8ea3dde48e55a4c709))
    - Release google-pubsub1 v2.0.7+20210322 ([`9315730`](https://github.com/Byron/google-apis-rs/commit/9315730f8994f836ba264b00bf3afc75046309a5))
    - update pubsub 1 for release test ([`2b567a5`](https://github.com/Byron/google-apis-rs/commit/2b567a5f36192110409aaf9058d8b0278db223cd))
    - Use cargo smart-release for publishing ([`834ad1b`](https://github.com/Byron/google-apis-rs/commit/834ad1b6ecabef087ff3f986481b71804bf2e4b7))
    - publish storage 1 API crate ([`758a859`](https://github.com/Byron/google-apis-rs/commit/758a859bf8d86344bb46944655e68deaa8715929))
    - Regen storage 1 ([`73fdfd4`](https://github.com/Byron/google-apis-rs/commit/73fdfd4fed85f46e2f33edd6ca971067828bf013))
    - Make API top level structure implement Clone, related to #295 ([`32a6d87`](https://github.com/Byron/google-apis-rs/commit/32a6d876a6dab905a17a10af4fe8e9fd8871d20c))
    - Make ReadSeek require 'Send', related to #294 ([`a22c187`](https://github.com/Byron/google-apis-rs/commit/a22c18725c690b763e11a724e69917068f68ac29))
    - republish drive3 ([`b19f955`](https://github.com/Byron/google-apis-rs/commit/b19f955819c61e0b4e4dadf53d175721bc5f8c4e))
    - bump patch level to 2.0.7; regen drive3 ([`732384a`](https://github.com/Byron/google-apis-rs/commit/732384aa36ee90ee3af5a5ced263659d90466d0f))
    - publish storage1 after multi-part fix ([`524d2c7`](https://github.com/Byron/google-apis-rs/commit/524d2c75d00a05e4dff87b97470912080bd1b218))
    - Update storage1 ([`b072625`](https://github.com/Byron/google-apis-rs/commit/b0726252d4b906cc32392cd1dc4ef4bbc480f713))
    - Correct multipart line endings ([`33e83bb`](https://github.com/Byron/google-apis-rs/commit/33e83bb610f627bd729bef5dd562f6b80bff7026))
    - selective release of drive3 ([`0b796f4`](https://github.com/Byron/google-apis-rs/commit/0b796f451db39b77e650f0be66ee0a8db1e52dcf))
    - bump patch level for selective release ([`898c52e`](https://github.com/Byron/google-apis-rs/commit/898c52ec36944fbed5ba2db1848c45e625d7a0ef))
    - Fix resumable uploads ([`19088aa`](https://github.com/Byron/google-apis-rs/commit/19088aa1787e7e3a6406cba4d509441c056e6f02))
    - republish drive3; related to #280 ([`0e278c3`](https://github.com/Byron/google-apis-rs/commit/0e278c383c4814abee92672497206bd44647fbed))
    - bump patch level; regen everything with latest fix ([`1b1b598`](https://github.com/Byron/google-apis-rs/commit/1b1b5985974df23050d4118f9b90a7a988169fe2))
    - removing unused code, seems to be some left-over from upgrading the hyper dependency ([`97f3a94`](https://github.com/Byron/google-apis-rs/commit/97f3a9499d7e122427e79ed7d2e233c1f42488e6))
    - Bug-fix for the upload error: Failed at uploading 'client requires absolute-form URIs' ([`6d56955`](https://github.com/Byron/google-apis-rs/commit/6d569550375f4009c7156cf750c14d9949c2951d))
    - Bunped lib use in the generator, because I got errors when pip tried to install them. Also fixed dependency warning ([`ee64b75`](https://github.com/Byron/google-apis-rs/commit/ee64b751eab809dbe1c125e23f866d88c5395f1a))
    - All publishes done ([`a16a5a2`](https://github.com/Byron/google-apis-rs/commit/a16a5a2758d106e118aa5b32bda006f57a1bc82e))
    - publish batch 3 ([`7eaaeba`](https://github.com/Byron/google-apis-rs/commit/7eaaebac8c5e38ea73cc654d07bae3513302eb72))
    - enable MAINTENANCE MODE :) ([`dc1e86b`](https://github.com/Byron/google-apis-rs/commit/dc1e86b4689d8c786cda65eb4d13226dbee269a6))
    - publish batch 2 ([`f7f665e`](https://github.com/Byron/google-apis-rs/commit/f7f665e80d9739ecf805805cc22d106c7a6aaca7))
    - publish batch 1 ([`95ac545`](https://github.com/Byron/google-apis-rs/commit/95ac545c46b67d8cf5dc4384751a4d363eeaee0b))
    - regen everything with fixed Send for Delegate ([`0adccb5`](https://github.com/Byron/google-apis-rs/commit/0adccb51c5c34bc0632dca971a437ed7df58afc8))
    - pump patch level ([`df5f102`](https://github.com/Byron/google-apis-rs/commit/df5f102bd4dfca00580630943c02d0459a141a0b))
    - second batch, probably needs more work and another publish :/ ([`86eb748`](https://github.com/Byron/google-apis-rs/commit/86eb74825b0e019e5fe93f58380e6b6d99af6d48))
    - first batch of publishings ([`f7bb712`](https://github.com/Byron/google-apis-rs/commit/f7bb712600fbc33d3220d2806bee18c0924e40ce))
    - regenerate everything ([`34096d3`](https://github.com/Byron/google-apis-rs/commit/34096d3862a9278d388358113df2dd7b955a69d7))
    - Fix CLI generator ([`65f9c22`](https://github.com/Byron/google-apis-rs/commit/65f9c2209496082f0dfde54e0f24fb25991aeaca))
    - republish drive3 (fixes #271) ([`feb88cb`](https://github.com/Byron/google-apis-rs/commit/feb88cbc1230b47c3083661a954b04d0952dfbb6))
    - Bump patch level ([`b679c86`](https://github.com/Byron/google-apis-rs/commit/b679c86820e698b40f97d8a37eb9b908a53bf472))
    - Simplify type system to leverage 'Sync' types ([`9c339da`](https://github.com/Byron/google-apis-rs/commit/9c339da955268502cc055d27e4f08e7fbf4203ee))
    - Remove RefCell around autenticator - they are sync now ([`fe6ccfe`](https://github.com/Byron/google-apis-rs/commit/fe6ccfed3f4b78d757f917c106eb4370eb800ffb))
    - rerelease most offected crates with patch ([`5dc84bd`](https://github.com/Byron/google-apis-rs/commit/5dc84bd041a0385b1d5100a4c4a07df3d81bbc1a))
    - Prepare patch release for drive3 and youtubt3 ([`613546a`](https://github.com/Byron/google-apis-rs/commit/613546a2434577238f364f628b582edce9828cf8))
    - Merge branch 'harababurel/main' into main ([`74d2695`](https://github.com/Byron/google-apis-rs/commit/74d2695de9ccad8b5ceaca32582addd4640287ac))
    - :iter not needed ([`819e1cc`](https://github.com/Byron/google-apis-rs/commit/819e1ccce5c503329bf6ed5dd9078553a48997c5))
    - Run code through cargo fmt. ([`83fed44`](https://github.com/Byron/google-apis-rs/commit/83fed44db0d5f436a5120aed0c3d13cc766b91d6))
    - Fix clippy warnings. ([`6ec4682`](https://github.com/Byron/google-apis-rs/commit/6ec46827e7719da88c5dfda67b138bf93bdadd47))
    - Use hyper::body::to_bytes() instead of aggregate(). This should not truncate the content. ([`30ea80b`](https://github.com/Byron/google-apis-rs/commit/30ea80bf31c66c26e83997b1b14bc8b93980a721))
    - cleanup exclude-list ([`a3e2835`](https://github.com/Byron/google-apis-rs/commit/a3e2835a89876cd6e20a41ecce92a8a4a65cf8cc))
    - Don't list APIs which don't have a description anymore ([`2704de1`](https://github.com/Byron/google-apis-rs/commit/2704de169108100e519491be704938283007cf41))
    - refactor ([`18b80cf`](https://github.com/Byron/google-apis-rs/commit/18b80cf37f3d4cf55c96a7cd4fc3b7f0905daede))
    - Fix instructions on how to update json files ([`e15f670`](https://github.com/Byron/google-apis-rs/commit/e15f67075ee473082eb68f700469a9cf5ecf53f9))
    - cleanup; fix security issue ([`9df245a`](https://github.com/Byron/google-apis-rs/commit/9df245a80d91e6be69977b0b21cb14d2bd359c11))
    - Adjust links in generated files to point to 'main' branch ([`7d36eaa`](https://github.com/Byron/google-apis-rs/commit/7d36eaa4099c8d4b8937b79e03eb7cd42e26535d))
    - republuish drive3 with patch ([`ba79f81`](https://github.com/Byron/google-apis-rs/commit/ba79f8117fb5dbb7f8bef54c2037653b41c31a54))
    - Rebuild drive3 to get file download patch; bump patch level ([`8d13292`](https://github.com/Byron/google-apis-rs/commit/8d132921757f969111c29a86c1a42db95674498c))
    - Complete v2 publish ([`7c96341`](https://github.com/Byron/google-apis-rs/commit/7c9634185a100fd8228cc960a4ef15afad6839de))
    - another batch of publishes ([`2d76e3b`](https://github.com/Byron/google-apis-rs/commit/2d76e3b748c3926dadedc55be55425cc4e1d72a8))
    - regen with patch #265, but without version bump ([`5bac5d1`](https://github.com/Byron/google-apis-rs/commit/5bac5d1af569374905df1d7524eb1c080c9935c0))
    - Bug fix: It was not possible to use FileGetCall to download binary files ([`1f776dd`](https://github.com/Byron/google-apis-rs/commit/1f776ddc54166b8d3cd59489a92e5bba429d004d))
    - another batch of 15 - asked for raising the limit ([`d71efba`](https://github.com/Byron/google-apis-rs/commit/d71efba45b538f0e3d32c529c6f57c574a16958a))
    - enable funding ([`257f53a`](https://github.com/Byron/google-apis-rs/commit/257f53a24f0f6cdfb795e9ca5820aca24cd8e4d4))
    - another batch ([`7dbf7e6`](https://github.com/Byron/google-apis-rs/commit/7dbf7e625dec3d8a75e08d49fdb877f05fb30c49))
    - second batch of publishes ([`e352478`](https://github.com/Byron/google-apis-rs/commit/e352478ab8527783554611e53a2a669779135878))
    - fix GHP-import in makefile ([`2bfeeb9`](https://github.com/Byron/google-apis-rs/commit/2bfeeb99f939ceb7bee67adcf5722d933078ca2c))
    - first batch of releases ([`503593b`](https://github.com/Byron/google-apis-rs/commit/503593b907f70fe6a468a97856d30375c9d36068))
    - prepare release of version 2.0, regen all APIs ([`c606d37`](https://github.com/Byron/google-apis-rs/commit/c606d37dd38af6586633b2bf6818d89237889842))
    - Bump major version ([`f1c05f2`](https://github.com/Byron/google-apis-rs/commit/f1c05f2554f2d9a2c497c05d4b09fbf57349b491))
    - Update all APIs ([`c28ba92`](https://github.com/Byron/google-apis-rs/commit/c28ba92ead25a5a4482690f0c87c45fe0d6ed186))
    - port API updates to python3 ([`9fe80b2`](https://github.com/Byron/google-apis-rs/commit/9fe80b24eb28f4783143bf33ed49b6f892c89c9f))
    - downgrade to compatible mkdocs ([`d1ceab4`](https://github.com/Byron/google-apis-rs/commit/d1ceab4cc5dfa10262db37d19ee32b8356739e43))
    - Use ghp-import of virtual environment ([`67c5495`](https://github.com/Byron/google-apis-rs/commit/67c5495c64721706c29778beb18bde4a03477708))
    - Add previously missed changes, CI will be happy. ([`0086b48`](https://github.com/Byron/google-apis-rs/commit/0086b481ea7412625121f8338b4c060e3abef3eb))
    - Fix docs, OMG, finding this was *hard* ([`11fae83`](https://github.com/Byron/google-apis-rs/commit/11fae8353b94f190fa83848cb9e082b9bf4c47ff))
    - Fix action runner ([`b272b3f`](https://github.com/Byron/google-apis-rs/commit/b272b3f4e25a4ac4a4c2a8eeb909786bc22fd198))
    - Fix doc tests ([`c4d4013`](https://github.com/Byron/google-apis-rs/commit/c4d40133582f4ff0bb63de6381beaa3f729b7b09))
    - Fix issue with handling downloads the brutal way… ([`e363097`](https://github.com/Byron/google-apis-rs/commit/e363097e43f0cfe27ce0ede7b3837cad5773e5ab))
    - disable resumable downloads for CLIs ([`06550d3`](https://github.com/Byron/google-apis-rs/commit/06550d3a55f4389c62ac392647859ab40d1dbe94))
    - Groupsmigration-cli compiles, even though resumable uploads are gone. ([`69acd16`](https://github.com/Byron/google-apis-rs/commit/69acd16746757918896cf2a910a3b9fa22d97564))
    - Just one more issue with groupsmigration - unexhaustive match ([`1569ff4`](https://github.com/Byron/google-apis-rs/commit/1569ff47b7b3d07defd4f5cf1be299052df736d7))
    - refactor ([`1a9d155`](https://github.com/Byron/google-apis-rs/commit/1a9d155449a91f9bd0db78354acad8c8cc61fa80))
    - Remove support for debug logging of CLI programs ([`0bf46b9`](https://github.com/Byron/google-apis-rs/commit/0bf46b95f35d2148b89e298fc433a00fb0028bd6))
    - Blacklist 'homegraph' API 'async' keyword is used as field name ([`112ef3f`](https://github.com/Byron/google-apis-rs/commit/112ef3f3ac2e1e64295ff6a008426c6ba5facea9))
    - Youtube 3 compiles; remove 'api_key()' authenticator call… ([`84655f8`](https://github.com/Byron/google-apis-rs/commit/84655f81db959f8f6da259204a0ba235e747378a))
    - Groupsmigration compiles ([`891f126`](https://github.com/Byron/google-apis-rs/commit/891f126a569e936d2fb201e8af26bf3294615328))
    - Make mime 0.2 work with more recent hyper ([`3dbb53e`](https://github.com/Byron/google-apis-rs/commit/3dbb53e924536b7854c2a7cf4d40eb95372316dc))
    - Fixes for CLI code ([`0236e24`](https://github.com/Byron/google-apis-rs/commit/0236e24291e2e7323d768648cf9db83e822a0715))
    - Update hyper, rustls and yup-oauth2 ([`b8a66bc`](https://github.com/Byron/google-apis-rs/commit/b8a66bc666354b262675e2ab4a25beb11c78386a))
    - Fix rust compile warnings ([`b88e163`](https://github.com/Byron/google-apis-rs/commit/b88e1633edf4e4ba9247778b72d2b15417fbd2ef))
    - Merge branch 'master' into namespaced-items ([`60e50da`](https://github.com/Byron/google-apis-rs/commit/60e50da47633c9d2724958afa91016f2365e1f7c))
    - Merge branch 'toolchain-update' ([`0cd2238`](https://github.com/Byron/google-apis-rs/commit/0cd223849887ae0bbb042fadfde43264c3f7217e))
    - make calling virtualenv independent of the binary… ([`1b87b1c`](https://github.com/Byron/google-apis-rs/commit/1b87b1ca6b0cde391ee70778d6629d78f30b907f))
    - Don't rely on pip user installations to be in the PATH ([`904b46d`](https://github.com/Byron/google-apis-rs/commit/904b46d1b2633371e98404f5addce59fb5f1bd46))
    - fix sorting of properties by name ([`3a4151b`](https://github.com/Byron/google-apis-rs/commit/3a4151b38cfd40ec0117f88428e813c9994653fc))
    - everything runs through now, only ordering missing ([`f7aba51`](https://github.com/Byron/google-apis-rs/commit/f7aba5114e2fc7595c1c819dae0778faa3ab30da))
    - nearly there… ([`80dadc5`](https://github.com/Byron/google-apis-rs/commit/80dadc5a80ea3965d7f3f8ae10dcf34cb5fbcb4a))
    - basic python 3 port, nearly works ([`043f375`](https://github.com/Byron/google-apis-rs/commit/043f3752e67b3c08d3db162e37f95a3df3293609))
    - Some simplification, but it runs venv install way too often ([`2573462`](https://github.com/Byron/google-apis-rs/commit/2573462792436744d6bc6c8bcdfea8568358dbd3))
    - Basic fixups for python3 ([`301f140`](https://github.com/Byron/google-apis-rs/commit/301f140c83cc17e32da76573d4f10e3d80ede99b))
    - pyenv is working, at least locally and without perfect isolation ([`7aba410`](https://github.com/Byron/google-apis-rs/commit/7aba410489a6cbc4c4d898350ff558e061aaf779))
    - fix cli imports ([`9ff51f9`](https://github.com/Byron/google-apis-rs/commit/9ff51f941730ccd9e7739e482e75c8f3ad6c1ced))
    - blacklist the spanner API ([`b2244df`](https://github.com/Byron/google-apis-rs/commit/b2244df4e0450fc06152a0f334ddeaebe2c8e77a))
    - remove unused import that was causing a clash ([`87c9c10`](https://github.com/Byron/google-apis-rs/commit/87c9c10ce171a7686520da131f095e1453e8af19))
    - handle cases where part is a repeated property ([`684d54d`](https://github.com/Byron/google-apis-rs/commit/684d54d445f25d50a2ebeaf3acef603da3c06bbc))
    - remove gmail from the blacklist ([`33aa680`](https://github.com/Byron/google-apis-rs/commit/33aa680b6fafa920b39345760c65a88bcec14c22))
    - rename cmn to client ([`af73936`](https://github.com/Byron/google-apis-rs/commit/af73936b9536b3844d8bc0a05432d413de58f0aa))
    - cleanup top-level namespace and organize items under cmn and api ([`0e69ffc`](https://github.com/Byron/google-apis-rs/commit/0e69ffc9eb36f31ab4695299f9812b71550ce872))
    - use direct references to types to generate documentation ([`b5923b2`](https://github.com/Byron/google-apis-rs/commit/b5923b277cd8f97411f8b63ae5aae0eaad5a729e))
    - use fully qualified names to access items from cmn ([`38b1ed1`](https://github.com/Byron/google-apis-rs/commit/38b1ed145cbd6740ba84345081d4f70da60e9088))
    - Don't expect any particular commit format for contributions ([`5c071ba`](https://github.com/Byron/google-apis-rs/commit/5c071ba03c2c060ce0ff0e40fe12dd7cc9ba6aa3))
    - last batch ([`4c4c669`](https://github.com/Byron/google-apis-rs/commit/4c4c6694410272c1e09ea4185a84c6b0cb0cde96))
    - second batch of published crates ([`221fe96`](https://github.com/Byron/google-apis-rs/commit/221fe9606c22028cdb57cee75fde152c0c070489))
    - intermediate result of republish ([`7bdbebf`](https://github.com/Byron/google-apis-rs/commit/7bdbebf4585f8c41c72da8012787c59e217958b1))
    - update all code after version update ([`d1ffa68`](https://github.com/Byron/google-apis-rs/commit/d1ffa68287a9d1ee81b3010a0a3f9fbd2a4176f0))
    - Bump patch level to prep for release ([`c949124`](https://github.com/Byron/google-apis-rs/commit/c949124df5b97dd24ebdfef3e4683342a9d6e693))
    - update API descriptions ([`69fb05c`](https://github.com/Byron/google-apis-rs/commit/69fb05c4e1d47365f87fc4c0f34b7af3b7ca79ab))
    - Update the PubSub schema to include new API features. ([`b6ee34d`](https://github.com/Byron/google-apis-rs/commit/b6ee34dcff4ff0f7d0f38c2e32ebac41b1b69dd5))
    - Fix '-force' target generation for JSON schemata. ([`6ad88cb`](https://github.com/Byron/google-apis-rs/commit/6ad88cb7733d6dc368104709366d0a3666fbd610))
    - Use api overrides to specialize building of drive2/3 APIs ([`5e5327f`](https://github.com/Byron/google-apis-rs/commit/5e5327f3f789db0e451de39e0d14c708426be24e))
    - Avoid talking to google each time dependencies are built… ([`e06c7c6`](https://github.com/Byron/google-apis-rs/commit/e06c7c67e0d55d30eb7b5e6a69ac9227f333d860))
    - Added the `doit_without_upload` function for the `drive3` & `drive2` lib/api ([`8f7ed2e`](https://github.com/Byron/google-apis-rs/commit/8f7ed2e5e9621e7e9f824f397a68dcf9cf6df820))
    - Added the `doit_without_upload` function to the code generator ([`041f895`](https://github.com/Byron/google-apis-rs/commit/041f8951e3409e26c511350a73c1aef0b26d307e))
    - Rebuild everything, becuase it makes it easier to compare everything to my changes ([`1cc94d5`](https://github.com/Byron/google-apis-rs/commit/1cc94d5281952c9b642be332254364ebe7e85389))
    - Extend attempted error message parsing to also cover ErrorResponse objects. Fixes errors in e.g. drive3 api not getting passed to delegate error handlers. ([`6ed8df5`](https://github.com/Byron/google-apis-rs/commit/6ed8df5f96c80c98d0a3b03e23e77a7edfd76459))
    - Fix typo: enocodable -> encodable ([`8e9ce08`](https://github.com/Byron/google-apis-rs/commit/8e9ce08aa21efc6309dc675b400a4bfbc457a1a0))
    - remaining working crates published ([`0b513a9`](https://github.com/Byron/google-apis-rs/commit/0b513a9a536ae115b1101438bafcb03bf3d2cc97))
    - initial bunch of releases 1.0.13 ([`42e2e44`](https://github.com/Byron/google-apis-rs/commit/42e2e44e4c693c6873be65e7036931417944166c))
    - regenerate all crates with updated patch level ([`d302542`](https://github.com/Byron/google-apis-rs/commit/d302542d8d7961821a5a167abb0ad905fa31f7c3))
    - update all json files ([`aacc30f`](https://github.com/Byron/google-apis-rs/commit/aacc30f08de2e8bab0df2d1c5e680792b4b1daeb))
    - update badges ([`ea3b428`](https://github.com/Byron/google-apis-rs/commit/ea3b428364b2d90cb93266e5317f8c48159eb31a))
    - It should work without pyenv, given that the github virtualenv has it installed ([`4d389e1`](https://github.com/Byron/google-apis-rs/commit/4d389e1f81946e40ed4afe102c7b37b36d3e4f97))
    - Make 'cargo test' work by pinning relevant versions ([`94cf4ad`](https://github.com/Byron/google-apis-rs/commit/94cf4ad5690261cca0e134e8241806d537691a0a))
    - Emulate TRAVIS to trigger short path ([`1b09b3a`](https://github.com/Byron/google-apis-rs/commit/1b09b3a2abf736021f3a30e133a9b6b22d72dc27))
    - Now also disable plugin autoload for pytest to avoid sqlite3 ([`f3a4fa1`](https://github.com/Byron/google-apis-rs/commit/f3a4fa109f7aaeef310427581ade26149ffd280c))
    - Don't use python test coverage in the hopes it won't need sqlite3, which isn't there ([`62441f3`](https://github.com/Byron/google-apis-rs/commit/62441f3dc0161f5b76656d86d175cab934977b2d))
    - Let's see if this make-file adjustment helps with the python path ([`092f8bc`](https://github.com/Byron/google-apis-rs/commit/092f8bc7bff02a00b34a9e425f443a904b6ad7da))
    - maybe the action will work ([`f9c7ff9`](https://github.com/Byron/google-apis-rs/commit/f9c7ff918e76eaaf4fc0c3d13d82050661c5f27d))
    - maybe the most recent python 2.7 version can build… ([`bcb71ec`](https://github.com/Byron/google-apis-rs/commit/bcb71ecf212f264a750bf2d8cad64cc9ca1068b9))
    - See if it can run on macOS ([`23de20f`](https://github.com/Byron/google-apis-rs/commit/23de20fcc7fdbda29763655913071bb795996c41))
    - sudo the build system installation ([`4569390`](https://github.com/Byron/google-apis-rs/commit/4569390f0a302210c2c14fe3626a37d0f32339d8))
    - also prepare build system before trying to use pyenv ([`8a7575d`](https://github.com/Byron/google-apis-rs/commit/8a7575d376c8ac029782b5e9f3cb0ffd0affc744))
    - And once more with feeling ([`2b0e536`](https://github.com/Byron/google-apis-rs/commit/2b0e5362712051e4b6ba6b749db0fb47a4304f75))
    - add all suggestions to pyenv ([`be34fc3`](https://github.com/Byron/google-apis-rs/commit/be34fc3ad1da9d24f2bcdd771bf6e540e036fb96))
    - let's try sourcing bashrc right after changing it ([`071950d`](https://github.com/Byron/google-apis-rs/commit/071950db5630601be4c2f9eb6f91a17d6817f966))
    - fix pyenv path, hopefully ([`99ae297`](https://github.com/Byron/google-apis-rs/commit/99ae297d5ea829370ad391d0be0f68cfa91572c3))
    - See if this solves the pyenv installation issue ([`d061b90`](https://github.com/Byron/google-apis-rs/commit/d061b902f35becd935e5a3e9ad213697f65f217a))
    - try to install pyenv before using it ([`c4d8ef7`](https://github.com/Byron/google-apis-rs/commit/c4d8ef799cb39757b72955bdbc28f67287ca46e1))
    - fix syntax error ([`00d68b9`](https://github.com/Byron/google-apis-rs/commit/00d68b9d486502a3bb792cb40433eb840cfcb174))
    - See if configuring pyenv does the job ([`b587cc8`](https://github.com/Byron/google-apis-rs/commit/b587cc865d7c142b2ee62105440dd7b423b14c21))
    - Github actions running the tests travis runs usually ([`cd8e834`](https://github.com/Byron/google-apis-rs/commit/cd8e8345532842d5095f67c9bcfca5db718fcabd))
    - publish additional crates ([`ebbddb3`](https://github.com/Byron/google-apis-rs/commit/ebbddb30aa0ec2f1997238bf362e8d648fd3cc24))
    - revome auto formatting ([`0a013dd`](https://github.com/Byron/google-apis-rs/commit/0a013ddae6d1f2e252fefe9d6b45d3f2acb9d512))
    - added my business api ([`d5519b9`](https://github.com/Byron/google-apis-rs/commit/d5519b9dfe345181d74793bfb338c8181c47588d))
    - Regenarate all files ([`80bf449`](https://github.com/Byron/google-apis-rs/commit/80bf4495f733e52785e49b2e904a7404a0411944))
    - Fix deprecation warnings in CLI ([`b66f868`](https://github.com/Byron/google-apis-rs/commit/b66f868ed10138c8397432f86c19577aeca45906))
    - Rewrite deprecated features, such as try! ([`65b8f00`](https://github.com/Byron/google-apis-rs/commit/65b8f0063dd4ba3a3f263e4a81a3e5158a5db0ef))
    - Remove linux for now to get tests green ([`a0e0312`](https://github.com/Byron/google-apis-rs/commit/a0e0312f69cec42643801347fd67e92c975ae5b2))
    - Try an older python version once again ([`21e829b`](https://github.com/Byron/google-apis-rs/commit/21e829bc4eb3803e8767423acedd0f69f60460cc))
    - Let's see if a more recent ubunutu does the trick ([`1ae7a49`](https://github.com/Byron/google-apis-rs/commit/1ae7a49f2e5e1e8d93d7cf4640acce5860d903c2))
    - Fix unit tests ([`f350baa`](https://github.com/Byron/google-apis-rs/commit/f350baa4a7667926a19b298ce612cabe684c602c))
    - remove orphan/unavailable API: doc1 ([`4209c20`](https://github.com/Byron/google-apis-rs/commit/4209c20caf2762cc18aefedb0a9337fdc9987bfa))
    - publish result ([`fd77683`](https://github.com/Byron/google-apis-rs/commit/fd776832a87dbc07e040795654ebe5caefd1e676))
    - regenerate all crates ([`f125912`](https://github.com/Byron/google-apis-rs/commit/f125912384fffaf76411442d94cf641b9793e9d5))
    - Bump version prior to release ([`f80ef20`](https://github.com/Byron/google-apis-rs/commit/f80ef206bc6f3030e44c917b4db6f361fcbfaefe))
    - Truncate all files we open to fix #240 ([`eb3a6ef`](https://github.com/Byron/google-apis-rs/commit/eb3a6ef578ca7805cb0424b26ddf15a27838c12b))
    - Regenerate all code ([`11db47c`](https://github.com/Byron/google-apis-rs/commit/11db47ca2ab87d2ea02b605f726d422d5c160e9a))
    - Fields in Error related structs are now accessible ([`bcc72e4`](https://github.com/Byron/google-apis-rs/commit/bcc72e4fc1bf9a7c01fbd00f0a7593d03b01d2cc))
    - Keep track of release of storage1 ([`3c1e509`](https://github.com/Byron/google-apis-rs/commit/3c1e509a2538f9a727a34be136e868d19b163186))
    - Related to #238: publish google-storage1 ([`114b9be`](https://github.com/Byron/google-apis-rs/commit/114b9be7e12a1d7f57cd7476825f73b31e87abca))
    - Another python version - probably it doesn't work thanks to openssl ([`d3b047c`](https://github.com/Byron/google-apis-rs/commit/d3b047c880560b2cbd15ec2b8ca93cd58b1891e3))
    - Try another python version ([`6831127`](https://github.com/Byron/google-apis-rs/commit/68311276ab14917581a9d901b96308484678b99d))
    - Try a more recent python version ([`ecccda1`](https://github.com/Byron/google-apis-rs/commit/ecccda15d518e2188b8ab66e870d198ef4445c7f))
    - Don't try to install pyenv - it's now pre-installed ([`abb2fa9`](https://github.com/Byron/google-apis-rs/commit/abb2fa9a810c9ba2458be13f0617b35a17bc298c))
    - Expose features from yup-oauth2 ([`f7ac5eb`](https://github.com/Byron/google-apis-rs/commit/f7ac5ebe36fd9d6c1f1817e0a47693c4a1c33721))
    - Finalize release 1.0.10 - all worked well ([`8cf36a3`](https://github.com/Byron/google-apis-rs/commit/8cf36a3504b379fb359013a1d7f8cee83c7cff8f))
    - On the way to publishing 1.0.10 ([`770f018`](https://github.com/Byron/google-apis-rs/commit/770f01826f1bd8f4f6945024980e1692ce72a5b5))
    - update version to 1.0.10; republish 4 select crates ([`8c1ca2a`](https://github.com/Byron/google-apis-rs/commit/8c1ca2aa9b324174a2ce69349d12ca0ef2847fda))
    - Finish publishing 1.0.9 ([`82e9070`](https://github.com/Byron/google-apis-rs/commit/82e90709fcb2b8a64ebdd5f8120ea4e9a1caa76d))
    - Align CLI generator with changes in string type handling ([`5b370e4`](https://github.com/Byron/google-apis-rs/commit/5b370e41900053f9b3bee93fbf71d8b021a71ca9))
    - blacklist osconfig ([`aceb717`](https://github.com/Byron/google-apis-rs/commit/aceb717ead315d8301a2d4308db8722798302cb7))
    - More releases ([`03bfd38`](https://github.com/Byron/google-apis-rs/commit/03bfd3833bd83f904550c3e4e33029a06125dda8))
    - Now cargo-doc can be run on all libraries ([`94deb74`](https://github.com/Byron/google-apis-rs/commit/94deb7433fe7e57992acb254fdef6800af88ace5))
    - And once again, python string handling bit me... ([`4ceeac6`](https://github.com/Byron/google-apis-rs/commit/4ceeac632054183d400692b838ec33e043216e34))
    - Regen all remaining APIs with preproc - more changes than anticipated ([`3365d9f`](https://github.com/Byron/google-apis-rs/commit/3365d9fba157f57a8351f346e750ee27656df322))
    - Add Rust preprocessor to handle code examples gracefully ([`f33fae4`](https://github.com/Byron/google-apis-rs/commit/f33fae47e9c846be1ee6491fa72cf3e6a185bea0))
    - Add simple preprocessor to handle descriptions interpreted by `cargo doc` ([`8cb73d6`](https://github.com/Byron/google-apis-rs/commit/8cb73d66c245f0f15bbe5c697f92c0351c41fd38))
    - Publishing state thus far - now running into publish limit ([`85f080c`](https://github.com/Byron/google-apis-rs/commit/85f080ce5e9f9fc4fb1e8b317df35eec92ea7ea3))
    - Regen APIs after removal of breaking special case for Rust types ([`7335a19`](https://github.com/Byron/google-apis-rs/commit/7335a1939f731c826f82b88c730b4e1bbe47fdca))
    - Omit special case for type inference; fixes #180 ([`24b5faa`](https://github.com/Byron/google-apis-rs/commit/24b5faa4179e7c47a0e07256c86ad3e4600c10da))
    - Bump version to 1.0.9; update JSON schemas; add new APIs ([`e42ebc0`](https://github.com/Byron/google-apis-rs/commit/e42ebc0c2be9eec858140e082c16008c0cbbc229))
    - Assure API descriptions can contain '"' characters ([`99e97ce`](https://github.com/Byron/google-apis-rs/commit/99e97ceece7da2f4960af6c4ab6bf57c771256ba))
    - Support types with '-' ([`25ba809`](https://github.com/Byron/google-apis-rs/commit/25ba809a093b72f0f3d5a68e824507ff32ea5e62))
    - Unconditionally re-export everything in cmn.rs ([`7a44b3e`](https://github.com/Byron/google-apis-rs/commit/7a44b3eae14e386ba634ace358c713324a6a0395))
    - Assure makefile dependency generation doesn't fail if a JSON file doesn't exist locally ([`a513598`](https://github.com/Byron/google-apis-rs/commit/a5135985d2fcb2be69740f5d6260c0c5796a3606))
    - add lang attribute to docs index ([`a2f5625`](https://github.com/Byron/google-apis-rs/commit/a2f5625fe36c5c257d2dc2a4483e5685f8848518))
    - teach remove_json_null_values arrays ([`499416c`](https://github.com/Byron/google-apis-rs/commit/499416c01101c162da133613d5c855912b17eb3e))
    - Pin yup-oauth2 version to 1.x ([`6d0f4ca`](https://github.com/Byron/google-apis-rs/commit/6d0f4cac7c78de43aba62af71d39f3a5bcb79447))
    - Regen all APIs from new JSON ([`2ad9f37`](https://github.com/Byron/google-apis-rs/commit/2ad9f3781d9fa4f8401d1657b33130e5a079a3d9))
    - Update JSON files ([`86a884c`](https://github.com/Byron/google-apis-rs/commit/86a884c48b178228abe841683d6c65869eb91635))
    - Sort JSON files to minimise git diffs ([`7467f81`](https://github.com/Byron/google-apis-rs/commit/7467f815948666241c5ea304d1af7b016f1bb1ac))
    - Add logging and switch to curl ([`93dba23`](https://github.com/Byron/google-apis-rs/commit/93dba234ddd990a0c93dfd966ab51072a4a650c6))
    - Always run Python in the Venv context ([`57a7ee3`](https://github.com/Byron/google-apis-rs/commit/57a7ee37e583ca4b6db3c361d85591a84f92c861))
    - fix build errors ([`c4055af`](https://github.com/Byron/google-apis-rs/commit/c4055af677f0d7041c90a2bd34f4ff45607a43ba))
    - fix typo ([`43b9534`](https://github.com/Byron/google-apis-rs/commit/43b953445e4352d20c671950c8538b04059661ac))
    - Add codecov integration ([`3eca215`](https://github.com/Byron/google-apis-rs/commit/3eca21544783cc63d5e62f8e54b527c3c587a5be))
    - Improve .gitignore ([`9174c01`](https://github.com/Byron/google-apis-rs/commit/9174c01f2953db7d4691bfbab613a318f160bf0c))
    - Wire in Python code coverage testing ([`73db014`](https://github.com/Byron/google-apis-rs/commit/73db014fe181198652b8f4f3af3141fe16100588))
    - Add first type inferance tests ([`4faa5f6`](https://github.com/Byron/google-apis-rs/commit/4faa5f6203a30ac20c6e266c73542e886e04ae87))
    - Make utils.method_params private as it is not used. ([`a793af5`](https://github.com/Byron/google-apis-rs/commit/a793af55869ade6c4ee0b202235f583a82952f8a))
    - Merge pull request #207 from TheBiggerGuy/python_testing ([`ca11d49`](https://github.com/Byron/google-apis-rs/commit/ca11d495fb859318f321e3dcd8bde3c43f4ce2c3))
    - Wire Python testing into Travis ([`081ccee`](https://github.com/Byron/google-apis-rs/commit/081cceecb7ccef1d5d8db4063afe3f3815f78b25))
    - Improve Python testing ([`9867b26`](https://github.com/Byron/google-apis-rs/commit/9867b260e1b4efac96eaae76337f9da61f4a8b9c))
    - Move to Python requirements.txt file ([`19e9943`](https://github.com/Byron/google-apis-rs/commit/19e99432794c6648e436b4447c07bf6aaef3456a))
    - Merge pull request #209 from TheBiggerGuy/fix-travis-yaml ([`43ac8d7`](https://github.com/Byron/google-apis-rs/commit/43ac8d7e292f34e1a5a245dfb88a2df77859ccb5))
    - Fix Travis Yaml file ([`8325ac1`](https://github.com/Byron/google-apis-rs/commit/8325ac1313300d85576107bed83a133684908b89))
    - Merge branch 'update_url_to_1_7_1' of https://github.com/edelangh/google-apis-rs into edelangh-update_url_to_1_7_1 ([`0360e61`](https://github.com/Byron/google-apis-rs/commit/0360e61c27661beb4b2935cf5ef1e63dd2849f15))
    - Update dep url to 1.7.1 ([`0403e19`](https://github.com/Byron/google-apis-rs/commit/0403e194767a6bac55e8ce0842c40a19b9dd4cd9))
    - Remove an API without corresponding json file ([`1487e6c`](https://github.com/Byron/google-apis-rs/commit/1487e6caa2c1b2b2191e6c7f2706244f4a0b1fb7))
    - Publish of v1.0.8 ([`cdfd978`](https://github.com/Byron/google-apis-rs/commit/cdfd978fc3f0823253b253277571c743aee7295a))
    - Bump version to 1.0.8 - the previous one was already published somehow ([`bce4f08`](https://github.com/Byron/google-apis-rs/commit/bce4f0855481c68027cea5f85b9cb079b70e5a76))
    - After publishing the latest versions ([`dd893eb`](https://github.com/Byron/google-apis-rs/commit/dd893eb29a5ecd4b71b8d068f413aa77e4912ab8))
    - Update to latest versions of API declarations ([`255c7f5`](https://github.com/Byron/google-apis-rs/commit/255c7f5ad58ef67fccfa69afc2f6f37550ab0fc7))
    - Merge branch 'photoslibrary' of https://github.com/TheBiggerGuy/google-apis-rs into TheBiggerGuy-photoslibrary ([`5a67475`](https://github.com/Byron/google-apis-rs/commit/5a67475b54595e05bde5a13d4223cce86a58bc8c))
    - blacklist versions that do not exist ([`d202f97`](https://github.com/Byron/google-apis-rs/commit/d202f9792ba0aea107213ad33ce5e7da06145ef1))
    - clean up after failed wget ([`38f086e`](https://github.com/Byron/google-apis-rs/commit/38f086ebb59b16d9e6be6747a38def03ae114aa0))
    - Add dependencies to README ([`7a041ec`](https://github.com/Byron/google-apis-rs/commit/7a041ecf2d62d35383e82432e6322e05c74d07d9))
    - add photoslibrary v1 code ([`f2363df`](https://github.com/Byron/google-apis-rs/commit/f2363df5b5014e265647f33124ba4d0af6c7144a))
    - Add support for adding unlisted APIs ([`4603769`](https://github.com/Byron/google-apis-rs/commit/4603769ca6fb33877dd15eedf5ea3dd1fbcdb152))
    - Merge pull request #205 from TheBiggerGuy/fix_travis ([`4ba9dbd`](https://github.com/Byron/google-apis-rs/commit/4ba9dbd03cbc8e04c9179a27c5799ea7764443d2))
    - Fix version of Python used on Travis ([`2740810`](https://github.com/Byron/google-apis-rs/commit/2740810b2aa40517f7492f7c67dca7a08b017600))
    - Update virtualenv ([`b7d2e02`](https://github.com/Byron/google-apis-rs/commit/b7d2e021f104a84914137e02ee1f0abf3ad082bd))
    - fix body and footer headers ([`8f0435a`](https://github.com/Byron/google-apis-rs/commit/8f0435ae4f8b1cfca65ec6374c60ced8b5080a18))
    - Pin hyper and mime crates to fix compilation. ([`7ef8049`](https://github.com/Byron/google-apis-rs/commit/7ef8049d2907b54f71566d5e8fb69538deed3a0c))
    - publish newly un-blacklisted crates ([`83843dd`](https://github.com/Byron/google-apis-rs/commit/83843dd796dd9b72dfa43d1fd78c0e678648ac58))
    - Merge pull request #185 from mdaffin/enable-working-apis ([`58c0133`](https://github.com/Byron/google-apis-rs/commit/58c0133879d9b6a057a736f7376ab9afdf587d21))
    - adds all missing apis ([`a0d6551`](https://github.com/Byron/google-apis-rs/commit/a0d6551deab6017ef1904eaf48ced0627eb29ae2))
    - remove working apis from blacklist ([`9e92a3c`](https://github.com/Byron/google-apis-rs/commit/9e92a3c1880d35e9da529e4cc44400d9d052a91f))
    - Merge pull request #184 from mdaffin/enable-compute1 ([`c3cdc16`](https://github.com/Byron/google-apis-rs/commit/c3cdc16fba90bf8e63b28d675f5e4abb714e51d3))
    - added compute1 ([`9330ca1`](https://github.com/Byron/google-apis-rs/commit/9330ca140a82561da406d4f9ba1f4a4421d7e92c))
    - remove compute1 from blacklist ([`eff8aac`](https://github.com/Byron/google-apis-rs/commit/eff8aac1edd20d83e7272e604440e978a3d1282b))
    - publish 'sheets' crates ([`8362289`](https://github.com/Byron/google-apis-rs/commit/8362289bd9c9aecbd0ef80fe1495d0a731dd5f0b))
    - added sheets ([`d6d8d60`](https://github.com/Byron/google-apis-rs/commit/d6d8d6037e853a2edd8358438de188ec91cdbdb0))
    - removed sheets from blacklist ([`deaf8f5`](https://github.com/Byron/google-apis-rs/commit/deaf8f5049c5abadd855a7d9a9f4dd86bbc8e5b6))
    - iteration over dicts with 'values' key ([`d042fcf`](https://github.com/Byron/google-apis-rs/commit/d042fcf1a7e50666e1a5090680d4d1ff6081d695))
    - Remove obsolete paragraphs from README ([`50da215`](https://github.com/Byron/google-apis-rs/commit/50da2152d082356c2fcb9ddc063ba02c49c487d6))
    - Mild reorganization of the README file ([`d238041`](https://github.com/Byron/google-apis-rs/commit/d238041cf740224c586ecaf0518b123ff8b4fd78))
    - Code for latest release ([`f6700f6`](https://github.com/Byron/google-apis-rs/commit/f6700f694302eb5b17a49e4a792d1ab672265fc3))
    - Publish latest version of everything ([`cfafe4c`](https://github.com/Byron/google-apis-rs/commit/cfafe4c6d1e23f900c2a984dc642197be1f5f490))
    - Update to release v1.0.7 ([`5a457b1`](https://github.com/Byron/google-apis-rs/commit/5a457b1b26d786f35ec5ff397f1d0687d4d2b27e))
    - Bump patch level just to allow using new releases ([`dfe263b`](https://github.com/Byron/google-apis-rs/commit/dfe263bb568c272a876be2b919544e8febe081b6))
    - Fix makefile ([`ff31a4a`](https://github.com/Byron/google-apis-rs/commit/ff31a4a749c9e67e3bffb1be80ad742aef12bea5))
    - Releases so far... ([`ad3f308`](https://github.com/Byron/google-apis-rs/commit/ad3f30855af72b91a98419d8340702347056e747))
    - Upgrade to latest API versions + code regen ([`357a0e6`](https://github.com/Byron/google-apis-rs/commit/357a0e650ee692bcb2c2a918e7d1e142cea5d500))
    - Restrict mkdocs to the last known working version ([`dc30217`](https://github.com/Byron/google-apis-rs/commit/dc30217711d007596ee8a069a4e1b79309890876))
    - Publish latest versions of all APIs ([`059d670`](https://github.com/Byron/google-apis-rs/commit/059d6700dd1a61a156adb765a7ae0fe95241e72a))
    - Finish publishing latest version ([`3a6ef3d`](https://github.com/Byron/google-apis-rs/commit/3a6ef3db41c751fb37a49116024443e84758182c))
    - Intermediate result of latest publish ([`7bb8bee`](https://github.com/Byron/google-apis-rs/commit/7bb8beed06ddce0ad0c0daf4ebe736d7bd43553a))
    - Update generated code with latest version ([`164d3f3`](https://github.com/Byron/google-apis-rs/commit/164d3f3cf50d9d2e61c369e07c200e39daaed25a))
    - Switch to serde to v1.0 ([`c284133`](https://github.com/Byron/google-apis-rs/commit/c28413321ba30c5342760d6c23bc5b29ca5ca052))
    - regenerate apis without leading slashes ([`92c80e2`](https://github.com/Byron/google-apis-rs/commit/92c80e238ec3c3ab3b2e65944d755ed7cdc6949d))
    - strip leading slashes from urls ([`f835835`](https://github.com/Byron/google-apis-rs/commit/f835835100f7fc744dd323dac9cbacb4b37ef726))
    - First step towards allowing scopes to be deactivated entirely ([`a630af5`](https://github.com/Byron/google-apis-rs/commit/a630af58309abebf206d473fb44305ee368c53e0))
    - v1.0.5 ([`282eb1c`](https://github.com/Byron/google-apis-rs/commit/282eb1c41728d4a0628ef00aa257d91b3a1cdfd2))
    - regen all source ([`77c26b7`](https://github.com/Byron/google-apis-rs/commit/77c26b7e6c08a16814b547335995565aca25509f))
    - upgrade patch level ([`f947a45`](https://github.com/Byron/google-apis-rs/commit/f947a4552fad3d1c27b13cc3636922f6d170a9b2))
    - kgsearch doesn't work out of the box ([`d8e94b5`](https://github.com/Byron/google-apis-rs/commit/d8e94b5a5dc53624511c77c7e8ee08f4e9bb6c07))
    - all api+cli ([`1dd5ead`](https://github.com/Byron/google-apis-rs/commit/1dd5eadac1cd9a8aea817067f6ef3c968feab794))
    - update toc ([`aaac92b`](https://github.com/Byron/google-apis-rs/commit/aaac92bad68634923c554f4f2e8bc28769a47e84))
    - Update examples ([`fc4ff6f`](https://github.com/Byron/google-apis-rs/commit/fc4ff6fe8b31b2a6c49ab1de4f4e74e0675d3a81))
    - Example now uses hyper_rustls ([`4660d23`](https://github.com/Byron/google-apis-rs/commit/4660d2367606641578bbbbfbe0e0e77bd29a9b72))
    - for now allow nightly to fail ([`ad2748a`](https://github.com/Byron/google-apis-rs/commit/ad2748a691e0fc173e426f26c8f8142141c9c663))
    - chore(regen-apis ([`390ce7f`](https://github.com/Byron/google-apis-rs/commit/390ce7f9d803276ac2934f726eef13c07c385170))
    - more idiomatic swapping of values ([`dd4bfe3`](https://github.com/Byron/google-apis-rs/commit/dd4bfe3de0ffbdce07a763bfc7d8fa237a322e68))
    - Merge branch 'LegNeato-master' ([`73043ef`](https://github.com/Byron/google-apis-rs/commit/73043ef6f60baf47e9f3caf0906ca3f21c930657))
    - use latest gen/ from origin/master ([`c6d67da`](https://github.com/Byron/google-apis-rs/commit/c6d67daa813db9d7d85537df0554b0ff03d47ab1))
    - update all apis ([`9cff808`](https://github.com/Byron/google-apis-rs/commit/9cff80836244ec600dafac1994943ed562107af0))
    - Fix gitignore ([`18f0cae`](https://github.com/Byron/google-apis-rs/commit/18f0cae1d0b1ed2a502883162b6ed6994914d6d0))
    - Allow overriding rootUrl and baseUrl ([`9ffa241`](https://github.com/Byron/google-apis-rs/commit/9ffa241f37fb1a8524cd15525944e253ad07db1f))
    - Merge pull request #167 from Michael-F-Bryan/master ([`3d64db2`](https://github.com/Byron/google-apis-rs/commit/3d64db2e9c91f4a5804928eb33e2eb004b3e00e7))
    - Added an assert to detect when docs need updating ([`ef070ee`](https://github.com/Byron/google-apis-rs/commit/ef070eef59b2eb3e54b77b5fe600e6f6c900c8dd))
    - Finished adjustments to index.html template (fixes #166) ([`de6528b`](https://github.com/Byron/google-apis-rs/commit/de6528be98503c07d04fbf42f740b6da91902672))
    - Fixed up indenting and line length ([`5bc4141`](https://github.com/Byron/google-apis-rs/commit/5bc4141fa584a247d507660bcbe46551789ad04a))
    - Added bootstrap to make things pretty ([`0d65541`](https://github.com/Byron/google-apis-rs/commit/0d655411ab9c81808f683c0ad26a1eb927cdde46))
    - Cleaned up a lot of the link logic ([`05b442d`](https://github.com/Byron/google-apis-rs/commit/05b442d5893ef46ddaf52306aef5807776bc1d05))
    - Converted data to a table ([`8934365`](https://github.com/Byron/google-apis-rs/commit/89343654018fb1a2fb3f6955f5f0e1c3eb4fe0bd))
    - refactor(index.html.mako) Renamed a couple variables ([`09ce891`](https://github.com/Byron/google-apis-rs/commit/09ce891eeb2037dcb090355bf792bf7c3d7edaa5))
    - remove obsolete notes about linux ([`c4e363d`](https://github.com/Byron/google-apis-rs/commit/c4e363d94ce1715f3ecfe6fd6ee56b93c670bfb2))
    - remaining publishes ([`83b916e`](https://github.com/Byron/google-apis-rs/commit/83b916e8f94112effb3c213eaae38d78cb1aa291))
    - more crates published ([`d0eb8f3`](https://github.com/Byron/google-apis-rs/commit/d0eb8f3d9a8aa5cfdf9527a7989cc457f9d69612))
    - intermediate publish ([`87c15a7`](https://github.com/Byron/google-apis-rs/commit/87c15a7406bb84d71255942a7d47cdf34fcecfc6))
    - regen all apis/clis for v1.0.4 ([`d18714e`](https://github.com/Byron/google-apis-rs/commit/d18714e9b2a76cebbe8c6004cddb95e77b9e04be))
    - v1.0.4 - serde upgrade ([`6cad082`](https://github.com/Byron/google-apis-rs/commit/6cad082b7632393cf1415c48023a4733534482cb))
    - remove local override ([`364fc9e`](https://github.com/Byron/google-apis-rs/commit/364fc9eb185c8c2887cf9dd87bd1ce7a11efb575))
    - use new serde map implementation ([`1323d0d`](https://github.com/Byron/google-apis-rs/commit/1323d0dccbb2ed7570e59b9b125f7b4a97ef7575))
    - Adapt to changed Map interface ([`7a611c3`](https://github.com/Byron/google-apis-rs/commit/7a611c39aa215c0025c0bddea564ae4152bd1697))
    - one step closer to getting it compiled ([`26f5794`](https://github.com/Byron/google-apis-rs/commit/26f57948a6631d42570f96498c2418ab457fcf4b))
    - fix error handling ([`1756d7d`](https://github.com/Byron/google-apis-rs/commit/1756d7dec47a5080cd90169b36ee226ea5f4a0e4))
    - upgrade to v0.9 ([`0337435`](https://github.com/Byron/google-apis-rs/commit/0337435cd44105749cb219cc75d61da6895d5d8a))
    - latest release ([`0e6ecb9`](https://github.com/Byron/google-apis-rs/commit/0e6ecb997c60ef6927bbb514f783a2f30eef3952))
    - regen all cli for new version ([`8876209`](https://github.com/Byron/google-apis-rs/commit/8876209143c06bb164bdf4d297667ab0467de161))
    - patch bump to v1.0.3 ([`d3c1faf`](https://github.com/Byron/google-apis-rs/commit/d3c1faf1f4900373f50004a9388200e96c1a06c9))
    - use hyper-rustls instead of openssl ([`d1ebc0f`](https://github.com/Byron/google-apis-rs/commit/d1ebc0ff0be566c749651321394ffb955633286c))
    - Update hyper for generated crates to ^0.10 ([`fd68159`](https://github.com/Byron/google-apis-rs/commit/fd6815997d9429af2a744b87eaea8e316e4e7708))
    - Merge pull request #159 from gdvalle/typo-fix ([`429a333`](https://github.com/Byron/google-apis-rs/commit/429a333a20f987734d909efa557b8b5e2ca248fc))
    - Fix typo Requst->Request ([`8f47126`](https://github.com/Byron/google-apis-rs/commit/8f47126103eb72d753eb5c24a24afb125c912079))
    - published all v1.0.2 ([`7a4d59d`](https://github.com/Byron/google-apis-rs/commit/7a4d59d19768377a7e14cc90b66674293bd1a0a8))
    - v1.0.2 ([`ab4410d`](https://github.com/Byron/google-apis-rs/commit/ab4410d9130ef50df7fe100c2b7d04420b189fd3))
    - v1.0.2 ([`ac325e4`](https://github.com/Byron/google-apis-rs/commit/ac325e4de06bbab638b90a1698c028f25569a4ec))
    - build better data ([`99789de`](https://github.com/Byron/google-apis-rs/commit/99789de208609f0b8ca39852492fe0bbc54689ba))
    - cli ([`629b1c2`](https://github.com/Byron/google-apis-rs/commit/629b1c21438b31976f20e2761eb6f9b30abaa036))
    - make cli publishing work ([`082e51e`](https://github.com/Byron/google-apis-rs/commit/082e51e16e9deb01e9c82ea7c8ac9be5bee80c79))
    - all api ([`64be016`](https://github.com/Byron/google-apis-rs/commit/64be0168811a42bc9820b44aed8f88b7a60524ad))
    - update code to latest version ([`7fe6e69`](https://github.com/Byron/google-apis-rs/commit/7fe6e698ba0de70aa96cffa803cdb7f705fc9843))
    - update ([`73038b2`](https://github.com/Byron/google-apis-rs/commit/73038b2c668dbbbe1a1b8e82dd1246d031ec0b35))
    - api-cli lock-step; depend on specific version ([`75316f4`](https://github.com/Byron/google-apis-rs/commit/75316f4c8400bd778050c44b1574babb074fa82c))
    - v1.0.1 ([`52a1dd2`](https://github.com/Byron/google-apis-rs/commit/52a1dd23c12cadc809297e2561bf9212954ea0c1))
    - try to depend on major version of api ([`b68b2a6`](https://github.com/Byron/google-apis-rs/commit/b68b2a6bf5786327afad1d95bceffa1111400e0a))
    - cli + api use a single base version ([`5e28a06`](https://github.com/Byron/google-apis-rs/commit/5e28a06dc0dfafd414765738fff35d019a903cab))
    - don't use relative links ([`cc30a2e`](https://github.com/Byron/google-apis-rs/commit/cc30a2e20b697ca318bd3b54e5b94f6935eaadd2))
    - improve UX ([`6279fd8`](https://github.com/Byron/google-apis-rs/commit/6279fd8f5df3ca9c9013635c36041e89df902428))
    - better install script + blacklist ([`09805e5`](https://github.com/Byron/google-apis-rs/commit/09805e59adba84d83184ef02f27abbb054359c45))
    - link to doc.rs for APIs ([`fad9d3b`](https://github.com/Byron/google-apis-rs/commit/fad9d3b0ca3f588f65faf6ec46caf51a7ca1c239))
    - button to copy install-script ([`fdc0141`](https://github.com/Byron/google-apis-rs/commit/fdc0141fbcabf68ed5d715314d483469e7a7ef14))
    - remove all download links ([`d6accb8`](https://github.com/Byron/google-apis-rs/commit/d6accb8f6194bac7f982ee93409821436dd8beed))
    - remove download information ([`e646898`](https://github.com/Byron/google-apis-rs/commit/e646898137de3897cdabdb65d2ad553e45c772c1))
    - use docs.rs for library documentation ([`fc34337`](https://github.com/Byron/google-apis-rs/commit/fc34337ee4ba708f63e3d2f164660edd5ffe5614))
    - DS_Store ([`684233c`](https://github.com/Byron/google-apis-rs/commit/684233ccee8ef89e17332f2992b80c5ffbc1bf4a))
    - get fixes into README ([`a0a264f`](https://github.com/Byron/google-apis-rs/commit/a0a264f4c36da6e26515b1399bf75ca199c14316))
    - correct link to license on github ([`f3d0ef4`](https://github.com/Byron/google-apis-rs/commit/f3d0ef45d26baaafa3b9120bebe371bce424309c))
    - re-publish as much as possible ([`ad91946`](https://github.com/Byron/google-apis-rs/commit/ad919460cdcadfa4c812bee54563138c21da554f))
    - fetch latest json and re-gen all code ([`06caa1d`](https://github.com/Byron/google-apis-rs/commit/06caa1de02a9a6b8b8d843bb45bad02efd7a9790))
    - remove .DS_Store files ([`e04b6d0`](https://github.com/Byron/google-apis-rs/commit/e04b6d023dcc906b9135037355ae05e9ef043e1b))
    - Merge pull request #158 from compressed/dollar_sign ([`eecab2f`](https://github.com/Byron/google-apis-rs/commit/eecab2ff05f698e16ffb9fe0f74204f9642acdba))
    - handle discovery urls with $ ([`0ba9535`](https://github.com/Byron/google-apis-rs/commit/0ba9535a1150750b80e862c8fc197819f0f25954))
    - Merge pull request #157 from Morsicus/fix_readme_multirust_deprecated ([`d898f14`](https://github.com/Byron/google-apis-rs/commit/d898f1497420f47f4b85be2f3684d23b1266df82))
    - multirust is deprecated - use rustup :) ([`f31ef51`](https://github.com/Byron/google-apis-rs/commit/f31ef51a61cf8e1343e4ab956d8be29271976e59))
    - update ([`d997051`](https://github.com/Byron/google-apis-rs/commit/d9970513cbf27f2a2128cac843abbff350ecc9c8))
    - all CLIs and APIs are available in v1.0 now! ([`7cf4034`](https://github.com/Byron/google-apis-rs/commit/7cf4034a415b14195d54e79cd2781e7f175cdbad))
    - all clis except for one ([`dd63b0f`](https://github.com/Byron/google-apis-rs/commit/dd63b0fa672b106890a2086f0d997cd965517fc9))
    - ignore cloudtrace ([`c3bd076`](https://github.com/Byron/google-apis-rs/commit/c3bd076c0faf39e2b07662ce2407c4cb61931a35))
    - keep state ([`0d3adb6`](https://github.com/Byron/google-apis-rs/commit/0d3adb658b47bb9e1bdd6b370e738aad06e238d4))
    - chore(cli-version-up) ([`ceceb8c`](https://github.com/Byron/google-apis-rs/commit/ceceb8c6e5bc136e2c1b5a4da121f73e1894973c))
    - add api publish notes ([`4258fd5`](https://github.com/Byron/google-apis-rs/commit/4258fd559035952b9a7c9e1f3292833fdb03557c))
    - chore(api-version-up) ([`3c5a07c`](https://github.com/Byron/google-apis-rs/commit/3c5a07c7bedd63f7dbf1ab39ab8e9bf98dfa1b6b))
    - keep things stable in v1.0 ([`96e07a3`](https://github.com/Byron/google-apis-rs/commit/96e07a35da8e06253228f16f473f571d80acd112))
    - allow failure on nightly ([`43d0289`](https://github.com/Byron/google-apis-rs/commit/43d028950e8f3b1d1b7f001161de66feab314bc8))
    - use serde_derive ([`73f0e83`](https://github.com/Byron/google-apis-rs/commit/73f0e83086ba168f354a3395911409b4f2b91856))
    - use yup-oauth2 1.0 ([`bc582e5`](https://github.com/Byron/google-apis-rs/commit/bc582e57612821e71ec056d4f213db3a4ff2bb83))
    - use latest of yup-oauth2 ([`8f59b9b`](https://github.com/Byron/google-apis-rs/commit/8f59b9ba79668223214a7f0ec6362b17e09c760e))
    - Merge pull request #155 from dermesser/parse-empty-correctly ([`b21d961`](https://github.com/Byron/google-apis-rs/commit/b21d96177d6d3498860078feec8c921c2b567044))
    - Merge pull request #154 from dermesser/no-urlencoded-slash ([`0acd71f`](https://github.com/Byron/google-apis-rs/commit/0acd71f1b61019f579c23f7cf2ffe4ac73902afc))
    - Add an unused field to empty API types. ([`0f14aa9`](https://github.com/Byron/google-apis-rs/commit/0f14aa966e5e878612111709568b13e7a2c70345))
    - Regenerate APIs ([`36db66b`](https://github.com/Byron/google-apis-rs/commit/36db66bf3ce1881b4e352bcc934917d4e4926b4e))
    - URL-encoding '/' in URLs is not accepted by Google APIs. ([`292dd2f`](https://github.com/Byron/google-apis-rs/commit/292dd2f34f04a2376c3d44990111d4a0fc2c400e))
    - publish remaining cli ([`4a92a47`](https://github.com/Byron/google-apis-rs/commit/4a92a47bed1f61d52453192e6ab2c823b860f17b))
    - all cli ([`569e802`](https://github.com/Byron/google-apis-rs/commit/569e8029e1dd9c7c2d45325ad6f034955534d8e6))
    - prepare cli for publish ([`93d053b`](https://github.com/Byron/google-apis-rs/commit/93d053b2d6d7a51db7ee2e2c53b66ecf6f45f1dd))
    - can now be published ([`91a657b`](https://github.com/Byron/google-apis-rs/commit/91a657b8cfc4769c69acaaefffa88a5960cd4b9a))
    - latest APIs ([`c4c4901`](https://github.com/Byron/google-apis-rs/commit/c4c49015f507ee0d80afef3cdd02dd44751c1dcd))
    - specify version to allow cli publishing ([`d249540`](https://github.com/Byron/google-apis-rs/commit/d2495405c5cfcf2d761bfc95b697789bbcc7774b))
    - publish all APIs ([`8a20d77`](https://github.com/Byron/google-apis-rs/commit/8a20d778a93258a08f2f062bec9e281ee402876a))
    - update to latest version ([`45d86f3`](https://github.com/Byron/google-apis-rs/commit/45d86f31f2f8aaf3a67f2c39c928435407f27ddc))
    - to latest ([`13ed4ea`](https://github.com/Byron/google-apis-rs/commit/13ed4eaecb827099d8000dc9733e39709ef02342))
    - version-update ([`33771a6`](https://github.com/Byron/google-apis-rs/commit/33771a6dc7137be3aa47cd4329859814afaa7cd0))
    - one more down ([`df2dc47`](https://github.com/Byron/google-apis-rs/commit/df2dc4784aaa988c7c1e2bc2598b538fa33e8aec))
    - v0.3.6 ([`527ffa3`](https://github.com/Byron/google-apis-rs/commit/527ffa35c23186a1253365448469030aae313b0c))
    - use redirect flow ([`b6f5fc6`](https://github.com/Byron/google-apis-rs/commit/b6f5fc6eb3e6be21d22fb667b541f13ee3881df1))
    - rustc version no longer needed ([`2a2e7bf`](https://github.com/Byron/google-apis-rs/commit/2a2e7bfc9a6ae870acc2901527fb69c41895fb08))
    - update to latest version ([`38c3d9a`](https://github.com/Byron/google-apis-rs/commit/38c3d9a34dcdb947324b02876a5d2f3ef4e614e2))
    - update with latest troublemakers ([`2b37fc4`](https://github.com/Byron/google-apis-rs/commit/2b37fc4d353f8a6f139eef70cc5254140675facc))
    - latest version to crates.io ([`b9f237e`](https://github.com/Byron/google-apis-rs/commit/b9f237eec04ca2547cc8c53deb149e88aea1d2cb))
    - remove workaround marker ([`e7721ce`](https://github.com/Byron/google-apis-rs/commit/e7721ce53bafc70ea4de3d14920739649c06c492))
    - relative path for custom target dir ([`dc367c3`](https://github.com/Byron/google-apis-rs/commit/dc367c34751e04036e56a4d984d6b7f8f92cef4d))
    - update code to latest version ([`ae27643`](https://github.com/Byron/google-apis-rs/commit/ae276438ae7dbc4cb141c8f216834435976e80f0))
    - increment versions ... ([`091d3f7`](https://github.com/Byron/google-apis-rs/commit/091d3f7e81de1f4ca2a6e07da836667c972b052d))
    - Revert "fix(cargo): remove cargo/config" ([`b2b9ab2`](https://github.com/Byron/google-apis-rs/commit/b2b9ab2831a3b7ae1c274987558d33af851bc624))
    - don't fail by default on non-nightly ([`e9fe17e`](https://github.com/Byron/google-apis-rs/commit/e9fe17ee3b5df32de65ed4017c65748eb8388a29))
    - use working version of serde-codegen ([`3921b6a`](https://github.com/Byron/google-apis-rs/commit/3921b6a5a071ec0dc9d803b0ae809a348c34f87f))
    - badges for issue stats ([`850e115`](https://github.com/Byron/google-apis-rs/commit/850e115e33e5da9fe6266718e4cf04c23e554d2b))
    - license year ([`5d1039e`](https://github.com/Byron/google-apis-rs/commit/5d1039e85fc1e8edada512ec16969e5fec901bb1))
    - `make regen-apis` ([`c6f9205`](https://github.com/Byron/google-apis-rs/commit/c6f92057582c576129db6f8ca27ee824df201c5d))
    - split doc and test to handle features ([`864fe84`](https://github.com/Byron/google-apis-rs/commit/864fe8424d83889ebe3aff69f0ada27c43a5bbc3))
    - as learned from yup-oauth ([`5ca0217`](https://github.com/Byron/google-apis-rs/commit/5ca021727511b8265da1abadc48eb241ca24e3c5))
    - work with latest serde ([`bed46ba`](https://github.com/Byron/google-apis-rs/commit/bed46ba2414167fcf6563ac1766f3239765f4800))
    - add last known working rustc version ([`e5dc49f`](https://github.com/Byron/google-apis-rs/commit/e5dc49f87411377a6b655efdffde14159cea5fc8))
    - Use flow for installed apps ([`d37bb19`](https://github.com/Byron/google-apis-rs/commit/d37bb19df2bb4b274ee69c8ed3e85056c216e8e0))
    - remove cargo/config ([`cb6679c`](https://github.com/Byron/google-apis-rs/commit/cb6679cb2b45022162a7e6a1b5de39b1fbbcf870))
    - inform about nightly builds ([`87dcf06`](https://github.com/Byron/google-apis-rs/commit/87dcf06eacc4fef9ed5bdec99fbb589c3d81666f))
    - use features for cli-dependency ([`3e2216c`](https://github.com/Byron/google-apis-rs/commit/3e2216c4454d34a6ef080c5bb7d64b0b451bdecd))
    - let's be sure to get the matrix right ([`95e9187`](https://github.com/Byron/google-apis-rs/commit/95e9187c842c8c301bfff8a3c228e9da189f580a))
    - Travis CI support ([`320d769`](https://github.com/Byron/google-apis-rs/commit/320d769c6f97460547395990efc07a441be9a927))
    - update to latest version + nightly support ([`8d7a498`](https://github.com/Byron/google-apis-rs/commit/8d7a49891f8e6db1c528ce4b212029612c077472))
    - pin `url` crate ([`ca5dca7`](https://github.com/Byron/google-apis-rs/commit/ca5dca7af93f7feef1b81237a9c7c1d5b07e1577))
    - publish state ([`8aefeb3`](https://github.com/Byron/google-apis-rs/commit/8aefeb37d98e49be7f2ad88597f5e93087d99aa6))
    - to latest schema version ([`e0de1b4`](https://github.com/Byron/google-apis-rs/commit/e0de1b4c108ebfd37f6b7ebea4bf1109445d2661))
    - all jsons; version-up ([`930ce6d`](https://github.com/Byron/google-apis-rs/commit/930ce6d5c2bf9776b4fdcb8c8dd00753cf7aa47d))
    - use compatible yup-oauth ([`d2c12c2`](https://github.com/Byron/google-apis-rs/commit/d2c12c296410be706c66bab824543f32be38da2d))
    - use hyper Bearer header style ([`065cfdd`](https://github.com/Byron/google-apis-rs/commit/065cfdd22f974afe9d8071e0227929464c1df796))
    - compatibility with serde 0.6 ([`9e8a047`](https://github.com/Byron/google-apis-rs/commit/9e8a047ebfddd7a94226b8d559b03325abf7ab54))
    - choose serde-version which works ([`33f2813`](https://github.com/Byron/google-apis-rs/commit/33f281360a0a5dfa93cd7e6f4f345689e86bcc3f))
    - first bunch of publishes ([`a25b593`](https://github.com/Byron/google-apis-rs/commit/a25b593969c972586a71101f1e6866b22db6c89e))
    - update after version-up ([`df301c1`](https://github.com/Byron/google-apis-rs/commit/df301c1c75fd60239fc93c50e6a6a4dd9d627f9d))
    - api version 0.1.12 ([`b3fd15e`](https://github.com/Byron/google-apis-rs/commit/b3fd15edec924c844f02aea4adfdcbe57d06681f))
    - update code matching latest jsons ([`b0c0196`](https://github.com/Byron/google-apis-rs/commit/b0c0196f502a93d42053f9e6d54999e648631ac6))
    - fetch latest json ([`5094f61`](https://github.com/Byron/google-apis-rs/commit/5094f61c883cb05034b3d95171bc63af027e97b5))
    - use venv-python to run any utility ([`4bb7a33`](https://github.com/Byron/google-apis-rs/commit/4bb7a33e9370f520b985f96aa8229b659320ff1d))
    - upgrade to 0.7 ([`be0faf0`](https://github.com/Byron/google-apis-rs/commit/be0faf0e1dd02c2efe4e8f903b948ae38596f04a))
    - Merge pull request #147 from programble/fix/repeated-props ([`4878fcb`](https://github.com/Byron/google-apis-rs/commit/4878fcb80304705eaf35ee7215fe5679ddd24bab))
    - Send repeated properties as repeated query parameters ([`3e1c442`](https://github.com/Byron/google-apis-rs/commit/3e1c4428effc689ff11b1e8ca4878d3f21e21d6f))
    - added custom client secret to allow operation ([`ab672c4`](https://github.com/Byron/google-apis-rs/commit/ab672c41f97ee1aa5e1ef69c68d3cfeebf9a9774))
    - update generated code ([`426b096`](https://github.com/Byron/google-apis-rs/commit/426b096ef8ebea1bf34b4b7bdbb2660ef7e7f921))
    - remove unimplemented option ([`52e5815`](https://github.com/Byron/google-apis-rs/commit/52e58154a2c5d151871894ca86e67743aa5d0cc8))
    - need Rust 1.6 now ([`b35a1d6`](https://github.com/Byron/google-apis-rs/commit/b35a1d6732022a41b846d6a8bcee8ecae940d260))
    - update all crates ([`8732b5f`](https://github.com/Byron/google-apis-rs/commit/8732b5f8694a7c1f489347d12b79d72f4c68882e))
    - update remaining APIs ([`1e5a6bb`](https://github.com/Byron/google-apis-rs/commit/1e5a6bbee8b461595269dc60b9cd92a6e2fc606d))
    - update ([`900c2bf`](https://github.com/Byron/google-apis-rs/commit/900c2bfde917383295761e6b743ce465e242333a))
    - updated to latest state ([`75076ac`](https://github.com/Byron/google-apis-rs/commit/75076acf16fc7263bea4b6b4fe063ef38cd6a44c))
    - increment versions of API + CLI ([`5cba22f`](https://github.com/Byron/google-apis-rs/commit/5cba22f0c6717574af77756324eae08431dd3cdb))
    - updated API descriptions ([`8f01e8e`](https://github.com/Byron/google-apis-rs/commit/8f01e8e91837b76092507b9313d914dce4fb1c49))
    - use latest oauth2 lib ([`a2c6b58`](https://github.com/Byron/google-apis-rs/commit/a2c6b58d5b8525110a5386e93c2de4f6851b95c6))
    - use new discoveryRestUrl field for json download ([`ef9e7f1`](https://github.com/Byron/google-apis-rs/commit/ef9e7f1bae2bff1629530fde14ca19ad424fc653))
    - clap-rs v1.5 -> 2.0 ([`ab1aa55`](https://github.com/Byron/google-apis-rs/commit/ab1aa55d395286e96a6508a6afcc5b8d723572f5))
    - use std::Thread::sleep ([`b54acb7`](https://github.com/Byron/google-apis-rs/commit/b54acb7c96c842228a7ec65ff6b6edaf2b19b0bd))
    - rever to multirust-rs ([`495ecef`](https://github.com/Byron/google-apis-rs/commit/495ecef8c8fcda27b08833df9fcfca503fa65002))
    - Merge pull request #144 from cmr/master ([`75dfa7f`](https://github.com/Byron/google-apis-rs/commit/75dfa7f40fb81776aadb4150365493a63fc267bf))
    - fix error from no trailing newline ([`da78e9f`](https://github.com/Byron/google-apis-rs/commit/da78e9fa4d68d772323c7a2927b8004b8ac5d1a8))
    - Relicense to dual MIT/Apache-2.0 ([`ac5886e`](https://github.com/Byron/google-apis-rs/commit/ac5886e47e4565c63e9f03aabb1710b07d547219))
    - update with latest changes ([`06e1d4b`](https://github.com/Byron/google-apis-rs/commit/06e1d4bff6311cda4cd7feb2e94a921bf26cd5e5))
    - no wildcards in dependencies ([`8295bf3`](https://github.com/Byron/google-apis-rs/commit/8295bf3c2dc310aeae293dc7287e2efe8d651abd))
    - support for latest hyper ([`e3f4fca`](https://github.com/Byron/google-apis-rs/commit/e3f4fcadad5310ad79d91a1ce449e1ee9f3d0c74))
    - Merge pull request #134 from programble/bug/hyper-0.7.0 ([`f0f4df4`](https://github.com/Byron/google-apis-rs/commit/f0f4df45c3c099c7e54ec7f39b14a771a367786d))
    - Fix impl of hyper::net::NetworkStream ([`61e74d9`](https://github.com/Byron/google-apis-rs/commit/61e74d99a225d48861c716bc2cb4fe6aa4b4cd54))
    - Depend on hyper 0.7.0 ([`dbc6402`](https://github.com/Byron/google-apis-rs/commit/dbc6402f483e1ff90b6eb8957c2654580d2f56b0))
    - improve handling of error code if stable is tested ([`78c7d46`](https://github.com/Byron/google-apis-rs/commit/78c7d46f9ddb7b102fd59135cac5d1033f090b0a))
    - test on nightly, and run cargo test ([`e1772d9`](https://github.com/Byron/google-apis-rs/commit/e1772d979d98c7d0d5c7fe14f0e81d6b5cc7ee18))
    - of latest API versions ... no functional change ([`6a7654d`](https://github.com/Byron/google-apis-rs/commit/6a7654d433acd39b26ada579664a3833ef28a708))
    - Merge pull request #132 from erickt/master ([`56dfdfc`](https://github.com/Byron/google-apis-rs/commit/56dfdfcf9103707d4689ed48c91facfff8e2126a))
    - get cmn compiling on nightly rust ([`8179f3b`](https://github.com/Byron/google-apis-rs/commit/8179f3bf89991d83f6cb5689618f8ee90b3f9a5b))
    - update everything to latest google API versions ([`5612d00`](https://github.com/Byron/google-apis-rs/commit/5612d004b9fab6be352b4013f882a7a1dee2eb6b))
    - assure license can be generated ([`9a2d2b5`](https://github.com/Byron/google-apis-rs/commit/9a2d2b576c84536a7a93deedcba68544bf4a10eb))
    - update to latest version ([`862842f`](https://github.com/Byron/google-apis-rs/commit/862842f62166f2c84efa24cf1e09ef644e3ceacf))
    - use PYTHONPATH for mako invocation ([`0bd7f20`](https://github.com/Byron/google-apis-rs/commit/0bd7f2004843b4e9dcd8af366e7ffc6632fb9e41))
    - improve version and library name handling ([`53c27da`](https://github.com/Byron/google-apis-rs/commit/53c27da2e786e12a29037addde15d571c3b53b39))
    - doc(presentation) using reveal.js ([`e4f89ce`](https://github.com/Byron/google-apis-rs/commit/e4f89ce0c6c16480cccbe92aeaad55e3811beb83))
    - code compiles with rust 1.3/serde 0.6 ([`a8d333f`](https://github.com/Byron/google-apis-rs/commit/a8d333f91681010f9c5a52c50339d9d36ea49dc3))
    - update to latest serde/rust ([`8dab8c0`](https://github.com/Byron/google-apis-rs/commit/8dab8c01249a9f54e43aebe8a009f60935279de8))
    - stackshare.io badge ([`7754a16`](https://github.com/Byron/google-apis-rs/commit/7754a160c9fff6fb3796982cb2cc284c033d1008))
    - re-publish lib crates at latest version ([`aecda18`](https://github.com/Byron/google-apis-rs/commit/aecda18821cb39edc8ccb6dd3f286b1c4ce626bc))
    - add source at latest version ([`08d65ba`](https://github.com/Byron/google-apis-rs/commit/08d65ba62b025fac9c2db3db3842b86510c03d51))
    - update to serde 0.5.0 ([`8ab4fd0`](https://github.com/Byron/google-apis-rs/commit/8ab4fd0bd4b64eb76c77adc82aff99df17a1070c))
    - latest json files ([`62b63b2`](https://github.com/Byron/google-apis-rs/commit/62b63b251ac13fb945c444bd100b97fc7846a3c8))
    - pretty-print errors in debug mode ([`152cdd8`](https://github.com/Byron/google-apis-rs/commit/152cdd848a41109819d890560d26270bd08c12ae))
    - upgrade to latest hyper+clap ([`e465359`](https://github.com/Byron/google-apis-rs/commit/e46535917ceeb9ba8cce2cbab39ae1e3f82942ac))
    - use clap 1.0.3 ([`be894be`](https://github.com/Byron/google-apis-rs/commit/be894becc38296a62760a0724ea1310081e713de))
    - compatibility with hyper 0.6.4 ([`e129a7d`](https://github.com/Byron/google-apis-rs/commit/e129a7d3ae878a9ee78ea21fe1c0aa8b5671a5e0))
    - Merge branch 'master' of https://github.com/Byron/google-apis-rs ([`2575d5a`](https://github.com/Byron/google-apis-rs/commit/2575d5abe7bbe1cb5ca94c5396468a6e4cafb626))
    - configure to use wait-lock ([`defbeaa`](https://github.com/Byron/google-apis-rs/commit/defbeaa630661400e212f598c98bc60596afa5fb))
    - adjust linux script to target dir ([`0f61fa4`](https://github.com/Byron/google-apis-rs/commit/0f61fa4c95c25c0e9f30cc10b6aa3b005d26e3ca))
    - latest version of v0.3.1 code ([`6393bbf`](https://github.com/Byron/google-apis-rs/commit/6393bbf7f69f63e1291dc5b2cbd71a79bfdac2cc))
    - CLI v0.3.1 ([`337f167`](https://github.com/Byron/google-apis-rs/commit/337f167e6c9cfd464231bbbac9ea4503f9c77e00))
    - update json and regen all code ([`7d58d66`](https://github.com/Byron/google-apis-rs/commit/7d58d66025ea58088950a0914bfb910a35a16748))
    - flush output stream on CLI output ([`615ac64`](https://github.com/Byron/google-apis-rs/commit/615ac64ec1d86c1c00ff05a4d2f6065c866330a7))
    - work with hyper v0.6.0 ([`d0491a4`](https://github.com/Byron/google-apis-rs/commit/d0491a4950f657c55dfbf6a16a16a64c72b9077c))
    - type-inference fails on empty vec ([`27fdd8e`](https://github.com/Byron/google-apis-rs/commit/27fdd8ee0c19dda409b6ca5a804edf23b8555ff3))
    - make statement shell compatible ([`a566b70`](https://github.com/Byron/google-apis-rs/commit/a566b702738c4b470988645f2867966d1d288370))
    - adjust to build.target-dir ([`1180314`](https://github.com/Byron/google-apis-rs/commit/1180314275b62979376ba5ebacf34763ef6ca610))
    - one target dir for all projects ([`31f22b1`](https://github.com/Byron/google-apis-rs/commit/31f22b1535feaa031586bdb6d16e2a306fd62a38))
    - cli code update ([`d0fb7a5`](https://github.com/Byron/google-apis-rs/commit/d0fb7a5ccc5729303b5aca2419ac06abf12e0133))
    - add type annotation ([`62db3ae`](https://github.com/Byron/google-apis-rs/commit/62db3ae87c1ad71082566a2e195a1e5d2cb7219f))
    - published latest versions ([`d1cf836`](https://github.com/Byron/google-apis-rs/commit/d1cf8360f979df5c53fe32a34c0cddc246a072f1))
    - added latest version of api+cli ([`3484fec`](https://github.com/Byron/google-apis-rs/commit/3484fecf9cf19c0a09f6241f8beea2d62d637cf7))
    - Merge branch 'next' ([`e336d37`](https://github.com/Byron/google-apis-rs/commit/e336d37d1366f40c2780d3cc4e964559b1f1d9c4))
    - Merge branch 'syntex' into next ([`ef3fb39`](https://github.com/Byron/google-apis-rs/commit/ef3fb39a4aa3f72867de6039fc9e015566566abb))
    - api+cli increment ([`d6ddff2`](https://github.com/Byron/google-apis-rs/commit/d6ddff240d1fefac28549efad78648d98e4ed9a4))
    - update info about rust stable ([`6e669ce`](https://github.com/Byron/google-apis-rs/commit/6e669ced2aca094b246c2c0eb805b362924112b1))
    - Merge branch 'master' into syntex ([`8aba8a5`](https://github.com/Byron/google-apis-rs/commit/8aba8a5c75f13ec7ead2a8a79a7a09f67c7b49e6))
    - explicitly use stable rust ([`1f9dc06`](https://github.com/Byron/google-apis-rs/commit/1f9dc06a57f45c8be216602661b68a0adce5beca))
    - work on stable ([`a9e0be6`](https://github.com/Byron/google-apis-rs/commit/a9e0be6583fd92b9a171091b70e81bdba4ad4aa2))
    - minor fixes ([`2ad8d88`](https://github.com/Byron/google-apis-rs/commit/2ad8d887cda32214dc520af5a9621366f4522fdf))
    - expanded header implementation ([`5483e32`](https://github.com/Byron/google-apis-rs/commit/5483e328320412c39798ba15f26d02b90dd7591d))
    - first big step towards syntex ([`b0a41c4`](https://github.com/Byron/google-apis-rs/commit/b0a41c4e788fd95f9ace6823c2e52d18f33195c9))
    - information about unstable rustc ([`5c798d5`](https://github.com/Byron/google-apis-rs/commit/5c798d5fb621f042a8851339df04709831f04ca5))
    - Merge branch 'master' into next ([`267868c`](https://github.com/Byron/google-apis-rs/commit/267868cea331b92a31e1ac4a52c2210a1c222992))
    - bump hyper dep to 0.5.0 ([`ed0debe`](https://github.com/Byron/google-apis-rs/commit/ed0debe999c201a82be5134b3dbec7bd5400085e))
    - disallow empty values explicitly ([`129fd38`](https://github.com/Byron/google-apis-rs/commit/129fd38e003d2ab23bad2ceb84f59bb74b4ae45b))
    - Merge branch 'worr-bug/clean-target' ([`b858286`](https://github.com/Byron/google-apis-rs/commit/b858286f6e03d640221b0306ad92bdd73961dff3))
    - clean was depending on unknown targets ([`d9ed001`](https://github.com/Byron/google-apis-rs/commit/d9ed001b46cc6e510ea2267fa205abd036be10b6))
    - fix clean target for docs/cli ([`bcf90cb`](https://github.com/Byron/google-apis-rs/commit/bcf90cbcc8f625b787596fb95eda4355e35b403b))
    - URL substitution handling ([`2cc4807`](https://github.com/Byron/google-apis-rs/commit/2cc48072344715c428c204e98ab991c8133cf4c2))
    - improved structure setter code ([`ca36dbc`](https://github.com/Byron/google-apis-rs/commit/ca36dbc50595f116952a95395d802d5be313b614))
    - OK version of json value setter ([`d0b69af`](https://github.com/Byron/google-apis-rs/commit/d0b69af41390df40f5a11d44e08d1b67167a969a))
    - handle recursive mut json values ([`464394a`](https://github.com/Byron/google-apis-rs/commit/464394af22714fee650ca3e310336584666f921a))
    - compilation without local overrides ([`e434563`](https://github.com/Byron/google-apis-rs/commit/e434563215b6b8cddc0aaa6a6c5ef48d6e7aedbb))
    - bring in all required field data ([`f83dff6`](https://github.com/Byron/google-apis-rs/commit/f83dff672bc5a739f1a4b76333e25d40523fbe2c))
    - basis for simplified value setting ([`a2dd714`](https://github.com/Byron/google-apis-rs/commit/a2dd71451deaf49e2bc4bb8de68a4e4cc87ec8a9))
    - added dev-diary episode 3 ([`a66f1ad`](https://github.com/Byron/google-apis-rs/commit/a66f1ad728980d431a1ceb1ae0163afd6949d983))
    - dc630d01e 2015-05-09 ([`2ca0529`](https://github.com/Byron/google-apis-rs/commit/2ca05292971d50adb267305492a8c703b929e99f))
    - deal with rustc lifetime issue ([`ee84fef`](https://github.com/Byron/google-apis-rs/commit/ee84fefb4a46bf816fd6fcedebaa1428d12969a5))
    - fix tar-handling ([`5e5f0dc`](https://github.com/Byron/google-apis-rs/commit/5e5f0dcc662160a378387a91a0719407dde503c9))
    - typo + fixed yaml references ([`296debd`](https://github.com/Byron/google-apis-rs/commit/296debda85e0afb22261b61ab671325f539e01ff))
    - change wording ([`9ba25af`](https://github.com/Byron/google-apis-rs/commit/9ba25af48b8c7794bb8f28475b821fdf957a0efd))
    - updated for cli v0.2.0 ([`23119a0`](https://github.com/Byron/google-apis-rs/commit/23119a06d263f130d9797f73c12c34eed3ba5102))
    - added Download information ([`383595c`](https://github.com/Byron/google-apis-rs/commit/383595c44e9b2aafbece20eb60a3ff36c1f88d81))
    - limit tar.gz to executable ([`9e64d1b`](https://github.com/Byron/google-apis-rs/commit/9e64d1bd10f0cd68c8519954bda14ce784805a7a))
    - download links to `tar.gz` files ([`e92f440`](https://github.com/Byron/google-apis-rs/commit/e92f440d9b980c80c31c04752a8fe0c21fa36585))
    - detailed deployment instructions ([`6bca4b7`](https://github.com/Byron/google-apis-rs/commit/6bca4b75d9b1abbe882f5c1bb7ab27232046f89d))
    - move all scripts into src/bash ([`ad6dd77`](https://github.com/Byron/google-apis-rs/commit/ad6dd7758e08e72cf3df3cee87df41286e9d5f0a))
    - osx-tar files without directory ([`be11776`](https://github.com/Byron/google-apis-rs/commit/be117767a11962e330568d1dd98035e7b142b910))
    - script to deploy for download ([`1d44d79`](https://github.com/Byron/google-apis-rs/commit/1d44d794eba6d7c371c10205e6f1d8b416748035))
    - publish APIs @v0.1.7 ([`3e70a89`](https://github.com/Byron/google-apis-rs/commit/3e70a896742dd682a55394be6936811eaad27111))
    - latest version of all code ([`9e6c953`](https://github.com/Byron/google-apis-rs/commit/9e6c9537a527528debd0c68a5fe4494291facdbd))
    - DL title contains os-name ([`69b1210`](https://github.com/Byron/google-apis-rs/commit/69b12104a9f9579773553825f63c321e7d1a6899))
    - added download links (osx,ubuntu) ([`52027c6`](https://github.com/Byron/google-apis-rs/commit/52027c6db59c2952f61ee03204fd947277d0cc62))
    - added back-link to crates.io ([`0e6605d`](https://github.com/Byron/google-apis-rs/commit/0e6605d7a4ee59e16d52fd93e037b5608fd5f61f))
    - non-redundant data access ([`5c284e1`](https://github.com/Byron/google-apis-rs/commit/5c284e1c418d93bca7da4a29c4f8feaf5800c1ce))
    - improved display of BadRequest ([`e86e55c`](https://github.com/Byron/google-apis-rs/commit/e86e55cae788506a2280816009b8620bad091477))
    - `after_help` url for method scmds ([`fff466f`](https://github.com/Byron/google-apis-rs/commit/fff466f6bcc1ff2dae882d5b1c29b0ff844e46fb))
    - use `json_tools::IteratorExt` ([`15daf31`](https://github.com/Byron/google-apis-rs/commit/15daf311ea79a95baf5b28760c88fbbff63a450b))
    - remove null in pretty-printed json ([`5894c81`](https://github.com/Byron/google-apis-rs/commit/5894c8163afa9f9d9bed592e7e41912c77cf993d))
    - faster null-value removal ([`26314e7`](https://github.com/Byron/google-apis-rs/commit/26314e743e2c4f38eb6c5824bf51209099000f9f))
    - filter null values of requrest structs ([`3efa4f2`](https://github.com/Byron/google-apis-rs/commit/3efa4f2b12219412cdabf8535e03974b94f71af5))
    - upgrade to hyper v0.4.0 ([`3fe2732`](https://github.com/Byron/google-apis-rs/commit/3fe2732a01371ededca2c35fe7499a4bbe63c318))
    - remove special clap configuration ([`8375dd0`](https://github.com/Byron/google-apis-rs/commit/8375dd0508c6e09761f79f8b47379cc240a0c7c4))
    - use `arg_enum!` clap-rs macro ([`2485343`](https://github.com/Byron/google-apis-rs/commit/2485343caa621bed4cca0df329abda7e61df813d))
    - special clap configuration ([`294da41`](https://github.com/Byron/google-apis-rs/commit/294da41a308e4f4125db876c5b24084ae8cfcb5f))
    - better vector building ([`ef63790`](https://github.com/Byron/google-apis-rs/commit/ef63790422db56158e2e1a6d651e329e14cd7ec0))
    - move global params to runtime ([`f1fe6ba`](https://github.com/Byron/google-apis-rs/commit/f1fe6bac018c2268d10233ec1635f0273f1192dc))
    - completed list of parameter names ([`9274938`](https://github.com/Byron/google-apis-rs/commit/9274938f9f69ecab2e8cb975467860f41466ad1d))
    - use raw strings for argparser ([`bbab1f2`](https://github.com/Byron/google-apis-rs/commit/bbab1f2e38f4445179e7385a9507098d6ff15cbf))
    - added dev diary episode 2 link ([`362781e`](https://github.com/Byron/google-apis-rs/commit/362781e6014a2b050f69f5487b33aba8434c14a0))
    - added information about imp ([`5ebb34e`](https://github.com/Byron/google-apis-rs/commit/5ebb34ed5839c6c9f588a95e5661b68d07c02d4d))
    - add support for 'imp' ([`ab22901`](https://github.com/Byron/google-apis-rs/commit/ab2290192ea790744851deb60e396ca908b6d190))
    - simplified call to form_urlencode ([`b27c990`](https://github.com/Byron/google-apis-rs/commit/b27c990db8a8701e2814e77136a34689be56c623))
    - added changelog ([`fb80b05`](https://github.com/Byron/google-apis-rs/commit/fb80b056ac6cc8eee1972eec979a7f810861f8b8))
    - did you mean for struct values ([`96415d1`](https://github.com/Byron/google-apis-rs/commit/96415d17ca383ba0653fb4df23df1ebe27d57f55))
    - added latest reference CLI code ([`d2a4e2f`](https://github.com/Byron/google-apis-rs/commit/d2a4e2ff8b16cb848869cc07b6c5a9107fb0a929))
    - gate usage of `upload_media_params` ([`89432cc`](https://github.com/Byron/google-apis-rs/commit/89432cc64600ba0711e412c6cf6b1e06e2f11102))
    - `-u <mode> <file>` parsing ([`75b80de`](https://github.com/Byron/google-apis-rs/commit/75b80de3c644a1487358561810c7c56bad8cca1d))
    - let delegate forget uploaded urls ([`c346645`](https://github.com/Byron/google-apis-rs/commit/c346645fc96abf9831ce723bb56e26f95e3c5b45))
    - disabled rust linter,but configured it ([`d8acd60`](https://github.com/Byron/google-apis-rs/commit/d8acd607aea2e4bed548695e46fe541cb72ee904))
    - handle repeated required strings ([`bf6a2ba`](https://github.com/Byron/google-apis-rs/commit/bf6a2ba60c364e7c30de198d335e481c0b3206f0))
    - 'about()' text for main commands ([`153324e`](https://github.com/Byron/google-apis-rs/commit/153324ebccf8a7846d9669f16c8f3ea52f0ec810))
    - adjust `JsonTokenStorage` to yup-oauth ([`94c821e`](https://github.com/Byron/google-apis-rs/commit/94c821e09d2b75756dd3dfa2d5f508b079413cf1))
    - unified error handling ([`2f20021`](https://github.com/Byron/google-apis-rs/commit/2f200217f942aa0317186811dbbe95d675a17ab0))
    - adjust to serde usage in `yup-oauth` ([`894b5b5`](https://github.com/Byron/google-apis-rs/commit/894b5b5ec7bf7cb027ba31bf83c40f27e0ab51bd))
    - escape subcommand descriptions ([`fac5041`](https://github.com/Byron/google-apis-rs/commit/fac50418a7156b1b2fa958008691dbb2f6cbb756))
    - update STRUCT_FLAG and UPLOAD flags ([`bd27046`](https://github.com/Byron/google-apis-rs/commit/bd27046cc8cd5ccf515355a5d810dace168a7db3))
    - to reflect recent changes ([`cc1bfd1`](https://github.com/Byron/google-apis-rs/commit/cc1bfd19c8aecb4a62bf68f3bf7db650eb8fc29d))
    - remove unused std_misc feature ([`d46c083`](https://github.com/Byron/google-apis-rs/commit/d46c083975201a6a4804fde9d4cec6ae0fc29479))
    - adjust to latest hyper header macros ([`4115d50`](https://github.com/Byron/google-apis-rs/commit/4115d50ca795ec2a2958f5f75b7681cb9f84720b))
    - Merge branch 'clap' ([`a2547b3`](https://github.com/Byron/google-apis-rs/commit/a2547b3321a90652805215f5790ef74ca60257f3))
    - re-introduce UploadProtocol,fix CallType ([`d0ce221`](https://github.com/Byron/google-apis-rs/commit/d0ce221ba39db621b969b8c1faad358c775502a5))
    - update docs and fix calltype handling ([`b039b38`](https://github.com/Byron/google-apis-rs/commit/b039b382446f450a58c12d2d881dbcd00b96928a))
    - various fixes and improvements ([`7a38f7e`](https://github.com/Byron/google-apis-rs/commit/7a38f7e4d5dea97b5bd2cbe6b10e4619b3b45b12))
    - print usage if command is missing ([`63e23dd`](https://github.com/Byron/google-apis-rs/commit/63e23dd48f7fb80268eb3bc95380b77b233de62a))
    - tweaks to make youtube3 work ([`5320a48`](https://github.com/Byron/google-apis-rs/commit/5320a48e68c0ee4457455c5caa5c01f322fc6c7e))
    - adjust option usage to changed API ([`bac4e1a`](https://github.com/Byron/google-apis-rs/commit/bac4e1a82fa331370c20a7c4843989f11974600c))
    - handle apis without media upload ([`feaa3a0`](https://github.com/Byron/google-apis-rs/commit/feaa3a06ed53ae039750e2d420817116b1140984))
    - call `iter()` directly ([`02a4129`](https://github.com/Byron/google-apis-rs/commit/02a41296628eb0cbc0c8b7b2e86b06678e8db084))
    - implement -u as good as possible ([`656fcae`](https://github.com/Byron/google-apis-rs/commit/656fcae2b481ac90254bf5e3081d2bbd659d5232))
    - commit before un-using UploadProtocol ([`1aff313`](https://github.com/Byron/google-apis-rs/commit/1aff3135d97435632599bf39cf5e8c5de9d773a8))
    - parse structure and build App ([`db4624b`](https://github.com/Byron/google-apis-rs/commit/db4624b46728379393372be40b1ce731fe8f28b4))
    - generate command data structure ([`8ac8d3b`](https://github.com/Byron/google-apis-rs/commit/8ac8d3b1cb59249d492a657fa8cd39fbe3fd99a7))
    - upload some code to help debugging ([`9a8ae4b`](https://github.com/Byron/google-apis-rs/commit/9a8ae4b7d66ec1b6a74316fceeccbf04a2f77469))
    - initial version of command generation ([`b39bc3a`](https://github.com/Byron/google-apis-rs/commit/b39bc3a9cd165db8f9ea3fa536697ca80d36628e))
    - setup infrastructure ([`988d37f`](https://github.com/Byron/google-apis-rs/commit/988d37f0dfaf8a1725bf92364e965c1f32e6802f))
    - make it work with latest hyper ([`57808cf`](https://github.com/Byron/google-apis-rs/commit/57808cf92adf7ff4dd65664a4a4ed3a361b60c6e))
    - visual gap between cursor and kv ([`4f98fc1`](https://github.com/Byron/google-apis-rs/commit/4f98fc175ea274d3854929c05637c337f1a6aaa7))
    - add link to general documentation ([`12743f5`](https://github.com/Byron/google-apis-rs/commit/12743f543d6021322915bfdd9b4d5ef6b88f72de))
    - simple linux deployment script ([`36513f1`](https://github.com/Byron/google-apis-rs/commit/36513f101e0c3299513fe1bf542c7fc7c492e771))
    - request values are moved, not borrowed ([`6f5c159`](https://github.com/Byron/google-apis-rs/commit/6f5c1599168524c0df0b47713ea4eb1a00d049f5))
    - simple osx deploy script ([`c248301`](https://github.com/Byron/google-apis-rs/commit/c248301951cc1266136e2ab7b6c6f5cc54d86164))
    - exclude cloudsearch from build ([`de85fb4`](https://github.com/Byron/google-apis-rs/commit/de85fb43e53723d1d38d0b6e8746acc962035233))
    - code updated to v0.1.6, latest CLI ([`c2dd9c7`](https://github.com/Byron/google-apis-rs/commit/c2dd9c7a020e0367bc87b20fa8054c85f48b71c1))
    - CLI + API release preps ([`4e275ea`](https://github.com/Byron/google-apis-rs/commit/4e275eaaddfd7a86ed42d04df24113015c6ea099))
    - filled README.md ([`944e04b`](https://github.com/Byron/google-apis-rs/commit/944e04bd12f6415e3818f444d2604fc103ec162b))
    - update changed `url` crate imports ([`607ba74`](https://github.com/Byron/google-apis-rs/commit/607ba745d140e5d2567a715c6ddaa775d2cf0d99))
    - integrate different program types ([`fbec9bd`](https://github.com/Byron/google-apis-rs/commit/fbec9bdbba375037ec3ac5886bb86390622194fd))
    - request value cursor handling and docs ([`b6a48bd`](https://github.com/Byron/google-apis-rs/commit/b6a48bdcd5fb215e94a00a69d11ce0ac007c2df3))
    - simple and resumable upload works ([`2f3b2d2`](https://github.com/Byron/google-apis-rs/commit/2f3b2d24ce2367356698b902becabb40b8636ab6))
    - use only one request structure ([`0bb30da`](https://github.com/Byron/google-apis-rs/commit/0bb30da78244abcf09c7d04571515e6584ccb4a3))
    - set request value to call ([`be7ccb0`](https://github.com/Byron/google-apis-rs/commit/be7ccb085cb5ea908fb75d0ae7cb6c91ded33bd4))
    - verified download works ([`6befdbc`](https://github.com/Byron/google-apis-rs/commit/6befdbc6fa730fc4a5513d2cad9e1784c580e516))
    - improved error handling ([`a328942`](https://github.com/Byron/google-apis-rs/commit/a3289420337c7f607f4393fcf0832167267cc473))
    - update all code to latest version ([`f8689be`](https://github.com/Byron/google-apis-rs/commit/f8689be4515d5693004da17bb2244a385ac1e794))
    - response value json decoding ([`845a568`](https://github.com/Byron/google-apis-rs/commit/845a568b25f387c58a17752852aed63e7305c7b1))
    - per-API-credentials with default ([`e42f6fb`](https://github.com/Byron/google-apis-rs/commit/e42f6fbedb0a2e609c6d1363a5c0eaa5b7967863))
    - implement deletion of tokens ([`6d84ef9`](https://github.com/Byron/google-apis-rs/commit/6d84ef906e6b9ff344fd7acac3140bdad3d48e78))
    - adapt to changed yup-oauth2 API ([`e523ddb`](https://github.com/Byron/google-apis-rs/commit/e523ddb6ec9f1e9e8bcc51fbec02e364dbddaa72))
    - resolve generator issues ([`797f289`](https://github.com/Byron/google-apis-rs/commit/797f289886d899a7e1b21216ee46218d179e38bf))
    - hashmap handling ([`b830c1c`](https://github.com/Byron/google-apis-rs/commit/b830c1c6decea4d5b3a16712b31daaa544cc837b))
    - repeated required args ([`c14ef9a`](https://github.com/Byron/google-apis-rs/commit/c14ef9afc86a17b5bc3952882f98fc7bf7a2ced8))
    - update make target ([`a4b73cc`](https://github.com/Byron/google-apis-rs/commit/a4b73cc1c4e3919cf8bf2f782d598d0840c4922f))
    - --debug-auth flag ([`03f35bd`](https://github.com/Byron/google-apis-rs/commit/03f35bd4f547da5843fab755ca678c01800aabed))
    - --debug flag to output traffix ([`159c659`](https://github.com/Byron/google-apis-rs/commit/159c65916f0fb4d0136a8cc622919daf60a7ecfd))
    - request-value parsing;Default ([`f7740ad`](https://github.com/Byron/google-apis-rs/commit/f7740ad149d78b4642670ff35deb6163ab56be22))
    - Merge branch 'docker' ([`52c8225`](https://github.com/Byron/google-apis-rs/commit/52c8225b8f2440cbd1c65c64d1710044648cb3e8))
    - README info + fix author email ([`e730281`](https://github.com/Byron/google-apis-rs/commit/e730281003b4a4caad0d48c2712e5d1433848bd7))
    - using docker ([`5165ff6`](https://github.com/Byron/google-apis-rs/commit/5165ff68dfe3d67f3bdcf8fee0a0d3aedee0fa33))
    - scopes were used illegally ([`d8fdf9d`](https://github.com/Byron/google-apis-rs/commit/d8fdf9df9f41719f6acb9bf3750aa8069cfab675))
    - (abf0548b5 2015-04-15) (built 2015-04-15) ([`9ea3fea`](https://github.com/Byron/google-apis-rs/commit/9ea3fea7750bce93c531f99b13c747c78a806b59))
    - latest version of all APIs ([`4cf0720`](https://github.com/Byron/google-apis-rs/commit/4cf0720ef1e025737416ad5fe07eff2389c86ad8))
    - added first versions of all CLI ([`f5f12c5`](https://github.com/Byron/google-apis-rs/commit/f5f12c559448f73a08a812f4ac40bfc6dafcbabb))
    - request value parsing compiles and inits ([`fa278a9`](https://github.com/Byron/google-apis-rs/commit/fa278a99c769e99727176f4faae081cc2d219342))
    - struct access compiles ... ([`bf22bef`](https://github.com/Byron/google-apis-rs/commit/bf22bef77ae62d06209c70d273ecccef29a4268a))
    - struct value parsing ([`15b78cd`](https://github.com/Byron/google-apis-rs/commit/15b78cd1ff148a20006e92fd9210e93f01d9f366))
    - field cursor complete and untested ([`1dd1fcf`](https://github.com/Byron/google-apis-rs/commit/1dd1fcf4b80e9554bac430326fa668b18cd9c678))
    - initial test ([`c9c3ad0`](https://github.com/Byron/google-apis-rs/commit/c9c3ad011fdb4ae693ddeef436f7a14de35ad7b0))
    - corrected cursor handling in mkdocs ([`bf37e51`](https://github.com/Byron/google-apis-rs/commit/bf37e515d2b5affec6296c34fbfa68fa89f7d4b9))
    - NULL default values instead of randoms ([`4b87d90`](https://github.com/Byron/google-apis-rs/commit/4b87d909f21daff696dd81da463fae3b14e59725))
    - make respective uppload_call ([`6119bfb`](https://github.com/Byron/google-apis-rs/commit/6119bfb7627c7e238a5641b0781bfca3689e8a36))
    - upload flag parsing ([`9eed405`](https://github.com/Byron/google-apis-rs/commit/9eed4056e53d71f2b8165fd4099fda6fc0d0798a))
    - alt-media handling in CLI+API-docs ([`306852d`](https://github.com/Byron/google-apis-rs/commit/306852d5147d7083ff011f990c5feedcf3e338bb))
    - global optional parameters+DL tracking ([`36a7cb2`](https://github.com/Byron/google-apis-rs/commit/36a7cb239a2717b54500ed41a346a382b092f76a))
    - optional parameter default handling ([`830529c`](https://github.com/Byron/google-apis-rs/commit/830529c40b6ab01381fe36f27753047a2b03244f))
    - parse method parameters and set them ([`6ae6ee8`](https://github.com/Byron/google-apis-rs/commit/6ae6ee88a05d8d8c76f69c4bff2c37684b3d81ad))
    - add rustc_serialize to test-crate ([`fa01131`](https://github.com/Byron/google-apis-rs/commit/fa011315c31815cf283ecff18e245553378f3cb9))
    - handle output json encoding and ostreams ([`3f49f50`](https://github.com/Byron/google-apis-rs/commit/3f49f50ac21fb921b61c1170c633214782f39cc7))
    - interpret output arguments ([`c3a9f1e`](https://github.com/Byron/google-apis-rs/commit/c3a9f1e8e594172ac783f0b9c76e093a534674ee))
    - optimze argument handling and conversion ([`76841da`](https://github.com/Byron/google-apis-rs/commit/76841da09801f23abef4955d76430ce1191c0b77))
    - required arg parsing + first doit() call ([`e34e24e`](https://github.com/Byron/google-apis-rs/commit/e34e24e04943e6cce8564295587bbf426c58077f))
    - infrastructure for call and dry-run ([`d6919f1`](https://github.com/Byron/google-apis-rs/commit/d6919f1eb65c7e29527360739555fce4a254d9e8))
    - Implementation of JsonTokenStorage ([`8afc76a`](https://github.com/Byron/google-apis-rs/commit/8afc76a7fe50ba8171f1e2045d989162c9864395))
    - init hub + refactor for dry-run mode ([`f71c286`](https://github.com/Byron/google-apis-rs/commit/f71c2862851f98c00fb893fa3b940a912b893845))
    - Display + Error traits for Error struct ([`7dc9972`](https://github.com/Byron/google-apis-rs/commit/7dc9972445593f592f369759b9839a3dedf8d12c))
    - engine checks resource and method args ([`be228f1`](https://github.com/Byron/google-apis-rs/commit/be228f19940d38e484809116c1bd84bb8edf5ee8))
    - Display for Errors + refactor ([`e45eb05`](https://github.com/Byron/google-apis-rs/commit/e45eb053d52db016342bd568d10bc368495dad86))
    - write default and read app-secret ([`4548644`](https://github.com/Byron/google-apis-rs/commit/4548644cb1498f4c7769d8e98cc7ddf8c0e4f47b))
    - create config directory, if possible ([`5799d44`](https://github.com/Byron/google-apis-rs/commit/5799d44fceb537f8f82ae4919682c9189a172792))
    - infrastructure ([`ca8e8c0`](https://github.com/Byron/google-apis-rs/commit/ca8e8c06220f858424c8c1b799b1f00bd89e9bb2))
    - random values + cursor information ([`b64722c`](https://github.com/Byron/google-apis-rs/commit/b64722cca85a0396cc1389da694e7abd2338ff2e))
    - absolute top-level cursor + details ([`47f9ca8`](https://github.com/Byron/google-apis-rs/commit/47f9ca8b209e0f2453dad6d8c121e60e138f511c))
    - relative cursor positioning ([`683cbbd`](https://github.com/Byron/google-apis-rs/commit/683cbbdd753611c6f09e5111bb1aa3c29e6b909d))
    - dynamic absolute cursor position example ([`92b1ef7`](https://github.com/Byron/google-apis-rs/commit/92b1ef7476d0adb9168c94b8d9bb1097ad682fbc))
    - improved scope handling; fix CLI ([`5b4f18d`](https://github.com/Byron/google-apis-rs/commit/5b4f18d341cbd8f87d3e3792b1dfa803f7849015))
    - upload and output flag ([`6d3bbce`](https://github.com/Byron/google-apis-rs/commit/6d3bbcea5713a7a868ba7e8def00ed18fda83b64))
    - optional paramters ([`24e0537`](https://github.com/Byron/google-apis-rs/commit/24e053718a6960466d4da69ba4113fc341646b69))
    - inforamtion about setting structs ([`c004840`](https://github.com/Byron/google-apis-rs/commit/c004840d5bbb5b5196a68b67f709008d055d496a))
    - add required scalar arguments ([`c65a8a6`](https://github.com/Byron/google-apis-rs/commit/c65a8a6bdf9296721f21f86266f744d656f00ee9))
    - name default scope in API docs ([`334061a`](https://github.com/Byron/google-apis-rs/commit/334061a5e20846cf4f21847c1950f58ca4f9c87e))
    - added CLI scope documentation ([`49c4a41`](https://github.com/Byron/google-apis-rs/commit/49c4a4101e73b516b67f66779072efe13a624ba6))
    - generate complete docopts grammar ([`310c81f`](https://github.com/Byron/google-apis-rs/commit/310c81f19cbfb8e1fc7d7f3766492c002a340761))
    - add commands.yml.mako ([`51ddcf7`](https://github.com/Byron/google-apis-rs/commit/51ddcf74a6d1cf204156c6a018ced2f2d85c9352))
    - dependencies are now per-program-type ([`acd42df`](https://github.com/Byron/google-apis-rs/commit/acd42dfccc87f49cf5c9bf51a206da8bed9c02c2))
    - 'bytes ...' -> 'bytes=...' ([`3e0a24d`](https://github.com/Byron/google-apis-rs/commit/3e0a24db0d8d25fec9457364d49106c22aee3c23))
    - better subtext + rename target ([`75e73d5`](https://github.com/Byron/google-apis-rs/commit/75e73d56d95dc4126ef39f0ae60d901a32af9954))
    - one folder per API docs ([`6d3dc77`](https://github.com/Byron/google-apis-rs/commit/6d3dc77635724602a89026477bfc0f8f785968ba))
    - use bytes=... when sending as well ([`b9a469c`](https://github.com/Byron/google-apis-rs/commit/b9a469c0a4e655da54940dc2876559f573c88c08))
    - per-method-markdown-files ([`3cef120`](https://github.com/Byron/google-apis-rs/commit/3cef120c58d304e120ba5e86a1717f1c47452452))
    - cli postprocessing support ([`c78ea53`](https://github.com/Byron/google-apis-rs/commit/c78ea5381aeeb7c97ce4fc35e0c9da40a7022423))
    - fix dependencies ([`2e74d91`](https://github.com/Byron/google-apis-rs/commit/2e74d9141313da1cc6a26149650ee59c43047f06))
    - docopt subcommands ([`39253d9`](https://github.com/Byron/google-apis-rs/commit/39253d988af3d7795b2167edb3a54b8988dda00c))
    - bin renaming + docopt infrastructure ([`f527c82`](https://github.com/Byron/google-apis-rs/commit/f527c8202b961d3dcb4c30a13e3c28a650fb144c))
    - basic usage of docopts ([`390354b`](https://github.com/Byron/google-apis-rs/commit/390354bd08b429fb438d60c54e2a36756e086c3c))
    - add publish state v0.1.5 ([`6db7332`](https://github.com/Byron/google-apis-rs/commit/6db733274d65f10a213612561a5771bf4b7b8316))
    - corrected absolute links ([`34d0a7a`](https://github.com/Byron/google-apis-rs/commit/34d0a7aad3b139c71b4d0dd7ca4e10c1336ebb8f))
    - v0.1.5 ([`a399488`](https://github.com/Byron/google-apis-rs/commit/a399488c2799e1acca0961f80a6c116a3330190c))
    - adjust to hyper client ([`191e822`](https://github.com/Byron/google-apis-rs/commit/191e822c5a93771e32e85bc5c00ef450c6719fb6))
    - v0.1.4 ([`6f2149b`](https://github.com/Byron/google-apis-rs/commit/6f2149b7d49ee693cc616b92f9de79f220ce6e2d))
    - adjust invalid make target ([`9dbdcc4`](https://github.com/Byron/google-apis-rs/commit/9dbdcc465f45c13faa85e5489073e7b7f5e18133))
    - v0.1.4 ([`dd1d191`](https://github.com/Byron/google-apis-rs/commit/dd1d191966aa41ec66c5a4baba5ebd43771c3a05))
    - v0.1.3 ([`3403bd1`](https://github.com/Byron/google-apis-rs/commit/3403bd1c5cec379cd2ad98040cca0ec6a4eef4a3))
    - version 0.1.3 ([`99f8b65`](https://github.com/Byron/google-apis-rs/commit/99f8b65f75822d54f32100655d0b5678f43a8478))
    - rustc (be9bd7c93 2015-04-05) ([`91861dc`](https://github.com/Byron/google-apis-rs/commit/91861dcb71b371e8ec5511ddedee0ae45cee9af0))
    - github-pages index generation ([`919ae4d`](https://github.com/Byron/google-apis-rs/commit/919ae4d8ae85f35f54c69c8c222ba43ba304e263))
    - update to include CLI targets ([`74bb79d`](https://github.com/Byron/google-apis-rs/commit/74bb79d6b4b73b0031ee233cf9a1667f7fdb8070))
    - check-in of latest sources ([`a2ca1cb`](https://github.com/Byron/google-apis-rs/commit/a2ca1cb28ec1ce9f5f381f55ea78aa59a56ea915))
    - set the API version to 0.1.2 ([`c7fb7c4`](https://github.com/Byron/google-apis-rs/commit/c7fb7c409343f19e26f1c3d488718decec7990b0))
    - incl. `Result` conform to standards ([`e953535`](https://github.com/Byron/google-apis-rs/commit/e953535473429b01293d679e23337b74645e0c18))
    - mkdocs generator works now ([`d1c9791`](https://github.com/Byron/google-apis-rs/commit/d1c97912cbebf8df3f2817b04b15a78d952b092a))
    - Merge branch 'refactor' ([`aa842bc`](https://github.com/Byron/google-apis-rs/commit/aa842bcc3911b944b00a2590667e64b36b0dedc6))
    - cli depends on API, generically ([`cefd606`](https://github.com/Byron/google-apis-rs/commit/cefd606b538ed86d7b659f83b64ee2b14f71fc3b))
    - prepare dep generation to use suffix ([`caaf62e`](https://github.com/Byron/google-apis-rs/commit/caaf62e51dae4b3f5a405008958a60a83c9aed43))
    - api generation works once again ([`be7d821`](https://github.com/Byron/google-apis-rs/commit/be7d8214c16287fb245918c38561544245a0aa1d))
    - put API relevant stuff into subdir ([`137ba8c`](https://github.com/Byron/google-apis-rs/commit/137ba8caf3b9bad5bb7d8e4a9fb236e9988476f2))
    - remove newlines interpreted as test ([`d1c5bf1`](https://github.com/Byron/google-apis-rs/commit/d1c5bf1e4ab2a91c30d2bcbd1e08a1a02c73ad41))
    - remove custom Result Enum ([`e5b013e`](https://github.com/Byron/google-apis-rs/commit/e5b013e97c56040dba266a43a8308448a32645eb))
    - update json files from discovery API ([`fca1b24`](https://github.com/Byron/google-apis-rs/commit/fca1b24cd186b090f75e35f362c8bbb2754e3e4d))
    - make publish ([`4bfdc9f`](https://github.com/Byron/google-apis-rs/commit/4bfdc9fd015b95bf9f3bcd311818de6cee342c9e))
    - typo fixes and misc. improvements ([`ea16189`](https://github.com/Byron/google-apis-rs/commit/ea161897f5fe25e024292755c753f2410211bea1))
    - whitespace and trait rename ([`6ad0c2e`](https://github.com/Byron/google-apis-rs/commit/6ad0c2ef79a634d4cb631a36eb92b2cf82b59121))
    - upload size now taken properly ([`04f4c95`](https://github.com/Byron/google-apis-rs/commit/04f4c95688f2cef0866ce07da68ae9d710596c7c))
    - 0.1.0 release ([`3bc930a`](https://github.com/Byron/google-apis-rs/commit/3bc930ae47c2544de4825ecec5346f53626a75e2))
    - upload() return value handling ([`cd1ff18`](https://github.com/Byron/google-apis-rs/commit/cd1ff18ba94966088a779b26347dc683f1f0c2d3))
    - Resumable upload implemented ([`29ee94b`](https://github.com/Byron/google-apis-rs/commit/29ee94b4c04f72d2676a98dda6632a06c5b8ba54))
    - implement query_transfer_status() ([`065753c`](https://github.com/Byron/google-apis-rs/commit/065753cc3a56227c2e87fbcc8b36121dc3bb1ab6))
    - ContentRange header (parse and format) ([`42a76e4`](https://github.com/Byron/google-apis-rs/commit/42a76e465549beadd3080c36f68922d8e44fba54))
    - re-export types used by delegate ([`556906c`](https://github.com/Byron/google-apis-rs/commit/556906ca60a90fc6eb34917d42813daf9792fbcb))
    - better introduction and version handling ([`3a9aa51`](https://github.com/Byron/google-apis-rs/commit/3a9aa519496be9da6283b847f38d9a2deaf682aa))
    - use of oauth2::Scheme ([`d26cf77`](https://github.com/Byron/google-apis-rs/commit/d26cf7740614134e97f1b6add19c3b91242fc994))
    - repository/source-code link ([`030c40d`](https://github.com/Byron/google-apis-rs/commit/030c40d2699196e29d1c8606d042403df52a7534))
    - crate version +<revision> ([`8ad316b`](https://github.com/Byron/google-apis-rs/commit/8ad316bda3fd5eaa7e9a993ff1a9120e71022365))
    - check upload size against max-size ([`57e0f06`](https://github.com/Byron/google-apis-rs/commit/57e0f0658379db524f1a964232a3fa39111be626))
    - minor phrasing changes ([`c1d09e6`](https://github.com/Byron/google-apis-rs/commit/c1d09e6d576b6f6bb1245af6e0b9b166c5f69b2f))
    - fix(cmn) re-export important types from cmn ([`f73204c`](https://github.com/Byron/google-apis-rs/commit/f73204c6b92eb03a6facd3432e1a28b68dc30f69))
    - make actual `store_upload_url()` call ([`ffef7dd`](https://github.com/Byron/google-apis-rs/commit/ffef7dda57c8f3f14d86712107416eaffe4c1bfc))
    - improved delegate calls ([`9ea8527`](https://github.com/Byron/google-apis-rs/commit/9ea85273cd18798c7f0c523a45de1f25c0648c92))
    - simplification and cleanup ([`4bf2800`](https://github.com/Byron/google-apis-rs/commit/4bf280079ed5cf33c4ed2617c3aa62151ec0dcd0))
    - resumable-upload infrastructure ([`307d3f4`](https://github.com/Byron/google-apis-rs/commit/307d3f487c6b35f42be643505a4e65c6ce04e6ec))
    - schema_markers() accessed map incorrectly ([`98f4bba`](https://github.com/Byron/google-apis-rs/commit/98f4bbab4774fb166936c60cbe8eee2302f35052))
    - prune unused and ToParts trait ([`80161f7`](https://github.com/Byron/google-apis-rs/commit/80161f72be1aa7f7551603c90752793c84eedb6d))
    - pretty names for methods and resources ([`0152138`](https://github.com/Byron/google-apis-rs/commit/0152138e0c019575caa3e40f87f19382d92a63ac))
    - exclude those with recursive schemas ([`5ff2285`](https://github.com/Byron/google-apis-rs/commit/5ff22851faec165258e5c3ff9c6eed58df3efee3))
    - add latest version ([`9abf0eb`](https://github.com/Byron/google-apis-rs/commit/9abf0eb64a96544b6db313690e4b3267c6987df7))
    - make recursive types possible ([`8d9f175`](https://github.com/Byron/google-apis-rs/commit/8d9f175f917ec19e4752c5c3806f6f5624e066e2))
    - don't crash if json decode fails. ([`0823dec`](https://github.com/Byron/google-apis-rs/commit/0823dec75cc89b8e0a87a41ab2dcd1d5a405a24e))
    - mark unused types with marker trait ([`8bb2166`](https://github.com/Byron/google-apis-rs/commit/8bb2166da0a11db45a68e53518e94119b6d5a3b3))
    - MethodBuilder -> CallBuilder ([`10dfeeb`](https://github.com/Byron/google-apis-rs/commit/10dfeeb1aa5a1de2919e9753444e8e63855d1285))
    - improved markdown for library overview ([`97da926`](https://github.com/Byron/google-apis-rs/commit/97da926e28d7ad7ed90d12b7ff48477bcf67ee68))
    - support for 'variant' schema ([`bb75c5b`](https://github.com/Byron/google-apis-rs/commit/bb75c5b69871ec88c888618d0c3292741c9cffff))
    - Option<_> in schema only if needed ([`55978ff`](https://github.com/Byron/google-apis-rs/commit/55978ff9a2fe332c5ed46476af4f921a72999e5c))
    - Merge branch 'serde' ([`29ecef6`](https://github.com/Byron/google-apis-rs/commit/29ecef622aeeccfdbe21064bea39bee7ef58ecdf))
    - just add latest youtube code ([`ff385e5`](https://github.com/Byron/google-apis-rs/commit/ff385e5cacb43d173912243fc033578b0c0b0f63))
    - Vec/HashMap are Optionals ([`cfb8fae`](https://github.com/Byron/google-apis-rs/commit/cfb8faefb8545114ddadea59871214b35e515d5a))
    - added field aliases, were needed ([`9f719dd`](https://github.com/Byron/google-apis-rs/commit/9f719dd9287ee112fa6c3ebb6be64e9793da8a81))
    - serde cleanup;JsonError pub fields ([`b9a81a9`](https://github.com/Byron/google-apis-rs/commit/b9a81a900ec054b102ce045cf25a4348c297f260))
    - use serge instead of serialize ([`d3bb130`](https://github.com/Byron/google-apis-rs/commit/d3bb130be0b25f984c75ab125d2b344929865213))
    - prevent type-clash with `Result` ([`b6ebb1e`](https://github.com/Byron/google-apis-rs/commit/b6ebb1ec371c833ef7386264ed9522b880586316))
    - simplify delegate calls ([`265b448`](https://github.com/Byron/google-apis-rs/commit/265b448297493afe11c38ac751376c67907e84da))
    - prevent duplicate schema types ([`3a15430`](https://github.com/Byron/google-apis-rs/commit/3a1543033949b8f25e2e3cd888c9f43029b4de3d))
    - begin()/finished() calls ([`508d14e`](https://github.com/Byron/google-apis-rs/commit/508d14eafbca167f9801a2ca7ff9a1ae922be734))
    - some links pointed to old doc name ([`a05426e`](https://github.com/Byron/google-apis-rs/commit/a05426e79b8c0773dbb219b327539431e4d1fdfc))
    - deal with 'virtual' methods resource ([`aadf370`](https://github.com/Byron/google-apis-rs/commit/aadf37004ee609d940674f6f30ae4c942ba522c8))
    - method features and general info ([`2f293f5`](https://github.com/Byron/google-apis-rs/commit/2f293f5e1bc14b8189d38424ef24d829fedd8743))
    - alt 'media' handling to allow dls ([`02d7a06`](https://github.com/Byron/google-apis-rs/commit/02d7a06fdff10d54c93d00fa18e0330e1f536162))
    - crates with 'google-' prefix ([`4a27ac7`](https://github.com/Byron/google-apis-rs/commit/4a27ac7e1d14207645915637c4817a17f10916b9))
    - allow to set user-agent ([`cb5a0a3`](https://github.com/Byron/google-apis-rs/commit/cb5a0a35bc36cbf234e2ac5d2cec0b2c14ac1d2f))
    - optimizations and simplification; seek ([`9d401f5`](https://github.com/Byron/google-apis-rs/commit/9d401f5486b447ea0fc43cb0d4bb84fac3329357))
    - MultiPartReader test case ([`6b23013`](https://github.com/Byron/google-apis-rs/commit/6b2301351f6792fb37b7dfec6c1f0592fdc6b9cc))
    - optimized memory allocation and options ([`224af64`](https://github.com/Byron/google-apis-rs/commit/224af64068c60649266aff7cc06abd001053015b))
    - MultiPartReader now works correctly ([`e53e23a`](https://github.com/Byron/google-apis-rs/commit/e53e23a893ce6d59777b8b53f94770d5c3c86b9c))
    - multibytereader single byte test ([`b127df1`](https://github.com/Byron/google-apis-rs/commit/b127df17b02a4823e74a5125961bdfa23f77f7a0))
    - MultiPartReader is working. ([`8db346b`](https://github.com/Byron/google-apis-rs/commit/8db346b8b01f003fed24d202822c398fa0994443))
    - initial part writing ([`71c827b`](https://github.com/Byron/google-apis-rs/commit/71c827b3067131a150bfd4a3503a61b836ec39b5))
    - multi-part mime-type and add_parts() ([`fc589cb`](https://github.com/Byron/google-apis-rs/commit/fc589cb965848332dd944a790cafd7d4745d9fc7))
    - handle 'alt' param ([`3ea5e19`](https://github.com/Byron/google-apis-rs/commit/3ea5e194859749e05632edcfd35cc21db8cf53ff))
    - fix lifetime issues ([`29d9e45`](https://github.com/Byron/google-apis-rs/commit/29d9e45c9fc8bbdbed23d3d5a9be20f8023bb22d))
    - more multipart infrastructure ([`b0a1f51`](https://github.com/Byron/google-apis-rs/commit/b0a1f518e957c96a0f5b5b2297a738cb42032e87))
    - improve body infrastructure ([`7cfb5af`](https://github.com/Byron/google-apis-rs/commit/7cfb5afd394041019899ca4cdcf10c9187204409))
    - remove map!, better dlg call ([`76827ff`](https://github.com/Byron/google-apis-rs/commit/76827ff6659d33b7b9430e4971a7189fa0d23798))
    - repeated params string addition ([`b90a191`](https://github.com/Byron/google-apis-rs/commit/b90a1916889b2d1cc6c595c3cd121739223db345))
    - simplify URL_ENCODE handling ([`d2bf24c`](https://github.com/Byron/google-apis-rs/commit/d2bf24ca859b945e1f5ee64dc5ccdf7357d01184))
    - uri-template handling complete ([`1fee21d`](https://github.com/Byron/google-apis-rs/commit/1fee21de24eee4fd62151595ef7915987f7a39db))
    - uri-template generation works ([`54eb784`](https://github.com/Byron/google-apis-rs/commit/54eb784a550a619b3773e44fc2ddd0b2a58ffcd2))
    - repeated parameters docs improvement ([`863a98c`](https://github.com/Byron/google-apis-rs/commit/863a98c0d7932475dc207d204ec91c26ddec326c))
    - repeated types in examples ([`64219e7`](https://github.com/Byron/google-apis-rs/commit/64219e7e7eed42f7491a2aba80f5e8fd7567385e))
    - repeatable parameters working ([`d758f41`](https://github.com/Byron/google-apis-rs/commit/d758f410f68b84cb635a6a0633bb09b147939397))
    - regenerate .api.deps less often ([`6399791`](https://github.com/Byron/google-apis-rs/commit/63997910decf909a8242a8a7f16f6a4c276e1d67))
    - decent solution for free methods ([`79879da`](https://github.com/Byron/google-apis-rs/commit/79879daf1b2a52593d2bc9b51ba244bfaddcf1f0))
    - intermed. support for 'methods' ([`60d953a`](https://github.com/Byron/google-apis-rs/commit/60d953a3428d11591954e7488bc46078d4765b1f))
    - partial implementation of url expr ([`3543707`](https://github.com/Byron/google-apis-rs/commit/354370705dd317b9839cf9a6ad34e22b9efe12dc))
    - set upload media type ([`33e85dd`](https://github.com/Byron/google-apis-rs/commit/33e85ddd29db5a75ce49718d850652c36ad7ce25))
    - unit-tests work once again ([`91f69ff`](https://github.com/Byron/google-apis-rs/commit/91f69ffd6ed85790d8b6d1c8b5b63d7f4c7e6259))
    - add more obvious crate and api version ([`79cbf3e`](https://github.com/Byron/google-apis-rs/commit/79cbf3ee3fccdbfadcb1176ebc319f8bbabb8b68))
    - pre-request delegate call. ([`60adacf`](https://github.com/Byron/google-apis-rs/commit/60adacf8d47eb43a0f82642a69c5216e79285dbc))
    - json decode and delegation ([`eef1471`](https://github.com/Byron/google-apis-rs/commit/eef1471357e7a16f7501575bcca1d17cddf05515))
    - authentication with and without scopes ([`2c79f6e`](https://github.com/Byron/google-apis-rs/commit/2c79f6e3cfbf7044a061eef1ddfb6fadac19401d))
    - remove BorrowMut until it's cleared ([`1349c78`](https://github.com/Byron/google-apis-rs/commit/1349c786b7e986511e4c2ca058d45bebb7f458dd))
    - attempt to send json-encoded request ([`9a58b0b`](https://github.com/Byron/google-apis-rs/commit/9a58b0badd0fea4220cccb953f6deb00c8edbaaa))
    - user lower-case library names,always ([`814c9c9`](https://github.com/Byron/google-apis-rs/commit/814c9c9ffab64a7607f4056fbad4203ea8f19991))
    - Merge pull request #13 from MrFloya/master ([`881c9dd`](https://github.com/Byron/google-apis-rs/commit/881c9dd3d2eb8fcfa7039d69e99857c350db7c5b))
    - force python2.7 in virtualenv ([`876772c`](https://github.com/Byron/google-apis-rs/commit/876772cf2296c4b7c80c2f828e245c903da67802))
    - incorrectly capitalized cargo.toml ([`31efbf4`](https://github.com/Byron/google-apis-rs/commit/31efbf4fb0033b9f1fdfae0054ece1717ec05b79))
    - add cargo.toml dependency information ([`7f33cf2`](https://github.com/Byron/google-apis-rs/commit/7f33cf22a5c22e3cc50dcc199604af78ba8e13fa))
    - add build instructions ([`bec5cd5`](https://github.com/Byron/google-apis-rs/commit/bec5cd5e5c12a38168e0a117adccccd6e3407e9f))
    - explicit subshell for cargo-doc ([`4c657ac`](https://github.com/Byron/google-apis-rs/commit/4c657ac9d132257a392bfbf2ed861142b6baf36a))
    - try using a subshell for cargo cmd ([`a87fbdf`](https://github.com/Byron/google-apis-rs/commit/a87fbdf0a86cfa410c79671aee931e3bf95fab11))
    - fixed dependency to wrong target ([`51d05d6`](https://github.com/Byron/google-apis-rs/commit/51d05d6db01edb4f78159c3c07d77d0aceb85b89))
    - install virtualenv automatically ([`5fd7cb5`](https://github.com/Byron/google-apis-rs/commit/5fd7cb511407de7176dc07c1443ef07075c063a4))
    - docs and tests of youtube3 on travis ([`dd0772f`](https://github.com/Byron/google-apis-rs/commit/dd0772f1d7e1330229bb36040686f91e088befd2))
    - fully qualified activity names ([`8006bb8`](https://github.com/Byron/google-apis-rs/commit/8006bb8ca910b14ece8dee6230d476a361c7c163))
    - Do not generate docs ! ([`b43eb0e`](https://github.com/Byron/google-apis-rs/commit/b43eb0e301c068500777fe580c1bd1017d0819b1))
    - removed generated go file ([`2c2585d`](https://github.com/Byron/google-apis-rs/commit/2c2585d16d5dd346fb16809db05495be93ab571b))
    - added milestone link ([`6167dc0`](https://github.com/Byron/google-apis-rs/commit/6167dc07fc63cec22a8d2b01fe69f05f03ac3f9a))
    - initial version ([`6800edb`](https://github.com/Byron/google-apis-rs/commit/6800edb4dd9b3655da231ef483780144c2b52884))
    - update-json using discovery API ([`c0a2476`](https://github.com/Byron/google-apis-rs/commit/c0a247605890be6553fa4709074b4c4ca4a199a9))
    - use function to make links correctly ([`c8061eb`](https://github.com/Byron/google-apis-rs/commit/c8061ebe2fbe97274c68b7af6e5a8d08c0245139))
    - Merge branch 'docs' ([`3269052`](https://github.com/Byron/google-apis-rs/commit/32690524cf38a6203529d9d8e5ba73fedd591e13))
    - result handling and remaining todos ([`4cf365d`](https://github.com/Byron/google-apis-rs/commit/4cf365d0263b66ee538eb5e31144469a3018d856))
    - full usage example on landing page ([`9a17ab9`](https://github.com/Byron/google-apis-rs/commit/9a17ab9e4e98d8797a9912d3d5094c0e2bf9716f))
    - assured it handles '0' correctly ([`4b9dbb2`](https://github.com/Byron/google-apis-rs/commit/4b9dbb28ff474661855f53143862b621e650f157))
    - oauth22 -> oauth2_v2 ([`664d822`](https://github.com/Byron/google-apis-rs/commit/664d8225d2d5275148395828af02c0bc54b7ee24))
    - make 'regen-apis' work ([`97b2649`](https://github.com/Byron/google-apis-rs/commit/97b2649094cc225d0cfc42857140f0d245e11352))
    - improved library names ([`b895610`](https://github.com/Byron/google-apis-rs/commit/b8956103d9460c73956dbc28ca2f1684ba8b853c))
    - bigger font for doc-index ([`206ccad`](https://github.com/Byron/google-apis-rs/commit/206ccadbb3849c27247d3670c3bf4591636b66d0))
    - new github-pages target ([`f27fda8`](https://github.com/Byron/google-apis-rs/commit/f27fda8f34e084e1532f4e6528b93e156f062503))
    - generate doc index ([`0bc6d21`](https://github.com/Byron/google-apis-rs/commit/0bc6d216c3be583cfc58143e136a16e573d7277f))
    - typo ([`7758f99`](https://github.com/Byron/google-apis-rs/commit/7758f99ff2e19c3518eddcfca2e1adeee12e0659))
    - Merge branch 'builders' ([`8e78502`](https://github.com/Byron/google-apis-rs/commit/8e78502ffe81fd67103b109a3fec64b10620d65a))
    - fix incorrect nested type names ([`4f794ef`](https://github.com/Byron/google-apis-rs/commit/4f794ef5ff7b5a068a568056d2bfd7372ec9b57c))
    - finally, we pick up all types ([`7e24393`](https://github.com/Byron/google-apis-rs/commit/7e243936f226f6e26d2b551765b62cddc866776b))
    - transitive, minimal traits for types ([`00de2b1`](https://github.com/Byron/google-apis-rs/commit/00de2b187d74fd78f049a13d1517fc91d218da71))
    - no unused types anymore ([`e3ab233`](https://github.com/Byron/google-apis-rs/commit/e3ab233a6cee8482c1c98b1e2c759e7a17cceab9))
    - now we pre-generate nested schemas ([`ac8c415`](https://github.com/Byron/google-apis-rs/commit/ac8c41530d082203f93d81851682d02ed5c98d9a))
    - part 1 to implement 'any' type ([`712fed5`](https://github.com/Byron/google-apis-rs/commit/712fed578a377c27bd6153b098ee4b3244b0355e))
    - improved camelCasing ([`de40a8b`](https://github.com/Byron/google-apis-rs/commit/de40a8bd1ee8759287cd2a489cc5d995c296a07e))
    - protect from nested-type-clash ([`614539a`](https://github.com/Byron/google-apis-rs/commit/614539a925c5e64508fa28506b1c6db3ccd96882))
    - nested type names are consistent now ([`32145e6`](https://github.com/Byron/google-apis-rs/commit/32145e645ea29ff43c451530906356564e12f817))
    - scope -> add_scope ([`538120f`](https://github.com/Byron/google-apis-rs/commit/538120f7d1425e026220211857658a775c958577))
    - deduplicate object creation code ([`5b5ad43`](https://github.com/Byron/google-apis-rs/commit/5b5ad43bfa06f5c525a7e00b537381cefe6b7aa4))
    - improved nested array type handling ([`dfcd554`](https://github.com/Byron/google-apis-rs/commit/dfcd554faa36cbcdf18ab985c2aed744dd45dc6d))
    - prevent struct recursion issue ([`da57505`](https://github.com/Byron/google-apis-rs/commit/da57505567a58b59f320016d92b50f1ea248067c))
    - nicer code and identifiers ([`9b308bb`](https://github.com/Byron/google-apis-rs/commit/9b308bb6ddebe979abca6f46da131c822f95c639))
    - nested types work for arrays ([`54540e6`](https://github.com/Byron/google-apis-rs/commit/54540e695a9b246ca3d412ab62e843e4dd7974d0))
    - now deals with non-objects ([`50fa189`](https://github.com/Byron/google-apis-rs/commit/50fa189a715332a7ce49fc7a9c95e5a1ef22b81f))
    - optionals are working once again ([`a268be2`](https://github.com/Byron/google-apis-rs/commit/a268be27d2123a77259fa1d7d1f831c7e72c4459))
    - nested type resolution and hashes ([`5d563c8`](https://github.com/Byron/google-apis-rs/commit/5d563c88a8e3ccb33ebe381b47beb6ecfd4444fc))
    - remove compiler warnings. ([`559cb8f`](https://github.com/Byron/google-apis-rs/commit/559cb8fe458e18fec05d0ca3cd2847fb981f2da0))
    - no compiler warnings ([`bfc3922`](https://github.com/Byron/google-apis-rs/commit/bfc392291666a40cf3fbe4db3dfeda69d23018fa))
    - deepcopy dicts instead ([`efe56ad`](https://github.com/Byron/google-apis-rs/commit/efe56ad25081b632f1e65fd8292e9c4d535659bc))
    - fixes to help more projects to build ([`cf258bf`](https://github.com/Byron/google-apis-rs/commit/cf258bf4e5148723940cc757ec032b5aff814f1e))
    - fix name clashes ([`d99ba9c`](https://github.com/Byron/google-apis-rs/commit/d99ba9c5b3c5f73ad148679a866698c811eec495))
    - deal with missing auth information ([`df9f029`](https://github.com/Byron/google-apis-rs/commit/df9f0299bf5db0b7affdd90b4dfb331c74f543f2))
    - resource-to-category map ([`c7e169d`](https://github.com/Byron/google-apis-rs/commit/c7e169dff3712ff5f73497d2d9cba3303a83277a))
    - do not degenerate during activity_split ([`7816cc8`](https://github.com/Byron/google-apis-rs/commit/7816cc81455c1c7a48e84289e176baf25e8480e2))
    - asssure candidate is in mapping ([`1e332dd`](https://github.com/Byron/google-apis-rs/commit/1e332ddb91540c19586e6d85869c8e54c47552b0))
    - intermediate improvements ... ([`92d8fa7`](https://github.com/Byron/google-apis-rs/commit/92d8fa76d0f419738e2efa7df3deebb974c1e0cf))
    - ignore beta/alpha,assure latest ([`ff5cbb3`](https://github.com/Byron/google-apis-rs/commit/ff5cbb3bf410276fbe5af8cc966ac363e448970c))
    - api-list is now in separte file ([`9377220`](https://github.com/Byron/google-apis-rs/commit/9377220c59cf9a8a29720b0528b9263e9e947580))
    - build all apis, were possible ([`2d036b6`](https://github.com/Byron/google-apis-rs/commit/2d036b6623a6f21e7d5706b382e2bc1e28dac87c))
    - now with flattened activities ([`2531011`](https://github.com/Byron/google-apis-rs/commit/2531011fc579df4edc38b15de459c135975fa077))
    - first recursive resource support ([`35bd1c3`](https://github.com/Byron/google-apis-rs/commit/35bd1c3e9c8a6ab52068e279d8f925eea8af055d))
    - make scope gen work with gmail ([`3b7e63f`](https://github.com/Byron/google-apis-rs/commit/3b7e63f28675ea2646c88dfa16c62c063e076b96))
    - update-json and all APIs ([`7b81646`](https://github.com/Byron/google-apis-rs/commit/7b81646f43c1c4de5165c5b3e9e7ea5c836eb664))
    - cargo calls for any API ([`874cfb6`](https://github.com/Byron/google-apis-rs/commit/874cfb6f680cd601271befa66e86a54c59e3618b))
    - scopes are sorted Strings now ([`6d2b0fc`](https://github.com/Byron/google-apis-rs/commit/6d2b0fc2649bc5203c07c29dd020b50550d15746))
    - Manual scope parameter ... ([`28878e0`](https://github.com/Byron/google-apis-rs/commit/28878e0618cbb5632a1353ceb2048a913e9355d2))
    - new Scope enum type ([`bb76832`](https://github.com/Byron/google-apis-rs/commit/bb76832b2f317501d398f5ea9fe8ea6b12dacf7b))
    - for additional parameters ([`9bcb3f8`](https://github.com/Byron/google-apis-rs/commit/9bcb3f8ba900e313bea4fd4203177851e6e86f9a))
    - scope as property ... ([`e1b7a63`](https://github.com/Byron/google-apis-rs/commit/e1b7a63f0660682a1680d9651cd5c3e784b12030))
    - query string setup ([`aabed38`](https://github.com/Byron/google-apis-rs/commit/aabed3858143bcd28d4b95e3831c408d3120719b))
    - generic result type ([`da300e0`](https://github.com/Byron/google-apis-rs/commit/da300e035ebc92728c5566071c26505a38b409f6))
    - additional fields and Result type ([`7c6f7d5`](https://github.com/Byron/google-apis-rs/commit/7c6f7d5e97344e7df0f397c65209795e5b8515bc))
    - put all fields onto a list ([`6c41660`](https://github.com/Byron/google-apis-rs/commit/6c4166094358fd236490239d12235a80b738f34f))
    - it now works in every which way ([`1423e46`](https://github.com/Byron/google-apis-rs/commit/1423e46210d95d823ff9bee9896cf407b0e9f0cc))
    - spike to see how delegate can be work ([`432faa2`](https://github.com/Byron/google-apis-rs/commit/432faa275f89bb1c3ab00b60ff07225eec5a4489))
    - first attempt to get it to work ([`678b692`](https://github.com/Byron/google-apis-rs/commit/678b6929ca7bffb4e4495272330aac02a082dbcd))
    - cross linking of resources/activities ([`ac35432`](https://github.com/Byron/google-apis-rs/commit/ac35432b3f200a02a1272b3e295dcf6029e8b441))
    - docs for terms.upload methods ([`4b12da4`](https://github.com/Byron/google-apis-rs/commit/4b12da4a12927f363f9ce2e208a1c92f05bbda2f))
    - added size and mime type support ([`baea071`](https://github.com/Byron/google-apis-rs/commit/baea071a6f1c52410c0ca79cf24ab325f6efa586))
    - doit() call with enum type annotation ([`6fad760`](https://github.com/Byron/google-apis-rs/commit/6fad7600a03f2f6a3964f309fc8e277b34f8aa60))
    - media-upload doit() methods ([`5b2d8a7`](https://github.com/Byron/google-apis-rs/commit/5b2d8a77a3cf17a1c5989e856b1ae2dc77613264))
    - `param()` to set any parameter ([`de0c7a4`](https://github.com/Byron/google-apis-rs/commit/de0c7a4ae049b6f7fbc256d64bc363ebd8de2101))
    - recursion for nested types ([`0d9f636`](https://github.com/Byron/google-apis-rs/commit/0d9f6363eb271f95624559b06cfd07ab6b5bc9b5))
    - added gogole drive API ([`66f3ae1`](https://github.com/Byron/google-apis-rs/commit/66f3ae14e5f088828d6c9d772643889366934fac))
    - scope docs for method builders ([`182d0c6`](https://github.com/Byron/google-apis-rs/commit/182d0c6facbc80cf30c072abd930aa15a1898123))
    - ground work for upload media ([`020300a`](https://github.com/Byron/google-apis-rs/commit/020300af15022124cfa0d3e1722d45ff371f924d))
    - fixed spacing ([`a7f93a9`](https://github.com/Byron/google-apis-rs/commit/a7f93a93b62a908f470cc0de1164551786d1b96a))
    - improved spacing ([`0ff1e07`](https://github.com/Byron/google-apis-rs/commit/0ff1e07e534e33d0815676270c90109a0195ff82))
    - examples section in mbuilder got lost ([`4bdee96`](https://github.com/Byron/google-apis-rs/commit/4bdee961d19fc6fc6cb3cf322dfb85d2769bbcee))
    - filter request value props by parts ([`fad0a71`](https://github.com/Byron/google-apis-rs/commit/fad0a7177aa296aa777b45d0001effa36332d24e))
    - added info about settable parts ([`42ae75c`](https://github.com/Byron/google-apis-rs/commit/42ae75c1a1a2bfa148a6c52884c88ac71bcf93c0))
    - into own def ([`a2550d1`](https://github.com/Byron/google-apis-rs/commit/a2550d11811de9f9ee51652975363d0f24b8d032))
    - more information, nicer visuals ([`4e8872b`](https://github.com/Byron/google-apis-rs/commit/4e8872b37af5bbefcbec6db8f9192d0fbf180eeb))
    - method builder examples work now ([`a3206ab`](https://github.com/Byron/google-apis-rs/commit/a3206abc92d7bc9d829a1e2e00dbd299c379f2ab))
    - have to handle required/optionals vals ([`9cbb2ad`](https://github.com/Byron/google-apis-rs/commit/9cbb2adc5a65bece45e524a71f2d66160f7aa133))
    - method builder call example ([`bfa20a1`](https://github.com/Byron/google-apis-rs/commit/bfa20a18c8138ddd7c76a2fcdeb43d40bc884b3d))
    - remove empty '/// # ' lines ([`f2dda42`](https://github.com/Byron/google-apis-rs/commit/f2dda421e64e9164557d5b3b94604bcb2be49254))
    - methods useful for mbuild as too ([`331ecf8`](https://github.com/Byron/google-apis-rs/commit/331ecf87a76189b10672770377d36877dbd7f53a))
    - fixed part handling,it compiles now ([`70ea612`](https://github.com/Byron/google-apis-rs/commit/70ea612f19fbe7e1ef0a01b0d399fb357a46c390))
    - request type handling part 1 ([`48d40d4`](https://github.com/Byron/google-apis-rs/commit/48d40d45c5ee2b8dce689eb0a0457e0364246899))
    - setters now copy copyables ([`452b658`](https://github.com/Byron/google-apis-rs/commit/452b658c27e265c6a2df90ea56502db338957154))
    - build insert/update ... methods ([`693b5c8`](https://github.com/Byron/google-apis-rs/commit/693b5c8f6a556941fcbfaf6b58f0d0dd00053a66))
    - new _setter method ([`1dc1684`](https://github.com/Byron/google-apis-rs/commit/1dc168497ee180fa3728be290c65535ba16117e2))
    - properties and setters for mbuilder ([`582aca3`](https://github.com/Byron/google-apis-rs/commit/582aca32494bf938889b04c60c5d3cec81872f77))
    - infrastructure for method builders ([`942cbe1`](https://github.com/Byron/google-apis-rs/commit/942cbe18f1f237fe8efacde93fd121879924d619))
    - move resource builder into own lib ([`f1b99af`](https://github.com/Byron/google-apis-rs/commit/f1b99af5dcca4e169463a8932fcf217f9cace8c6))
    - Partial MethodBuilder impl ([`01db890`](https://github.com/Byron/google-apis-rs/commit/01db89057deca47d86355e35c86b4fb88c218db0))
    - using visual markers now ([`8746f5e`](https://github.com/Byron/google-apis-rs/commit/8746f5e0e20297ce58203da01638fafad155132c))
    - defs are now more readable ([`e96260b`](https://github.com/Byron/google-apis-rs/commit/e96260bacc959aee2d3baa1353d48087637f3df9))
    - generate hub implementation and docs ([`615a124`](https://github.com/Byron/google-apis-rs/commit/615a12465415cfa155271ce2fb94be9faa7405db))
    - Merge branch 'schema' ([`85171ce`](https://github.com/Byron/google-apis-rs/commit/85171ceb97e8999808b1c087e37c8ea39ea42abd))
    - library overview as far as possible ([`74aa7bb`](https://github.com/Byron/google-apis-rs/commit/74aa7bba2d9e5e5375d15ee2500e385d4b33415b))
    - def for DO NOT EDIT comments ([`f1d9582`](https://github.com/Byron/google-apis-rs/commit/f1d95822f784bce84927c2a9d4134d5477495217))
    - Traits now show up as part of lib ([`e164cf7`](https://github.com/Byron/google-apis-rs/commit/e164cf73667a6b64908a1dd41c5adf91191a5237))
    - perfected trait recognition. ([`8dc5e2a`](https://github.com/Byron/google-apis-rs/commit/8dc5e2a53dbe4d620e97089e2af9e3a94a82a4a4))
    - add marker traits to schema types ([`c1eeee0`](https://github.com/Byron/google-apis-rs/commit/c1eeee0591f96e2865db1ed13900ba7b59475ac9))
    - now the map is complete ([`f4030f0`](https://github.com/Byron/google-apis-rs/commit/f4030f02841521220fa52856fa733b828a59ab6b))
    - LUTs and context to make better docs ([`ba98bee`](https://github.com/Byron/google-apis-rs/commit/ba98bee62fa2e067e9bc18f6f52db8be1da35161))
    - dependency handling:dirs with timestamp ([`bb04b60`](https://github.com/Byron/google-apis-rs/commit/bb04b60dc405d74765161bc75e35b4de72c5dcc4))
    - first generated result ... ([`d8edf1d`](https://github.com/Byron/google-apis-rs/commit/d8edf1dcd46c6f7ae27e6f61b8aa1dea071a44a0))
    - make all pods optionals. ([`ddb48a4`](https://github.com/Byron/google-apis-rs/commit/ddb48a4303a7a0653898e9eea69b3d358a14fa0c))
    - now docs look good too ([`49c2ffb`](https://github.com/Byron/google-apis-rs/commit/49c2ffb8e0f02698657aba46a7b34981258c6e35))
    - generating valid rust from schemas ([`a5e675e`](https://github.com/Byron/google-apis-rs/commit/a5e675e7a958327938a31ec38ddebfaf58af9f42))
    - Merge branch 'project-template' ([`939e631`](https://github.com/Byron/google-apis-rs/commit/939e631edb2b36e390720d35ae9b75b75caac417))
    - now sets up entire project structure ([`475163e`](https://github.com/Byron/google-apis-rs/commit/475163ec29e5d20e74141de76f38b88a51bfbd06))
    - unify generated constants ([`317554a`](https://github.com/Byron/google-apis-rs/commit/317554aff398a823beae63fa09a6014ee1508f4b))
    - improved license information ([`fc15a70`](https://github.com/Byron/google-apis-rs/commit/fc15a7030f81658663ff416a86880bfde01f23f0))
    - LICENSE + README.md ([`3670e4f`](https://github.com/Byron/google-apis-rs/commit/3670e4f6c98d1b04a618fa9c14d5470a7a6765b7))
    - mako-render generates output dirs ([`4e5f2c0`](https://github.com/Byron/google-apis-rs/commit/4e5f2c05d93dd2f4cbf7472a8911fbd7e0463d9d))
    - mv youtube-rs to google-apis-rs ([`11b6fe2`](https://github.com/Byron/google-apis-rs/commit/11b6fe212ff33c1b2378997411cb11524d73a81c))
    - apis target - make all apis ([`e3b6aee`](https://github.com/Byron/google-apis-rs/commit/e3b6aee6d631c589cb277b999583aa460631c34d))
    - can now use custom libraries in pycode ([`2298601`](https://github.com/Byron/google-apis-rs/commit/2298601165f5b65f76c86f4542139965c2486e58))
    - cargo.toml template ([`be93825`](https://github.com/Byron/google-apis-rs/commit/be938255bd14202cc77c6bc543c6e92060a7ccb0))
    - generic source/output mappings ([`2d77857`](https://github.com/Byron/google-apis-rs/commit/2d77857aaf9b6a7e1a5dc7a3f77349a3662f8c7c))
    - multiple input-outputs per call ([`087a076`](https://github.com/Byron/google-apis-rs/commit/087a0762ac936f40bc4cec6f2281db34d9cab95b))
    - handle whitespace and add GENINFO ([`c3d399e`](https://github.com/Byron/google-apis-rs/commit/c3d399e91a6fea7a09316f018865815214a14be8))
    - Merge branch 'mako-templates' ([`d79f041`](https://github.com/Byron/google-apis-rs/commit/d79f041e3016c1728d848faf09efafad409f9c28))
    - api deps generation works ([`30041e9`](https://github.com/Byron/google-apis-rs/commit/30041e9c7da099c4843cd987ff34349394d8613d))
    - mako autosetup and improved executable ([`20410ad`](https://github.com/Byron/google-apis-rs/commit/20410adb786a1f35e870b38fc3b5b3140b626708))
    - Merge branch 'template-engine' ([`7b38b4c`](https://github.com/Byron/google-apis-rs/commit/7b38b4c12f47b9a04fb525821bd16b5884f13a70))
    - successfully generating make deps ([`c0bfeab`](https://github.com/Byron/google-apis-rs/commit/c0bfeabbc39cd7449f59c8e1fd1fe9e5abba315a))
    - is now self-contained ([`179c64c`](https://github.com/Byron/google-apis-rs/commit/179c64c5e74c7a783a3dc4ef68e900440e587c83))
    - removed gsl, added pyratemp ([`e06738a`](https://github.com/Byron/google-apis-rs/commit/e06738a7bd49538d402f8c995710cf231d47221d))
    - fixed dependencies ([`f2ca8c3`](https://github.com/Byron/google-apis-rs/commit/f2ca8c3fb79e482ca39d3aeb40be9b8c7f9c58d8))
    - forgot to add shared.xml ([`e081017`](https://github.com/Byron/google-apis-rs/commit/e081017cb3631df007937fe4bce09c554e8c58c0))
    - my first gsl program ... ([`0812068`](https://github.com/Byron/google-apis-rs/commit/0812068c905463c10352ac194f44c9a317352647))
    - unified make based build system ([`0c2f149`](https://github.com/Byron/google-apis-rs/commit/0c2f149b1e168497a376ce48105fa4d4089612e6))
    - added authenticator arg ([`f13c296`](https://github.com/Byron/google-apis-rs/commit/f13c2960ab8b3441a32bde892a8ee53f8497b987))
    - Merge branch 'gsl-setup' ([`6fe9a5d`](https://github.com/Byron/google-apis-rs/commit/6fe9a5d8bfb2e07d9eb83960d6d6e8df5ab242f7))
    - makefile for handling json-to-xml ([`1980f76`](https://github.com/Byron/google-apis-rs/commit/1980f76c3240b44c306158df30793ca20ffc9461))
    - works exactly as needed. ([`e83b063`](https://github.com/Byron/google-apis-rs/commit/e83b063f0527d7e5253f14a22c90fd3b4197584a))
    - xml.tostring works now ... ([`e0724fb`](https://github.com/Byron/google-apis-rs/commit/e0724fb56f4a49fc5da4d6b5ea75dd1029ee9a44))
    - make it handle top-level keys ([`143aa6f`](https://github.com/Byron/google-apis-rs/commit/143aa6fd8638b3541d71954c6e3493bc961813dd))
    - add conversion tool and youtube api ([`eebcf54`](https://github.com/Byron/google-apis-rs/commit/eebcf549295fe5b0521092bd0c79d83c416d351d))
    - first primitive types and api ([`aaf432f`](https://github.com/Byron/google-apis-rs/commit/aaf432fb545b47a64692dda0296414edbf3017b6))
    - make sure we get correct openssl vers. ([`d4869cf`](https://github.com/Byron/google-apis-rs/commit/d4869cfefc58db4580e98e8dd1ae040c81083ba9))
    - improved module layout ([`24a727f`](https://github.com/Byron/google-apis-rs/commit/24a727fdea7c2ae47dd23b7ff571cd717ec4d870))
    - figure out ownership model ([`67b052c`](https://github.com/Byron/google-apis-rs/commit/67b052c5f376c85ceb2f3e94e676e4906df9fd10))
    - initial commit ([`dda8476`](https://github.com/Byron/google-apis-rs/commit/dda847607fc88ab6bb6d9646d52cd9795f7af0b3))
</details>

