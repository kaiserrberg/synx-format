# ðŸ“˜ SYNX â€” Full Format Specification

**Version:** 4.2  
**Name:** SYNX (Syndicate Exchange), pronounced "Sine-X"  
**File extension:** `.synx`

---

## 1. Philosophy

SYNX is a data format that strips away all syntactic noise â€” quotes, commas, brackets, colons around values. All that remains is **key**, **space**, **value**.

| Criteria | JSON | YAML | SYNX |
|---|---|---|---|
| Quotes | Required | Sometimes | No |
| Commas / brackets | Yes | No | No |
| Built-in logic | No | No | Yes (`!active`) |
| Tokens for AI | ~100% | ~75% | ~40% |

---

## 2. Basic Syntax (always works)

### 2.1 Keyâ€“Value

Everything after the **first space** is the value. No quotes needed.

```synx
name John
age 25
phrase I love programming and drinking coffee!
```

JSON equivalent:
```json
{
  "name": "John",
  "age": 25,
  "phrase": "I love programming and drinking coffee!"
}
```

**Key rules:**
- No spaces (use `_` or `camelCase`)
- Cannot start with `-`, `#`, `//`, `!`
- May contain letters of any alphabet, digits, `_`, `-` (inside)

---

### 2.2 Automatic Type Detection

The parser detects types automatically:

```synx
greeting Hello World
count 42
rate 3.14
enabled true
disabled false
empty null
```

Result:
```json
{
  "greeting": "Hello World",
  "count": 42,
  "rate": 3.14,
  "enabled": true,
  "disabled": false,
  "empty": null
}
```

**Explicit type casting** (if you need a number as a string):
```synx
zip_code(string) 90210
id(int) 007
```

Supported casts: `(int)`, `(float)`, `(bool)`, `(string)`

---

### 2.3 Nesting (objects)

An indent of **2 spaces** creates a nested object. A key without a value is a group (folder).

```synx
server
  host 0.0.0.0
  port 8080
  ssl
    enabled true
    cert /etc/ssl/cert.pem
```

Result:
```json
{
  "server": {
    "host": "0.0.0.0",
    "port": 8080,
    "ssl": {
      "enabled": true,
      "cert": "/etc/ssl/cert.pem"
    }
  }
}
```

> âš ï¸ Use **spaces**, not TABs. The standard is **2 spaces** per level.

---

### 2.4 Lists

The `- ` (dash + space) symbol creates a list item. Items without names form an ordered collection.

```synx
inventory
  - Sword
  - Shield
  - Health Potion
```

Result:
```json
{
  "inventory": ["Sword", "Shield", "Health Potion"]
}
```

**List of objects** (each item is a mini-record):

```synx
garage
  - make BMW
    color black
    year 2023
  - make Audi
    color white
    year 2021
```

Result:
```json
{
  "garage": [
    { "make": "BMW", "color": "black", "year": 2023 },
    { "make": "Audi", "color": "white", "year": 2021 }
  ]
}
```

---

### 2.5 Multiline Text (blocks)

The `|` character after a key starts a text block. Everything indented below is part of that text.

```synx
description |
  This is a long description
  that spans multiple lines.
  Each line is joined with a newline character.
```

Result:
```json
{
  "description": "This is a long description\nthat spans multiple lines.\nEach line is joined with a newline character."
}
```

Forced line break inside a single line: `/n`

```synx
banner Welcome!/nEnjoy the game!
```

Result:
```
Welcome!
Enjoy the game!
```

---

### 2.6 Comments

**Two styles** are supported:

```synx
# This is a comment (Python/YAML style)
// This is also a comment (JS/C++ style)

name John  # Inline comment after value
port 8080 // Also inline
```

Comments are completely ignored by the parser.

---

## 3. `!active` Mode â€” Live Config

### 3.1 What Is It

By default a `.synx` file contains **static data**. Just keys and values, like JSON.

But if the **first line** of the file says `!active`, the file becomes a **live config** â€” enabling:

- **Functions** (`:random`, `:calc`, `:env`, etc.)
- **Constraints** (`[min:3]`, `[type:int]`, etc.)

```synx
!active

port:env PORT
greeting:random
  - Hello!
  - Welcome!
```

Without `!active` â€” markers `:random`, `:calc` and square brackets `[]` are **completely ignored**. The parser reads the file as plain text:

```synx
// No !active â€” functions DO NOT work
port:env PORT           // key = "port:env", value = "PORT" (just a string)
greeting:random          // key = "greeting:random", value = {} (object)
```

> ðŸ’¡ This is a security feature: a static file will **never** execute code, access environment variables, or run commands.

---

### 3.2 Full List of Functions (`:`)

Functions are written with a colon immediately after the key name: `key:function value`.

---

#### `:random` â€” Random Selection

Selects **one** element from a list on each parse.

**Equal chances** (no percentages):
```synx
!active

battle_cry:random
  - Time to win!
  - Ha-ha-ha!
  - For the Syndicate!
```
Each item has a 33.3% chance.

**Weighted random** (with percentages):
```synx
!active

// Percentages go after :random separated by spaces
// Order of percentages matches order of items
loot:random 70 20 10
  - Common Chest
  - Rare Chest
  - Legendary Chest
```

Here:
- "Common Chest" drops with **70%** chance
- "Rare Chest" â€” **20%**
- "Legendary Chest" â€” **10%**

**Percentage rules:**
| Situation | Behavior |
|---|---|
| No percentages | All items are equally likely |
| Sum = 100 | Used as-is |
| Sum â‰  100 | Automatically normalized (proportions preserved) |
| Fewer percentages than items | Remainder is split evenly among items without a percentage |
| More percentages than items | Extra percentages are ignored |

