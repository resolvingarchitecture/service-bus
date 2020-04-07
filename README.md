[![Build Status](https://travis-ci.com/resolvingarchitecture/service-bus.svg?branch=master)](https://travis-ci.com/resolvingarchitecture/service-bus)
# Service Bus
A form of bus that manages services routing messages between them where each service gets two channels (in/out)
using a SEDA bus.

[Crates.io](https://crates.io/crates/service_bus)

!! WIP - not stable until version 1.0 !!

## Design Goals

*[x] Minimal light-weight service bus
*[x] provide service un-/registration
*[x] Use seda bus underneath
*[ ] Implement Enterprise Integration pattern routing between services
    *[x] Dynamic Slip
*[ ] enable inter-service pipelines for cross-concerns like security and pub/sub
*[ ] Support notification pub/sub
*[ ] support full life-cycle management of service bus