use std::fs::File;
use std::io::{BufReader, Read};

pub struct WaveFile {
    pub chunk_size : i32,
    pub format_type : i16,
    pub channels : i16,
    pub sample_rate : i32,
    pub avg_bytes_per_second : i32,
    pub bytes_per_sample : i16,
    pub bits_per_sample : i16,
    pub path: String,
    pub data_size : i32,
    pub buf: Vec<u8>
}

impl WaveFile {
    pub unsafe fn new(path:String) -> WaveFile {
        let mut file = File::open(path.clone()).unwrap();
        let mut reader = BufReader::new(file);
        let mut array: [char;4] = WaveFile::read_char_array(&mut reader);
        if array[0] != 'R' || array[1] != 'I' || array[2] != 'F' || array[3] != 'F' {
            panic!("NO RIFF");
        }

        let mut size: [u8;4] = [0;4];
        reader.read(&mut size);

        let mut array: [char;4] = WaveFile::read_char_array(&mut reader);
        if array[0] != 'W' || array[1] != 'A' || array[2] != 'V' || array[3] != 'E' {
            panic!("NO WAVE");
        }

        let mut array: [char;4] = WaveFile::read_char_array(&mut reader);
        if array[0] != 'f' || array[1] != 'm' || array[2] != 't' || array[3] != ' ' {
            panic!("NO fmt");
        }

        let chunk_size = WaveFile::read_i32_le(&mut reader);

        let format_type = WaveFile::read_i16_le(&mut reader);

        let channels = WaveFile::read_i16_le(&mut reader);

        let sample_rate = WaveFile::read_i32_le(&mut reader);

        let avg_bytes_per_second = WaveFile::read_i32_le(&mut reader);

        let bytes_per_sample = WaveFile::read_i16_le(&mut reader);

        let bits_per_sample = WaveFile::read_i16_le(&mut reader);

        let array = WaveFile::read_char_array(&mut reader);
        if array[0] as char != 'd' || array[1] as char != 'a' || array[2] as char != 't' || array[3] as char != 'a' {
            panic!("NO DATA");
        }

        let data_size = WaveFile::read_i32_le(&mut reader);


        let mut buf: Vec<u8> = Vec::with_capacity(data_size as usize);
        buf.set_len(data_size as usize);
        reader.read_exact(&mut buf);

        return WaveFile {
            chunk_size,
            format_type,
            channels,
            sample_rate,
            avg_bytes_per_second,
            bytes_per_sample,
            bits_per_sample,
            path,
            data_size,
            buf
        }
    }

    pub fn read_i32(reader:&mut BufReader<File>) -> i32 {
        let mut array: [u8;4] = [0;4];
        reader.read_exact(&mut array);
        return i32::from_be_bytes(array);
    }

    pub fn read_i16(reader: &mut BufReader<File>) -> i16 {
        let mut array: [u8;2] = [0;2];
        reader.read_exact(&mut array);
        return i16::from_be_bytes(array);
    }

    pub fn read_i32_le(reader:&mut BufReader<File>) -> i32 {
        let mut array: [u8;4] = [0;4];
        reader.read_exact(&mut array);
        return i32::from_le_bytes(array);
    }

    pub fn read_i16_le(reader: &mut BufReader<File>) -> i16 {
        let mut array: [u8;2] = [0;2];
        reader.read_exact(&mut array);
        return i16::from_le_bytes(array);
    }

    pub fn read_char_array(reader:&mut BufReader<File>) -> [char;4] {
        let mut array: [u8;4] = [0;4];
        reader.read_exact(&mut array);

        let mut char: [char;4] = [array[0] as char,array[1] as char, array[2] as char, array[3] as char];

        return char;
    }
}

