# `embedded-error`

> A universal set of specific error kinds for use in embedded systems

## What is `embedded-error`?

`embedded-error` has grown out of the desire to have universal but specific
"error" kinds for use in applications running on embedded systems.

Unlike regular applications running on a server or desktop, an embedded system
doesn't typically have a user or supervisor process monitoring it's output;
most systems do not even have a display which could be used to display an
error.  Instead the application must be able to identify any signalled error in
order to handle them appropriately.

It is also important to note, that many of error kinds are not fatal but a way
to communicate between a peripheral or a device connected to the peripheral
with the application. Thus it is often not useful to treat the `Err` case of a
`Result` as a wholesale fatal incident and halt or reboot the application which
would be the case if there was no way to discriminate between error kinds, e.g.
because the error kinds are implementation specific and not portable across
different hardware.

`embedded-error` sets out to collect all possible error kinds grouped by
peripheral and provide them as
[`non-exhaustive`](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute)
enumerations, very much like
[`std::io::ErrorKind`](https://doc.rust-lang.org/std/io/enum.ErrorKind.html)
does.

This allows drivers to use well-defined error types and applications as well
as universal drivers to generically handle typical "errors".

## How-to use `embedded-error`

Usage is straight forward:

1. Declare the use of `embedded-error` in your `Cargo.toml`:
```
embedded-error = "*"
```
2. If you're a implementor of [`embedded-hal`] traits, define the appropriate
   `error` as the associated `Error` type in your implementation, e.g.:
```
impl WriteRead for I2c
{
    type Error = embedded_error::I2cError;
    ...
```
3. In case you're implementing a standalone driver or function, simply use it
   in the `Result`, e.g.:
```
pub fn new(spi: &mut SPI) -> Result<Self, embedded_error::SpiError>
```
   and return or forward the appropriate error (via the `?` operator).
4. As an (embedded) application writer, to handle an error you simply match it.
   Please note that since errors are declared `non_exhaustive` you will always
   need to match with a wildcard for unhandled or error cases to be added in the
   future. The Rust compiler will tell you if you're doing it wrong. Here's
   what this could look like:

```
use embedded_error::I2cError;

match i2c.write(addr, &[]) {
        Err(I2cError::NACK) => { writeln!(out, "No device on address {}", addr).ok(); },
        Ok(_) => { writeln!(out, "Found device on address {}", addr).ok(); },
        _ => { writeln!(out, "Ohoh, the device is on fire, let's reboot".ok(); reboot(); },
}
```

Plese check the [embedded-error documentation] for details.

## Currently supported error kinds

At the moment we have support for the following peripherals:

* Serial
* SPI
* I2C
* MCI (MultiMedia Card Interface)
* Generic implementation errors

## How to contribute?

Thanks for your interest in contributing to this effort! Contributing is as
easy as creating a pull request (PR) against the [embedded-error] repository.
All additions are required to contain proper doc strings; due to that it is
typically not required to explain the motiviation for a PR. If we have concerns
or questions we will contact you, otherwise your PR is going to be approved.

The aim is to have frequent releases which are going to be forwards compatible
so only the patch number will change and everyone can profit immediately. If we
forget to make a release and you would like to have one, please contact us or
open an issue.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[embedded-error documentation]: https://docs.rs/embedded-error
[embedded-error]: https://github.com/therealprof/embedded-error
[embedded-hal]: https://github.com/rust-embedded/embedded-hal
