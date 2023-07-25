# kafka_client

Kafka Client is an attempt to create a Rust native client library for Apache Kafka, along with high-level streams DSL feature.

API structure currently being scaffolded.

Currently there are 3 main libraries that can be used to create Kafka applications in Rust:
 - rdkafka: https://github.com/fede1024/rust-rdkafka:
     - A set of bindings to the librdkafka C library
     - Not too bad a library, but being that it is a library of bindings, 
        it can get be unwieldy at times
     - Can potentially require `unsafe` code to interact with
 - kafka-rust: https://github.com/kafka-rust/kafka-rust
     - Unmaintained at this current point in time
     - Library maintainer has mentioned that they are intending to resume
        maintenance in September of 2023: https://github.com/kafka-rust/kafka-rust/issues/219#issuecomment-1640909927
 - callysto: https://github.com/vertexclique/callysto
     - A promising high-level streams style library
     - The completed features appear to be specific to a particular company, or set thereof
     - Very little activity since late 2022
 - kafkas: https://github.com/iamazy/kafkas
     - Uses kafka-protocol-rs for Wire protocol calls
     - Seems like it was heading towards an excellent option, however, seems
        to no longer be maintained
     - Doesn't appear to have any API documentation

One of the core intentions of this library is to provide as much compile-time safety to the API interfaces as possible, 
in order to reduce testing requirements for library users. Examples can be seen in the implementations of configurations
where all configs except for the RawConfig type are type safe and provide TryFrom\<RawConfig\> implementations for 
interoperability with existing kafka app configuration files. Another example is the consumer interface being split into
different traits:
 - Unallocated: unsubscribed, unassigned
 - SubscriberConsumer: subscribed, cannot be assigned
 - AssignedConsumer: assigned, cannot be subscribed
 - AsyncConsumer: Consumer running on an asynchronous runtime, Cannot wake
 - SyncConsumer: Blocking API calls

The intention is to use https://github.com/tychedelia/kafka-protocol-rs as an accelerator to help with protocol relevant interfaces. Some interfaces may need to be wrapped in order to meet the libraries safety goals. It should be noted that
certain additions should be made back to kafka-protocol-rs, specifically things like additional compression algos, and
the like.

async_traits is currently being used to provide async functionality to trait objects. This will be migrated to std native
when stabilized (hopefully in 1.74.0) https://blog.rust-lang.org/inside-rust/2023/05/03/stabilizing-async-fn-in-trait.html.

APIs are considered stable if they provide a clean way for a user to interact with object, without excessive boilerplate;
Things like Box, Pin, raw pointers, obscure object. Interfaces should also provide excellent resilience to misuse by API users.

// TODO

#Contributions

Contributions are welcome, please open a feature issue with your intended implementation, then lodge a pull request.