Normalization example:
```synx
!active

// 2 + 1 = 3, normalized: ~66.7% and ~33.3%
class:random 2 1
  - Warrior
  - Mage
```

Partial percentages example:
```synx
!active

// First gets 80%, remaining 20% split between two: 10% each
drop:random 80
  - Nothing
  - Sword
  - Shield
```

---

#### `:calc` â€” Expression Evaluation

Evaluates an arithmetic expression. Can reference other keys by name.

```synx
!active

price 100
tax:calc price * 0.2
total:calc price + tax
```

Result:
```
price 100
tax 20    // â† your program receives this value
total 120 // â† your program receives this value
```

As a JSON file:
```json
{
  "price": 100,
  "tax": 20,
  "total": 120
}
```

**Supported operations:** `+`, `-`, `*`, `/`, `%` (remainder), `(`, `)`.

> âš ï¸ Arithmetic only. No arbitrary code. The parser uses a safe evaluator, not `eval()`.

---

#### `:env` â€” Environment Variable

Substitutes the value of a system environment variable.

```synx
!active

port:env PORT
home_dir:env HOME
```

If the variable is not found â€” the value will be `null`.

**With a default** (via `:default`):
```synx
!active

port:env:default:8080 PORT
```
If `PORT` is not set â†’ returns `8080`.

---

#### `:alias` â€” Reference to Another Key

Copies the value of another key. Does not duplicate data â€” references it.

```synx
!active

admin_email alex@example.com
complaints_email:alias admin_email
```

Result:
```json
{
  "admin_email": "alex@example.com",
  "complaints_email": "alex@example.com"
}
```

---

#### `:secret` â€” Hidden Value

The value is readable by your program but **not shown** in logs, during `freeze`, or when serialized to JSON. Protects against accidental leaks.

```synx
!active

api_key:secret sk-1234567890abcdef
db_password:secret P@ssw0rd!
```

With `console.log(data)`, `print(data)`, `JSON.stringify(data)` â€” shows `"[SECRET]"`.  
To **get the real value**, use the `.reveal()` method:

```javascript
// JavaScript / TypeScript
const key = data.api_key;          // SynxSecret object
console.log(String(key));           // "[SECRET]"
console.log(JSON.stringify(key));   // '"[SECRET]"'
console.log(key.reveal());          // "sk-1234567890abcdef"  â† real value
```

```python
# Python
key = data['api_key']               # SynxSecret object
print(key)                          # [SECRET]
print(key.reveal())                 # sk-1234567890abcdef  â† real value
```

> âš ï¸ **Important:** Never log the result of `.reveal()`. This method is intended only for passing the value to APIs, database connections, etc.

---

#### `:default` â€” Default Value

Sets a fallback if the main value is empty or not found. Most often combined with `:env`.

```synx
!active

// If PORT env var is not set â€” will be 3000
port:env:default:3000 PORT

// Just a default value (if value = null or empty)
theme:default dark
```

---

#### `:unique` â€” Remove Duplicates from List

Keeps only unique elements.

```synx
!active

tags:unique
  - action
  - rpg
  - action
  - simulation
  - rpg
```

Result: `["action", "rpg", "simulation"]`

---

#### `:include` â€” Include Another File

Inserts the contents of another `.synx` file into the current one. Path is relative to the current file.

```synx
!active

// Pull database settings from a separate file
database:include ./db.synx
```

Where `db.synx`:
```synx
host localhost
port 5432
name mydb
```

Result:
```json
{
  "database": {
    "host": "localhost",
    "port": 5432,
    "name": "mydb"
  }
}
```

---

#### `:geo` â€” Value by Geolocation / Region

Selects a value based on user region (determined by IP or system locale).

```synx
!active

currency:geo
  - US USD
  - EU EUR
  - GB GBP
```

> This function requires runtime support. The parser passes the current region to the engine.

---

#### `:template` â€” String Interpolation

Substitutes `{placeholders}` with values from other keys. Supports dot-path for nested access.

```synx
!active

first_name John
last_name Doe
greeting:template Hello, {first_name} {last_name}!
```

Result: `"Hello, John Doe!"`

**With nested references** (dot-path):
```synx
!active

server
  host localhost
  port 8080

db_url:template http://{server.host}:{server.port}/db
```

Result: `"http://localhost:8080/db"`

---

#### `:split` â€” Split String into Array

Splits a string by a delimiter into an array of items. Auto-casts values if they look like numbers.

**Default delimiter (comma):**
```synx
!active

colors:split red, green, blue
```

Result: `["red", "green", "blue"]`

**With delimiter keyword:**
```synx
!active

// Space delimiter
words:split:space hello world foo

// Pipe delimiter
flags:split:pipe read write execute
```

Supported delimiter keywords: `space`, `pipe`, `dash`, `dot`, `semi`, `tab`

**Auto-casting:**
```synx
!active

numbers:split 1, 2, 3
```

Result: `[1, 2, 3]` (auto-detected as integers)

---

#### `:join` â€” Join Array into String

Joins array elements into a string with a delimiter.

**Default delimiter (comma):**
```synx
!active

tags:join
  - action
  - rpg
  - adventure
```

Result: `"action, rpg, adventure"`

**With delimiter keyword:**
```synx
!active

path:join:slash
  - home
  - user
  - documents
```

Result: `"home/user/documents"`

---

### 3.3 Functions Summary Table

