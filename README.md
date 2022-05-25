# eAIP-rs

## Overview

eAIP is a library that provides utilities, and abstracts access to electronic Aeronautical
Information Packages (eAIPs). It includes a lot of functions to collect navigational data into
convenient types, and for sections which have fewer pre-written tools (for example, operational data
regarding aerodromes), tools are provided to fetch the pages from each Aeronautical Information Service
(AIS) for processing yourself.

## ⚠️ Important Notice

Although data provided by this library is sourced from real-world sources, this data is not suitable for
use in real-world flight, particularly as errors in this library could cause corruption of vital data.

**Do not use for real world flight.**

## Getting Started

See the `examples/` directory for examples of how to fetch the data, or view the
[documentation](https://docs.rs/eaip).

## Considerations

Do consider copyright over the data this library can access. This library only accesses the live data and
parses it into useful, machine-readable data. If you use this library to store the data in any fashion, be
aware that it may be under various licenses.

## Tested Against

The following is a list of AISs that this library is tested to work against.

- NATS (UK, working 2022-05-25)
