
pub struct InstructionInfo {
    pub label: &'static str,
    pub detail: &'static str,
    pub documentation: &'static str,
}

pub fn get_documentation() -> Vec<InstructionInfo> {
    vec![
        InstructionInfo {
            label: "A",
            detail: "A - Accumulator",
            documentation: "The **Accumulator** is an **8-bit** register used for **arithmetic, logical, I/O, and data transfer** operations. Results of ALU operations are stored here.",
        },
        InstructionInfo {
            label: "B",
            detail: "B - General purpose register",
            documentation: "**8-bit** general purpose register. Can be paired with **register C** to form the **BC register pair** for 16-bit operations.",
        },
        InstructionInfo {
            label: "C",
            detail: "C - General purpose register",
            documentation: "**8-bit** general purpose register. Can be paired with **register B** to form the **BC register pair** for 16-bit operations.",
        },
        InstructionInfo {
            label: "D",
            detail: "D - General purpose register",
            documentation: "**8-bit** general purpose register. Can be paired with **register E** to form the **DE register pair** for 16-bit operations.",
        },
        InstructionInfo {
            label: "E",
            detail: "E - General purpose register",
            documentation: "**8-bit** general purpose register. Can be paired with **register D** to form the **DE register pair** for 16-bit operations.",
        },
        InstructionInfo {
            label: "H",
            detail: "H - High register",
            documentation: "**8-bit** general purpose register. Paired with **register L** to form the **HL register pair**, which is widely used as a **memory address pointer**.",
        },
        InstructionInfo {
            label: "L",
            detail: "L - Low register",
            documentation: "**8-bit** general purpose register. Paired with **register H** to form the **HL register pair**, which is widely used as a **memory address pointer**.",
        },
        InstructionInfo {
            label: "M",
            detail: "M - Memory reference (via HL)",
            documentation: "**M** is not a physical register but refers to the **memory location** whose address is held in the **HL register pair**. Used as an operand in place of a register.",
        },
        InstructionInfo {
            label: "SP",
            detail: "SP - Stack pointer",
            documentation: "The **Stack Pointer** is a **16-bit** register that holds the **address of the top of the stack**. It is decremented on **PUSH** and incremented on **POP**.",
        },
        InstructionInfo {
            label: "PSW",
            detail: "PSW - Program status word",
            documentation: "The **Program Status Word** is a 16-bit combination of the **Accumulator (A)** and the **Flag register**. Used with **PUSH PSW** and **POP PSW** to save and restore processor state.",
        },

        InstructionInfo {
            label: "MOV",
            detail: "MOV - Move data between registers",
            documentation: "`MOV` instruction **copies** the content of the **source register** into **destination register**.",
        },
        InstructionInfo {
            label: "MVI",
            detail: "MVI - Move immediate data",
            documentation: "The **8-bit data** is stored in the **destination register** or **memory**.",
        },
        InstructionInfo {
            label: "LDA",
            detail: "LDA - Load accumulator direct",
            documentation: "The contents of a **memory location**, specified by a **16-bit address** in the operand, are copied to the **accumulator**.",
        },
        InstructionInfo {
            label: "LDAX",
            detail: "LDAX - Load accumulator indirect",
            documentation: "The contents of the **designated register pair** point to a **memory location**. This instruction **copies** the contents of that memory location into the **accumulator**.",
        },
        InstructionInfo {
            label: "LXI",
            detail: "LXI - Load register pair immediate",
            documentation: "The instruction **loads 16-bit data** in the **register pair** designated in the operand.",
        },
        InstructionInfo {
            label: "LHLD",
            detail: "LHLD - Load H and L registers direct",
            documentation: "The instruction **copies** the contents of the **memory location** pointed out by the **16-bit address** into **register L** and copies the contents of the **next memory location** into **register H**. The contents of the **source memory** are not altered.",
        },
        InstructionInfo {
            label: "STA",
            detail: "STA - Store accumulator direct",
            documentation: "The contents of the **accumulator** are copied to the **memory location** specified by the **16-bit address** in the operand.",
        },
        InstructionInfo {
            label: "STAX",
            detail: "STAX - Store accumulator indirect",
            documentation: "Stores the contents of the **accumulator** into the **memory location** pointed to by the **designated register pair**.",
        },
        InstructionInfo {
            label: "SHLD",
            detail: "SHLD - Store H and L registers direct",
            documentation: "The contents of **register L** are stored in the **memory location** specified by the **16-bit address**, and the contents of **register H** are stored in the **next memory location**.",
        },
        InstructionInfo {
            label: "XCHG",
            detail: "XCHG - Exchange H and L with D and E",
            documentation: "The contents of **register H** are exchanged with **register D**, and the contents of **register L** are exchanged with **register E**.",
        },
        InstructionInfo {
            label: "XTHL",
            detail: "XTHL - Exchange top of stack with H and L",
            documentation: "The contents of **register L** are exchanged with the **top of the stack**, and the contents of **register H** are exchanged with the **next stack location**.",
        },
        InstructionInfo {
            label: "SPHL",
            detail: "SPHL - Move HL to stack pointer",
            documentation: "The contents of the **HL register pair** are moved to the **stack pointer** register.",
        },
        InstructionInfo {
            label: "PCHL",
            detail: "PCHL - Load program counter with HL",
            documentation: "The contents of the **HL register pair** are loaded into the **program counter**. Execution continues from that address.",
        },
        InstructionInfo {
            label: "IN",
            detail: "IN - Input from port",
            documentation: "The contents of the **input port** specified by the **8-bit address** are read into the **accumulator**.",
        },
        InstructionInfo {
            label: "OUT",
            detail: "OUT - Output to port",
            documentation: "The contents of the **accumulator** are sent to the **output port** specified by the **8-bit address**.",
        },

        InstructionInfo {
            label: "PUSH",
            detail: "PUSH - Push register pair to stack",
            documentation: "The contents of the specified **register pair** are **pushed onto the stack**, decrementing the **stack pointer** by 2.",
        },
        InstructionInfo {
            label: "POP",
            detail: "POP - Pop register pair from stack",
            documentation: "Two bytes from the **stack** are **popped** and loaded into the specified **register pair**, incrementing the **stack pointer** by 2.",
        },

        InstructionInfo {
            label: "ADD",
            detail: "ADD - Add register to accumulator",
            documentation: "The contents of the specified **register** (or memory) are added to the **accumulator**. All flags are affected.",
        },
        InstructionInfo {
            label: "ADI",
            detail: "ADI - Add immediate",
            documentation: "The **8-bit immediate data** in the operand is added to the contents of the **accumulator**. All flags are affected.",
        },
        InstructionInfo {
            label: "ADC",
            detail: "ADC - Add with carry",
            documentation: "The contents of the specified **register** (or memory) and the **Carry flag** are added to the **accumulator**. All flags are affected.",
        },
        InstructionInfo {
            label: "ACI",
            detail: "ACI - Add immediate with carry",
            documentation: "The **8-bit immediate data** and the **Carry flag** are added to the contents of the **accumulator**. All flags are affected.",
        },
        InstructionInfo {
            label: "SUB",
            detail: "SUB - Subtract register from accumulator",
            documentation: "The contents of the specified **register** (or memory) are subtracted from the **accumulator**. All flags are affected.",
        },
        InstructionInfo {
            label: "SUI",
            detail: "SUI - Subtract immediate",
            documentation: "The **8-bit immediate data** is subtracted from the contents of the **accumulator**. All flags are affected.",
        },
        InstructionInfo {
            label: "SBB",
            detail: "SBB - Subtract with borrow",
            documentation: "The contents of the specified **register** (or memory) and the **Carry flag (borrow)** are subtracted from the **accumulator**. All flags are affected.",
        },
        InstructionInfo {
            label: "SBI",
            detail: "SBI - Subtract immediate with borrow",
            documentation: "The **8-bit immediate data** and the **Carry flag (borrow)** are subtracted from the **accumulator**. All flags are affected.",
        },
        InstructionInfo {
            label: "INR",
            detail: "INR - Increment register",
            documentation: "Increments the contents of the specified **register** by **1**. Flags are affected except Carry.",
        },
        InstructionInfo {
            label: "INX",
            detail: "INX - Increment register pair",
            documentation: "The contents of the specified **register pair** are incremented by **1**. No flags are affected.",
        },
        InstructionInfo {
            label: "DCR",
            detail: "DCR - Decrement register",
            documentation: "Decrements the contents of the specified **register** by **1**. Flags are affected except Carry.",
        },
        InstructionInfo {
            label: "DCX",
            detail: "DCX - Decrement register pair",
            documentation: "The contents of the specified **register pair** are decremented by **1**. No flags are affected.",
        },
        InstructionInfo {
            label: "DAD",
            detail: "DAD - Double add register pair",
            documentation: "Adds the contents of the specified **register pair** to the **HL pair**. Only the **Carry flag** is affected.",
        },
        InstructionInfo {
            label: "DAA",
            detail: "DAA - Decimal adjust accumulator",
            documentation: "Adjusts the **accumulator** to a **BCD (Binary Coded Decimal)** value after a BCD addition. All flags are affected.",
        },

        InstructionInfo {
            label: "ANA",
            detail: "ANA - AND with accumulator",
            documentation: "The contents of the specified **register** (or memory) are logically **ANDed** with the **accumulator**. Carry and Auxiliary Carry flags are reset.",
        },
        InstructionInfo {
            label: "ANI",
            detail: "ANI - AND immediate",
            documentation: "The **8-bit immediate data** is logically **ANDed** with the contents of the **accumulator**. Carry and Auxiliary Carry flags are reset.",
        },
        InstructionInfo {
            label: "ORA",
            detail: "ORA - OR with accumulator",
            documentation: "The contents of the specified **register** (or memory) are logically **ORed** with the **accumulator**. Carry and Auxiliary Carry flags are reset.",
        },
        InstructionInfo {
            label: "ORI",
            detail: "ORI - OR immediate",
            documentation: "The **8-bit immediate data** is logically **ORed** with the contents of the **accumulator**. Carry and Auxiliary Carry flags are reset.",
        },
        InstructionInfo {
            label: "XRA",
            detail: "XRA - XOR with accumulator",
            documentation: "The contents of the specified **register** (or memory) are **Exclusive-ORed** with the **accumulator**. Carry and Auxiliary Carry flags are reset.",
        },
        InstructionInfo {
            label: "XRI",
            detail: "XRI - XOR immediate",
            documentation: "The **8-bit immediate data** is **Exclusive-ORed** with the contents of the **accumulator**. Carry and Auxiliary Carry flags are reset.",
        },
        InstructionInfo {
            label: "CMP",
            detail: "CMP - Compare register with accumulator",
            documentation: "The contents of the specified **register** (or memory) are subtracted from the **accumulator** to **set flags**. The accumulator remains unchanged. Zero flag = 1 if equal; Carry flag = 1 if accumulator is less.",
        },
        InstructionInfo {
            label: "CPI",
            detail: "CPI - Compare immediate with accumulator",
            documentation: "The **8-bit immediate data** is subtracted from the **accumulator** to **set flags**. The accumulator remains unchanged. Zero flag = 1 if equal; Carry flag = 1 if accumulator is less.",
        },
        InstructionInfo {
            label: "CMA",
            detail: "CMA - Complement accumulator",
            documentation: "Each bit of the **accumulator** is **complemented** (0 becomes 1, 1 becomes 0). No flags are affected.",
        },
        InstructionInfo {
            label: "CMC",
            detail: "CMC - Complement carry flag",
            documentation: "The **Carry flag** is **complemented**. If Carry = 1, it becomes 0; if Carry = 0, it becomes 1.",
        },
        InstructionInfo {
            label: "STC",
            detail: "STC - Set carry flag",
            documentation: "The **Carry flag** is **set to 1**. No other flags are affected.",
        },

        InstructionInfo {
            label: "RLC",
            detail: "RLC - Rotate accumulator left",
            documentation: "Each bit of the **accumulator** is **rotated left** by one position. Bit 7 is moved to Bit 0 and also copied to the **Carry flag**.",
        },
        InstructionInfo {
            label: "RRC",
            detail: "RRC - Rotate accumulator right",
            documentation: "Each bit of the **accumulator** is **rotated right** by one position. Bit 0 is moved to Bit 7 and also copied to the **Carry flag**.",
        },
        InstructionInfo {
            label: "RAL",
            detail: "RAL - Rotate accumulator left through carry",
            documentation: "Each bit of the **accumulator** is **rotated left** through the **Carry flag**. Bit 7 moves to Carry, and the old Carry moves to Bit 0.",
        },
        InstructionInfo {
            label: "RAR",
            detail: "RAR - Rotate accumulator right through carry",
            documentation: "Each bit of the **accumulator** is **rotated right** through the **Carry flag**. Bit 0 moves to Carry, and the old Carry moves to Bit 7.",
        },

        InstructionInfo {
            label: "JMP",
            detail: "JMP - Unconditional jump",
            documentation: "Program execution **jumps** to the specified **16-bit address** unconditionally.",
        },
        InstructionInfo {
            label: "JC",
            detail: "JC - Jump if carry",
            documentation: "Jumps to the given address **if the Carry flag = 1**.",
        },
        InstructionInfo {
            label: "JNC",
            detail: "JNC - Jump if no carry",
            documentation: "Jumps to the given address **if the Carry flag = 0**.",
        },
        InstructionInfo {
            label: "JZ",
            detail: "JZ - Jump if zero",
            documentation: "Jumps to the given address **if the Zero flag = 1**.",
        },
        InstructionInfo {
            label: "JNZ",
            detail: "JNZ - Jump if not zero",
            documentation: "Jumps to the given address **if the Zero flag = 0**.",
        },
        InstructionInfo {
            label: "JM",
            detail: "JM - Jump if minus",
            documentation: "Jumps to the given address **if the Sign flag = 1** (result was negative).",
        },
        InstructionInfo {
            label: "JP",
            detail: "JP - Jump if positive",
            documentation: "Jumps to the given address **if the Sign flag = 0** (result was positive).",
        },
        InstructionInfo {
            label: "JPE",
            detail: "JPE - Jump if parity even",
            documentation: "Jumps to the given address **if the Parity flag = 1** (even number of 1-bits in result).",
        },
        InstructionInfo {
            label: "JPO",
            detail: "JPO - Jump if parity odd",
            documentation: "Jumps to the given address **if the Parity flag = 0** (odd number of 1-bits in result).",
        },

        InstructionInfo {
            label: "CALL",
            detail: "CALL - Unconditional subroutine call",
            documentation: "The **current program counter** is pushed onto the **stack** and execution jumps to the specified **16-bit address**. Used to call subroutines.",
        },
        InstructionInfo {
            label: "CC",
            detail: "CC - Call if carry",
            documentation: "Calls the subroutine at the given address **if the Carry flag = 1**.",
        },
        InstructionInfo {
            label: "CNC",
            detail: "CNC - Call if no carry",
            documentation: "Calls the subroutine at the given address **if the Carry flag = 0**.",
        },
        InstructionInfo {
            label: "CZ",
            detail: "CZ - Call if zero",
            documentation: "Calls the subroutine at the given address **if the Zero flag = 1**.",
        },
        InstructionInfo {
            label: "CNZ",
            detail: "CNZ - Call if not zero",
            documentation: "Calls the subroutine at the given address **if the Zero flag = 0**.",
        },
        InstructionInfo {
            label: "CM",
            detail: "CM - Call if minus",
            documentation: "Calls the subroutine at the given address **if the Sign flag = 1**.",
        },
        InstructionInfo {
            label: "CP",
            detail: "CP - Call if positive",
            documentation: "Calls the subroutine at the given address **if the Sign flag = 0**.",
        },
        InstructionInfo {
            label: "CPE",
            detail: "CPE - Call if parity even",
            documentation: "Calls the subroutine at the given address **if the Parity flag = 1**.",
        },
        InstructionInfo {
            label: "CPO",
            detail: "CPO - Call if parity odd",
            documentation: "Calls the subroutine at the given address **if the Parity flag = 0**.",
        },

        InstructionInfo {
            label: "RET",
            detail: "RET - Unconditional return from subroutine",
            documentation: "The **program counter** is restored from the **stack**, returning execution to the instruction after the calling **CALL**.",
        },
        InstructionInfo {
            label: "RC",
            detail: "RC - Return if carry",
            documentation: "Returns from subroutine **if the Carry flag = 1**.",
        },
        InstructionInfo {
            label: "RNC",
            detail: "RNC - Return if no carry",
            documentation: "Returns from subroutine **if the Carry flag = 0**.",
        },
        InstructionInfo {
            label: "RZ",
            detail: "RZ - Return if zero",
            documentation: "Returns from subroutine **if the Zero flag = 1**.",
        },
        InstructionInfo {
            label: "RNZ",
            detail: "RNZ - Return if not zero",
            documentation: "Returns from subroutine **if the Zero flag = 0**.",
        },
        InstructionInfo {
            label: "RM",
            detail: "RM - Return if minus",
            documentation: "Returns from subroutine **if the Sign flag = 1**.",
        },
        InstructionInfo {
            label: "RP",
            detail: "RP - Return if positive",
            documentation: "Returns from subroutine **if the Sign flag = 0**.",
        },
        InstructionInfo {
            label: "RPE",
            detail: "RPE - Return if parity even",
            documentation: "Returns from subroutine **if the Parity flag = 1**.",
        },
        InstructionInfo {
            label: "RPO",
            detail: "RPO - Return if parity odd",
            documentation: "Returns from subroutine **if the Parity flag = 0**.",
        },

        InstructionInfo {
            label: "RST",
            detail: "RST - Restart (software interrupt)",
            documentation: "The **program counter** is pushed onto the **stack** and execution jumps to one of **8 fixed restart addresses** (0x00 to 0x38). Acts as a 1-byte **CALL** instruction.",
        },

        InstructionInfo {
            label: "NOP",
            detail: "NOP - No operation",
            documentation: "No operation is performed. The **program counter** is incremented and execution continues. Used for **timing delays** or placeholder purposes.",
        },
        InstructionInfo {
            label: "HLT",
            detail: "HLT - Halt",
            documentation: "The processor **stops execution** and enters a **halt state**. Execution can only resume upon an **interrupt** or **reset**.",
        },
        InstructionInfo {
            label: "DI",
            detail: "DI - Disable interrupts",
            documentation: "The **Interrupt Enable flip-flop** is reset, **disabling all maskable interrupts** (INTR, RST 5.5, 6.5, 7.5). TRAP is not affected.",
        },
        InstructionInfo {
            label: "EI",
            detail: "EI - Enable interrupts",
            documentation: "The **Interrupt Enable flip-flop** is set, **enabling all maskable interrupts**. The effect takes place after the **next instruction** is executed.",
        },
        InstructionInfo {
            label: "RIM",
            detail: "RIM - Read interrupt mask",
            documentation: "Reads the **interrupt mask** and **serial input data** into the **accumulator**. Bits reflect the mask status of RST 5.5, 6.5, 7.5 and any pending interrupts.",
        },
        InstructionInfo {
            label: "SIM",
            detail: "SIM - Set interrupt mask",
            documentation: "Sets the **interrupt mask** for RST 5.5, 6.5, and 7.5 and can output **serial data** using the accumulator contents as a control word.",
        },
    ]
}
