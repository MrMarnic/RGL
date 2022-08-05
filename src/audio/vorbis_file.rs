use std::fs::File;
use lewton::inside_ogg::OggStreamReader;
use openal_sys::{AL_FORMAT_MONO16, AL_FORMAT_STEREO16};

pub struct VorbisFile{
    pub path: String,
    pub buf : Vec<i16>,
    pub data_len : u32,
    pub sample_rate : i32,
    pub channels: i32
}

impl VorbisFile {
    pub fn new(path:String) -> VorbisFile {
        let mut vorbis_file = OggStreamReader::new(File::open(path.clone()).unwrap()).unwrap();

        let data_len = vorbis_file.ident_hdr.audio_sample_rate * vorbis_file.ident_hdr.audio_channels as u32 * 2;

        let mut data = Vec::new();

        while let Some(mut pcks) = vorbis_file.read_dec_packet_itl().unwrap() {
            data.append(&mut pcks);
        }

        return VorbisFile { path,buf:data ,data_len,sample_rate: vorbis_file.ident_hdr.audio_sample_rate as i32,channels: vorbis_file.ident_hdr.audio_channels as i32 };
    }
}