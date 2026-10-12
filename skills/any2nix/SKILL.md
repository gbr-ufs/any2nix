---
name: any2nix
description: >-
  Convert configuration files (JSON, TOML, YAML, INI) to Nix expressions using any2nix.
  Use when asked to convert configuration files or dotfiles to Nix, translate structured
  data to Nix syntax, or when the user invokes /any2nix.
---

# Convert Configurations to Nix with any2nix

Use `any2nix` to deterministically translate structured configuration files into formatted Nix expressions using Serde serializers.

## How to Run

### 1. Direct CLI (if installed in `$PATH`)

```sh
any2nix -f <format> <path/to/file>
```

### 2. Nix Run Fallback (if `any2nix` is not in `$PATH`)

```sh
nix run github:gbr-ufs/any2nix -- -f <format> <path/to/file>
```

### 3. Workspace Fallback (when working inside the any2nix repository)

```sh
cargo run -p any2nix-cli -- -f <format> <path/to/file>
```

### 4. Standard Input (stdin)

When converting snippets or piped data:

```sh
cat << 'EOF' | any2nix -f <format> -
<content>
EOF
```

## Supported Formats

| Format   | Option    | Common Extensions | Notes                                   |
|:---------|:----------|:------------------|:----------------------------------------|
| **TOML** | `-f toml` | `.toml`           | Tables, arrays of tables, inline tables |
| **YAML** | `-f yaml` | `.yaml`, `.yml`   | Mappings, sequences, scalars            |
| **JSON** | `-f json` | `.json`           | Objects, arrays, primitive types        |
| **INI**  | `-f ini`  | `.ini`, `.conf`   | Sections and key-value pairs            |

## Migration & Integration Workflows

### Converting a Standalone File

1. Determine the format from the file extension or content.
2. Run `any2nix` and redirect to a `.nix` file:
   ```sh
   any2nix -f <format> config.<ext> > config.nix
   ```
3. Validate the generated expression (if `nix` is available):
   ```sh
   nix-instantiate --parse config.nix > /dev/null
   ```

### Integrating with Home Manager or NixOS

Home Manager programs often accept configuration via `programs.<name>.settings`:

1. Convert the original configuration file using `any2nix`.
2. Assign the generated attribute set to the module option:
   ```nix
   { config, pkgs, ... }:

   {
     programs.foo.settings =
       # Paste or import the generated any2nix attribute set
       {
         ...
       };
   }
   ```

## Rules & Best Practices

- Always prefer `any2nix` over manual LLM conversion for structured formats to avoid syntax errors with attribute set formatting, string escaping, or reserved keywords.
- Note that format conversion is based on data serialization; source comments in TOML/YAML/INI files are not preserved during conversion.
- If the format cannot be deduced from the file extension, inspect the content to determine whether it is JSON, TOML, YAML, or INI before passing `-f`.
