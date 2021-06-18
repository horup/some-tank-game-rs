use bevy::prelude::*;

pub struct AudioPlugin;

pub struct PlayAudioEvent {
    pub path:String
}

impl PlayAudioEvent {
    pub fn new(path:&str) -> Self {
        Self {
            path:path.into()
        }
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
            path:str
        }
    }
}

fn player(asset_server:Res<AssetServer>, mut reader:EventReader<PlayAudioEvent>, audio:Res<bevy_kira_audio::Audio>) {
    for e in reader.iter() {
        let res = asset_server.load(e.path.as_str());
        audio.play(res);
        audio.set_playback_rate(0.5);
        audio.get
    }
}

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(bevy_kira_audio::AudioPlugin);
        app.add_event::<PlayAudioEvent>();
        app.add_system_to_stage(CoreStage::PostUpdate, player.system());
    }
}