use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    // Define the CHIP-8 program opcodes and sprite data
    let chip8_program: [u8; 14] = [
        0x00, 0xE0, // Clear screen
        0xA2, 0x20, // LD I, 0x220
        0xA2, 0x30, // LD I, 0x230
        0x12, 0x0A, // Jump to 0x210
        0xA2, 0x40, // LD I, 0x240 (this will be skipped)
        0xA2, 0x34, // LD I, 0x250
        0x00, 0xDF, // Exit
    ];

    // Create a new file for the CHIP-8 program
    let mut file = File::create("example.ch8")?;

    // Write the CHIP-8 program to the file
    file.write_all(&chip8_program)?;

    println!("CHIP-8 program written to 'example.ch8'");

    Ok(())
}
