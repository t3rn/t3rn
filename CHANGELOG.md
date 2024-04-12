# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v1.87.4-rc.0] - 2024-04-12
### :bug: Bug Fixes
- [`3388c47`](https://github.com/t3rn/t3rn/commit/3388c470d6589edc8fc64e5ab6e0cac6af08f712) - updated native token from 12 to 18 decimals *(PR [#1597](https://github.com/t3rn/t3rn/pull/1597) by [@chexware](https://github.com/chexware))*


## [v1.87.3-rc.0] - 2024-04-12
### :bug: Bug Fixes
- [`5df29af`](https://github.com/t3rn/t3rn/commit/5df29afeb738b9a43ffcaa87f1b82570d25085d6) - contract deployment works without producing OutOfGas and UnknownPrecompile errors *(PR [#1598](https://github.com/t3rn/t3rn/pull/1598) by [@chexware](https://github.com/chexware))*


## [v1.87.2-rc.0] - 2024-04-04
### :bug: Bug Fixes
- [`445569e`](https://github.com/t3rn/t3rn/commit/445569e51762b928edbf798d39f6859553424d04) - claiming EVM address with Substrate account also transfer the balance from the address *(PR [#1592](https://github.com/t3rn/t3rn/pull/1592) by [@chexware](https://github.com/chexware))*


## [v1.87.1-rc.0] - 2024-04-04
### :bug: Bug Fixes
- [`9048c01`](https://github.com/t3rn/t3rn/commit/9048c01b7fd32ec7928baed8bd9c4376381a839c) - update docs executor breakdown *(PR [#1571](https://github.com/t3rn/t3rn/pull/1571) by [@jossifelefteriadis](https://github.com/jossifelefteriadis))*
- [`b01617e`](https://github.com/t3rn/t3rn/commit/b01617e17cebdd393a3fb672c6cabad092c35449) - update link structure *(PR [#1583](https://github.com/t3rn/t3rn/pull/1583) by [@jossifelefteriadis](https://github.com/jossifelefteriadis))*
- [`4446d48`](https://github.com/t3rn/t3rn/commit/4446d4819ee7231759e2a00e281c16d24d23108c) - support  for all AssetId  values through TokensPrecompile *(PR [#1593](https://github.com/t3rn/t3rn/pull/1593) by [@chexware](https://github.com/chexware))*


## [v1.87.0-rc.0] - 2024-02-19
### :sparkles: New Features
- [`59a6a97`](https://github.com/t3rn/t3rn/commit/59a6a97601a02ba53bb5d2ef25943add0e25f8b8) - Substrate tokens ERC20 precompile,  EVM CLI, EVM to Substrate decimals conversion, Working EVM Transfers *(PR [#1563](https://github.com/t3rn/t3rn/pull/1563) by [@chexware](https://github.com/chexware))*


## [v1.86.1-rc.0] - 2024-02-06
### :bug: Bug Fixes
- [`244968c`](https://github.com/t3rn/t3rn/commit/244968c3976b75ab029a83da9cabaee3af042d8c) - update docs sidebar *(PR [#1552](https://github.com/t3rn/t3rn/pull/1552) by [@jossifelefteriadis](https://github.com/jossifelefteriadis))*


## [v1.86.0-rc.0] - 2024-01-23
### :sparkles: New Features
- [`4514ef9`](https://github.com/t3rn/t3rn/commit/4514ef9356579e0be4f27e9928154d9844711477) - enable pre-funded Metamask accounts for t0rn & t2rn  *(PR [#1550](https://github.com/t3rn/t3rn/pull/1550) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.85.0-rc.0] - 2024-01-17
### :sparkles: New Features
- [`4d20bf2`](https://github.com/t3rn/t3rn/commit/4d20bf2f9e3ff76a89f506e591f4709f7ccfc48c) - update Celestia light client to v1.0.4 *(PR [#1543](https://github.com/t3rn/t3rn/pull/1543) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.84.0-rc.0] - 2024-01-16
### :sparkles: New Features
- [`72925c8`](https://github.com/t3rn/t3rn/commit/72925c8839699f6ebcce8f10e7f6023e1ad6aa1f) - bump sepolia & ethereum light client versions; fix benchmarks  *(PR [#1540](https://github.com/t3rn/t3rn/pull/1540) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.83.1-rc.0] - 2024-01-13
### :bug: Bug Fixes
- [`6bacb0f`](https://github.com/t3rn/t3rn/commit/6bacb0ff5fd5a5cd9fa4b8bf3b74c6386877fc55) - correct typo in docs release main docs CI pipeline [skip ci] *(PR [#1535](https://github.com/t3rn/t3rn/pull/1535) by [@ahkohd](https://github.com/ahkohd))*

### :wrench: Chores
- [`2200ea2`](https://github.com/t3rn/t3rn/commit/2200ea2611032e7b920d4d4626ffccf544d579f3) - plug GRANDPA proveFinality RPC to standalone node *(PR [#1534](https://github.com/t3rn/t3rn/pull/1534) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.83.0-rc.0] - 2024-01-12
### :sparkles: New Features
- [`43fbf67`](https://github.com/t3rn/t3rn/commit/43fbf67d9871b7a16731f5ee753a9742ca3f5d5b) - add pallet-ethereum to t0rn and t2rn with EVM RPC *(PR [#1531](https://github.com/t3rn/t3rn/pull/1531) by [@MaciejBaj](https://github.com/MaciejBaj))*

### :bug: Bug Fixes
- [`699e00a`](https://github.com/t3rn/t3rn/commit/699e00a80d394102a02743a614603de5eccf2125) - all issues that hinders build success *(PR [#1530](https://github.com/t3rn/t3rn/pull/1530) by [@ahkohd](https://github.com/ahkohd))*

### :wrench: Chores
- [`caa4b66`](https://github.com/t3rn/t3rn/commit/caa4b6623dcece08db7a4b22338454876f2ab22c) - update environment variable *(PR [#1533](https://github.com/t3rn/t3rn/pull/1533) by [@ahkohd](https://github.com/ahkohd))*


## [v1.82.1-rc.0] - 2024-01-10
### :bug: Bug Fixes
- [`21ebf2f`](https://github.com/t3rn/t3rn/commit/21ebf2f4a5f7f1c224eb0e2d2b23f2293b44f1aa) - update link and title *(PR [#1516](https://github.com/t3rn/t3rn/pull/1516) by [@jossifelefteriadis](https://github.com/jossifelefteriadis))*

### :wrench: Chores
- [`fd60dfc`](https://github.com/t3rn/t3rn/commit/fd60dfc6621d93c2f299345f091ec6a65d28785f) - plug GRANDPA proveFinality RPC to standalone node *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.82.0-rc.0] - 2024-01-09
### :recycle: Refactors
- [`20c0ca1`](https://github.com/t3rn/t3rn/commit/20c0ca190e1395a8facf4a1570c3e8ed3d6b7351) - resolve, address, fix todos left across the workspace *(PR [#1515](https://github.com/t3rn/t3rn/pull/1515) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.81.3-rc.0] - 2023-12-30
### :wrench: Chores
- [`504cc38`](https://github.com/t3rn/t3rn/commit/504cc38c678147903be5e38a7075d4348c441d5b) - add t7rn release *(PR [#1509](https://github.com/t3rn/t3rn/pull/1509) by [@3h4x](https://github.com/3h4x))*


## [v1.81.2-rc.0] - 2023-12-30
### :wrench: Chores
- [`6f66fa3`](https://github.com/t3rn/t3rn/commit/6f66fa3aa745b14029867a7c7d27faefbf0dcf04) - uncomment version checking *(PR [#1484](https://github.com/t3rn/t3rn/pull/1484) by [@3h4x](https://github.com/3h4x))*


## [v1.81.1-rc.0] - 2023-12-30
### :bug: Bug Fixes
- [`aff2e85`](https://github.com/t3rn/t3rn/commit/aff2e858258dd470462aeb78a54b6e41a1301fba) - removed automated USDT XCM transfers and updated yaml formatting *(PR [#1485](https://github.com/t3rn/t3rn/pull/1485) by [@chexware](https://github.com/chexware))*
- [`607ad23`](https://github.com/t3rn/t3rn/commit/607ad232d496e6c86cbfe326dceeed8c5359dec5) - update automated XCM transactions signer to use CIRCUIT_SIGNER_KEY *(PR [#1503](https://github.com/t3rn/t3rn/pull/1503) by [@chexware](https://github.com/chexware))*


## [v1.81.0-rc.0] - 2023-12-20
### :sparkles: New Features
- [`ad81580`](https://github.com/t3rn/t3rn/commit/ad815805b48e95c624ce07308905ca823ed35510) - bump celestia light client to version updating weights *(PR [#1502](https://github.com/t3rn/t3rn/pull/1502) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.80.0-rc.0] - 2023-12-19
### :sparkles: New Features
- [`795a5fa`](https://github.com/t3rn/t3rn/commit/795a5fab403e74ae62000b017a349e1b3d547dc7) - hook Celestia light client to t0rn & t2rn runtime  *(PR [#1501](https://github.com/t3rn/t3rn/pull/1501) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.79.0-rc.0] - 2023-12-11
### :sparkles: New Features
- [`f8b0bde`](https://github.com/t3rn/t3rn/commit/f8b0bde6a5e9f648afe4b082a46432134d32a522) - added support bi-directional AccountId-H160 and AssetId-H160 mapping *(PR [#1492](https://github.com/t3rn/t3rn/pull/1492) by [@chexware](https://github.com/chexware))*

### :wrench: Chores
- [`c0b1683`](https://github.com/t3rn/t3rn/commit/c0b16834fb5a22ece4f439c6196f793134ff5492) - deploy rangers *(PR [#1493](https://github.com/t3rn/t3rn/pull/1493) by [@3h4x](https://github.com/3h4x))*


## [v1.78.0-rc.0] - 2023-12-06
### :sparkles: New Features
- [`9ae04f4`](https://github.com/t3rn/t3rn/commit/9ae04f48239ed940c9ea743e14ce0552097e7732) - enable multi optimistic & escrow orders with Dynamic Destination Deal  *(PR [#1489](https://github.com/t3rn/t3rn/pull/1489) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.77.0-rc.0] - 2023-11-30
### :sparkles: New Features
- [`d59af68`](https://github.com/t3rn/t3rn/commit/d59af68a81ffc266a59b4cfb0e1842a356b050bb) - added XCM SDK support for automated sending of RUSD  and USDT from AssetHub to t0rn *(PR [#1444](https://github.com/t3rn/t3rn/pull/1444) by [@chexware](https://github.com/chexware))*


## [v1.76.0-rc.0] - 2023-11-30
### :sparkles: New Features
- [`ef380dd`](https://github.com/t3rn/t3rn/commit/ef380dd49c5d0901af2d763aa1323801e8ee8f8d) - enable multiple blocks per slot for t0rn *(PR [#1426](https://github.com/t3rn/t3rn/pull/1426) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.75.0-rc.0] - 2023-11-25
### :sparkles: New Features
- [`b7075d4`](https://github.com/t3rn/t3rn/commit/b7075d458d0ec3d0697346674b3ea6589d2219ae) - skip light clients registration for empty init data *(PR [#1476](https://github.com/t3rn/t3rn/pull/1476) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.74.9-rc.0] - 2023-11-25
### :bug: Bug Fixes
- [`1d6b86b`](https://github.com/t3rn/t3rn/commit/1d6b86bb0be6481a545ec9ce2cd00a7e32c07ed2) - set correct weights to Vacuum & Circuit orders; mitigate state bloat *(PR [#1459](https://github.com/t3rn/t3rn/pull/1459) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.74.3-rc.0] - 2023-11-13
### :wrench: Chores
- [`fc1d6d2`](https://github.com/t3rn/t3rn/commit/fc1d6d22d975c612f21dd7de112d4ca0be57c9b7) - update t0rn specs updated and wasm  *(PR [#1463](https://github.com/t3rn/t3rn/pull/1463) by [@3h4x](https://github.com/3h4x))*


## [v1.74.2-rc.0] - 2023-11-12
### :bug: Bug Fixes
- [`5c9380b`](https://github.com/t3rn/t3rn/commit/5c9380b0a6974ef2f459bee2ffd7f18bb3d295a6) - increase specs *(PR [#1457](https://github.com/t3rn/t3rn/pull/1457) by [@3h4x](https://github.com/3h4x))*


## [v1.74.1-rc.0] - 2023-11-12
### :bug: Bug Fixes
- [`9fb63c7`](https://github.com/t3rn/t3rn/commit/9fb63c776b1f14d80589ef4522e362f6f1ab5e6b) - cli reset register light client *(PR [#1435](https://github.com/t3rn/t3rn/pull/1435) by [@gvko](https://github.com/gvko))*


## [v1.74.0-rc.0] - 2023-11-12
### :sparkles: New Features
- [`a79a5a3`](https://github.com/t3rn/t3rn/commit/a79a5a3cb908a2285f0524120debfd7c0af183b7) - connect Light Client Async API to Attesters and allow generic InfluxMessage attestations  *(PR [#1436](https://github.com/t3rn/t3rn/pull/1436) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.73.0-rc.0] - 2023-11-12
### :sparkles: New Features
- [`0a018f6`](https://github.com/t3rn/t3rn/commit/0a018f66a20d509c9bb72975b94b0a2dee621135) - t2rn *(PR [#1450](https://github.com/t3rn/t3rn/pull/1450) by [@3h4x](https://github.com/3h4x))*


## [v1.72.2-rc.0] - 2023-11-09
### :bug: Bug Fixes
- [`3abf371`](https://github.com/t3rn/t3rn/commit/3abf37168b162d4737c8dc9f5986545435d0b48c) - revert to t3rn spec name for t1rn *(PR [#1448](https://github.com/t3rn/t3rn/pull/1448) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.72.0-rc.0] - 2023-11-08
### :sparkles: New Features
- [`6600fb3`](https://github.com/t3rn/t3rn/commit/6600fb31694f783a200c581130a35b14bad9cb54) - fast writer *(PR [#1428](https://github.com/t3rn/t3rn/pull/1428) by [@3h4x](https://github.com/3h4x))*
  - :arrow_lower_right: *addresses issue [#28](undefined) opened by [@3h4x](https://github.com/3h4x)*
- [`b25badc`](https://github.com/t3rn/t3rn/commit/b25badca5dea1907bdc3ea72c31c1d92ad26df5b) - t1rn on kusama *(PR [#1445](https://github.com/t3rn/t3rn/pull/1445) by [@3h4x](https://github.com/3h4x))*

### :bug: Bug Fixes
- [`2aecaeb`](https://github.com/t3rn/t3rn/commit/2aecaeb70361749762eb12441530ae184cf7f7d0) - **fast-writer**: unrecognized exception handled *(PR [#1442](https://github.com/t3rn/t3rn/pull/1442) by [@3h4x](https://github.com/3h4x))*


## [v1.71.0-rc.0] - 2023-11-03
### :sparkles: New Features
- [`735f315`](https://github.com/t3rn/t3rn/commit/735f3151b874ecea0c8481c93f89ec4eb559a46a) - include SFX evaluation by executors for transfer assets against t0rn *(PR [#1432](https://github.com/t3rn/t3rn/pull/1432) by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`7d6e383`](https://github.com/t3rn/t3rn/commit/7d6e383fb443365ec5a2cc2d1487a63f8da4cf62) - **ci**: add automated XCM transactions sending using helm *(PR [#1430](https://github.com/t3rn/t3rn/pull/1430) by [@chexware](https://github.com/chexware))*

### :bug: Bug Fixes
- [`d58ea3c`](https://github.com/t3rn/t3rn/commit/d58ea3cee99b05391f3a9ca0ea91173a9f80c2ed) - cli reset register light client *(PR [#1429](https://github.com/t3rn/t3rn/pull/1429) by [@gvko](https://github.com/gvko))*
- [`578e08e`](https://github.com/t3rn/t3rn/commit/578e08e37d17b363a1ae97d5f7a700558b7b2d73) - correct signAndSendSafe not resolving on success *(PR [#1434](https://github.com/t3rn/t3rn/pull/1434) by [@gvko](https://github.com/gvko))*

### :white_check_mark: Tests
- [`a287cc5`](https://github.com/t3rn/t3rn/commit/a287cc513ff809345bf6b3815bfd5ef5b483d6fa) - extend mock single vacuum order to test all registered gateways *(PR [#1431](https://github.com/t3rn/t3rn/pull/1431) by [@MaciejBaj](https://github.com/MaciejBaj))*

### :wrench: Chores
- [`fb17d09`](https://github.com/t3rn/t3rn/commit/fb17d09329548dedd916fc4a49f9eeddbcba94e9) - update sepolia LC version  *(PR [#1433](https://github.com/t3rn/t3rn/pull/1433) by [@gvko](https://github.com/gvko))*


## [v1.70.2-rc.0] - 2023-11-02
### :bug: Bug Fixes
- [`63b00a5`](https://github.com/t3rn/t3rn/commit/63b00a570bdde7516125c63e844d9b5be740f068) - **grandpa-ranger**: sending limited ranges to not exceed blocksize *(PR [#1404](https://github.com/t3rn/t3rn/pull/1404) by [@coun7zero](https://github.com/coun7zero))*
- [`5952849`](https://github.com/t3rn/t3rn/commit/59528492bbafa440a3914db70e7ec8fd83425022) - **executor**: disable attestations *(PR [#1423](https://github.com/t3rn/t3rn/pull/1423) by [@3h4x](https://github.com/3h4x))*
- [`d6508e3`](https://github.com/t3rn/t3rn/commit/d6508e3d51cdad329f7fefc76de106e5eeeceb71) - correct ABI descriptor for RLP-encoded "tass" & refresh standard ABI list at XDNS::reboot *(PR [#1427](https://github.com/t3rn/t3rn/pull/1427) by [@MaciejBaj](https://github.com/MaciejBaj))*

### :wrench: Chores
- [`1574969`](https://github.com/t3rn/t3rn/commit/157496902385af3991fa75cc28c2c7c742b21e1c) - add missing @t3rn/sdk TS options of Vendor::XBI & Instant *(PR [#1421](https://github.com/t3rn/t3rn/pull/1421) by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`2339cf0`](https://github.com/t3rn/t3rn/commit/2339cf0efbbb55da8380db4eaa3969aaf001df67) - add prettier rules and reformat the whole project *(PR [#1424](https://github.com/t3rn/t3rn/pull/1424) by [@gvko](https://github.com/gvko))*


## [v1.70.1-rc.0] - 2023-11-02
### :wrench: Chores
- [`4563cf7`](https://github.com/t3rn/t3rn/commit/4563cf73e367490aed828707c4b1d19825ad79f5) - add extrinsic for linking token to gateways *(PR [#1419](https://github.com/t3rn/t3rn/pull/1419) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.70.0-rc.0] - 2023-11-02
### :sparkles: New Features
- [`f66fd15`](https://github.com/t3rn/t3rn/commit/f66fd1523da1362e0203666ba0e27d30245fa54e) - add command to build executor binaries for linux, macOS and window (x64 and arm64) targets *(PR [#1420](https://github.com/t3rn/t3rn/pull/1420) by [@ahkohd](https://github.com/ahkohd))*
- [`570c328`](https://github.com/t3rn/t3rn/commit/570c328e700ecadedac9861ca6c8b27edb414fc5) - make SFX validation verbose & extend CLI with writer of Orders to Vacuum *(PR [#1409](https://github.com/t3rn/t3rn/pull/1409) by [@MaciejBaj](https://github.com/MaciejBaj))*

### :bug: Bug Fixes
- [`dcc3191`](https://github.com/t3rn/t3rn/commit/dcc3191033297c7791dd13897b56bf1fbe3d6478) - speed up CI with actions that can run on smaller workers *(PR [#1422](https://github.com/t3rn/t3rn/pull/1422) by [@3h4x](https://github.com/3h4x))*


## [v1.69.1-rc.0] - 2023-11-02
### :wrench: Chores
- [`46a1619`](https://github.com/t3rn/t3rn/commit/46a1619f9323c201b4f70d6f2ce2257c5e5bdcef) - update t0rn session parameters: time to 1h, kick threshold to 6h, no disabled validators from session *(PR [#1417](https://github.com/t3rn/t3rn/pull/1417) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.69.0-rc.0] - 2023-11-01
### :sparkles: New Features
- [`f0da32a`](https://github.com/t3rn/t3rn/commit/f0da32a80070c142f4d8c42dadab152649dc61fe) - registering assets on t0rn through CLI  and integration tests for XCM transfers of ROC *(PR [#1414](https://github.com/t3rn/t3rn/pull/1414) by [@chexware](https://github.com/chexware))*

### :bug: Bug Fixes
- [`b0da569`](https://github.com/t3rn/t3rn/commit/b0da569e0c9b46c1e494e42c2464fc08499780db) - loosen fail checks in CLI SDK signAndSafeSafe  *(PR [#1411](https://github.com/t3rn/t3rn/pull/1411) by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`9515591`](https://github.com/t3rn/t3rn/commit/9515591015134a065e704ab4424b33a5bbf156d3) - **sdk**: add CI and allow only yarn *(PR [#1412](https://github.com/t3rn/t3rn/pull/1412) by [@3h4x](https://github.com/3h4x))*
- [`6902b9c`](https://github.com/t3rn/t3rn/commit/6902b9cab4a770d3d46c6167c02f3339b6d3d4fc) - updated SDK version in CLI package.json to enable asset registration *(PR [#1415](https://github.com/t3rn/t3rn/pull/1415) by [@chexware](https://github.com/chexware))*

### :wrench: Chores
- [`d4f9f6d`](https://github.com/t3rn/t3rn/commit/d4f9f6d6ef09758fe9dbbf5e4c6f080c8b81b756) - update sepolia pallet version to tag v1.1.9 *(PR [#1416](https://github.com/t3rn/t3rn/pull/1416) by [@gvko](https://github.com/gvko))*


## [v1.68.0-rc.0] - 2023-10-30
### :sparkles: New Features
- [`9259eca`](https://github.com/t3rn/t3rn/commit/9259eca14e81aa3ec86065e444afe767e543429b) - add tx enrolling new sfx abi to selected gateway *(PR [#1402](https://github.com/t3rn/t3rn/pull/1402) by [@MaciejBaj](https://github.com/MaciejBaj))*

### :bug: Bug Fixes
- [`2c94881`](https://github.com/t3rn/t3rn/commit/2c94881d270ab8d979f80a747d36a538a999df19) - catch pallet errors in signAndSafe & format client SDK files *(PR [#1408](https://github.com/t3rn/t3rn/pull/1408) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.67.1-rc.0] - 2023-10-26
### :bug: Bug Fixes
- [`53e0586`](https://github.com/t3rn/t3rn/commit/53e0586ff0019d405af99d7b888eb9e5d5fb6d75) - update t1rn specs and remove obsoleted wasm binaries *(PR [#1400](https://github.com/t3rn/t3rn/pull/1400) by [@3h4x](https://github.com/3h4x))*


## [v1.67.0-rc.0] - 2023-10-25
### :sparkles: New Features
- [`a251d6d`](https://github.com/t3rn/t3rn/commit/a251d6dada3d59e4b18adb14bda0121bc021ab33) - include extra SpeedMode::Instant (only) *(PR [#1393](https://github.com/t3rn/t3rn/pull/1393) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.66.1-rc.0] - 2023-10-25
### :bug: Bug Fixes
- [`1716b0b`](https://github.com/t3rn/t3rn/commit/1716b0bff04c7b6466c8be5602040338e8acc716) - normalise ecdsa signature's recovery id for libsecp256k1 & ethereum *(PR [#1391](https://github.com/t3rn/t3rn/pull/1391) by [@MaciejBaj](https://github.com/MaciejBaj))*

### :wrench: Chores
- [`8ea57fa`](https://github.com/t3rn/t3rn/commit/8ea57faea90da5109f0b3ed09941c0671048c3f3) - update cli for local circuit node run *(PR [#1388](https://github.com/t3rn/t3rn/pull/1388) by [@gvko](https://github.com/gvko))*


## [v1.66.0-rc.0] - 2023-10-25
### :sparkles: New Features
- [`2cd87e7`](https://github.com/t3rn/t3rn/commit/2cd87e7ff8359931a79e1044d5c0b9f45edd15d5) - Grandpa Ranger Height Diff Loop *(PR [#1389](https://github.com/t3rn/t3rn/pull/1389) by [@coun7zero](https://github.com/coun7zero))*
- [`ea65d18`](https://github.com/t3rn/t3rn/commit/ea65d18c1aca0ffa296b9fd55965217567196b60) - make Vacuum use remote order addresses added to XDNS *(PR [#1394](https://github.com/t3rn/t3rn/pull/1394) by [@MaciejBaj](https://github.com/MaciejBaj))*

### :bug: Bug Fixes
- [`33a29c4`](https://github.com/t3rn/t3rn/commit/33a29c46718284f3ae9876430250cdd8102e5516) - updated CLI config to use latest SDK version *(PR [#1397](https://github.com/t3rn/t3rn/pull/1397) by [@chexware](https://github.com/chexware))*
- [`51d5843`](https://github.com/t3rn/t3rn/commit/51d58430e6ad309cf7dbaa20ab2a3379396c93df) - rewrite scheduleHeightMonitoring method *(PR [#1396](https://github.com/t3rn/t3rn/pull/1396) by [@coun7zero](https://github.com/coun7zero))*


## [v1.65.2-rc.0] - 2023-10-25
### :bug: Bug Fixes
- [`87d1882`](https://github.com/t3rn/t3rn/commit/87d1882c411a10575ebddfbad16c0dbe7daedde4) - successful USDT and TRN XCM transfers using CLI *(PR [#1392](https://github.com/t3rn/t3rn/pull/1392) by [@chexware](https://github.com/chexware))*


## [v1.65.0-rc.0] - 2023-10-22
### :sparkles: New Features
- [`b68ad72`](https://github.com/t3rn/t3rn/commit/b68ad72468b98ea299d159152387b3688013bbe3) - eth2-proof npm package *(PR [#1374](https://github.com/t3rn/t3rn/pull/1374) by [@3h4x](https://github.com/3h4x))*

### :bug: Bug Fixes
- [`cac2a09`](https://github.com/t3rn/t3rn/commit/cac2a09f9907f311b86ab8cc72dc722d6d61f233) - decrease CommitteeMajorityThresholdSepolia for sepolia ranger *(PR [#1379](https://github.com/t3rn/t3rn/pull/1379) by [@3h4x](https://github.com/3h4x))*


## [v1.64.1-rc.0] - 2023-10-19
### :bug: Bug Fixes
- [`60fe33a`](https://github.com/t3rn/t3rn/commit/60fe33a391e3dbf36724bfe191d84cf40bc53e1c) - **executor**: error with transfer on rococo *(PR [#1376](https://github.com/t3rn/t3rn/pull/1376) by [@3h4x](https://github.com/3h4x))*
- [`ad57e65`](https://github.com/t3rn/t3rn/commit/ad57e6571023d44d6f7ffb6e8d17ecc9a26ba442) - enable XCM transfers of USDT, ROC, and TRN between AssetHub & t0rn *(PR [#1375](https://github.com/t3rn/t3rn/pull/1375) by [@chexware](https://github.com/chexware))*


## [v1.64.0-rc.0] - 2023-10-19
### :sparkles: New Features
- [`0075308`](https://github.com/t3rn/t3rn/commit/0075308b00b04606d9f3a88ae8ab87bb658feb8f) - **sdk**: added SDK support for XCM transfers and enabled TRN XCM transfers through CLI *(PR [#1365](https://github.com/t3rn/t3rn/pull/1365) by [@chexware](https://github.com/chexware))*
- [`89f84ce`](https://github.com/t3rn/t3rn/commit/89f84ceb9902741a1356e70da39930f3b234ecd2) - add force flag for purge commands *(PR [#1370](https://github.com/t3rn/t3rn/pull/1370) by [@3h4x](https://github.com/3h4x))*

### :bug: Bug Fixes
- [`24e9c4a`](https://github.com/t3rn/t3rn/commit/24e9c4a6d9b4a5c0725b208352cf7ede4fdc293e) - runtime upgrade *(PR [#1367](https://github.com/t3rn/t3rn/pull/1367) by [@3h4x](https://github.com/3h4x))*
- [`e5c07e9`](https://github.com/t3rn/t3rn/commit/e5c07e94a0c2735774b4d4db6c5d12dbd7198c8a) - remove matrix which messes up CI required checks *(PR [#1373](https://github.com/t3rn/t3rn/pull/1373) by [@3h4x](https://github.com/3h4x))*


## [v1.63.1-rc.0] - 2023-10-16
### :bug: Bug Fixes
- [`5bc68dd`](https://github.com/t3rn/t3rn/commit/5bc68ddd12707e1b4d1b987c47b9340426c7ece3) - include binary for docker context *(PR [#1364](https://github.com/t3rn/t3rn/pull/1364) by [@3h4x](https://github.com/3h4x))*


## [v1.63.0-rc.0] - 2023-10-16
### :sparkles: New Features
- [`449bf83`](https://github.com/t3rn/t3rn/commit/449bf838952e379cb963dbf2c911510cd8e3b2cc) - t1rn release docker *(PR [#1355](https://github.com/t3rn/t3rn/pull/1355) by [@3h4x](https://github.com/3h4x))*


## [v1.62.0-rc.0] - 2023-10-16
### :sparkles: New Features
- [`554c231`](https://github.com/t3rn/t3rn/commit/554c2319d83e67e9280bdc02b708a3f4b83bd5a9) - t1rn release *(PR [#1352](https://github.com/t3rn/t3rn/pull/1352) by [@3h4x](https://github.com/3h4x))*

### :recycle: Refactors
- [`e6ce951`](https://github.com/t3rn/t3rn/commit/e6ce951107a711e82bb444394ae6dea4b78666f3) - add XOrders on top of RemoteOrders contracts  *(PR [#1353](https://github.com/t3rn/t3rn/pull/1353) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.61.0-rc.0] - 2023-10-13
### :sparkles: New Features
- [`5f98223`](https://github.com/t3rn/t3rn/commit/5f982232b37063b991b21d88adb8a6a3f3c56424) - add polkadot and kusama to executor *(PR [#1300](https://github.com/t3rn/t3rn/pull/1300) by [@3h4x](https://github.com/3h4x))*


## [v1.60.0-rc.0] - 2023-10-12
### :sparkles: New Features
- [`97e0f5c`](https://github.com/t3rn/t3rn/commit/97e0f5c93fbc64b12e6a6adbf94bfbabb0aaddd5) - **cli**: enabled XCM transactions sending using the CLI *(PR [#1350](https://github.com/t3rn/t3rn/pull/1350) by [@chexware](https://github.com/chexware))*
- [`624c90a`](https://github.com/t3rn/t3rn/commit/624c90a19c01e29cf6745ce9ffa02024cb35b487) - enable inbound bridging of ERC-20 tokens via GMP & SFX attestations  *(PR [#1293](https://github.com/t3rn/t3rn/pull/1293) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.59.0-rc.0] - 2023-10-09
### :sparkles: New Features
- [`e695992`](https://github.com/t3rn/t3rn/commit/e695992ed2296369bcfb6a643ff4d42dc0cfa968) - **executor**: handling nonce in relayer *(PR [#1330](https://github.com/t3rn/t3rn/pull/1330) by [@3h4x](https://github.com/3h4x))*


## [v1.58.1-rc.0] - 2023-10-03
### :bug: Bug Fixes
- [`bccb14d`](https://github.com/t3rn/t3rn/commit/bccb14ddd56ae4941c9b5c8d9273f8a5032c1b90) - lower SyncCommitteeSupermajority sepolia *(PR [#1336](https://github.com/t3rn/t3rn/pull/1336) by [@3h4x](https://github.com/3h4x))*


## [v1.58.0-rc.0] - 2023-09-29
### :sparkles: New Features
- [`c691c45`](https://github.com/t3rn/t3rn/commit/c691c4545e837f8c348cb44f9653edc0c6d37b84) - **executor**: batching transactions *(PR [#1320](https://github.com/t3rn/t3rn/pull/1320) by [@3h4x](https://github.com/3h4x))*
- [`93b5d85`](https://github.com/t3rn/t3rn/commit/93b5d8528722a2a0f6950cf274a85fb2d96d0735) - **cli**: xcm transfer *(PR [#1308](https://github.com/t3rn/t3rn/pull/1308) by [@3h4x](https://github.com/3h4x))*

### :bug: Bug Fixes
- [`6181cf9`](https://github.com/t3rn/t3rn/commit/6181cf9be6148c2c9b2048aae1e61e192e47fe10) - bump outdated t3rn SDK version at CLI package.json *(PR [#1332](https://github.com/t3rn/t3rn/pull/1332) by [@chexware](https://github.com/chexware))*

### :wrench: Chores
- [`24e6656`](https://github.com/t3rn/t3rn/commit/24e6656c49a166822900aeb0bf0c2610f2e147d9) - upgrade eth2-lc for sepolia to next version *(PR [#1333](https://github.com/t3rn/t3rn/pull/1333) by [@gvko](https://github.com/gvko))*


## [v1.57.0-rc.0] - 2023-09-28
### :sparkles: New Features
- [`9831fc0`](https://github.com/t3rn/t3rn/commit/9831fc0f2317b4973833c8f211d6a80b5facd3a5) - updated XCM configuration including Zombienet setup with enabled dev accounts on AssetHub *(PR [#1323](https://github.com/t3rn/t3rn/pull/1323) by [@chexware](https://github.com/chexware))*


## [v1.56.1-rc.0] - 2023-09-26
### :wrench: Chores
- [`64960cf`](https://github.com/t3rn/t3rn/commit/64960cf7e85e245b73c59d1b00cc8174ab75e784) - add dependabot config with grouping *(PR [#1266](https://github.com/t3rn/t3rn/pull/1266) by [@3h4x](https://github.com/3h4x))*


## [v1.56.0-rc.0] - 2023-09-26
### :sparkles: New Features
- [`a457399`](https://github.com/t3rn/t3rn/commit/a457399d5e9b579fa7ead2f324054947c107a4da) - zombienet test setup with AssetHub *(PR [#1313](https://github.com/t3rn/t3rn/pull/1313) by [@chexware](https://github.com/chexware))*


## [v1.55.2-rc.0] - 2023-09-22
### :bug: Bug Fixes
- [`4013a46`](https://github.com/t3rn/t3rn/commit/4013a4611ab0c1a21bb9573f7e9f8db6bfd644ed) - disregard past accumulated settlements from current round calculation *(PR [#1310](https://github.com/t3rn/t3rn/pull/1310) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.55.1-rc.0] - 2023-09-22
### :wrench: Chores
- [`bf9af27`](https://github.com/t3rn/t3rn/commit/bf9af274dc4d8bdd5e1966c5e8bb260a7ec6b77e) - disable inactive t0rn collators once per 6h session *(PR [#1311](https://github.com/t3rn/t3rn/pull/1311) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.55.0-rc.0] - 2023-09-21
### :sparkles: New Features
- [`3bcdf54`](https://github.com/t3rn/t3rn/commit/3bcdf54c856fbc1b67e49a4c7d82854e735dbdf2) - recover executor for each Vacuum::read_xtx_status *(PR [#1303](https://github.com/t3rn/t3rn/pull/1303) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.54.3-rc.0] - 2023-09-21
### :wrench: Chores
- [`a47e434`](https://github.com/t3rn/t3rn/commit/a47e4345920b6bcf49161d3c2d320464403bf5d9) - update sepolia pallet version to tag 1.1.3 *(PR [#1309](https://github.com/t3rn/t3rn/pull/1309) by [@gvko](https://github.com/gvko))*


## [v1.54.2-rc.0] - 2023-09-20
### :bug: Bug Fixes
- [`90cda7e`](https://github.com/t3rn/t3rn/commit/90cda7e7a29b486ea8d280ca5ba211de0031e24d) - check for above-zero target height increase before setting isActive *(PR [#1307](https://github.com/t3rn/t3rn/pull/1307) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.54.1-rc.0] - 2023-09-18
### :wrench: Chores
- [`93b3c26`](https://github.com/t3rn/t3rn/commit/93b3c263b5bbdebaff65b3def899b7a12ae9f724) - upgrade sepolia pallet version *(PR [#1306](https://github.com/t3rn/t3rn/pull/1306) by [@gvko](https://github.com/gvko))*


## [v1.54.0-rc.0] - 2023-09-16
### :sparkles: New Features
- [`f1af8c4`](https://github.com/t3rn/t3rn/commit/f1af8c4211d23449e93f59f277e1a1130932038f) - sdk align for ethereum ranger *(PR [#1304](https://github.com/t3rn/t3rn/pull/1304) by [@3h4x](https://github.com/3h4x))*

### :bug: Bug Fixes
- [`723f330`](https://github.com/t3rn/t3rn/commit/723f330a64a15f5f9676d16d44638c4ae06d36a6) - grandpa-ranger should not escape submission loop *(PR [#1299](https://github.com/t3rn/t3rn/pull/1299) by [@3h4x](https://github.com/3h4x))*
- [`7e14090`](https://github.com/t3rn/t3rn/commit/7e140907fa4ea2de7bf48c0880209fa236577072) - sdk have utils exported *(PR [#1305](https://github.com/t3rn/t3rn/pull/1305) by [@3h4x](https://github.com/3h4x))*


## [v1.53.0-rc.0] - 2023-09-12
### :sparkles: New Features
- [`1f2faba`](https://github.com/t3rn/t3rn/commit/1f2fababa6a560002becbe7a269f2b9cda0adbcb) - deployment kusama and polkadot ranger *(PR [#1289](https://github.com/t3rn/t3rn/pull/1289) by [@3h4x](https://github.com/3h4x))*
- [`9f923cc`](https://github.com/t3rn/t3rn/commit/9f923cc6240df58622b48dedd996d33a55e1a7bd) - t0rn XCM config update *(PR [#1298](https://github.com/t3rn/t3rn/pull/1298) by [@chexware](https://github.com/chexware))*

### :recycle: Refactors
- [`ebacf3e`](https://github.com/t3rn/t3rn/commit/ebacf3e22ad5eb20cbd9440b358cc6ebad668621) - **grandpa-ranger**: submissions without interval *(PR [#1296](https://github.com/t3rn/t3rn/pull/1296) by [@3h4x](https://github.com/3h4x))*

### :wrench: Chores
- [`2e86869`](https://github.com/t3rn/t3rn/commit/2e86869aa3ba52211f8e017dc819559c1054fcd0) - grandpa ranger healthcheck *(PR [#1294](https://github.com/t3rn/t3rn/pull/1294) by [@3h4x](https://github.com/3h4x))*


## [v1.52.4-rc.0] - 2023-09-08
### :white_check_mark: Tests
- [`a62119f`](https://github.com/t3rn/t3rn/commit/a62119f2b7969b74c6cba8a2981cc3778a430224) - check for drained ImportedHeaders at grandpa LC reset *(PR [#1291](https://github.com/t3rn/t3rn/pull/1291) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.52.3-rc.0] - 2023-09-08
### :bug: Bug Fixes
- [`540674f`](https://github.com/t3rn/t3rn/commit/540674f010b91b492f81486974ac500b951ed53a) - drain imported hashes at light client reset *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.52.2-rc.0] - 2023-09-08
### :bug: Bug Fixes
- [`da39864`](https://github.com/t3rn/t3rn/commit/da398646722cdb3ee24c662459c13f2a2301b694) - cli with config file *(PR [#1287](https://github.com/t3rn/t3rn/pull/1287) by [@3h4x](https://github.com/3h4x))*
- [`e01a035`](https://github.com/t3rn/t3rn/commit/e01a035335af8b4ef1759b2a616d775fcb25ab4d) - gha for cli *(PR [#1288](https://github.com/t3rn/t3rn/pull/1288) by [@3h4x](https://github.com/3h4x))*
- [`31760cc`](https://github.com/t3rn/t3rn/commit/31760ccfd6cb08cfd83c533b60fd17927d4612c0) - drain imported headers at grandpa reset *(PR [#1290](https://github.com/t3rn/t3rn/pull/1290) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.52.1-rc.0] - 2023-09-07
### :bug: Bug Fixes
- [`3b66878`](https://github.com/t3rn/t3rn/commit/3b6687890faa0ad2e051b5ad585ad010315377e1) - correct assignment for Polkadot and Kusama vendors and instances to runtimes *(PR [#1286](https://github.com/t3rn/t3rn/pull/1286) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.52.0-rc.0] - 2023-09-06
### :sparkles: New Features
- [`0af5253`](https://github.com/t3rn/t3rn/commit/0af52539c1aa3ac5df212c26e982b75bdc0f7641) - **grandpa-ranger**: new sdk and types npm + logging for ranger *(PR [#1278](https://github.com/t3rn/t3rn/pull/1278) by [@3h4x](https://github.com/3h4x))*
- [`f6b47af`](https://github.com/t3rn/t3rn/commit/f6b47afe57b7417e85543cb7fa2d89b898cfd8e6) - renaming CLI commands *(PR [#1285](https://github.com/t3rn/t3rn/pull/1285) by [@3h4x](https://github.com/3h4x))*

### :wrench: Chores
- [`727e0f2`](https://github.com/t3rn/t3rn/commit/727e0f218cfa00a5cabbf249bc6c8757ce072223) - change initialize grandpa light client to dispatch result *(PR [#1284](https://github.com/t3rn/t3rn/pull/1284) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.51.0-rc.0] - 2023-09-06
### :sparkles: New Features
- [`fbb238f`](https://github.com/t3rn/t3rn/commit/fbb238f6dc1d77f211057564ffbe71b45b9810e3) - migrate attesters + circuit to include finality fee against v1.0.0 Polkadot dependencies  *(PR [#1245](https://github.com/t3rn/t3rn/pull/1245) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.50.2-rc.0] - 2023-09-06
### :wrench: Chores
- [`c0e4b7b`](https://github.com/t3rn/t3rn/commit/c0e4b7bd73db7db030494fd32ee79913fd97375d) - add XDNS standard sfx abi to standalone chain specs *(PR [#1283](https://github.com/t3rn/t3rn/pull/1283) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.50.1-rc.0] - 2023-09-06
### :bug: Bug Fixes
- [`dfe6984`](https://github.com/t3rn/t3rn/commit/dfe69845c2a3821193e557cc114c102aa2c53999) - sdk/types support sepolia gw *(PR [#1281](https://github.com/t3rn/t3rn/pull/1281) by [@3h4x](https://github.com/3h4x))*

### :wrench: Chores
- [`4876db1`](https://github.com/t3rn/t3rn/commit/4876db12bf4d8190a7cd286a40a38c6e4f126bb3) - reinstantiate standalone RPC for Portal + XDNS *(PR [#1282](https://github.com/t3rn/t3rn/pull/1282) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.50.0-rc.0] - 2023-09-05
### :sparkles: New Features
- [`cf3d8f8`](https://github.com/t3rn/t3rn/commit/cf3d8f817f633d7f7e7fc5dec746646e40d6ef56) - zombienet base test for executing SFX *(PR [#1269](https://github.com/t3rn/t3rn/pull/1269) by [@3h4x](https://github.com/3h4x))*
- [`9b52dae`](https://github.com/t3rn/t3rn/commit/9b52dae6a3e6a44342fca83241e9271e2031b896) - add sdk to npmjs *(PR [#1273](https://github.com/t3rn/t3rn/pull/1273) by [@3h4x](https://github.com/3h4x))*

### :bug: Bug Fixes
- [`d238dfd`](https://github.com/t3rn/t3rn/commit/d238dfd7a933f0f24866f66f8f3fc3f116aa399e) - sfx in xtx existence check *(PR [#1276](https://github.com/t3rn/t3rn/pull/1276) by [@Sj-001](https://github.com/Sj-001))*
- [`6567047`](https://github.com/t3rn/t3rn/commit/6567047a86145383ddd41981e78214ed48f1a183) - npm packages have to be built before publishing *(PR [#1277](https://github.com/t3rn/t3rn/pull/1277) by [@3h4x](https://github.com/3h4x))*
- [`abbdcd9`](https://github.com/t3rn/t3rn/commit/abbdcd983fbcb5e5d0ce1da65734564436662f33) - derive admin origin set to Escrow for Root token destroys *(PR [#1279](https://github.com/t3rn/t3rn/pull/1279) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.49.0-rc.0] - 2023-08-29
### :bug: Bug Fixes
- [`3294ecc`](https://github.com/t3rn/t3rn/commit/3294eccd03ee10eb123a0503ee3d4f0e0205b581) - cli register gateway *(PR [#1270](https://github.com/t3rn/t3rn/pull/1270) by [@ahkohd](https://github.com/ahkohd))*

### :recycle: Refactors
- [`d2e7891`](https://github.com/t3rn/t3rn/commit/d2e7891b757122be44b7433f63eae61f6f5758fe) - revisit and test attestation targets removal *(PR [#1271](https://github.com/t3rn/t3rn/pull/1271) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.48.6-rc.0] - 2023-08-28
### :bug: Bug Fixes
- [`b87c183`](https://github.com/t3rn/t3rn/commit/b87c1830abd0ac53566e28bd233a344b37c88425) - docker image paths and docs *(PR [#1268](https://github.com/t3rn/t3rn/pull/1268) by [@3h4x](https://github.com/3h4x))*


## [v1.48.5-rc.0] - 2023-08-25
### :bug: Bug Fixes
- [`e4ea788`](https://github.com/t3rn/t3rn/commit/e4ea788ab94b5939e5f782e4b8a327c73927700c) - **executor**: balance calculation and new metrics *(PR [#1260](https://github.com/t3rn/t3rn/pull/1260) by [@3h4x](https://github.com/3h4x))*
- [`b6bdfb6`](https://github.com/t3rn/t3rn/commit/b6bdfb64b44b0cc3724b9af48dd1d97faf2b8d9c) - docker image name aligned *(PR [#1263](https://github.com/t3rn/t3rn/pull/1263) by [@3h4x](https://github.com/3h4x))*

### :wrench: Chores
- [`8ca4111`](https://github.com/t3rn/t3rn/commit/8ca4111e68106bd5ecc513bd8eb99aaf88572df5) - move docker images from aws to ghcr *(PR [#1267](https://github.com/t3rn/t3rn/pull/1267) by [@3h4x](https://github.com/3h4x))*


## [v1.48.4-rc.0] - 2023-08-23
### :bug: Bug Fixes
- [`4f10ae4`](https://github.com/t3rn/t3rn/commit/4f10ae4752d20286cf868bb7c4dccea7b7b5d2c4) - inclusion_receipt.height should be lower than submission_target_height *(PR [#1262](https://github.com/t3rn/t3rn/pull/1262) by [@3h4x](https://github.com/3h4x))*


## [v1.48.3-rc.0] - 2023-08-23
### :bug: Bug Fixes
- [`0cbba7d`](https://github.com/t3rn/t3rn/commit/0cbba7d2c39df3a4dbee0ed01b73c8dc4f51871f) - **executor**: outdated transactions *(PR [#1256](https://github.com/t3rn/t3rn/pull/1256) by [@3h4x](https://github.com/3h4x))*
- [`b59ff49`](https://github.com/t3rn/t3rn/commit/b59ff49029e88941e2b999e237a2debe37c61726) - **cli**: correctly define secret variables in cronjob.yaml *(PR [#1258](https://github.com/t3rn/t3rn/pull/1258) by [@3h4x](https://github.com/3h4x))*

### :wrench: Chores
- [`527f158`](https://github.com/t3rn/t3rn/commit/527f158a07bc5b1bb4d00a245f6bbd82dbe3ac1d) - add debug logs for inclusion proof *(PR [#1259](https://github.com/t3rn/t3rn/pull/1259) by [@3h4x](https://github.com/3h4x))*
- [`ac97fa8`](https://github.com/t3rn/t3rn/commit/ac97fa888d950943c8588aa31047854a9995065e) - improve failed xtx logging *(PR [#1261](https://github.com/t3rn/t3rn/pull/1261) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.48.1-rc.0] - 2023-08-21
### :bug: Bug Fixes
- [`f38c65b`](https://github.com/t3rn/t3rn/commit/f38c65b4811ce7f533c7dca984d7f0b83ccf3936) - runtime upgrade script using sudo flag *(PR [#1252](https://github.com/t3rn/t3rn/pull/1252) by [@3h4x](https://github.com/3h4x))*


## [v1.48.0-rc.0] - 2023-08-21
### :sparkles: New Features
- [`1ffa9e3`](https://github.com/t3rn/t3rn/commit/1ffa9e3419fa6d66d005585a55671239c9bb232d) - **zombienet**: zombienet parachain id fix and upgrade ci *(PR [#1251](https://github.com/t3rn/t3rn/pull/1251) by [@3h4x](https://github.com/3h4x))*


## [v1.47.2-rc.0] - 2023-08-18
### :bug: Bug Fixes
- [`ee49da9`](https://github.com/t3rn/t3rn/commit/ee49da9ce4fd92ae788e6d2844eec5ddb520db3d) - zombienet and improve runtime upgrade pipeline *(PR [#1249](https://github.com/t3rn/t3rn/pull/1249) by [@3h4x](https://github.com/3h4x))*


## [v1.47.1-rc.0] - 2023-08-18
### :wrench: Chores
- [`06c7f1e`](https://github.com/t3rn/t3rn/commit/06c7f1e8293b0702c44d490be16239a3b731fd71) - add extra Xtx Status log for submission target height *(PR [#1244](https://github.com/t3rn/t3rn/pull/1244) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.47.0-rc.0] - 2023-08-16
### :sparkles: New Features
- [`7680747`](https://github.com/t3rn/t3rn/commit/768074798abaca51d0c3b2107c22cd0058bb51fd) - **executor**: optimistic execution for roco *(PR [#1212](https://github.com/t3rn/t3rn/pull/1212) by [@3h4x](https://github.com/3h4x))*
- [`46d58e4`](https://github.com/t3rn/t3rn/commit/46d58e4bb3504bdcefb776a0b1da5bc5dcb3446f) - implement substrate price estimation *(PR [#1234](https://github.com/t3rn/t3rn/pull/1234) by [@ahkohd](https://github.com/ahkohd))*

### :bug: Bug Fixes
- [`1ef71ea`](https://github.com/t3rn/t3rn/commit/1ef71ea2d8459077b072843f8e3b6cf0e6785e26) - sdk docs build *(PR [#1236](https://github.com/t3rn/t3rn/pull/1236) by [@ahkohd](https://github.com/ahkohd))*
- [`ae8fb8a`](https://github.com/t3rn/t3rn/commit/ae8fb8a69208af573f0fc9c4a8a4587f710661f4) - **executor**: custom signer for relaychain *(PR [#1241](https://github.com/t3rn/t3rn/pull/1241) by [@3h4x](https://github.com/3h4x))*


## [v1.46.2-rc.0] - 2023-08-09
### :bug: Bug Fixes
- [`e32892e`](https://github.com/t3rn/t3rn/commit/e32892e9f9e1ff3fb2072e2c434f3d6e6f1b43ba) - **executor**: new contract for rotated committee *(PR [#1228](https://github.com/t3rn/t3rn/pull/1228) by [@3h4x](https://github.com/3h4x))*
- [`d1178a6`](https://github.com/t3rn/t3rn/commit/d1178a6f3d507e9636e196b976e7ef4e40f3cd58) - reduce majority threshold to unbrick LC *(PR [#1235](https://github.com/t3rn/t3rn/pull/1235) by [@3h4x](https://github.com/3h4x))*


## [v1.46.1-rc.0] - 2023-08-03
### :bug: Bug Fixes
- [`e410842`](https://github.com/t3rn/t3rn/commit/e41084273564fffc586d532810701036dbb5d340) - temporarily set 67% committee majority threshold to Sepolia light client  *(PR [#1232](https://github.com/t3rn/t3rn/pull/1232) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.46.0-rc.0] - 2023-08-03
### :sparkles: New Features
- [`7dca20e`](https://github.com/t3rn/t3rn/commit/7dca20e525ab4b1e3648c32bc0d010fabc004b19) - implement evm call price estimation *(PR [#1229](https://github.com/t3rn/t3rn/pull/1229) by [@ahkohd](https://github.com/ahkohd))*

### :bug: Bug Fixes
- [`9811de3`](https://github.com/t3rn/t3rn/commit/9811de379f3c59a0a5563ebd4e3274e0edb9fcea) - reduce majority threshold to unbrick LC *(PR [#1230](https://github.com/t3rn/t3rn/pull/1230) by [@petscheit](https://github.com/petscheit))*


## [v1.45.1-rc.0] - 2023-07-28
### :bug: Bug Fixes
- [`e4b9c82`](https://github.com/t3rn/t3rn/commit/e4b9c82a38a9ffb2fa40b9ceeb55ab6eddc0c492) - move process estimation of treasury balances to on_initialize hook *(PR [#1223](https://github.com/t3rn/t3rn/pull/1223) by [@MaciejBaj](https://github.com/MaciejBaj))*
  - :arrow_lower_right: *fixes issue [#1162](undefined) opened by [@3h4x](https://github.com/3h4x)*


## [v1.45.0-rc.0] - 2023-07-28
### :sparkles: New Features
- [`12050ac`](https://github.com/t3rn/t3rn/commit/12050ac82ea545076b1edbdf27c7c1817fd701d1) - extend GatewayVendor select with XBI and Sepolia *(PR [#1224](https://github.com/t3rn/t3rn/pull/1224) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.44.2-rc.0] - 2023-07-28
### :bug: Bug Fixes
- [`25073b1`](https://github.com/t3rn/t3rn/commit/25073b1f98f92ce246333de14e829e03774a7780) - remove remaining deprecated XDNS records with storage migration *(PR [#1225](https://github.com/t3rn/t3rn/pull/1225) by [@MaciejBaj](https://github.com/MaciejBaj))*

### :wrench: Chores
- [`6377990`](https://github.com/t3rn/t3rn/commit/6377990cdc19197fb503f7f8c9744702ade17c8e) - disable dependabot until polkadot upgrade is done *(PR [#1227](https://github.com/t3rn/t3rn/pull/1227) by [@3h4x](https://github.com/3h4x))*


## [v1.44.1-rc.0] - 2023-07-27
### :bug: Bug Fixes
- [`6075c3b`](https://github.com/t3rn/t3rn/commit/6075c3b35aa29a45b96e8cb5931795e49cad5e27) - stop committee shuffle if previous transition request still awaits attestation *(PR [#1221](https://github.com/t3rn/t3rn/pull/1221) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.44.0-rc.0] - 2023-07-27
### :sparkles: New Features
- [`3713380`](https://github.com/t3rn/t3rn/commit/3713380f6097e36a93742360478a1aeb2cfb4029) - implement bridge contract to receive remote orders *(PR [#1174](https://github.com/t3rn/t3rn/pull/1174) by [@MaciejBaj](https://github.com/MaciejBaj))*

### :white_check_mark: Tests
- [`8daf064`](https://github.com/t3rn/t3rn/commit/8daf0647764b68d0ad117842cc841cab6f8a5218) - cover xtx id calculations for nonces 0,1 and 2 *(PR [#1205](https://github.com/t3rn/t3rn/pull/1205) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.43.4-rc.0] - 2023-07-27
### :bug: Bug Fixes
- [`38fefaa`](https://github.com/t3rn/t3rn/commit/38fefaa0af383bd8220280d8cf09851f13e8d5f7) - add storage migration to raw key of XDNSRegistry *(PR [#1222](https://github.com/t3rn/t3rn/pull/1222) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.43.3-rc.0] - 2023-07-27
### :bug: Bug Fixes
- [`3827ef9`](https://github.com/t3rn/t3rn/commit/3827ef92b284f949aebd88040d9a02213958e84b) - ensure XDNS override stores single Gateway ID *(PR [#1218](https://github.com/t3rn/t3rn/pull/1218) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.43.1-rc.0] - 2023-07-25
### :bug: Bug Fixes
- [`0fadd75`](https://github.com/t3rn/t3rn/commit/0fadd750e4937e3ab1f9266bdd5560cac05193ac) - executor sending Confirmed Batches *(PR [#1207](https://github.com/t3rn/t3rn/pull/1207) by [@3h4x](https://github.com/3h4x))*


## [v1.43.0-rc.0] - 2023-07-24
### :sparkles: New Features
- [`e7715f9`](https://github.com/t3rn/t3rn/commit/e7715f9009e27ef3a3447d82887dbf6910323e9a) - **cli**: send continuously SFX to circuit *(PR [#1185](https://github.com/t3rn/t3rn/pull/1185) by [@3h4x](https://github.com/3h4x))*
- [`6641262`](https://github.com/t3rn/t3rn/commit/664126223dde67ad275fb5c2dcdab70612896107) - **cli**: parametrize CLI with env variables *(PR [#1196](https://github.com/t3rn/t3rn/pull/1196) by [@3h4x](https://github.com/3h4x))*
- [`ecbb98a`](https://github.com/t3rn/t3rn/commit/ecbb98a9e3a49e929135163c618cc6bb25cfc1df) - **executor**: bid monitoring and logging *(PR [#1200](https://github.com/t3rn/t3rn/pull/1200) by [@3h4x](https://github.com/3h4x))*

### :bug: Bug Fixes
- [`e24ba6a`](https://github.com/t3rn/t3rn/commit/e24ba6a179c7644f10dfd27c522981b3bdb2e0d4) - **executor**: contract data types *(PR [#1183](https://github.com/t3rn/t3rn/pull/1183) by [@3h4x](https://github.com/3h4x))*

### :wrench: Chores
- [`c3c8f13`](https://github.com/t3rn/t3rn/commit/c3c8f1352452bcbd22c8082693871c697cc628f6) - **docs**: update packages and add dependabot *(PR [#1184](https://github.com/t3rn/t3rn/pull/1184) by [@3h4x](https://github.com/3h4x))*


## [v1.42.0-rc.0] - 2023-07-18
### :sparkles: New Features
- [`d2d3436`](https://github.com/t3rn/t3rn/commit/d2d3436ea14fdc78c274d7280c33b24a68145365) - change XTX & SFX hashing to Keccak256 *(PR [#1173](https://github.com/t3rn/t3rn/pull/1173) by [@MaciejBaj](https://github.com/MaciejBaj))*

### :recycle: Refactors
- [`fc398d7`](https://github.com/t3rn/t3rn/commit/fc398d7a4e6e7a275df131a70ee92f62b9b0fc43) - **executor**: prometheus as a global class *(PR [#1182](https://github.com/t3rn/t3rn/pull/1182) by [@3h4x](https://github.com/3h4x))*


## [v1.41.0-rc.0] - 2023-07-17
### :sparkles: New Features
- [`0aa9ba6`](https://github.com/t3rn/t3rn/commit/0aa9ba655d04e9dda4757a4c6501f80a27ab8a71) - executor send batch to sepl with new hash *(PR [#1145](https://github.com/t3rn/t3rn/pull/1145) by [@3h4x](https://github.com/3h4x))*
- [`360c947`](https://github.com/t3rn/t3rn/commit/360c947b93fb910aea574ecba5df5eee63f73d43) - change XTX & DLQ lifecycle to adaptive timeouts *(PR [#1140](https://github.com/t3rn/t3rn/pull/1140) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.40.0-rc.0] - 2023-07-17
### :sparkles: New Features
- [`a54d856`](https://github.com/t3rn/t3rn/commit/a54d856e3ea915fe7f623c0dd9b8e796cbdefd9f) - add ETH gas fees price estimation utility methods to @t3rn/sdk *(PR [#1091](https://github.com/t3rn/t3rn/pull/1091) by [@ahkohd](https://github.com/ahkohd))*
  - :arrow_lower_right: *addresses issue [#1163](undefined) opened by [@3h4x](https://github.com/3h4x)*


## [v1.39.4-rc.0] - 2023-07-14
### :bug: Bug Fixes
- [`2fa3304`](https://github.com/t3rn/t3rn/commit/2fa33046c256c43b62cab43d3877e721a1579aeb) - current committee transitions to next committee correctly *(PR [#1165](https://github.com/t3rn/t3rn/pull/1165) by [@3h4x](https://github.com/3h4x))*


## [v1.39.3-rc.0] - 2023-07-14
### :wrench: Chores
- [`2a4fd88`](https://github.com/t3rn/t3rn/commit/2a4fd881d61245a7dc10f3441b3eb168919c7f43) - skip t0rn-release only when commit starts with  build(release) *(PR [#1134](https://github.com/t3rn/t3rn/pull/1134) by [@3h4x](https://github.com/3h4x))*


## [v1.39.0-rc.0] - 2023-07-13
### :sparkles: New Features
- [`65030ad`](https://github.com/t3rn/t3rn/commit/65030ad8e60ff7c0682de9957e867258e3eb5a94) - add vacuum pallet to enhance transfer assets SFX path *(PR [#1132](https://github.com/t3rn/t3rn/pull/1132) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.38.2-rc.0] - 2023-07-11
### :wrench: Chores
- [`fd40dba`](https://github.com/t3rn/t3rn/commit/fd40dba403e9a18f6e17084f347942de5b71b393) - empty PR template *(PR [#1148](https://github.com/t3rn/t3rn/pull/1148) by [@3h4x](https://github.com/3h4x))*


## [v1.38.1-rc.0] - 2023-07-10
### :bug: Bug Fixes
- [`a88c4ad`](https://github.com/t3rn/t3rn/commit/a88c4ad00122f28b8a438861c7800c4ad754d302) - filter modification *(PR [#1135](https://github.com/t3rn/t3rn/pull/1135) by [@palozano](https://github.com/palozano))*


## [v1.38.0-rc.0] - 2023-07-10
### :sparkles: New Features
- [`af3fd73`](https://github.com/t3rn/t3rn/commit/af3fd731f9ce2755b300799468ba457d685cf481) - executor send batches to sepl *(PR [#1087](https://github.com/t3rn/t3rn/pull/1087) by [@3h4x](https://github.com/3h4x))*


## [v1.37.0-rc.0] - 2023-07-05
### :sparkles: New Features
- [`213bce9`](https://github.com/t3rn/t3rn/commit/213bce9ec42f41c8d27f5523704ef045c973c0b8) - include smart contract targets in source verification  *(PR [#1065](https://github.com/t3rn/t3rn/pull/1065) by [@MaciejBaj](https://github.com/MaciejBaj))*

### :recycle: Refactors
- [`e9ab115`](https://github.com/t3rn/t3rn/commit/e9ab115255aaed6ec9a803148e80a35abd8506f3) - check lock files *(PR [#1118](https://github.com/t3rn/t3rn/pull/1118) by [@ahkohd](https://github.com/ahkohd))*

### :wrench: Chores
- [`257019c`](https://github.com/t3rn/t3rn/commit/257019c80ade92770afd4e384af3473f18daf15e) - only one package manager to rule them all [skip release] *(PR [#1116](https://github.com/t3rn/t3rn/pull/1116) by [@palozano](https://github.com/palozano))*


## [v1.36.4-rc.0] - 2023-06-28
### :bug: Bug Fixes
- [`60d2017`](https://github.com/t3rn/t3rn/commit/60d2017fe52407ba1e0098ec8d51bb9ac74ee7d3) - update SDK metadata *(PR [#1095](https://github.com/t3rn/t3rn/pull/1095) by [@3h4x](https://github.com/3h4x))*

### :wrench: Chores
- [`5f76d56`](https://github.com/t3rn/t3rn/commit/5f76d56c4146bd1925190f544e13220d0c4c6cba) - **grandpa-ranger**: types/SDK changes should trigger deploy *(PR [#1099](https://github.com/t3rn/t3rn/pull/1099) by [@3h4x](https://github.com/3h4x))*


## [v1.36.2-rc.0] - 2023-06-26
### :bug: Bug Fixes
- [`90bee60`](https://github.com/t3rn/t3rn/commit/90bee60e22a53cbc81c8daffeca9a897692611ad) - deploy phase should not be run by dependabot *(PR [#1085](https://github.com/t3rn/t3rn/pull/1085) by [@3h4x](https://github.com/3h4x))*
- [`9d6b2e3`](https://github.com/t3rn/t3rn/commit/9d6b2e3ecb5a04005158ce4543fe911c78a2cf99) - executor deploy step *(PR [#1086](https://github.com/t3rn/t3rn/pull/1086) by [@3h4x](https://github.com/3h4x))*


## [v1.36.0-rc.0] - 2023-06-21
### :sparkles: New Features
- [`e9e52ce`](https://github.com/t3rn/t3rn/commit/e9e52ce1b772da87348fe14b65e8a514ad9e732c) - add telemetry to executor *(PR [#1051](https://github.com/t3rn/t3rn/pull/1051) by [@ahkohd](https://github.com/ahkohd))*
- [`d3672dd`](https://github.com/t3rn/t3rn/commit/d3672dd762cb1e7bcf7de958993e3d2caaad5b93) - process gateways activity overview in XDNS *(PR [#1054](https://github.com/t3rn/t3rn/pull/1054) by [@MaciejBaj](https://github.com/MaciejBaj))*

### :bug: Bug Fixes
- [`0f711aa`](https://github.com/t3rn/t3rn/commit/0f711aaeb5a5cc0779f5f8f5e9ab49bffa1e5895) - log all attester events and update metrics *(PR [#1078](https://github.com/t3rn/t3rn/pull/1078) by [@3h4x](https://github.com/3h4x))*


## [v1.35.0-rc.0] - 2023-06-20
### :sparkles: New Features
- [`3ce9ff9`](https://github.com/t3rn/t3rn/commit/3ce9ff9c7e3aa05c712a7ba85b1e8631cd9095cb) - added attestationsDone array *(PR [#1075](https://github.com/t3rn/t3rn/pull/1075) by [@3h4x](https://github.com/3h4x))*

### :wrench: Chores
- [`e09d383`](https://github.com/t3rn/t3rn/commit/e09d383ed0e1646c48f0726e3e66687f07fdbf5a) - update CODEOWNERS and remove obsoleted files *(PR [#1076](https://github.com/t3rn/t3rn/pull/1076) by [@3h4x](https://github.com/3h4x))*
- [`2466847`](https://github.com/t3rn/t3rn/commit/2466847b54e56c0eb9415630e07b91618cacb81d) - move pr template to t3rn repo *(PR [#1077](https://github.com/t3rn/t3rn/pull/1077) by [@3h4x](https://github.com/3h4x))*


## [v1.34.2-rc.0] - 2023-06-19
### :bug: Bug Fixes
- [`85d9a26`](https://github.com/t3rn/t3rn/commit/85d9a26d3bd461348fdc0543285e26cf01b6127b) - remove unused labeled metric  *(PR [#1073](https://github.com/t3rn/t3rn/pull/1073) by [@petscheit](https://github.com/petscheit))*
- [`42ef660`](https://github.com/t3rn/t3rn/commit/42ef6601fca29018b7116b422153e6a94efb374d) - quorum calculates Committee not ActiveSet size *(PR [#1074](https://github.com/t3rn/t3rn/pull/1074) by [@MaciejBaj](https://github.com/MaciejBaj))*

### :wrench: Chores
- [`eb92841`](https://github.com/t3rn/t3rn/commit/eb92841e2617badd58385e213423a8be0cb3312d) - run review only when label is added *(PR [#1072](https://github.com/t3rn/t3rn/pull/1072) by [@3h4x](https://github.com/3h4x))*


## [v1.34.1-rc.0] - 2023-06-19
### :bug: Bug Fixes
- [`b79e72a`](https://github.com/t3rn/t3rn/commit/b79e72aeb89e67896a9d0b1510d30b5d66329d56) - skip decoding to H160 in attestation signature verification  *(PR [#1067](https://github.com/t3rn/t3rn/pull/1067) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.34.0-rc.0] - 2023-06-19
### :recycle: Refactors
- [`293a7a6`](https://github.com/t3rn/t3rn/commit/293a7a6fed8cae763b62830eb653eeb38a579791) - customise max rewards executors kickback and default to zero *(PR [#1068](https://github.com/t3rn/t3rn/pull/1068) by [@MaciejBaj](https://github.com/MaciejBaj))*

### :wrench: Chores
- [`6b3b797`](https://github.com/t3rn/t3rn/commit/6b3b797dd4ee56fa2243f9650e899b7f0ce4a50d) - deploy docs on self hosted runners *(PR [#1071](https://github.com/t3rn/t3rn/pull/1071) by [@3h4x](https://github.com/3h4x))*


## [v1.33.0-rc.0] - 2023-06-16
### :sparkles: New Features
- [`7e884bc`](https://github.com/t3rn/t3rn/commit/7e884bc7dc2218dad636a8c3a7fb9e003e560e91) - **attester**: submit attestation *(PR [#1037](https://github.com/t3rn/t3rn/pull/1037) by [@3h4x](https://github.com/3h4x))*

### :bug: Bug Fixes
- [`be15410`](https://github.com/t3rn/t3rn/commit/be154103fb85a2fdb3a522b4552f6542c96e5fa8) - sdk ignores local gateway entry *(PR [#1061](https://github.com/t3rn/t3rn/pull/1061) by [@petscheit](https://github.com/petscheit))*

### :recycle: Refactors
- [`37933ab`](https://github.com/t3rn/t3rn/commit/37933abb08ed6adb8c897c4f67c2a3d0ec838217) - check XDNS entry exists before adding as attestation targets + permanent slash as StorageValue *(PR [#1064](https://github.com/t3rn/t3rn/pull/1064) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.32.3-rc.0] - 2023-06-14
### :bug: Bug Fixes
- [`b80f031`](https://github.com/t3rn/t3rn/commit/b80f0314fe46d68cea24287c8517258a4788aa55) - add reboot self gateway extrinsic to XDNS *(PR [#1060](https://github.com/t3rn/t3rn/pull/1060) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.32.0-rc.0] - 2023-06-13
### :sparkles: New Features
- [`dcece94`](https://github.com/t3rn/t3rn/commit/dcece94a68375054d918ab735ea9751311d18b14) - connect XDNS with Assets to create, purge, lookup via Overlay trait *(PR [#1027](https://github.com/t3rn/t3rn/pull/1027) by [@MaciejBaj](https://github.com/MaciejBaj))*

### :bug: Bug Fixes
- [`a758412`](https://github.com/t3rn/t3rn/commit/a7584120b8510c61eea8cb8b371829441b7b5c0d) - executor bidding works *(PR [#1052](https://github.com/t3rn/t3rn/pull/1052) by [@3h4x](https://github.com/3h4x))*


## [v1.31.1-rc.0] - 2023-06-13
### :bug: Bug Fixes
- [`eaa0fec`](https://github.com/t3rn/t3rn/commit/eaa0fec43b1d1481aea1af769983bdf0f87c8c90) - client apps should deploy in order *(PR [#1036](https://github.com/t3rn/t3rn/pull/1036) by [@3h4x](https://github.com/3h4x))*
- [`ce7aa01`](https://github.com/t3rn/t3rn/commit/ce7aa01947adaab62461e8292477678dfe414587) - create new batches for each committee transition *(PR [#1044](https://github.com/t3rn/t3rn/pull/1044) by [@MaciejBaj](https://github.com/MaciejBaj))*

### :wrench: Chores
- [`72237ed`](https://github.com/t3rn/t3rn/commit/72237ed2a1f431d01e66f97dc54049b1d97c8c16) - offchain script for automated nominations *(PR [#1042](https://github.com/t3rn/t3rn/pull/1042) by [@3h4x](https://github.com/3h4x))*


## [v1.31.0-rc.0] - 2023-06-09
### :sparkles: New Features
- [`cf76b08`](https://github.com/t3rn/t3rn/commit/cf76b08b634ab2061c675223265968e5eba9e22e) - add support to register eth2 gateway via CLI *(PR [#1031](https://github.com/t3rn/t3rn/pull/1031) by [@ahkohd](https://github.com/ahkohd))*

### :bug: Bug Fixes
- [`38e9bdc`](https://github.com/t3rn/t3rn/commit/38e9bdc3976773cdea0bcca223c0c967444f01d4) - **executor**: config in Docker and libraries update *(PR [#1028](https://github.com/t3rn/t3rn/pull/1028) by [@3h4x](https://github.com/3h4x))*
- [`be9f99a`](https://github.com/t3rn/t3rn/commit/be9f99a6a53438ddb90bac4082eee0f01ccddcc6) - **attester**: info logs for events are spamming *(PR [#1030](https://github.com/t3rn/t3rn/pull/1030) by [@3h4x](https://github.com/3h4x))*


## [v1.30.0-rc.0] - 2023-06-06
### :sparkles: New Features
- [`275f71b`](https://github.com/t3rn/t3rn/commit/275f71b6164c11246cafcd0b741062ca372616b0) - update executor *(PR [#1026](https://github.com/t3rn/t3rn/pull/1026) by [@3h4x](https://github.com/3h4x))*
- [`f4684f0`](https://github.com/t3rn/t3rn/commit/f4684f0e36100b4bff8c59264c58c8bf59dcbb33) - allow attesters to agree on the new targets & support eth  *(PR [#1015](https://github.com/t3rn/t3rn/pull/1015) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.29.2-rc.0] - 2023-06-01
### :bug: Bug Fixes
- [`97c14f1`](https://github.com/t3rn/t3rn/commit/97c14f1608a48ed7f2f57d8575092dd1ccc89703) - **chainspecs**: replace outdated bootnodes for rococo *(PR [#1024](https://github.com/t3rn/t3rn/pull/1024) by [@3h4x](https://github.com/3h4x))*


## [v1.29.0-rc.0] - 2023-05-30
### :sparkles: New Features
- [`103f832`](https://github.com/t3rn/t3rn/commit/103f8321cc8940724687642716aa7cb78305a217) - data generation framework for unhappy paths *(PR [#1014](https://github.com/t3rn/t3rn/pull/1014) by [@palozano](https://github.com/palozano))*


## [v1.28.0-rc.0] - 2023-05-26
### :sparkles: New Features
- [`2116659`](https://github.com/t3rn/t3rn/commit/211665983df714cc6cf48d226b5b734de145e5e6) - allow attester calls *(PR [#1016](https://github.com/t3rn/t3rn/pull/1016) by [@palozano](https://github.com/palozano))*

### :wrench: Chores
- [`f810e0c`](https://github.com/t3rn/t3rn/commit/f810e0c3eeb67816e29392a3a68b77cf7ec87725) - remove unused scripts *(PR [#999](https://github.com/t3rn/t3rn/pull/999) by [@3h4x](https://github.com/3h4x))*


## [v1.27.1-rc.0] - 2023-05-22
### :bug: Bug Fixes
- [`b05216d`](https://github.com/t3rn/t3rn/commit/b05216d82696740d946ce4a1d931580a808b717f) - grandpa range edge case *(PR [#1001](https://github.com/t3rn/t3rn/pull/1001) by [@petscheit](https://github.com/petscheit))*


## [v1.27.0-rc.0] - 2023-05-19
### :sparkles: New Features
- [`5243ee1`](https://github.com/t3rn/t3rn/commit/5243ee12816ad84e990d19198d1ea1898b13725e) - batch attestations per target for committee transitions, penalties and SFX *(PR [#980](https://github.com/t3rn/t3rn/pull/980) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.26.0-rc.0] - 2023-05-19
### :recycle: Refactors
- [`e991c25`](https://github.com/t3rn/t3rn/commit/e991c25737a597ae2573d32d3697751e10ad578b) - connect and use ReadSFX interface from Circuit to Attesters *(PR [#884](https://github.com/t3rn/t3rn/pull/884) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.25.1-rc.0] - 2023-05-18
### :bug: Bug Fixes
- [`2825cd5`](https://github.com/t3rn/t3rn/commit/2825cd5b37483c17d5bbebfbe66f6d91ea7f4709) - add correct filter *(PR [#989](https://github.com/t3rn/t3rn/pull/989) by [@palozano](https://github.com/palozano))*


## [v1.25.0-rc.0] - 2023-05-18
### :sparkles: New Features
- [`f45fa58`](https://github.com/t3rn/t3rn/commit/f45fa580e3a44b0a3a88c36ddb8a4e801eb137a4) - distribute inflation rewards to block authors  *(PR [#966](https://github.com/t3rn/t3rn/pull/966) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.24.0-rc.0] - 2023-05-18
### :recycle: Refactors
- [`1311cd9`](https://github.com/t3rn/t3rn/commit/1311cd9d077d76a26447ddff7e511c8841818370) - sfx actions *(PR [#981](https://github.com/t3rn/t3rn/pull/981) by [@ahkohd](https://github.com/ahkohd))*


## [v1.23.0-rc.0] - 2023-05-17
### :sparkles: New Features
- [`6991de8`](https://github.com/t3rn/t3rn/commit/6991de886f51a092a48ad1c562fb2d723e2e13b6) - add support for enforce executioner field *(PR [#970](https://github.com/t3rn/t3rn/pull/970) by [@ahkohd](https://github.com/ahkohd))*
- [`6f95446`](https://github.com/t3rn/t3rn/commit/6f95446e54c3a6128659190d40cd5b022cfdbe63) - impl eth2 pallet *(PR [#961](https://github.com/t3rn/t3rn/pull/961) by [@petscheit](https://github.com/petscheit))*


## [v1.22.3-rc.0] - 2023-05-16
### :bug: Bug Fixes
- [`27c9b40`](https://github.com/t3rn/t3rn/commit/27c9b40b3e140a6f09213b6e5d2fa975bf99586c) - portal rpc in t0rn runtime *(PR [#978](https://github.com/t3rn/t3rn/pull/978) by [@petscheit](https://github.com/petscheit))*


## [v1.22.0-rc.0] - 2023-05-15
### :sparkles: New Features
- [`1e87f21`](https://github.com/t3rn/t3rn/commit/1e87f21435d4584fc28c9ffff7f3a3f226d73d38) - implement export extrinsic data to file *(PR [#969](https://github.com/t3rn/t3rn/pull/969) by [@ahkohd](https://github.com/ahkohd))*
- [`b5855ee`](https://github.com/t3rn/t3rn/commit/b5855ee38e7e361653ccd5ed9d784311f675b79d) - implement inflation rewards to attesters and executors *(PR [#918](https://github.com/t3rn/t3rn/pull/918) by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.21.0-rc.0] - 2023-05-12
### :sparkles: New Features
- [`6b24f01`](https://github.com/t3rn/t3rn/commit/6b24f019dde78e52301b5cad11f2fa74ea1287ab) - activate portal rpc service in t0rn runtime *(PR [#968](https://github.com/t3rn/t3rn/pull/968) by [@petscheit](https://github.com/petscheit))*


## [v1.20.1-rc.0] - 2023-05-12
### :bug: Bug Fixes
- [`9367cb7`](https://github.com/t3rn/t3rn/commit/9367cb71c174d4f18e40493f5092ed0ff02d6e1d) - expose portal rpc endpoint in t0rn runtime *(PR [#967](https://github.com/t3rn/t3rn/pull/967) by [@petscheit](https://github.com/petscheit))*


## [v1.20.0-rc.0] - 2023-05-12
### :sparkles: New Features
- [`897781b`](https://github.com/t3rn/t3rn/commit/897781b205ad49c3134df18b7c4741e21425dd0f) - standalone grandpa ranger *(PR [#914](https://github.com/t3rn/t3rn/pull/914) by [@petscheit](https://github.com/petscheit))*
- [`bf9fa77`](https://github.com/t3rn/t3rn/commit/bf9fa772c79c5bf506d91edee3481c1a6e21d5f0) - impl grandpa reset + xnds cleanup *(PR [#964](https://github.com/t3rn/t3rn/pull/964) by [@petscheit](https://github.com/petscheit))*


## [v1.19.2-rc.0] - 2023-05-11
### :bug: Bug Fixes
- [`770b2eb`](https://github.com/t3rn/t3rn/commit/770b2eb364f65ae686291c9634c1b76a6ba3ee1a) - executor docs build issue [skip release] *(PR [#959](https://github.com/t3rn/t3rn/pull/959) by [@ahkohd](https://github.com/ahkohd))*


## [v1.19.1-rc.0] - 2023-05-11
### :bug: Bug Fixes
- [`2486c27`](https://github.com/t3rn/t3rn/commit/2486c27e3a14b4531ff0367b2aa57dcc567de28a) - submit headers & bid process.exit *(PR [#952](https://github.com/t3rn/t3rn/pull/952) by [@3h4x](https://github.com/3h4x))*


## [v1.19.0-rc.0] - 2023-05-10
### :bug: Bug Fixes
- [`c287158`](https://github.com/t3rn/t3rn/commit/c28715874b7c73c32b8742061754a35a9423d4fb) - cli shoud not have set-operational [skip release] *(PR [#948](https://github.com/t3rn/t3rn/pull/948) by [@3h4x](https://github.com/3h4x))*

### :recycle: Refactors
- [`d79c770`](https://github.com/t3rn/t3rn/commit/d79c770cfae73e0f65bc64d3f38e9227c91f2ca8) - cli revamp *(PR [#881](https://github.com/t3rn/t3rn/pull/881) by [@3h4x](https://github.com/3h4x))*


## [v1.18.0-rc.0] - 2023-05-09
### :sparkles: New Features
- [`6ade20c`](https://github.com/t3rn/t3rn/commit/6ade20cb4e85127683f3cfe19008756b358f9624) - add maintenance mode to runtime *(PR [#882](https://github.com/t3rn/t3rn/pull/882) by [@palozano](https://github.com/palozano))*


## [v1.17.0-rc.0] - 2023-05-09
### :sparkles: New Features
- [`76ae0e5`](https://github.com/t3rn/t3rn/commit/76ae0e578eed48af8a10a1796b05fcf9dc19623b) - add fault tolerant ws connection handler *(commit by [@petscheit](https://github.com/petscheit))*
- [`a2f0c7f`](https://github.com/t3rn/t3rn/commit/a2f0c7f1de77c82a547df950f7c7323f52855c64) - portal rpc reimplemented *(commit by [@petscheit](https://github.com/petscheit))*
- [`b5ed30b`](https://github.com/t3rn/t3rn/commit/b5ed30bbdc664d0010478910e6ef68ed49dde4ea) - connect fetch_head_height to portal properly *(commit by [@petscheit](https://github.com/petscheit))*
- [`3f630bf`](https://github.com/t3rn/t3rn/commit/3f630bfb79b2d520b29f3105db2cdcc1eefaae68) - imple header collection and circuit submission logic *(commit by [@petscheit](https://github.com/petscheit))*
- [`7b739b2`](https://github.com/t3rn/t3rn/commit/7b739b2138cd6cae7386a0f1890743e59e018025) - impl prometheus metrics *(commit by [@petscheit](https://github.com/petscheit))*
- [`e1d0380`](https://github.com/t3rn/t3rn/commit/e1d038098fd809ef6780ceddfe0e0e2b12fc7522) - impl prometheus status endpoint *(commit by [@petscheit](https://github.com/petscheit))*
- [`a1b4d1c`](https://github.com/t3rn/t3rn/commit/a1b4d1c1fc852da4c02c05ea13b63b36223b708c) - impl dynamic http selection for circuit & add final prometheus counter *(commit by [@petscheit](https://github.com/petscheit))*

### :wrench: Chores
- [`87d35f5`](https://github.com/t3rn/t3rn/commit/87d35f5ef4b5c5078eb61ce2ce44be6d922cd83e) - checkin package files *(commit by [@petscheit](https://github.com/petscheit))*
- [`cbc7f72`](https://github.com/t3rn/t3rn/commit/cbc7f72c9f2e90e928e9dcba588e02dc40b10f98) - update lock *(commit by [@petscheit](https://github.com/petscheit))*
- [`9dbfd20`](https://github.com/t3rn/t3rn/commit/9dbfd20b7e41488154c528df3b17a20ab8e268cb) - impl change requests *(commit by [@petscheit](https://github.com/petscheit))*


## [v1.16.2-rc.0] - 2023-05-08
### :bug: Bug Fixes
- [`77a811c`](https://github.com/t3rn/t3rn/commit/77a811c609f267728c84a15723861e735aa8a48e) - remove incompatible executor tests *(commit by [@petscheit](https://github.com/petscheit))*

### :white_check_mark: Tests
- [`8e26283`](https://github.com/t3rn/t3rn/commit/8e26283bc79c9d968f918009fb1acc2e2b5ac8b3) - add tests to test-util *(commit by [@petscheit](https://github.com/petscheit))*

### :wrench: Chores
- [`a09e23f`](https://github.com/t3rn/t3rn/commit/a09e23f86e3326544c281c356151a802c0416f23) - removes old fixtures *(commit by [@petscheit](https://github.com/petscheit))*
- [`45370d6`](https://github.com/t3rn/t3rn/commit/45370d64b2a57f7f6f157b8282f08419621a7cd6) - rename testing util to replay *(commit by [@petscheit](https://github.com/petscheit))*


## [v1.16.0-rc.0] - 2023-05-08
### :sparkles: New Features
- [`5015a14`](https://github.com/t3rn/t3rn/commit/5015a148272366afd6ea8a04aa5699626f252146) - decode and export tx params *(commit by [@petscheit](https://github.com/petscheit))*
- [`ffdeeb9`](https://github.com/t3rn/t3rn/commit/ffdeeb98e0767667c0c654042ef35be04439466b) - add file export logic *(commit by [@petscheit](https://github.com/petscheit))*
- [`91f3ee6`](https://github.com/t3rn/t3rn/commit/91f3ee67f5c37bd29f29fc7347477a491cab0b5e) - add event to export functionality *(commit by [@petscheit](https://github.com/petscheit))*
- [`dbcc5a2`](https://github.com/t3rn/t3rn/commit/dbcc5a21f76c6002f70572201eedc1753d682e48) - add decoding, submission, and evaluation logic as new test util *(commit by [@petscheit](https://github.com/petscheit))*
- [`47caf63`](https://github.com/t3rn/t3rn/commit/47caf63e3390b02ff2edbed95225b8c5b60e5b1a) - impl block forwarding and address lookup logic *(commit by [@petscheit](https://github.com/petscheit))*
- [`a1ec253`](https://github.com/t3rn/t3rn/commit/a1ec253b3cda65ac0a7b85767c81ad12682a2adc) - impl demo test in portal *(commit by [@petscheit](https://github.com/petscheit))*

### :bug: Bug Fixes
- [`7fa4fe5`](https://github.com/t3rn/t3rn/commit/7fa4fe537664202875817ea2eff98387a6bac4b7) - missing import in circuit *(commit by [@petscheit](https://github.com/petscheit))*

### :wrench: Chores
- [`8263846`](https://github.com/t3rn/t3rn/commit/8263846d44b3f26cf71e75d33a84e7aeb5364c8f) - remove old fixtures *(commit by [@petscheit](https://github.com/petscheit))*
- [`ce3173a`](https://github.com/t3rn/t3rn/commit/ce3173aeb902547837d5c068cf64714eacdeca69) - align unchecked extrisics impl across runtimes *(commit by [@petscheit](https://github.com/petscheit))*
- [`0cef536`](https://github.com/t3rn/t3rn/commit/0cef536b70ffcc2a6fdcc3aeadd813ed0a8c19bd) - add sudo key to mock *(commit by [@petscheit](https://github.com/petscheit))*
- [`6f63066`](https://github.com/t3rn/t3rn/commit/6f63066952fc95c3fe203a1cbc3b3abaa7167acd) - align sudo pallet index between mock & standalone *(commit by [@petscheit](https://github.com/petscheit))*
- [`355fff6`](https://github.com/t3rn/t3rn/commit/355fff6bb2a38ad048589b4402817f207e99b4ab) - unused mock-data & add demo fixtures *(commit by [@petscheit](https://github.com/petscheit))*
- [`8b178f2`](https://github.com/t3rn/t3rn/commit/8b178f22ac2f9925069e7abaaa16d8cdfdd42492) - clippy *(commit by [@petscheit](https://github.com/petscheit))*
- [`cece16f`](https://github.com/t3rn/t3rn/commit/cece16f1553a26073640173c6e29a53a0ffff1cd) - remove deactivated e2e tests *(commit by [@petscheit](https://github.com/petscheit))*
- [`31aae1b`](https://github.com/t3rn/t3rn/commit/31aae1b575590be0f659c145646c6abca1136f56) - remove redundant comment line from standalone *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.15.0-rc.0] - 2023-05-03
### :sparkles: New Features
- [`894d5da`](https://github.com/t3rn/t3rn/commit/894d5daf12c7cacdbc68f1a2ca7b3429b462721a) - implement scaffold of Portal precompile to evm *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`e0c829c`](https://github.com/t3rn/t3rn/commit/e0c829c067fe34a0a431b93d1e6abff5161ff22a) - create and test ABI for portal precompile enum *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`81a71af`](https://github.com/t3rn/t3rn/commit/81a71af9bfcce040c06f4d1b00a1d3fcf89bc1e8) - implement Portal precompile based on Enum recoding *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`3c30442`](https://github.com/t3rn/t3rn/commit/3c30442b407c6fbee7b7355276e455bc2d4e434a) - implement PortalReadApi trait in Portal *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`2d8aa79`](https://github.com/t3rn/t3rn/commit/2d8aa79c4886ff91f2d63b60b55876d49799a18a) - add Portal Precompile to CustomPrecompiles utils *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`425f450`](https://github.com/t3rn/t3rn/commit/425f450930a08d5f8e5174c16be5e829d60f6318) - move portal precompile into 3vm *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`3383a6e`](https://github.com/t3rn/t3rn/commit/3383a6e0de41ffdf5284cc8e1a51cfbcdc0d6ad3) - connect evm to portal *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`76f5ff4`](https://github.com/t3rn/t3rn/commit/76f5ff43bfed0058eaff7e615aca0e2c0b0ab8a3) - add portal precompile *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`4c4b42e`](https://github.com/t3rn/t3rn/commit/4c4b42e7e681b9c2fabcf4d7374a1929a2d5d8bd) - make codec byte index more ergonomic *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*

### :bug: Bug Fixes
- [`b3db09e`](https://github.com/t3rn/t3rn/commit/b3db09e7b6443695cc0e034164807c92f469ba3f) - implement from over into trait for Portal::Inclusion *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`2c5dd6a`](https://github.com/t3rn/t3rn/commit/2c5dd6a39ce9f7c33a5801a1d2542e923b20b81c) - add missing imports after merge with development *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*

### :recycle: Refactors
- [`0b285a8`](https://github.com/t3rn/t3rn/commit/0b285a86fcb722e55e13452a18f2ab6e1729d51d) - move precompile to within the module *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*

### :white_check_mark: Tests
- [`5c2ebf5`](https://github.com/t3rn/t3rn/commit/5c2ebf56d08e326cf1ae4648a0adf52cbfe95f21) - add mini-mock runtime available for pallets to use *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`2d62ad8`](https://github.com/t3rn/t3rn/commit/2d62ad88948893b658cdb4756c38b37d258bebec) - add cases for from-rlp 32b words decoding to enum *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`92dd5c1`](https://github.com/t3rn/t3rn/commit/92dd5c1d641350c5718dc233c91c65d8ddb768b9) - make compile *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`fe5ae79`](https://github.com/t3rn/t3rn/commit/fe5ae7998f3bfd4802f7e1b95f52575cb2b79dc1) - fix invoke submit sfx 3vm precompile test exp *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`3f154f4`](https://github.com/t3rn/t3rn/commit/3f154f4252668bd630d7de2fedb7521278024881) - add confirm single SFX zombienet test *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`60fb346`](https://github.com/t3rn/t3rn/commit/60fb346d86f5ce27bd7d2fa03afc5a6cb3470529) - rewrite xtx confirm zombienet test to shell *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*

### :wrench: Chores
- [`96f9264`](https://github.com/t3rn/t3rn/commit/96f92646bba350311516dd8a39239e868674db06) - lint types files and correct empty buf ensure *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`66d6563`](https://github.com/t3rn/t3rn/commit/66d6563a889bdb71387d53ed44c593963ddb11d0) - turn off raw invocations for non-abi-enabled functionality *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`6cf8e45`](https://github.com/t3rn/t3rn/commit/6cf8e45a589e3205ff32b34fe0e9e2b4bcc6a4e3) - add logging to portal *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*


## [v1.14.0-rc.0] - 2023-05-03
### :recycle: Refactors
- [`edbbf13`](https://github.com/t3rn/t3rn/commit/edbbf13641f8ccd36d308793fa4994d7455438af) - rm redundant finalize + claim behaviour; optimize batch *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.13.0-rc.0] - 2023-04-30
### :sparkles: New Features
- [`7bd6dad`](https://github.com/t3rn/t3rn/commit/7bd6dad3027512220c6422521fa59c6e42b3d312) - add and check against SFX Speed Modes at confirm *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`3f13b16`](https://github.com/t3rn/t3rn/commit/3f13b16a0027e1ee0baf3c16b81b3004b34191b6) - extend 3vm local trigger with SpeedMode *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*

### :bug: Bug Fixes
- [`2ab8754`](https://github.com/t3rn/t3rn/commit/2ab87549c14f4a313806b9a86b67e97ad16ae8f2) - use xtx set speed mode at submission *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`574076c`](https://github.com/t3rn/t3rn/commit/574076c8e0ca91defaa16c2fc183de4f7f4f40b6) - roco cli config *(commit by [@petscheit](https://github.com/petscheit))*

### :recycle: Refactors
- [`9106118`](https://github.com/t3rn/t3rn/commit/91061180c8b1c5471d3cb67afdffc40ecadfb4c1) - return InclusionReceipt out of Portal and use for SpeedMode *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`97a87bc`](https://github.com/t3rn/t3rn/commit/97a87bc1ab87dadaca7034cf906a909bfc3c8fb1) - move speed modes check to LightClients *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*

### :white_check_mark: Tests
- [`1fea573`](https://github.com/t3rn/t3rn/commit/1fea573485362d7d715b33a7405131426a471e89) - change test params from seqential to speed modes *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`2960862`](https://github.com/t3rn/t3rn/commit/296086214edd33ded51e81b1834e707c018e1f7d) - cover speed mode satisfied with tests *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`f78e91d`](https://github.com/t3rn/t3rn/commit/f78e91d6eeebdd926361990e83c76e35c7ea85ad) - modify test-skip-verification flag positions in Circuit *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`a6980cd`](https://github.com/t3rn/t3rn/commit/a6980cd8cf4e90c4d3f429d32b6237f7c184b091) - rm test-skip-verification from default Circuit flags *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`3fcfd2e`](https://github.com/t3rn/t3rn/commit/3fcfd2e246d9a25eb447365d7b0b75a62a9502a4) - cover initialize, reads and speed_modes in Portal *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*

### :wrench: Chores
- [`51588dc`](https://github.com/t3rn/t3rn/commit/51588dcd570cd95f817ff451930186c3a0d0bb54) - add test-skip-verifcation feature flag and revamp confirm *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`8da6419`](https://github.com/t3rn/t3rn/commit/8da6419568ad36781001cc7bc529123525036baa) - lint project files *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`db5e1e5`](https://github.com/t3rn/t3rn/commit/db5e1e500be6f4c126c0d03da39e27868bf74ce5) - move SpeedModes to root primitives *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`dff3ab5`](https://github.com/t3rn/t3rn/commit/dff3ab5c65c07daa23eeeb3a616fe172065c6c9f) - rebuild client packages to support SpeedModes *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`1ac7dd9`](https://github.com/t3rn/t3rn/commit/1ac7dd92da01f8eb575e7fd616a4f0ce83721f23) - add test-skip-verification feature flag to CI *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`af90fea`](https://github.com/t3rn/t3rn/commit/af90fea561ec412dd3a90c9cc71e996a37561c22) - handle result of Assets::resolve in t0rn handle_credit *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`b1bf094`](https://github.com/t3rn/t3rn/commit/b1bf09442108c27ba69988cd5bc3c2a2878382bd) - decrease Fast Rational Finalised Grandpa offsets to 0 *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`5b3cbaa`](https://github.com/t3rn/t3rn/commit/5b3cbaa16786f7f9fb9c8db93b641073eaad628e) - fix client SFX confirmation with executors+rangers *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.12.4-rc.0] - 2023-04-26
### :wrench: Chores
- [`756c63e`](https://github.com/t3rn/t3rn/commit/756c63e09672cdd036377097371e17d1ce95b34b) - removes unused modules *(PR [#895](https://github.com/t3rn/t3rn/pull/895) by [@petscheit](https://github.com/petscheit))*


## [v1.12.0-rc.0] - 2023-04-22
### :sparkles: New Features
- [`e029213`](https://github.com/t3rn/t3rn/commit/e02921375708cb122fba828f81ae577698f025ab) - add compiler and deployer *(commit by [@petscheit](https://github.com/petscheit))*
- [`1cc003a`](https://github.com/t3rn/t3rn/commit/1cc003a43a65b27eb5679921d69b5f41873475cf) - add ballot contract, deploy args and transact *(commit by [@petscheit](https://github.com/petscheit))*

### :bug: Bug Fixes
- [`c59a7ae`](https://github.com/t3rn/t3rn/commit/c59a7ae58b43bbb1d04aaeca3bfe5229ca846c99) - append extra prefix memo byte only if specified in ABI SFX *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`067c99b`](https://github.com/t3rn/t3rn/commit/067c99bfaa0079596092513e7d18836a85f4c3e5) - remove signals from queue at processing by default *(PR [#666](https://github.com/t3rn/t3rn/pull/666) by [@petscheit](https://github.com/petscheit))*


## [v1.11.0-rc.0] - 2023-04-20
### :sparkles: New Features
- [`de93371`](https://github.com/t3rn/t3rn/commit/de93371c13451896e4078bbf9068b3bcc025f036) - expose entire light client trait in primitives *(PR [#848](https://github.com/t3rn/t3rn/pull/848) by [@petscheit](https://github.com/petscheit))*


## [v1.10.4-rc.0] - 2023-04-20
### :bug: Bug Fixes
- [`df07e2d`](https://github.com/t3rn/t3rn/commit/df07e2d8781ab54a5d6c905f00c766622a6e4f9f) - make extrinsics transactional *(PR [#857](https://github.com/t3rn/t3rn/pull/857) by [@petscheit](https://github.com/petscheit))*


## [v1.10.0-rc.0] - 2023-04-13
### :sparkles: New Features
- [`1968ab4`](https://github.com/t3rn/t3rn/commit/1968ab4a49e2c9815122afd9b81c5fd75597bec4) - add eth specific token type *(commit by [@petscheit](https://github.com/petscheit))*
- [`9fc5efa`](https://github.com/t3rn/t3rn/commit/9fc5efac1b7b09a0f1253db8f16020aa015474c0) - add execution vendor fiel *(commit by [@petscheit](https://github.com/petscheit))*
- [`186a95c`](https://github.com/t3rn/t3rn/commit/186a95cae15705975901c447f2a89b20c840fd6b) - add token execution vendor check and tests *(commit by [@petscheit](https://github.com/petscheit))*

### :recycle: Refactors
- [`859fc54`](https://github.com/t3rn/t3rn/commit/859fc540f282a652b59e737b58e7f2f7b52108d5) - cli registration and execution vendor selection in sdk *(commit by [@petscheit](https://github.com/petscheit))*

### :wrench: Chores
- [`0b6de3d`](https://github.com/t3rn/t3rn/commit/0b6de3da31fbc195202f7022d7ece82a2deb62c5) - remove preset gateways from standalone *(commit by [@petscheit](https://github.com/petscheit))*
- [`cc487f1`](https://github.com/t3rn/t3rn/commit/cc487f168c38c9932cefd5226bd4d3a0d436475e) - remove old files *(commit by [@petscheit](https://github.com/petscheit))*
- [`13533ea`](https://github.com/t3rn/t3rn/commit/13533eadf34dc2ccea27abf08ff6930529702b94) - fmt *(commit by [@petscheit](https://github.com/petscheit))*
- [`6902ef4`](https://github.com/t3rn/t3rn/commit/6902ef4243d1b1d5606ba883ad9512dfff8a127b) - update cli registration to conform with new fields *(commit by [@petscheit](https://github.com/petscheit))*
- [`ee21eae`](https://github.com/t3rn/t3rn/commit/ee21eae81b2614e6686f6b273e4bb254a3f95880) - sdk gateway loading restored *(commit by [@petscheit](https://github.com/petscheit))*
- [`69eabf1`](https://github.com/t3rn/t3rn/commit/69eabf1b31a4d1a232f89ffdb981db6ec73e6b9a) - update token_info fields, prepare cli for eth2 and add config *(commit by [@petscheit](https://github.com/petscheit))*
- [`5fe9959`](https://github.com/t3rn/t3rn/commit/5fe99594794b3b5ae5b2b7d2b55e0786758fe187) - fix pr comments *(commit by [@petscheit](https://github.com/petscheit))*


## [v1.9.1-rc.0] - 2023-04-11
### :bug: Bug Fixes
- [`87d9baf`](https://github.com/t3rn/t3rn/commit/87d9bafbcdb60302df2b97e5f9c75de11b0046d4) - update rust toolchain *(PR [#822](https://github.com/t3rn/t3rn/pull/822) by [@3h4x](https://github.com/3h4x))*


## [v1.9.0-rc.0] - 2023-04-11
### :sparkles: New Features
- [`5b2c7bb`](https://github.com/t3rn/t3rn/commit/5b2c7bb32f961f7703d38567776b683180037502) - add base call filter for t0rn runtime *(commit by [@palozano](https://github.com/palozano))*


## [v1.8.1-rc.0] - 2023-04-07
### :wrench: Chores
- [`b940d72`](https://github.com/t3rn/t3rn/commit/b940d7287599af8882b2af2b120ea9ca9808ba02) - extend GH linguist with package-lock and Cargo.lock *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.8.0-rc.0] - 2023-04-06
### :bug: Bug Fixes
- [`105eb9b`](https://github.com/t3rn/t3rn/commit/105eb9befa144c962e837efe37136e6444a23415) - update SFX ABI Standards to encode ingress in 2 bytes + fix tuples *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*

### :recycle: Refactors
- [`43592f8`](https://github.com/t3rn/t3rn/commit/43592f8f84aa11490094f627901fe315409e743c) - remove unwanted files *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*

### :white_check_mark: Tests
- [`9d1cfbe`](https://github.com/t3rn/t3rn/commit/9d1cfbe1b2e856d55aba80c4a9aec0b275c0299d) - handle unimplemented Abi cases in produce mock sfx *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*

### :wrench: Chores
- [`d414887`](https://github.com/t3rn/t3rn/commit/d414887f53bfc27af8e2f1fba4feeae61efc3a6a) - lint types files and correct empty buf ensure *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.7.1-rc.0] - 2023-04-05
### :bug: Bug Fixes
- [`2bc23f4`](https://github.com/t3rn/t3rn/commit/2bc23f40775ae54d2602b675e24e4d263c4ac4ef) - 3vm mocks *(PR [#783](https://github.com/t3rn/t3rn/pull/783) by [@petscheit](https://github.com/petscheit))*


## [v1.7.0-rc.0] - 2023-04-05
### :sparkles: New Features
- [`f2e523b`](https://github.com/t3rn/t3rn/commit/f2e523b40f7ddca78258f8c084451a28e9538962) - enable an extrinsic for executing SFX *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*

### :wrench: Chores
- [`74d45a0`](https://github.com/t3rn/t3rn/commit/74d45a0afab32f6ec8caccd1b3c6673f284e26a9) - add implementation for sfx2xbi *(commit by [@palozano](https://github.com/palozano))*
- [`6854fb7`](https://github.com/t3rn/t3rn/commit/6854fb76e760e5cc700891f015e6c4677a6c916f) - move fns around *(commit by [@palozano](https://github.com/palozano))*
- [`7616a65`](https://github.com/t3rn/t3rn/commit/7616a65b5204a3a05a38d3e2f93c29b6f77e1e4c) - make compileable *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`394d52f`](https://github.com/t3rn/t3rn/commit/394d52fae2cb5d97d42d5b951de8ac2f0d4cec40) - add aliq and swap *(commit by [@palozano](https://github.com/palozano))*
- [`5dfa403`](https://github.com/t3rn/t3rn/commit/5dfa403770fa387869956a66bf4a2d2342ead8d4) - update all sfx xbi instrucs *(commit by [@palozano](https://github.com/palozano))*
- [`41cb7c3`](https://github.com/t3rn/t3rn/commit/41cb7c31d6a7f4dcc461288fcbfdb3116346f18d) - add non passing test *(commit by [@palozano](https://github.com/palozano))*
- [`20fcd81`](https://github.com/t3rn/t3rn/commit/20fcd8159ec0a1cc1a90311f164dfe9a29d786ed) - add test but missing import *(commit by [@palozano](https://github.com/palozano))*
- [`1caccd7`](https://github.com/t3rn/t3rn/commit/1caccd7053be6a043ae00629ca9dae1a03f8b595) - remove unused imports *(commit by [@palozano](https://github.com/palozano))*
- [`148d4d9`](https://github.com/t3rn/t3rn/commit/148d4d99ab9310d4dc97595d1398e838c3f7ef86) - fix test compilation and failing test *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`cc346f3`](https://github.com/t3rn/t3rn/commit/cc346f3d2800cd66d694bcd343c224b1353602e6) - stylistic changes to tryinto *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`0a51538`](https://github.com/t3rn/t3rn/commit/0a51538dc32ecdd621c2f5a2c333373e0b3dbd89) - add note on crazy imports *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`5c2311f`](https://github.com/t3rn/t3rn/commit/5c2311ffed3ab576ace7d69463032940d612925d) - writing tests for sfx abi conversion *(commit by [@palozano](https://github.com/palozano))*
- [`50ab4fb`](https://github.com/t3rn/t3rn/commit/50ab4fb09f16063d2e4f13b546585e5a12e2a5fa) - add the lock file *(commit by [@palozano](https://github.com/palozano))*
- [`2261139`](https://github.com/t3rn/t3rn/commit/226113914843105494c333ddd244b52dd551cd62) - add missing tests *(commit by [@palozano](https://github.com/palozano))*
- [`064d65f`](https://github.com/t3rn/t3rn/commit/064d65f38481f6bb93dd0594a9b9500e1980496c) - fix formating *(commit by [@palozano](https://github.com/palozano))*
- [`315c5d0`](https://github.com/t3rn/t3rn/commit/315c5d0b7eeecdbae676b9cf59841bb2fc0e86fa) - add missing std *(commit by [@palozano](https://github.com/palozano))*
- [`7d9a0f1`](https://github.com/t3rn/t3rn/commit/7d9a0f1822365c0e1d606e977b92e43fd9257a10) - missing stds *(commit by [@palozano](https://github.com/palozano))*
- [`868d2ea`](https://github.com/t3rn/t3rn/commit/868d2eabcd1618701370b403fe1e0e2ae4f35f8b) - clean debug error enum *(commit by [@palozano](https://github.com/palozano))*
- [`43a828c`](https://github.com/t3rn/t3rn/commit/43a828c708932549121aaaee8f76996558e96a29) - add more tests *(commit by [@palozano](https://github.com/palozano))*


## [v1.6.0-rc.0] - 2023-04-04
### :sparkles: New Features
- [`1ae8a3e`](https://github.com/t3rn/t3rn/commit/1ae8a3e0fdfbc83d76dc44cebdacf82763f3bc51) - re-expose register as extrinsic *(commit by [@petscheit](https://github.com/petscheit))*
- [`04dca3b`](https://github.com/t3rn/t3rn/commit/04dca3bde029f86cd2d70d69329d7ab8facf1ec7) - move grandpa header submission extrinsic out of portal and expose in FV *(commit by [@petscheit](https://github.com/petscheit))*
- [`6735beb`](https://github.com/t3rn/t3rn/commit/6735bebe77fdef4761d163c95b94d9e5ea93295c) - emit header add event from grandpa fv *(commit by [@petscheit](https://github.com/petscheit))*
- [`c8765be`](https://github.com/t3rn/t3rn/commit/c8765be3f3d7e905bd99fc0ad1917cdd1c327384) - add full record endpoint to xdns, including token entries *(commit by [@petscheit](https://github.com/petscheit))*

### :bug: Bug Fixes
- [`288f5a7`](https://github.com/t3rn/t3rn/commit/288f5a7120cce14923e6a094305f7cb5c0690dd5) - restore CLI registration *(commit by [@petscheit](https://github.com/petscheit))*

### :wrench: Chores
- [`bb84147`](https://github.com/t3rn/t3rn/commit/bb841474fc87e972229e96926fcac4970c25a3b9) - impl event type to t0rn runtime *(commit by [@petscheit](https://github.com/petscheit))*
- [`10f85f5`](https://github.com/t3rn/t3rn/commit/10f85f56e1bb2172e78808c1d2f9174217c49727) - remove seed gateways *(commit by [@petscheit](https://github.com/petscheit))*
- [`e64a7a4`](https://github.com/t3rn/t3rn/commit/e64a7a4749ba52f3332ce531612548c74c3a7829) - restore header submission and sfx creation *(commit by [@petscheit](https://github.com/petscheit))*


## [v1.5.0-rc.0] - 2023-04-03
### :sparkles: New Features
- [`2277b41`](https://github.com/t3rn/t3rn/commit/2277b41cff515453329d40df26a73af5c6dd49b7) - extend XDNS with SeenSFX interface storing ABI *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`9b624a5`](https://github.com/t3rn/t3rn/commit/9b624a589463dd4097e3e0bbf5fd400f6bfcb595) - extend portal interfaces with no decode, Polkadot and Kusama *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`29148af`](https://github.com/t3rn/t3rn/commit/29148afa79a01e16919e241ae98e4558eed66623) - implement Kusama and Polkadot bridges to runtimes *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`15e24f2`](https://github.com/t3rn/t3rn/commit/15e24f20aa1372eab8516e4354df755e28f97475) - reintroduce SFXAbi with recoding between Scale and RLP *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`6190229`](https://github.com/t3rn/t3rn/commit/619022942933a7260bc4fd8e668c17c0bb379cfd) - add SFXAbi handles to XDNS to enable dynamic validation *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`ac16f89`](https://github.com/t3rn/t3rn/commit/ac16f89cf8a681d13312367551a07bf8e435206a) - update Portal interface to suppport ABI Recoding *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`5d4c5a0`](https://github.com/t3rn/t3rn/commit/5d4c5a0519f1215f2fa14327d35a5fd9761b4293) - extend ABI with ingress Eth events recoding *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`22ef71b`](https://github.com/t3rn/t3rn/commit/22ef71b565fd86cfa8b0452c084222f4914c8ca6) - SFX Abi validates ingress from Substrate and Eth *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`f77fa43`](https://github.com/t3rn/t3rn/commit/f77fa433d7e4c50c36d2d8a956ba50d656de30ec) - separate t3rn ABI to separate module, fix deps *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`70489dd`](https://github.com/t3rn/t3rn/commit/70489ddf846b56b1945b3a1b727a65863d7b7b8e) - standard SFX ABI use recode_as for payload validation *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`cf67b34`](https://github.com/t3rn/t3rn/commit/cf67b34663ad4a3df1f8cd3b5a3b7d0b6a756ba6) - add egress SFX Abi validation *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`ccd0a76`](https://github.com/t3rn/t3rn/commit/ccd0a76b9aa81dcb66fe7a083f4ad24673152cb6) - switch SFX validation + confirmation to use latest ABI *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`463f9df`](https://github.com/t3rn/t3rn/commit/463f9df2f4a5b0501a574c34f84b730f05a2380d) - improve esm build *(commit by [@ahkohd](https://github.com/ahkohd))*
- [`7d23d36`](https://github.com/t3rn/t3rn/commit/7d23d365f67d8d1bd2042e7167eaa9f2e72656bc) - enable optional submission height check *(commit by [@petscheit](https://github.com/petscheit))*
- [`eb8ddd4`](https://github.com/t3rn/t3rn/commit/eb8ddd4e12d3fe2cef0ae41af5be9fcc66d04b05) - implement LightClient trait by Grandpa FV *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`d50f335`](https://github.com/t3rn/t3rn/commit/d50f335360165449e4d3efab451ee2aa8cfff9a1) - implement light client commons traits *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`9e74479`](https://github.com/t3rn/t3rn/commit/9e74479737bff66756f536588d643b0af6ce6d4b) - implement LightClient trait for multiple GrandpaFV *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`fcb50ed`](https://github.com/t3rn/t3rn/commit/fcb50ed29fc6bd7c803c42ae028ca753005127e1) - remove deprecated XDNSRPCRecord field *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*

### :bug: Bug Fixes
- [`b447384`](https://github.com/t3rn/t3rn/commit/b44738461a453ed507a1c139313e6be6f97bc85b) - limit ranger submission batch size to 10 *(commit by [@petscheit](https://github.com/petscheit))*
- [`a3ed12c`](https://github.com/t3rn/t3rn/commit/a3ed12c4d2fc76a3b6b17e1046cfc2f110fdcd11) - add memo prefix before SFX ABI validation to args *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`19587eb`](https://github.com/t3rn/t3rn/commit/19587eb379c408504525173691e7f81e1bc7de4b) - fix rlp_topics decode for reversed tuple,vec,struct *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`b0a4f2a`](https://github.com/t3rn/t3rn/commit/b0a4f2a8590584a03c697f63ad1724003f57d0de) - commit remaining changes after rebase with origin *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`2395cd6`](https://github.com/t3rn/t3rn/commit/2395cd6e8a7f69ab2dfc4479d70079985db5f130) - extend SFXAbi with prefix_memo field check for Polka Events *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`6102a7f`](https://github.com/t3rn/t3rn/commit/6102a7fc6520c5a5dd0c96e98d671ade3d1c7b6b) - round information was never bumped *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`b90b4c4`](https://github.com/t3rn/t3rn/commit/b90b4c45fc9869132706939ba2b362d1786b5b3d) - add missing init value for update freq in price engine *(commit by [@palozano](https://github.com/palozano))*
- [`288e0e9`](https://github.com/t3rn/t3rn/commit/288e0e9e171803be0c58a892a873172ee5c056dc) - add flag to not update prices *(commit by [@palozano](https://github.com/palozano))*

### :recycle: Refactors
- [`dcae08e`](https://github.com/t3rn/t3rn/commit/dcae08e1b948358c7ddbd4f4114ada357f78e6f5) - add and use dedicated SFX confirmation handle for Circuit Machine *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`9e42f45`](https://github.com/t3rn/t3rn/commit/9e42f45c6c80667614613c998fa47c7ae00a093e) - use SFXAbi to confirm SFX exec in Circuit *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`8cdd510`](https://github.com/t3rn/t3rn/commit/8cdd510e6f5cb01a4ccc88fe4d533502ac688f2e) - separate recode for RLP and Scale to separate modules *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`ee0cc48`](https://github.com/t3rn/t3rn/commit/ee0cc48c03eabfae704b64cd9ffea3697c74eec4) - use lastest SFX Abi in primitives *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`9514e69`](https://github.com/t3rn/t3rn/commit/9514e6951ffe79e2dd1a5baa127ff2e365e22dde) - align Circuit SFX tests and validation with latest ABI *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`d0e5a50`](https://github.com/t3rn/t3rn/commit/d0e5a502f950ed3cfabdbb19d8b29270a7a511a2) - switch pallets, runtimes, nodes to latest SFX ABI *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`0a92cd4`](https://github.com/t3rn/t3rn/commit/0a92cd4e4d9fd544264c711e541499e54016de78) - rely on bytes crate vs split_bytes in ABI *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`0544df8`](https://github.com/t3rn/t3rn/commit/0544df88e9e2c92729e8c8dd02079db47a792bff) - remove the use of trim_bytes and rely on slicing in ABI *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`19e2308`](https://github.com/t3rn/t3rn/commit/19e2308929dbf031e551a816d2f971a3eab895f9) - rewrite rlp topic decoding without take_last_n *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`1ba2963`](https://github.com/t3rn/t3rn/commit/1ba29634f034a8c007d69de86b8d6384b65941eb) - remove unsafe unwrap use in ABI *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`b2e09f3`](https://github.com/t3rn/t3rn/commit/b2e09f371bad10b53abcdc6ef6079477397c6332) - introduce Tokens and Gateways to XDNS *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`595f152`](https://github.com/t3rn/t3rn/commit/595f1521b6a333ebc30e2581634478bbf8c29b04) - remove operational methods out of Portal *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`e141943`](https://github.com/t3rn/t3rn/commit/e1419438205164209168f83d1d94cde915066930) - introduce new LightClient trait to primitives *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`fb367de`](https://github.com/t3rn/t3rn/commit/fb367de9ea04aea338e04d8abe8160facb2c5c1d) - create new Token and Gateway entries to XDNS *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`617e95b`](https://github.com/t3rn/t3rn/commit/617e95b7f0c2d74e8d35a5b3650604bcdd0b15b0) - rewrite Portal to align with LightClient interface *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`bbf1d8a`](https://github.com/t3rn/t3rn/commit/bbf1d8a5559277e754b5607847d0c2dd3d948dea) - try implement new LightClient interface to mock *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`d1a7739`](https://github.com/t3rn/t3rn/commit/d1a77390392154b4e95d8eba92c1718db5d6a31b) - breaking! change FSX::submitted_at to blocknumber *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`c9bb2a0`](https://github.com/t3rn/t3rn/commit/c9bb2a0f11ed9c6fe48a432e7b47a162c1274bd3) - use grandpa in Portal via LightClient trait and test *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`07b1f35`](https://github.com/t3rn/t3rn/commit/07b1f350b6d7244b796b9478312096300fa5383c) - re-export LightClient trait out of primitives *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`d341e31`](https://github.com/t3rn/t3rn/commit/d341e31ce10930ae48c9d62ccbb6b5e94ec5b93b) - elevate LightClient selector from Portal to Runtime *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`64d6030`](https://github.com/t3rn/t3rn/commit/64d6030b066180cf100ad64f02a9d2a790084842) - change Portal to select Light Client from runtime *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`abd1754`](https://github.com/t3rn/t3rn/commit/abd1754386798e2cbe82b1e0c801b5b1307a39c1) - remove update_ttl and security_lvl from XDNS *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`98dfff3`](https://github.com/t3rn/t3rn/commit/98dfff389ab898d1c34f00baf4c30f6530300eb9) - update LightClient trait not to rely on Option *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`844ec84`](https://github.com/t3rn/t3rn/commit/844ec848b390344f0784b7768d00f83c7b4a6694) - rename options in Header and Height Results *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`99269a9`](https://github.com/t3rn/t3rn/commit/99269a97141cd52787d309394ee6050021df8d67) - add new initialize gateway token to Portal *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`d11844f`](https://github.com/t3rn/t3rn/commit/d11844f135973ba91b13a519f6e57ee4c64c3851) - xdns fetch_records reads gateway records *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*

### :white_check_mark: Tests
- [`57cc514`](https://github.com/t3rn/t3rn/commit/57cc514cb533fe3d08ba48e184f0d326bb00808f) - add Polka and Kusama LightClients to 3VM mock *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`aa72280`](https://github.com/t3rn/t3rn/commit/aa72280ea9dc4b453cebf1896f839e8aef0485e1) - add Polka and Kusama LightClients to EVM mock *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`c4a94da`](https://github.com/t3rn/t3rn/commit/c4a94dac63444b5ea37b1d0effce60f0e8ce3041) - rewrite prodice test sfx args in mock with latest ABI *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`9a5a8b7`](https://github.com/t3rn/t3rn/commit/9a5a8b798bc31baddf9032e69ab67a9ae24c7de2) - fix Circuit tests after using latest SFX ABI *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`75a3f16`](https://github.com/t3rn/t3rn/commit/75a3f16d3f5415afc7833640db420d0edd945e41) - fix portal tests by adding default XDNS entries *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`46cbeb5`](https://github.com/t3rn/t3rn/commit/46cbeb5fd72d0803d7f17584549c079042d1e6a3) - test StandardSideEffects -> StandardSFXABIs storage migration *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`2639306`](https://github.com/t3rn/t3rn/commit/263930631b7c02d2fdc1d95fb049356ba9d67fbe) - cover ABI decode_topics_as_rlp with unit tests *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`9017ca3`](https://github.com/t3rn/t3rn/commit/9017ca3e11366aabf8ba96bfa7caf8c6d1046dcc) - re-compile XDNS tests after API changes *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`b7d077a`](https://github.com/t3rn/t3rn/commit/b7d077a5323176bf64bf9480c4c3b06284afbc00) - convert primitives Xtx::FSX submission height to u32 *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`d2701c7`](https://github.com/t3rn/t3rn/commit/d2701c7e67418420f67cdf7f775eaa604cb367e7) - implement new traits for GrandpaFV to contracts *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`fe7798c`](https://github.com/t3rn/t3rn/commit/fe7798c708388cee8daacddda57701365a52bb38) - unify block_number in mocks to u32 *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`01b9414`](https://github.com/t3rn/t3rn/commit/01b9414277fac582ce7a4ac7c692974d695f5432) - unify all pallets test to use block_no as u32 *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`1a0ffa4`](https://github.com/t3rn/t3rn/commit/1a0ffa47002d60c65c56976d2eac270f3a7033a9) - add all allowed SFX to mock [0,0,0,0] gateway *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`a029bde`](https://github.com/t3rn/t3rn/commit/a029bde633a79ee5a15e55b35180405f330e4fea) - correct assertion to 4b length in 3vm wasm test *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`c4568e8`](https://github.com/t3rn/t3rn/commit/c4568e8eb08a2770703a2039fbd81a3835480681) - fix test expecting HeightResult enum *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*

### :wrench: Chores
- [`5777797`](https://github.com/t3rn/t3rn/commit/5777797678d7b098c79777bfb1bbdabf8203bcb4) - add codecov *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`352c6ba`](https://github.com/t3rn/t3rn/commit/352c6ba0ad375b6744aa2e29f0cd6bf048f70518) - remove unused files from types *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`5f8b416`](https://github.com/t3rn/t3rn/commit/5f8b41639cadb6f5bcc7f79073e9deb91bc182c1) - remove remaining SFX::encoded_action use *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`9108ede`](https://github.com/t3rn/t3rn/commit/9108ede046d5b3f6cdd57100a14f8ff6d158fd96) - remove Box::leak replaced with debug msg, fix std tests *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`ba1fc3f`](https://github.com/t3rn/t3rn/commit/ba1fc3f0703c9ee3db4bd99027465aef157e769b) - support multiple StorageMigrations in XDNS *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`494cebf`](https://github.com/t3rn/t3rn/commit/494cebf8e7b4f0cbb47e02599e4fe2ae7c84b7b5) - remove deps on grandpa_FV from primitives *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`5a2ef48`](https://github.com/t3rn/t3rn/commit/5a2ef48e21b10a6da59210dc6d7a8241e148df8f) - remove wrongly merged client types interfaces *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`3d64c4a`](https://github.com/t3rn/t3rn/commit/3d64c4a97a3f3e971c9dd7a7c9a770fc08ab56a1) - remove test package *(commit by [@ahkohd](https://github.com/ahkohd))*
- [`e3dae36`](https://github.com/t3rn/t3rn/commit/e3dae36c543de6cb4bfea70920ddc5e4c9d425a3) - resolve merge conflicts *(commit by [@petscheit](https://github.com/petscheit))*
- [`5613e45`](https://github.com/t3rn/t3rn/commit/5613e45e3ea8010c491b2ec34790d125c764ad34) - fix registration params *(commit by [@petscheit](https://github.com/petscheit))*
- [`83200a3`](https://github.com/t3rn/t3rn/commit/83200a3f4a8ecd33425984bbf5f01d18f03a326b) - rm unused files *(commit by [@petscheit](https://github.com/petscheit))*
- [`da5181d`](https://github.com/t3rn/t3rn/commit/da5181d8fdd7459ff2c54d263bb36c349b485ad1) - fix sfx types *(commit by [@petscheit](https://github.com/petscheit))*
- [`a97166f`](https://github.com/t3rn/t3rn/commit/a97166f02e85202a8027a5f36941c791a089ed15) - adds palletIndex field to tran abi *(commit by [@petscheit](https://github.com/petscheit))*
- [`ccee54a`](https://github.com/t3rn/t3rn/commit/ccee54a1f3f9aa93fe7079f94ec4bacbd0f0f088) - fix abi test *(commit by [@petscheit](https://github.com/petscheit))*
- [`0f25382`](https://github.com/t3rn/t3rn/commit/0f2538266f4a20e08bcfd15415345a9210f41002) - update ranger update policy to release *(commit by [@petscheit](https://github.com/petscheit))*
- [`a6d63d0`](https://github.com/t3rn/t3rn/commit/a6d63d0afcfd7118fc1241efdfa8519a5ddcb44d) - rm unneeded files *(commit by [@petscheit](https://github.com/petscheit))*
- [`d3cf062`](https://github.com/t3rn/t3rn/commit/d3cf062ecfde6147d8f01298d2ba0dcbefdfc2a6) - cleanup *(commit by [@petscheit](https://github.com/petscheit))*
- [`1ea99d6`](https://github.com/t3rn/t3rn/commit/1ea99d6c3ac410b648c0e022751184c740df4953) - rm prefix byte *(commit by [@petscheit](https://github.com/petscheit))*
- [`534663a`](https://github.com/t3rn/t3rn/commit/534663a8490e8cbb63a7e0fe7f12b60168fd4a0f) - add a frequency to update the prices *(commit by [@palozano](https://github.com/palozano))*
- [`7fa08ca`](https://github.com/t3rn/t3rn/commit/7fa08ca66d3c40c1277c38d380b8e66feae29685) - move endpoint args to config *(commit by [@palozano](https://github.com/palozano))*
- [`990c352`](https://github.com/t3rn/t3rn/commit/990c352a1f0eb73c8b05c247d2acc3613622a0f3) - create .github/dependabot.yml *(commit by [@3h4x](https://github.com/3h4x))*
- [`4cb46df`](https://github.com/t3rn/t3rn/commit/4cb46df97a15eaa09f813f9a89e6aaf51b5b3333) - stringify SFX ids in debug message and add 2 dev Gateways *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.4.0-rc.0] - 2023-03-29
### :boom: BREAKING CHANGES
- due to [`1c5d335`](https://github.com/t3rn/t3rn/commit/1c5d33553229d05a1810c0319c6184f3528d9e8c) - sfx action field renamed and type updated *(commit by [@petscheit](https://github.com/petscheit))*:

  renamed encoded_action to action and updated type to [u8; 4]


### :sparkles: New Features
- [`b1cd339`](https://github.com/t3rn/t3rn/commit/b1cd3394657fba3db99580c7788d1db869f7e6e5) - install xbi 0.9.37 *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*

### :bug: Bug Fixes
- [`90fda5f`](https://github.com/t3rn/t3rn/commit/90fda5fcf7f0033cb44a666218241595fbfb7a69) - cli type error *(commit by [@petscheit](https://github.com/petscheit))*
- [`d0a5598`](https://github.com/t3rn/t3rn/commit/d0a5598aa95bb8f045d30dd5b7a044bfb2a3e725) - **clock**: round info config should be every 400 blocks *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`3a672bd`](https://github.com/t3rn/t3rn/commit/3a672bdfe4fbebd7770e94523575662cd6e603ee) - **clock**: actually check before releasing a round info *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*

### :recycle: Refactors
- [`9e7994e`](https://github.com/t3rn/t3rn/commit/9e7994e652ac4eb3fae19383b32da20121350058) - await wasm crypto ready *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`1c5d335`](https://github.com/t3rn/t3rn/commit/1c5d33553229d05a1810c0319c6184f3528d9e8c) - **API**: sfx action field renamed and type updated *(commit by [@petscheit](https://github.com/petscheit))*

### :white_check_mark: Tests
- [`4bf24d7`](https://github.com/t3rn/t3rn/commit/4bf24d74ff992b39404de0db795fa9d08c62b4cd) - add t0rn back to zombienet *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`5d964d5`](https://github.com/t3rn/t3rn/commit/5d964d556a7313b590be06acc28c8f056894d460) - migrate zombienet to scripts *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`51c81a0`](https://github.com/t3rn/t3rn/commit/51c81a0ea086339e824df6ca8f211fd40dea8b22) - single xtx deletion weight is not affected by round anymore *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*

### :wrench: Chores
- [`904a026`](https://github.com/t3rn/t3rn/commit/904a026ba0126ee2851c1c09d50c1f6e6f83206e) - executors should connect to ws t0rn *(PR [#691](https://github.com/t3rn/t3rn/pull/691) by [@3h4x](https://github.com/3h4x))*
- [`af87a71`](https://github.com/t3rn/t3rn/commit/af87a71734ba4b2783d0bbe7a0b06c1c62359542) - bump dns-packet from 5.3.1 to 5.4.0 in /docs/main *(commit by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`666aec5`](https://github.com/t3rn/t3rn/commit/666aec585434b63bec17dd66cca2947401362ba9) - revamp ts types package *(PR [#705](https://github.com/t3rn/t3rn/pull/705) by [@petscheit](https://github.com/petscheit))*
- [`2e4bdab`](https://github.com/t3rn/t3rn/commit/2e4bdab7ea773c532c00f02e7890018db1c1b5ab) - bump madge from 3.12.0 to 4.0.1 in /client/packages/types *(commit by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`66d3d9b`](https://github.com/t3rn/t3rn/commit/66d3d9bc477e77453fab2911ba902e86ab780baf) - bump webpack from 5.72.1 to 5.76.1 in /docs/main *(commit by [@dependabot[bot]](https://github.com/apps/dependabot))*


## [v1.3.0-rc.0] - 2023-03-29
### :boom: BREAKING CHANGES
- due to [`a665590`](https://github.com/t3rn/t3rn/commit/a665590c85e5d0c7b343bc52acd848469d6a50df) - xbi portal, remove escrowed & fixes to dependencies *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*:

  balance of has been removed  
  escrowed has been removed  
  * chore: pr comments  
  * chore: remove frontier  
  * chore: lock to #07cd855d  
  * docs: add setup to readme


### :sparkles: New Features
- [`dd705f0`](https://github.com/t3rn/t3rn/commit/dd705f03363e67bba4436974c2284ad9b08a38ba) - update event emission and adds test *(commit by [@petscheit](https://github.com/petscheit))*
- [`9a35df4`](https://github.com/t3rn/t3rn/commit/9a35df41ec43693d2e964abbcf81e67a7535bfc9) - add optional SFX field for reward asset id *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`6726b65`](https://github.com/t3rn/t3rn/commit/6726b654ccec969bb92bf7e1803ac78569498ba8) - add optional asset id to AccountManager deposits *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`a93005e`](https://github.com/t3rn/t3rn/commit/a93005eef4bd6f27e6661fd8290a5461f5fa2d3d) - extend AccountManager with monetary handling assets and native *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`5b885c3`](https://github.com/t3rn/t3rn/commit/5b885c369ec8894df75a5e9026f8fc5c8d60ebbc) - use monetary submodule to reserve SFX requesters *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`9464fe6`](https://github.com/t3rn/t3rn/commit/9464fe68cb2d4f3397fa70a3076f3b7d4e7f7ac6) - extend SFXBid with reserved asset field *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`de6bda9`](https://github.com/t3rn/t3rn/commit/de6bda97d770084443939e1f9f605dd3b5694758) - slash/repatriate optimistic SFX to executors with foreign assets *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`653e0b6`](https://github.com/t3rn/t3rn/commit/653e0b62e8d09dfb358dce20dfee0815f8fe097f) - use multiasset monetary for optimistic dropped bids *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`542bbea`](https://github.com/t3rn/t3rn/commit/542bbea88dbcf79a183162c7a0ff83d7d62251f4) - all runtimes incl a parity treasury *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`b1112d4`](https://github.com/t3rn/t3rn/commit/b1112d45185e50efe30d60d7ea4aad83231543a6) - safe math operations *(commit by [@palozano](https://github.com/palozano))*
- [`0326488`](https://github.com/t3rn/t3rn/commit/03264883e18b59d077e4a2da3b5891417cca3890) - change monetary deposits to infallible with Unbalanced *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`a665590`](https://github.com/t3rn/t3rn/commit/a665590c85e5d0c7b343bc52acd848469d6a50df) - xbi portal, remove escrowed & fixes to dependencies *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`dceb2db`](https://github.com/t3rn/t3rn/commit/dceb2db083f6f6918a27956371be01c637e9788a) - add subalfred gh action *(PR [#579](https://github.com/t3rn/t3rn/pull/579) by [@3h4x](https://github.com/3h4x))*
- [`8231c75`](https://github.com/t3rn/t3rn/commit/8231c75dfd8c41683d1f2d9aa48ea0368380bd6c) - add pr title lint *(commit by [@3h4x](https://github.com/3h4x))*
- [`eb5294d`](https://github.com/t3rn/t3rn/commit/eb5294d9575e1215c06068227703117b340acff8) - update subalfred false positives list *(commit by [@3h4x](https://github.com/3h4x))*
- [`fb8f117`](https://github.com/t3rn/t3rn/commit/fb8f1172112073d5b09fd296e92c5e1d95bf5c54) - conventional changelog GHA creating tag and markdown file  *(PR [#587](https://github.com/t3rn/t3rn/pull/587) by [@3h4x](https://github.com/3h4x))*
  - :arrow_lower_right: *addresses issue [#63](undefined) opened by [@AwesomeIbex](https://github.com/AwesomeIbex)*
- [`c20fbbe`](https://github.com/t3rn/t3rn/commit/c20fbbec5ecdfb2820788b70a34124e062816bd3) - github action stale issues *(commit by [@3h4x](https://github.com/3h4x))*
- [`de82d22`](https://github.com/t3rn/t3rn/commit/de82d2299b9db3979ac1fbd7873e9c7a00df69b5) - add Dockerfile for executor *(commit by [@3h4x](https://github.com/3h4x))*
- [`27c8cd7`](https://github.com/t3rn/t3rn/commit/27c8cd7a1a6935239adf0b69474590d2dd75a042) - finish clean up *(commit by [@palozano](https://github.com/palozano))*
- [`01760e0`](https://github.com/t3rn/t3rn/commit/01760e0710625fa800f9855f7c03fcd3d86ad320) - add GHA for tests *(commit by [@3h4x](https://github.com/3h4x))*
- [`f01d19e`](https://github.com/t3rn/t3rn/commit/f01d19e57c36c68908796230a015e2dd4d7205d5) - enable try-runtime and benchmarks in mainnet *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`2e341a6`](https://github.com/t3rn/t3rn/commit/2e341a6ebdb9c043d809b1399c8803b111dcf092) - ensure parachain has some movement in relay chain block height *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`242ffad`](https://github.com/t3rn/t3rn/commit/242ffad1150a39f89dbedba00adeb35007433512) - fix try-runtime and endowed accounts to be less rigid *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`05cd854`](https://github.com/t3rn/t3rn/commit/05cd8547ac60fae6fe81615938a31347726dfc2d) - expose xbi to executors *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`f6659e0`](https://github.com/t3rn/t3rn/commit/f6659e099923b3b822b0eb26a1828d5dfa0994c0) - foreign fee split *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`84de0ab`](https://github.com/t3rn/t3rn/commit/84de0ab6c0be354fffc854adb5f0189dbfa011ba) - introduce OnHook traits to clock *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`cbb23a8`](https://github.com/t3rn/t3rn/commit/cbb23a8758c39f3ca30dea254d508b02c66f802a) - add logging for ranger monitoring *(commit by [@petscheit](https://github.com/petscheit))*
- [`d2071ee`](https://github.com/t3rn/t3rn/commit/d2071eeb15ea3ccd648235f2114a24c548507a55) - standalone Dockerfile *(commit by [@3h4x](https://github.com/3h4x))*
- [`bd21797`](https://github.com/t3rn/t3rn/commit/bd217974f354477594b11fa40e9c6bd82d2c889b) - added inital workflow for building docker *(commit by [@3h4x](https://github.com/3h4x))*
- [`0bd169e`](https://github.com/t3rn/t3rn/commit/0bd169e2ae0a54d81f87f649896c9460b4f51ccb) - multistage docker build for node standalone *(commit by [@3h4x](https://github.com/3h4x))*
- [`313aa8c`](https://github.com/t3rn/t3rn/commit/313aa8c39244e537d2b2ad622f0a6e6118ede6ac) - implement confirm() and validate() as part of SFX struct *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`c1314e7`](https://github.com/t3rn/t3rn/commit/c1314e73785f8dec49b57e97a33418fd09d24b2a) - test harness with parachains integration tool *(PR [#629](https://github.com/t3rn/t3rn/pull/629) by [@palozano](https://github.com/palozano))*

### :bug: Bug Fixes
- [`c734d46`](https://github.com/t3rn/t3rn/commit/c734d466aad47091b0429b2db1d9841d2835301a) - bidding for multiple sfxs *(commit by [@petscheit](https://github.com/petscheit))*
- [`0466fa0`](https://github.com/t3rn/t3rn/commit/0466fa0521e52b8f14c1842adedc181c733d6347) - droppedAtBidding slashing issue *(commit by [@petscheit](https://github.com/petscheit))*
- [`c1873a6`](https://github.com/t3rn/t3rn/commit/c1873a606c330eb31d2a4ab6cc5dc3ed27d4d1e4) - update docs deployment link *(commit by [@petscheit](https://github.com/petscheit))*
- [`97fbce9`](https://github.com/t3rn/t3rn/commit/97fbce9ecd18b850c551d55aeee20e7bb0e330b5) - revert docs alias domain back to old *(commit by [@alexand3rwilke](https://github.com/alexand3rwilke))*
- [`2a92dd9`](https://github.com/t3rn/t3rn/commit/2a92dd91f477dfcea7886e4e5ddab3651de47bc4) - state machine never reaching terminal state, causing reverts on confirmed and finalized XTXs *(commit by [@petscheit](https://github.com/petscheit))*
- [`5a340e0`](https://github.com/t3rn/t3rn/commit/5a340e0e182caa158bd20505f02797ef76e0a95c) - make monetary::deposit_immidiately fallible *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`c686ccd`](https://github.com/t3rn/t3rn/commit/c686ccd9bd54c6096df87366cca72429db6bb494) - use SFXBid::reward_asset_id field at bidding *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`ae47ae3`](https://github.com/t3rn/t3rn/commit/ae47ae333c98ba7f0748aaf2f7d63dbc5d2e0f6a) - enforce all SFX to have the same reward asset field *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`951d15a`](https://github.com/t3rn/t3rn/commit/951d15af7376a6cdab73662837514d246365c01b) - change alias url for deploying docs *(commit by [@alexand3rwilke](https://github.com/alexand3rwilke))*
- [`175bce5`](https://github.com/t3rn/t3rn/commit/175bce54d2dd766c161f15d7e63fdf879075a05d) - change alias url for deploying docs *(PR [#541](https://github.com/t3rn/t3rn/pull/541) by [@petscheit](https://github.com/petscheit))*
- [`9dfb2f5`](https://github.com/t3rn/t3rn/commit/9dfb2f5db4de627479c800ca1a47b986b8347d08) - enforce encoded arguments length to match gatewayabiconfig *(commit by [@palozano](https://github.com/palozano))*
- [`ccf5e1b`](https://github.com/t3rn/t3rn/commit/ccf5e1b273bf0248c0bd5cd5b856bb7e3e8536e1) - remove leftover from merge *(commit by [@palozano](https://github.com/palozano))*
- [`a5c30a2`](https://github.com/t3rn/t3rn/commit/a5c30a20c6ad0960388967e59ad8a685ed98083b) - makefile setup step should be a dependency for tests *(commit by [@3h4x](https://github.com/3h4x))*
- [`83206c1`](https://github.com/t3rn/t3rn/commit/83206c1bae769bd0e40e7026fa543dbb79dc932a) - typos *(commit by [@omahs](https://github.com/omahs))*
- [`e8e7a5e`](https://github.com/t3rn/t3rn/commit/e8e7a5e96635862e34bb0d4092e4987bceb1ddf5) - conventional changelog need to have valid commit signature *(PR [#591](https://github.com/t3rn/t3rn/pull/591) by [@3h4x](https://github.com/3h4x))*
- [`99f0596`](https://github.com/t3rn/t3rn/commit/99f0596c1c91325cbb8a3eedbda8b23d1199c486) - rm primitives import *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`eaa03a4`](https://github.com/t3rn/t3rn/commit/eaa03a425ecaa6c3322389be595af34391cf12e3) - subalfred errors out correctly *(PR [#596](https://github.com/t3rn/t3rn/pull/596) by [@3h4x](https://github.com/3h4x))*
- [`0d66472`](https://github.com/t3rn/t3rn/commit/0d6647286d750438d63f09772f31d3c2c414844a) - correct tests expectations after existential deposit=1 added *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`57fdc6c`](https://github.com/t3rn/t3rn/commit/57fdc6ca632db5564afaa65d8e9b938d1d0b1c1e) - bidding engine maps *(commit by [@palozano](https://github.com/palozano))*
- [`67e38f9`](https://github.com/t3rn/t3rn/commit/67e38f9c0170492a77dd4e7277e821f61e76a39a) - build pkgs before then test *(commit by [@palozano](https://github.com/palozano))*
- [`cbceada`](https://github.com/t3rn/t3rn/commit/cbceada7e68d6799857e8180b54b47a113f9a739) - use yarn as before *(commit by [@palozano](https://github.com/palozano))*
- [`0c1150c`](https://github.com/t3rn/t3rn/commit/0c1150c257b1601d6e485039fdc2f8afd363c77a) - build before *(commit by [@palozano](https://github.com/palozano))*
- [`180fdd4`](https://github.com/t3rn/t3rn/commit/180fdd4aeef89a7fe867d1c3711701e877c4a434) - workflow fix *(commit by [@palozano](https://github.com/palozano))*
- [`8847f32`](https://github.com/t3rn/t3rn/commit/8847f3216f0c8856a629c0547ed2e16095808927) - correct type names *(commit by [@palozano](https://github.com/palozano))*
- [`27b7f3f`](https://github.com/t3rn/t3rn/commit/27b7f3f0aa8329ec1bbc0b0257b91c902efd5479) - maybe correct type *(commit by [@palozano](https://github.com/palozano))*
- [`7425a2a`](https://github.com/t3rn/t3rn/commit/7425a2a8c9fe3c50a8708572a98bea5943bbc073) - fixed type *(commit by [@palozano](https://github.com/palozano))*
- [`a55c806`](https://github.com/t3rn/t3rn/commit/a55c806f647d9b72866b989b605eb2ac1bca8c4d) - naming in export types *(commit by [@palozano](https://github.com/palozano))*
- [`9afaaed`](https://github.com/t3rn/t3rn/commit/9afaaed8b1735225c0ae91521bd45571c1c3ccda) - add export to index so its documented *(commit by [@palozano](https://github.com/palozano))*
- [`5c55d91`](https://github.com/t3rn/t3rn/commit/5c55d9191eb035ef9df430af8449e4ca3503b9bc) - rename enum variant from type *(commit by [@palozano](https://github.com/palozano))*
- [`ec4e085`](https://github.com/t3rn/t3rn/commit/ec4e085fb8f463ca4d42b7c5be9d3fa2911549c9) - loosen collators conf MaxCandidate and SessionTime *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`4d2fadc`](https://github.com/t3rn/t3rn/commit/4d2fadc9d1e37dad842ebc534d4b23244da3236d) - loosen t0rn collators config and bump to v1.2.0-rc.5 *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`21ede2c`](https://github.com/t3rn/t3rn/commit/21ede2ce9f735e152b02d345cbc9acddf89214e9) - reset MaximumSchedulerWeight to 80 percent *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`b92aa1b`](https://github.com/t3rn/t3rn/commit/b92aa1bdea4bf6bc9d311788e9f2fd7c2603182a) - division in fee calc *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`fa78011`](https://github.com/t3rn/t3rn/commit/fa78011a0314682710032c104cee138587697abe) - return 0 weight if on_init hook consumes more than allowed *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`d40dfc2`](https://github.com/t3rn/t3rn/commit/d40dfc2c18d22c376c00480a6865b068379955ce) - fix PendingStakeRequestNotDueYet test setup *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`5b27826`](https://github.com/t3rn/t3rn/commit/5b278267c82700aadd3e68498422d7fc88b49221) - update SDK imports *(commit by [@petscheit](https://github.com/petscheit))*
- [`9c7b5a5`](https://github.com/t3rn/t3rn/commit/9c7b5a53b33a91a8dcf3d15c91e94d7b8363fca2) - **t0rn**: add slow adjusting fee *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`3cf65c8`](https://github.com/t3rn/t3rn/commit/3cf65c8876c501039ec05657bab04174f6d8b975) - remove primary Round key from PendingCharges *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*

### :recycle: Refactors
- [`de6c7d3`](https://github.com/t3rn/t3rn/commit/de6c7d3b842ac6e9a1e0d19edd7b67b3b471a752) - remove total xtx reward from Circuit *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`92296ad`](https://github.com/t3rn/t3rn/commit/92296adebe147f9b400ffde7f2382b562716f5f5) - use match instead of if let for slash/refund selection *(commit by [@petscheit](https://github.com/petscheit))*
- [`7dfb1f7`](https://github.com/t3rn/t3rn/commit/7dfb1f7773bff7ca012e4b4b65a9eb14fbb07a5b) - standard parity treasury and T::Clock::current_round() *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`b344eae`](https://github.com/t3rn/t3rn/commit/b344eae5673f5b7919c6f64d2467a085331bac64) - simpler clock on_init hook *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`3015d2d`](https://github.com/t3rn/t3rn/commit/3015d2d4019397888d5ce727efedabe2f09da959) - use safe getter at SFX validation assets check *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`3cd1181`](https://github.com/t3rn/t3rn/commit/3cd118193ed4d0f8e4964c486fb5312bd471ed92) - change variable name to human readable *(commit by [@palozano](https://github.com/palozano))*
- [`dfc93b4`](https://github.com/t3rn/t3rn/commit/dfc93b4857d39b7fb2285c9a751e35a4f8e3d5c7) - add documentation, comments and other configuration changes. *(PR [#549](https://github.com/t3rn/t3rn/pull/549) by [@ahkohd](https://github.com/ahkohd))*
- [`338af38`](https://github.com/t3rn/t3rn/commit/338af38380fea8aeb1311ed07712f3751c7f0124) - rewrite circuit as state machine with infallible monetary *(PR [#573](https://github.com/t3rn/t3rn/pull/573) by [@MaciejBaj](https://github.com/MaciejBaj))*
  - :arrow_lower_right: *addresses issue [#562](undefined) opened by [@MaciejBaj](https://github.com/MaciejBaj)*
- [`23e556b`](https://github.com/t3rn/t3rn/commit/23e556b75f785531322e594761d5169f9e0dba25) - add bid in execution manager *(commit by [@palozano](https://github.com/palozano))*
- [`d032bae`](https://github.com/t3rn/t3rn/commit/d032bae24b2e1b1dfc4a8a9697d8bebdd23c4863) - address pr comments *(commit by [@palozano](https://github.com/palozano))*
- [`14fd3a4`](https://github.com/t3rn/t3rn/commit/14fd3a47cad5e2df375a68d329626021a518a223) - runtime upgrades use scheduler.scheduleAfter(100, ...) *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`a2d92c4`](https://github.com/t3rn/t3rn/commit/a2d92c4402d1d75187a9863df649af29b38d9f4e) - encapsulate and calc weight for on_init actions *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`6bc8f49`](https://github.com/t3rn/t3rn/commit/6bc8f494a963afaa17d09f722e50d79f7269889a) - implement global on init hooks for t0rn and mock runtimes *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`9b94f81`](https://github.com/t3rn/t3rn/commit/9b94f81951562a0c14db22466251b99ec21a1a93) - move SFX interfaces and standards to t3rn-types *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`572344d`](https://github.com/t3rn/t3rn/commit/572344d860e8c218ba288ca7c2dd2ce3e7fdcb9b) - use SFX validate() and confirm() in Circuit *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`60fc421`](https://github.com/t3rn/t3rn/commit/60fc42183578332c872b927ac1ad7307facff45d) - delete protocol submodule and align Circuit *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`ef022f7`](https://github.com/t3rn/t3rn/commit/ef022f79797fa019ec9fd11b4ebc3f357d728386) - move SFX, FSX, HDX to types, delete duplicates from primitives *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`47cd382`](https://github.com/t3rn/t3rn/commit/47cd382ea9c5f2659d2ccd09d17d88912aea5090) - drop protocol dependency from XDNS *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`6a24aeb`](https://github.com/t3rn/t3rn/commit/6a24aebd427a56c741c0f6fc6d409e18b61dc2cf) - replace dependency on protocol with types to runtimes and nodes *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`0c971c7`](https://github.com/t3rn/t3rn/commit/0c971c71644d0f993a7a6ee06151e177d9485729) - remove duplicated imports between types and primitives *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`5b7dadd`](https://github.com/t3rn/t3rn/commit/5b7daddbdcbe91604bf11eb849737a7d540e2e14) - ensure FSX calculates SFX id in types *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`be9096d`](https://github.com/t3rn/t3rn/commit/be9096da5ac58832d67a4e362a2556a3262d3e81) - add and use dedicated SFX confirmation handle for Circuit Machine *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*

### :white_check_mark: Tests
- [`278035e`](https://github.com/t3rn/t3rn/commit/278035ee259c94a018e0228865c2a4043ef17210) - correct SFX init typo in types tests *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`2ae59e9`](https://github.com/t3rn/t3rn/commit/2ae59e98c1b295e22a733762d6748c017884f9af) - skeleton setup *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`b37b3e0`](https://github.com/t3rn/t3rn/commit/b37b3e0aefd80cb50d1d08391c07d8e252fb9b6b) - add a negative case *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`9d81be2`](https://github.com/t3rn/t3rn/commit/9d81be246d3ce10fe98d305aab382758e9a1213d) - fix evm test that fails due to step count *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`7c7e3d8`](https://github.com/t3rn/t3rn/commit/7c7e3d86350141dbe48367c15d1a3f829e493240) - **3vm**: add flipper contract to fixture manually rather than sdk *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`0239c1f`](https://github.com/t3rn/t3rn/commit/0239c1f9355dd5f8145586729efa9914f539f0d6) - fix executors and force it to use mock runtime *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`6214488`](https://github.com/t3rn/t3rn/commit/621448884953f69af68206b73008f27b29fda75d) - retrofit zombienet to t0rn *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*

### :wrench: Chores
- [`e922d61`](https://github.com/t3rn/t3rn/commit/e922d61706764394fd6289130d3faedd5b5d5511) - remove obsolete insurance event and connect XTransactionReadyForExec *(commit by [@petscheit](https://github.com/petscheit))*
- [`4790c40`](https://github.com/t3rn/t3rn/commit/4790c404195a272732cf87556d32bb97ff6f2383) - **deps**: bump loader-utils from 2.0.3 to 2.0.4 in /docs/main *(commit by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`f96ae20`](https://github.com/t3rn/t3rn/commit/f96ae20e3c090458aa26a7271302ece91cdd95fd) - remove PR template in favour of org-wide one *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`716184f`](https://github.com/t3rn/t3rn/commit/716184f5e935a122d4af610a23d4fb82e0d89854) - cleanup *(commit by [@petscheit](https://github.com/petscheit))*
- [`63486b6`](https://github.com/t3rn/t3rn/commit/63486b6ec52b86dcb67e4943bfbe3950d71d5930) - **deps**: bump minimatch, recursive-readdir and serve-handler *(commit by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`ea9e6b2`](https://github.com/t3rn/t3rn/commit/ea9e6b29b51b5aa67bca7cd857241ae117bfd2cd) - preserve existing pallet asset id *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`e6bbca0`](https://github.com/t3rn/t3rn/commit/e6bbca0e7f9984a58ade1aeff430688c9efd1961) - **deps**: bump secp256k1 from 0.24.0 to 0.24.2 in /pallets/contracts-registry/rpc/runtime-api *(PR [#546](https://github.com/t3rn/t3rn/pull/546) by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`16ee160`](https://github.com/t3rn/t3rn/commit/16ee1606dde1064a3d34ee06304ab417a11d41d4) - **deps**: bump secp256k1 from 0.24.0 to 0.24.2 in /pallets/contracts-registry/rpc *(PR [#547](https://github.com/t3rn/t3rn/pull/547) by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`d75582e`](https://github.com/t3rn/t3rn/commit/d75582ea4947812455d0f00f80e00056d43f4b1b) - format primitives *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`bfec404`](https://github.com/t3rn/t3rn/commit/bfec404218f1275c8ff771bea08922f18f7b2849) - merge branch 'development' into feat/mult-currency-sfx *(commit by [@palozano](https://github.com/palozano))*
- [`d260979`](https://github.com/t3rn/t3rn/commit/d2609793ae95df6b0e9523f8b2cb01cdf7ff42f8) - remedy merge remote-tracking branch 'origin/development' into feat/mult-currency-sfx *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`868b80f`](https://github.com/t3rn/t3rn/commit/868b80f7e90ec56f5518cb73b858dcd8ac35afbe) - fix error in merge resolution *(commit by [@palozano](https://github.com/palozano))*
- [`4502dbd`](https://github.com/t3rn/t3rn/commit/4502dbd1d36042d76450c606d22fccacb09779e3) - merge branch 'development' into feat/mult-currency-sfx *(commit by [@palozano](https://github.com/palozano))*
- [`860d2af`](https://github.com/t3rn/t3rn/commit/860d2af1fc3a76b3395c95aba08aab33fa2ef9f8) - reformat code to comply with CI *(commit by [@palozano](https://github.com/palozano))*
- [`704e3e6`](https://github.com/t3rn/t3rn/commit/704e3e6e7f122b12a6abe60df5e427c1d5f1722c) - fix account manager *(commit by [@palozano](https://github.com/palozano))*
- [`8af478b`](https://github.com/t3rn/t3rn/commit/8af478b2f6049ccf4cb2301bef6af1a01754924d) - better and safer var names *(commit by [@palozano](https://github.com/palozano))*
- [`07b8da9`](https://github.com/t3rn/t3rn/commit/07b8da9cd5e51a030840ff44c9f50b49a23b458d) - **deps**: bump json5 from 2.2.1 to 2.2.3 in /docs/main *(PR [#565](https://github.com/t3rn/t3rn/pull/565) by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`b1b6d03`](https://github.com/t3rn/t3rn/commit/b1b6d0394ef88f5fcbd87a654234e589ccc85ea5) - **deps**: bump json5 from 2.2.1 to 2.2.3 in /client/packages/types *(PR [#568](https://github.com/t3rn/t3rn/pull/568) by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`f804a91`](https://github.com/t3rn/t3rn/commit/f804a9105c15799cd86ce5162eaa9a1f2829fd43) - **deps**: bump json5 from 2.2.1 to 2.2.3 in /client/packages/cli *(commit by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`bbdbf2b`](https://github.com/t3rn/t3rn/commit/bbdbf2bf08536904967d163c27e3745c9967be8d) - zombienet remove setup step from GHA *(commit by [@3h4x](https://github.com/3h4x))*
- [`50ef384`](https://github.com/t3rn/t3rn/commit/50ef3841ecc11c9327aaf64b7b91983fbf6826d3) - update description *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`fe0b4db`](https://github.com/t3rn/t3rn/commit/fe0b4db857e29e9d5d67ed820c49fd22ac0f4788) - add contributing file *(commit by [@palozano](https://github.com/palozano))*
- [`86afa8b`](https://github.com/t3rn/t3rn/commit/86afa8b4e57827b645bd124a1dedc9922692c51b) - remove primitives and proto from mainnet *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`02612e5`](https://github.com/t3rn/t3rn/commit/02612e5a2a59d9b48d3727cc04da2d0ad95be33f) - **release**: v1.2.0-rc.5 [skip ci] *(commit by [@t3rn-ci](https://github.com/t3rn-ci))*
- [`c3167e2`](https://github.com/t3rn/t3rn/commit/c3167e2a041d681a26aa622fc9df3edf58a406b7) - pallet-asset-tx-payment config *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`5f7e5ed`](https://github.com/t3rn/t3rn/commit/5f7e5ede02c2b9afde6e5e8efcafcf2877f4712e) - install pallet-asset-tx-payment to t0rn *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`492a959`](https://github.com/t3rn/t3rn/commit/492a959e8e1d20434cb2fe477099707858a97581) - introduce a common existential deposit of 1 *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`0974a8f`](https://github.com/t3rn/t3rn/commit/0974a8f2c726007b752ff9eb35fa57d97cc74342) - cleanup following review *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`5f27f28`](https://github.com/t3rn/t3rn/commit/5f27f283388944bb46dafb07eba84eaf18ebe3e6) - **release**: v1.2.0-rc.6 [skip ci] *(commit by [@t3rn-ci](https://github.com/t3rn-ci))*
- [`5243e90`](https://github.com/t3rn/t3rn/commit/5243e902f8677f4c59e7851a46de85da579d810a) - **release**: v1.2.0-rc.7 [skip ci] *(commit by [@t3rn-ci](https://github.com/t3rn-ci))*
- [`3556a79`](https://github.com/t3rn/t3rn/commit/3556a795a72f02ca1245acebb2f049265f19e42f) - **release**: v1.2.0-rc.8 [skip ci] *(commit by [@t3rn-ci](https://github.com/t3rn-ci))*
- [`c2fd683`](https://github.com/t3rn/t3rn/commit/c2fd6836ed5ca713200232b335b03e350d34e91a) - **release**: v1.2.0-rc.9 [skip ci] *(commit by [@t3rn-ci](https://github.com/t3rn-ci))*
- [`1c91b68`](https://github.com/t3rn/t3rn/commit/1c91b684fbeb5e5b2bdcbb87557e33917f9ffca7) - implement bidding scenarios *(commit by [@palozano](https://github.com/palozano))*
- [`a912634`](https://github.com/t3rn/t3rn/commit/a91263404d70e19d9f02091a43ee1bcd5bf04519) - add makefile for local package building *(commit by [@petscheit](https://github.com/petscheit))*
- [`e430533`](https://github.com/t3rn/t3rn/commit/e4305335fc6639fb4a3558b189c3ca0fe434564c) - add scenarios for bidding engine *(commit by [@palozano](https://github.com/palozano))*
- [`a6a0e1f`](https://github.com/t3rn/t3rn/commit/a6a0e1f3d7a58ebe37ab1e78ab2e99b38e67ec89) - add to the bidding engine *(commit by [@palozano](https://github.com/palozano))*
- [`f5b248e`](https://github.com/t3rn/t3rn/commit/f5b248e5f5ea6090f0941169f14d93b07fe675a6) - wrapping up the new bidding engine *(commit by [@palozano](https://github.com/palozano))*
- [`25b983d`](https://github.com/t3rn/t3rn/commit/25b983dc44ee48cfba850797da80c0dadb02f7fa) - almost finish bidding engine with tests *(commit by [@palozano](https://github.com/palozano))*
- [`0673a09`](https://github.com/t3rn/t3rn/commit/0673a09666100bb4dfecc363b01fc15cd2a23d62) - add missing tests for executor *(commit by [@palozano](https://github.com/palozano))*
- [`f03bf44`](https://github.com/t3rn/t3rn/commit/f03bf44ad0307c41e5d691386400c6ea30c661bc) - finish bidding engine *(commit by [@palozano](https://github.com/palozano))*
- [`829d33f`](https://github.com/t3rn/t3rn/commit/829d33f9e499aa2b0859d1b175bae3be282c79fd) - missed one tracker *(commit by [@palozano](https://github.com/palozano))*
- [`9e488be`](https://github.com/t3rn/t3rn/commit/9e488be5dd977afd14de4a171ef93f67c500eefa) - add tests for bidding engine *(commit by [@palozano](https://github.com/palozano))*
- [`dde13b3`](https://github.com/t3rn/t3rn/commit/dde13b3dd7991575e1368cfe11ca1b8be538be95) - add missing tests *(commit by [@palozano](https://github.com/palozano))*
- [`402ba53`](https://github.com/t3rn/t3rn/commit/402ba536c472a9f38658668db16946a6a8b7921c) - clean after feature *(commit by [@palozano](https://github.com/palozano))*
- [`dc7044f`](https://github.com/t3rn/t3rn/commit/dc7044f23a6b6e39a94ae0522877b9e4c5b0f96c) - **release**: v1.2.0-rc.10 [skip ci] *(commit by [@t3rn-ci](https://github.com/t3rn-ci))*
- [`ab8ec6b`](https://github.com/t3rn/t3rn/commit/ab8ec6b1710356f00fa7d74539ebcd25ec13fad2) - **deps**: bump ua-parser-js from 0.7.31 to 0.7.33 in /docs/main *(PR [#600](https://github.com/t3rn/t3rn/pull/600) by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`6e0e1f3`](https://github.com/t3rn/t3rn/commit/6e0e1f36500bc9457ae9ed9ab6e9423fb1766121) - set para id to 2100 *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`084d9c2`](https://github.com/t3rn/t3rn/commit/084d9c20d68fd69ff16d25a3d070f7fde45ac314) - remove magic numbers *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`b3223cc`](https://github.com/t3rn/t3rn/commit/b3223cce2cd59a5bfa14154bcc87d82104aafa5e) - remove unused deposit *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`45f2cb6`](https://github.com/t3rn/t3rn/commit/45f2cb6e7038f0d60d7d58efb150145ab8652d23) - remove unneeded primitives module *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`08fb3c8`](https://github.com/t3rn/t3rn/commit/08fb3c866b048190e0be0a581a580d93806f395d) - clean up para config *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`4c7000c`](https://github.com/t3rn/t3rn/commit/4c7000c8d151105d9aa23d2934dfdd279f34a151) - clean up imports *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`a7c0fff`](https://github.com/t3rn/t3rn/commit/a7c0fff9ac89487da5fab039915c9b21e3cea70b) - add missing benchmarks *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`a07108e`](https://github.com/t3rn/t3rn/commit/a07108e627ec2e0499652d98772994b1264320ba) - move xcm configs to the xcm config module *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`7def134`](https://github.com/t3rn/t3rn/commit/7def13458128a41dea6ab3e582df91d26d3cf664) - remove magic numbers *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`e23429f`](https://github.com/t3rn/t3rn/commit/e23429f894ad5e83065082a4b6b203a5dabd445f) - add note on bug *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`5c60d42`](https://github.com/t3rn/t3rn/commit/5c60d427e2f3d70469c750b2ba9a83c96d209dda) - remove async-std - we use tokio *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`d4227f0`](https://github.com/t3rn/t3rn/commit/d4227f0314435ecf5e91e669edc50ab502e45436) - add missing unchecked *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`30249f6`](https://github.com/t3rn/t3rn/commit/30249f68fb09a2757762f48fe9ffaae45b72be33) - update chainspec for new address and version *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`732c78d`](https://github.com/t3rn/t3rn/commit/732c78d94c492c492df3e761a29c9a8adef72fa1) - update t3rn collators in shell chain_spec *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`8118ebd`](https://github.com/t3rn/t3rn/commit/8118ebd34f8ad47e1699ee7d6d89810330d72f49) - update to new collators and keys *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`efece9c`](https://github.com/t3rn/t3rn/commit/efece9c97cbc82f52051ae106a0b0b2dfc8d3abc) - **release**: v1.2.0-rc.11 [skip ci] *(commit by [@t3rn-ci](https://github.com/t3rn-ci))*
- [`c1bd2e3`](https://github.com/t3rn/t3rn/commit/c1bd2e3aebf5f9312c4cae4266743139d1a5755c) - **release**: v1.2.0-rc.12 [skip ci] *(commit by [@t3rn-ci](https://github.com/t3rn-ci))*
- [`d735190`](https://github.com/t3rn/t3rn/commit/d7351906c715cdef0d81dcea839ea9e72180d574) - **release**: v1.2.1-rc.1 [skip ci] *(commit by [@t3rn-ci](https://github.com/t3rn-ci))*
- [`06d1b4c`](https://github.com/t3rn/t3rn/commit/06d1b4cd8593e0a533de00a6e5286711eeca576e) - maintain two separate release changelog processes in CI *(PR [#606](https://github.com/t3rn/t3rn/pull/606) by [@3h4x](https://github.com/3h4x))*
- [`34f9459`](https://github.com/t3rn/t3rn/commit/34f945900c9c397bbcc43728117f69ac6dd47a0d) - **deps**: bump http-cache-semantics from 4.1.0 to 4.1.1 in /docs/main *(PR [#624](https://github.com/t3rn/t3rn/pull/624) by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`a59c28e`](https://github.com/t3rn/t3rn/commit/a59c28e996bf2ae6deefe2abf0f27cb37f614396) - **release**: v1.2.1-rc.2 [skip ci] *(commit by [@t3rn-ci](https://github.com/t3rn-ci))*
- [`3ad1117`](https://github.com/t3rn/t3rn/commit/3ad11179beb255c1f1b8ed5754ac82ab1dec643f) - **release**: v1.2.1-rc.3 [skip ci] *(commit by [@t3rn-ci](https://github.com/t3rn-ci))*
- [`a9e1f62`](https://github.com/t3rn/t3rn/commit/a9e1f62cbb1812e2dc7f2d85fb0fa0fbd17f69fb) - bump @sideway/formula from 3.0.0 to 3.0.1 in /docs/main *(PR [#634](https://github.com/t3rn/t3rn/pull/634) by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`a00d637`](https://github.com/t3rn/t3rn/commit/a00d637dadf4a41f80b0b638bf995fa7991386b3) - **release**: v1.2.1-rc.4 [skip ci] *(commit by [@t3rn-ci](https://github.com/t3rn-ci))*
- [`fbb7ab0`](https://github.com/t3rn/t3rn/commit/fbb7ab00ca4fc913781bcea77dd4dd0aa7140f3e) - support 3vm noops *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`afc0f08`](https://github.com/t3rn/t3rn/commit/afc0f083f465ce0cb4463e556eb309e36c51986f) - fix noops panicking *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`ce6e9d4`](https://github.com/t3rn/t3rn/commit/ce6e9d46a2047434f803ffe8410011d84c269f3f) - take 3vm from [#29](https://github.com/t3rn/t3rn/pull/29)a687c1837db2bc1e44c871c4548f0216f5f8db *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`231e03a`](https://github.com/t3rn/t3rn/commit/231e03a6eab0e560a6c3a3a3fa9f2d01b881a657) - update to use xbi public and force a version *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`f9710ba`](https://github.com/t3rn/t3rn/commit/f9710baffb445aab14c2805b2a8cb54ff3e17e41) - fix executors coupling to everything *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`37dc880`](https://github.com/t3rn/t3rn/commit/37dc8804c94be73bb263adea82eb18f5a7251f20) - lints *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`db65f55`](https://github.com/t3rn/t3rn/commit/db65f558a9c09bf812aa675ac860a1b8ccc0ef44) - **release**: v1.2.1-rc.5 [skip ci] *(commit by [@t3rn-ci](https://github.com/t3rn-ci))*
- [`401d0c0`](https://github.com/t3rn/t3rn/commit/401d0c00c1aa6675a28bfba750813ed26e0c8dac) - edit missing titles and descriptions *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`273d02c`](https://github.com/t3rn/t3rn/commit/273d02cdcece35dc84e6611455fc762bc5a97c69) - extend DeliveryDomain with DepsUpdate *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`081958c`](https://github.com/t3rn/t3rn/commit/081958c07fc7701bfc060065b4229a13ee89fc3b) - log new validation code hash before submitting via HRMP *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`9c26558`](https://github.com/t3rn/t3rn/commit/9c26558923536cf7ea6661e419ba97182c2055d7) - take 20% of block weight for hooks *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`cff3a66`](https://github.com/t3rn/t3rn/commit/cff3a66fb837c289df3aa388dcefbf42767cf555) - ensure runtimes % share = 100 for on_init hooks *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`658b36c`](https://github.com/t3rn/t3rn/commit/658b36c5537f6c8277583b83fb5bb40503c3b4f1) - add check for modulo *(commit by [@palozano](https://github.com/palozano))*
- [`3b2ec5b`](https://github.com/t3rn/t3rn/commit/3b2ec5bbadf107e7dfee3bd7666e7294f1e35d5b) - add makefile for local package building *(commit by [@petscheit](https://github.com/petscheit))*
- [`0147960`](https://github.com/t3rn/t3rn/commit/01479602b90acf72d92140bd00c2b648f51154da) - **release**: v1.2.1-rc.6 [skip ci] *(commit by [@t3rn-ci](https://github.com/t3rn-ci))*
- [`0d37bbd`](https://github.com/t3rn/t3rn/commit/0d37bbd94c2a4ca6ffe15c3f5c944103db948c32) - rename t3rn-types side_effect -> sfx + safe args acccess *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`def84a5`](https://github.com/t3rn/t3rn/commit/def84a5234caf15cb84153998a9395e3e39ebf3f) - ensure all deposits are assigned under FSX id after request *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`069870d`](https://github.com/t3rn/t3rn/commit/069870d70ee540f33330da74e071ee8f5a6387dc) - remove unused comments in sfx calculation *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`7172ef1`](https://github.com/t3rn/t3rn/commit/7172ef1f6e7fdae56981749a9557de2ea5d5ea75) - update testnet collators doc links *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.2.0-rc.4] - 2023-03-29
### :sparkles: New Features
- [`b361f80`](https://github.com/t3rn/t3rn/commit/b361f80b293124223233fd38313863bc3901cf60) - remove badblocks from chainspec *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*


## [v1.2.0-rc.3] - 2023-03-29
### :boom: BREAKING CHANGES
- due to [`bedcaa0`](https://github.com/t3rn/t3rn/commit/bedcaa0afcaa71de37c9432492212c5ead824530) - remove developer membership *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*:

  remove developer membership


### :bug: Bug Fixes
- [`72c2062`](https://github.com/t3rn/t3rn/commit/72c206295e2d5cf52630cce67f764ee49b175568) - remove bad_blocks from t0rn chain specs *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`2d33ec9`](https://github.com/t3rn/t3rn/commit/2d33ec9abd61b553cc2f3262d2f99116573a0bc6) - ensureroot needs account type *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`7d81dfd`](https://github.com/t3rn/t3rn/commit/7d81dfdeeda1af88e2c86e8f3efdf0ab37cbcaa6) - imports were bad from find and replace *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`40d3898`](https://github.com/t3rn/t3rn/commit/40d38986ed562c527114eb0891c7918dc1268c14) - align millis per block constant to 12000 across codebaseC *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`3f4f0f8`](https://github.com/t3rn/t3rn/commit/3f4f0f83fae4a99352ddc55cc2495b31590b8973) - ensureroot needs account type *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`a394a35`](https://github.com/t3rn/t3rn/commit/a394a3507577a592b492c976184cbbca8195c8fc) - imports were bad from find and replace *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`48068df`](https://github.com/t3rn/t3rn/commit/48068df610c2376dc4b053e43e4d3001a63d1e74) - align millis per block constant to 12000 across codebaseC *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`b47d699`](https://github.com/t3rn/t3rn/commit/b47d6992661c273e5edfdf03f4b2183667a72512) - bump t0rn patchfix version and merge remedy chainspecs *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*

### :recycle: Refactors
- [`bedcaa0`](https://github.com/t3rn/t3rn/commit/bedcaa0afcaa71de37c9432492212c5ead824530) - remove developer membership *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*

### :wrench: Chores
- [`bdfe894`](https://github.com/t3rn/t3rn/commit/bdfe89470303c0635d42566bbe2904ac45693263) - add remedy blobs *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`498ebf8`](https://github.com/t3rn/t3rn/commit/498ebf8aa579d580d3b2a2eefafa7b2e3e631699) - add badblocks *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`6cc137c`](https://github.com/t3rn/t3rn/commit/6cc137cf196790c52910f834ec429d26b3344e3d) - add badblocks *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*


## [v1.2.0-rc.2] - 2023-03-29
### :bug: Bug Fixes
- [`fb427e3`](https://github.com/t3rn/t3rn/commit/fb427e3eb801449a4027951b3bd0a2469b8a16ac) - add bad_blocks extension to t0rn chain specs *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.2.0-rc.1] - 2023-03-29
### :wrench: Chores
- [`6b869b2`](https://github.com/t3rn/t3rn/commit/6b869b25e41fdee87681a09fd4cc10e542c593e0) - **deps**: bump loader-utils from 2.0.2 to 2.0.3 in /docs/main *(commit by [@dependabot[bot]](https://github.com/apps/dependabot))*


## [v1.2.0-rc.0] - 2023-03-29
### :boom: BREAKING CHANGES
- due to [`a280fef`](https://github.com/t3rn/t3rn/commit/a280fef004be99a08428292875be88e9bc5deb53) - modify contract metadata to use the enum rather than bytes *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*:

  contract metadata now uses an enum rather than raw bytes, since it's compact anyway and enums resolve to one byte

- due to [`39525c4`](https://github.com/t3rn/t3rn/commit/39525c43040b8d5797f2df8647cb4c36ca1b5e6b) - remove pallet-membership for mainnet *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*:

  remove pallet-membership for mainnet


### :sparkles: New Features
- [`d5fb62c`](https://github.com/t3rn/t3rn/commit/d5fb62cb602c358e9363b28d6736127103b2fda8) - commit linter workflow on PR
- [`5112b99`](https://github.com/t3rn/t3rn/commit/5112b991fc248c0cbe298d6356bf1b84a3616deb) - changelog action added for releases by conventional commit
- [`154cbba`](https://github.com/t3rn/t3rn/commit/154cbba0cfc409a36657a9aceed6a8c938a6cfd3) - implements rococo bridge pallet *(commit by [@petscheit](https://github.com/petscheit))*
- [`f4448b3`](https://github.com/t3rn/t3rn/commit/f4448b3c109ae7da9075da25d5950f9481dad6c9) - connects graqndpa FV init to portal pallet *(commit by [@petscheit](https://github.com/petscheit))*
- [`6416634`](https://github.com/t3rn/t3rn/commit/641663452d10d5a5f00159accd6b180abd1f1c03) - portal registration working *(commit by [@petscheit](https://github.com/petscheit))*
- [`9f1ffd6`](https://github.com/t3rn/t3rn/commit/9f1ffd60196b502186751d6fd2a57a8061c20be1) - adds gateway vendor getter to xdns *(commit by [@petscheit](https://github.com/petscheit))*
- [`2f5d007`](https://github.com/t3rn/t3rn/commit/2f5d0070b8e98becfb917a6ab9021fcc921e8aef) - adds set_owner extrinsic *(commit by [@petscheit](https://github.com/petscheit))*
- [`a9344a7`](https://github.com/t3rn/t3rn/commit/a9344a79c783daa68a918f001a392d79b9d3beb7) - adds set_operational extrinsic *(commit by [@petscheit](https://github.com/petscheit))*
- [`0ef0285`](https://github.com/t3rn/t3rn/commit/0ef0285bf4464cb9434e380c301580bfe29485cf) - adds register parameter export feature to cli *(commit by [@petscheit](https://github.com/petscheit))*
- [`94afb9a`](https://github.com/t3rn/t3rn/commit/94afb9afb0786d581d4774b2f15ff216ecbf0ae8) - adds relaychain/parachain lookup logic and removes submit_finality_proof *(commit by [@petscheit](https://github.com/petscheit))*
- [`ddb0c27`](https://github.com/t3rn/t3rn/commit/ddb0c27e2d5f60077cebf39a9d7c370cab0284b5) - adds relay/parachain specific registration *(commit by [@petscheit](https://github.com/petscheit))*
- [`c1d7cc8`](https://github.com/t3rn/t3rn/commit/c1d7cc883225024356e71d836a5d9372fe16d9ff) - adds header submission for parachains *(commit by [@petscheit](https://github.com/petscheit))*
- [`72e2f66`](https://github.com/t3rn/t3rn/commit/72e2f663f44c9f22f50a1dd4c4ef9c87cd226aea) - new cli export format and adds submit_header to cli *(commit by [@petscheit](https://github.com/petscheit))*
- [`9a0ef6d`](https://github.com/t3rn/t3rn/commit/9a0ef6d286b0a25b7b5856097ae6dba6b15caee0) - adds relaychain header submission to portal and cli *(commit by [@petscheit](https://github.com/petscheit))*
- [`14840a2`](https://github.com/t3rn/t3rn/commit/14840a2176a464e7b33c2f991d7afb4aa7f728ea) - header range submission in one function + removes call and event from FV *(commit by [@petscheit](https://github.com/petscheit))*
- [`236d823`](https://github.com/t3rn/t3rn/commit/236d8235927185b5f72dfa40d3ffbce73981ea85) - header submission updated in CLI *(commit by [@petscheit](https://github.com/petscheit))*
- [`6b75ccc`](https://github.com/t3rn/t3rn/commit/6b75ccc19a08e1a403d8a38743ebaadfff7fbc21) - adds getLatestFinalizedHeader rpc endpoint *(commit by [@petscheit](https://github.com/petscheit))*
- [`33d09c6`](https://github.com/t3rn/t3rn/commit/33d09c6ae650bfd35dedf168c304baecaf54ed7a) - adds batch header submit for relaychains in CLI *(commit by [@petscheit](https://github.com/petscheit))*
- [`d9dd3d9`](https://github.com/t3rn/t3rn/commit/d9dd3d9dbf214fdb5e3f9024016e9439ca05aad7) - adds epochsAgo option for registrations via CLI *(commit by [@petscheit](https://github.com/petscheit))*
- [`aebb153`](https://github.com/t3rn/t3rn/commit/aebb153fdbc0ac65fea4cd3fd6319b2b5959ae30) - adds parachain registration to CLI *(commit by [@petscheit](https://github.com/petscheit))*
- [`f44e6c0`](https://github.com/t3rn/t3rn/commit/f44e6c086543144f2c5a0e89b19a44c209929c68) - decode side effect from encoded bytes *(commit by [@beqaabu](https://github.com/beqaabu))*
- [`5e844b4`](https://github.com/t3rn/t3rn/commit/5e844b4bbc446df9ba0c28123a9ac28bc305b497) - match side effect to encoded action *(commit by [@beqaabu](https://github.com/beqaabu))*
- [`792c981`](https://github.com/t3rn/t3rn/commit/792c9816b4b0ef70b6bfd295e80809eb7be22d8f) - convert side effects from local trigger to side effects *(commit by [@beqaabu](https://github.com/beqaabu))*
- [`071d287`](https://github.com/t3rn/t3rn/commit/071d287d0e0c16c713481b17dc1dce1c4307f2d7) - adds parachain submit-headers to CLI and portal *(commit by [@petscheit](https://github.com/petscheit))*
- [`d933242`](https://github.com/t3rn/t3rn/commit/d93324275e212c72e05beb633a6a9e1b81acd730) - wraps CLI in commander and adds transaction type to export *(commit by [@petscheit](https://github.com/petscheit))*
- [`cd2369e`](https://github.com/t3rn/t3rn/commit/cd2369e7d74d35390c29956dc94432ea40efb4bf) - programatically generate live testing data *(commit by [@petscheit](https://github.com/petscheit))*
- [`3166c14`](https://github.com/t3rn/t3rn/commit/3166c14e3ea489ebbc51886d35f11b9ef134ab57) - test live data sequentially *(commit by [@petscheit](https://github.com/petscheit))*
- [`2a703f5`](https://github.com/t3rn/t3rn/commit/2a703f5be54bc9f0d58c0a9682c69446cb102969) - add latest finalized head to portal *(commit by [@petscheit](https://github.com/petscheit))*
- [`c730d1a`](https://github.com/t3rn/t3rn/commit/c730d1ab6c96c47660147b801396c5b4448098b8) - connect circuit to portal *(commit by [@petscheit](https://github.com/petscheit))*
- [`3a477b4`](https://github.com/t3rn/t3rn/commit/3a477b459a52d07e215321adbbb4fb0528b7a238) - make side effects decoder idiomatic *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`2f62e48`](https://github.com/t3rn/t3rn/commit/2f62e481ab29281724523887e5b0be8096e31bb8) - add asset to aliq *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`a63c316`](https://github.com/t3rn/t3rn/commit/a63c316a190db1f95670d6ed41016b23edb79336) - adds event inclusion proof verification *(commit by [@petscheit](https://github.com/petscheit))*
- [`00cfb37`](https://github.com/t3rn/t3rn/commit/00cfb37b3dbe225f6e5f0917608baf8d8bb36020) - support new protocol side effects and move the target id's here *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`f089e41`](https://github.com/t3rn/t3rn/commit/f089e4141a507348e68975199fd828909e7aed04) - connects event decoding to grandpa FV and portal *(commit by [@petscheit](https://github.com/petscheit))*
- [`bade099`](https://github.com/t3rn/t3rn/commit/bade0992e1f45b33b84928a56f61d050b3a60028) - connect side_effect confirmation to portal and protocol *(commit by [@petscheit](https://github.com/petscheit))*
- [`2abe2da`](https://github.com/t3rn/t3rn/commit/2abe2da2dba01f4a46719fb2a419347bba443574) - update traits to be more ergonomic *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`a280fef`](https://github.com/t3rn/t3rn/commit/a280fef004be99a08428292875be88e9bc5deb53) - modify contract metadata to use the enum rather than bytes *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`f6a1d4a`](https://github.com/t3rn/t3rn/commit/f6a1d4a20eaf447ec9163c076f9a0fb122f41518) - updates to threevm trait *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`2216dc4`](https://github.com/t3rn/t3rn/commit/2216dc4da4cb350d1bfbdf82355ad88e2b81219d) - install 3vm to standalone and parachain *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`67d8a5a`](https://github.com/t3rn/t3rn/commit/67d8a5a81b32748483bdb457574b1961c2447d50) - install first pass of 3vm enabled evm into standalone *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`09e5a02`](https://github.com/t3rn/t3rn/commit/09e5a02605ca73437768d0c6dfa799bd11f09997) - supplement to allow users to claim accounts *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`cbc5338`](https://github.com/t3rn/t3rn/commit/cbc53383f77ea2891315e90f1130f75376282290) - implement precompiles on standalone *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`e7c34c4`](https://github.com/t3rn/t3rn/commit/e7c34c4f15fc249c09b78b348c7bc2beaa5c9a8e) - configure precompiles at genesis *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`0b91ced`](https://github.com/t3rn/t3rn/commit/0b91ced19191144ac6e6611d297843c91c98b63e) - implement evm rpc *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`da19688`](https://github.com/t3rn/t3rn/commit/da19688ceffe1cb8a51993ea12790c71af876cb6) - return the nonce from an account manager depositchore: transform imbalance to dispatchresult *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`8f5e697`](https://github.com/t3rn/t3rn/commit/8f5e697da7466d6f84ceb5c78fa47d235824e049) - use new version of t3rn-sdk-primitives *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`9a4589f`](https://github.com/t3rn/t3rn/commit/9a4589fa616885a10c50a14fca5acc6577f802fc) - executor can confirm side effects *(commit by [@petscheit](https://github.com/petscheit))*
- [`98fb666`](https://github.com/t3rn/t3rn/commit/98fb6663518155ed16e665345f5577db53c59e28) - provide a better way to determine if try_remunerate was a noop *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`26520d4`](https://github.com/t3rn/t3rn/commit/26520d418565ccb84de3d3d1ba708dceb8377ad8) - pallet-inflation boilerplate
- [`83ad40d`](https://github.com/t3rn/t3rn/commit/83ad40daa96028e48be85e8902a67d98a6b78577) - more factory accounts *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`340e882`](https://github.com/t3rn/t3rn/commit/340e88276ba62064674f60c89fc371b4d5841b71) - gradual inflation regression *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`ce6a49c`](https://github.com/t3rn/t3rn/commit/ce6a49c425ff7cb4cab00888b5a5d357c8cac3ca) - draft pallet-staking *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`f0589e1`](https://github.com/t3rn/t3rn/commit/f0589e1b5fc94b4f82336a1219d87664f64065b4) - staking moves *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`7302d64`](https://github.com/t3rn/t3rn/commit/7302d642edba991bac824cd95956586fd7359521) - proto enforced max risk *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`3ada922`](https://github.com/t3rn/t3rn/commit/3ada9222ce11f72c16f9968ea6e65f0cbaa0031a) - scheduled exec conf reqs *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`29f3ae2`](https://github.com/t3rn/t3rn/commit/29f3ae23f24e3719b62c8e217e5fbcd376d9b493) - active staked per round storage map *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`c84bf89`](https://github.com/t3rn/t3rn/commit/c84bf89b332607f29c6e1bbb84f515d9676a1d56) - fn select_active_set() *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`58a7462`](https://github.com/t3rn/t3rn/commit/58a746248b867a70bb5282065627456b54d70cf8) - staking fixtures *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`517873a`](https://github.com/t3rn/t3rn/commit/517873a9f906a85cc3dba68a1c026ea0eeac93ef) - unbonding *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`0688f17`](https://github.com/t3rn/t3rn/commit/0688f17fc447cc5268b5a3a94a27ea224398811c) - draft candidate movements *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`7292c39`](https://github.com/t3rn/t3rn/commit/7292c39b0eb811a0158f9c94b5dbc1e4ccd6c7ca) - join + leave stakers *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`d66acd0`](https://github.com/t3rn/t3rn/commit/d66acd0a567af260e5dc19675d954b3824a6eb3c) - core staking actions *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`72eaabf`](https://github.com/t3rn/t3rn/commit/72eaabf0e2f2091856d25fee3b3b3aa91c5fd27f) - stake adjust enum *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`24e671b`](https://github.com/t3rn/t3rn/commit/24e671bf9e5df56360ea2710039acc1b7edc48d4) - add cancel_configure_executor and make execute_configure_executor an unprivileged xt *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`0f8b951`](https://github.com/t3rn/t3rn/commit/0f8b951af803c5a4d4e22ac8fa6bf5013f76c0ef) - use circuit types to make transfer transaction exportable *(commit by [@petscheit](https://github.com/petscheit))*
- [`be785f5`](https://github.com/t3rn/t3rn/commit/be785f56381993610aee9facc3256108e235ff1a) - add export to executor *(commit by [@petscheit](https://github.com/petscheit))*
- [`6dd6501`](https://github.com/t3rn/t3rn/commit/6dd65013e9222f96c477534e3976d65772409f3a) - add transfer tests to circuit *(commit by [@petscheit](https://github.com/petscheit))*
- [`2b8de3c`](https://github.com/t3rn/t3rn/commit/2b8de3c19d3d4aca3550ab971925c21d21f0eee5) - transfer confirmation test working *(commit by [@petscheit](https://github.com/petscheit))*
- [`cdf6b54`](https://github.com/t3rn/t3rn/commit/cdf6b54a76a1d681ae9c46e59640d238f1f76945) - adds automatic block height forwarding for circuit tests *(commit by [@petscheit](https://github.com/petscheit))*
- [`7b43db9`](https://github.com/t3rn/t3rn/commit/7b43db9d29c12ce9ae891b1d657a1017fd280918) - adds multi-step SideEffect support to CLI *(commit by [@petscheit](https://github.com/petscheit))*
- [`62e36a2`](https://github.com/t3rn/t3rn/commit/62e36a264682f0553608096862f71f24330bfbf7) - adds basic side effects to circuit tests *(commit by [@petscheit](https://github.com/petscheit))*
- [`f13d1c8`](https://github.com/t3rn/t3rn/commit/f13d1c819645f41701cd79afa40d115572c32216) - publish a collator docker image with every release *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`317f8e6`](https://github.com/t3rn/t3rn/commit/317f8e6a4f140185c6cafad24ef8f24ca795cf4b) - cross repo pipeline trigger for chainops *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`9eb8a4d`](https://github.com/t3rn/t3rn/commit/9eb8a4db060c8b4661793fd44957572254dc45d3) - adds new circuit tests *(commit by [@petscheit](https://github.com/petscheit))*
- [`ea89513`](https://github.com/t3rn/t3rn/commit/ea895134db1ae40ab7c6b07a8a3c5f090e95794c) - adds large mixed rococo test *(commit by [@petscheit](https://github.com/petscheit))*
- [`3f42d14`](https://github.com/t3rn/t3rn/commit/3f42d14868d86895c5aba2f72c70ae332b030591) - adds parachain test *(commit by [@petscheit](https://github.com/petscheit))*
- [`3ddf446`](https://github.com/t3rn/t3rn/commit/3ddf446f93909db2ce9f77b8edff3453058f3f32) - adds execution managment using types *(commit by [@petscheit](https://github.com/petscheit))*
- [`f9ebab1`](https://github.com/t3rn/t3rn/commit/f9ebab1c5d95b5fa638ba9fa55df942f151633c2) - updates executor to support types and handle sfx lifecycle correctly *(commit by [@petscheit](https://github.com/petscheit))*
- [`cb86de9`](https://github.com/t3rn/t3rn/commit/cb86de962e4e79e92a8b491d991cfcf99429ec59) - executor handles dirty transfers again *(commit by [@petscheit](https://github.com/petscheit))*
- [`71ef737`](https://github.com/t3rn/t3rn/commit/71ef737922192e4c028734e10256c02a2f773af0) - executions are closed correctly *(commit by [@petscheit](https://github.com/petscheit))*
- [`de182f4`](https://github.com/t3rn/t3rn/commit/de182f4a51c355cf0898349b48e87a9d5c8e4cc9) - adds step based confirmation trigger for executed sfx *(commit by [@petscheit](https://github.com/petscheit))*
- [`2fb0931`](https://github.com/t3rn/t3rn/commit/2fb09312540c70f561e9e58e3ceb6314029bd3be) - adds dirty execution trigger on step confirmation *(commit by [@petscheit](https://github.com/petscheit))*
- [`5d879d0`](https://github.com/t3rn/t3rn/commit/5d879d0307b2dc7f45e3c72af9d0fb1d2c9b931e) - adds manual optimistic nonce tracking for parallel executions *(commit by [@petscheit](https://github.com/petscheit))*
- [`d99a64e`](https://github.com/t3rn/t3rn/commit/d99a64ec2492cf344ae1d5d51a59a8798a7a0721) - adds export feature again used for test generation *(commit by [@petscheit](https://github.com/petscheit))*
- [`d606fed`](https://github.com/t3rn/t3rn/commit/d606fed0f9f90af8f900b4fa6bed3964bc0df8a0) - adds multi-chain test cases *(commit by [@petscheit](https://github.com/petscheit))*
- [`decd8e1`](https://github.com/t3rn/t3rn/commit/decd8e1a3596180e1be17e95afaa781de420c077) - adds ring buffer to relaychain *(commit by [@petscheit](https://github.com/petscheit))*
- [`1339b88`](https://github.com/t3rn/t3rn/commit/1339b88185581456676d229908f421ebdda3d5b2) - refactor header writes and implemented ring buffer for parachain header writes *(commit by [@petscheit](https://github.com/petscheit))*
- [`14217ae`](https://github.com/t3rn/t3rn/commit/14217aef445ccf5edb152821f87ae85bb2276ece) - refactor grandpa header submission to fix buffer overwrite issue *(commit by [@petscheit](https://github.com/petscheit))*
- [`375b82f`](https://github.com/t3rn/t3rn/commit/375b82ff8b3a273b3224c15e05aa62d7c6f0d71b) - adds registration duplicate checker *(commit by [@petscheit](https://github.com/petscheit))*
- [`f28f9ca`](https://github.com/t3rn/t3rn/commit/f28f9ca394deb1f3a2ef79721515425c47054ed8) - adds security coordinates *(commit by [@petscheit](https://github.com/petscheit))*
- [`df7cb3c`](https://github.com/t3rn/t3rn/commit/df7cb3cc5db27c9d557aaecb40741ea339863a55) - Update submodule paths to 3VM and XBI *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`0e883f3`](https://github.com/t3rn/t3rn/commit/0e883f3244aabee6641dc7eeb3c4030b24e337b8) - install pallet assets to mock, standalone and t0rn runtimes *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`08e6caa`](https://github.com/t3rn/t3rn/commit/08e6caa86e800c05d3406164b3553ec6eb5fa492) - extend SFX insurance deposits with bids *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`53a0c66`](https://github.com/t3rn/t3rn/commit/53a0c665f32ff735783431c3baeb2191a0511c47) - plug Assets as Foreign Fungibles Transactor to XCM *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`2d87327`](https://github.com/t3rn/t3rn/commit/2d87327f59077d8830263bd63eded46920d5bf9a) - do not engage collateral in SFX bid, unlock and slash *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`1c1114c`](https://github.com/t3rn/t3rn/commit/1c1114ca3457b04bbd34be8e3de88d9c1c5f51be) - implement field getters to SFXBid and FSX *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`3133813`](https://github.com/t3rn/t3rn/commit/3133813c3351a0de8dd412043c875a028396347c) - completely cleanup xtx storage after dropped in bidding *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`9f78784`](https://github.com/t3rn/t3rn/commit/9f78784f6c76019d40a2ff9c328dfce05adf2442) - add and track pending bidding config trait to runtimes *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`de07ed7`](https://github.com/t3rn/t3rn/commit/de07ed729366b12a042eeaff517560cdae8ee2aa) - update types package *(commit by [@petscheit](https://github.com/petscheit))*
- [`df5af68`](https://github.com/t3rn/t3rn/commit/df5af6826e1f85ed6272d2ba39a4f7738607f75d) - change SFX protocol with max_fee and require insurance *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`480c15c`](https://github.com/t3rn/t3rn/commit/480c15cb005f050465ad103169e653e443a42db8) - update SFX types and primitives with bidding *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`734d613`](https://github.com/t3rn/t3rn/commit/734d61395a549aa8c925d3bc9a9baa3575c043b2) - settle SFX names at max_reward and nonce; fix steps validation *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`fa221da`](https://github.com/t3rn/t3rn/commit/fa221da5af4bc449b889610338e50622709fbbab) - add pallet identity to Standalone & t0rn Runtime *(commit by [@ahkohd](https://github.com/ahkohd))*
- [`f3c1c8c`](https://github.com/t3rn/t3rn/commit/f3c1c8cdc25ce3cb10a36902c0ae9dce6a234722) - move sfx nonce to FullSideEffect and set in validate step based on index *(commit by [@petscheit](https://github.com/petscheit))*
- [`564c32e`](https://github.com/t3rn/t3rn/commit/564c32e1250399220867415e644ffe3761e413d9) - add new id hashing to FullSideEffect *(commit by [@petscheit](https://github.com/petscheit))*
- [`97569d5`](https://github.com/t3rn/t3rn/commit/97569d5673ec8fd7d702b8e6cbe4cb2216d1d1d8) - rename bid_execution and remove unneeded parameter *(commit by [@petscheit](https://github.com/petscheit))*
- [`5d42b46`](https://github.com/t3rn/t3rn/commit/5d42b46fc5f4f4d51b306489320c72d8870f2b84) - add extrinsic for cancelling xtx before bidding *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`ed980fd`](https://github.com/t3rn/t3rn/commit/ed980fdc9315a6e9acd2249a63dd62bce0d62f35) - add weights for cancel xtx at Circuit *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`13cabef`](https://github.com/t3rn/t3rn/commit/13cabefbf8adc465e0db78be6c4f805cfbce0072) - membership including a way for a small set to be able to runtime upgrade *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`641b359`](https://github.com/t3rn/t3rn/commit/641b359951fc81dcf5f6ef6e17bc4088bbecb927) - ensure signed by dev membership instead sudo in t0rn runtime *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`725fe20`](https://github.com/t3rn/t3rn/commit/725fe207c011dfbd410782bcae89fe4b9269767a) - enable Sfx2Xtx map for all Sfx *(commit by [@petscheit](https://github.com/petscheit))*
- [`dceb9ca`](https://github.com/t3rn/t3rn/commit/dceb9cae0f40c8e6ab0f1db5b7e32a251e65685b) - fix ensure signed by imports at t0rn runtime *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`8c76988`](https://github.com/t3rn/t3rn/commit/8c76988a2fbf5b8f334e09dfbb17a1ba6846e6a9) - refactors client side packages and adds ts-sdk *(commit by [@petscheit](https://github.com/petscheit))*
- [`8e36da9`](https://github.com/t3rn/t3rn/commit/8e36da9edf8bc51e37173f784eec253fd6f8c470) - add xtx_id and sfx_id collision tests *(commit by [@petscheit](https://github.com/petscheit))*

### :bug: Bug Fixes
- [`f9141bd`](https://github.com/t3rn/t3rn/commit/f9141bdea34d8c35b7e5170a5bb5e113945226f8) - pr alert syntax update to include version
- [`9cd37e3`](https://github.com/t3rn/t3rn/commit/9cd37e3097326b40754d45de3a4d53eb1158e4de) - latest wagoid/commitlint-github-action version v5.0.2
- [`bf9fe70`](https://github.com/t3rn/t3rn/commit/bf9fe707225e892513099b83ae78092f3fb8993d) - cli registration encoding bug *(commit by [@petscheit](https://github.com/petscheit))*
- [`e2a7866`](https://github.com/t3rn/t3rn/commit/e2a7866efe54e9b94e518bfe27d1864862fd6865) - devnet runs by unpinning subkey *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`ba52f48`](https://github.com/t3rn/t3rn/commit/ba52f486ad2b3464cd40b49c0ef63d6d559c5dc0) - the account manager is now in line with the changes to reserve_repatriating *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`29f843c`](https://github.com/t3rn/t3rn/commit/29f843cb247a775912a9786df26894e474e841e7) - intro default round term pallet config item 2 untange min and dflt *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`92ca446`](https://github.com/t3rn/t3rn/commit/92ca44648f3269344fb9344f13109d58e3545df9) - executing scheduled conf reqs *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`79f377e`](https://github.com/t3rn/t3rn/commit/79f377edc1f05447f3861759f3d042d9ac8a889a) - improve fixtures handling and set defaults *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`c9ae07d`](https://github.com/t3rn/t3rn/commit/c9ae07df8936afee2a5fac8b404db768405b34c0) - collator docker file instructions *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`dbe7670`](https://github.com/t3rn/t3rn/commit/dbe76702beb59e5f15c60f325e85ea8cc75c9f0c) - step confirmations handling order correctly *(commit by [@petscheit](https://github.com/petscheit))*
- [`ae0e062`](https://github.com/t3rn/t3rn/commit/ae0e0629cb44e4cab491d7721bac4ea32de42c46) - base collator.Dockerfile on ubuntu:21.04 to have libssl.o v1 available *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`5a497be`](https://github.com/t3rn/t3rn/commit/5a497be4901bc831ac83c8455b6cc5583355cb52) - dev dependencies for pallet-evm had balances enabled in std feature *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`31ea1c5`](https://github.com/t3rn/t3rn/commit/31ea1c56c6762be0e4f2ceb631f4d3bebe77dcd1) - Update Cargo.lock *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`841c54a`](https://github.com/t3rn/t3rn/commit/841c54ad77f27a277491f2317ac60f1224ea04da) - testing return type *(commit by [@petscheit](https://github.com/petscheit))*
- [`3878b5d`](https://github.com/t3rn/t3rn/commit/3878b5da21695c23ae90284bdaf9ef4a5cbaf390) - resolve compiler errors from merge *(commit by [@petscheit](https://github.com/petscheit))*
- [`af19448`](https://github.com/t3rn/t3rn/commit/af1944874c51b61a303d38073054444efe542363) - circuit tests *(commit by [@petscheit](https://github.com/petscheit))*
- [`97e097c`](https://github.com/t3rn/t3rn/commit/97e097ca4c7b1f889a03e261c60e36fe8fa83c7a) - make finality verifier expose crage_valid_storage_proof as std *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`a12aa96`](https://github.com/t3rn/t3rn/commit/a12aa9663a29643ee62ae913c56ce5b52b4eae43) - fix standalone collator node to work with v0.9.27 *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`41386b5`](https://github.com/t3rn/t3rn/commit/41386b5bbd6f1e3af51a6f993ee150fde5e5d110) - update runtime and node for t0rn with latest contracts RPC *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`b499b53`](https://github.com/t3rn/t3rn/commit/b499b532405225ed4116fa7f2aec79040876ac13) - use common-parachains::AssetId for XCM Fungibles *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`88b9ac0`](https://github.com/t3rn/t3rn/commit/88b9ac03c46b92838512bbc16e64e9d45acc8224) - reserve and unreserve executors of best bids and fix statuses *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`0870431`](https://github.com/t3rn/t3rn/commit/08704314585717a2e04918b7ad90017a2afb45f5) - update SFX determined security LVL at validation *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`7316e74`](https://github.com/t3rn/t3rn/commit/7316e74cb4e3e2937c3dc60935b623fd509bb3ba) - force change xtx status update before XBI charge *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`e7bfd2d`](https://github.com/t3rn/t3rn/commit/e7bfd2d979478a2b8e60dd3153ca68dc111beffb) - correct SS58Prefix to 9935 at runtime para config *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`3636cfd`](https://github.com/t3rn/t3rn/commit/3636cfd147b032710169c61b72677da5128b99d5) - fix kill xtx for timeouts and optimistic mod *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`edb0305`](https://github.com/t3rn/t3rn/commit/edb03052ea47bc6b6ad664ebcb33b1fa7076cad6) - add is bid resolved check to SFX primitives *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`765d314`](https://github.com/t3rn/t3rn/commit/765d3143fd04c8c4754bd078be995aaf4868d862) - correct SS58Prefix to 9935 at runtime para config *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`ae34886`](https://github.com/t3rn/t3rn/commit/ae34886a6fddf78c1021d485bdc53ad54fd484a6) - set desired collator candidates to 32 at genesis *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`d292815`](https://github.com/t3rn/t3rn/commit/d29281503452d9cd60bdfedc327756b3deef9474) - introduce fees + rewards collective split at AccountManager *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`7197a7a`](https://github.com/t3rn/t3rn/commit/7197a7ae6522bccdc22f65ddb47412bf899c11ed) - distinct t0rn and t3rn for docker images *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`654bbec`](https://github.com/t3rn/t3rn/commit/654bbec3ef2ab826150b4dd442923f38c6cdd93d) - add separate collator dockerfile for t3rn *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`3dbd662`](https://github.com/t3rn/t3rn/commit/3dbd66238985804923c1e7edee2d5512c3ebe63b) - add polkadot raw chainspec to t3rn specs *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`5851d74`](https://github.com/t3rn/t3rn/commit/5851d74aeb4f7a048de49f6e3c929b887937cee3) - take into account root /usr/local/bin machines *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`3968ee7`](https://github.com/t3rn/t3rn/commit/3968ee7dc56257e3cb9f5dd7db15b7e9af77da59) - only access reserved bond when other SFX exist when Slash *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`04d32ea`](https://github.com/t3rn/t3rn/commit/04d32ea1afbe09fee1dd68aab6ca0349f6f0986b) - fix regenerate parachain buld script paths *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`6353502`](https://github.com/t3rn/t3rn/commit/63535026ff023125d7dab2445bf95c8e279376b3) - bump transaction version for t0rn runtime *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`b3f0b80`](https://github.com/t3rn/t3rn/commit/b3f0b80f44641f7e8e915a164a31dd5f44d7aa9c) - correct parameters to upgrade runtime script at deploy pipe *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`af2ca63`](https://github.com/t3rn/t3rn/commit/af2ca63608acef83a8eebd2dc13ef0ccb488a4c6) - correct tag param to upgrade runtime script at deploy pipe *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`b14465a`](https://github.com/t3rn/t3rn/commit/b14465aa5913077021aed369acda05fc646f0064) - revert build artifacts for t0rn to v1.1-rc.0 *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`9c0bad0`](https://github.com/t3rn/t3rn/commit/9c0bad008784f11e486b898cece958c7a7890a40) - update pushed tag to GH pipelines for t0rn release *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`222e48f`](https://github.com/t3rn/t3rn/commit/222e48f6285eaedf4f398347bc9c4c5c1f885c8b) - use unsafe runtime upgrade script for CD release *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*

### :recycle: Refactors
- [`1009e70`](https://github.com/t3rn/t3rn/commit/1009e702200e1603a82667a96385611ed860036c) - provide non-sensitive artifacts for collators *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`a48a5c9`](https://github.com/t3rn/t3rn/commit/a48a5c9d78f528bd19b68b497c178379bd9dfd01) - use std ubuntu as docker collator base image *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`74c7f28`](https://github.com/t3rn/t3rn/commit/74c7f28b8c5f34df5a890c2e6b7160d0f0ecd2b0) - Add common runtime types as a separate Runtime crate *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`27c0fed`](https://github.com/t3rn/t3rn/commit/27c0fed79f12cc86c2a31ca1b9e5642cb922e889) - SFX primitives get confirmed with safe access *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`0a530a3`](https://github.com/t3rn/t3rn/commit/0a530a332c69977d2931ec1f3dcb94db050a1e2c) - move zombienet bins into repo and ignore *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`0bf6a24`](https://github.com/t3rn/t3rn/commit/0bf6a2485cadcf7f7ee8f773f715ded135db0ed6) - align collator ports and names *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`cc78823`](https://github.com/t3rn/t3rn/commit/cc78823d4b1b6702cb0db8f4aa26a24750d9511b) - in confirm use sfx from storage & remove unneeded parameters *(commit by [@petscheit](https://github.com/petscheit))*
- [`7c8569f`](https://github.com/t3rn/t3rn/commit/7c8569f71237f09d90f9dfc14a058c45160b1bd1) - pin http rpc ports *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`39525c4`](https://github.com/t3rn/t3rn/commit/39525c43040b8d5797f2df8647cb4c36ca1b5e6b) - remove pallet-membership for mainnet *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*

### :white_check_mark: Tests
- [`843bb2f`](https://github.com/t3rn/t3rn/commit/843bb2f4164c8e6527cb1da27b6901aae8853f4f) - on_local_trigger works correctly for different side effects *(commit by [@beqaabu](https://github.com/beqaabu))*
- [`6cc1dea`](https://github.com/t3rn/t3rn/commit/6cc1dea497738168aa3622bea95dd7dde3a99e20) - make test compile *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`39ffb82`](https://github.com/t3rn/t3rn/commit/39ffb82ef3572e2680af45fd9ce5acfaeb77cec6) - take standardised side effects from protocol *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`bd5672c`](https://github.com/t3rn/t3rn/commit/bd5672c6f0d371121151be9413303626a750a429) - suite skeleton *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`370708a`](https://github.com/t3rn/t3rn/commit/370708aa5465214cc05837841eb3fa4afcdb31a6) - inflation regression *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`e3f7262`](https://github.com/t3rn/t3rn/commit/e3f7262a6a5a83ca71b127b4abce2474060f1a73) - minor unit test fixes *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`e017a37`](https://github.com/t3rn/t3rn/commit/e017a37597297b0c74c68d5511e159c7668c426b) - initial staking unit tests *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`707f148`](https://github.com/t3rn/t3rn/commit/707f148e537c0b9baf4360cabf1a332e2ecc301d) - comprehensive pallet staking unit tests *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`bd46c5e`](https://github.com/t3rn/t3rn/commit/bd46c5eddb8d1b08f0b8c0baf8273e44a1cf6bd0) - update contracts registry tests to utilise enum *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`65be1a2`](https://github.com/t3rn/t3rn/commit/65be1a2f402a8a5cddded6c1ec9159906580802b) - align expected Circuit Account at tests to 0x333...3 *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`87c8e61`](https://github.com/t3rn/t3rn/commit/87c8e61eb1a4fe5a6b0729ded923660bc0b8976e) - compile circuit tests after bidding rework *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`2f87928`](https://github.com/t3rn/t3rn/commit/2f879281d492713ebeb6aee048dcc333ee273969) - compile circuit test suite and add basic coverage for bidding *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`50bbcea`](https://github.com/t3rn/t3rn/commit/50bbceac9296c2a036ce885b97e8b2d51d89f96e) - fix circuit submit sfx tests *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`3839c43`](https://github.com/t3rn/t3rn/commit/3839c437c3de82774240431f8acfb9f3923778e5) - ignore 4 sdk & mock-data tests at Circuit *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`76bf29b`](https://github.com/t3rn/t3rn/commit/76bf29b5372b033157158c5232ef79593abbf5d0) - fix primitives tests after executors bidding *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`a37a333`](https://github.com/t3rn/t3rn/commit/a37a333a3513332ded122daa1b037a72ea502582) - temporarily comment out types test *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`a82d767`](https://github.com/t3rn/t3rn/commit/a82d767cd562c58404ee321b4aad851257def4c2) - leave fixme comment at disbled types to sdk tests *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`f6c6fce`](https://github.com/t3rn/t3rn/commit/f6c6fcef012388a8d5c6a07b344ac08c41b4a62c) - remove tests of Default SFX *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`068d370`](https://github.com/t3rn/t3rn/commit/068d370d6864e54600b6c3cebf4f459dacc03a58) - cover actual fees overflow error at AccountManager *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`05dc25c`](https://github.com/t3rn/t3rn/commit/05dc25c942441fc1002ff764a4bbb57f37eb8e24) - cover percent ratio with tests at Account Manager *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`6c37194`](https://github.com/t3rn/t3rn/commit/6c3719409adc633d5aa5665a90ae4d695c8eadce) - fix test on cancellation id generation after recent updates *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*

### :wrench: Chores
- [`8a21439`](https://github.com/t3rn/t3rn/commit/8a2143974630317929655f68d76ad4f6f363601b) - update executor event name *(commit by [@petscheit](https://github.com/petscheit))*
- [`7f119da`](https://github.com/t3rn/t3rn/commit/7f119dabce6856a92e9652d1d9c0a06b6feff0c7) - Removes commented out import *(commit by [@petscheit](https://github.com/petscheit))*
- [`d910be4`](https://github.com/t3rn/t3rn/commit/d910be445af2af85f8b7475d8baa96a1fc711760) - remove xdns and escrow trait from grandpa FV *(commit by [@petscheit](https://github.com/petscheit))*
- [`1d8e804`](https://github.com/t3rn/t3rn/commit/1d8e804c7614264de8fc3b620878220f478c8fdf) - restore tests *(commit by [@petscheit](https://github.com/petscheit))*
- [`9b87e3b`](https://github.com/t3rn/t3rn/commit/9b87e3b8308085bc273c3f5ca93d5fa120f3582b) - remove genesis config from grandpa and create portal mock *(commit by [@petscheit](https://github.com/petscheit))*
- [`c31e10f`](https://github.com/t3rn/t3rn/commit/c31e10f4ab20de6bc519ba1936bb13a30f614fb8) - update grandpa registration data type name *(commit by [@petscheit](https://github.com/petscheit))*
- [`a1a7783`](https://github.com/t3rn/t3rn/commit/a1a77833cd09a26bdff09ceea1cd43289870da06) - restores register_gateway test for roco *(commit by [@petscheit](https://github.com/petscheit))*
- [`5c268de`](https://github.com/t3rn/t3rn/commit/5c268ded6b6b353bc21f030ef7590171d70dafa7) - cleanup *(commit by [@petscheit](https://github.com/petscheit))*
- [`4d6cc93`](https://github.com/t3rn/t3rn/commit/4d6cc934bb74982db9ab1420051e36224f8bba3f) - update storage to only store relay-chain authority set *(commit by [@petscheit](https://github.com/petscheit))*
- [`5903be0`](https://github.com/t3rn/t3rn/commit/5903be0f332d2337fd2bf8cb8723d2cae39bc6df) - adds registration tests *(commit by [@petscheit](https://github.com/petscheit))*
- [`ee4e315`](https://github.com/t3rn/t3rn/commit/ee4e315113ad7ca9428265b27872bcd6b5c81c10) - remove test warnings and activate para/relaychain matchin when submitting header *(commit by [@petscheit](https://github.com/petscheit))*
- [`e93b101`](https://github.com/t3rn/t3rn/commit/e93b101df8306ad7622e92101c26eb2bb18c24f1) - **deps**: bump terser from 5.13.1 to 5.14.2 in /docs/main *(commit by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`0f1ea21`](https://github.com/t3rn/t3rn/commit/0f1ea212e96b869317efb9877f6d2457856258c9) - reverts registration type changes *(commit by [@petscheit](https://github.com/petscheit))*
- [`0432ddd`](https://github.com/t3rn/t3rn/commit/0432dddd1d349f598bb3a74a0186140f8143d70a) - **deps**: bump terser from 4.8.0 to 4.8.1 in /gateway/frontend *(commit by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`41b2f71`](https://github.com/t3rn/t3rn/commit/41b2f71b718c4d6db1f8943b2b10d263b0698a8d) - **deps**: bump @openzeppelin/contracts *(commit by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`3f39341`](https://github.com/t3rn/t3rn/commit/3f39341897cca8f684b67b39cb01526f37282e6b) - **deps**: bump @openzeppelin/contracts *(commit by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`ea80a16`](https://github.com/t3rn/t3rn/commit/ea80a16ba9c377dbe7e07477e41eb0420ff1f2f9) - fix CLI export encoding *(commit by [@petscheit](https://github.com/petscheit))*
- [`d28a520`](https://github.com/t3rn/t3rn/commit/d28a52014dd6f4146b8596e06cf26b6b77d24d7f) - adds portal tests with real data *(commit by [@petscheit](https://github.com/petscheit))*
- [`cb8c22f`](https://github.com/t3rn/t3rn/commit/cb8c22f21048363687319fb824ff2d0b1cd1b6f8) - add multitransfer and call to allowed side effects on polkadot *(commit by [@beqaabu](https://github.com/beqaabu))*
- [`944d023`](https://github.com/t3rn/t3rn/commit/944d023233c17c96601af5d683db0214fee22f15) - update lockfile *(commit by [@beqaabu](https://github.com/beqaabu))*
- [`77b394d`](https://github.com/t3rn/t3rn/commit/77b394d9d7a92d75d1a771d341264cf78c00dd2b) - update submodule *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`a0f7142`](https://github.com/t3rn/t3rn/commit/a0f7142e6c080dfa2531f94a39ec554718068652) - remove unneeded functionality *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`dcfd4fc`](https://github.com/t3rn/t3rn/commit/dcfd4fc7074132e88d2d52b0aaf3f2eae5dcce36) - fix the ordering of operations until sdk is updated *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`82894e2`](https://github.com/t3rn/t3rn/commit/82894e2cd1e8a00f24b4d998a5bbd1b4394c837d) - point to new 3vm directories *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`9228391`](https://github.com/t3rn/t3rn/commit/9228391738150d5c6f136b89ba51326c93c86cb5) - update 3vm *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`f36e492`](https://github.com/t3rn/t3rn/commit/f36e4926fb6e6c6b176057b43cea94a942ef396c) - use updated rpc naming from 3vm *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`41b3fd1`](https://github.com/t3rn/t3rn/commit/41b3fd151b0439b01ad4f88fff95b73cb4bfdcae) - update 3vm submodule *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`e8d7827`](https://github.com/t3rn/t3rn/commit/e8d7827077b99a2f4772359fc0215214e460602a) - rename 2 custom common pallet staking *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`e9c8cb0`](https://github.com/t3rn/t3rn/commit/e9c8cb0280567cf3eab6b44e6568ff056ce09b5e) - rename storage getter to total_value_locked *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`da5df98`](https://github.com/t3rn/t3rn/commit/da5df983d955ffbda1b506d09221aca93705a2ef) - Add pallet-staking's weight info trait *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`bfa2804`](https://github.com/t3rn/t3rn/commit/bfa2804cb942f5dce875e0c01220b0c59e05a0cb) - cleanup dead code *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`1e9fbdc`](https://github.com/t3rn/t3rn/commit/1e9fbdc201849d093af5f4fdd2f6dd5739cf4d64) - update submodule to 3vm develop *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`9de5e54`](https://github.com/t3rn/t3rn/commit/9de5e54636ada142331c54b4096251fb6915691f) - remove unneeded imports and add the evm client to the parachain service *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`ff9dcbe`](https://github.com/t3rn/t3rn/commit/ff9dcbeebf9db027a02ddf558cce93a64f875b17) - set mock types to mirror runtime ones *(commit by [@petscheit](https://github.com/petscheit))*
- [`2197fd3`](https://github.com/t3rn/t3rn/commit/2197fd3a666e197e31b677a7572a1eab2a213ba7) - add documentation on return type and note on configuring evm *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`130b2f2`](https://github.com/t3rn/t3rn/commit/130b2f2525338ad03febbb18c2226b18f60f9910) - restore circuit tests *(commit by [@petscheit](https://github.com/petscheit))*
- [`7d2f07e`](https://github.com/t3rn/t3rn/commit/7d2f07eebbc78579db74db15f90447474b2e850a) - remove warnings *(commit by [@petscheit](https://github.com/petscheit))*
- [`985f8df`](https://github.com/t3rn/t3rn/commit/985f8df2c8ddd846819a85a69a05a4efa9704ad9) - **deps**: bump @openzeppelin/contracts *(commit by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`7a1d128`](https://github.com/t3rn/t3rn/commit/7a1d12831969950645e242016be224ed0a6ede95) - **deps**: bump @openzeppelin/contracts *(commit by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`03b9bab`](https://github.com/t3rn/t3rn/commit/03b9bab5a2b01e2e33275df8d5cfad0086bb3755) - adds remaining on_extrinsic_trigger options to CLI *(commit by [@petscheit](https://github.com/petscheit))*
- [`e50b31c`](https://github.com/t3rn/t3rn/commit/e50b31c07d9cc423c7ae9c104a55e583059e8f16) - enable collator telemetry *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`0ee966a`](https://github.com/t3rn/t3rn/commit/0ee966a8735b0c90ba9fca6fc5624ddabf6f396b) - **deps**: bump @openzeppelin/contracts *(commit by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`8fb7974`](https://github.com/t3rn/t3rn/commit/8fb79740f6486f9019fb309a479401400f43504a) - **deps**: bump @openzeppelin/contracts *(commit by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`974c941`](https://github.com/t3rn/t3rn/commit/974c94150536f4f24d71f15414304793b12298fa) - update release pipeline tool versions *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`b3bc45d`](https://github.com/t3rn/t3rn/commit/b3bc45dffbc3e0b6ae74cee99cca8d4a50ba56a1) - reuse the collator prebuilt within the release pipeline *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`fbe40eb`](https://github.com/t3rn/t3rn/commit/fbe40ebb9ee71e67176c88c6d3add0cc5cd3b129) - switch from docker hub to ghcr *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`8479b7d`](https://github.com/t3rn/t3rn/commit/8479b7d7f1548d01f53cd922aa4fbdb7c754c924) - update types package *(commit by [@petscheit](https://github.com/petscheit))*
- [`e3af4cc`](https://github.com/t3rn/t3rn/commit/e3af4cc8db78cfc71a07ff93c23ee19171025551) - cleanup + comments + make logs useful *(commit by [@petscheit](https://github.com/petscheit))*
- [`eec9f1a`](https://github.com/t3rn/t3rn/commit/eec9f1a0ff7f9f89e98134c79af57764a9f3efa5) - removes unneeded files from CLI *(commit by [@petscheit](https://github.com/petscheit))*
- [`e52416f`](https://github.com/t3rn/t3rn/commit/e52416fa4cfb547c2eecf3e0a5373815b017c9c5) - update types again to include new events *(commit by [@petscheit](https://github.com/petscheit))*
- [`8f31f71`](https://github.com/t3rn/t3rn/commit/8f31f71c04174f8add615d449ffb02bb8e66df55) - refactored gateway_vendor lookup *(commit by [@petscheit](https://github.com/petscheit))*
- [`7b84cd0`](https://github.com/t3rn/t3rn/commit/7b84cd0faec8f921564ec8570f46bef5a083378f) - remove MFV & circuit_portal + add testing feature flag to share test_utils between pallets *(commit by [@petscheit](https://github.com/petscheit))*
- [`ca3007a`](https://github.com/t3rn/t3rn/commit/ca3007ab885692dddf6c451c4aedede8565f9aae) - fix portal tests and cleanup Cargo.toml *(commit by [@petscheit](https://github.com/petscheit))*
- [`0329177`](https://github.com/t3rn/t3rn/commit/032917703b01311d0669bc0c3f4dd789f0ca6ce4) - fixes circuit tests and cleanup *(commit by [@petscheit](https://github.com/petscheit))*
- [`d591ac1`](https://github.com/t3rn/t3rn/commit/d591ac1713690567b95a71df9bcd5c4f4ae8b339) - integrate portal into parachain runtime and add mock weights for portal *(commit by [@petscheit](https://github.com/petscheit))*
- [`809c9be`](https://github.com/t3rn/t3rn/commit/809c9be4810dc593f77df2c1b05ca6da4cdbea6b) - cargo fmt *(commit by [@petscheit](https://github.com/petscheit))*
- [`23781bf`](https://github.com/t3rn/t3rn/commit/23781bf5d0a883fab67f3117824f087efe4635ed) - updates protocol *(commit by [@petscheit](https://github.com/petscheit))*
- [`1401cfc`](https://github.com/t3rn/t3rn/commit/1401cfc64a3fa3a5e5bde0d6080e623d933f2e16) - removes max_request limitation from grandpa_fv *(commit by [@petscheit](https://github.com/petscheit))*
- [`40dbab4`](https://github.com/t3rn/t3rn/commit/40dbab493434901057bcdca2d63f9fb9ded3ba54) - updates headers_to_store *(commit by [@petscheit](https://github.com/petscheit))*
- [`be873d4`](https://github.com/t3rn/t3rn/commit/be873d4b6cdaf3dafb23d838db9b5a50b3fea5c1) - fixes cli for range submission order *(commit by [@petscheit](https://github.com/petscheit))*
- [`9841c92`](https://github.com/t3rn/t3rn/commit/9841c92a25ab1310b46d6231366d970f053b7a71) - updates tests for seq. header submission *(commit by [@petscheit](https://github.com/petscheit))*
- [`6274b4d`](https://github.com/t3rn/t3rn/commit/6274b4d2f7f62ae83285c9cc1dca70ea243e17d4) - updates and adds portal tests *(commit by [@petscheit](https://github.com/petscheit))*
- [`b8893e4`](https://github.com/t3rn/t3rn/commit/b8893e43d79bdc16cb113a27c042e6e21c0ab17e) - removes add_xdns_record call from xdns *(commit by [@petscheit](https://github.com/petscheit))*
- [`d25ccc2`](https://github.com/t3rn/t3rn/commit/d25ccc20adaf9e4eeb23ea47aecddf98bc732f01) - remove str errors from portal, grandpa-fv and xdns *(commit by [@petscheit](https://github.com/petscheit))*
- [`1b0a9f9`](https://github.com/t3rn/t3rn/commit/1b0a9f9aa20d8f27e47676de259a78e79f3bf2ff) - comment out portal rpc, until chain_id issue is resolved *(commit by [@petscheit](https://github.com/petscheit))*
- [`12b18f4`](https://github.com/t3rn/t3rn/commit/12b18f4a21350adc1c791f28e19d5a8a71f446cd) - cargo fmt *(commit by [@petscheit](https://github.com/petscheit))*
- [`03ccae6`](https://github.com/t3rn/t3rn/commit/03ccae62ad11adb177c76157b29c845065b56e4e) - **deps**: bump node-fetch from 3.2.6 to 3.2.10 in /client/cli *(commit by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`868cfa2`](https://github.com/t3rn/t3rn/commit/868cfa29f80cd58b141eb1fd1c4d8b7ffc27b497) - **deps**: bump jsdom and react-scripts in /gateway/frontend *(commit by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`cc627c9`](https://github.com/t3rn/t3rn/commit/cc627c95e26b7e05b263838979a76e4930062ec9) - **deps**: bump shell-quote and react-scripts in /gateway/frontend *(commit by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`a1691a0`](https://github.com/t3rn/t3rn/commit/a1691a0751cc8d924e1ba28afeab40f8e48003ab) - Emit DepositReceived event from AccountManager *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`ae52fb3`](https://github.com/t3rn/t3rn/commit/ae52fb31f15f916b44081027b70fc98ee17ce976) - merge development and resolve conflicts *(commit by [@petscheit](https://github.com/petscheit))*
- [`4bd89e8`](https://github.com/t3rn/t3rn/commit/4bd89e8950bdd571b6d4439b450a02a9855c92c3) - fix portal tests and remove mock *(commit by [@petscheit](https://github.com/petscheit))*
- [`2437caf`](https://github.com/t3rn/t3rn/commit/2437caf742293009f796d1f1b159485644d2bcb8) - cleanup and activate remaining tests *(commit by [@petscheit](https://github.com/petscheit))*
- [`8bc4c46`](https://github.com/t3rn/t3rn/commit/8bc4c46b8063dbe2531aae5761f4919825369ba3) - update 3vm version *(commit by [@petscheit](https://github.com/petscheit))*
- [`31295e1`](https://github.com/t3rn/t3rn/commit/31295e1245e1509ad0867a178ab216a85a52d69f) - deactivate broken tests and add reserved balance checks *(commit by [@petscheit](https://github.com/petscheit))*
- [`7de3905`](https://github.com/t3rn/t3rn/commit/7de3905a8ca157d2f04709f619350546a1d0bf5d) - fixed primitives test (sturct order changed) *(commit by [@petscheit](https://github.com/petscheit))*
- [`4ffee98`](https://github.com/t3rn/t3rn/commit/4ffee9898e0027e700bb2962996d125cae7c637f) - update protocol *(commit by [@petscheit](https://github.com/petscheit))*
- [`bc2acc3`](https://github.com/t3rn/t3rn/commit/bc2acc3067269284eae78701dc00906cb7acefb8) - update path to latest XBI Portal *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`3503fb1`](https://github.com/t3rn/t3rn/commit/3503fb106116886bb894780632e3901688170f2f) - Dummy variable rename in Circuit Tests to trigger CI *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`60981f5`](https://github.com/t3rn/t3rn/commit/60981f59348f9549f1eaa534475257a14ba1d913) - update 3vm and xbi repo path *(commit by [@petscheit](https://github.com/petscheit))*
- [`310ada5`](https://github.com/t3rn/t3rn/commit/310ada5f73374e979d2919772b3f4ffcaf38de02) - remove error log and add to .gitignore *(commit by [@petscheit](https://github.com/petscheit))*
- [`169d7c3`](https://github.com/t3rn/t3rn/commit/169d7c34bd88bc4957e94f508b77348a574c70ad) - fix PR comments *(commit by [@petscheit](https://github.com/petscheit))*
- [`51b66fc`](https://github.com/t3rn/t3rn/commit/51b66fc6a7535831620f7dd182feadbad2e39099) - removes old GatewayVendors *(commit by [@petscheit](https://github.com/petscheit))*
- [`9f42ff9`](https://github.com/t3rn/t3rn/commit/9f42ff920e1299dbee01aff5c68cb3a223c14169) - updates protocol version *(commit by [@petscheit](https://github.com/petscheit))*
- [`730ee31`](https://github.com/t3rn/t3rn/commit/730ee31b01c60d0bbf41613d8129fb649edc99c3) - update xdns getters *(commit by [@petscheit](https://github.com/petscheit))*
- [`3a6438c`](https://github.com/t3rn/t3rn/commit/3a6438c2ee5e2b8d5a54483faea62a7281d0ff0e) - resolve last PR comments *(commit by [@petscheit](https://github.com/petscheit))*
- [`4b4efff`](https://github.com/t3rn/t3rn/commit/4b4efffa37c5eeb7898454112372e32f37af336d) - update path to the protocol *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`21d68fe`](https://github.com/t3rn/t3rn/commit/21d68fe5b2733cd9e9ea2903d47e32be0ea64529) - cleanup error returns for XDNS RPC *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`7404492`](https://github.com/t3rn/t3rn/commit/740449213363e4135941e849c2cccf88f966122b) - **deps**: bump thread_local *(commit by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`07bce0c`](https://github.com/t3rn/t3rn/commit/07bce0c3bd4998a0dd11969473cdc3bb6825ca57) - **deps**: bump thread_local in /pallets/contracts-registry/rpc *(commit by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`d796028`](https://github.com/t3rn/t3rn/commit/d79602835ac30bd7f9f560e765d5037a716e4623) - remove old directories *(commit by [@petscheit](https://github.com/petscheit))*
- [`da26e61`](https://github.com/t3rn/t3rn/commit/da26e611be6527f9add604e95d155d8545ce1a64) - **deps**: bump jsdom and jest in /gateway/tests *(commit by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`943d4a2`](https://github.com/t3rn/t3rn/commit/943d4a22242a29174ae4ad4556a00bff230fdd9a) - bump collator candidacy bond to 10000TRN for mainnet *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`e66d22e`](https://github.com/t3rn/t3rn/commit/e66d22eabe786589504efc3ed6a3fee9b875e1a6) - cleanup pipelines *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`1b39bf9`](https://github.com/t3rn/t3rn/commit/1b39bf9fa3b5dcd088eea5fba514b63edf3c3949) - default telemetry endpoint in chain spec *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`03701e5`](https://github.com/t3rn/t3rn/commit/03701e5710d87e09c498aed9d3d2562a0ed85465) - mainnet genesis artifacts *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`3a18388`](https://github.com/t3rn/t3rn/commit/3a183884e9d089c724829885aaad28f0b367d001) - change para id to 3000 *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`46ef826`](https://github.com/t3rn/t3rn/commit/46ef82695d22d61ae852d802f179b68593e99627) - trash devnet and obsolete install script *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`deb744d`](https://github.com/t3rn/t3rn/commit/deb744d039fb7665da5965162699097d9aceb857) - fix tests *(commit by [@petscheit](https://github.com/petscheit))*
- [`9a6abd1`](https://github.com/t3rn/t3rn/commit/9a6abd17999ea74839515c1a55dcb5388c582ac0) - remove warnings *(commit by [@petscheit](https://github.com/petscheit))*
- [`3234453`](https://github.com/t3rn/t3rn/commit/3234453481cccd0c9327325fd905030d81c417fa) - rename FSX nonce to index *(commit by [@petscheit](https://github.com/petscheit))*
- [`a825e81`](https://github.com/t3rn/t3rn/commit/a825e81ed8d886cae25673a6a6a441143c3caea4) - remove unsafe unwraps *(commit by [@petscheit](https://github.com/petscheit))*
- [`d7c1d8e`](https://github.com/t3rn/t3rn/commit/d7c1d8ea9a26d163bfa81cccb774d91e476d219a) - remove old comment *(commit by [@petscheit](https://github.com/petscheit))*
- [`3ddd3f1`](https://github.com/t3rn/t3rn/commit/3ddd3f1ad64aab61c388db40b5fd08731b04b9be) - update t0rn membership to include sudo ci *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`c6a7a82`](https://github.com/t3rn/t3rn/commit/c6a7a8264ae04f8447cb222528922a4b33c51764) - slim down t3rn chainspec dev membership *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`85abba4`](https://github.com/t3rn/t3rn/commit/85abba46b126305aac2c6ac182fb8ba483439e97) - updates types to latest circuit *(commit by [@petscheit](https://github.com/petscheit))*
- [`81734c9`](https://github.com/t3rn/t3rn/commit/81734c9fd332cd03822957632a2c14b7056631c9) - change runs-on param to self-hosted at collator release GH actions *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*


## [v1.1.0-rc.0] - 2023-03-29
### :sparkles: New Features
- [`b6fae2d`](https://github.com/t3rn/t3rn/commit/b6fae2d2add831e9ee348fb6bb4d3381b34a2a51) - create pallet account manager *(PR [#273](https://github.com/t3rn/t3rn/pull/273) by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`f695358`](https://github.com/t3rn/t3rn/commit/f695358cc11457d907f1250d29084d6841cc3723) - standalone node now builds on 0.9.19 *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`35a6b68`](https://github.com/t3rn/t3rn/commit/35a6b68332874fd45690b2e827cbf7465c4b527e) - jig the configs around so we can have a default signaller *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`6c4f099`](https://github.com/t3rn/t3rn/commit/6c4f0991286940af2f1aa4cb5fd739d4262a106d) - move hash field and handle signals *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`0ea7a66`](https://github.com/t3rn/t3rn/commit/0ea7a667c33efdb0f4b65d1a0753cb2cf79a6287) - modify signal *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`10dafc0`](https://github.com/t3rn/t3rn/commit/10dafc025c148567e329b9c9213854208b66f406) - start handling posted signals asynchronously *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`efe896c`](https://github.com/t3rn/t3rn/commit/efe896c051a984d89d6be3b57b63674804b6185e) - create a t3rn types crate, move what we can from side effects to there *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`ea827a1`](https://github.com/t3rn/t3rn/commit/ea827a12c78be96fb4cdbc644f944cfce634e9ea) - **3VM**: Disallow contract characteristics *(commit by [@beqaabu](https://github.com/beqaabu))*
- [`798e94f`](https://github.com/t3rn/t3rn/commit/798e94f6cea5e1b4a7056f9dd5bc3507ecea6a0e) - adds relaychain registration logic to CLI *(commit by [@petscheit](https://github.com/petscheit))*
- [`f8e6bfb`](https://github.com/t3rn/t3rn/commit/f8e6bfb8e898a412b4fb9d83e1814cef553be508) - adds setOperational command *(commit by [@petscheit](https://github.com/petscheit))*
- [`b7c25a3`](https://github.com/t3rn/t3rn/commit/b7c25a394ebf939877401d0cfcfdda4c319610f6) - adds transfers *(commit by [@petscheit](https://github.com/petscheit))*
- [`305d786`](https://github.com/t3rn/t3rn/commit/305d786c1f39e854fc8f2012fd225886b95f0454) - finalizes transfers and adds readme *(commit by [@petscheit](https://github.com/petscheit))*

### :bug: Bug Fixes
- [`df04fd3`](https://github.com/t3rn/t3rn/commit/df04fd3e1742a0075ca3542d74660f699a1f0bbd) - unhalt the pallet and any preregistered gateways *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`9e91964`](https://github.com/t3rn/t3rn/commit/9e919641fe7ac50f1b984c0b0954753aa41e3ab7) - remove unneeded type from the tx handler, fix clippy lint *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`21a3a3c`](https://github.com/t3rn/t3rn/commit/21a3a3c45ba6b358fa9915f2da2080dbe5c918ec) - tests *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`d65efc9`](https://github.com/t3rn/t3rn/commit/d65efc9518b8001ddeccafc8224979e06894523b) - some tests and new config for elections *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`d635f41`](https://github.com/t3rn/t3rn/commit/d635f414852de93af0f38c4772c0bc845503fca6) - import madness *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`dc9294b`](https://github.com/t3rn/t3rn/commit/dc9294b0b52c106d8fb5d96c85070fd85dbc5b03) - collation check ci pipeline points to development explicitely *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`8cb6957`](https://github.com/t3rn/t3rn/commit/8cb695701485c280798c9389d78f46023933208a) - adjust compose file version and run against latest circuit *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`2daf8ea`](https://github.com/t3rn/t3rn/commit/2daf8ea63fa734381adee31527c07c004c3dabcf) - untangle devnet and collator ports *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`b957856`](https://github.com/t3rn/t3rn/commit/b95785650d8a2bc9e9b8c27b746130dbfe7629bf) - unbound var *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`0475ce2`](https://github.com/t3rn/t3rn/commit/0475ce2e6474277368a6132330d65b60129c9c4f) - out of bounds port *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`bd69104`](https://github.com/t3rn/t3rn/commit/bd6910408eefe4b31e33bc6e1c4172746954f8dd) - devnet is now #![no_docker] *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`27241fa`](https://github.com/t3rn/t3rn/commit/27241fa469cec8b751d5c317c924796802d8a90b) - alice needs balance 2 perform superuser duties *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`11239d5`](https://github.com/t3rn/t3rn/commit/11239d563dc4d570262ae062c71d9b476c87770b) - grandpa-ranger decoder deps *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`dca5dd2`](https://github.com/t3rn/t3rn/commit/dca5dd2962d207252f45186ef973db7f4f9857b2) - maybe install subkey while ./devnet/run.sh up *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`fcba7d7`](https://github.com/t3rn/t3rn/commit/fcba7d7377519032ba638dbe43fd9b92062a9912) - pass wasm code as array instead of buf *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`7c3a0b5`](https://github.com/t3rn/t3rn/commit/7c3a0b5351d60367c02dccdcac796ea7e55aa7e2) - the account manager is now in line with the changes to reserve_repatriating *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`a876f5f`](https://github.com/t3rn/t3rn/commit/a876f5fce565b3e8e7c7ad56ee23ec905942c3de) - broken tests and setup import *(commit by [@petscheit](https://github.com/petscheit))*
- [`5ba8461`](https://github.com/t3rn/t3rn/commit/5ba8461b55d45249e1c3085516fe17ee9356731e) - GatewaySysProps encoding *(commit by [@petscheit](https://github.com/petscheit))*

### :recycle: Refactors
- [`b36a7aa`](https://github.com/t3rn/t3rn/commit/b36a7aa6b46d3c0b29f1932cd32215d8d5608552) - Accept FSX as a reference in Circuit::confirm_inclusion *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*
- [`43f22a3`](https://github.com/t3rn/t3rn/commit/43f22a32f973a642b66e0994ae7ad521bf3e9d73) - Replace expects with ok_or_else while accessing XDNS fields *(commit by [@MaciejBaj](https://github.com/MaciejBaj))*

### :white_check_mark: Tests
- [`35562ae`](https://github.com/t3rn/t3rn/commit/35562ae81cb4b2c340f6848dd661a5b821a138b9) - **circuit**: write a test for signal sdk flow *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*

### :wrench: Chores
- [`7fc1167`](https://github.com/t3rn/t3rn/commit/7fc1167756d69524f098b3c5a5ffa4a755c8adcd) - update submodules *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`d331942`](https://github.com/t3rn/t3rn/commit/d331942c843101a082db6274b4c3a56a8854fb3d) - update submodule to 3vm with local state *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`85b53d2`](https://github.com/t3rn/t3rn/commit/85b53d2c68febae554ba565cbe1b34868b1c4323) - fix formatting *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`b7c0cfd`](https://github.com/t3rn/t3rn/commit/b7c0cfd2ebc196d6ae37be564506a3068c79b8d4) - parachain now builds on 0.9.19 *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`cdeb2c9`](https://github.com/t3rn/t3rn/commit/cdeb2c94b937ec2aa611844c77340d737f1bfbee) - bump 3vm *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`2597418`](https://github.com/t3rn/t3rn/commit/25974184605a7c2f9628a0e8d247f9201eec4e4c) - **deps**: bump cross-fetch in /gateway/escrow-smart-contract/acala *(commit by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`a9b1476`](https://github.com/t3rn/t3rn/commit/a9b147692e0539d13d8911d2fe1679a4b63edf72) - **deps**: bump cross-fetch in /gateway/escrow-smart-contract/ethereum *(commit by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`e70f5dd`](https://github.com/t3rn/t3rn/commit/e70f5ddb98770b26cf1d49770ae9f4c528717353) - **deps**: bump ansi-regex from 3.0.0 to 3.0.1 in /gateway/tests *(commit by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`91c4fec`](https://github.com/t3rn/t3rn/commit/91c4fec2751094958e6059b51ee5c230d265a0b9) - pin types pkg deps *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`0fa1256`](https://github.com/t3rn/t3rn/commit/0fa12569f019c14eb4fa1db8ce3e51be70f8f475) - update submodule fix warning *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`f4f4a54`](https://github.com/t3rn/t3rn/commit/f4f4a547e0a34f1c448c26ebe9f901e001b82c5d) - update submod *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`b78ae4f`](https://github.com/t3rn/t3rn/commit/b78ae4fe371a514f0b0249cc5a141afa38965bbf) - add logging in circuit on_signal *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`c956362`](https://github.com/t3rn/t3rn/commit/c956362976e6a90294992cc2c8fc825bc383ce4e) - update the submodules *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`274c823`](https://github.com/t3rn/t3rn/commit/274c823e4e3d53e5dcd9fcfda54d5c456cd361e1) - exclude sdk workspace *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`70e8ab6`](https://github.com/t3rn/t3rn/commit/70e8ab6b0c0d1881cd2b5bf9d5441424173fc434) - update 3vm *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`b2ea412`](https://github.com/t3rn/t3rn/commit/b2ea412c41a25ecc0c051583bd1b3dd9c574838d) - register para gtwy script *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`c45a8d8`](https://github.com/t3rn/t3rn/commit/c45a8d8e413c4a9bcddce62a2d34ceea11e741ea) - PR alerts for non-drafts only + await build-test check *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`a40067e`](https://github.com/t3rn/t3rn/commit/a40067e95741f61d9e086397d8766916b6a47c4a) - xbi devnet *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`adcb4fc`](https://github.com/t3rn/t3rn/commit/adcb4fcc3bbd772fc6be4096466ebf0367976bd7) - **deps**: bump eventsource from 1.0.7 to 1.1.1 in /gateway/frontend *(commit by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`113c77d`](https://github.com/t3rn/t3rn/commit/113c77d481516693ecb2757a6986aeea83bbfd53) - only kill devnet collators *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`64a6364`](https://github.com/t3rn/t3rn/commit/64a6364ccfda8becfc785d7df3c3aee08a27dca9) - switch 2 a #[no_docker] devnet *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`bf83d3b`](https://github.com/t3rn/t3rn/commit/bf83d3b75f7771c7c76b6d20fb4c907ccd06b1f2) - **deps**: bump regex in /pallets/contracts-registry/rpc *(commit by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`6e2ecc1`](https://github.com/t3rn/t3rn/commit/6e2ecc10ee7b24a0c166b5db6434a984e4b7db92) - **deps**: bump regex in /pallets/contracts-registry/rpc/runtime-api *(commit by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`aa68b33`](https://github.com/t3rn/t3rn/commit/aa68b33a86e61d290c6f87e4e9f2dcdb01bc8546) - **deps**: bump lru in /pallets/contracts-registry/rpc *(commit by [@dependabot[bot]](https://github.com/apps/dependabot))*
- [`209a50c`](https://github.com/t3rn/t3rn/commit/209a50cb7f6deffbd8f7ed9f229546425d82afc1) - #[no_docker] *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*
- [`8d165fa`](https://github.com/t3rn/t3rn/commit/8d165fab8c2facc5daaa68e490fd7fe2ce1c04bf) - add a codeowners file *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`01f3fc6`](https://github.com/t3rn/t3rn/commit/01f3fc67162d08c8fdcdab031369c667a9da88d3) - remove rust based justification decoder from ranger *(commit by [@petscheit](https://github.com/petscheit))*
- [`d3f9196`](https://github.com/t3rn/t3rn/commit/d3f9196f92505f24920322dc40dc0e36a90d2c10) - fixes wasm init error *(commit by [@petscheit](https://github.com/petscheit))*
- [`ad8c4b2`](https://github.com/t3rn/t3rn/commit/ad8c4b21f172d0ecabb4cdff1f5195b3e746bbc3) - update 3vm submod *(commit by [@beqaabu](https://github.com/beqaabu))*
- [`89ded8f`](https://github.com/t3rn/t3rn/commit/89ded8f948efd67340d41de05bbb1805633d5b95) - cleanup *(commit by [@petscheit](https://github.com/petscheit))*
- [`6f2f7f3`](https://github.com/t3rn/t3rn/commit/6f2f7f347b98d9ce4670461cbae238696e74304c) - move XDNS rpc return type to primitives *(commit by [@petscheit](https://github.com/petscheit))*
- [`2292c1f`](https://github.com/t3rn/t3rn/commit/2292c1ffdc1d292d267217203baf0da551e162b3) - fix format *(commit by [@petscheit](https://github.com/petscheit))*
- [`4a48191`](https://github.com/t3rn/t3rn/commit/4a48191cd860612047b72d6cdfc8a06a73427786) - removes rust justification decoder from xtx *(commit by [@petscheit](https://github.com/petscheit))*
- [`108f250`](https://github.com/t3rn/t3rn/commit/108f250633712b906e673073f3f7bee2de4b8c98) - remove xdns pre-seeding *(commit by [@petscheit](https://github.com/petscheit))*
- [`271a3c5`](https://github.com/t3rn/t3rn/commit/271a3c59c022c762ff9d905770f202dfa66f10cb) - make licenses valid identifiers as per the specification *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`4f85c3b`](https://github.com/t3rn/t3rn/commit/4f85c3b9d5a0e01bfefb5dce887b38b17f8a0f15) - use sdk primitives from crates.io *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`d76dba5`](https://github.com/t3rn/t3rn/commit/d76dba56a58369549b11e64b7f22ddb9e71df78f) - fmt toml *(commit by [@AwesomeIbex](https://github.com/AwesomeIbex))*
- [`b571b27`](https://github.com/t3rn/t3rn/commit/b571b27dfa9365f3975eb6efdb520f8a5d1d82f3) - bump runtime version fields for the first t0rn upgrade *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*


## [v1.0.0-rc.2] - 2023-03-29
### :wrench: Chores
- [`45d00fd`](https://github.com/t3rn/t3rn/commit/45d00fd258e89df9234bdac7137a89e70c1ec69b) - align specs and runtime *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*


## [v1.0.0-rc.1] - 2023-03-29
### :bug: Bug Fixes
- [`f210092`](https://github.com/t3rn/t3rn/commit/f2100920fbc7621090bcfaf92a07a71b2d5f2df5) - collation by aligning specs with runtime + allow pdotapps connection *(commit by [@chiefbiiko](https://github.com/chiefbiiko))*


[v1.0.0-rc.1]: https://github.com/t3rn/t3rn/compare/v1.0.0-rc.0...v1.0.0-rc.1
[v1.0.0-rc.2]: https://github.com/t3rn/t3rn/compare/v1.0.0-rc.1...v1.0.0-rc.2
[v1.1.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.0.0-rc.2...v1.1.0-rc.0
[v1.2.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.1.0-rc.0...v1.2.0-rc.0
[v1.2.0-rc.1]: https://github.com/t3rn/t3rn/compare/v1.2.0-rc.0...v1.2.0-rc.1
[v1.2.0-rc.2]: https://github.com/t3rn/t3rn/compare/v1.2.0-rc.1...v1.2.0-rc.2
[v1.2.0-rc.3]: https://github.com/t3rn/t3rn/compare/v1.2.0-rc.2...v1.2.0-rc.3
[v1.2.0-rc.4]: https://github.com/t3rn/t3rn/compare/v1.2.0-rc.3...v1.2.0-rc.4
[v1.3.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.2.0-rc.4...v1.3.0-rc.0
[v1.4.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.3.0-rc.0...v1.4.0-rc.0
[v1.5.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.4.0-rc.0...v1.5.0-rc.0
[v1.6.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.5.1-rc.0...v1.6.0-rc.0
[v1.7.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.6.0-rc.0...v1.7.0-rc.0
[v1.7.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.7.0-rc.0...v1.7.1-rc.0
[v1.8.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.7.2-rc.0...v1.8.0-rc.0
[v1.8.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.8.0-rc.0...v1.8.1-rc.0
[v1.9.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.8.2-rc.0...v1.9.0-rc.0
[v1.9.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.9.0-rc.0...v1.9.1-rc.0
[v1.10.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.9.3-rc.0...v1.10.0-rc.0
[v1.10.4-rc.0]: https://github.com/t3rn/t3rn/compare/v1.10.3-rc.0...v1.10.4-rc.0
[v1.11.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.10.4-rc.0...v1.11.0-rc.0
[v1.12.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.11.0-rc.0...v1.12.0-rc.0
[v1.12.4-rc.0]: https://github.com/t3rn/t3rn/compare/v1.12.3-rc.0...v1.12.4-rc.0
[v1.13.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.12.8-rc.0...v1.13.0-rc.0
[v1.14.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.13.0-rc.0...v1.14.0-rc.0
[v1.15.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.14.0-rc.0...v1.15.0-rc.0
[v1.16.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.15.1-rc.0...v1.16.0-rc.0
[v1.16.2-rc.0]: https://github.com/t3rn/t3rn/compare/v1.16.1-rc.0...v1.16.2-rc.0
[v1.17.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.16.4-rc.0...v1.17.0-rc.0
[v1.18.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.17.1-rc.0...v1.18.0-rc.0
[v1.19.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.18.2-rc.0...v1.19.0-rc.0
[v1.19.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.19.0-rc.0...v1.19.1-rc.0
[v1.19.2-rc.0]: https://github.com/t3rn/t3rn/compare/v1.19.1-rc.0...v1.19.2-rc.0
[v1.20.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.19.2-rc.0...v1.20.0-rc.0
[v1.20.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.20.0-rc.0...v1.20.1-rc.0
[v1.21.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.20.1-rc.0...v1.21.0-rc.0
[v1.22.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.21.0-rc.0...v1.22.0-rc.0
[v1.22.3-rc.0]: https://github.com/t3rn/t3rn/compare/v1.22.2-rc.0...v1.22.3-rc.0
[v1.23.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.22.3-rc.0...v1.23.0-rc.0
[v1.24.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.23.0-rc.0...v1.24.0-rc.0
[v1.25.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.24.1-rc.0...v1.25.0-rc.0
[v1.25.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.25.0-rc.0...v1.25.1-rc.0
[v1.26.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.25.1-rc.0...v1.26.0-rc.0
[v1.27.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.26.0-rc.0...v1.27.0-rc.0
[v1.27.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.27.0-rc.0...v1.27.1-rc.0
[v1.28.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.27.2-rc.0...v1.28.0-rc.0
[v1.29.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.28.0-rc.0...v1.29.0-rc.0
[v1.29.2-rc.0]: https://github.com/t3rn/t3rn/compare/v1.29.1-rc.0...v1.29.2-rc.0
[v1.30.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.29.2-rc.0...v1.30.0-rc.0
[v1.31.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.30.0-rc.0...v1.31.0-rc.0
[v1.31.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.31.0-rc.0...v1.31.1-rc.0
[v1.32.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.31.1-rc.0...v1.32.0-rc.0
[v1.32.3-rc.0]: https://github.com/t3rn/t3rn/compare/v1.32.2-rc.0...v1.32.3-rc.0
[v1.33.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.32.3-rc.0...v1.33.0-rc.0
[v1.34.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.33.1-rc.0...v1.34.0-rc.0
[v1.34.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.34.0-rc.0...v1.34.1-rc.0
[v1.34.2-rc.0]: https://github.com/t3rn/t3rn/compare/v1.34.1-rc.0...v1.34.2-rc.0
[v1.35.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.34.2-rc.0...v1.35.0-rc.0
[v1.36.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.35.0-rc.0...v1.36.0-rc.0
[v1.36.2-rc.0]: https://github.com/t3rn/t3rn/compare/v1.36.1-rc.0...v1.36.2-rc.0
[v1.36.4-rc.0]: https://github.com/t3rn/t3rn/compare/v1.36.3-rc.0...v1.36.4-rc.0
[v1.37.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.36.5-rc.0...v1.37.0-rc.0
[v1.38.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.37.1-rc.0...v1.38.0-rc.0
[v1.38.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.38.0-rc.0...v1.38.1-rc.0
[v1.38.2-rc.0]: https://github.com/t3rn/t3rn/compare/v1.38.1-rc.0...v1.38.2-rc.0
[v1.39.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.38.5-rc.0...v1.39.0-rc.0
[v1.39.3-rc.0]: https://github.com/t3rn/t3rn/compare/v1.39.2-rc.0...v1.39.3-rc.0
[v1.39.4-rc.0]: https://github.com/t3rn/t3rn/compare/v1.39.3-rc.0...v1.39.4-rc.0
[v1.40.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.39.4-rc.0...v1.40.0-rc.0
[v1.41.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.40.0-rc.0...v1.41.0-rc.0
[v1.42.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.41.1-rc.0...v1.42.0-rc.0
[v1.43.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.42.1-rc.0...v1.43.0-rc.0
[v1.43.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.43.0-rc.0...v1.43.1-rc.0
[v1.43.3-rc.0]: https://github.com/t3rn/t3rn/compare/v1.43.2-rc.0...v1.43.3-rc.0
[v1.43.4-rc.0]: https://github.com/t3rn/t3rn/compare/v1.43.3-rc.0...v1.43.4-rc.0
[v1.44.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.43.4-rc.0...v1.44.0-rc.0
[v1.44.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.44.0-rc.0...v1.44.1-rc.0
[v1.44.2-rc.0]: https://github.com/t3rn/t3rn/compare/v1.44.1-rc.0...v1.44.2-rc.0
[v1.45.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.44.2-rc.0...v1.45.0-rc.0
[v1.45.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.45.0-rc.0...v1.45.1-rc.0
[v1.46.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.45.1-rc.0...v1.46.0-rc.0
[v1.46.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.46.0-rc.0...v1.46.1-rc.0
[v1.46.2-rc.0]: https://github.com/t3rn/t3rn/compare/v1.46.1-rc.0...v1.46.2-rc.0
[v1.47.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.46.2-rc.0...v1.47.0-rc.0
[v1.47.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.47.0-rc.0...v1.47.1-rc.0
[v1.47.2-rc.0]: https://github.com/t3rn/t3rn/compare/v1.47.1-rc.0...v1.47.2-rc.0
[v1.48.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.47.2-rc.0...v1.48.0-rc.0
[v1.48.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.48.0-rc.0...v1.48.1-rc.0
[v1.48.3-rc.0]: https://github.com/t3rn/t3rn/compare/v1.48.2-rc.0...v1.48.3-rc.0
[v1.48.4-rc.0]: https://github.com/t3rn/t3rn/compare/v1.48.3-rc.0...v1.48.4-rc.0
[v1.48.5-rc.0]: https://github.com/t3rn/t3rn/compare/v1.48.4-rc.0...v1.48.5-rc.0
[v1.48.6-rc.0]: https://github.com/t3rn/t3rn/compare/v1.48.5-rc.0...v1.48.6-rc.0
[v1.49.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.48.6-rc.0...v1.49.0-rc.0
[v1.50.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.49.0-rc.0...v1.50.0-rc.0
[v1.50.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.50.0-rc.0...v1.50.1-rc.0
[v1.50.2-rc.0]: https://github.com/t3rn/t3rn/compare/v1.50.1-rc.0...v1.50.2-rc.0
[v1.51.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.50.2-rc.0...v1.51.0-rc.0
[v1.52.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.51.0-rc.0...v1.52.0-rc.0
[v1.52.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.52.0-rc.0...v1.52.1-rc.0
[v1.52.2-rc.0]: https://github.com/t3rn/t3rn/compare/v1.52.1-rc.0...v1.52.2-rc.0
[v1.52.3-rc.0]: https://github.com/t3rn/t3rn/compare/v1.52.2-rc.0...v1.52.3-rc.0
[v1.52.4-rc.0]: https://github.com/t3rn/t3rn/compare/v1.52.3-rc.0...v1.52.4-rc.0
[v1.53.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.52.4-rc.0...v1.53.0-rc.0
[v1.54.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.53.0-rc.0...v1.54.0-rc.0
[v1.54.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.54.0-rc.0...v1.54.1-rc.0
[v1.54.2-rc.0]: https://github.com/t3rn/t3rn/compare/v1.54.1-rc.0...v1.54.2-rc.0
[v1.54.3-rc.0]: https://github.com/t3rn/t3rn/compare/v1.54.2-rc.0...v1.54.3-rc.0
[v1.55.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.54.3-rc.0...v1.55.0-rc.0
[v1.55.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.55.0-rc.0...v1.55.1-rc.0
[v1.55.2-rc.0]: https://github.com/t3rn/t3rn/compare/v1.55.1-rc.0...v1.55.2-rc.0
[v1.56.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.55.2-rc.0...v1.56.0-rc.0
[v1.56.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.56.0-rc.0...v1.56.1-rc.0
[v1.57.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.56.3-rc.0...v1.57.0-rc.0
[v1.58.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.57.0-rc.0...v1.58.0-rc.0
[v1.58.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.58.0-rc.0...v1.58.1-rc.0
[v1.59.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.58.2-rc.0...v1.59.0-rc.0
[v1.60.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.59.0-rc.0...v1.60.0-rc.0
[v1.61.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.60.0-rc.0...v1.61.0-rc.0
[v1.62.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.61.0-rc.0...v1.62.0-rc.0
[v1.63.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.62.0-rc.0...v1.63.0-rc.0
[v1.63.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.63.0-rc.0...v1.63.1-rc.0
[v1.64.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.63.2-rc.0...v1.64.0-rc.0
[v1.64.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.64.0-rc.0...v1.64.1-rc.0
[v1.65.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.64.1-rc.0...v1.65.0-rc.0
[v1.65.2-rc.0]: https://github.com/t3rn/t3rn/compare/v1.65.1-rc.0...v1.65.2-rc.0
[v1.66.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.65.2-rc.0...v1.66.0-rc.0
[v1.66.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.66.0-rc.0...v1.66.1-rc.0
[v1.67.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.66.1-rc.0...v1.67.0-rc.0
[v1.67.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.67.0-rc.0...v1.67.1-rc.0
[v1.68.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.67.1-rc.0...v1.68.0-rc.0
[v1.69.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.68.0-rc.0...v1.69.0-rc.0
[v1.69.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.69.0-rc.0...v1.69.1-rc.0
[v1.70.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.69.1-rc.0...v1.70.0-rc.0
[v1.70.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.70.0-rc.0...v1.70.1-rc.0
[v1.70.2-rc.0]: https://github.com/t3rn/t3rn/compare/v1.70.1-rc.0...v1.70.2-rc.0
[v1.71.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.70.2-rc.0...v1.71.0-rc.0
[v1.72.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.71.0-rc.0...v1.72.0-rc.0
[v1.72.2-rc.0]: https://github.com/t3rn/t3rn/compare/v1.72.1-rc.0...v1.72.2-rc.0
[v1.73.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.72.2-rc.0...v1.73.0-rc.0
[v1.74.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.73.0-rc.0...v1.74.0-rc.0
[v1.74.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.74.0-rc.0...v1.74.1-rc.0
[v1.74.2-rc.0]: https://github.com/t3rn/t3rn/compare/v1.74.1-rc.0...v1.74.2-rc.0
[v1.74.3-rc.0]: https://github.com/t3rn/t3rn/compare/v1.74.2-rc.0...v1.74.3-rc.0
[v1.74.9-rc.0]: https://github.com/t3rn/t3rn/compare/v1.74.8-rc.0...v1.74.9-rc.0
[v1.75.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.74.10-rc.0...v1.75.0-rc.0
[v1.76.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.75.3-rc.0...v1.76.0-rc.0
[v1.77.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.76.0-rc.0...v1.77.0-rc.0
[v1.78.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.77.0-rc.0...v1.78.0-rc.0
[v1.79.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.78.0-rc.0...v1.79.0-rc.0
[v1.80.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.79.0-rc.0...v1.80.0-rc.0
[v1.81.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.80.0-rc.0...v1.81.0-rc.0
[v1.81.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.81.0-rc.0...v1.81.1-rc.0
[v1.81.2-rc.0]: https://github.com/t3rn/t3rn/compare/v1.81.1-rc.0...v1.81.2-rc.0
[v1.81.3-rc.0]: https://github.com/t3rn/t3rn/compare/v1.81.2-rc.0...v1.81.3-rc.0
[v1.82.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.81.4-rc.0...v1.82.0-rc.0
[v1.82.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.82.0-rc.0...v1.82.1-rc.0
[v1.83.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.82.1-rc.0...v1.83.0-rc.0
[v1.83.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.83.0-rc.0...v1.83.1-rc.0
[v1.84.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.83.1-rc.0...v1.84.0-rc.0
[v1.85.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.84.1-rc.0...v1.85.0-rc.0
[v1.86.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.85.0-rc.0...v1.86.0-rc.0
[v1.86.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.86.0-rc.0...v1.86.1-rc.0
[v1.87.0-rc.0]: https://github.com/t3rn/t3rn/compare/v1.86.1-rc.0...v1.87.0-rc.0
[v1.87.1-rc.0]: https://github.com/t3rn/t3rn/compare/v1.87.0-rc.0...v1.87.1-rc.0
[v1.87.2-rc.0]: https://github.com/t3rn/t3rn/compare/v1.87.1-rc.0...v1.87.2-rc.0
[v1.87.3-rc.0]: https://github.com/t3rn/t3rn/compare/v1.87.2-rc.0...v1.87.3-rc.0
[v1.87.4-rc.0]: https://github.com/t3rn/t3rn/compare/v1.87.3-rc.0...v1.87.4-rc.0