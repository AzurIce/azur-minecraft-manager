



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
    game_version: String,
    belong_mod: Mod, // From modrinth API(get vsha1)
}
```

- 由文件直接获取/计算
  - `filename` 文件名
  - `sha1` 文件的 sha1 值
- 通过以压缩文件方式读取其中可能存在的 Modloader 文件
  - `mod_loader` 此 ModFile 所支持的 Modloader
  - `mod_id` 此 ModFile 的标识符
  - `mod_version` 此 ModFile 的版本
  - `game_version` 此 ModFile 对应的游戏版本
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