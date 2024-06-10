
**Introduction**

-   The video will cover 5 ways to improve coding experience in Rust, plus a bonus tip
-   The tips will be valuable for both beginners and experienced developers

**Tip 1: Setting up VS Code for Rust Development**

-   The author recommends using VS Code as the IDE for Rust development
-   Three must-have extensions for Rust development in VS Code:
    
    1.  Rust Analyzer (official language server for Rust)
    2.  Code LLDB (debugging extension)
    3.  Better TOML (syntax highlighting for TOML files)
    
-   Optional extensions:
    
    1.  Aerolens (improves error highlighting)
    2.  To Do Tree (helps find to-do items in code)
    3.  Crates (checks for outdated dependencies)
    

**Tip 2: Setting up Rust Project**

-   Configuring automatic linting and formatting using Clippy and Rust Format
-   Setting up continuous integration with GitHub Actions
-   Using cargo watch for live reloading

**Tip 3: Avoiding Over-Engineering**

-   The principle of "make it work, then make it right, then make it fast"
-   Writing code that simply solves the problem, then making it more idiomatic and performant
-   Example of refactoring code to make it more idiomatic and performant

**Tip 4: Familiarizing with Popular Rust Crates**

-   Importance of knowing popular third-party crates in Rust
-   Examples of popular crates:
    
    1.  Serde and Certain Json (serialization and deserialization)
    2.  This Error and Anyhow (error handling)
    3.  Tokio (async runtime)
    
-   Resource: blessed.rs (curated list of Rust crates)

**Tip 5: Learning Rust-Specific Design Patterns**

-   Importance of learning Rust-specific design patterns
-   Example of the extension trait pattern (adding methods to a type outside of the crate where the type was defined)
-   Other design patterns mentioned:
    
    1.  Type state pattern
    2.  Interior mutability
    3.  RAII (Resource Acquisition Is Initialization)
    4.  Builder pattern
    

**Bonus Tip: Leveraging AI**

-   The author's opinion on AI in software development
-   Examples of AI tools that can improve productivity:
    
    1.  GitHub Copilot (helps write more idiomatic code)
    2.  ChatGPT (helps with repetitive or generic code)
    
-   The author's use of AI tools in the video (e.g. generating GitHub Actions workflow file)