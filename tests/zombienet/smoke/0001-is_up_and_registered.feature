Description: Smoke test - parachain is up and registered
Network: ../zombienet.toml
Creds: config

alice: is up
bob: is up

alice: parachain 3333 is registered within 225 seconds

{% set nodes = ["collator01", "collator02"] %}
{% for node in nodes %}
{{node}}: reports block height is at least 5 within 250 seconds
{% endfor %}
