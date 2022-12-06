Description: Smoke test - parachains are up and registered
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