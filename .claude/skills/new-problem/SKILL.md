---
name: new-problem
description: Scaffold a new LeetCode problem from the web. Usage: /new-problem <number> <Problem Name> (e.g., /new-problem 1 Two Sum)
allowed-tools: Bash, Read, Write, Glob, Grep, WebSearch
---

# Scaffold a New LeetCode Problem

The user has invoked `/new-problem <number> <Problem Name>`. Parse the arguments to get the number and problem name.

## Step 1: Derive titleSlug
Convert the problem name to lowercase with spaces replaced by hyphens.
Example: `Two Sum` → `two-sum`

## Step 2: Obtain the Rust function signature and example test cases

**Do NOT use the LeetCode API or curl.** Instead:

1. **Use built-in knowledge first** — if the problem number/name is a well-known LeetCode problem, directly write the correct Rust function signature and example inputs/outputs from memory.
2. **If unsure, search the web** — use the WebSearch tool to look up the problem's Rust signature and examples (e.g., search `leetcode <number> <name> rust solution signature`). Extract:
   - The function name and parameter types (for the signature)
   - Example inputs and expected outputs (for the tests)

## Step 5: Create the crate
```bash
cargo new No<number> --vcs none
```
(Workspace `Cargo.toml` uses `members = ["*"]`, so no further changes needed.)

## Step 6: Write `No<number>/src/main.rs`

Rules:
- Copy Rust snippet from LeetCode verbatim, but **prefix every parameter with `_`** (e.g. `nums` → `_nums`)
- Keep function body as **`todo!()`** — do not implement the solution
- Add `#[allow(dead_code)]` on both `struct Solution` and `impl Solution`
- Tests use real example inputs/expected outputs from `exampleTestcaseList`, but mark each `#[ignore]` so CI passes before the solution is filled in

```rust
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn example_fn(_param1: Type1, _param2: Type2) -> ReturnType {
        todo!()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_example1() {
        // input / expected from LeetCode example 1
        assert_eq!(Solution::example_fn(...), expected);
    }

    #[test]
    #[ignore]
    fn test_example2() {
        // input / expected from LeetCode example 2
        assert_eq!(Solution::example_fn(...), expected);
    }
}
```

## Step 7: Verify
```bash
cargo build -p No<number>
cargo clippy -p No<number> --all-targets -- -D warnings
cargo fmt -p No<number>
```
Fix any remaining warnings before finishing.