| Function | Description | Example |
|---|---|---|
| `:random` | Random item from list | `phrase:random` |
| `:random N N N` | Weighted random (percentages) | `loot:random 70 20 10` |
| `:calc` | Arithmetic evaluation | `total:calc price * 1.2` |
| `:env` | Environment variable | `port:env PORT` |
| `:alias` | Reference to another key | `copy:alias original` |
| `:secret` | Value hidden from logs | `password:secret abc123` |
| `:default` | Default value | `theme:default dark` |
| `:default:X` | Fallback (in combination) | `port:env:default:8080 PORT` |
| `:unique` | List deduplication | `tags:unique` |
| `:include` | Include external file | `db:include ./db.synx` |
| `:geo` | Value by region | `currency:geo` |
| `:template` | String interpolation | `greeting:template Hello, {name}!` |
| `:split` | String â†’ array | `colors:split red, green, blue` |
| `:join` | Array â†’ string | `tags:join` |

**Chaining functions** â€” via `:` chain:
```synx
!active
port:env:default:8080 PORT
```

---

### 3.4 Constraints (`[]`) â€” Data Validation

Constraints are written in square brackets between the key and the function (or value). They only work in `!active` mode.

General syntax:
```
key[constraint1:value, constraint2:value]:function value
```

---

#### `min` / `max` â€” Minimum and Maximum

For **numbers** â€” restricts the value range.  
For **strings** â€” restricts the length (character count).

```synx
!active

// String from 3 to 30 characters
app_name[min:3, max:30] TotalWario

// Number from 1 to 100
volume[min:1, max:100] 75

// Minimum only
password[min:8] mypassword123
```

---

#### `required` â€” Required Field

The parser will throw an error if the value is empty or missing.

```synx
!active

api_key[required]:env API_KEY
name[required, min:1] Wario
```

---

#### `pattern` â€” Regular Expression

The value must match a regex pattern.

```synx
!active

country_code[pattern:^[A-Z]{2}$] US
phone[pattern:^\+\d{10,15}$] +12025551234
```

---

#### `enum` â€” Allowed Values

The value must be one of the listed options.

```synx
!active

theme[enum:light|dark|auto] dark
region[enum:EU|US|GB|AS] US
```

---

#### `readonly` â€” Read Only

The value cannot be changed via API / hot-reload of the config. Only manual file editing.

```synx
!active

version[readonly] 2.0.0
```

---

### 3.5 Constraints Summary Table

| Constraint | Description | Syntax |
|---|---|---|
| `min` | Minimum (length or value) | `[min:3]` |
| `max` | Maximum (length or value) | `[max:100]` |
| `type` | Data type | `[type:int]` |
| `required` | Required field | `[required]` |
| `pattern` | Regex validation | `[pattern:^\d+$]` |
| `enum` | List of allowed values | `[enum:a\|b\|c]` |
| `readonly` | Modification forbidden | `[readonly]` |

Constraints can be combined with commas:
```synx
!active
password[required, min:8, max:64, type:string] MyP@ssw0rd
```

---

## 4. Full Examples

### 4.1 Plain File (no `!active`)

A simple static config. No magic â€” just data.

```synx
# TotalWario game config (static)

app_name TotalWario
version 2.0.0

server
  host 0.0.0.0
  port 8080
  ssl_enabled false

gameplay
  base_hp 100
  boss_hp 500
  max_players 16
  greeting Prepare to fight.

map_rotation
  - Arena of Doom
  - Crystal Caverns
  - Wario Stadium

rules |
  1. No cheating.
  2. Respect the Syndicate.
  3. Have fun!

credits
  lead_dev KaiserBerg
  studio APERTURESyndicate
  year 2026
```

Result (`Synx.parse()`):
```json
{
  "app_name": "TotalWario",
  "version": "2.0.0",
  "server": {
    "host": "0.0.0.0",
    "port": 8080,
    "ssl_enabled": false
  },
  "gameplay": {
    "base_hp": 100,
    "boss_hp": 500,
    "max_players": 16,
    "greeting": "Prepare to fight."
  },
  "map_rotation": [
    "Arena of Doom",
    "Crystal Caverns",
    "Wario Stadium"
  ],
  "rules": "1. No cheating.\n2. Respect the Syndicate.\n3. Have fun!",
  "credits": {
    "lead_dev": "KaiserBerg",
    "studio": "APERTURESyndicate",
    "year": 2026
  }
}
```

---

### 4.2 File with `!active` â€” Live Config

The same config, but with dynamic functions and validation.

```synx
!active
# Live config for TotalWario

app_name[required, min:3, max:30] TotalWario
version[readonly] 2.0.0

server
  // Port from environment variable, fallback to 8080
  port:env:default:8080 PORT
  host 0.0.0.0
  ssl_enabled false

gameplay
  base_hp 100
  // Engine calculates: 100 * 5 = 500
  boss_hp:calc base_hp * 5
  max_players[type:int, min:2, max:64] 16
  difficulty[enum:easy|normal|hard|nightmare] normal

  // A random phrase on every parse
  greeting:random
    - Welcome to the arena!
    - Prepare to fight.
    - Wario time!

  // Weighted random: common 70%, rare 20%, legendary 10%
  loot_tier:random 70 20 10
    - common
    - rare
    - legendary

map_rotation
  - Arena of Doom
  - Crystal Caverns
  - Wario Stadium

rules |
  1. No cheating.
  2. Respect the Syndicate.
  3. Have fun!

// Include database settings from a separate file
database:include ./db.synx

// Secrets â€” won't appear in logs
api_key[required]:secret sk-live-abc123def456

credits
  lead_dev KaiserBerg
  studio APERTURESyndicate
  year 2026
  contact:alias lead_dev
```

Result of one `Synx.parse()` call:
```json
{
  "app_name": "TotalWario",
  "version": "2.0.0",
  "server": {
    "port": 8080,
    "host": "0.0.0.0",
    "ssl_enabled": false
  },
  "gameplay": {
    "base_hp": 100,
    "boss_hp": 500,
    "max_players": 16,
    "difficulty": "normal",
    "greeting": "Wario time!",
    "loot_tier": "common"
  },
  "map_rotation": [
    "Arena of Doom",
    "Crystal Caverns",
    "Wario Stadium"
  ],
  "rules": "1. No cheating.\n2. Respect the Syndicate.\n3. Have fun!",
  "database": {
    "host": "localhost",
    "port": 5432,
    "name": "mydb"
  },
  "api_key": "[SECRET]",
  "credits": {
    "lead_dev": "KaiserBerg",
    "studio": "APERTURESyndicate",
    "year": 2026,
    "contact": "KaiserBerg"
  }
}
```

