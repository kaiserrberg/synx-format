# SYNX for JS/TS — @aperturesyndicate/synx

The official JavaScript & TypeScript parser for the SYNX format.

## Install

```bash
npm install @aperturesyndicate/synx
```

## Usage

```javascript
const Synx = require('@aperturesyndicate/synx');

// Load from file
const data = Synx.loadSync('config.synx');
console.log(data.server.port); // 8080

// Parse from string
const data2 = Synx.parse('name Wario\nage 30');
console.log(data2.name); // "Wario"
```

### TypeScript

```typescript
import Synx from '@aperturesyndicate/synx';

interface Config {
  server: { port: number; host: string };
  app_name: string;
}

const data = Synx.loadSync<Config>('config.synx');
console.log(data.server.port); // typed as number
```

## API

| Method | Description |
|---|---|
| `Synx.parse<T>(text, options?)` | Parse a .synx string → object |
| `Synx.loadSync<T>(path, options?)` | Load & parse file (sync) |
| `Synx.load<T>(path, options?)` | Load & parse file (async, returns Promise) |
| `Synx.stringify(obj, active?)` | Serialize object → .synx string |

### Options

```typescript
{
  basePath?: string;                 // For :include resolution
  env?: Record<string, string>;      // Override env vars
  region?: string;                   // For :geo ("RU", "US", etc.)
}
```

## Other Languages

- **Python** — [synx-format](https://pypi.org/project/synx-format/) on PyPI
- **Rust** — [synx](https://crates.io/crates/synx) on crates.io
  ```bash
  cargo add synx
  ```
  ```rust
  use synx::Synx;
  
  let data = Synx::load("config.synx")?;
  println!("{}", &data["server"]["port"]);
  ```

## License

MIT — © APERTURESyndicate
