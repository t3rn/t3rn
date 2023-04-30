Description: Given the setup of rangers + executors + single transfer SFX submission can be confirmed
Network: ../zombienet.toml
Creds: config

alice: is up
bob: is up

alice: parachain 3000 is registered within 225 seconds

{% set nodes = ["collator01", "collator02"] %}
{% for node in nodes %}
{{node}}: reports block height is at least 3 within 225 seconds
{% endfor %}

collator02: bash-script ./confirm_single_optimistic_transfer_sfx.js within 200 seconds
collator02: js-script ./confirm_single_optimistic_transfer_sfx.js within 200 seconds
collator02: reports block height is at least 15 within 200 seconds