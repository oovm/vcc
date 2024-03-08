```
- source/
    - index.vk
- scripts/
    - main.vk
- examples/
    - main.vk
- benchmarks/
    - main.vk
- tests/
    - main.vk
main.vk
package.toml
package.lock
readme.md
```

- `main.vk`:  the default running script
- `readme.md`:
- `legion.json`:

--- 

### `scripts`

支持 `*.vk`, `*.wat`, `*.wasm` 三种格式, 以及目录形式:

- `scripts/`: **executable scripts**
    - `text.wat`:
    - `single.vk`:   a single file script
    - `nested/`:     a directory of scripts
        - `main.vk`:   the script entry point

比如这个结构体

- vp binary
- vp text
- vp single
- vp nested

### `examples`

- `examples/`: **executable scripts**
    - `nested/`:     a directory of scripts
        - `main.vk`:   the script entry point

通过 vp 直接运行, 只能用 `vp example` 命令运行.

- vp example nested

默认不打包发布.

## `tests`

和 `scripts` 目录结构完全相同, 但是无法通过 `vp` 直接运行, 只能用 `vp test` 命令运行.

默认不打包发布.

## `benchmarks`

和 `scripts` 目录结构完全相同, 但是无法通过 `vp` 直接运行, 只能用 `vp benchmark` 命令运行.

默认不打包发布.

## Example

```json5
{
    workspace: {
    },
    package: {
        name: "std",
        version: "0.0.0",
        publish: false,
        description: "The standard function library of the Valkyrie language",
        authors: [
            "Aster <192607617@qq.com>"
        ],
        repository: "https://github.com/oovm/XCell",
        documentation: "https://docs.rs/xcell-types",
        edition: "2000",
        license: "MPL-2.0",
    },
    features: {
        default: [],
    },
    scripts: {
        "command": "running"
    },
    binaries: {
    },
    "dependencies": {},
    "dependencies-build": {},
    "dependencies-debug": {},
    // pretty: "path"

    pretty: {
        // pretty format config, inherit parent
    }
}
```
