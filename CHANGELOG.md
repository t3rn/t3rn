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



