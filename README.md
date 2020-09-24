This library is an experiment to make `core::{future, task}` available in a version that is not yet stable in the standard library.

The purpose of this library is to allow experiment with how the `Stream` trait re-export proposed in [rust-lang/futures-rs#2207] works.

In that proposal, once the std `Stream` trait is stable, `futures_core::Stream` is replaced with re-export from the standard library. In the version that has stable std `Stream`, std `Stream` and `futures_core::Stream` is the same trait.

### Known limitations/issues

Known limitations/issues when applying this way to `Stream` trait are:

* This way will not work if the std `Stream` trait changes after the `Stream` trait is published as part of `futures_core` v1. See also [this comment](https://github.com/rust-lang/futures-rs/issues/2207#issuecomment-687134204) in [rust-lang/futures-rs#2207].

* If use this way, the build script that detect std `Stream` must be enabled from the first release of that major version (even if the `Straem` trait is not yet implemented in `std`). Otherwise, the compile may fail if it relies on an older version of the library.

* If you rely on both the `Stream` trait provided by the futures_core and the *unstable* std `Stream`, the compile will successful while std `Stream` trait is unstable, the compile will fail when the Stream trait stabilizes.

  ```rust
  #![feature(stream_trait)]
  impl core::stream::Stream for Type { ... }
  impl futures_core::Stream for Type { ... }
  ```

  Note: This issue does not occur unless you rely on the unstable feature of the standard library. (They are the same trait from the beginning)

* If you are using a build system that does not support build scripts, if you don't update the compiler and dependencies at the same time, the compile will fail when the `Stream` trait stabilizes.

  Note: If build-script works, it should be able to detect it properly. So similar issues should not occur.

* If you are using a build system that does not support build scripts, a guaranteed MSRV is basically the latest stable compiler.

  (Of course, if build-script works, it can basically compile with older compilers.)

Known issues when using this crate with 1.33 (MSRV) - 1.35 compilers are:

* If you are using a build system that does not support build scripts, this crate will not be available in Rust 1.33 - 1.35.

* Features that depend on features that are not stable in those versions are not available:

  * `RawWakerVTable::new` is not `const` as function pointers in const fn are unstable in those version.
  * `Try` trait is not stable yet. so the following impls does is not provided.
    * `impl<T, E> Try for Poll<Result<T, E>>`
    * `impl<T, E> Try for Poll<Option<Result<T, E>>>`

[rust-lang/futures-rs#2207]: https://github.com/rust-lang/futures-rs/issues/2207
