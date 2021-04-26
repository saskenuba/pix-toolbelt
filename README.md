# Pix-toolbelt

Features a collection of crates to ease the creation and validation of PIX BrCode, a subset of the EMV-QrCode
specification.

It hopes to be the go-to library for br-code and pix related operations.

## Table of Contents

- [Pix-toolbelt](#pix-toolbelt)
    * [emv-qrcps](#emv-qrcps)
    * [pix-spec](#pix-spec)
    * [pix-api-client](#pix-api-client)
    * [LICENSE](#license)

## emv-qrcps

Status: Implemented.

Features a non-alloc deserializer, and a serializer for any specification based on EMV-QrCode.

## pix-brcode

Status: In progress...

Features the creation, deserialization of PIX Br Code strings, with proper CRC16 checks. Works by extending the
functionality provided by [emv-qrcps crate](#emv-qrcps).

Works for both PIX Dynamic and Static QR Codes.

Can also validate `Location` URL for `Merchant Account Information`, used on PIX Dynamic QR Codes.

## pix-api-client

Status: In progress...

Features a strongly typed API client in Rust, that should work for any PIX-API compliant PSPs.

## LICENSE

The whole project is licensed with the permissive MIT license. If you use it commercially, you are not obliged, but as a
token of gratitude, please give a heads up to the creators.

Every contribution is also licensed to the MIT license.
