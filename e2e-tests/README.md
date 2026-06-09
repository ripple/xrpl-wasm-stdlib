# XRPL WASM E2E Tests

This folder contains end-to-end test Smart Escrows that are meant to be built and executed during the `xrpl-wasm-stdlib` build cycle.
These tests include both integration tests that execute on a standalone rippled node and tests that run on the host simulator.
While these smart escrows aren't meant to be example contracts, they do illustrate how to implement various use-cases
defined in the [Smart Escrows XLS proposal](https://github.com/XRPLF/XRPL-Standards/discussions/270).

## Running Locally

E2E tests connect to a local `xrpld` node at `ws://127.0.0.1:6006`. Start the Docker container before running any test, then wait ~10 seconds for xrpld to be ready.

```shell
docker run -d --rm -p 5005:5005 -p 6006:6006 --volume "$(pwd)/.ci-config/":"/etc/xrpld/" \
  --entrypoint bash rippleci/xrpld:ripple--se--supported -c "xrpld -a"
```

Alternatively, run against Devnet instead:

```shell
DEVNET=true ./scripts/run-tests.sh e2e-tests/<test-name>
```

## Future Enhancements

- [ ] Fail the build if any of these do not succeed with a positive result code.
