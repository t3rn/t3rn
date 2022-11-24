Description: Runtime Upgrade test
Network: ../zombienet-real-upgrade-sim.toml
Creds: config

alice: is up
bob: is up

alice: parachain 3000 is registered within 225 seconds

{% set nodes = ["collator01", "collator02"] %}
{% for node in nodes %}
{{node}}: reports block height is at least 5 within 250 seconds
{% endfor %}

collator01: parachain 3000 perform upgrade with ../../../target/release/wbuild/parachain_runtime.compact.compressed.wasm within 200 seconds
collator02: reports block height is at least 20 within 250 seconds
collator02: js-script ./runtime_upgrade.js within 200 seconds
collator02: reports block height is at least 50 within 1000 seconds