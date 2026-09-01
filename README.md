# lsp85

`lsp85` is a standalone Language Server Protocol (LSP) implementation for Intel 8085 Assembly, supporting native IDEs and WebAssembly (WASM) environments.

## Features

- **Code Completion**: Contextual auto-completion for 8085 instruction opcodes, registers (`A`, `B`, `C`, `D`, `E`, `H`, `L`, `M`, `SP`, `PSW`), and user-defined assembly labels.
- **Hover Information**: Detailed documentation for instructions, register roles, label locations, and multi-base immediate value conversions (Hexadecimal, Binary, Decimal).
- **Diagnostics**: Real-time error reporting for incorrect instruction operand counts, duplicate label definitions, and undefined label references.
- **Go to Definition**: Jump directly to label definitions.
- **Document Symbols**: Symbol outline for assembly labels.
- **Signature Help**: Active opcode signature assistance while typing operands.
- **WASM & Native Support**: Runs via stdio for native IDEs or via WebAssembly bindings (`wasm_handle_message`) for in-browser editors.

## Screenshots

### Code Completions
<img width="1404" height="325" alt="image" src="https://github.com/user-attachments/assets/68892087-4392-4610-8b7b-7c574700e58e" />

### Hover Information
<img width="1711" height="96" alt="2026-04-23_17-53" src="https://github.com/user-attachments/assets/a109a620-37fd-4e34-b811-3305d877b010" />

## LLM Assistance

LLMs were utilized for:
- Implementing the WebAssembly (WASM) bindings layer for browser integration.
- Generating and structuring this `README.md` documentation.

## License

MIT
