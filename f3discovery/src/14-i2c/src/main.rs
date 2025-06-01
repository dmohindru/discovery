#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux14::{entry, iprint, iprintln, prelude::*};

// Magnetometer I²C address (7-bit: 0x1E → 8-bit: 0x3C for write)
const MAGNETOMETER: u16 = 0b0011_1100;

// LSM303AGR WHO_AM_I register address
const WHO_AM_I_M: u8 = 0x4F;

#[entry]
fn main() -> ! {
    let (i2c1, _delay, mut itm) = aux14::init();

    // === Stage 1: Send the register address we want to read ===
    {
        i2c1.cr2.write(|w| {
            w.start().set_bit();
            w.sadd().bits(MAGNETOMETER);
            w.rd_wrn().clear_bit(); // write mode
            w.nbytes().bits(1);
            w.autoend().clear_bit()
        });

        while i2c1.isr.read().txis().bit_is_clear() {}

        i2c1.txdr.write(|w| w.txdata().bits(WHO_AM_I_M));

        while i2c1.isr.read().tc().bit_is_clear() {}
    }

    // === Stage 2: Read from the magnetometer ===
    let byte = {
        i2c1.cr2.modify(|_, w| {
            w.start().set_bit();
            w.nbytes().bits(1);
            w.rd_wrn().set_bit(); // read mode
            w.autoend().set_bit()
        });

        while i2c1.isr.read().rxne().bit_is_clear() {}

        i2c1.rxdr.read().rxdata().bits()
    };

    // Expected output: 0x4F - 0b01000000
    iprintln!(&mut itm.stim[0], "0x{:02X} - 0b{:08b}", WHO_AM_I_M, byte);

    // if byte == EXPECTED_ID {
    //     iprintln!(&mut itm.stim[0], "Sensor ID verified.");
    // } else {
    //     iprintln!(&mut itm.stim[0], "Unexpected ID!");
    // }

    loop {}
}