> On the next `parse()` call, `greeting` and `loot_tier` will be different â€” that's the point of `:random`.

---

## 5. Using in Code â€” Directly, Without Conversion

> ðŸš€ **Core Principle:** SYNX is read by your code **directly**. No conversion to JSON is needed. The library parses the `.synx` file itself and returns a native object for your language â€” `dict` in Python, `Object` in JavaScript. You work with data immediately.

### 5.1 Installation

```bash
# JavaScript / TypeScript
npm install @aperturesyndicate/synx

# Python
pip install synx-format
```

---

### 5.2 Python â€” Reading `.synx` Directly

```python
from synx import Synx

# Read a .synx file â€” and get a dict immediately
data = Synx.load('config.synx')

# Done. Data is ready. Work with it like a regular dictionary:
print(data['app_name'])                # "TotalWario"
print(data['server']['port'])          # 8080
print(data['gameplay']['base_hp'])     # 100
print(data['gameplay']['boss_hp'])     # 500 (computed by :calc)
print(data['gameplay']['greeting'])    # "Wario time!" (chosen by :random)

# Lists are regular lists:
for map_name in data['map_rotation']:
    print(f'Loading map: {map_name}')

# Nesting is regular dicts:
if data['server']['ssl_enabled']:
    print('SSL is enabled')
```

**`Synx.load(path)`** â€” reads the file and parses in one call.  
**`Synx.parse(text)`** â€” parses a string (if you already have the content).

```python
# Alternatively â€” from a string:
with open('config.synx', 'r', encoding='utf-8') as f:
    data = Synx.parse(f.read())
```

---

### 5.3 JavaScript â€” Reading `.synx` Directly

```javascript
const Synx = require('@aperturesyndicate/synx');

// Read a .synx file â€” get a regular JS object
const data = Synx.loadSync('config.synx');

// Work with it like a normal object using dot notation:
console.log(data.app_name);               // "TotalWario"
console.log(data.server.port);             // 8080
console.log(data.gameplay.base_hp);        // 100
console.log(data.gameplay.boss_hp);        // 500
console.log(data.gameplay.greeting);       // random phrase

// Lists are regular Arrays:
data.map_rotation.forEach(map => {
    console.log(`Loading map: ${map}`);
});

// Destructuring:
const { base_hp, boss_hp, greeting } = data.gameplay;
console.log(`Boss HP: ${boss_hp}, greeting: ${greeting}`);
```

**`Synx.loadSync(path)`** â€” reads the file and parses synchronously.  
**`Synx.load(path)`** â€” async version (returns `Promise`).  
**`Synx.parse(text)`** â€” parses a string.

```javascript
// Async variant:
const data = await Synx.load('config.synx');

// From a string:
const fs = require('fs');
const text = fs.readFileSync('config.synx', 'utf-8');
const data = Synx.parse(text);
```

---

### 5.4 TypeScript â€” With Type Safety

```typescript
import Synx from '@aperturesyndicate/synx';

// Define the structure of your config:
interface GameConfig {
  app_name: string;
  version: string;
  server: {
    port: number;
    host: string;
    ssl_enabled: boolean;
  };
  gameplay: {
    base_hp: number;
    boss_hp: number;
    greeting: string;
    loot_tier: string;
  };
  map_rotation: string[];
}

// The parser returns a typed object:
const data = Synx.loadSync<GameConfig>('config.synx');

console.log(data.gameplay.boss_hp);  // number â€” autocomplete works
console.log(data.server.port);       // number
```

---

### 5.5 Using SYNX Data for CSS

SYNX returns a plain object â€” `Object` in JS, `dict` in Python. You can use this data to generate CSS, CSS custom properties, or pass it into any styling system.

**Example: CSS custom properties from SYNX in Node.js**

```synx
# theme.synx
primary #5a6eff
secondary #ff5a8a
font_size 16
border_radius 8
spacing 12
```

```javascript
const Synx = require('@aperturesyndicate/synx');
const fs = require('fs');

const theme = Synx.loadSync('theme.synx');

// Generate CSS custom properties
const css = `:root {
  --color-primary: ${theme.primary};
  --color-secondary: ${theme.secondary};
  --font-size: ${theme.font_size}px;
  --border-radius: ${theme.border_radius}px;
  --spacing: ${theme.spacing}px;
}`;

fs.writeFileSync('theme.css', css);
```

**Example: Inline styles in React**

```tsx
import Synx from '@aperturesyndicate/synx';

const theme = Synx.loadSync('theme.synx');

function Button({ children }) {
  return (
    <button style={{
      backgroundColor: theme.primary,
      borderRadius: `${theme.border_radius}px`,
      padding: `${theme.spacing}px`,
      fontSize: `${theme.font_size}px`,
    }}>
      {children}
    </button>
  );
}
```

**Example: CSS-in-JS (styled-components, Tailwind config)**

```javascript
// tailwind.config.js
const Synx = require('@aperturesyndicate/synx');
const theme = Synx.loadSync('theme.synx');

module.exports = {
  theme: {
    extend: {
      colors: {
        primary: theme.primary,
        secondary: theme.secondary,
      },
      spacing: {
        base: `${theme.spacing}px`,
      },
    },
  },
};
```

> SYNX doesn't replace CSS â€” it provides **data** (colors, sizes, tokens) that your code uses
to generate styles. This is especially useful for design systems and theming.

