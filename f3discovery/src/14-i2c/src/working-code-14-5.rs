#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux14::{entry, iprint, iprintln, prelude::*};

// Slave address (7-bit: 0x1E → shifted left becomes 0x3C)
const MAGNETOMETER: u16 = 0b0011_1100;

// Magnetometer output register base address
const OUT_X_L_M: u8 = 0x68;

#[entry]
fn main() -> ! {
    let (i2c1, mut delay, mut itm) = aux14::init();

    loop {
        // === Stage 1: Write the starting register address (with auto-increment) ===
        i2c1.cr2.write(|w| {
            w.start().set_bit();
            w.sadd().bits(MAGNETOMETER);
            w.rd_wrn().clear_bit(); // write mode
            w.nbytes().bits(1);
            w.autoend().clear_bit()
        });

        while i2c1.isr.read().txis().bit_is_clear() {}

        // Send register address with MSB set to enable auto-increment
        i2c1.txdr.write(|w| w.txdata().bits(OUT_X_L_M | 0x80));

        while i2c1.isr.read().tc().bit_is_clear() {}

        // === Stage 2: Read 6 bytes: X_L, X_H, Y_L, Y_H, Z_L, Z_H ===
        i2c1.cr2.modify(|_, w| {
            w.start().set_bit();
            w.sadd().bits(MAGNETOMETER);
            w.rd_wrn().set_bit(); // read mode
            w.nbytes().bits(6);
            w.autoend().set_bit()
        });

        let mut buffer = [0u8; 6];
        for byte in &mut buffer {
            while i2c1.isr.read().rxne().bit_is_clear() {}
            *byte = i2c1.rxdr.read().rxdata().bits();
        }

        // Convert to signed 16-bit values
        let x = i16::from_le_bytes([buffer[0], buffer[1]]);
        let y = i16::from_le_bytes([buffer[2], buffer[3]]);
        let z = i16::from_le_bytes([buffer[4], buffer[5]]);

        iprintln!(&mut itm.stim[0], "Mag X: {}, Y: {}, Z: {}", x, y, z);

        delay.delay_ms(1_000_u16);
    }
}
