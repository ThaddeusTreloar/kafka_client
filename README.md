# kafka_client

Kafka Client is an attempt to create a Rust native client library for Apache Kafka, along with high-level streams DSL feature.

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

// TODO

#Contributions

Contributions are welcome, please open a feature issue with your intended implementation, then lodge a pull request.
