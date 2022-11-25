---
sidebar_position: 3
---

# XBI in the future

Here we will lay out other foundations concerning improvements on XBI; we will likely submit further PSPs to support hardening the specification and fulfilling the goal of a specific, extensible smart contract messaging interface over XCM.

#### Discoverable and Dynamically updated schemas

Web2 presented several patterns that were an integral building block for developers, one of them being the rise of [OpenAPI](https://swagger.io/specification/) and API-driven design. We aim for XBI to facilitate the discoverability of such a dynamic API, focusing on schema upgrades without upgrading the runtime. This approach is much like the usability of [GraphQL](https://graphql.org/learn/schema/), where developers provide as many handlers in business logic as possible, whilst maintainers can modify the schema at runtime or even create new ways to join and handle the data.

### XBI-as-a-module

Whilst XBI can and should easily be implemented as a pallet that developers can reuse. We also strive to introduce XBI as a set of modules. We think a general approach would allow as much adoption and ease of use as possible, with minimal code changes. Namely, a Transmitter(enter)/Receiver(exit) pair with a set of interfaces allowing as much customization as possible.

Some examples are:
- custom serialization layers
- custom storage approaches
- only transmitters
- only receivers
