This repo exists to reproduce https://github.com/tokio-rs/axum/issues/145

Uncommenting the `let app = app.handle_error [..]` section has a big impact on
compile times.

On my machine it goes from under 1 second when not included, to 4.5 seconds with
the `handle_error` call included.
