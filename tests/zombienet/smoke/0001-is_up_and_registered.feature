Description: Smoke test - parachains are up and registered
Network: ../../zombienet.toml

alice: is up
bob: is up

alice: parachain 3000 is registered within 225 seconds
alice: parachain 4000 is registered within 225 seconds

{% set nodes = ["t3rn01-collator01", "t3rn01-collator02", "t3rn02-collator01", "t3rn02-collator02"] %}
{% for node in nodes %}
{{node}}: reports block height is at least 5 within 250 seconds
{% endfor %}