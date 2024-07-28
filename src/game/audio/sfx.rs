use bevy::{audio::PlaybackMode, prelude::*};
use rand::{seq::SliceRandom, Rng};

use crate::game::assets::{HandleMap, SfxKey};

pub(super) fn plugin(app: &mut App) {
    app.observe(play_sfx);
}

fn play_sfx(
    trigger: Trigger<PlaySfx>,
    mut commands: Commands,
    sfx_handles: Res<HandleMap<SfxKey>>,
) {
    let sfx_key = match trigger.event() {
        PlaySfx::Key(key) => *key,
        PlaySfx::RandomStep => random_step(),
        PlaySfx::BlockHit => random_hit(),
    };
    commands.spawn((
        Name::new("SFX Source"),
        AudioSourceBundle {
            source: sfx_handles[&sfx_key].clone_weak(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                ..default()
            },
        },
    ));
}

/// Trigger this event to play a single sound effect.
#[derive(Event)]
pub enum PlaySfx {
    Key(SfxKey),
    RandomStep,
    BlockHit,
}

fn random_step() -> SfxKey {
    [SfxKey::Step1, SfxKey::Step2, SfxKey::Step3, SfxKey::Step4]
        .choose(&mut rand::thread_rng())
        .copied()
        .unwrap()
}

fn random_hit() -> SfxKey {
    if rand::thread_rng().gen_bool(0.001) {
        SfxKey::HitThree
    } else {
        [
            SfxKey::HitOne,
            SfxKey::HitOne,
            SfxKey::HitOne,
            SfxKey::HitTwo,
        ]
        .choose(&mut rand::thread_rng())
        .copied()
        .unwrap()
    }
}
