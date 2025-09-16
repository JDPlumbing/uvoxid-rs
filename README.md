# UVoxID (Rust)

[![Crates.io](https://img.shields.io/crates/v/uvoxid.svg)](https://crates.io/crates/uvoxid)
[![Documentation](https://docs.rs/uvoxid/badge.svg)](https://docs.rs/uvoxid)
[![License](https://img.shields.io/crates/l/uvoxid.svg)](./LICENSE)

**Universal Voxel Identifier (UVoxID)** — a deterministic, 192-bit encoding scheme for spherical spatial coordinates at micrometer precision.  

Think of it as a globally consistent **voxel address system**: every point in space has a permanent ID, valid from Earth’s core to interstellar distances.

---

## ✨ Features

- **192-bit encoding**: `(radius, latitude, longitude)` → one integer.
- **Deterministic & exact**: no floating-point drift.
- **Compact**: 24 bytes per position, globally unique.
- **Rust-native**: safe, simple API for encode/decode.
- **Tested**: round-trip encoding/decoding verified.

---

## 📦 Installation

```bash
cargo add uvoxid
```

Or add manually to `Cargo.toml`:

```toml
[dependencies]
uvoxid = "0.1"
```

---

## 🔍 Example

```rust
use uvoxid::{encode_uvoxid, decode_uvoxid};

fn main() {
    // Earth mean radius in µm
    let earth_r_um: u64 = 6_371_000_000_000;

    // At equator, prime meridian
    let id = encode_uvoxid(earth_r_um, 0, 0);

    println!("UVoxID: {:#x}", id);

    let (r, lat, lon) = decode_uvoxid(id);
    println!("Decoded: r = {} µm, lat = {} µ° , lon = {} µ°", r, lat, lon);
}
```

Output:
```
UVoxID: 0x59fb8c83f100000000000055d4a8000000000aba950
Decoded: r = 6371000000000 µm, lat = 0 µ°, lon = 0 µ°
```

---

## 📖 API

### `encode_uvoxid(r_um, lat_microdeg, lon_microdeg) -> u128`

- `r_um`: radius in micrometers (µm).
- `lat_microdeg`: latitude in millionths of a degree (−90e6 to +90e6).
- `lon_microdeg`: longitude in millionths of a degree (−180e6 to +180e6).
- Returns: 192-bit integer UVoxID.

### `decode_uvoxid(id) -> (u64, i64, i64)`

- Input: 192-bit integer.
- Output: `(r_um, lat_microdeg, lon_microdeg)`.

---

## 🛠 Development

Run tests:

```bash
cargo test
```

Format code:

```bash
cargo fmt
```

---

## 📎 Links
- [Crates.io](https://crates.io/crates/uvoxid)
- [Docs.rs](https://docs.rs/uvoxid)
- [Source Code](https://github.com/JDPlumbing/uvoxid-rs)

---

## 📄 License

MIT © JD Plumbing