---

### 5.6 Rust â€” Type-Safe Parsing

The SYNX Rust crate (`synx`) provides zero-dependency parsing with a rich type system.

**Installation (Cargo.toml):**
```toml
[dependencies]
synx = "4.1"
```

**Basic Usage:**
```rust
use synx::Synx;

fn main() {
    // Parse a .synx file
    let data = Synx::parse_file("config.synx").unwrap();
    
    // Access values with type safety
    if let Some(port) = data.get("server").and_then(|s| s.get("port")) {
        let port_num: i64 = port.as_int().unwrap_or(8080);
        println!("Server running on port {}", port_num);
    }
    
    // Work with nested objects
    let app_name = data
        .as_object()
        .and_then(|obj| obj.get("app_name"))
        .and_then(|v| v.as_str());
    
    // Convert to JSON if needed
    let json = serde_json::to_string(&data).unwrap();
}
```

**Value type methods:**
- `.as_str()` â†’ `Option<&str>`
- `.as_int()` â†’ `Option<i64>`
- `.as_float()` â†’ `Option<f64>`
- `.as_bool()` â†’ `Option<bool>`
- `.as_object()` â†’ `Option<&HashMap<String, Value>>`
- `.as_array()` â†’ `Option<&Vec<Value>>`
- `.is_null()` â†’ `bool`

**Indexing:**
```rust
let value = data["server"]["port"];
```

**Note:** The Rust parser currently supports **static mode** (no `:random`, `:calc`, `:env` functions). Use it for parsing SYNX data files. For active mode, use the JavaScript or Python libraries.

---

### 5.7 C â€” Zero-Dependency Static Parser

The SYNX C parser is a single `.h` + `.c` file pair. Copy them into your project â€” no build system required.

**Include in your project:**
```c
#include "synx.h"
```

**Basic Usage:**
```c
#include "synx.h"
#include <stdio.h>

int main(void) {
    SynxValue *data = synx_parse("server\n  host 0.0.0.0\n  port 8080");

    SynxValue *server = synx_get(data, "server");
    SynxValue *port = synx_get(server, "port");
    printf("Port: %lld\n", port->int_val);  // 8080

    SynxValue *host = synx_get(server, "host");
    printf("Host: %s\n", host->string_val);  // "0.0.0.0"

    // Arrays
    SynxValue *items = synx_parse("inv\n  - Sword\n  - Shield");
    SynxValue *inv = synx_get(items, "inv");
    for (size_t i = 0; i < synx_array_count(inv); i++) {
        printf("Item: %s\n", synx_array_get(inv, i)->string_val);
    }

    synx_free(data);
    synx_free(items);
    return 0;
}
```

**Build:**
```bash
gcc -o myapp myapp.c synx.c -I.
```

**API:**
- `synx_parse(text)` â†’ `SynxValue*` â€” parse and return root object
- `synx_parse_full(text)` â†’ `SynxParseResult` â€” parse with mode detection
- `synx_get(obj, key)` â†’ `SynxValue*` â€” get child by key
- `synx_array_get(arr, index)` â†’ `SynxValue*` â€” get array element
- `synx_array_count(arr)` â†’ `size_t` â€” array length
- `synx_free(value)` â€” free all memory

**Type checking:** `synx_is_null(v)`, `synx_is_bool(v)`, `synx_is_int(v)`, `synx_is_float(v)`, `synx_is_string(v)`, `synx_is_array(v)`, `synx_is_object(v)`

> ðŸ“Œ **Note:** The C parser is static-only, like Rust. No active function evaluation at parse time.

---

### 5.8 C++ â€” Header-Only Parser (C++17)

The SYNX C++ parser is a single header file `synx.hpp`. Include it and you're ready.

**Include in your project:**
```cpp
#include "synx.hpp"
```

**Basic Usage:**
```cpp
#include "synx.hpp"
#include <iostream>

int main() {
    // Load from file
    auto data = synx::Synx::load("config.synx");

    // Access with [] operator
    std::cout << *data["server"]["port"].as_int() << std::endl;    // 8080
    std::cout << *data["app_name"].as_str() << std::endl;          // "TotalWario"

    // Arrays
    if (auto* arr = data["inventory"].as_array()) {
        for (const auto& item : *arr) {
            std::cout << *item.as_str() << std::endl;
        }
    }

    // Parse from string
    auto cfg = synx::Synx::parse("name Wario\nage 30");
    std::cout << *cfg["name"].as_str() << std::endl;  // "Wario"

    return 0;
}
```

**Build:**
```bash
g++ -std=c++17 -o myapp myapp.cpp -I path/to/synx
```

**Value methods:**
- `.as_str()` â†’ `std::optional<std::string>`
- `.as_int()` â†’ `std::optional<int64_t>`
- `.as_float()` â†’ `std::optional<double>`
- `.as_bool()` â†’ `std::optional<bool>`
- `.as_array()` â†’ `const Array*` (nullptr if wrong type)
- `.as_object()` â†’ `const Object*` (nullptr if wrong type)
- `.is_null()` â†’ `bool`
- `operator[](key)` â€” index by string key
- `operator[](index)` â€” index by array position

> ðŸ“Œ **Note:** The C++ parser is static-only, like Rust and C. Header-only â€” no build system required.

---

### 5.9 C# â€” .NET Standard Parser

The SYNX C# parser targets .NET Standard 2.0+ and works with .NET Core, .NET 5/6/7/8, and .NET Framework.

**Install via NuGet:**
```bash
dotnet add package Synx
```

