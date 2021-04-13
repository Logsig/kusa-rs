**KUSA**

A High performance IoT Server.

![build-and-test](https://github.com/Logsig/kusa-rs/workflows/build-and-test/badge.svg?event=push)


---
How to run server:
`cargo run`

---
Server command(telnet to port 6142)

`ping: ping-pong server`

`quit: exit server`

---

Developer: Logsig

TODO:

    - Spin up broker
    - Implement MQTT5 base Codec
    - Implement MQTT5 CONNECT
    - Implement protocol handler for subscribe topics
    - Implement protocol handler for publish topics


Roadmap

- v0.1.x Build up the prototype and lay down the framework.
- v0.2.0 Publish to github project.

- v0.2.1 Basic MQTT5 Protocol namely CONNECT, DISCONNET, PUB, SUB
- v0.2.4 Complete MQTT5 Protocol
- v0.2.5 Communicatable via WebSocket
- v0.2.6 Communicatable via REST API
- v0.3.0 Documentation with Test Suite
- v0.3.1 Benchmark Test Suite