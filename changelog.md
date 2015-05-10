<a name="cli-v0.2.0"></a>

## cli-v0.2.0 (2015-05-10)

I proudly present, the first release [you can actually use][youtube-workflow]!

#### Features

* **[Clap][clap] - The Argument Parser Users will *Love* !**
  * Thanks to [clap][clap], using the command-line interface is so much more comfortable and helpful.
    In addition to that, it will provide wonderful usage information and help texts to get
    you going fast without stying the online-manual all the time.
  * Special thanks to [@kbknapp][kbknapp] for all the great support. It was my pleasure evolving my CLIs alongside [clap][clap].

#### Improvements
  * **Usage-to-Manual Backlinks**
    * Sometimes even the extended usage, e.g. `youtube3 videos insert --help`, is not enough. Now a 
      URL to the exact method manual is provided as well to help filling in [complex information][youtube3-example].
  * ***(More)* Human JSON Output**
    * Previously obtained JSON information would contain `null` values, adding unwanted noise. These are now 
      filtered out. Thanks to [@erickt][erickt] [for the hint][json-value-null-filtering].
    * *Did you know ...* that you can use [JQ][jq-homepage] to filter the JSON output and extract data ?

#### Bug Fixes
  * **Out-of-memory during Uploads**
    * When uploading anything over SSL (the default for Google Services), due to an [issue in rust-openssl][openssl-blocker-bug]
      all data would first be cached in-memory before sending it with ulta-high CPU usage. Now that the aforementioned 
      bug is fixed, you can easily saturate a 1Gb link for encrypted uploads.
  * **YouTube uploads didn't work**
    * Any YouTube upload was rejected as the server didn't want to see `null` values within the request structure. Thanks
      to [additional filtering][json-tools] an optimized JSON stream is produced, much to the joy of said server.


<a name="api-v0.1.7"></a>
## api-v0.1.7 (2015-05-10)


#### Improvements

