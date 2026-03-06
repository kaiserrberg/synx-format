<p align="center">
  <img src="https://aperturesyndicate.com/branding/aperturesyndicate.png" alt="APERTURESyndicate" width="400" />
</p>

<h1 align="center">SYNX v3.0 — Guía Completa</h1>

<p align="center">
  <strong>Mejor que JSON. Más barato que YAML. Hecho para IA y humanos.</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-3.0.0-5a6eff?style=for-the-badge" />
  <img src="https://img.shields.io/badge/license-MIT-blue?style=for-the-badge" />
  <img src="https://img.shields.io/badge/format-SYNX-blueviolet?style=for-the-badge" />
  <img src="https://img.shields.io/badge/written_in-Rust-orange?style=for-the-badge" />
</p>

<p align="center">
  <a href="https://www.npmjs.com/package/@aperturesyndicate/synx">npm</a> ·
  <a href="https://pypi.org/project/synx-format/">PyPI</a> ·
  <a href="https://crates.io/crates/synx-core">crates.io</a> ·
  <a href="https://marketplace.visualstudio.com/items?itemName=APERTURESyndicate.synx-vscode">VS Code</a> ·
  <a href="https://github.com/kaiserrberg/synx-format">GitHub</a>
</p>

---

## Tabla de Contenidos

