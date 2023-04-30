Description: Smoke test - parachains are up and registered
Network: ../zombienet.toml
Creds: config

alice: is up
bob: is up

alice: parachain 3333 is registered within 225 seconds
alice: parachain 3334 is registered within 225 seconds

{% set nodes = ["t3rn-collator01", "t3rn-collator02"] %}
{% for node in nodes %}
{{node}}: reports block height is at least 5 within 250 seconds
{% endfor %}

{% set nodes = ["t0rn-collator01", "t0rn-collator02"] %}
{% for node in nodes %}
{{node}}: reports block height is at least 5 within 250 seconds
{% endfor %}
#  Given the setup of rangers + executors + single transfer SFX submission can be confirmed
collator02: js-script ./confirm_single_optimistic_transfer_sfx.js within 200 seconds
