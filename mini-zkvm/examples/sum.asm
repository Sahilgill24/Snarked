; compute (2 + 3) * 4 and leave the result in r0
MOV  r1, 2
MOV  r2, 3
ADD  r0, r1, r2   ; r0 = 5
MOV  r3, 4
MUL  r0, r0, r3   ; r0 = 20
STORE r0, 64      ; write r0 to memory[64]
LOAD r4, 64       ; read it back into r4
