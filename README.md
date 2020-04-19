# mitey

Small and mighty.

Yet another web framework (in Rust).

Created on the smallest, simplest, and most transparent async stack possible.

Is it possible to provide a web framework so minimal that execution details can be controlled by the user, only calling into the web framework for convenience? What is the minimal feature set? And who would use this kind of library?

For basic REST apis and CRUD frameworks, people probably want a simple all-in-one, so something like actix, rocket or warp would be the easiest. They provide routing, middleware, and more convenient error handling. It's easy for users to access Req, Resp, state, and router vars all in one function, and just write that fn.

The main pain point for non-power users is trying to incorporate a library that depends on a different runtime.

For power users, they would already go down to hyper, and maybe write their own routing library, and just do error handling on their own, perhaps with their own wrapper. 

// Designed for smol, an async runtime which supports async-ifies libraries instead of requiring libraries to depend on a particular runtime's implementation. (which creates a locked-in ecosystem).

Currently developed on async-h1, a minimal http parser.

No middlewares?

Component libraries:
- `mitey-router` (regex based, normalizes paths)

# POC
So far I've made the library "independent" of the tcp stream. I can see how it should be able to hook into any tcp stream that implements AsyncWrite and AsyncRead (or even better, async-ified std::TcpStream via smol). But now I need to figure out how to plug an executor into the library? Or am I missing another step.
