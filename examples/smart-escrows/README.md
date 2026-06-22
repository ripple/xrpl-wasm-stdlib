# XRPL WASM Examples

This folder contains example Smart Escrow projects that demonstrate how to implement various use-cases defined in the
[Smart Escrows XLS proposal](https://github.com/XRPLF/XRPL-Standards/discussions/270).

## Running Locally

To run examples against a local `xrpld` node, start the Docker container first (from the repo root):

```shell
docker run -d --rm -p 5005:5005 -p 6006:6006 --volume "$(pwd)/.ci-config/":"/etc/xrpld/" \
  --entrypoint bash rippleci/xrpld:ripple--se--supported -c "xrpld -a"
```

Wait ~3 seconds for xrpld to start, then run each example without `DEVNET=true`.