**Basic Usage:**
```csharp
using Synx;

// Parse from string
var data = SynxParser.Parse("server\n  host 0.0.0.0\n  port 8080");
Console.WriteLine(data["server"]["port"].AsInt());   // 8080
Console.WriteLine(data["server"]["host"].AsStr());   // "0.0.0.0"

// Load from file
var config = SynxParser.Load("config.synx");
Console.WriteLine(config["app_name"].AsStr());       // "TotalWario"

// Arrays
var items = config["inventory"].AsArray();
foreach (var item in items)
    Console.WriteLine(item.AsStr());

// Mode detection
var result = SynxParser.ParseFull(text);
if (result.Mode == SynxMode.Active)
    Console.WriteLine("Active config!");
```

**API:**
- `SynxParser.Parse(text)` â†’ `SynxValue` â€” parse string
- `SynxParser.ParseFull(text)` â†’ `SynxParseResult` â€” parse with mode
- `SynxParser.Load(path)` â†’ `SynxValue` â€” load and parse file

**SynxValue methods:**
- `.AsStr()` â†’ `string`
- `.AsInt()` â†’ `long?`
- `.AsFloat()` â†’ `double?`
- `.AsBool()` â†’ `bool?`
- `.AsArray()` â†’ `List<SynxValue>`
- `.AsObject()` â†’ `Dictionary<string, SynxValue>`
- `.IsNull` / `.IsBool` / `.IsInt` / `.IsFloat` / `.IsString` / `.IsArray` / `.IsObject` â€” type checks
- `this[string key]` â€” object indexer
- `this[int index]` â€” array indexer

> ðŸ“Œ **Note:** The C# parser is static-only, like Rust, C, and C++. No active function evaluation.

---

### 5.10 Go â€” Zero-Dependency Parser

**Installation:**

```bash
go get github.com/aperturesyndicate/synx
```

**Usage:**

```go
package main

import (
    "fmt"
    "github.com/aperturesyndicate/synx"
)

func main() {
    data, _ := synx.ParseFile("config.synx")

    host := data.Get("server").Get("host").AsStr()
    port := data.Get("server").Get("port").AsInt()
    debug := data.Get("server").Get("debug").AsBool()

    fmt.Println("host:", host)   // localhost
    fmt.Println("port:", port)   // 8080
    fmt.Println("debug:", debug) // true

    // Lists
    items := data.Get("items")
    for i := 0; i < items.Len(); i++ {
        fmt.Println(" -", items.Index(i).AsStr())
    }
}
```

**API:**
- `Parse(text string) *Value` â€” parse SYNX text (static mode)
- `ParseFull(text string) ParseResult` â€” parse with mode detection
- `ParseFile(path string) (*Value, error)` â€” load and parse a file

**Value Methods:**
- `Get(key string) *Value` â€” access by key
- `Index(i int) *Value` â€” access by index
- `AsStr() string`, `AsInt() int64`, `AsFloat() float64`, `AsBool() bool` â€” type accessors
- `IsNull() bool` â€” null check
- `Len() int` â€” child count
- `Keys() []string` â€” ordered keys

> ðŸ“Œ **Note:** The Go parser is static-only. No active function evaluation.

---

### 5.11 Java / Kotlin â€” JVM Parser

**Installation (Maven):**

```xml
<dependency>
    <groupId>com.aperturesyndicate</groupId>
    <artifactId>synx</artifactId>
    <version>3.0.0</version>
</dependency>
```

**Installation (Gradle / Kotlin):**

```kotlin
implementation("com.aperturesyndicate:synx:3.0.0")
```

**Usage (Java):**

```java
import com.aperturesyndicate.synx.*;

SynxValue data = Synx.load("config.synx");

String host = data.get("server").get("host").asStr();
long port = data.get("server").get("port").asInt();
boolean debug = data.get("server").get("debug").asBool();

System.out.println("host: " + host);   // localhost
System.out.println("port: " + port);   // 8080
System.out.println("debug: " + debug); // true
```

**Usage (Kotlin):**

```kotlin
import com.aperturesyndicate.synx.*

val data = Synx.load("config.synx")

val host = data.get("server").get("host").asStr()
val port = data.get("server").get("port").asInt()

println("host: $host")   // localhost
println("port: $port")   // 8080
```

**API:**
- `Synx.parse(String text)` â€” parse SYNX text
- `Synx.parseFull(String text)` â€” parse with mode detection
- `Synx.load(String path)` â€” load and parse a file

**SynxValue Methods:**
- `get(String key)` / `get(int index)` â€” access by key or index
- `asStr()`, `asInt()`, `asFloat()`, `asBool()` â€” type accessors
- `isNull()`, `isString()`, `isInt()`, `isBool()`, `isArray()`, `isObject()` â€” type checks
- `size()` â€” child count
- `keys()` â€” ordered keys

> ðŸ“Œ **Note:** The Java/Kotlin parser is static-only. No active function evaluation. Compatible with Java 11+ and all Kotlin versions.

---

### 5.12 Swift â€” Native Parser

**Installation (Swift Package Manager):**

```swift
// Package.swift
dependencies: [
    .package(url: "https://github.com/kaiserrberg/synx-format.git", from: "3.0.0")
]
```

**Usage:**

```swift
import Synx

let data = try Synx.load("config.synx")

let host = data["server"]["host"].asStr ?? ""
let port = data["server"]["port"].asInt ?? 0
let debug = data["server"]["debug"].asBool ?? false

print("host: \(host)")   // localhost
print("port: \(port)")   // 8080
print("debug: \(debug)") // true

// Lists
let items = data["items"]
for i in 0..<items.count {
    print("  - \(items[i].asStr ?? "")")
}
```

**API:**
- `Synx.parse(_ text: String) -> SynxValue` â€” parse SYNX text
- `Synx.parseFull(_ text: String) -> SynxParseResult` â€” parse with mode detection
- `Synx.load(_ path: String) throws -> SynxValue` â€” load and parse a file

