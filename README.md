# Bevy Procedural Audio

[![CI](https://github.com/bsgbryan/bevy_procedural_audio/actions/workflows/rust.yml/badge.svg)](https://github.com/bsgbryan/bevy_procedural_audio/actions/workflows/rust.yml) ![Crates.io](https://img.shields.io/crates/v/bevy_procedural_audio) ![Crates.io](https://img.shields.io/crates/l/bevy_procedural_audio) ![docs.rs](https://img.shields.io/docsrs/bevy_procedural_audio)

A third party Bevy plugin that integrates [FunDSP] into [Bevy].

`bevy_procedural_audio` supports integration for `bevy_audio`, [`bevy_kira_audio`], and `bevy_oddio`.

[FunDSP]: https://github.com/SamiPerttu/fundsp
[Bevy]: https://github.com/bevyengine/bevy
[`bevy_kira_audio`]: https://github.com/NiklasEi/bevy_kira_audio
[`bevy_oddio`]: https://github.com/harudagondi/bevy_oddio

⚠ **WARNING: Lower your volume before testing your sounds!** ⚠

Remember to lower the volume by passing the settings with `DspManager::add_graph_with_settings` or multiplying your DSP graph with a low constant (lower than 1.0).

## Usage

```rust no_run
#![allow(clippy::precedence)]

use bevy::prelude::*;

use bevy_procedural_audio::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            DspPlugin::default(),
        ))
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
            .unwrap_or_else(|| panic!("DSP source not found!"))
            .clone(),
    );
    commands.spawn(AudioPlayer(source));
}

```

## Acknowledgement

I'd like to offer a big thank you to the authors of [FunDSP] and [Bevy] for making this plugin possible.

## License

`bevy_procedural_audio` is distributed under the terms of either the MIT license or the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
