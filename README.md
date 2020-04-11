# mitey

Small and mighty.

Yet another web framework (in Rust).

Created on the smallest, simplest, and most transparent async stack possible.

Designed to be executor agnostic, made with `smol` in mind. Initially developed on `async-std` and `async-h1`.

No middlewares?

Component libraries:
- `mitey-router` (regex based, normalizes paths)

# POC
So far I've made the library "independent" of the tcp stream. I can see how it should be able to hook into any tcp stream that implements AsyncWrite and AsyncRead (or even better, async-ified std::TcpStream via smol). But now I need to figure out how to plug an executor into the library? Or am I missing another step.
