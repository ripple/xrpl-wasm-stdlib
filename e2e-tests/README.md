# XRPL WASM E2E Tests

This folder contains end-to-end test Smart Escrows that are meant to be built and executed during the `xrpl-common-stdlib` build cycle.
These tests include both integration tests that execute on a standalone rippled node and tests that run on the host simulator.
While these smart escrows aren't meant to be example contracts, they do illustrate how to implement various use-cases
defined in the [Smart Escrows XLS proposal](https://github.com/XRPLF/XRPL-Standards/discussions/270).

## Future Enhancements

- [ ] Fail the build if any of these do not succeed with a positive result code.
