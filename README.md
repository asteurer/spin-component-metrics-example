# Overview
This is an example application that is used for testing a new feature of Spin (see [issue #2933](https://github.com/spinframework/spin/issues/2933)).

## Prerequisites
- Build [this fork of Spin](https://github.com/asteurer/spin/tree/component-metrics) with this command: 
```sh
cargo build --features call-hook
```

## Usage
Using the above build of Spin, run the example application and invoke it:
```sh
spin build && spin up

# In a different terminal...
curl localhost:3000
```

After the application has been invoked, stop the running application with `CTRL + C`. Spin will then print the CPU and memory that the Spin application used.
