Description: Runtime Upgrade test
Network: ../zombienet.toml
Creds: config

alice: is up
bob: is up

alice: parachain 3000 is registered within 225 seconds
alice: parachain 4000 is registered within 225 seconds

{% set nodes = ["t0rn-collator01", "t0rn-collator02", "t3rn-collator01", "t3rn-collator02"] %}
{% for node in nodes %}
{{node}}: reports block height is at least 5 within 250 seconds
{% endfor %}

t0rn-collator01: parachain 3000 perform upgrade with ../../../target/release/wbuild/t0rn-parachain-runtime/t0rn_parachain_runtime.compact.compressed.wasm within 200 seconds
t0rn-collator02: reports block height is at least 20 within 250 seconds
t0rn-collator02: js-script ./runtime_upgrade.js within 200 seconds
t0rn-collator02: reports block height is at least 50 within 1000 seconds

t3rn-collator01: parachain 4000 perform upgrade with ../../../target/release/wbuild/t3rn-parachain-runtime/t3rn_parachain_runtime.compact.compressed.wasm within 200 seconds
t3rn-collator02: reports block height is at least 20 within 250 seconds
t3rn-collator02: js-script ./runtime_upgrade.js within 200 seconds
t3rn-collator02: reports block height is at least 50 within 1000 seconds
