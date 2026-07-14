# XRPL WASM E2E Tests

This folder contains end-to-end tests that are meant to be built and executed during the `xrpl-wasm-stdlib` build cycle.
These tests include both integration tests that execute on a standalone rippled node and tests that run on the host simulator.

Top-level crates test **Smart Escrows** (XLS-100) against `xrpl-wasm-stdlib`. While these aren't meant to be example
contracts, they do illustrate how to implement various use-cases defined in the
[Smart Escrows XLS proposal](https://github.com/XRPLF/XRPL-Standards/discussions/270).

Crates under [`smart-contracts/`](smart-contracts) test **Smart Contracts** (XLS-101) against `xrpl-contract-stdlib`
instead -- a different entry point, host functions, and context type from Smart Escrows.

## Future Enhancements

- [ ] Fail the build if any of these do not succeed with a positive result code.
