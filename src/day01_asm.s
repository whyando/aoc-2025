.text
.globl day01_solve_asm
.type day01_solve_asm, @function

# void day01_solve_asm(const unsigned char *ptr, unsigned long len,
#                       int *out_part1, int *out_part2);
# System V x86_64 ABI
day01_solve_asm:
    # Prologue
    push    %rbp
    mov     %rsp, %rbp
    push    %rbx
    push    %r12
    push    %r13
    push    %r14
    push    %r15

    # Save callee-needed arguments
    # rdi: input pointer (kept in rdi)
    # rsi: remaining length (kept in rsi)
    mov     %rdx, %r13        # r13 = out_part1 pointer
    mov     %rcx, %r14        # r14 = out_part2 pointer

    # Initialize state: x = 100_050, part1 = 0, part2 = 0.
    # The +100_000 offset keeps x positive while preserving /100 and %100 results.
    mov     $100050, %r8d     # x
    xor     %r9d, %r9d        # part1
    xor     %r10d, %r10d      # part2

# Main loop over lines
.L_loop_start:
    cmp     $0, %rsi
    je      .L_done

    # Load direction character: 'L' or 'R'
    movzbq  (%rdi), %rax      # al = *ptr
    inc     %rdi
    dec     %rsi

    # dir = -1 for 'L', +1 for 'R'
    cmp     $'L', %al
    je      .L_dir_left
    cmp     $'R', %al
    je      .L_dir_right
    # Fallback: treat anything else as +1
.L_dir_right:
    mov     $1, %r15d
    jmp     .L_parse_steps
.L_dir_left:
    mov     $-1, %r15d

# Parse decimal steps until '\n' or end of input
.L_parse_steps:
    xor     %ebx, %ebx        # steps accumulator in ebx

.L_parse_digit:
    cmp     $0, %rsi
    je      .L_after_line

    movzbq  (%rdi), %rax
    cmp     $'\n', %al
    je      .L_end_line

    # steps = steps * 10 + (ch - '0')
    imull   $10, %ebx, %ebx
    mov     %eax, %edx
    sub     $'0', %edx
    add     %edx, %ebx

    inc     %rdi
    dec     %rsi
    jmp     .L_parse_digit

.L_end_line:
    # consume '\n'
    inc     %rdi
    dec     %rsi

.L_after_line:
    # At this point:
    #   dir  in r15d
    #   steps in ebx
    #   x    in r8d

    # x1 = x + steps * dir
    mov     %ebx, %eax
    imul    %r15d, %eax       # eax = steps * dir
    add     %r8d, %eax        # eax = x1
    mov     %eax, %r11d       # r11d = x1

    # Compute passes
    # if dir == -1:
    #   passes = ((x - 1).div_euclid(100) - (x1 - 1).div_euclid(100)).abs()
    # else:
    #   passes = (x1.div_euclid(100) - x.div_euclid(100)).abs()

    cmp     $-1, %r15d
    jne     .L_dir_positive

    # Negative direction branch: use (x - 1) and (x1 - 1)
    # q1 = (x - 1) / 100
    mov     %r8d, %eax
    sub     $1, %eax
    cdq                     # edx:eax = sign-extended eax
    mov     $100, %ecx
    idiv    %ecx            # eax = x / 100
    mov     %eax, %r12d     # r12d = q1

    # q2 = (x1 - 1) / 100
    mov     %r11d, %eax
    sub     $1, %eax
    cdq
    mov     $100, %ecx
    idiv    %ecx
    sub     %eax, %r12d     # r12d = q1 - q2
    jmp     .L_abs_passes

.L_dir_positive:
    # Positive direction branch: use x and x1 directly
    # q1 = x1 / 100
    mov     %r11d, %eax
    cdq
    mov     $100, %ecx
    idiv    %ecx
    mov     %eax, %r12d     # r12d = q1

    # q2 = x / 100
    mov     %r8d, %eax
    cdq
    mov     $100, %ecx
    idiv    %ecx
    sub     %eax, %r12d     # r12d = q1 - q2

# passes = abs(r12d)
.L_abs_passes:
    mov     %r12d, %eax
    cmp     $0, %eax
    jge     .L_abs_done
    neg     %eax
.L_abs_done:
    # passes now in eax, save it to r12d
    mov     %eax, %r12d

    # if x % 100 == 0 then part1 += 1
    mov     %r8d, %eax
    cdq
    mov     $100, %ecx
    idiv    %ecx            # edx = x % 100
    cmp     $0, %edx
    jne     .L_skip_part1_inc
    inc     %r9d
.L_skip_part1_inc:

    # part2 += passes (passes is in r12d)
    add     %r12d, %r10d

    # x = x1
    mov     %r11d, %r8d

    jmp     .L_loop_start

.L_done:
    # Store results
    mov     %r9d, (%r13)    # *out_part1 = part1
    mov     %r10d, (%r14)   # *out_part2 = part2

    # Epilogue
    pop     %r15
    pop     %r14
    pop     %r13
    pop     %r12
    pop     %rbx
    pop     %rbp
    ret

.size day01_solve_asm, .-day01_solve_asm


