# 0.0.1
Changes since the `0.0.0` release.

- **asmkit-x86_64**:
    - Fixes:
        - optimize `InstructionStream` implementation by converting slice to vec instead of iterating and collecting its values.
    - Intrinsics:
        - add: `intrinsic_push_rbp` intrinsic.
    - Mnemonics:
        - `ret`:
            - change!: `ret` mnemonic to `ret_near` mnemonic.
            - add: `ret_far` mnemonic.
            - add: `ret_near_imm16` mnemonic.
            - add: `ret_far_imm16` mnemonic.
        - `push`:
            - add: `push_reg16` mnemonic.
            - add: `push_reg64` mnemonic.
            - add: `push_imm8` mnemonic.
            - add: `push_imm16` mnemonic.
            - add: `push_imm32` mnemonic.
            - add: `push_fs` mnemonic.
            - add: `push_gs` mnemonic.