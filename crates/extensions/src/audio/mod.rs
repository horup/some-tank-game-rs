use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;

pub struct AudioPlugin;

pub struct PlayAudioEvent {
    pub path:String,
    pub music:bool
}

impl PlayAudioEvent {
    pub fn new(path:&str) -> Self {
        Self {
            path:path.into(),
            music:false
        }
    }

    pub fn with_music(mut self, music:bool) -> Self {
        self.music = music;
        self
    }
}

impl From<&str> for PlayAudioEvent {
    fn from(str: &str) -> Self {
        Self::new(str)
    }
}

impl From<String> for PlayAudioEvent {
    fn from(str: String) -> Self {
        Self {
            path:str,
            music:false
        }
    }
}

fn player(asset_server:Res<AssetServer>, mut reader:EventReader<PlayAudioEvent>, audio:Res<bevy_kira_audio::Audio>) {
    for e in reader.iter() {
        if e.music == false {
            if e.path.len() > 0 {
                let res = asset_server.load(e.path.as_str());
                audio.play(res);
            }
        } else {
            audio.stop_channel(&AudioChannel::new("music".into()));
            if e.path.len() > 0 {
                let res = asset_server.load(e.path.as_str());
                audio.play_looped_in_channel(res, &AudioChannel::new("music".into()));
            }
        }
    }
}

fn startup(audio:Res<bevy_kira_audio::Audio>) {
    audio.set_volume_in_channel(0.5, &AudioChannel::new("music".into()));
}

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(bevy_kira_audio::AudioPlugin);
        app.add_startup_system(startup.system());
        app.add_event::<PlayAudioEvent>();
        app.add_system_to_stage(CoreStage::PostUpdate, player.system());
    }
}