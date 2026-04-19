use lsp_types::{CompletionItem, CompletionItemKind, Documentation, MarkupContent, MarkupKind};

pub fn get_completion_items() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "MOV".to_string(),
            detail: Some("MOV - Move data between registers".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "`MOV` instruction **copies** the content of the **source register** into **destination register**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },
        CompletionItem {
            label: "MVI".to_string(),
            detail: Some("MVI - Move immediate data".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The **8-bit data** is stored in the **destination register** of **memory**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },
        CompletionItem {
            label: "LDA".to_string(),
            detail: Some("LDA - Load accumulator direct".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The contents of a **memory location**, specified by a **16-bit address** in the operand, are copied to the **accumulator**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },
        CompletionItem {
            label: "LDAX".to_string(),
            detail: Some("LDAX - Load accumulator indirect".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The contents of the **designated register pair** point to a **memory location**. This instruction **copies** the contents of that memory location into the **accumulator**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },
        CompletionItem {
            label: "LXI".to_string(),
            detail: Some("LXI - Load register pair immediate".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The instruction **loads 16-bit data** in the **register pair** designated in the operand.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },
        CompletionItem {
            label: "LHLD".to_string(),
            detail: Some("LHLD - Load H and L registers direct".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The instruction **copies** the contents of the **memory location** pointed out by the **16-bit address** into **register L** and copies the contents of the **next memory location** into **register H**. The contents of the **source memory** are not altered.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },
        CompletionItem {
            label: "SUB".to_string(),
            detail: Some("SUB - Subtract".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "**Subtract** instruction".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },
        CompletionItem {
            label: "ADD".to_string(),
            detail: Some("ADD - Add".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "**Add** values".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },
        CompletionItem {
            label: "STAX".to_string(),
            detail: Some("STAX - Store accumulator indirect".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Stores the contents of the **accumulator** into the **memory location** pointed to by the **designated register pair**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "PUSH".to_string(),
            detail: Some("PUSH - Push register pair to stack".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The contents of the specified **register pair** are **pushed onto the stack**, decrementing the **stack pointer** by 2.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "POP".to_string(),
            detail: Some("POP - Pop register pair from stack".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Two bytes from the **stack** are **popped** and loaded into the specified **register pair**, incrementing the **stack pointer** by 2.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "INR".to_string(),
            detail: Some("INR - Increment register".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Increments the contents of the specified **register** by **1**. Flags are affected except Carry.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "DCR".to_string(),
            detail: Some("DCR - Decrement register".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Decrements the contents of the specified **register** by **1**. Flags are affected except Carry.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "DAD".to_string(),
            detail: Some("DAD - Double add register pair".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Adds the contents of the specified **register pair** to the **HL pair**. Only the **Carry flag** is affected.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        // --- JMP Variants ---
        CompletionItem {
            label: "JMP".to_string(),
            detail: Some("JMP - Unconditional jump".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Program execution **jumps** to the specified **16-bit address** unconditionally.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "JC".to_string(),
            detail: Some("JC - Jump if carry".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Jumps to the given address **if the Carry flag = 1**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "JNC".to_string(),
            detail: Some("JNC - Jump if no carry".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Jumps to the given address **if the Carry flag = 0**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "JZ".to_string(),
            detail: Some("JZ - Jump if zero".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Jumps to the given address **if the Zero flag = 1**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "JNZ".to_string(),
            detail: Some("JNZ - Jump if not zero".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Jumps to the given address **if the Zero flag = 0**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },
        // --- Data Transfer ---
        CompletionItem {
            label: "STA".to_string(),
            detail: Some("STA - Store accumulator direct".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The contents of the **accumulator** are copied to the **memory location** specified by the **16-bit address** in the operand.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "SHLD".to_string(),
            detail: Some("SHLD - Store H and L registers direct".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The contents of **register L** are stored in the **memory location** specified by the **16-bit address**, and the contents of **register H** are stored in the **next memory location**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "XCHG".to_string(),
            detail: Some("XCHG - Exchange H and L with D and E".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The contents of **register H** are exchanged with **register D**, and the contents of **register L** are exchanged with **register E**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "XTHL".to_string(),
            detail: Some("XTHL - Exchange top of stack with H and L".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The contents of **register L** are exchanged with the **top of the stack**, and the contents of **register H** are exchanged with the **next stack location**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "SPHL".to_string(),
            detail: Some("SPHL - Move HL to stack pointer".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The contents of the **HL register pair** are moved to the **stack pointer** register.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "PCHL".to_string(),
            detail: Some("PCHL - Load program counter with HL".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The contents of the **HL register pair** are loaded into the **program counter**. Execution continues from that address.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "IN".to_string(),
            detail: Some("IN - Input from port".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The contents of the **input port** specified by the **8-bit address** are read into the **accumulator**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "OUT".to_string(),
            detail: Some("OUT - Output to port".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The contents of the **accumulator** are sent to the **output port** specified by the **8-bit address**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        // --- Arithmetic ---
        CompletionItem {
            label: "ADI".to_string(),
            detail: Some("ADI - Add immediate".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The **8-bit immediate data** in the operand is added to the contents of the **accumulator**. All flags are affected.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "ADC".to_string(),
            detail: Some("ADC - Add with carry".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The contents of the specified **register** (or memory) and the **Carry flag** are added to the **accumulator**. All flags are affected.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "ACI".to_string(),
            detail: Some("ACI - Add immediate with carry".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The **8-bit immediate data** and the **Carry flag** are added to the contents of the **accumulator**. All flags are affected.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "SUI".to_string(),
            detail: Some("SUI - Subtract immediate".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The **8-bit immediate data** is subtracted from the contents of the **accumulator**. All flags are affected.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "SBB".to_string(),
            detail: Some("SBB - Subtract with borrow".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The contents of the specified **register** (or memory) and the **Carry flag (borrow)** are subtracted from the **accumulator**. All flags are affected.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "SBI".to_string(),
            detail: Some("SBI - Subtract immediate with borrow".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The **8-bit immediate data** and the **Carry flag (borrow)** are subtracted from the **accumulator**. All flags are affected.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "INX".to_string(),
            detail: Some("INX - Increment register pair".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The contents of the specified **register pair** are incremented by **1**. No flags are affected.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "DCX".to_string(),
            detail: Some("DCX - Decrement register pair".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The contents of the specified **register pair** are decremented by **1**. No flags are affected.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "DAA".to_string(),
            detail: Some("DAA - Decimal adjust accumulator".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Adjusts the **accumulator** to a **BCD (Binary Coded Decimal)** value after a BCD addition. All flags are affected.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        // --- Logical ---
        CompletionItem {
            label: "ANA".to_string(),
            detail: Some("ANA - AND with accumulator".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The contents of the specified **register** (or memory) are logically **ANDed** with the **accumulator**. Carry and Auxiliary Carry flags are reset.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "ANI".to_string(),
            detail: Some("ANI - AND immediate".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The **8-bit immediate data** is logically **ANDed** with the contents of the **accumulator**. Carry and Auxiliary Carry flags are reset.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "ORA".to_string(),
            detail: Some("ORA - OR with accumulator".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The contents of the specified **register** (or memory) are logically **ORed** with the **accumulator**. Carry and Auxiliary Carry flags are reset.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "ORI".to_string(),
            detail: Some("ORI - OR immediate".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The **8-bit immediate data** is logically **ORed** with the contents of the **accumulator**. Carry and Auxiliary Carry flags are reset.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "XRA".to_string(),
            detail: Some("XRA - XOR with accumulator".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The contents of the specified **register** (or memory) are **Exclusive-ORed** with the **accumulator**. Carry and Auxiliary Carry flags are reset.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "XRI".to_string(),
            detail: Some("XRI - XOR immediate".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The **8-bit immediate data** is **Exclusive-ORed** with the contents of the **accumulator**. Carry and Auxiliary Carry flags are reset.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "CMP".to_string(),
            detail: Some("CMP - Compare register with accumulator".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The contents of the specified **register** (or memory) are subtracted from the **accumulator** to **set flags**. The accumulator remains unchanged. Zero flag = 1 if equal; Carry flag = 1 if accumulator is less.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "CPI".to_string(),
            detail: Some("CPI - Compare immediate with accumulator".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The **8-bit immediate data** is subtracted from the **accumulator** to **set flags**. The accumulator remains unchanged. Zero flag = 1 if equal; Carry flag = 1 if accumulator is less.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "CMA".to_string(),
            detail: Some("CMA - Complement accumulator".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Each bit of the **accumulator** is **complemented** (0 becomes 1, 1 becomes 0). No flags are affected.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "CMC".to_string(),
            detail: Some("CMC - Complement carry flag".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The **Carry flag** is **complemented**. If Carry = 1, it becomes 0; if Carry = 0, it becomes 1.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "STC".to_string(),
            detail: Some("STC - Set carry flag".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The **Carry flag** is **set to 1**. No other flags are affected.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        // --- Rotate ---
        CompletionItem {
            label: "RLC".to_string(),
            detail: Some("RLC - Rotate accumulator left".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Each bit of the **accumulator** is **rotated left** by one position. Bit 7 is moved to Bit 0 and also copied to the **Carry flag**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "RRC".to_string(),
            detail: Some("RRC - Rotate accumulator right".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Each bit of the **accumulator** is **rotated right** by one position. Bit 0 is moved to Bit 7 and also copied to the **Carry flag**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "RAL".to_string(),
            detail: Some("RAL - Rotate accumulator left through carry".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Each bit of the **accumulator** is **rotated left** through the **Carry flag**. Bit 7 moves to Carry, and the old Carry moves to Bit 0.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "RAR".to_string(),
            detail: Some("RAR - Rotate accumulator right through carry".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Each bit of the **accumulator** is **rotated right** through the **Carry flag**. Bit 0 moves to Carry, and the old Carry moves to Bit 7.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        // --- Branch: Jump ---
        CompletionItem {
            label: "JM".to_string(),
            detail: Some("JM - Jump if minus".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Jumps to the given address **if the Sign flag = 1** (result was negative).".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "JP".to_string(),
            detail: Some("JP - Jump if positive".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Jumps to the given address **if the Sign flag = 0** (result was positive).".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "JPE".to_string(),
            detail: Some("JPE - Jump if parity even".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Jumps to the given address **if the Parity flag = 1** (even number of 1-bits in result).".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "JPO".to_string(),
            detail: Some("JPO - Jump if parity odd".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Jumps to the given address **if the Parity flag = 0** (odd number of 1-bits in result).".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        // --- Branch: Call ---
        CompletionItem {
            label: "CALL".to_string(),
            detail: Some("CALL - Unconditional subroutine call".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The **current program counter** is pushed onto the **stack** and execution jumps to the specified **16-bit address**. Used to call subroutines.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "CC".to_string(),
            detail: Some("CC - Call if carry".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Calls the subroutine at the given address **if the Carry flag = 1**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "CNC".to_string(),
            detail: Some("CNC - Call if no carry".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Calls the subroutine at the given address **if the Carry flag = 0**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "CZ".to_string(),
            detail: Some("CZ - Call if zero".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Calls the subroutine at the given address **if the Zero flag = 1**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "CNZ".to_string(),
            detail: Some("CNZ - Call if not zero".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Calls the subroutine at the given address **if the Zero flag = 0**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "CM".to_string(),
            detail: Some("CM - Call if minus".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Calls the subroutine at the given address **if the Sign flag = 1**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "CP".to_string(),
            detail: Some("CP - Call if positive".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Calls the subroutine at the given address **if the Sign flag = 0**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "CPE".to_string(),
            detail: Some("CPE - Call if parity even".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Calls the subroutine at the given address **if the Parity flag = 1**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "CPO".to_string(),
            detail: Some("CPO - Call if parity odd".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Calls the subroutine at the given address **if the Parity flag = 0**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        // --- Branch: Return ---
        CompletionItem {
            label: "RET".to_string(),
            detail: Some("RET - Unconditional return from subroutine".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The **program counter** is restored from the **stack**, returning execution to the instruction after the calling **CALL**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "RC".to_string(),
            detail: Some("RC - Return if carry".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Returns from subroutine **if the Carry flag = 1**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "RNC".to_string(),
            detail: Some("RNC - Return if no carry".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Returns from subroutine **if the Carry flag = 0**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "RZ".to_string(),
            detail: Some("RZ - Return if zero".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Returns from subroutine **if the Zero flag = 1**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "RNZ".to_string(),
            detail: Some("RNZ - Return if not zero".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Returns from subroutine **if the Zero flag = 0**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "RM".to_string(),
            detail: Some("RM - Return if minus".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Returns from subroutine **if the Sign flag = 1**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "RP".to_string(),
            detail: Some("RP - Return if positive".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Returns from subroutine **if the Sign flag = 0**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "RPE".to_string(),
            detail: Some("RPE - Return if parity even".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Returns from subroutine **if the Parity flag = 1**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "RPO".to_string(),
            detail: Some("RPO - Return if parity odd".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Returns from subroutine **if the Parity flag = 0**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        // --- Restart ---
        CompletionItem {
            label: "RST".to_string(),
            detail: Some("RST - Restart (software interrupt)".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The **program counter** is pushed onto the **stack** and execution jumps to one of **8 fixed restart addresses** (0x00 to 0x38). Acts as a 1-byte **CALL** instruction.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        // --- Machine Control ---
        CompletionItem {
            label: "NOP".to_string(),
            detail: Some("NOP - No operation".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "No operation is performed. The **program counter** is incremented and execution continues. Used for **timing delays** or placeholder purposes.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "HLT".to_string(),
            detail: Some("HLT - Halt".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The processor **stops execution** and enters a **halt state**. Execution can only resume upon an **interrupt** or **reset**.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "DI".to_string(),
            detail: Some("DI - Disable interrupts".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The **Interrupt Enable flip-flop** is reset, **disabling all maskable interrupts** (INTR, RST 5.5, 6.5, 7.5). TRAP is not affected.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "EI".to_string(),
            detail: Some("EI - Enable interrupts".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "The **Interrupt Enable flip-flop** is set, **enabling all maskable interrupts**. The effect takes place after the **next instruction** is executed.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "RIM".to_string(),
            detail: Some("RIM - Read interrupt mask".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Reads the **interrupt mask** and **serial input data** into the **accumulator**. Bits reflect the mask status of RST 5.5, 6.5, 7.5 and any pending interrupts.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },

        CompletionItem {
            label: "SIM".to_string(),
            detail: Some("SIM - Set interrupt mask".to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Sets the **interrupt mask** for RST 5.5, 6.5, and 7.5 and can output **serial data** using the accumulator contents as a control word.".to_string(),
            })),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        },
    ]
}
