#![allow(clippy::precedence)]

use bevy::prelude::*;

use bevy_procedural_audio::prelude::*;

use bevy_oddio::{
  Audio,
  AudioPlugin,
  AudioSource,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AudioPlugin::new())
        .add_plugins(DspPlugin::default())
        .add_dsp_source(sine_wave, SourceType::Static { duration: 0.5 })
        .add_dsp_source(triangle_wave, SourceType::Static { duration: 0.5 })
        .add_systems(Update, interactive_audio)
        .run();
}

fn sine_wave() -> impl AudioUnit {
    // Note is A4
    sine_hz(440.0) >> split::<U2>() * 0.2
}

fn triangle_wave() -> impl AudioUnit {
    // Note is G4
    triangle_hz(392.0) >> split::<U2>() * 0.2
}

fn interactive_audio(
    input: Res<ButtonInput<KeyCode>>,
    mut assets: ResMut<Assets<AudioSource<[f32; 2]>>>,
    dsp_manager: Res<DspManager>,
    mut audio: ResMut<Audio<[f32; 2], AudioSource<[f32; 2]>>>,
) {
    if input.just_pressed(KeyCode::KeyS) {
        audio.play_dsp(assets.as_mut(), &dsp_manager.get_graph(sine_wave).unwrap());
    }

    if input.just_pressed(KeyCode::KeyT) {
        audio.play_dsp(
            assets.as_mut(),
            &dsp_manager.get_graph(triangle_wave).unwrap(),
        );
    }
}
