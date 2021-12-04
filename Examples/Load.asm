Main:
    ldr b r1 r2 ; Load register with byte from memory address.
    ldr h r1 r2 ; Load register with half from memory address.
    ldr w r1 r2 ; Load register with word from memory address.
    ldr d r1 r2 ; Load register with double word from memory address.
    ldr q r1 r2 r3 ; Load register with quad word from memory address.

    lda b r1 0050 ; Load register with byte from memory address.
    lda h r1 0050 ; Load register with half from memory address.
    lda w r1 0050 ; Load register with word from memory address.
    lda d r1 0050 ; Load register with double word from memory address.
    lda q r1 r2 0050 ; Load register with quad word from memory address.

    ldi b r1 1 ; Load register with byte from imediate value.
    ldi h r1 1 ; Load register with half from imediate value.
    ldi w r1 1 ; Load register with word from imediate value.
    ldi d r1 1 ; Load register with double word from imediate value.
    ldi q r1 r2 1 ; Load register with quad word from imediate value.
