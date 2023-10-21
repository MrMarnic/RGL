use openal_sys::{ALCdevice, ALCcontext};
use std::collections::HashMap;
use crate::audio::audio_source::AudioSource;
use std::rc::Rc;
use crate::audio::mp3_file::MP3File;
use crate::audio::wave_file::WaveFile;
use crate::audio::vorbis_file::VorbisFile;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicPtr, AtomicBool, Ordering};
use std::time::Duration;
use crate::audio::audio_data::AudioData;

pub struct AudioHandler {
    device : AtomicPtr<ALCdevice>,
    context: AtomicPtr<ALCcontext>,
    listener_pos : [f32;3],
    listener_vel : [f32;3],
    listener_ori : [f32;6],
    audio_sources: Arc<Mutex<HashMap<String,Arc<Mutex<AudioSource>>>>>,
    audio_data: Arc<Mutex<HashMap<String,Arc<AudioData>>>>,
    is_loading: Arc<Mutex<AtomicBool>>,
    loading_max : Arc<Mutex<usize>>,
    keep_updating: Arc<Mutex<bool>>
}

impl AudioHandler {
    pub fn new() -> AudioHandler {
        unsafe {
            let mut device = openal_sys::alcOpenDevice(std::ptr::null());

            if device.is_null() {
                panic!("No sound device");
            }
            let context = openal_sys::alcCreateContext(*&mut device, std::ptr::null());
            openal_sys::alcMakeContextCurrent(context);
            if context.is_null() {
                panic!("No sound context");
            }

            let listener_pos : [f32;3] = [0.0,0.0,0.0];
            let listener_vel : [f32;3] = [0.0,0.0,0.0];
            let listener_ori : [f32;6] = [0.0,0.0,-1.0,0.0,1.0,0.0];

            openal_sys::alListenerfv(openal_sys::AL_POSITION as i32,listener_pos.as_ptr());
            openal_sys::alListenerfv(openal_sys::AL_VELOCITY as i32,listener_vel.as_ptr());
            openal_sys::alListenerfv(openal_sys::AL_ORIENTATION as i32,listener_ori.as_ptr());

            return AudioHandler { device: AtomicPtr::new(device),context: AtomicPtr::new(context), listener_pos, listener_vel, listener_ori, audio_sources: Arc::new(Mutex::new(HashMap::new())), audio_data: Arc::new(Mutex::new(HashMap::new())), is_loading: Arc::new(Mutex::new(AtomicBool::new(false))), loading_max: Arc::new(Mutex::new(0)), keep_updating: Arc::new(Mutex::new(true)) }
        }
    }

    pub fn set_pos(&mut self, pos:[f32;3]) {
        unsafe {
            self.listener_pos = pos;
            openal_sys::alListenerfv(openal_sys::AL_POSITION as i32,self.listener_pos.as_ptr());
        }
    }

    pub fn set_vel(&mut self, pos:[f32;3]) {
        unsafe {
            self.listener_pos = pos;
            openal_sys::alListenerfv(openal_sys::AL_VELOCITY as i32,self.listener_vel.as_ptr());
        }
    }

    pub fn set_ori(&mut self, pos:[f32;3]) {
        unsafe {
            self.listener_pos = pos;
            openal_sys::alListenerfv(openal_sys::AL_ORIENTATION as i32,self.listener_ori.as_ptr());
        }
    }

    pub fn destroy(&mut self) {
        unsafe {

            for (id,s) in self.audio_sources.lock().unwrap().iter() {
                s.lock().unwrap().destroy();
            }

            for data in self.audio_data.lock().unwrap().iter() {
                data.1.destroy();
            }

            openal_sys::alcMakeContextCurrent(std::ptr::null_mut());
            openal_sys::alcDestroyContext(*self.context.get_mut());
            openal_sys::alcCloseDevice(*self.device.get_mut());

            *self.keep_updating.lock().unwrap() = false;
        }
    }

    pub fn create_source(&mut self,id:String,data_id:String) -> Arc<Mutex<AudioSource>>{
        unsafe {
            let data = self.audio_data.lock().unwrap()[data_id.as_str()].clone();

            let source = Arc::new(Mutex::new(AudioSource::new(data)));

            self.audio_sources.lock().unwrap().insert(id,source.clone());

            return source;
        }
    }

    pub fn load_all_audio_in_folder(&mut self,folder: String,working_dir:String) {

        let v = self.audio_sources.clone();
        let datas = self.audio_data.clone();
        let l = self.is_loading.clone();
        let m = self.loading_max.clone();

        self.is_loading.lock().unwrap().store(true,Ordering::Relaxed);

        std::thread::spawn(move || {
            unsafe {
                let p = format!("{}\\{}",working_dir,folder);
                let paths = std::fs::read_dir(format!("{}\\{}",working_dir,folder)).unwrap();

                *m.lock().unwrap() = std::fs::read_dir(format!("{}\\{}",working_dir,folder)).unwrap().count();

                for path in paths {
                    let file = path.unwrap();
                    let path = file.path();
                    if path.is_file() {
                        let ext = path.extension().unwrap().to_os_string().into_string().unwrap();
                        if ext == "mp3" {
                            let fp = format!("{}\\{}",p,path.file_name().unwrap().to_os_string().into_string().unwrap());
                            let data = Arc::new(AudioData::new_mp3(&MP3File::new(fp.to_string())));
                            datas.lock().unwrap().insert(path.file_name().unwrap().to_os_string().into_string().unwrap(),data);
                        } else if ext == "wav" {
                            let fp = format!("{}\\{}",p,path.file_name().unwrap().to_os_string().into_string().unwrap());
                            let data = Arc::new(AudioData::new_wav(&WaveFile::new(fp.to_string())));
                            datas.lock().unwrap().insert(path.file_name().unwrap().to_os_string().into_string().unwrap(),data);
                        } else if ext == "ogg" {
                            let fp = format!("{}\\{}",p,path.file_name().unwrap().to_os_string().into_string().unwrap());
                            let data = Arc::new(AudioData::new_vorbis(&VorbisFile::new(fp.to_string())));
                            datas.lock().unwrap().insert(path.file_name().unwrap().to_os_string().into_string().unwrap(),data);
                        }
                    }
                }

                l.lock().unwrap().store(false,Ordering::Relaxed);
            }
        });
    }

    pub fn start(&self) {
        let s = self.audio_sources.clone();
        let u = self.keep_updating.clone();
        std::thread::spawn(move || {
            while *u.lock().unwrap(){
                std::thread::sleep(Duration::from_secs(1));
                s.lock().unwrap().retain(|id,source| {
                    if source.lock().unwrap().started {
                        return source.lock().unwrap().is_playing();
                    }
                    return true;
                })
            }
        });
    }

    pub fn get_source(&self,id:String) -> Arc<Mutex<AudioSource>> {
        return self.audio_sources.lock().unwrap().get(id.as_str()).unwrap().clone();
    }

    pub fn exists(&self, name:String) -> bool {
        return self.audio_data.lock().unwrap().contains_key(&name);
    }

    pub fn is_loading(&self) -> bool{
        return self.is_loading.lock().unwrap().load(Ordering::Relaxed);
    }

    pub fn get_loading_max(&self) -> usize {
        return *self.loading_max.lock().unwrap();
    }

    pub fn get_sources_count(&self) -> usize {
        return self.audio_sources.lock().unwrap().len();
    }
}

impl Drop for AudioHandler {
    fn drop(&mut self) {
        self.destroy();
    }
}