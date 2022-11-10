Description: Runtime Upgrade test
Network: ../../zombienet.toml

alice: is up
bob: is up

alice: parachain 3000 is registered within 225 seconds
alice: parachain 4000 is registered within 225 seconds

{% set nodes = ["t3rn01-collator01", "t3rn01-collator02", "t3rn02-collator01", "t3rn02-collator02"] %}
{% for node in nodes %}
{{node}}: reports block height is at least 5 within 250 seconds
{% endfor %}

t3rn01-collator01: parachain 3000 perform upgrade with /tmp/wasm_binary_spec_version_incremented.rs.compact.compressed.wasm within 200 seconds

t3rn01-collator02: reports block height is at least 20 within 250 seconds
t3rn01-collator02: js-script ./runtime_upgrade.js within 200 seconds
t3rn01-collator02: reports block height is at least 50 within 1000 seconds
