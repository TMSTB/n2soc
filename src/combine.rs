use std::env::current_dir;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Write};

pub const CHUNK_SIZE: usize = 0x14;
pub const START_OFFSET: usize = 0x10;
pub const END_OFFSET: usize = 0x13;

fn read_id_hex(path: &str) -> std::io::Result<Vec<u8>> {
    let mut buffer: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];
    let file = File::open(path)?;
    let mut reader = BufReader::with_capacity(CHUNK_SIZE, file);

    let n = reader.read(&mut buffer)?;

    if n != CHUNK_SIZE {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!(
                "Save data may be corrupted. Expected {} bytes, got {} bytes",
                CHUNK_SIZE, n
            ),
        ));
    }

    let target_hex = buffer
        .get(START_OFFSET..=END_OFFSET)
        .expect("failed to get target hex")
        .to_vec();
    Ok(target_hex)
}

pub fn combine(id_save_path: &str, target_path: &str) -> std::io::Result<()> {
    let source_id_hex = read_id_hex(id_save_path)?;
    overwrite(target_path, source_id_hex)
}

fn overwrite(target_path: &str, source_id_hex: Vec<u8>) -> std::io::Result<()> {
    let target_file = File::open(target_path)?;
    let output_dir = current_dir()?.join("output");
    fs::create_dir_all(&output_dir)?;
    let output_path = output_dir.join("SAVEDATA.BIN");
    let mut reader = BufReader::with_capacity(CHUNK_SIZE, target_file);
    let mut output_file = File::create(&output_path)?;
    let mut buffer: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];

    let n = reader.read(&mut buffer)?;
    if n != CHUNK_SIZE {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!(
                "Save data may be corrupted. Expected {} bytes, got {} bytes",
                CHUNK_SIZE, n
            ),
        ));
    }
    buffer[START_OFFSET..=END_OFFSET].copy_from_slice(&source_id_hex);
    output_file.write_all(&buffer)?;

    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        output_file.write_all(&buffer[..n])?;
    }

    println!("output: {}", output_path.to_str().unwrap());
    Ok(())
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    #[test]
    fn test_procedure() {
        let id_save_path = "test_id.bin";
        let target_path = "test_target.bin";
        let output_dir = current_dir().unwrap().join("output");
        let output_path = output_dir.join("SAVEDATA.BIN");
        let mut id_data = Vec::<u8>::with_capacity(25);
        let mut target_data = Vec::<u8>::with_capacity(25);
        let mut output_data = Vec::<u8>::with_capacity(25);
        for i in 0x00..0x19 {
            id_data.push(i as u8);
        }
        for i in 0xAA..0xC3 {
            target_data.push(i as u8);
            output_data.push(i as u8);
        }
        let mut id_file = File::create(id_save_path).unwrap();
        let mut target_file = File::create(target_path).unwrap();
        id_file.write_all(&id_data).unwrap();
        target_file.write_all(&target_data).unwrap();

        output_data[START_OFFSET..=END_OFFSET].copy_from_slice(&[0x10, 0x11, 0x12, 0x13]);
        combine(id_save_path, target_path).unwrap();

        let mut output_file = File::open(output_path).unwrap();
        let mut output_buffer = Vec::<u8>::with_capacity(25);
        let n = output_file.read_to_end(&mut output_buffer).unwrap();

        assert_eq!(n, 25);
        assert_eq!(output_buffer, output_data);

        fs::remove_file(id_save_path).unwrap();
        fs::remove_file(target_path).unwrap();
        fs::remove_dir_all(output_dir).unwrap()
    }
}
