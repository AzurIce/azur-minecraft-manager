一个本地的 Mod 文件可以分为两种情况：

- **本地 Mod**：无法与任何 Mod 平台中的 Mod 对应
- **非本地 Mod**：可以与 Mod 平台中的 Mod 对应。

他们之间也存在着信息差异，往往获取前者的信息只能通过 Mod 文件本身来提取，较为受限，而后者则可以利用平台 API 获取更多的信息。

## ModSource

ModSource 为对应着远端 Mod 网站中的 Mod本身的信息。而非具体的某一个本地 Mod 文件。

## ModFile 解析

ModFile 为对应着本地具体 mod 文件的结构体，其结构如下：

```rust
#[derive(Serialize, Deserialize, Debug)]
pub struct ModFile {
    filename: String, // From file
    sha1: String,
    mod_loader: ModLoader, // From file's zipped content
    mod_id: String,
    mod_version: String,
    belong_mod: ModSource, // From modrinth API(get version from sha1)
}
```

- 由文件直接获取/计算
  - `filename` 文件名
  - `sha1` 文件的 sha1 值
- 通过以压缩文件方式读取其中可能存在的 Modloader 文件
  - `mod_loader` 此 ModFile 所支持的 Modloader
  - `mod_id` 此 ModFile 的标识符
  - `mod_version` 此 ModFile 的版本
- 通过 Modrinth API 获取的信息
  - `belong_mod` 所属的 Mod

### Modloader 文件

- `fabric.mod.json`

  - `schemaVersion`

    要求始终为 `1`

  - `id` 对应 Mod 标识符（即所谓的 mod_id，游戏中 xxx:... 的 xxx）

    字符串，由字母、数字、下划线组成，长为 1~63 

  - `version` 版本

    字符串