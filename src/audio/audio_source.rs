use std::sync::Arc;
use crate::audio::audio_data::AudioData;
use openal_sys::{AL_TRUE, ALint, AL_FALSE, AL_PLAYING, AL_SOURCE_STATE, AL_SEC_OFFSET, alGetBufferi, AL_SIZE, AL_CHANNELS, AL_BITS, AL_FREQUENCY};

pub struct AudioSource {
    pub source_id: u32,
    pub data: Arc<AudioData>,
    pub pitch: f32,
    pub gain: f32,
    pub position: [f32;3],
    pub velocity: [f32;3],
    pub looping: bool,
    pub started: bool,
    pub length: f32
}

impl AudioSource {
    pub unsafe fn new(data:Arc<AudioData>) -> AudioSource {
        let mut source = 0;

        openal_sys::alGenSources(1,&mut source);

        let source_pos : [f32;3] = [0.0,0.0,0.0];
        let source_vel : [f32;3] = [0.0,0.0,0.0];
        let pitch = 1.0;
        let gain = 1.0;
        let looping = false;

        openal_sys::alSourcef(source, openal_sys::AL_PITCH as i32, pitch);
        openal_sys::alSourcef(source, openal_sys::AL_GAIN as i32, gain);
        openal_sys::alSourcefv(source, openal_sys::AL_POSITION as i32, source_pos.as_ptr());
        openal_sys::alSourcefv(source, openal_sys::AL_VELOCITY as i32, source_vel.as_ptr());
        openal_sys::alSourcei(source, openal_sys::AL_LOOPING as i32, if looping { AL_TRUE as i32} else {AL_FALSE as i32});

        openal_sys::alSourcei(source, openal_sys::AL_BUFFER as i32, data.buffer_id as ALint);

        let mut source = AudioSource { source_id: source, data, pitch, gain, position: source_pos, velocity: source_vel,looping, started: false, length: 0.0 };
        source.length = source.get_length();

        return source;

    }

    pub fn set_pitch(&mut self,pitch:f32) {
        unsafe {
            self.pitch = pitch;
            openal_sys::alSourcef(self.source_id, openal_sys::AL_PITCH as i32, pitch);
        }
    }

    pub fn set_gain(&mut self,gain:f32) {
        unsafe {
            self.gain = gain;
            openal_sys::alSourcef(self.source_id, openal_sys::AL_GAIN as i32, gain);
        }
    }

    pub fn set_pos(&mut self,pos:[f32;3]) {
        unsafe {
            self.position = pos;
            openal_sys::alSourcefv(self.source_id, openal_sys::AL_POSITION as i32, self.position.as_ptr());
        }
    }

    pub fn set_vel(&mut self,vel:[f32;3]) {
        unsafe {
            self.velocity = vel;
            openal_sys::alSourcefv(self.source_id, openal_sys::AL_VELOCITY as i32, self.position.as_ptr());
        }
    }

    pub fn set_looping(&mut self,looping:bool) {
        unsafe {
            self.looping = looping;
            openal_sys::alSourcei(self.source_id, openal_sys::AL_LOOPING as i32, if looping { AL_TRUE as i32} else {AL_FALSE as i32});
        }
    }

    pub fn play(&mut self) {
        unsafe {
            openal_sys::alSourcePlay(self.source_id);
            self.started = true;
        }
    }

    pub fn stop(&self) {
        unsafe {
            openal_sys::alSourceStop(self.source_id);
        }
    }

    pub fn destroy(&self) {
        unsafe {
            openal_sys::alDeleteSources(1,&self.source_id);
        }
    }

    pub fn is_playing(&self) -> bool {
        unsafe {
            let mut state = 0;
            openal_sys::alGetSourcei(self.source_id,AL_SOURCE_STATE as i32,&mut state);

            return state == AL_PLAYING as i32;
        }
    }

    pub fn get_time(&self) -> f32 {
        unsafe {
            let mut time = 0.0;
            openal_sys::alGetSourcef(self.source_id,AL_SEC_OFFSET as i32,&mut time);

            return time;
        }
    }

    fn get_length(&self) -> f32 {
        unsafe {
            let mut sizeInBytes = 0;
            let mut channels = 0;
            let mut bits = 0;

            alGetBufferi(self.data.buffer_id, AL_SIZE as i32, &mut sizeInBytes);
            alGetBufferi(self.data.buffer_id, AL_CHANNELS as i32, &mut channels);
            alGetBufferi(self.data.buffer_id, AL_BITS as i32, &mut bits);

            let lengthInSamples = sizeInBytes * 8 / (channels * bits);

            let mut frequency = 0;

            alGetBufferi(self.data.buffer_id, AL_FREQUENCY as i32, &mut frequency);

            return lengthInSamples as f32 / frequency as f32;
        }
    }
}