**SynxValue Properties:**
- `subscript(key: String)` / `subscript(index: Int)` â€” access by key or index
- `asStr: String?`, `asInt: Int64?`, `asFloat: Double?`, `asBool: Bool?` â€” type accessors
- `isNull`, `isString`, `isInt`, `isBool`, `isArray`, `isObject` â€” type checks
- `count: Int` â€” child count
- `keys: [String]` â€” ordered keys

> ðŸ“Œ **Note:** The Swift parser is static-only. No active function evaluation. Requires Swift 5.9+.

---

### 5.13 Lua â€” Zero-Dependency Parser

**Installation:**

Copy `synx.lua` into your project:

```lua
local synx = require("synx")
```

**Usage:**

```lua
local synx = require("synx")

-- Parse from string
local data = synx.parse(text)
print(data:get("server"):get("port"):as_int())  -- 8080

-- Parse from file
local data = synx.parse_file("config.synx")
print(data:get("server"):get("host"):as_str())  -- localhost

-- Full parse (mode detection)
local result = synx.parse_full(text)
print(result.mode)  -- "static" or "active"
```

**API:**

- `synx.parse(text)` â†’ `SynxValue`
- `synx.parse_full(text)` â†’ `{ root, mode }`
- `synx.parse_file(path)` â†’ `SynxValue, err`
- `SynxValue:get(key)` â€” child by key or 1-based index
- `SynxValue:as_bool()`, `:as_int()`, `:as_float()`, `:as_str()`
- `SynxValue:len()`, `:keys()`, `:type()`, `:is_null()`

> ðŸ“Œ **Note:** The Lua parser is static-only. No active function evaluation. Works with Lua 5.1+ and LuaJIT.

---

### 5.14 Dart / Flutter â€” Native Parser

**Installation (pubspec.yaml):**

```yaml
dependencies:
  synx:
    git:
      url: https://github.com/kaiserrberg/synx-format.git
      path: packages/synx-dart
```

**Usage:**

```dart
import 'package:synx/synx.dart';

// Parse from string
final data = Synx.parse(text);
print(data['server']['port'].asInt); // 8080

// Parse from file
final data = Synx.parseFile('config.synx');

// Full parse (mode detection)
final result = Synx.parseFull(text);
print(result.mode); // SynxMode.active
```

**Flutter usage:**

```dart
import 'package:synx/synx.dart';
import 'package:flutter/services.dart' show rootBundle;

Future<SynxValue> loadConfig() async {
  final text = await rootBundle.loadString('assets/config.synx');
  return Synx.parse(text);
}
```

**API:**

- `Synx.parse(text)` â†’ `SynxValue`
- `Synx.parseFull(text)` â†’ `SynxParseResult { root, mode }`
- `Synx.parseFile(path)` â†’ `SynxValue`
- `SynxValue[key]` â€” subscript by string key or int index
- `.asBool`, `.asInt`, `.asFloat`, `.asStr`
- `.length`, `.keys`, `.type`, `.isNull`

> ðŸ“Œ **Note:** The Dart parser is static-only. No active function evaluation. Requires Dart 3.0+.

---

### 5.15 PHP â€” Native Parser

**Installation:**

Copy `Synx.php` into your project:

```php
require_once 'Synx.php';
```

**Usage:**

```php
require_once 'Synx.php';

// Parse from string
$data = Synx::parse($text);
echo $data->get('server')->get('port')->asInt(); // 8080

// Parse from file
$data = Synx::loadFile('config.synx');

// Full parse (mode detection)
$result = Synx::parseFull($text);
echo $result->mode; // "static" or "active"
```

**API:**

- `Synx::parse($text)` â†’ `SynxValue`
- `Synx::parseFull($text)` â†’ `SynxParseResult { root, mode }`
- `Synx::loadFile($path)` â†’ `SynxValue`
- `->get($key)` â€” child by string key or int index
- `->asBool()`, `->asInt()`, `->asFloat()`, `->asStr()`
- `->length()`, `->keys()`, `->type()`, `->isNull()`

> ðŸ“Œ **Note:** The PHP parser is static-only. No active function evaluation. Requires PHP 8.0+.

---

### 5.16 Bash / PowerShell â€” Shell Parsers

#### Bash (4.0+)

**Installation:**

```bash
source synx.sh
```

**Usage:**

```bash
source synx.sh

synx_parse_file "config.synx"
echo "$(synx_get server.host)"     # localhost
echo "$(synx_get server.port)"     # 8080
echo "$(synx_type server.port)"    # int
echo "$(synx_mode)"                # static

# Arrays
echo "$(synx_get items.0)"         # Sword
echo "$(synx_get items.__length)"  # 3
```

**API:**

- `synx_parse "$text"` â€” parse SYNX text
- `synx_parse_file "$path"` â€” read and parse file
- `synx_get "$path"` â€” value at dot-notation path
- `synx_type "$path"` â€” type name
- `synx_mode` â€” document mode

> **Note:** Values are stored in flat associative arrays with dot-notation keys (e.g. `server.host`).

#### PowerShell (5.1+)

**Installation:**

```powershell
. .\Synx.ps1
```

**Usage:**

```powershell
. .\Synx.ps1

$result = Read-SynxFile "config.synx"
$result.Root.server.host           # localhost
$result.Root.server.port           # 8080 (int)
$result.Root.items[0]              # Sword
$result.Mode                       # static
```

**API:**

- `ConvertFrom-Synx $text` â†’ `PSCustomObject { Root, Mode }`
- `Read-SynxFile $path` â†’ same

Objects are `[ordered]@{}`, arrays are native PowerShell arrays, scalars are typed (`[long]`, `[double]`, `[bool]`, `[string]`, `$null`).

> ðŸ“Œ **Note:** Both shell parsers are static-only. No active function evaluation.

