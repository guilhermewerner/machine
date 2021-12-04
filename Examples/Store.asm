Main:
    str b r1 r2 ; Store register byte in memory address.
    str h r1 r2 ; Store register half in memory address.
    str w r1 r2 ; Store register word in memory address.
    str d r1 r2 ; Store register double in memory address.
    str q r1 r2 r3 ; Store register quad in memory address.

    sta b 0050 r1 ; Store register byte in memory address.
    sta h 0050 r1 ; Store register half in memory address.
    sta w 0050 r1 ; Store register word in memory address.
    sta d 0050 r1 ; Store register double in memory address.
    sta q 0050 r1 r2 ; Store register quad in memory address.

    sti b 0050 1 ; Store imediate byte in memory address.
    sti h 0050 1 ; Store imediate half in memory address.
    sti w 0050 1 ; Store imediate word in memory address.
    sti d 0050 1 ; Store imediate double in memory address.
    sti q 0050 1 ; Store imediate quad in memory address.
