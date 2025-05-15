#![allow(clippy::precedence)]

use bevy::prelude::*;

use bevy_procedural_audio::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DspPlugin::default())
        .add_dsp_source(white_noise, SourceType::Dynamic)
        .add_systems(PostStartup, play_noise)
        .run();
}

fn white_noise() -> impl AudioUnit {
    white() >> split::<U2>() * 0.2
}

fn play_noise(
    mut commands: Commands,
    mut assets: ResMut<Assets<DspSource>>,
    dsp_manager: Res<DspManager>,
) {
    let source = assets.add(
        dsp_manager
            .get_graph(white_noise)
            .unwrap_or_else(|| panic!("DSP source not found!")),
    );
    commands.spawn(AudioPlayer(source));
}