---

### 5.17 Comparison: SYNX Directly vs JSON

| | JSON | SYNX |
|---|---|---|
| **Reading (JS)** | `JSON.parse(fs.readFileSync(...))` | `Synx.loadSync('file.synx')` |
| **Reading (Python)** | `json.load(open(...))` | `Synx.load('file.synx')` |
| **Reading (Rust)** | `serde_json::from_str(...)` | `Synx::parse_file(...)` |
| **Reading (C)** | `cJSON_Parse(...)` | `synx_parse(text)` |
| **Reading (C++)** | `nlohmann::json::parse(...)` | `synx::Synx::load(path)` |
| **Reading (C#)** | `JsonSerializer.Deserialize(...)` | `SynxParser.Load(path)` |
| **Reading (Go)** | `json.Unmarshal(...)` | `synx.ParseFile(path)` |
| **Reading (Java)** | `new ObjectMapper().readTree(...)` | `Synx.load(path)` |
| **Reading (Swift)** | `JSONDecoder().decode(...)` | `Synx.load(path)` |
| **Reading (Lua)** | `cjson.decode(...)` | `synx.parse(text)` |
| **Reading (Dart)** | `jsonDecode(...)` | `Synx.parse(text)` |
| **Reading (PHP)** | `json_decode(...)` | `Synx::parse($text)` |
| **Reading (Bash)** | `jq '.key' file.json` | `synx_get "key"` |
| **Reading (PowerShell)** | `ConvertFrom-Json` | `ConvertFrom-Synx` |
| **Intermediate format** | None | None â€” same! |
| **What you get** | Object / dict / Value | Object / dict / Value (same) |
| **Built-in logic** | No â€” write it yourself | `:random`, `:calc`, `:env`, etc. |
| **Validation** | No â€” need a separate lib | `[min:3, type:int]` in the file |
| **File size** | ~100% | ~40% smaller |

**By language:**

| Language | Package | Method | Notes |
|---|---|---|---|
| JavaScript | `@aperturesyndicate/synx` | `Synx.loadSync()` | Full engine (active + static) |
| Python | `synx-format` | `Synx.load()` | Full engine (active + static) |
| Rust | `synx` (crates.io) | `Synx::parse()` | Zero-dependency, static-only |
| C | `synx.h` + `synx.c` | `synx_parse()` | Zero-dependency, static-only |
| C++ | `synx.hpp` | `synx::Synx::load()` | Header-only, C++17, static-only |
| C# | `Synx` (NuGet) | `SynxParser.Parse()` | .NET Standard 2.0+, static-only |
| Go | `github.com/aperturesyndicate/synx` | `synx.Parse()` | Zero-dependency, static-only |
| Java/Kotlin | `com.aperturesyndicate:synx` | `Synx.parse()` | Java 11+, static-only |
| Swift | Swift Package Manager | `Synx.parse()` | Swift 5.9+, static-only |
| Lua | `synx.lua` (copy) | `synx.parse()` | Lua 5.1+, zero-dep, static-only |
| Dart/Flutter | `synx` (pub.dev / git) | `Synx.parse()` | Dart 3.0+, static-only |
| PHP | `Synx.php` (copy) | `Synx::parse()` | PHP 8.0+, zero-dep, static-only |
| Bash | `synx.sh` (copy) | `synx_get()` | Bash 4.0+, zero-dep, static-only |
| PowerShell | `Synx.ps1` (copy) | `ConvertFrom-Synx` | PowerShell 5.1+, zero-dep, static-only |

---

## 6. Conversion to JSON (optional)

> ðŸ“Ž JSON conversion is an **optional** feature. It's only needed if you want to pass data to a system that exclusively understands JSON (third-party APIs, legacy code, etc.).

### 6.1 Via VS Code (SYNX Extension)

If the **SYNX for VS Code** extension is installed:

1. Open a `.synx` file
2. **Right-click** on the file
3. Select **"Convert to JSON"**
4. A `.json` file will appear alongside

### 6.2 Via Terminal (CLI)

```bash
# Convert to JSON and print to console
synx to-json config.synx

# Convert and save to file
synx to-json config.synx -o config.json

# Freeze an !active config into a static .synx (without functions)
synx freeze active_config.synx -o static_config.synx
```

### 6.3 Programmatically (if you really need to)

```python
import json
from synx import Synx

data = Synx.load('config.synx')
json_string = json.dumps(data, ensure_ascii=False, indent=2)
```

```javascript
const Synx = require('@aperturesyndicate/synx');
const data = Synx.loadSync('config.synx');
const jsonString = JSON.stringify(data, null, 2);
```

> But again: **this is not needed to work with SYNX**. Your code reads `.synx` directly.

---

## 7. Quick Reference

```
KEY VALUE                        â†’ simple pair
key                              â†’ empty object (group)
  nested_key value               â†’ nesting (2 spaces)
key |                            â†’ multiline text
  line 1                           (block)
  line 2
list                             â†’ list
  - item 1
  - item 2
# comment                       â†’ single-line comment
// comment                      â†’ single-line comment

â”€â”€â”€ Only with !active â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
key:random                       â†’ random item
key:random 70 20 10              â†’ weighted random
key:calc A * B                   â†’ arithmetic
key:env VAR                      â†’ environment variable
key:env:default:X VAR            â†’ env with default
key:alias other_key              â†’ reference to another key
key:secret value                 â†’ hidden value
key:unique                       â†’ list deduplication
key:include ./file.synx          â†’ include file
key[min:N, max:N]                â†’ length/value constraints
key[type:int]                    â†’ data type
key[required]                    â†’ required field
key[enum:a|b|c]                  â†’ allowed values
key[pattern:regex]               â†’ regex validation
key[readonly]                    â†’ read-only
```
