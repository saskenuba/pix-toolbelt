# Pix-toolbelt

Features a collection of crates to ease the creation and validation BrCode, and PIX
(a subset of BrCode).

It hopes to be the go-to library for br-code and pix related operations.

## Table of Contents

- [Libraries](#pix-toolbelt)
  * [br-code-spec](#br-code-spec)
  * [pix-spec](#pix-spec)
  * [pix-api-client](#pix-api-client) (Rust only)
- [License Information](#license)

## br-code-spec

In progress.

Features a non-alloc deserializer of BrCode strings, and allows the creation of
new structures based on lookup tables.

## pix-spec

Not implemented.

Features the creation, and no-alloc deserialization of valid PIX Br Code
strings, with proper CRC16 checks.

Works for both PIX Dynamic and Static QR Codes.

Can also validate `Location` URL for `Merchant Account Information`, used on PIX
Dynamic QR Codes.

## pix-api-client

Not implemented.

Features a strongly typed API client in Rust, that should work for any PIX-API
compliant PSPs.

## LICENSE

The whole project is licensed with the permissive MIT license. If you use it
commercially, you are not obliged, but as a token of gratitude, please give a
heads up to the creators.

Every contribution is also licensed to the MIT license.
