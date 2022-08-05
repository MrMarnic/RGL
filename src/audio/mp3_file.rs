use std::fs::File;
use lewton::inside_ogg::OggStreamReader;
use openal_sys::{AL_FORMAT_MONO16, AL_FORMAT_STEREO16};
use std::path::Path;

pub struct MP3File{
    pub path: String,
    pub buf : Vec<i16>,
    pub data_len : u32,
    pub sample_rate : i32,
    pub channels: i32
}

impl MP3File {
    pub fn new(path:String) -> MP3File {
        let mut mp3_file = minimp3::Decoder::new(File::open(path.clone()).unwrap());
        let mut data = Vec::new();

        let mut sample_rate = 0;
        let mut channels = 0;
        let mut started: bool = false;

        while let frame = mp3_file.next_frame() {

            if frame.is_ok() {
                let mut f = frame.unwrap();
                if !started {
                    sample_rate = f.sample_rate;
                    channels = f.channels;
                }
                data.append(&mut f.data);
            }else {
                break;
            }
        }

        let data_len = (data.len() * 2) as u32;

        return MP3File { path,buf:data ,data_len,sample_rate,channels: channels as i32 };
    }
}