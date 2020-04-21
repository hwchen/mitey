# mitey

A web framework for Rust.

Core values:
- Minimal abstractions.
- Minimal layers.
- Compatibility with the widest range of async runtimes and io primitives.
- You call the library, instead of the framework calling you.

My hope is that ease of use comes out of being able to understand the system, instead of by hiding complexity behind an ergonomic interface.

In addition, in order to be able to interact with the widest range of async runtimes and io primitives, there will be some necessary boilerplate, as well possibly implementing some executor details.

## Web Framework Random Thoughts

The underlying goal is to provide the basics of routing and state management without interfering with the user's ability to when and where the web server interacts with async runtimes. For example, most current web frameworks have a `run` or `serve` method which is the final blocking call in `fn main`, and it basically spins up a loop for spawning new tasks (http connections). The web framework controls that loop, so it's not possible for another server to have its own loop to spawn tasks, unless it's on another thread.

But there's no particular reason that a web framework needs to monopolize this task-spawning loop. For example, if you want different parts of the app to bind to multiple ports with different protocols. Mitey gets out of the way, and lets the programmer orchestrate all the io streams as necessary, and only steps in to route and handle the http requests on whatever stream is needed.

The programmer is also able to orchestrate threads and async runtimes in whatever configuration is necessary for highest performance, Mitey can be passed around, refernced, cloned, or whatever is necessary. Mitey does not impose a runtime architecture.

In essence, for complex apps, Mitey will help you feel like it's just a part of the system, as opposed a force you have to either go along with or work around.

## Previous random thoughts

Is it possible to provide a web framework so minimal that execution details can be controlled by the user, only calling into the web framework for convenience? What is the minimal feature set? And who would use this kind of library?

For basic REST apis and CRUD frameworks, people probably want a simple all-in-one, so something like actix, rocket or warp would be the easiest. They provide routing, middleware, and more convenient error handling. It's easy for users to access Req, Resp, state, and router vars all in one function, and just write that fn.

The main pain point for non-power users is trying to incorporate a library that depends on a different runtime?

For power users, they would already go down to hyper, and maybe write their own routing library, and just do error handling on their own, perhaps with their own wrapper.

Well, whatever. Still going to give a shot at a cross-executor web framework.
