use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    // Define the CHIP-8 program opcodes and sprite data
    let chip8_program: [u8; 6] = [
        0x00, 0xE0, // LD V0, 0xA0
        0x00, 0xE0, // LD V1, 0x0C
        0xA2, 0x20, // LD I, 0x220
    ];

    // Create a new file for the CHIP-8 program
    let mut file = File::create("example.ch8")?;

    // Write the CHIP-8 program to the file
    file.write_all(&chip8_program)?;

    println!("CHIP-8 program written to 'example.ch8'");

    Ok(())
}
