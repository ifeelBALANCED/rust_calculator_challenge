# Build Your Own Calculator Challenge

## Overview

Welcome to the "Build Your Own Calculator" challenge! This project is an opportunity to create a calculator that can parse and compute mathematical expressions. You can choose to build it as a command-line tool, a desktop application, or a web-based application. By completing this challenge, you will get hands-on experience with parsing algorithms and the stack data structure in a real-world application.

## Getting Started

### Step Zero: Setup

Before diving into the code, set up your development environment:

- Choose your target platform (CLI, desktop, or web).
- Select your IDE/editor.
- Pick a programming language.

Prepare your environment to begin developing and testing your solution.

### Step 1: Handling Simple Expressions

Your first goal is to parse and compute four basic arithmetic operations:

- Addition (`1 + 2`)
- Subtraction (`2 - 1`)
- Multiplication (`2 * 3`)
- Division (`3 / 2`)

Ensure your calculator can handle these inputs and return the correct answer, accounting for both integer and floating-point operations. Be mindful of potential invalid inputs.

**Testing Tip:** When testing multiplication in the shell, quote the expression to prevent character expansion (e.g., `calc '2 * 3'` should return `6`).

#### Development Choice

Before proceeding, decide whether to start coding for Step 1 immediately, potentially using test-driven development (TDD), or to read through all the steps first, which might influence your approach. Choose your adventure!

### Step 2: Handling Complex Expressions

Expand your calculator's capabilities to parse and compute more complex expressions involving precedence and parentheses, such as `1 + 1 * 5` and `(1 + 1) * 5`.

#### Implementing the Shunting Yard Algorithm

A powerful method for dealing with complex expressions is converting infix notation to postfix notation (reverse polish notation). This conversion simplifies the computation of expressions with precedence and parentheses.

- **Basic Example:** `(1 * 2) - (3 * 4)` in postfix notation is `1 2 * 3 4 * -`.
- Use the shunting yard algorithm for parsing expressions into postfix notation, leveraging a stack for efficient computation.

### Step 3: Advanced Expression Handling

Finally, enhance your calculator to handle even more complex expressions. This step encourages creativity in generating test cases and implementing features such as:

- Brackets for precedence.
- Advanced mathematical functions like `sin`, `cos`, and `tan`.

#### Automated Testing

Develop automated tests for your calculator to ensure accuracy and reliability. Test a variety of expressions to cover basic and advanced scenarios.

## Conclusion

Completing this challenge will deepen your understanding of parsing algorithms, the stack data structure, and test-driven development. It's a valuable opportunity to apply theoretical knowledge to a practical project. Good luck, and have fun coding your own calculator!