- [Filosofía de Diseño](#-filosofía-de-diseño)
- [Demostración](#-demostración)
- [Cómo Funciona](#-cómo-funciona)
- [Rendimiento y Benchmarks](#-rendimiento-y-benchmarks)
- [Instalación](#-instalación)
- [Referencia de Sintaxis](#-referencia-de-sintaxis)
  - [Sintaxis Básica](#sintaxis-básica)
  - [Anidación](#anidación)
  - [Listas](#listas)
  - [Conversión de Tipos](#conversión-de-tipos)
  - [Texto Multilínea](#texto-multilínea)
  - [Comentarios](#comentarios)
- [Modo Activo (`!active`)](#-modo-activo-active)
- [Referencia de Marcadores](#-referencia-completa-de-marcadores)
  - [:env — Variables de Entorno](#env--variables-de-entorno)
  - [:default — Valor por Defecto](#default--valor-por-defecto)
  - [:calc — Expresiones Aritméticas](#calc--expresiones-aritméticas)
  - [:random — Selección Aleatoria](#random--selección-aleatoria)
  - [:alias — Referencia a Otra Clave](#alias--referencia-a-otra-clave)
  - [:secret — Valor Oculto](#secret--valor-oculto)
  - [:template — Interpolación de Cadenas](#template--interpolación-de-cadenas)
  - [:include — Importar Archivo Externo](#include--importar-archivo-externo)
  - [:unique — Eliminar Duplicados](#unique--eliminar-duplicados)
  - [:split — Cadena a Arreglo](#split--cadena-a-arreglo)
  - [:join — Arreglo a Cadena](#join--arreglo-a-cadena)
  - [:geo — Selección por Región](#geo--selección-por-región)
  - [:clamp — Limitación Numérica](#clamp--limitación-numérica)
  - [:round — Redondeo](#round--redondeo)
  - [:map — Tabla de Búsqueda](#map--tabla-de-búsqueda)
  - [:format — Formateo de Números](#format--formateo-de-números)
  - [:fallback — Ruta de Archivo con Respaldo](#fallback--ruta-de-archivo-con-respaldo)
  - [:once — Generar y Persistir](#once--generar-y-persistir)
  - [:version — Comparación Semántica de Versiones](#version--comparación-semántica-de-versiones)
  - [:watch — Leer Archivo Externo](#watch--leer-archivo-externo)
- [Restricciones](#-restricciones)
- [Encadenamiento de Marcadores](#-encadenamiento-de-marcadores)
- [Ejemplos de Código](#-ejemplos-de-código)
- [Soporte de Editores](#-soporte-de-editores)
- [Arquitectura](#-arquitectura)
- [Enlaces](#-enlaces)

---

## 💡 Filosofía de Diseño

La configuración es la base de cada aplicación. Sin embargo, los formatos estándar de la industria — **JSON** y **YAML** — nunca fueron diseñados para esto:

| Problema | JSON | YAML | SYNX |
|---|:---:|:---:|:---:|
| Comillas en todas partes | ✗ | ✓ | ✓ |
| Error por coma final | ✗ | — | ✓ |
| Indentación sensible a espacios | — | ✗ (peligroso) | ✓ (seguro, 2 espacios) |
| Soporte de comentarios | ✗ | ✓ | ✓ |
| Variables de entorno | ✗ | ✗ | ✓ nativo |
| Valores calculados | ✗ | ✗ | ✓ nativo |
| Costo en tokens IA (110 claves) | ~3300 chars | ~2500 chars | **~2000 chars** |
| Legibilidad | Baja | Media | **Alta** |

SYNX se construye sobre tres principios:

1. **Sintaxis mínima** — clave, espacio, valor. Sin comillas, sin comas, sin llaves, sin dos puntos.
2. **Activo por naturaleza** — la configuración no es solo datos, es lógica. Variables de entorno, matemáticas, referencias, selección aleatoria y validación — todo integrado.
3. **Eficiente en tokens** — al enviar configuración a través de un LLM, cada carácter importa. SYNX ahorra 30–40% de tokens respecto a JSON.

> **SYNX no es un reemplazo de JSON. SYNX es lo que JSON debió haber sido.**

---

## 🎬 Demostración

### Escritura de datos — limpia y sencilla

Solo **clave**, **espacio**, **valor**. Sin comillas, sin comas, sin llaves:

<p align="center">
  <img src="https://aperturesyndicate.com/branding/gifs/synx/synx.gif" alt="Escribir SYNX estático" width="720" />
</p>

### Modo `!active` — configuración con lógica

Agrega `!active` en la primera línea y tu configuración cobra vida — funciones integradas directamente en el formato:

<p align="center">
  <img src="https://aperturesyndicate.com/branding/gifs/synx/synx2.gif" alt="Escribir SYNX activo con marcadores" width="720" />
</p>

---

## ⚙ Cómo Funciona

El pipeline de SYNX tiene **dos etapas** — esta separación es clave para el rendimiento:

```
┌───────────────┐         ┌─────────────┐         ┌──────────────┐
│  Archivo .synx │ ──────► │   Parser    │ ──────► │    Salida    │
│  (texto)       │         │ (siempre)   │         │ (objeto JS)  │
└───────────────┘         └──────┬──────┘         └──────────────┘
                                 │
                          ¿tiene !active?
                                 │
                            ┌────▼────┐
                            │  Motor  │
                            │(ejecuta │
                            │marcado- │
                            │  res)   │
                            └─────────┘
```

### Etapa 1 — Parser

El **parser** lee el texto crudo y construye el árbol de clave-valor. Maneja pares clave-valor, anidación (indentación de 2 espacios), listas, conversión de tipos, comentarios y texto multilínea.

El parser registra los marcadores (`:env`, `:calc`, etc.) como **metadatos** adjuntos a cada clave, pero **no los ejecuta**. Esto significa que **agregar nuevos marcadores no ralentiza el parsing**.

### Etapa 2 — Motor (solo con `!active`)

Si el archivo comienza con `!active`, el **motor** recorre el árbol parseado y resuelve cada marcador.

**Los archivos sin `!active` nunca tocan el motor.**

---

## 📊 Rendimiento y Benchmarks

Todos los benchmarks son con datos reales, ejecutados sobre una configuración SYNX estándar de 110 claves (2.5 KB):

### Rust (criterion, llamada directa)

| Benchmark | Tiempo |
|---|---|
| `Synx::parse` (110 claves) | **~39 µs** |
| `parse_to_json` (110 claves) | **~42 µs** |
| `Synx::parse` (4 claves) | **~1.2 µs** |

### Node.js (50,000 iteraciones)

| Parser | µs/op | vs JSON | vs YAML |
|---|---:|---:|---:|
| `JSON.parse` (3.3 KB) | 6.08 µs | 1× | — |
| **`synx-js` TS puro** | **39.20 µs** | 6.4× | **2.1× más rápido que YAML** |
| `js-yaml` (2.5 KB) | 82.85 µs | 13.6× | 1× |

### Python (10,000 iteraciones)

| Parser | µs/op | vs YAML |
|---|---:|---:|
| `json.loads` (3.3 KB) | 13.04 µs | — |
| **`synx_native.parse`** | **55.44 µs** | **67× más rápido que YAML** |
| `yaml.safe_load` (2.5 KB) | 3,698 µs | 1× |

> En Python, SYNX parsea **67 veces** más rápido que YAML.

---

## 📦 Instalación

### Node.js / Navegador

```bash
npm install @aperturesyndicate/synx
```

### Python

```bash
pip install synx-format
```

### Rust

```bash
cargo add synx-core
```

### Extensión VS Code

Busca **"SYNX"** en el panel de extensiones, o:

```bash
code --install-extension APERTURESyndicate.synx-vscode
```

---

## 📝 Referencia de Sintaxis

### Sintaxis Básica

Regla fundamental: **clave** `(espacio)` **valor**.

```synx
name John
age 25
phrase ¡Me encanta programar!
empty_value
```

> Los números, booleanos (`true`/`false`) y `null` se detectan automáticamente. Todo lo demás es cadena.

---

### Anidación

La indentación crea jerarquía — **2 espacios** por nivel:

```synx
server
  host 0.0.0.0
  port 8080
  ssl
    enabled true
```

---

### Listas

Las líneas que comienzan con `- ` crean arreglos:

```synx
fruits
  - Apple
  - Banana
  - Cherry
```

---

### Conversión de Tipos

Usa `(tipo)` después del nombre de la clave para forzar el tipo:

```synx
zip_code(string) 90210
id(int) 007
ratio(float) 3
enabled(bool) 1
```

Tipos disponibles: `int`, `float`, `bool`, `string`.

---

### Texto Multilínea

Usa el operador `|`:

```synx
description |
  Esta es una descripción larga
  que abarca múltiples líneas.
```

---

### Comentarios

```synx
# Comentario con almohadilla
// Comentario con barras
name John  # Comentario en línea
```

---

## 🔥 Modo Activo (`!active`)

Coloca `!active` en la **primera línea** para desbloquear marcadores y restricciones.

```synx
!active

port:env PORT
boss_hp:calc base_hp * 5
```

---

## 🧩 Referencia Completa de Marcadores

SYNX v3.0 proporciona **20 marcadores**. Cada marcador es una función que se adjunta a una clave mediante la sintaxis `:marcador`.

### `:env` — Variables de Entorno

```synx
!active
port:env PORT
port:env:default:8080 PORT
```

### `:default` — Valor por Defecto

```synx
!active
theme:default dark
```

### `:calc` — Expresiones Aritméticas

```synx
!active
base_price 100
tax:calc base_price * 0.2
total:calc base_price + tax
```

Operadores: `+` `-` `*` `/` `%` `(` `)`

### `:random` — Selección Aleatoria

```synx
!active
loot:random 70 20 10
  - common
  - rare
  - legendary
```

### `:alias` — Referencia a Otra Clave

```synx
!active
admin_email alex@example.com
billing:alias admin_email
```

### `:secret` — Valor Oculto

```synx
!active
api_key:secret sk-1234567890
```

### `:template` — Interpolación de Cadenas

```synx
!active
name John
greeting:template ¡Hola, {name}!
```

### `:include` — Importar Archivo Externo

```synx
!active
database:include ./db.synx
```

### `:unique` — Eliminar Duplicados

```synx
!active
tags:unique
  - action
  - rpg
  - action
```

Resultado: `["action", "rpg"]`

### `:split` — Cadena a Arreglo

```synx
!active
colors:split red, green, blue
words:split:space hello world foo
```

Palabras clave de separador: `space`, `pipe`, `dash`, `dot`, `semi`, `tab`

### `:join` — Arreglo a Cadena

```synx
!active
path:join:slash
  - home
  - user
  - docs
```

Resultado: `"home/user/docs"`

### `:geo` — Selección por Región

```synx
!active
currency:geo
  - US USD
  - EU EUR
  - MX MXN
```

### `:clamp` — Limitación Numérica

```synx
!active
volume:clamp:0:100 150
```

Resultado: `100`

### `:round` — Redondeo

```synx
!active
price:round:2 19.999
profit:calc:round:2 revenue * 0.337
```

### `:map` — Tabla de Búsqueda

```synx
!active
status_code 1
status:map:status_code
  - 0 desconectado
  - 1 en línea
  - 2 ausente
```

Resultado: `"en línea"`

### `:format` — Formateo de Números

```synx
!active
price:format:%.2f 1234.5
id:format:%06d 42
```

Resultado: `"1234.50"`, `"000042"`

### `:fallback` — Ruta de Archivo con Respaldo

```synx
!active
icon:fallback:./default.png ./custom.png
```

### `:once` — Generar y Persistir

```synx
!active
session_id:once uuid
app_seed:once random
build_time:once timestamp
```

Tipos de generación: `uuid` (por defecto), `random`, `timestamp`

### `:version` — Comparación Semántica de Versiones

```synx
!active
runtime:version:>=:18.0 20.11.0
```

Resultado: `true`. Operadores: `>=` `<=` `>` `<` `==` `!=`

### `:watch` — Leer Archivo Externo

```synx
!active
app_name:watch:name ./package.json
config:watch ./data.txt
```

---

## 🔒 Restricciones

Las restricciones validan valores durante el parsing. Se definen en `[corchetes]` después del nombre de clave.

| Restricción | Sintaxis | Descripción |
|---|---|---|
| `required` | `key[required]` | Debe tener un valor |
| `readonly` | `key[readonly]` | Solo lectura |
| `min:N` | `key[min:3]` | Longitud/valor mínimo |
| `max:N` | `key[max:100]` | Longitud/valor máximo |
| `type:T` | `key[type:int]` | Forzar tipo |
| `pattern:R` | `key[pattern:^\d+$]` | Validar con regex |
| `enum:A\|B` | `key[enum:light\|dark]` | Valores permitidos |

```synx
!active
app_name[required, min:3, max:30] TotalWario
volume[min:0, max:100, type:int] 75
theme[enum:light|dark|auto] dark
```

---

## 🔗 Encadenamiento de Marcadores

```synx
!active
port:env:default:8080 PORT
profit:calc:round:2 revenue * margin
```

---

## 💻 Ejemplos de Código

### JavaScript / TypeScript

```typescript
import { Synx } from '@aperturesyndicate/synx';

const config = Synx.parse(`
  app_name TotalWario
  server
    host 0.0.0.0
    port 8080
`);

console.log(config.server.port);  // 8080
```

### Python

```python
import synx_native

config = synx_native.parse("""
app_name TotalWario
server
  host 0.0.0.0
  port 8080
""")

print(config["server"]["port"])  # 8080
```

### Rust

```rust
use synx_core::Synx;

let config = Synx::parse("
    app_name TotalWario
    version 3.0.0
");
```

---

## 🛠 Soporte de Editores

### Visual Studio Code

Soporte completo del lenguaje: resaltado de sintaxis, IntelliSense (20 marcadores), diagnósticos en tiempo real (15 verificaciones), ir a definición, formateo, vista previa de colores, sugerencias inline de `:calc`, vista previa JSON en vivo.

### Visual Studio 2022

Extensión MEF: resaltado de sintaxis, IntelliSense, marcado de errores, plegado de código, comandos de conversión.

---

## 🏗 Arquitectura

```
synx-format/
├── crates/synx-core/          # Núcleo Rust — parser + motor
├── bindings/
│   ├── node/                  # NAPI-RS → módulo nativo npm
│   └── python/                # PyO3 → módulo nativo PyPI
├── packages/
│   ├── synx-js/               # Parser + motor TypeScript puro
│   ├── synx-vscode/           # Extensión VS Code
│   └── synx-visualstudio/     # Extensión Visual Studio 2022
├── publish-npm.bat
├── publish-pypi.bat
└── publish-crates.bat
```

---

## 🔗 Enlaces

| Recurso | URL |
|---|---|
| **GitHub** | [github.com/kaiserrberg/synx-format](https://github.com/kaiserrberg/synx-format) |
| **npm** | [npmjs.com/package/@aperturesyndicate/synx](https://www.npmjs.com/package/@aperturesyndicate/synx) |
| **PyPI** | [pypi.org/project/synx-format](https://pypi.org/project/synx-format/) |
| **crates.io** | [crates.io/crates/synx-core](https://crates.io/crates/synx-core) |

---

<p align="center">
  <img src="https://aperturesyndicate.com/branding/logos/asp_128.png" width="96" height="96" />
</p>

<p align="center">
  MIT — © <a href="https://github.com/kaiserrberg">APERTURESyndicate</a>
</p>
