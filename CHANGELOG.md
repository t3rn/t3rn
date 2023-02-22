## [1.3.0-rc.0](https://github.com/t3rn/t3rn/compare/1.2.0-rc.5...1.3.0-rc.0) (2023-02-22)


### features

* add logging for ranger monitoring ([cbb23a8](https://github.com/t3rn/t3rn/commit/cbb23a8758c39f3ca30dea254d508b02c66f802a))
* added inital workflow for building docker ([bd21797](https://github.com/t3rn/t3rn/commit/bd217974f354477594b11fa40e9c6bd82d2c889b))
* foreign fee split ([f6659e0](https://github.com/t3rn/t3rn/commit/f6659e099923b3b822b0eb26a1828d5dfa0994c0))
* implement confirm() and validate() as part of sfx struct ([313aa8c](https://github.com/t3rn/t3rn/commit/313aa8c39244e537d2b2ad622f0a6e6118ede6ac))
* introduce onhook traits to clock ([84de0ab](https://github.com/t3rn/t3rn/commit/84de0ab6c0be354fffc854adb5f0189dbfa011ba))
* multistage docker build for node standalone ([0bd169e](https://github.com/t3rn/t3rn/commit/0bd169e2ae0a54d81f87f649896c9460b4f51ccb))
* standalone dockerfile ([d2071ee](https://github.com/t3rn/t3rn/commit/d2071eeb15ea3ccd648235f2114a24c548507a55))
* test harness with parachains integration tool ([#629](https://github.com/t3rn/t3rn/issues/629)) ([c1314e7](https://github.com/t3rn/t3rn/commit/c1314e73785f8dec49b57e97a33418fd09d24b2a))


### bug fixes

* division in fee calc ([b92aa1b](https://github.com/t3rn/t3rn/commit/b92aa1bdea4bf6bc9d311788e9f2fd7c2603182a))
* fix pendingstakerequestnotdueyet test setup ([d40dfc2](https://github.com/t3rn/t3rn/commit/d40dfc2c18d22c376c00480a6865b068379955ce))
* remove primary round key from pendingcharges ([3cf65c8](https://github.com/t3rn/t3rn/commit/3cf65c8876c501039ec05657bab04174f6d8b975))
* return 0 weight if on_init hook consumes more than allowed ([fa78011](https://github.com/t3rn/t3rn/commit/fa78011a0314682710032c104cee138587697abe))
* **t0rn:** add slow adjusting fee  ([9c7b5a5](https://github.com/t3rn/t3rn/commit/9c7b5a53b33a91a8dcf3d15c91e94d7b8363fca2))
* update sdk imports ([5b27826](https://github.com/t3rn/t3rn/commit/5b278267c82700aadd3e68498422d7fc88b49221))

# [1.2.0-rc.5] 2023-02-06
* fix: t0rn runtime upgrade

# [1.2.0-rc.4](https://github.com/t3rn/t3rn/compare/v1.2.0-rc.3...v1.2.0-rc.4) (2022-11-23)


### Features

* remove badblocks from chainspec ([b361f80](https://github.com/t3rn/t3rn/commit/b361f80b293124223233fd38313863bc3901cf60))



# [1.2.0-rc.3](https://github.com/t3rn/t3rn/compare/v1.2.0-rc.2...v1.2.0-rc.3) (2022-11-19)


### Bug Fixes

* align millis per block constant to 12000 across codebaseC ([48068df](https://github.com/t3rn/t3rn/commit/48068df610c2376dc4b053e43e4d3001a63d1e74))
* align millis per block constant to 12000 across codebaseC ([40d3898](https://github.com/t3rn/t3rn/commit/40d38986ed562c527114eb0891c7918dc1268c14))
* bump t0rn patchfix version and merge remedy chainspecs ([b47d699](https://github.com/t3rn/t3rn/commit/b47d6992661c273e5edfdf03f4b2183667a72512))
* ensureroot needs account type ([3f4f0f8](https://github.com/t3rn/t3rn/commit/3f4f0f83fae4a99352ddc55cc2495b31590b8973))
* ensureroot needs account type ([2d33ec9](https://github.com/t3rn/t3rn/commit/2d33ec9abd61b553cc2f3262d2f99116573a0bc6))
* imports were bad from find and replace ([a394a35](https://github.com/t3rn/t3rn/commit/a394a3507577a592b492c976184cbbca8195c8fc))
* imports were bad from find and replace ([7d81dfd](https://github.com/t3rn/t3rn/commit/7d81dfdeeda1af88e2c86e8f3efdf0ab37cbcaa6))
* remove bad_blocks from t0rn chain specs ([72c2062](https://github.com/t3rn/t3rn/commit/72c206295e2d5cf52630cce67f764ee49b175568))



# [1.2.0-rc.2](https://github.com/t3rn/t3rn/compare/v1.2.0-rc.1...v1.2.0-rc.2) (2022-11-15)


### Bug Fixes

* add bad_blocks extension to t0rn chain specs ([fb427e3](https://github.com/t3rn/t3rn/commit/fb427e3eb801449a4027951b3bd0a2469b8a16ac))



# [1.2.0-rc.1](https://github.com/t3rn/t3rn/compare/v1.2.0-rc.0...v1.2.0-rc.1) (2022-11-12)


### Reverts

* back to 12s block time ([1bb0db1](https://github.com/t3rn/t3rn/commit/1bb0db16221a7cdedd872679969b816beebb2797))



