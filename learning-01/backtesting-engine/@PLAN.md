# Backtesting Engine in Rust - Master Plan

*This plan is designed for a beginner to Rust who wants to tackle a genuinely hard, worthwhile project. We will build a robust backtesting engine from scratch. It's okay to fail and restart; that's the best way to learn how the pieces fit together.*

## The Goal
Build a high-performance, event-driven backtesting engine capable of simulating trading strategies over historical market data. 
**Ultimate Objective:** This engine will serve as the core environment for training and backtesting your own custom **Deep Reinforcement Learning (DRL)** models.

By the end of this project, you will have a deep understanding of core Rust concepts: Ownership, Borrowing, Traits, Iterators, Generics, Error Handling, and Concurrency, and you will have a production-ready simulation engine to plug your Python AI models into.

## Collaboration Rules (AI vs. Human)
To maximize learning, the **USER** will write the vast majority of the code. The **AI** will only be utilized for:
1. **Setup & Boilerplate** (e.g., scaffolding Cargo, adding dependencies).
2. **Debugging** (when genuinely stuck on a borrow checker error or cryptic compiler message).
3. **Planning & Architecture** (brainstorming how to attack the next phase).
4. **"Lazy" Tasks** (finishing up highly repetitive or obvious patterns once the user has implemented the core concept).

## Architecture Deep Dive

The engine is built on an **Event-Driven Architecture**. Data flows in one direction:
`Data Source -> Engine -> Strategy -> Engine -> Broker (Execution) -> Portfolio Update`

### 1. Domain Models (`src/models.rs`)
**What it is:** The nouns of our system. 
**Why it matters:** If our foundational types are wrong, everything else gets messy. We need strict types to leverage Rust's compiler guarantees.
- `Candle` & `Tick`: Represent market data.
- `Order`: An intent to buy or sell (Market, Limit, Stop).
- `Trade`/`Fill`: A confirmed execution from the broker.
- `Position`: A currently held asset state.
**Rust Concepts to Learn:** Structs, Enums, `format!`/`Display` traits, `derive` macros (Debug, Clone, PartialEq).

### 2. Data Ingestion (`src/data.rs`)
**What it is:** Loading historical data (e.g., CSV files) and turning it into our Rust structs.
**Why it matters:** Backtesting requires processing millions of rows. We cannot load all of it into memory at once.
**Rust Concepts to Learn:**
- **Iterators:** How to yield data line-by-line (`Iterator` trait, `next()`).
- **File I/O:** Reading files efficiently (`BufReader`).
- **Serialization:** Using the `serde` and `csv` crates to automatically convert CSV rows to Rust structs.
- **Error Handling:** Using `Result<T, E>` and `?` operator when file parsing fails.

### 3. Broker / Execution Simulator (`src/broker.rs`)
**What it is:** The "fake exchange". It receives `Order`s and historical data, and determines if/when those orders would have filled (considering slippage and fees).
**Why it matters:** A backtest is only as good as its execution simulation.
**Rust Concepts to Learn:**
- **Mutable State:** Tracking pending orders and processing them requires mutating state safely.
- **Borrowing (`&` vs `&mut`):** Passing the market data (immutable reference) vs updating the broker state (mutable reference).
- **Option Enum:** Handling cases where an order *might* not fill (`Option<Fill>`).

### 4. Portfolio Management (`src/portfolio.rs`)
**What it is:** Tracks cash balance, active positions, and performance metrics.
**Why it matters:** You need to know if your strategy actually made money and track risk.
**Rust Concepts to Learn:**
- **HashMaps:** Storing positions by symbol (`HashMap<String, Position>`).
- **Math & Decimals:** Handling floating-point precision (using `f64` or specialized crates like `rust_decimal` to avoid rounding errors).

### 5. Strategy Interface (`src/strategy.rs`)
**What it is:** Where the trading logic lives. It looks at data and portfolio state, and emits `Order`s.
**Why it matters:** We want the engine to be reusable. You should be able to plug in Strategy A or Strategy B without rewriting the engine.
**Rust Concepts to Learn:**
- **Traits:** Defining a common interface (`trait Strategy { fn process(...) }`).
- **Dynamic vs Static Dispatch:** Understanding `Box<dyn Strategy>` vs Generics `<S: Strategy>`. (We'll start with generics for performance).

### 6. The Engine Loop (`src/engine.rs`)
**What it is:** The conductor of the orchestra. It loops over the data, hands it to the strategy, takes orders, gives them to the broker, and updates the portfolio.
**Why it matters:** Ties everything together.
**Rust Concepts to Learn:**
- **Ownership:** Who "owns" the data as it passes through the loop? 
- **Lifetimes (`'a`):** If the engine holds a reference to a strategy, how long does that reference live?

---

## The "Fail Forward" Approach
1. **Build a naive version:** We will write code that is ugly but works.
2. **Hit the compiler wall:** The Rust borrow checker will yell at us. This is good. It teaches us architecture.
3. **Refactor:** We will learn the "Rust way" to solve the architecture problems we create.
