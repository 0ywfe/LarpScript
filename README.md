
# LarpScript

> A programming language for the chronically online.

LarpScript is a custom interpreted programming language written in Rust. It features a slang-inspired syntax and uses [Pest](https://pest.rs/) for grammar definition and parsing.

## Syntax

LarpScript uses custom keywords for its language constructs.

| Keyword | Description |
|---|---|
| `cook` | Variable declaration |
| `locked` | Constant declaration |
| `yap` | Print statement |
| `fax` | Boolean `true` |
| `cap` | Boolean `false` |
| `nah` | Null value |
| `if` | Conditional statement |
| `else` | Alternative branch |
| `loop` | Loop statement |
| `bet` | Break statement |
| `clown` | Continue statement |
| `fun` | Function declaration |
| `slay` | Return statement |
| `and` | Logical AND |
| `or` | Logical OR |
| `not` | Logical NOT |

## Features

### Language Grammar
- Custom grammar defined using Pest.
- Variable and constant declarations.
- Print statements.
- Conditional statements with optional `else` blocks.
- Loop statements.
- Function declarations with parameters.
- Return, break, and continue statements.
- Assignment statements.
- Single-line comments using `//`.

### Data Types
- Numbers (`f64`)
- Strings
- Booleans (`fax`, `cap`)
- Null (`nah`)

### Expressions
The grammar supports:
- Arithmetic operators: `+`, `-`, `*`, `/`, `^`
- Comparison operators: `==`, `!=`, `<`, `>`
- Logical operators: `and`, `or`, `not`
- Unary negation: `-`
- Factorial: `!`
- Parenthesized expressions
- Operator precedence through Pest's Pratt parser

### Interpreter Components
- Grammar definition using Pest.
- Program and expression parsing.
- Expression evaluation using a Pratt parser.
- Runtime value representation.
- Runtime error definitions.

## Example Syntax

### Variables

```larp
cook username = "sulfurcodes";
cook age = 19;
locked language = "LarpScript";
```

### Output

```larp
yap("Hello, world!");
yap("Welcome to LarpScript");
```

### Conditions

```larp
if age > 18 {
    yap("You're good to go!");
} else {
    yap("Not old enough.");
}
```

### Loops

```larp
loop age < 25 {
    yap(age);
}
```

### Functions

```larp
fun greet(name) {
    yap("Hello!");
    slay;
}
```

## Built With

- **Rust** — Implementation language
- **Pest** — Parsing and grammar definition

## Project Structure

```text
LarpScript/
├── src/
│   ├── grammar.pest
│   ├── parser.rs
│   ├── evaluator.rs
│   ├── runtime.rs
│   ├── value.rs
│   ├── environment.rs
│   ├── interpreter.rs
│   ├── error.rs
│   ├── lib.rs
│   └── main.rs
├── examples/
└── tests/
```

## License

This project is licensed under the MIT License.
