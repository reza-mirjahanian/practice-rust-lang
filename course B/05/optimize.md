
### Compacting and Optimizing Rust Executables


### Default Build with Cargo

When you write a piece of code in Rust and use `cargo build` by default, the executable is not highly optimized. This is the debug version, which is larger and not suitable for production.

**Example:**
- **Code:** A simplistic code running a server with Tokio.
- **File Size:** The default build size is 4.4 MB.

### Optimizing with Cargo Release

One of the basic approaches to optimize the binary file is to use the release mode.

**Steps:**
1. **Build Command:**
   ```sh
   cargo build --release
   ```
2. **Check File Size:** The file size reduces significantly in release mode.

**Result:**
- **Debug Mode Size:** 4.4 MB
- **Release Mode Size:** 1.4 MB

### Using `cargo bloat`

`cargo bloat` helps identify which parts of your code take up the most space.

**Installation:**
```sh
cargo install cargo-bloat
```

**Usage:**
1. **Analyze Release Build:**
   ```sh
   cargo bloat --release
   ```
2. **Example Output:**
   ```sh
   $ cargo bloat --release
   File: target/release/your_project
   Size: 1.4 MB
   Sections: ...
   ```

### Identifying Large Crates

Dependencies can add a lot of space to your binary file. Using `cargo bloat`, you can identify space-consuming crates.

**Command:**
```sh
cargo bloat --release --crates
```

**Example Output:**
```sh
STD 61% (Standard library)
Tokio 27% (Async runtime)
```

### Stripping Symbols

Stripping symbols from the binary can further reduce its size.

**Steps:**
1. **Edit `Cargo.toml`:**
   ```toml
   [profile.release]
   strip = true
   ```
2. **Rebuild Project:**
   ```sh
   cargo build --release
   ```

**Result:**
- **Size Before Stripping:** 1.4 MB
- **Size After Stripping:** 0.7 MB

### Optimizing Further with `opt-level`

Setting the optimization level to `s` can further reduce the size.

**Steps:**
1. **Edit `Cargo.toml`:**
   ```toml
   [profile.release]
   opt-level = "s"
   ```
2. **Rebuild Project:**
   ```sh
   cargo build --release
   ```

**Result:** A slight reduction in size for further optimization.

### Conclusion

To summarize, here are the steps to optimize a Rust executable:

1. **Use Release Mode:** `cargo build --release`
2. **Analyze with `cargo bloat`:** `cargo bloat --release`
3. **Identify Large Crates:** `cargo bloat --release --crates`
4. **Strip Symbols:** Add `strip = true` to `[profile.release]` in `Cargo.toml`
5. **Optimize Level:** Add `opt-level = "s"` to `[profile.release]` in `Cargo.toml`

By following these steps, you can significantly reduce the size of your Rust binaries. There are more advanced techniques and tools available, which I may cover in future videos.

