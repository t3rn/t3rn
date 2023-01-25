# [1.2.0-rc.9](https://github.com/t3rn/t3rn/compare/v1.2.0-rc.8...v1.2.0-rc.9) (2023-01-25)


### Features

* github action stale issues ([c20fbbe](https://github.com/t3rn/t3rn/commit/c20fbbec5ecdfb2820788b70a34124e062816bd3))



# [1.2.0-rc.8](https://github.com/t3rn/t3rn/compare/v1.2.0-rc.7...v1.2.0-rc.8) (2023-01-24)


### Bug Fixes

* correct tests expectations after existential deposit=1 added ([0d66472](https://github.com/t3rn/t3rn/commit/0d6647286d750438d63f09772f31d3c2c414844a))



# [1.2.0-rc.7](https://github.com/t3rn/t3rn/compare/v1.2.0-rc.6...v1.2.0-rc.7) (2023-01-24)


### Bug Fixes

* subalfred errors out correctly ([#596](https://github.com/t3rn/t3rn/issues/596)) ([eaa03a4](https://github.com/t3rn/t3rn/commit/eaa03a425ecaa6c3322389be595af34391cf12e3))



# [1.2.0-rc.6](https://github.com/t3rn/t3rn/compare/v1.2.0-rc.5...v1.2.0-rc.6) (2023-01-23)


### Bug Fixes

* rm primitives import ([99f0596](https://github.com/t3rn/t3rn/commit/99f0596c1c91325cbb8a3eedbda8b23d1199c486))



# [1.2.0-rc.5](https://github.com/t3rn/t3rn/compare/v1.2.0-rc.4...v1.2.0-rc.5) (2023-01-20)


### Bug Fixes

* bidding for multiple sfxs ([c734d46](https://github.com/t3rn/t3rn/commit/c734d466aad47091b0429b2db1d9841d2835301a))
* change alias url for deploying docs ([951d15a](https://github.com/t3rn/t3rn/commit/951d15af7376a6cdab73662837514d246365c01b))
* conventional changelog need to have valid commit signature ([#591](https://github.com/t3rn/t3rn/issues/591)) ([e8e7a5e](https://github.com/t3rn/t3rn/commit/e8e7a5e96635862e34bb0d4092e4987bceb1ddf5))
* droppedAtBidding slashing issue ([0466fa0](https://github.com/t3rn/t3rn/commit/0466fa0521e52b8f14c1842adedc181c733d6347))
* enforce all SFX to have the same reward asset field ([ae47ae3](https://github.com/t3rn/t3rn/commit/ae47ae333c98ba7f0748aaf2f7d63dbc5d2e0f6a))
* enforce encoded arguments length to match gatewayabiconfig ([9dfb2f5](https://github.com/t3rn/t3rn/commit/9dfb2f5db4de627479c800ca1a47b986b8347d08)), closes [#403](https://github.com/t3rn/t3rn/issues/403) [#403](https://github.com/t3rn/t3rn/issues/403) [#403](https://github.com/t3rn/t3rn/issues/403) [#403](https://github.com/t3rn/t3rn/issues/403)
* make monetary::deposit_immidiately fallible ([5a340e0](https://github.com/t3rn/t3rn/commit/5a340e0e182caa158bd20505f02797ef76e0a95c))
* makefile setup step should be a dependency for tests ([a5c30a2](https://github.com/t3rn/t3rn/commit/a5c30a20c6ad0960388967e59ad8a685ed98083b))
* remove leftover from merge ([ccf5e1b](https://github.com/t3rn/t3rn/commit/ccf5e1b273bf0248c0bd5cd5b856bb7e3e8536e1))
* revert docs alias domain back to old ([97fbce9](https://github.com/t3rn/t3rn/commit/97fbce9ecd18b850c551d55aeee20e7bb0e330b5))
* state machine never reaching terminal state, causing reverts on confirmed and finalized XTXs ([2a92dd9](https://github.com/t3rn/t3rn/commit/2a92dd91f477dfcea7886e4e5ddab3651de47bc4))
* typos ([83206c1](https://github.com/t3rn/t3rn/commit/83206c1bae769bd0e40e7026fa543dbb79dc932a))
* update docs deployment link ([c1873a6](https://github.com/t3rn/t3rn/commit/c1873a606c330eb31d2a4ab6cc5dc3ed27d4d1e4))
* use SFXBid::reward_asset_id field at bidding ([c686ccd](https://github.com/t3rn/t3rn/commit/c686ccd9bd54c6096df87366cca72429db6bb494))


### Features

* add optional asset id to AccountManager deposits ([6726b65](https://github.com/t3rn/t3rn/commit/6726b654ccec969bb92bf7e1803ac78569498ba8))
* add optional SFX field for reward asset id ([9a35df4](https://github.com/t3rn/t3rn/commit/9a35df41ec43693d2e964abbcf81e67a7535bfc9))
* add pr title lint ([8231c75](https://github.com/t3rn/t3rn/commit/8231c75dfd8c41683d1f2d9aa48ea0368380bd6c))
* add subalfred gh action ([#579](https://github.com/t3rn/t3rn/issues/579)) ([dceb2db](https://github.com/t3rn/t3rn/commit/dceb2db083f6f6918a27956371be01c637e9788a))
* all runtimes incl a parity treasury ([542bbea](https://github.com/t3rn/t3rn/commit/542bbea88dbcf79a183162c7a0ff83d7d62251f4))
* change monetary deposits to infallible with Unbalanced ([0326488](https://github.com/t3rn/t3rn/commit/03264883e18b59d077e4a2da3b5891417cca3890))
* conventional changelog GHA creating tag and markdown file  ([#587](https://github.com/t3rn/t3rn/issues/587)) ([fb8f117](https://github.com/t3rn/t3rn/commit/fb8f1172112073d5b09fd296e92c5e1d95bf5c54))
* extend AccountManager with monetary handling assets and native ([a93005e](https://github.com/t3rn/t3rn/commit/a93005eef4bd6f27e6661fd8290a5461f5fa2d3d))
* extend SFXBid with reserved asset field ([9464fe6](https://github.com/t3rn/t3rn/commit/9464fe68cb2d4f3397fa70a3076f3b7d4e7f7ac6))
* safe math operations ([b1112d4](https://github.com/t3rn/t3rn/commit/b1112d45185e50efe30d60d7ea4aad83231543a6))
* slash/repatriate optimistic SFX to executors with foreign assets ([de6bda9](https://github.com/t3rn/t3rn/commit/de6bda97d770084443939e1f9f605dd3b5694758))
* update event emission and adds test ([dd705f0](https://github.com/t3rn/t3rn/commit/dd705f03363e67bba4436974c2284ad9b08a38ba))
* update subalfred false positives list ([eb5294d](https://github.com/t3rn/t3rn/commit/eb5294d9575e1215c06068227703117b340acff8))
* use monetary submodule to reserve SFX requesters ([5b885c3](https://github.com/t3rn/t3rn/commit/5b885c369ec8894df75a5e9026f8fc5c8d60ebbc))
* use multiasset monetary for optimistic dropped bids ([653e0b6](https://github.com/t3rn/t3rn/commit/653e0b62e8d09dfb358dce20dfee0815f8fe097f))
* xbi portal, remove escrowed & fixes to dependencies ([a665590](https://github.com/t3rn/t3rn/commit/a665590c85e5d0c7b343bc52acd848469d6a50df)), closes [#07cd855](https://github.com/t3rn/t3rn/issues/07cd855)


### BREAKING CHANGES

* balance of has been removed
* escrowed has been removed

* chore: pr comments

* chore: remove frontier



