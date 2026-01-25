## Testing and Building Rule

After every code change, the following steps must be performed:

1. Run `cargo build` to ensure the code compiles
2. Run `cargo test` to ensure all tests pass

This ensures that changes don't introduce compilation errors or break existing functionality.


## ThreadRng rule

Use this rule when using the ThreadRng crate

When using `ThreadRng` module
1. the `rand::thread_rng` method has been deprecated and renamed `rand::Rng`
2. the `rand::Rng::gen_range` method is deprecated and renamed `random_range`
