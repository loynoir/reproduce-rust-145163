# bug

same `X`, both `X == 0` and `X != 0` are triggered, when above `MaybeUninit` under release mode

## prepare

```sh
$ bash ./build.sh
...
```

## fixed

```console
=== Calling debug reproduce ===
[reproduce] status == napi_ok, should not error

=== Calling release reproduce ===
[reproduce] status == napi_ok, should not error

=== Calling release workaround ===
[workaround] status == napi_ok, should not error
```

## actual

When debug mode

- same `X`, only `X == 0` is triggered

When release mode

- same `X`, both `X == 0` and `X != 0` are triggered

```console
$ node ./reproduce.js 
=== Calling debug reproduce ===
[reproduce] status == napi_ok, should not error

=== Calling release reproduce ===
[reproduce] status == napi_ok, should not error
Error: [reproduce] Error: status != napi_ok
    at Object.<anonymous> (/workspaces/loynoir/repo/reproduce-rust-145163/reproduce.js:9:15)
    at Module._compile (node:internal/modules/cjs/loader:1738:14)
    at Object..js (node:internal/modules/cjs/loader:1871:10)
    at Module.load (node:internal/modules/cjs/loader:1470:32)
    at Module._load (node:internal/modules/cjs/loader:1290:12)
    at TracingChannel.traceSync (node:diagnostics_channel:322:14)
    at wrapModuleLoad (node:internal/modules/cjs/loader:238:24)
    at Module.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:154:5)
    at node:internal/main/run_main_module:33:47

=== Calling release workaround ===
[workaround] status == napi_ok, should not error
```

## workaround

```rs
let result = MaybeUninit::<napi_value>::uninit();
```

```rs
let result = MaybeUninit::<[napi_value; 1]>::uninit();
```

## expected

- release mode and debug mode have same behavior

- `X == 0` and `X != 0` cannot be both triggered

## version

nightly-2025-08-06