* **CLI**  remove null in pretty-printed json ([5894c816](https://github.com/Byron/google-apis-rs/commit/5894c8163afa9f9d9bed592e7e41912c77cf993d), closes [#102](https://github.com/Byron/google-apis-rs/issues/102))
* **index.html**  DL title contains os-name ([69b12104](https://github.com/Byron/google-apis-rs/commit/69b12104a9f9579773553825f63c321e7d1a6899))
* **API**
  *  improved display of BadRequest ([e86e55ca](https://github.com/Byron/google-apis-rs/commit/e86e55cae788506a2280816009b8620bad091477), closes [#103](https://github.com/Byron/google-apis-rs/issues/103))
  *  faster null-value removal ([26314e74](https://github.com/Byron/google-apis-rs/commit/26314e743e2c4f38eb6c5824bf51209099000f9f))

#### Features

* **clap**
  *  implement -u as good as possible ([656fcae2](https://github.com/Byron/google-apis-rs/commit/656fcae2b481ac90254bf5e3081d2bbd659d5232))
  *  parse structure and build App ([db4624b4](https://github.com/Byron/google-apis-rs/commit/db4624b46728379393372be40b1ce731fe8f28b4), closes [#87](https://github.com/Byron/google-apis-rs/issues/87))
  *  initial version of command generation ([b39bc3a9](https://github.com/Byron/google-apis-rs/commit/b39bc3a9cd165db8f9ea3fa536697ca80d36628e))
  *  setup infrastructure ([988d37f0](https://github.com/Byron/google-apis-rs/commit/988d37f0dfaf8a1725bf92364e965c1f32e6802f))
* **CLI**
  *  did you mean for struct values ([96415d17](https://github.com/Byron/google-apis-rs/commit/96415d17ca383ba0653fb4df23df1ebe27d57f55), closes [#67](https://github.com/Byron/google-apis-rs/issues/67))
  *  `-u <mode> <file>` parsing ([75b80de3](https://github.com/Byron/google-apis-rs/commit/75b80de3c644a1487358561810c7c56bad8cca1d), closes [#92](https://github.com/Byron/google-apis-rs/issues/92))
  *  adjust to serde usage in `yup-oauth` ([894b5b5e](https://github.com/Byron/google-apis-rs/commit/894b5b5ec7bf7cb027ba31bf83c40f27e0ab51bd), closes [#93](https://github.com/Byron/google-apis-rs/issues/93))
* **index.html**
  * added download links (osx,ubuntu) ([52027c6d](https://github.com/Byron/google-apis-rs/commit/52027c6db59c2952f61ee03204fd947277d0cc62), closes [#106](https://github.com/Byron/google-apis-rs/issues/106))
  *  added back-link to crates.io ([0e6605d7](https://github.com/Byron/google-apis-rs/commit/0e6605d7a4ee59e16d52fd93e037b5608fd5f61f), closes [#105](https://github.com/Byron/google-apis-rs/issues/105))
* **deploy**
  *  simple linux deployment script ([36513f10](https://github.com/Byron/google-apis-rs/commit/36513f101e0c3299513fe1bf542c7fc7c492e771))
  *  simple osx deploy script ([c2483019](https://github.com/Byron/google-apis-rs/commit/c248301951cc1266136e2ab7b6c6f5cc54d86164))

#### Bug Fixes

* **CLI**
  *  completed list of parameter names ([9274938f](https://github.com/Byron/google-apis-rs/commit/9274938f9f69ecab2e8cb975467860f41466ad1d))
  *  added latest reference CLI code ([d2a4e2ff](https://github.com/Byron/google-apis-rs/commit/d2a4e2ff8b16cb848869cc07b6c5a9107fb0a929))
  *  gate usage of `upload_media_params` ([89432cc6](https://github.com/Byron/google-apis-rs/commit/89432cc64600ba0711e412c6cf6b1e06e2f11102))
  *  handle repeated required strings ([bf6a2ba6](https://github.com/Byron/google-apis-rs/commit/bf6a2ba60c364e7c30de198d335e481c0b3206f0), closes [#96](https://github.com/Byron/google-apis-rs/issues/96))
  *  'about()' text for main commands ([153324eb](https://github.com/Byron/google-apis-rs/commit/153324ebccf8a7846d9669f16c8f3ea52f0ec810), closes [#95](https://github.com/Byron/google-apis-rs/issues/95))
  *  adjust `JsonTokenStorage` to yup-oauth ([94c821e0](https://github.com/Byron/google-apis-rs/commit/94c821e09d2b75756dd3dfa2d5f508b079413cf1))
  *  unified error handling ([2f200217](https://github.com/Byron/google-apis-rs/commit/2f200217f942aa0317186811dbbe95d675a17ab0), closes [#66](https://github.com/Byron/google-apis-rs/issues/66))
  *  escape subcommand descriptions ([fac50418](https://github.com/Byron/google-apis-rs/commit/fac50418a7156b1b2fa958008691dbb2f6cbb756))
* **API**
  *  filter null values of requrest structs ([3efa4f2b](https://github.com/Byron/google-apis-rs/commit/3efa4f2b12219412cdabf8535e03974b94f71af5))
  *  simplified call to form_urlencode ([b27c990d](https://github.com/Byron/google-apis-rs/commit/b27c990db8a8701e2814e77136a34689be56c623))
  *  let delegate forget uploaded urls ([c346645f](https://github.com/Byron/google-apis-rs/commit/c346645fc96abf9831ce723bb56e26f95e3c5b45), closes [#85](https://github.com/Byron/google-apis-rs/issues/85))
  *  remove unused std_misc feature ([d46c0839](https://github.com/Byron/google-apis-rs/commit/d46c083975201a6a4804fde9d4cec6ae0fc29479))
  *  adjust to latest hyper header macros ([4115d50c](https://github.com/Byron/google-apis-rs/commit/4115d50ca795ec2a2958f5f75b7681cb9f84720b))
  *  exclude cloudsearch from build ([de85fb43](https://github.com/Byron/google-apis-rs/commit/de85fb43e53723d1d38d0b6e8746acc962035233))
* **compat**
  *  upgrade to hyper v0.4.0 ([3fe2732a](https://github.com/Byron/google-apis-rs/commit/3fe2732a01371ededca2c35fe7499a4bbe63c318))
  *  make it work with latest hyper ([57808cf9](https://github.com/Byron/google-apis-rs/commit/57808cf92adf7ff4dd65664a4a4ed3a361b60c6e))
* **clap**
  * re-introduce UploadProtocol,fix CallType ([d0ce221b](https://github.com/Byron/google-apis-rs/commit/d0ce221ba39db621b969b8c1faad358c775502a5), closes [#81](https://github.com/Byron/google-apis-rs/issues/81))
  *  update docs and fix calltype handling ([b039b382](https://github.com/Byron/google-apis-rs/commit/b039b382446f450a58c12d2d881dbcd00b96928a))
  *  various fixes and improvements ([7a38f7e4](https://github.com/Byron/google-apis-rs/commit/7a38f7e4d5dea97b5bd2cbe6b10e4619b3b45b12))
  *  print usage if command is missing ([63e23dd4](https://github.com/Byron/google-apis-rs/commit/63e23dd48f7fb80268eb3bc95380b77b233de62a))
  *  tweaks to make youtube3 work ([5320a48e](https://github.com/Byron/google-apis-rs/commit/5320a48e68c0ee4457455c5caa5c01f322fc6c7e))
  *  adjust option usage to changed API ([bac4e1a8](https://github.com/Byron/google-apis-rs/commit/bac4e1a82fa331370c20a7c4843989f11974600c))
  *  handle apis without media upload ([feaa3a06](https://github.com/Byron/google-apis-rs/commit/feaa3a06ed53ae039750e2d420817116b1140984))
  *  call `iter()` directly ([02a41296](https://github.com/Byron/google-apis-rs/commit/02a41296628eb0cbc0c8b7b2e86b06678e8db084))
  *  commit before un-using UploadProtocol ([1aff3135](https://github.com/Byron/google-apis-rs/commit/1aff3135d97435632599bf39cf5e8c5de9d773a8))
  *  generate command data structure ([8ac8d3b1](https://github.com/Byron/google-apis-rs/commit/8ac8d3b1cb59249d492a657fa8cd39fbe3fd99a7))
  *  upload some code to help debugging ([9a8ae4b7](https://github.com/Byron/google-apis-rs/commit/9a8ae4b7d66ec1b6a74316fceeccbf04a2f77469))



<a name="api-v0.1.6"></a>
## api-v0.1.6 (2015-05-02)


#### Bug Fixes

* **rustup**  (abf0548b5 2015-04-15) (built 2015-04-15) ([9ea3fea7](https://github.com/Byron/google-apis-rs/commit/9ea3fea7750bce93c531f99b13c747c78a806b59))
* **make-deps**  fix dependencies ([2e74d914](https://github.com/Byron/google-apis-rs/commit/2e74d9141313da1cc6a26149650ee59c43047f06))
* **make**  dependencies are now per-program-type ([acd42dfc](https://github.com/Byron/google-apis-rs/commit/acd42dfccc87f49cf5c9bf51a206da8bed9c02c2))
* **api-version-up**  v0.1.4 ([6f2149b7](https://github.com/Byron/google-apis-rs/commit/6f2149b7d49ee693cc616b92f9de79f220ce6e2d))
* **lib**  update changed `url` crate imports ([607ba745](https://github.com/Byron/google-apis-rs/commit/607ba745d140e5d2567a715c6ddaa775d2cf0d99))
* **token-storage**  implement deletion of tokens ([6d84ef90](https://github.com/Byron/google-apis-rs/commit/6d84ef906e6b9ff344fd7acac3140bdad3d48e78), closes [#79](https://github.com/Byron/google-apis-rs/issues/79))
* **cli**  add commands.yml.mako ([51ddcf74](https://github.com/Byron/google-apis-rs/commit/51ddcf74a6d1cf204156c6a018ced2f2d85c9352))
* **hyper-client**  adjust to hyper client ([191e822c](https://github.com/Byron/google-apis-rs/commit/191e822c5a93771e32e85bc5c00ef450c6719fb6), closes [#47](https://github.com/Byron/google-apis-rs/issues/47))
* **docker**  README info + fix author email ([e7302810](https://github.com/Byron/google-apis-rs/commit/e730281003b4a4caad0d48c2712e5d1433848bd7), closes [#71](https://github.com/Byron/google-apis-rs/issues/71))
* **docs**
  *  corrected cursor handling in mkdocs ([bf37e515](https://github.com/Byron/google-apis-rs/commit/bf37e515d2b5affec6296c34fbfa68fa89f7d4b9))
  *  one folder per API docs ([6d3dc776](https://github.com/Byron/google-apis-rs/commit/6d3dc77635724602a89026477bfc0f8f785968ba), closes [#48](https://github.com/Byron/google-apis-rs/issues/48))
* **README+deps**  better subtext + rename target ([75e73d56](https://github.com/Byron/google-apis-rs/commit/75e73d56d95dc4126ef39f0ae60d901a32af9954))
* **cmn**  use bytes=... when sending as well ([b9a469c0](https://github.com/Byron/google-apis-rs/commit/b9a469c0a4e655da54940dc2876559f573c88c08))
* **test**  add rustc_serialize to test-crate ([fa011315](https://github.com/Byron/google-apis-rs/commit/fa011315c31815cf283ecff18e245553378f3cb9))
* **api-update**  'bytes ...' -> 'bytes=...' ([3e0a24db](https://github.com/Byron/google-apis-rs/commit/3e0a24db0d8d25fec9457364d49106c22aee3c23))
* **travis**
  *  update make target ([a4b73cc1](https://github.com/Byron/google-apis-rs/commit/a4b73cc1c4e3919cf8bf2f782d598d0840c4922f))
  *  adjust invalid make target ([9dbdcc46](https://github.com/Byron/google-apis-rs/commit/9dbdcc465f45c13faa85e5489073e7b7f5e18133))
* **README**  corrected absolute links ([34d0a7aa](https://github.com/Byron/google-apis-rs/commit/34d0a7aad3b139c71b4d0dd7ca4e10c1336ebb8f))
* **CLI**
  *  request value cursor handling and docs ([b6a48bdc](https://github.com/Byron/google-apis-rs/commit/b6a48bdcd5fb215e94a00a69d11ce0ac007c2df3), closes [#86](https://github.com/Byron/google-apis-rs/issues/86))
  *  simple and resumable upload works ([2f3b2d24](https://github.com/Byron/google-apis-rs/commit/2f3b2d24ce2367356698b902becabb40b8636ab6))
  *  use only one request structure ([0bb30da7](https://github.com/Byron/google-apis-rs/commit/0bb30da78244abcf09c7d04571515e6584ccb4a3))
  *  set request value to call ([be7ccb08](https://github.com/Byron/google-apis-rs/commit/be7ccb085cb5ea908fb75d0ae7cb6c91ded33bd4))
  *  verified download works ([6befdbc6](https://github.com/Byron/google-apis-rs/commit/6befdbc6fa730fc4a5513d2cad9e1784c580e516), closes [#75](https://github.com/Byron/google-apis-rs/issues/75))
  *  response value json decoding ([845a568b](https://github.com/Byron/google-apis-rs/commit/845a568b25f387c58a17752852aed63e7305c7b1), closes [#73](https://github.com/Byron/google-apis-rs/issues/73))
  *  resolve generator issues ([797f2898](https://github.com/Byron/google-apis-rs/commit/797f289886d899a7e1b21216ee46218d179e38bf), closes [#77](https://github.com/Byron/google-apis-rs/issues/77))
  *  request value parsing compiles and inits ([fa278a99](https://github.com/Byron/google-apis-rs/commit/fa278a99c769e99727176f4faae081cc2d219342), closes [#64](https://github.com/Byron/google-apis-rs/issues/64))
  *  struct access compiles ... ([bf22bef7](https://github.com/Byron/google-apis-rs/commit/bf22bef77ae62d06209c70d273ecccef29a4268a))
  *  NULL default values instead of randoms ([4b87d909](https://github.com/Byron/google-apis-rs/commit/4b87d909f21daff696dd81da463fae3b14e59725))
  *  alt-media handling in CLI+API-docs ([306852d5](https://github.com/Byron/google-apis-rs/commit/306852d5147d7083ff011f990c5feedcf3e338bb), closes [#61](https://github.com/Byron/google-apis-rs/issues/61))
  *  optional parameter default handling ([830529c4](https://github.com/Byron/google-apis-rs/commit/830529c40b6ab01381fe36f27753047a2b03244f))
  *  optimze argument handling and conversion ([76841da0](https://github.com/Byron/google-apis-rs/commit/76841da09801f23abef4955d76430ce1191c0b77), closes [#65](https://github.com/Byron/google-apis-rs/issues/65))
  *  Display for Errors + refactor ([e45eb053](https://github.com/Byron/google-apis-rs/commit/e45eb053d52db016342bd568d10bc368495dad86), closes [#54](https://github.com/Byron/google-apis-rs/issues/54))
* **version-up**
  *  code updated to v0.1.6, latest CLI ([c2dd9c7a](https://github.com/Byron/google-apis-rs/commit/c2dd9c7a020e0367bc87b20fa8054c85f48b71c1))
  *  CLI + API release preps ([4e275eaa](https://github.com/Byron/google-apis-rs/commit/4e275eaaddfd7a86ed42d04df24113015c6ea099))
  *  add publish state v0.1.5 ([6db73327](https://github.com/Byron/google-apis-rs/commit/6db733274d65f10a213612561a5771bf4b7b8316))
  *  v0.1.5 ([a399488c](https://github.com/Byron/google-apis-rs/commit/a399488c2799e1acca0961f80a6c116a3330190c))
* **all**  update all code to latest version ([f8689be4](https://github.com/Byron/google-apis-rs/commit/f8689be4515d5693004da17bb2244a385ac1e794))
* **API**
  *  adapt to changed yup-oauth2 API ([e523ddb6](https://github.com/Byron/google-apis-rs/commit/e523ddb6ec9f1e9e8bcc51fbec02e364dbddaa72))
  *  scopes were used illegally ([d8fdf9df](https://github.com/Byron/google-apis-rs/commit/d8fdf9df9f41719f6acb9bf3750aa8069cfab675))
* **checkin**  latest version of all APIs ([4cf0720e](https://github.com/Byron/google-apis-rs/commit/4cf0720ef1e025737416ad5fe07eff2389c86ad8))
* **api+cli**  improved scope handling; fix CLI ([5b4f18d3](https://github.com/Byron/google-apis-rs/commit/5b4f18d341cbd8f87d3e3792b1dfa803f7849015))

#### Features

* **engine**  infrastructure ([ca8e8c06](https://github.com/Byron/google-apis-rs/commit/ca8e8c06220f858424c8c1b799b1f00bd89e9bb2), closes [#52](https://github.com/Byron/google-apis-rs/issues/52))
* **CLI**
  *  per-API-credentials with default ([e42f6fbe](https://github.com/Byron/google-apis-rs/commit/e42f6fbedb0a2e609c6d1363a5c0eaa5b7967863), closes [#80](https://github.com/Byron/google-apis-rs/issues/80))
  *  hashmap handling ([b830c1c6](https://github.com/Byron/google-apis-rs/commit/b830c1c6decea4d5b3a16712b31daaa544cc837b), closes [#68](https://github.com/Byron/google-apis-rs/issues/68))
  *  repeated required args ([c14ef9af](https://github.com/Byron/google-apis-rs/commit/c14ef9afc86a17b5bc3952882f98fc7bf7a2ced8))
  *  --debug-auth flag ([03f35bd4](https://github.com/Byron/google-apis-rs/commit/03f35bd4f547da5843fab755ca678c01800aabed), closes [#70](https://github.com/Byron/google-apis-rs/issues/70))
  *  --debug flag to output traffix ([159c6591](https://github.com/Byron/google-apis-rs/commit/159c65916f0fb4d0136a8cc622919daf60a7ecfd))
  *  added first versions of all CLI ([f5f12c55](https://github.com/Byron/google-apis-rs/commit/f5f12c559448f73a08a812f4ac40bfc6dafcbabb))
  *  struct value parsing ([15b78cd1](https://github.com/Byron/google-apis-rs/commit/15b78cd1ff148a20006e92fd9210e93f01d9f366))
  *  field cursor complete and untested ([1dd1fcf4](https://github.com/Byron/google-apis-rs/commit/1dd1fcf4b80e9554bac430326fa668b18cd9c678))
  *  make respective uppload_call ([6119bfb7](https://github.com/Byron/google-apis-rs/commit/6119bfb7627c7e238a5641b0781bfca3689e8a36), closes [#62](https://github.com/Byron/google-apis-rs/issues/62))
  *  upload flag parsing ([9eed4056](https://github.com/Byron/google-apis-rs/commit/9eed4056e53d71f2b8165fd4099fda6fc0d0798a))
  *  global optional parameters+DL tracking ([36a7cb23](https://github.com/Byron/google-apis-rs/commit/36a7cb239a2717b54500ed41a346a382b092f76a))
  *  parse method parameters and set them ([6ae6ee88](https://github.com/Byron/google-apis-rs/commit/6ae6ee88a05d8d8c76f69c4bff2c37684b3d81ad))
  * handle output json encoding and ostreams ([3f49f50a](https://github.com/Byron/google-apis-rs/commit/3f49f50ac21fb921b61c1170c633214782f39cc7), closes [#63](https://github.com/Byron/google-apis-rs/issues/63))
  *  interpret output arguments ([c3a9f1e8](https://github.com/Byron/google-apis-rs/commit/c3a9f1e8e594172ac783f0b9c76e093a534674ee))
  * required arg parsing + first doit() call ([e34e24e0](https://github.com/Byron/google-apis-rs/commit/e34e24e04943e6cce8564295587bbf426c58077f), closes [#60](https://github.com/Byron/google-apis-rs/issues/60))
  *  infrastructure for call and dry-run ([d6919f1e](https://github.com/Byron/google-apis-rs/commit/d6919f1eb65c7e29527360739555fce4a254d9e8), closes [#59](https://github.com/Byron/google-apis-rs/issues/59))
  *  Implementation of JsonTokenStorage ([8afc76a7](https://github.com/Byron/google-apis-rs/commit/8afc76a7fe50ba8171f1e2045d989162c9864395), closes [#58](https://github.com/Byron/google-apis-rs/issues/58))
  *  init hub + refactor for dry-run mode ([f71c2862](https://github.com/Byron/google-apis-rs/commit/f71c2862851f98c00fb893fa3b940a912b893845), closes [#57](https://github.com/Byron/google-apis-rs/issues/57))
  *  engine checks resource and method args ([be228f19](https://github.com/Byron/google-apis-rs/commit/be228f19940d38e484809116c1bd84bb8edf5ee8), closes [#55](https://github.com/Byron/google-apis-rs/issues/55))
  * write default and read app-secret ([4548644c](https://github.com/Byron/google-apis-rs/commit/4548644cb1498f4c7769d8e98cc7ddf8c0e4f47b), closes [#53](https://github.com/Byron/google-apis-rs/issues/53))
* **cli**
  *  generate complete docopts grammar ([310c81f1](https://github.com/Byron/google-apis-rs/commit/310c81f19cbfb8e1fc7d7f3766492c002a340761))
  *  docopt subcommands ([39253d98](https://github.com/Byron/google-apis-rs/commit/39253d988af3d7795b2167edb3a54b8988dda00c))
  *  bin renaming + docopt infrastructure ([f527c820](https://github.com/Byron/google-apis-rs/commit/f527c8202b961d3dcb4c30a13e3c28a650fb144c))
  *  basic usage of docopts ([390354bd](https://github.com/Byron/google-apis-rs/commit/390354bd08b429fb438d60c54e2a36756e086c3c))
* **config**  create config directory, if possible ([5799d44f](https://github.com/Byron/google-apis-rs/commit/5799d44fceb537f8f82ae4919682c9189a172792))
* **mkdocs**
  *  per-method-markdown-files ([3cef120c](https://github.com/Byron/google-apis-rs/commit/3cef120c58d304e120ba5e86a1717f1c47452452))
  *  cli postprocessing support ([c78ea538](https://github.com/Byron/google-apis-rs/commit/c78ea5381aeeb7c97ce4fc35e0c9da40a7022423))
* **API**
  *  improved error handling ([a3289420](https://github.com/Byron/google-apis-rs/commit/a3289420337c7f607f4393fcf0832167267cc473), closes [#82](https://github.com/Byron/google-apis-rs/issues/82))
  *  Display + Error traits for Error struct ([7dc99724](https://github.com/Byron/google-apis-rs/commit/7dc9972445593f592f369759b9839a3dedf8d12c), closes [#56](https://github.com/Byron/google-apis-rs/issues/56))



<a name="api-v0.1.4"></a>
## api-v0.1.4 (2015-05-02)


#### Bug Fixes

* **publish**
  *  v0.1.4 ([dd1d1919](https://github.com/Byron/google-apis-rs/commit/dd1d191966aa41ec66c5a4baba5ebd43771c3a05))
  *  v0.1.3 ([3403bd1c](https://github.com/Byron/google-apis-rs/commit/3403bd1c5cec379cd2ad98040cca0ec6a4eef4a3))



<a name="api-v0.1.3"></a>
## api-v0.1.3 (2015-05-02)


#### Bug Fixes

* **deps**  github-pages index generation ([919ae4d8](https://github.com/Byron/google-apis-rs/commit/919ae4d8ae85f35f54c69c8c222ba43ba304e263))
* **version-up**  check-in of latest sources ([a2ca1cb2](https://github.com/Byron/google-apis-rs/commit/a2ca1cb28ec1ce9f5f381f55ea78aa59a56ea915))
* **api-version-up**  version 0.1.3 ([99f8b65f](https://github.com/Byron/google-apis-rs/commit/99f8b65f75822d54f32100655d0b5678f43a8478), closes [#46](https://github.com/Byron/google-apis-rs/issues/46))
* **rustup**  rustc (be9bd7c93 2015-04-05) ([91861dcb](https://github.com/Byron/google-apis-rs/commit/91861dcb71b371e8ec5511ddedee0ae45cee9af0))



<a name="api-v0.1.2"></a>
## api-v0.1.2 (2015-05-02)


#### Bug Fixes

* **docs**
  *  remove newlines interpreted as test ([d1c5bf1e](https://github.com/Byron/google-apis-rs/commit/d1c5bf1e4ab2a91c30d2bcbd1e08a1a02c73ad41))
  *  typo fixes and misc. improvements ([ea161897](https://github.com/Byron/google-apis-rs/commit/ea161897f5fe25e024292755c753f2410211bea1), closes [#42](https://github.com/Byron/google-apis-rs/issues/42))
* **json-up**  update json files from discovery API ([fca1b24c](https://github.com/Byron/google-apis-rs/commit/fca1b24cd186b090f75e35f362c8bbb2754e3e4d))
* **versionup**
  *  set the API version to 0.1.2 ([c7fb7c40](https://github.com/Byron/google-apis-rs/commit/c7fb7c409343f19e26f1c3d488718decec7990b0))
  * incl. `Result` conform to standards ([e9535354](https://github.com/Byron/google-apis-rs/commit/e953535473429b01293d679e23337b74645e0c18))
* **result**  remove custom Result Enum ([e5b013e9](https://github.com/Byron/google-apis-rs/commit/e5b013e97c56040dba266a43a8308448a32645eb), closes [#39](https://github.com/Byron/google-apis-rs/issues/39))
* **misc**  whitespace and trait rename ([6ad0c2ef](https://github.com/Byron/google-apis-rs/commit/6ad0c2ef79a634d4cb631a36eb92b2cf82b59121))

#### Features

* **make**  cli depends on API, generically ([cefd606b](https://github.com/Byron/google-apis-rs/commit/cefd606b538ed86d7b659f83b64ee2b14f71fc3b), closes [#11](https://github.com/Byron/google-apis-rs/issues/11))
* **mkdocs**  mkdocs generator works now ([d1c97912](https://github.com/Byron/google-apis-rs/commit/d1c97912cbebf8df3f2817b04b15a78d952b092a), closes [#43](https://github.com/Byron/google-apis-rs/issues/43))
* **api**  api generation works once again ([be7d8214](https://github.com/Byron/google-apis-rs/commit/be7d8214c16287fb245918c38561544245a0aa1d))



<a name="api-v0.1.1"></a>
## api-v0.1.1 (2015-05-02)


#### Bug Fixes

* **mbuild**  upload size now taken properly ([04f4c956](https://github.com/Byron/google-apis-rs/commit/04f4c95688f2cef0866ce07da68ae9d710596c7c))



<a name="api-v0.1.0"></a>
## api-v0.1.0 (2015-05-02)


#### Bug Fixes

* **doit**
  *  fix lifetime issues ([29d9e45c](https://github.com/Byron/google-apis-rs/commit/29d9e45c9fc8bbdbed23d3d5a9be20f8023bb22d))
  *  repeated params string addition ([b90a1916](https://github.com/Byron/google-apis-rs/commit/b90a1916889b2d1cc6c595c3cd121739223db345))
  *  remove BorrowMut until it's cleared ([1349c786](https://github.com/Byron/google-apis-rs/commit/1349c786b7e986511e4c2ca058d45bebb7f458dd))
* **to_version**  assured it handles '0' correctly ([4b9dbb28](https://github.com/Byron/google-apis-rs/commit/4b9dbb28ff474661855f53143862b621e650f157), closes [#3](https://github.com/Byron/google-apis-rs/issues/3))
* **readme**
  * improved markdown for library overview ([97da926e](https://github.com/Byron/google-apis-rs/commit/97da926e28d7ad7ed90d12b7ff48477bcf67ee68))
  *  added milestone link ([6167dc07](https://github.com/Byron/google-apis-rs/commit/6167dc07fc63cec22a8d2b01fe69f05f03ac3f9a))
* **scope**
  *  make scope gen work with gmail ([3b7e63f2](https://github.com/Byron/google-apis-rs/commit/3b7e63f28675ea2646c88dfa16c62c063e076b96))
  *  scopes are sorted Strings now ([6d2b0fc2](https://github.com/Byron/google-apis-rs/commit/6d2b0fc2649bc5203c07c29dd020b50550d15746))
* **activities**
  *  fully qualified activity names ([8006bb8c](https://github.com/Byron/google-apis-rs/commit/8006bb8ca910b14ece8dee6230d476a361c7c163), closes [#7](https://github.com/Byron/google-apis-rs/issues/7))
  *  now the map is complete ([f4030f02](https://github.com/Byron/google-apis-rs/commit/f4030f02841521220fa52856fa733b828a59ab6b))
* **pyratemp**  is now self-contained ([179c64c5](https://github.com/Byron/google-apis-rs/commit/179c64c5e74c7a783a3dc4ef68e900440e587c83))
* **cmn**
  *  upload() return value handling ([cd1ff18b](https://github.com/Byron/google-apis-rs/commit/cd1ff18ba94966088a779b26347dc683f1f0c2d3), closes [#18](https://github.com/Byron/google-apis-rs/issues/18))
  *  serde cleanup;JsonError pub fields ([b9a81a90](https://github.com/Byron/google-apis-rs/commit/b9a81a900ec054b102ce045cf25a4348c297f260))
* **json2xml**
  *  works exactly as needed. ([e83b063f](https://github.com/Byron/google-apis-rs/commit/e83b063f0527d7e5253f14a22c90fd3b4197584a))
  *  xml.tostring works now ... ([e0724fb5](https://github.com/Byron/google-apis-rs/commit/e0724fb56f4a49fc5da4d6b5ea75dd1029ee9a44))
  *  make it handle top-level keys ([143aa6fd](https://github.com/Byron/google-apis-rs/commit/143aa6fd8638b3541d71954c6e3493bc961813dd))
* **docs**
  *  re-export types used by delegate ([556906ca](https://github.com/Byron/google-apis-rs/commit/556906ca60a90fc6eb34917d42813daf9792fbcb))
  * better introduction and version handling ([3a9aa519](https://github.com/Byron/google-apis-rs/commit/3a9aa519496be9da6283b847f38d9a2deaf682aa))
  *  pretty names for methods and resources ([0152138e](https://github.com/Byron/google-apis-rs/commit/0152138e0c019575caa3e40f87f19382d92a63ac))
  *  repeated parameters docs improvement ([863a98c0](https://github.com/Byron/google-apis-rs/commit/863a98c0d7932475dc207d204ec91c26ddec326c))
  *  filter request value props by parts ([fad0a717](https://github.com/Byron/google-apis-rs/commit/fad0a7177aa296aa777b45d0001effa36332d24e))
  *  have to handle required/optionals vals ([9cbb2adc](https://github.com/Byron/google-apis-rs/commit/9cbb2adc5a65bece45e524a71f2d66160f7aa133))
  *  remove empty '/// # ' lines ([f2dda421](https://github.com/Byron/google-apis-rs/commit/f2dda421e64e9164557d5b3b94604bcb2be49254))
* **schema**
  *  no unused types anymore ([e3ab233a](https://github.com/Byron/google-apis-rs/commit/e3ab233a6cee8482c1c98b1e2c759e7a17cceab9))
  *  improved nested array type handling ([dfcd554f](https://github.com/Byron/google-apis-rs/commit/dfcd554faa36cbcdf18ab985c2aed744dd45dc6d))
  *  now deals with non-objects ([50fa189a](https://github.com/Byron/google-apis-rs/commit/50fa189a715332a7ce49fc7a9c95e5a1ef22b81f))
  *  make all pods optionals. ([ddb48a43](https://github.com/Byron/google-apis-rs/commit/ddb48a4303a7a0653898e9eea69b3d358a14fa0c))
  *  now docs look good too ([49c2ffb8](https://github.com/Byron/google-apis-rs/commit/49c2ffb8e0f02698657aba46a7b34981258c6e35))
* **resources**  first recursive resource support ([35bd1c3e](https://github.com/Byron/google-apis-rs/commit/35bd1c3e9c8a6ab52068e279d8f925eea8af055d))
* **version-up**  0.1.0 release ([3bc930ae](https://github.com/Byron/google-apis-rs/commit/3bc930ae47c2544de4825ecec5346f53626a75e2))
* **names**  nested type names are consistent now ([32145e64](https://github.com/Byron/google-apis-rs/commit/32145e645ea29ff43c451530906356564e12f817))
* **mako**
  *  fix name clashes ([d99ba9c5](https://github.com/Byron/google-apis-rs/commit/d99ba9c5b3c5f73ad148679a866698c811eec495))
  *  deal with missing auth information ([df9f0299](https://github.com/Byron/google-apis-rs/commit/df9f0299bf5db0b7affdd90b4dfb331c74f543f2))
  *  unify generated constants ([317554af](https://github.com/Byron/google-apis-rs/commit/317554aff398a823beae63fa09a6014ee1508f4b))
* **libdocs**  asssure candidate is in mapping ([1e332ddb](https://github.com/Byron/google-apis-rs/commit/1e332ddb91540c19586e6d85869c8e54c47552b0))
* **api-versions**  ignore beta/alpha,assure latest ([ff5cbb3b](https://github.com/Byron/google-apis-rs/commit/ff5cbb3bf410276fbe5af8cc966ac363e448970c))
* **type-clashes**  protect from nested-type-clash ([614539a9](https://github.com/Byron/google-apis-rs/commit/614539a925c5e64508fa28506b1c6db3ccd96882))
* **builders**  fixed part handling,it compiles now ([70ea612f](https://github.com/Byron/google-apis-rs/commit/70ea612f19fbe7e1ef0a01b0d399fb357a46c390))
* **rename**  mv youtube-rs to google-apis-rs ([11b6fe21](https://github.com/Byron/google-apis-rs/commit/11b6fe212ff33c1b2378997411cb11524d73a81c))
* **hash**  nested type resolution and hashes ([5d563c88](https://github.com/Byron/google-apis-rs/commit/5d563c88a8e3ccb33ebe381b47beb6ecfd4444fc))
* **test**
  *  unit-tests work once again ([91f69ffd](https://github.com/Byron/google-apis-rs/commit/91f69ffd6ed85790d8b6d1c8b5b63d7f4c7e6259))
  *  method builder examples work now ([a3206abc](https://github.com/Byron/google-apis-rs/commit/a3206abc92d7bc9d829a1e2e00dbd299c379f2ab))
* **apis**
  *  exclude those with recursive schemas ([5ff22851](https://github.com/Byron/google-apis-rs/commit/5ff22851faec165258e5c3ff9c6eed58df3efee3))
  *  intermediate improvements ... ([92d8fa76](https://github.com/Byron/google-apis-rs/commit/92d8fa76d0f419738e2efa7df3deebb974c1e0cf))
* **util**
  *  deepcopy dicts instead ([efe56ad2](https://github.com/Byron/google-apis-rs/commit/efe56ad25081b632f1e65fd8292e9c4d535659bc))
  *  resource-to-category map ([c7e169df](https://github.com/Byron/google-apis-rs/commit/c7e169dff3712ff5f73497d2d9cba3303a83277a))
  *  do not degenerate during activity_split ([7816cc81](https://github.com/Byron/google-apis-rs/commit/7816cc81455c1c7a48e84289e176baf25e8480e2))
* **types**
  * prune unused and ToParts trait ([80161f72](https://github.com/Byron/google-apis-rs/commit/80161f72be1aa7f7551603c90752793c84eedb6d), closes [#35](https://github.com/Byron/google-apis-rs/issues/35))
  *  make recursive types possible ([8d9f175f](https://github.com/Byron/google-apis-rs/commit/8d9f175f917ec19e4752c5c3806f6f5624e066e2))
  *  prevent type-clash with `Result` ([b6ebb1ec](https://github.com/Byron/google-apis-rs/commit/b6ebb1ec371c833ef7386264ed9522b880586316))
  *  fix incorrect nested type names ([4f794ef5](https://github.com/Byron/google-apis-rs/commit/4f794ef5ff7b5a068a568056d2bfd7372ec9b57c))
  *  prevent struct recursion issue ([da575055](https://github.com/Byron/google-apis-rs/commit/da57505567a58b59f320016d92b50f1ea248067c))
  *  nested types work for arrays ([54540e69](https://github.com/Byron/google-apis-rs/commit/54540e695a9b246ca3d412ab62e843e4dd7974d0))
  *  optionals are working once again ([a268be27](https://github.com/Byron/google-apis-rs/commit/a268be27d2123a77259fa1d7d1f831c7e72c4459))
* **typename**  improved camelCasing ([de40a8bd](https://github.com/Byron/google-apis-rs/commit/de40a8bd1ee8759287cd2a489cc5d995c296a07e))
* **mbuild**
  *  simplification and cleanup ([4bf28007](https://github.com/Byron/google-apis-rs/commit/4bf280079ed5cf33c4ed2617c3aa62151ec0dcd0))
  *  scope -> add_scope ([538120f7](https://github.com/Byron/google-apis-rs/commit/538120f7d1425e026220211857658a775c958577))
  *  added size and mime type support ([baea071a](https://github.com/Byron/google-apis-rs/commit/baea071a6f1c52410c0ca79cf24ab325f6efa586))
  *  doit() call with enum type annotation ([6fad7600](https://github.com/Byron/google-apis-rs/commit/6fad7600a03f2f6a3964f309fc8e277b34f8aa60))
  *  setters now copy copyables ([452b658c](https://github.com/Byron/google-apis-rs/commit/452b658c27e265c6a2df90ea56502db338957154))
* **markup**  examples section in mbuilder got lost ([4bdee961](https://github.com/Byron/google-apis-rs/commit/4bdee961d19fc6fc6cb3cf322dfb85d2769bbcee))
* **common**
  *  MultiPartReader test case ([6b230135](https://github.com/Byron/google-apis-rs/commit/6b2301351f6792fb37b7dfec6c1f0592fdc6b9cc))
  *  MultiPartReader now works correctly ([e53e23a8](https://github.com/Byron/google-apis-rs/commit/e53e23a893ce6d59777b8b53f94770d5c3c86b9c))
* **visuals**  using visual markers now ([8746f5e0](https://github.com/Byron/google-apis-rs/commit/8746f5e0e20297ce58203da01638fafad155132c))
* **mako-deps**  handle whitespace and add GENINFO ([c3d399e9](https://github.com/Byron/google-apis-rs/commit/c3d399e91a6fea7a09316f018865815214a14be8))
* **xml**  forgot to add shared.xml ([e081017c](https://github.com/Byron/google-apis-rs/commit/e081017cb3631df007937fe4bce09c554e8c58c0))
* **json**
  *  just add latest youtube code ([ff385e5c](https://github.com/Byron/google-apis-rs/commit/ff385e5cacb43d173912243fc033578b0c0b0f63))
  *  Vec/HashMap are Optionals ([cfb8faef](https://github.com/Byron/google-apis-rs/commit/cfb8faefb8545114ddadea59871214b35e515d5a))
* **#5**  use function to make links correctly ([c8061ebe](https://github.com/Byron/google-apis-rs/commit/c8061ebe2fbe97274c68b7af6e5a8d08c0245139))
* **cosmetics**  nicer code and identifiers ([9b308bb6](https://github.com/Byron/google-apis-rs/commit/9b308bb6ddebe979abca6f46da131c822f95c639))
* **methods**  decent solution for free methods ([79879daf](https://github.com/Byron/google-apis-rs/commit/79879daf1b2a52593d2bc9b51ba244bfaddcf1f0), closes [#19](https://github.com/Byron/google-apis-rs/issues/19))
* **lib** schema_markers() accessed map incorrectly ([98f4bbab](https://github.com/Byron/google-apis-rs/commit/98f4bbab4774fb166936c60cbe8eee2302f35052))
* **makefile**  regenerate .api.deps less often ([63997910](https://github.com/Byron/google-apis-rs/commit/63997910decf909a8242a8a7f16f6a4c276e1d67))
* **build**
  *  remove compiler warnings. ([559cb8fe](https://github.com/Byron/google-apis-rs/commit/559cb8fe458e18fec05d0ca3cd2847fb981f2da0))
  *  fixes to help more projects to build ([cf258bf4](https://github.com/Byron/google-apis-rs/commit/cf258bf4e5148723940cc757ec032b5aff814f1e))
* **compile**  no compiler warnings ([bfc39229](https://github.com/Byron/google-apis-rs/commit/bfc392291666a40cf3fbe4db3dfeda69d23018fa))
* **make**
  *  make 'regen-apis' work ([97b26490](https://github.com/Byron/google-apis-rs/commit/97b2649094cc225d0cfc42857140f0d245e11352))
  *  dependency handling:dirs with timestamp ([bb04b60d](https://github.com/Byron/google-apis-rs/commit/bb04b60dc405d74765161bc75e35b4de72c5dcc4))
  *  fixed dependencies ([f2ca8c3f](https://github.com/Byron/google-apis-rs/commit/f2ca8c3fb79e482ca39d3aeb40be9b8c7f9c58d8))
* **travis**
  *  incorrectly capitalized cargo.toml ([31efbf4f](https://github.com/Byron/google-apis-rs/commit/31efbf4fb0033b9f1fdfae0054ece1717ec05b79))
  *  explicit subshell for cargo-doc ([4c657ac9](https://github.com/Byron/google-apis-rs/commit/4c657ac9d132257a392bfbf2ed861142b6baf36a))
  *  try using a subshell for cargo cmd ([a87fbdf0](https://github.com/Byron/google-apis-rs/commit/a87fbdf0a86cfa410c79671aee931e3bf95fab11))
  *  install virtualenv automatically ([5fd7cb51](https://github.com/Byron/google-apis-rs/commit/5fd7cb511407de7176dc07c1443ef07075c063a4))
  *  Do not generate docs ! ([b43eb0e3](https://github.com/Byron/google-apis-rs/commit/b43eb0e301c068500777fe580c1bd1017d0819b1))
* **Scope**  Manual scope parameter ... ([28878e06](https://github.com/Byron/google-apis-rs/commit/28878e0618cbb5632a1353ceb2048a913e9355d2))
* **template-engine**  removed gsl, added pyratemp ([e06738a7](https://github.com/Byron/google-apis-rs/commit/e06738a7bd49538d402f8c995710cf231d47221d))
* **doc-links**  some links pointed to old doc name ([a05426e7](https://github.com/Byron/google-apis-rs/commit/a05426e79b8c0773dbb219b327539431e4d1fdfc))
* **name**  MethodBuilder -> CallBuilder ([10dfeeb1](https://github.com/Byron/google-apis-rs/commit/10dfeeb1aa5a1de2919e9753444e8e63855d1285))
* **cargo**
  *  repository/source-code link ([030c40d2](https://github.com/Byron/google-apis-rs/commit/030c40d2699196e29d1c8606d042403df52a7534))
  *  make sure we get correct openssl vers. ([d4869cfe](https://github.com/Byron/google-apis-rs/commit/d4869cfefc58db4580e98e8dd1ae040c81083ba9))
* **traits**
  *  finally, we pick up all types ([7e243936](https://github.com/Byron/google-apis-rs/commit/7e243936f226f6e26d2b551765b62cddc866776b))
  *  transitive, minimal traits for types ([00de2b18](https://github.com/Byron/google-apis-rs/commit/00de2b187d74fd78f049a13d1517fc91d218da71))
  *  perfected trait recognition. ([8dc5e2a5](https://github.com/Byron/google-apis-rs/commit/8dc5e2a53dbe4d620e97089e2af9e3a94a82a4a4))
* **delegate**  it now works in every which way ([1423e462](https://github.com/Byron/google-apis-rs/commit/1423e46210d95d823ff9bee9896cf407b0e9f0cc))
* **nestedtypes**  recursion for nested types ([0d9f6363](https://github.com/Byron/google-apis-rs/commit/0d9f6363eb271f95624559b06cfd07ab6b5bc9b5))
* **resource**  now with flattened activities ([2531011f](https://github.com/Byron/google-apis-rs/commit/2531011fc579df4edc38b15de459c135975fa077))
* **dev**  typo ([7758f99f](https://github.com/Byron/google-apis-rs/commit/7758f99ff2e19c3518eddcfca2e1adeee12e0659))
* **lib-name** user lower-case library names,always ([814c9c9f](https://github.com/Byron/google-apis-rs/commit/814c9c9ffab64a7607f4056fbad4203ea8f19991))
* **deps**  fixed dependency to wrong target ([51d05d6d](https://github.com/Byron/google-apis-rs/commit/51d05d6db01edb4f78159c3c07d77d0aceb85b89))
* **Makefile**  force python2.7 in virtualenv ([876772cf](https://github.com/Byron/google-apis-rs/commit/876772cf2296c4b7c80c2f828e245c903da67802))

#### Features

* **names**  improved library names ([b8956103](https://github.com/Byron/google-apis-rs/commit/b8956103d9460c73956dbc28ca2f1684ba8b853c))
* **result**  generic result type ([da300e03](https://github.com/Byron/google-apis-rs/commit/da300e035ebc92728c5566071c26505a38b409f6))
* **types**
  *  mark unused types with marker trait ([8bb2166d](https://github.com/Byron/google-apis-rs/commit/8bb2166da0a11db45a68e53518e94119b6d5a3b3))
  *  prevent duplicate schema types ([3a154303](https://github.com/Byron/google-apis-rs/commit/3a1543033949b8f25e2e3cd888c9f43029b4de3d), closes [#26](https://github.com/Byron/google-apis-rs/issues/26))
* **docs**
  *  add more obvious crate and api version ([79cbf3ee](https://github.com/Byron/google-apis-rs/commit/79cbf3ee3fccdbfadcb1176ebc319f8bbabb8b68), closes [#16](https://github.com/Byron/google-apis-rs/issues/16))
  *  add cargo.toml dependency information ([7f33cf22](https://github.com/Byron/google-apis-rs/commit/7f33cf22a5c22e3cc50dcc199604af78ba8e13fa), closes [#10](https://github.com/Byron/google-apis-rs/issues/10))
  *  full usage example on landing page ([9a17ab9e](https://github.com/Byron/google-apis-rs/commit/9a17ab9e4e98d8797a9912d3d5094c0e2bf9716f))
  *  Traits now show up as part of lib ([e164cf73](https://github.com/Byron/google-apis-rs/commit/e164cf73667a6b64908a1dd41c5adf91191a5237))
* **downloads**  alt 'media' handling to allow dls ([02d7a06f](https://github.com/Byron/google-apis-rs/commit/02d7a06fdff10d54c93d00fa18e0330e1f536162), closes [#21](https://github.com/Byron/google-apis-rs/issues/21))
* **doit**
  *  don't crash if json decode fails. ([0823dec7](https://github.com/Byron/google-apis-rs/commit/0823dec75cc89b8e0a87a41ab2dcd1d5a405a24e), closes [#33](https://github.com/Byron/google-apis-rs/issues/33))
  *  simplify delegate calls ([265b4482](https://github.com/Byron/google-apis-rs/commit/265b448297493afe11c38ac751376c67907e84da), closes [#30](https://github.com/Byron/google-apis-rs/issues/30))
  *  optimizations and simplification; seek ([9d401f54](https://github.com/Byron/google-apis-rs/commit/9d401f5486b447ea0fc43cb0d4bb84fac3329357), closes [#17](https://github.com/Byron/google-apis-rs/issues/17))
  * optimized memory allocation and options ([224af640](https://github.com/Byron/google-apis-rs/commit/224af64068c60649266aff7cc06abd001053015b))
  *  initial part writing ([71c827b3](https://github.com/Byron/google-apis-rs/commit/71c827b3067131a150bfd4a3503a61b836ec39b5))
  *  multi-part mime-type and add_parts() ([fc589cb9](https://github.com/Byron/google-apis-rs/commit/fc589cb965848332dd944a790cafd7d4745d9fc7))
  *  handle 'alt' param ([3ea5e194](https://github.com/Byron/google-apis-rs/commit/3ea5e194859749e05632edcfd35cc21db8cf53ff), closes [#20](https://github.com/Byron/google-apis-rs/issues/20))
  *  more multipart infrastructure ([b0a1f518](https://github.com/Byron/google-apis-rs/commit/b0a1f518e957c96a0f5b5b2297a738cb42032e87))
  *  improve body infrastructure ([7cfb5afd](https://github.com/Byron/google-apis-rs/commit/7cfb5afd394041019899ca4cdcf10c9187204409))
  *  simplify URL_ENCODE handling ([d2bf24ca](https://github.com/Byron/google-apis-rs/commit/d2bf24ca859b945e1f5ee64dc5ccdf7357d01184))
  *  uri-template handling complete ([1fee21de](https://github.com/Byron/google-apis-rs/commit/1fee21de24eee4fd62151595ef7915987f7a39db))
  *  uri-template generation works ([54eb784a](https://github.com/Byron/google-apis-rs/commit/54eb784a550a619b3773e44fc2ddd0b2a58ffcd2))
  *  repeated types in examples ([64219e7e](https://github.com/Byron/google-apis-rs/commit/64219e7e7eed42f7491a2aba80f5e8fd7567385e))
  *  repeatable parameters working ([d758f410](https://github.com/Byron/google-apis-rs/commit/d758f410f68b84cb635a6a0633bb09b147939397))
  *  partial implementation of url expr ([35437070](https://github.com/Byron/google-apis-rs/commit/354370705dd317b9839cf9a6ad34e22b9efe12dc))
  *  set upload media type ([33e85ddd](https://github.com/Byron/google-apis-rs/commit/33e85ddd29db5a75ce49718d850652c36ad7ce25))
  *  pre-request delegate call. ([60adacf8](https://github.com/Byron/google-apis-rs/commit/60adacf8d47eb43a0f82642a69c5216e79285dbc))
  *  json decode and delegation ([eef14713](https://github.com/Byron/google-apis-rs/commit/eef1471357e7a16f7501575bcca1d17cddf05515))
  *  authentication with and without scopes ([2c79f6e3](https://github.com/Byron/google-apis-rs/commit/2c79f6e3cfbf7044a061eef1ddfb6fadac19401d))
  *  attempt to send json-encoded request ([9a58b0ba](https://github.com/Byron/google-apis-rs/commit/9a58b0badd0fea4220cccb953f6deb00c8edbaaa))
  *  query string setup ([aabed385](https://github.com/Byron/google-apis-rs/commit/aabed3858143bcd28d4b95e3831c408d3120719b))
* **schema**
  *  support for 'variant' schema ([bb75c5b6](https://github.com/Byron/google-apis-rs/commit/bb75c5b69871ec88c888618d0c3292741c9cffff))
  *  generating valid rust from schemas ([a5e675e7](https://github.com/Byron/google-apis-rs/commit/a5e675e7a958327938a31ec38ddebfaf58af9f42))
* **deps**  update-json using discovery API ([c0a24760](https://github.com/Byron/google-apis-rs/commit/c0a247605890be6553fa4709074b4c4ca4a199a9))
* **hub**
  *  allow to set user-agent ([cb5a0a35](https://github.com/Byron/google-apis-rs/commit/cb5a0a35bc36cbf234e2ac5d2cec0b2c14ac1d2f), closes [#24](https://github.com/Byron/google-apis-rs/issues/24))
  *  generate hub implementation and docs ([615a1246](https://github.com/Byron/google-apis-rs/commit/615a12465415cfa155271ce2fb94be9faa7405db))
* **all-apis**  build all apis, were possible ([2d036b66](https://github.com/Byron/google-apis-rs/commit/2d036b6623a6f21e7d5706b382e2bc1e28dac87c))
* **common**
  *  multibytereader single byte test ([b127df17](https://github.com/Byron/google-apis-rs/commit/b127df17b02a4823e74a5125961bdfa23f77f7a0))
  *  MultiPartReader is working. ([8db346b8](https://github.com/Byron/google-apis-rs/commit/8db346b8b01f003fed24d202822c398fa0994443))
* **mako-render**  multiple input-outputs per call ([087a0762](https://github.com/Byron/google-apis-rs/commit/087a0762ac936f40bc4cec6f2281db34d9cab95b))
* **params**  additional fields and Result type ([7c6f7d5e](https://github.com/Byron/google-apis-rs/commit/7c6f7d5e97344e7df0f397c65209795e5b8515bc))
* **dlg**  make actual `store_upload_url()` call ([ffef7dda](https://github.com/Byron/google-apis-rs/commit/ffef7dda57c8f3f14d86712107416eaffe4c1bfc))
* **dev**  spike to see how delegate can be work ([432faa27](https://github.com/Byron/google-apis-rs/commit/432faa275f89bb1c3ab00b60ff07225eec5a4489))
* **name**  oauth22 -> oauth2_v2 ([664d8225](https://github.com/Byron/google-apis-rs/commit/664d8225d2d5275148395828af02c0bc54b7ee24))
* **visuals**  defs are now more readable ([e96260ba](https://github.com/Byron/google-apis-rs/commit/e96260bacc959aee2d3baa1353d48087637f3df9))
* **builder**  Partial MethodBuilder impl ([01db8905](https://github.com/Byron/google-apis-rs/commit/01db89057deca47d86355e35c86b4fb88c218db0))
* **videos**  first primitive types and api ([aaf432fb](https://github.com/Byron/google-apis-rs/commit/aaf432fb545b47a64692dda0296414edbf3017b6))
* **gsl**  my first gsl program ... ([0812068c](https://github.com/Byron/google-apis-rs/commit/0812068c905463c10352ac194f44c9a317352647))
* **mbuild**
  *  use of oauth2::Scheme ([d26cf774](https://github.com/Byron/google-apis-rs/commit/d26cf7740614134e97f1b6add19c3b91242fc994))
  *  check upload size against max-size ([57e0f065](https://github.com/Byron/google-apis-rs/commit/57e0f0658379db524f1a964232a3fa39111be626), closes [#37](https://github.com/Byron/google-apis-rs/issues/37))
  *  improved delegate calls ([9ea85273](https://github.com/Byron/google-apis-rs/commit/9ea85273cd18798c7f0c523a45de1f25c0648c92))
  *  resumable-upload infrastructure ([307d3f48](https://github.com/Byron/google-apis-rs/commit/307d3f487c6b35f42be643505a4e65c6ce04e6ec))
  *  scope as property ... ([e1b7a63f](https://github.com/Byron/google-apis-rs/commit/e1b7a63f0660682a1680d9651cd5c3e784b12030))
  *  media-upload doit() methods ([5b2d8a77](https://github.com/Byron/google-apis-rs/commit/5b2d8a77a3cf17a1c5989e856b1ae2dc77613264))
  *  `param()` to set any parameter ([de0c7a4a](https://github.com/Byron/google-apis-rs/commit/de0c7a4ae049b6f7fbc256d64bc363ebd8de2101))
  *  infrastructure for method builders ([942cbe18](https://github.com/Byron/google-apis-rs/commit/942cbe18f1f237fe8efacde93fd121879924d619))
* **setters**  properties and setters for mbuilder ([582aca32](https://github.com/Byron/google-apis-rs/commit/582aca32494bf938889b04c60c5d3cec81872f77))
* **architecture**  figure out ownership model ([67b052c5](https://github.com/Byron/google-apis-rs/commit/67b052c5f376c85ceb2f3e94e676e4906df9fd10))
* **youtube**  first generated result ... ([d8edf1dc](https://github.com/Byron/google-apis-rs/commit/d8edf1dcd46c6f7ae27e6f61b8aa1dea071a44a0))
* **type-params**  ground work for upload media ([020300af](https://github.com/Byron/google-apis-rs/commit/020300af15022124cfa0d3e1722d45ff371f924d))
* **service**  added authenticator arg ([f13c2960](https://github.com/Byron/google-apis-rs/commit/f13c2960ab8b3441a32bde892a8ee53f8497b987))
* **builders**  request type handling part 1 ([48d40d45](https://github.com/Byron/google-apis-rs/commit/48d40d45c5ee2b8dce689eb0a0457e0364246899))
* **doc**  def for DO NOT EDIT comments ([f1d95822](https://github.com/Byron/google-apis-rs/commit/f1d95822f784bce84927c2a9d4134d5477495217))
* **xmlconv**  add conversion tool and youtube api ([eebcf549](https://github.com/Byron/google-apis-rs/commit/eebcf549295fe5b0521092bd0c79d83c416d351d))
* **travis**  docs and tests of youtube3 on travis ([dd0772f1](https://github.com/Byron/google-apis-rs/commit/dd0772f1d7e1330229bb36040686f91e088befd2))
* **lib**
  *  use serge instead of serialize ([d3bb130b](https://github.com/Byron/google-apis-rs/commit/d3bb130be0b25f984c75ab125d2b344929865213))
  *  new Scope enum type ([bb76832b](https://github.com/Byron/google-apis-rs/commit/bb76832b2f317501d398f5ea9fe8ea6b12dacf7b))
* **mako**
  *  now sets up entire project structure ([475163ec](https://github.com/Byron/google-apis-rs/commit/475163ec29e5d20e74141de76f38b88a51bfbd06))
  *  LICENSE + README.md ([3670e4f6](https://github.com/Byron/google-apis-rs/commit/3670e4f6c98d1b04a618fa9c14d5470a7a6765b7))
  *  mako-render generates output dirs ([4e5f2c05](https://github.com/Byron/google-apis-rs/commit/4e5f2c05d93dd2f4cbf7472a8911fbd7e0463d9d))
  *  can now use custom libraries in pycode ([22986011](https://github.com/Byron/google-apis-rs/commit/2298601165f5b65f76c86f4542139965c2486e58))
  *  cargo.toml template ([be938255](https://github.com/Byron/google-apis-rs/commit/be938255bd14202cc77c6bc543c6e92060a7ccb0))
  *  generic source/output mappings ([2d77857a](https://github.com/Byron/google-apis-rs/commit/2d77857aaf9b6a7e1a5dc7a3f77349a3662f8c7c))
  *  api deps generation works ([30041e9c](https://github.com/Byron/google-apis-rs/commit/30041e9c7da099c4843cd987ff34349394d8613d))
  *  mako autosetup and improved executable ([20410adb](https://github.com/Byron/google-apis-rs/commit/20410adb786a1f35e870b38fc3b5b3140b626708))
* **make**
  *  apis target - make all apis ([e3b6aee6](https://github.com/Byron/google-apis-rs/commit/e3b6aee6d631c589cb277b999583aa460631c34d))
  *  unified make based build system ([0c2f149b](https://github.com/Byron/google-apis-rs/commit/0c2f149b1e168497a376ce48105fa4d4089612e6))
  *  makefile for handling json-to-xml ([1980f76c](https://github.com/Byron/google-apis-rs/commit/1980f76c3240b44c306158df30793ca20ffc9461))
* **traits**  add marker traits to schema types ([c1eeee05](https://github.com/Byron/google-apis-rs/commit/c1eeee0591f96e2865db1ed13900ba7b59475ac9))
* **delegate**
  *  begin()/finished() calls ([508d14ea](https://github.com/Byron/google-apis-rs/commit/508d14eafbca167f9801a2ca7ff9a1ae922be734), closes [#25](https://github.com/Byron/google-apis-rs/issues/25))
  *  first attempt to get it to work ([678b6929](https://github.com/Byron/google-apis-rs/commit/678b6929ca7bffb4e4495272330aac02a082dbcd))
* **fields**  put all fields onto a list ([6c416609](https://github.com/Byron/google-apis-rs/commit/6c4166094358fd236490239d12235a80b738f34f))
* **drive**  added gogole drive API ([66f3ae14](https://github.com/Byron/google-apis-rs/commit/66f3ae14e5f088828d6c9d772643889366934fac))
* **schemas**  now we pre-generate nested schemas ([ac8c4153](https://github.com/Byron/google-apis-rs/commit/ac8c41530d082203f93d81851682d02ed5c98d9a))
* **lookup**  LUTs and context to make better docs ([ba98bee6](https://github.com/Byron/google-apis-rs/commit/ba98bee62fa2e067e9bc18f6f52db8be1da35161))
* **json**
  *  Option<_> in schema only if needed ([55978ff9](https://github.com/Byron/google-apis-rs/commit/55978ff9a2fe332c5ed46476af4f921a72999e5c), closes [#32](https://github.com/Byron/google-apis-rs/issues/32))
  *  added field aliases, were needed ([9f719dd9](https://github.com/Byron/google-apis-rs/commit/9f719dd9287ee112fa6c3ebb6be64e9793da8a81))
  *  part 1 to implement 'any' type ([712fed57](https://github.com/Byron/google-apis-rs/commit/712fed578a377c27bd6153b098ee4b3244b0355e))
* **gh-pages**  new github-pages target ([f27fda8f](https://github.com/Byron/google-apis-rs/commit/f27fda8f34e084e1532f4e6528b93e156f062503))
* **layout**  improved module layout ([24a727fd](https://github.com/Byron/google-apis-rs/commit/24a727fdea7c2ae47dd23b7ff571cd717ec4d870))
* **license**  improved license information ([fc15a703](https://github.com/Byron/google-apis-rs/commit/fc15a7030f81658663ff416a86880bfde01f23f0))
* **cargo**
  *  crate version +<revision> ([8ad316bd](https://github.com/Byron/google-apis-rs/commit/8ad316bda3fd5eaa7e9a993ff1a9120e71022365), closes [#38](https://github.com/Byron/google-apis-rs/issues/38))
  *  crates with 'google-' prefix ([4a27ac7e](https://github.com/Byron/google-apis-rs/commit/4a27ac7e1d14207645915637c4817a17f10916b9), closes [#23](https://github.com/Byron/google-apis-rs/issues/23))
* **rbuild**  build insert/update ... methods ([693b5c8f](https://github.com/Byron/google-apis-rs/commit/693b5c8f6a556941fcbfaf6b58f0d0dd00053a66))
* **cmn**
  *  Resumable upload implemented ([29ee94b4](https://github.com/Byron/google-apis-rs/commit/29ee94b4c04f72d2676a98dda6632a06c5b8ba54))
  *  implement query_transfer_status() ([065753cc](https://github.com/Byron/google-apis-rs/commit/065753cc3a56227c2e87fbcc8b36121dc3bb1ab6))
  *  ContentRange header (parse and format) ([42a76e46](https://github.com/Byron/google-apis-rs/commit/42a76e465549beadd3080c36f68922d8e44fba54))
* **methods**  intermed. support for 'methods' ([60d953a3](https://github.com/Byron/google-apis-rs/commit/60d953a3428d11591954e7488bc46078d4765b1f))
* **pyratemp**  successfully generating make deps ([c0bfeabb](https://github.com/Byron/google-apis-rs/commit/c0bfeabbc39cd7449f59c8e1fd1fe9e5abba315a))


[clap]: https://github.com/kbknapp/clap-rs
[youtube3-example]: https://github.com/Byron/depot/blob/master/src/bash/upload-session_google-rs.bash#L22
[jq-homepage]: http://stedolan.github.io/jq/
[openssl-blocker-bug]: https://github.com/sfackler/rust-openssl/issues/208
[json-value-null-filtering]: https://github.com/serde-rs/serde/issues/65#issuecomment-100244937
[json-tools]: https://github.com/Byron/json-tools
[kbknapp]: https://github.com/kbknapp
[erickt]: https://github.com/erickt
[youtube-workflow]: https://youtu.be/2HiJUiHdQvw