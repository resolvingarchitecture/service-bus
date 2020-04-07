<div align="center">
  <img src="https://resolvingarchitecture.io/images/ra.png"  />

  <h1>Resolving Architecture</h1>

  <p>
    <strong>Clarity in Design</strong>
  </p>
  
  <h2>Service Bus</h2>
  
  <p>
   A form of bus that manages services routing messages between them where each service gets two channels (in/out)
   using a SEDA bus.
   </p>
  
  <p>
    <a href="https://travis-ci.com/resolvingarchitecture/service-bus"><img alt="build" src="https://img.shields.io/travis/resolvingarchitecture/service-bus"/></a>
    <a href="https://crates.io/crates/service-bus"><img alt="Crate Info" src="https://img.shields.io/crates/v/service-bus.svg"/></a>
    <a href="https://docs.rs/crate/service-bus/"><img alt="API Docs" src="https://img.shields.io/badge/docs.service-bus-green"/></a>
  </p>
  <p>
    <a href="https://github.com/resolvingarchitecture/service-bus/blob/master/LICENSE"><img alt="License" src="https://img.shields.io/github/license/resolvingarchitecture/service-bus"/></a>
    <a href="https://resolvingarchitecture.io/ks/publickey.brian@resolvingarchitecture.io.asc"><img alt="PGP" src="https://img.shields.io/keybase/pgp/objectorange"/></a>
  </p>
  <p>
    <img alt="commits" src="https://img.shields.io/crates/d/service-bus"/>
    <img alt="repo size" src="https://img.shields.io/github/repo-size/resolvingarchitecture/service-bus"/>
  </p>
  <p>
    <img alt="num lang" src="https://img.shields.io/github/languages/count/resolvingarchitecture/service-bus"/>
    <img alt="top lang" src="https://img.shields.io/github/languages/top/resolvingarchitecture/service-bus"/>
    <a href="https://blog.rust-lang.org/2020/03/12/Rust-1.42.html"><img alt="Rustc Version 1.42+" src="https://img.shields.io/badge/rustc-1.42+-green.svg"/></a>
  </p>

  <h4>
    <a href="https://resolvingarchitecture.io">Info</a>
    <span> | </span>
    <a href="https://docs.rs/crate/service-bus/">Docs</a>
    <span> | </span>
    <a href="https://github.com/resolvingarchitecture/service-bus/blob/master/CHANGELOG.md">Changelog</a>
  </h4>
</div>

## Donate
Request BTC/XMR/ZEC address for a donation at brian@resolvingarchitecture.io.

## Notes
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