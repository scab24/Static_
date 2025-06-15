

# Parses this toy language program into an AST:

```
x = 5;
y = 1;
while (x > 0) {
    y = y * x;
    x = x - 1;
}
```

## Dependencies

`Cargo.toml`:

```toml
[dependencies]
log = "0.4"
env_logger = "0.11"
```

- **What**: Adds `log` for logging and `env_logger` for console output.
- **How**: Included in `Cargo.toml`, installed with `cargo build`.
- **Why**: Enables `log::error!` for structured error logging.

## Code Summary (`src/main.rs`)

### 1. Definition of `Expr` (lines 1-5)
```rust
// Expressions
#[derive(Clone, Debug)]
enum Expr { Int(i32), Var(String), BinOp(String, Box<Expr>, Box<Expr>) }
```
- **What**: Enum for expressions (numbers, variables, operations).
- **How**: Defines `Int`, `Var`, `BinOp`; uses `Clone`, `Debug`.
- **Why**: Models expressions like `5`, `x`, `x + y` for the AST.

### 2. Definition of `Stmt` (lines 7-12)
```rust
// Statements
#[derive(Clone, Debug)]
enum Stmt { Assign(String, Expr), If(Expr, Vec<Stmt>, Vec<Stmt>), While(Expr, Vec<Stmt>), Seq(Vec<Stmt>) }
```
- **What**: Enum for statements (assignments, conditionals, loops).
- **How**: Defines variants; uses `Vec<Stmt>` for blocks.
- **Why**: Represents actions like `x = 5` or `while`.

### 3. Definition of `Program` (lines 14-17)
```rust
// Program
#[derive(Debug)]
struct Program { stmts: Vec<Stmt> }
```
- **What**: Struct for the program.
- **How**: Stores statement list with `Debug`.
- **Why**: Represents the full AST.

### 4. Definition of `ParseError` (lines 19-23)
```rust
// Errors
#[derive(Debug)]
enum ParseError { InvalidAssignment(String), InvalidNumber(String) }
```
- **What**: Enum for parser errors.
- **How**: Defines `InvalidAssignment`, `InvalidNumber`.
- **Why**: Structures error handling for `Result`.

### 5. Function `parse_assignment` (lines 25-35)
```rust
fn parse_assignment(line: &str, var: &str) -> Result<Stmt, ParseError> {
```
- **What**: Parses assignments like `x = 5;`.
- **How**: Takes line and prefix; returns `Result`.
- **Why**: Builds assignment nodes safely.

```rust
if !line.starts_with(var) { return Err(ParseError::InvalidAssignment(format!("Expected '{} =', got '{}'", var, line))); }
```
- **What**: Checks assignment prefix.
- **How**: Errors if prefix doesn’t match.
- **Why**: Validates line format.

```rust
let parts: Vec<&str> = line.split('=').collect();
if parts.len() < 2 { return Err(ParseError::InvalidAssignment(format!("Missing value in '{}'", line))); }
```
- **What**: Splits line, checks for value.
- **How**: Uses `split('=')`; errors if no value.
- **Why**: Ensures valid assignment structure.

```rust
let value_str = parts[1].trim().replace(";", "");
```
- **What**: Cleans value string.
- **How**: Removes spaces, semicolon.
- **Why**: Prepares for number parsing.

```rust
let value = value_str.parse::<i32>().map_err(|_| ParseError::InvalidNumber(format!("Invalid number in '{}': '{}'", line, value_str)))?;
```
- **What**: Parses value to `i32`.
- **How**: Uses `parse`; maps errors to `ParseError`.
- **Why**: Safely converts string to number.

```rust
Ok(Stmt::Assign(var.trim_end_matches('=').to_string(), Expr::Int(value)))
```
- **What**: Returns parsed assignment.
- **How**: Creates `Stmt::Assign` with clean variable.
- **Why**: Adds assignment to AST.

### 6. Function `parse_program` (lines 37-72)
```rust
fn parse_program(input: &str) -> Result<Program, ParseError> {
```
- **What**: Parses program into `Program`.
- **How**: Returns `Result` with AST or error.
- **Why**: Builds the program AST.

```rust
let lines: Vec<&str> = input.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
```
- **What**: Splits input into non-empty lines.
- **How**: Trims and filters lines.
- **Why**: Simplifies parsing.

```rust
let mut stmts = Vec::new(); let mut i = 0; while i < lines.len() { let line = lines[i];
```
- **What**: Sets up statement list, loops lines.
- **How**: Uses `while` with index `i`.
- **Why**: Processes lines sequentially.

```rust
if line.starts_with("x =") { stmts.push(parse_assignment(line, "x =")?); i += 1;
```
- **What**: Parses `x` assignment.
- **How**: Calls `parse_assignment`, adds to `stmts`.
- **Why**: Handles `x = 5;`.

```rust
} else if line.starts_with("y =") { stmts.push(parse_assignment(line, "y =")?); i += 1;
```
- **What**: Parses `y` assignment.
- **How**: Similar to `x =`.
- **Why**: Handles `y = 1;`.

```rust
} else if line.starts_with("while") { let body = vec![...]; stmts.push(Stmt::While(...)); i += 5;
```
- **What**: Parses `while` loop.
- **How**: Builds loop AST, skips 5 lines.
- **Why**: Represents `while` block.

```rust
} else { i += 1; }
```
- **What**: Ignores unrecognized lines.
- **How**: Advances index.
- **Why**: Tolerates `{`, `}`, etc.

```rust
Ok(Program { stmts })
```
- **What**: Returns parsed program.
- **How**: Creates `Program` with statements.
- **Why**: Completes AST.

### 7. Function `main` (lines 74-85)
```rust
fn main() {
```
- **What**: Program entry point.
- **How**: Runs the parser.
- **Why**: Executes logic.

```rust
env_logger::init();
```
- **What**: Sets up logging.
- **How**: Calls `env_logger::init()`.
- **Why**: Enables `log::error!`.

```rust
let input = "...";
```
- **What**: Defines input program.
- **How**: Multiline string.
- **Why**: Provides parsing input.

```rust
match parse_program(input) { Ok(program) => println!("Program parsed: {:?}", program), Err(e) => { log::error!("Error parsing: {:?}", e); std::process::exit(1); }
```
- **What**: Runs parser, handles result.
- **How**: Prints AST or logs error, exits.
- **Why**: Shows output or errors.

## How to Test

1. Update `Cargo.toml`.
2. Save code in `src/main.rs`.
3. Run: `cargo run`
   - **Output**: `Program parsed: Program { stmts: [Assign("x", Int(5)), Assign("y", Int(1)), While(...)] }`
4. Test error (change `x = 5;` to `x = abc;`): `RUST_LOG=error cargo run`
   - **Output**: `[2025-06-15T12:37:00Z ERROR static_book] Error parsing: InvalidNumber(...)`
