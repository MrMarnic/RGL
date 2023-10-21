use crate::audio::mp3_file::MP3File;
use crate::audio::vorbis_file::VorbisFile;
use crate::audio::wave_file::WaveFile;
use openal_sys::{AL_FORMAT_MONO16, AL_FORMAT_STEREO16};
use std::ffi::c_void;

pub struct AudioData {
    pub buffer_id: u32
}

impl AudioData {
    pub unsafe fn new_mp3(mp3_file:&MP3File) -> AudioData {
        let mut buffer = 0;
        let frequency = mp3_file.sample_rate;
        let mut format = 0;

        openal_sys::alGenBuffers(1,&mut buffer);

        if mp3_file.channels == 1 {
            format = AL_FORMAT_MONO16;
        } else {
            format = AL_FORMAT_STEREO16;
        }

        openal_sys::alBufferData(buffer, format as i32, mp3_file.buf.as_ptr() as *const c_void, mp3_file.buf.len() as i32 * 2, frequency);

        return AudioData { buffer_id: buffer }
    }

    pub unsafe fn new_vorbis(vorbis_file:&VorbisFile) -> AudioData {
        let mut buffer = 0;
        let frequency = vorbis_file.sample_rate;
        let mut format = 0;

        openal_sys::alGenBuffers(1,&mut buffer);

        if vorbis_file.channels == 1 {
            format = AL_FORMAT_MONO16;
        } else {
            format = AL_FORMAT_STEREO16;
        }

        openal_sys::alBufferData(buffer, format as i32, vorbis_file.buf.as_ptr() as *const c_void, vorbis_file.buf.len() as i32 * 2, frequency);

        return AudioData { buffer_id: buffer }
    }


    pub unsafe fn new_wav(wave_file:&WaveFile) -> AudioData {
        let mut buffer = 0;
        let frequency = wave_file.sample_rate;
        let mut format = 0;

        openal_sys::alGenBuffers(1,&mut buffer);

        if wave_file.bits_per_sample == 8 {
            if wave_file.channels == 1 {
                format = openal_sys::AL_FORMAT_MONO8;
            } else if wave_file.channels == 2 {
                format = openal_sys::AL_FORMAT_STEREO8;
            }
        }else if wave_file.bits_per_sample == 16 {
            if wave_file.channels == 1 {
                format = openal_sys::AL_FORMAT_MONO16;
            } else if wave_file.channels == 2 {
                format = openal_sys::AL_FORMAT_STEREO16;
            }
        }

        openal_sys::alBufferData(buffer,format as i32,wave_file.buf.as_ptr() as *const c_void,wave_file.data_size,frequency);

        return AudioData { buffer_id: buffer }
    }

    pub fn destroy(&self) {
        unsafe {
            openal_sys::alDeleteBuffers(1,&self.buffer_id);
        }
    }

}

impl Drop for AudioData {
    fn drop(&mut self) {
        self.destroy();
    }
}