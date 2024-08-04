use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
    utils::HashMap,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<HandleMap<ImageKey>>();
    app.init_resource::<HandleMap<ImageKey>>();

    app.register_type::<HandleMap<SfxKey>>();
    app.init_resource::<HandleMap<SfxKey>>();

    app.register_type::<HandleMap<SoundtrackKey>>();
    app.init_resource::<HandleMap<SoundtrackKey>>();

    app.register_type::<ButtonLayout>();
    app.init_resource::<ButtonLayout>();
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum ImageKey {
    Ducky,
    ButtonIcons,
}

impl AssetKey for ImageKey {
    type Asset = Image;
}

#[derive(Resource, Reflect)]
pub struct ButtonLayout(pub Handle<TextureAtlasLayout>);

impl FromWorld for ButtonLayout {
    fn from_world(world: &mut World) -> Self {
        world.resource_mut::<Assets<TextureAtlasLayout>>().insert(
            crate::ui::palette::BUTTON_KEYBIND_ICONS.id(),
            TextureAtlasLayout::from_grid(UVec2::new(16, 16), 34, 24, None, None),
        );
        let handle = world
            .resource_mut::<Assets<TextureAtlasLayout>>()
            .get_strong_handle(crate::ui::palette::BUTTON_KEYBIND_ICONS.id())
            .expect("Handle to work");
        ButtonLayout(handle)
    }
}

impl FromWorld for HandleMap<ImageKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let key_bind = asset_server.load("kenney/button_icons.png");
        [
            (ImageKey::ButtonIcons, key_bind.clone()),
            (
                ImageKey::Ducky,
                asset_server.load_with_settings(
                    "images/ducky.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
        ]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SfxKey {
    ButtonHover,
    ButtonPress,
    Step1,
    Step2,
    Step3,
    Step4,
    HitOne,
    HitTwo,
    HitThree,
    Progress,
    NoProgress,
    Melt,
}

impl AssetKey for SfxKey {
    type Asset = AudioSource;
}

impl FromWorld for HandleMap<SfxKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                SfxKey::ButtonHover,
                asset_server.load("audio/sfx/button_hover.ogg"),
            ),
            (
                SfxKey::ButtonPress,
                asset_server.load("audio/sfx/button_press.ogg"),
            ),
            (SfxKey::Step1, asset_server.load("audio/sfx/step1.ogg")),
            (SfxKey::Step2, asset_server.load("audio/sfx/step2.ogg")),
            (SfxKey::Step3, asset_server.load("audio/sfx/step3.ogg")),
            (SfxKey::Step4, asset_server.load("audio/sfx/step4.ogg")),
            (SfxKey::HitOne, asset_server.load("audio/sfx/hit_one.ogg")),
            (
                SfxKey::Progress,
                asset_server.load("audio/sfx/progress.ogg"),
            ),
            (
                SfxKey::NoProgress,
                asset_server.load("audio/sfx/no_progress.ogg"),
            ),
            (SfxKey::Melt, asset_server.load("audio/sfx/cook.ogg")),
            (SfxKey::HitTwo, asset_server.load("audio/sfx/hit_two.ogg")),
            (
                SfxKey::HitThree,
                asset_server.load("audio/sfx/hit_three.ogg"),
            ),
        ]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SoundtrackKey {
    Credits,
    Gameplay,
}

impl AssetKey for SoundtrackKey {
    type Asset = AudioSource;
}

impl FromWorld for HandleMap<SoundtrackKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                SoundtrackKey::Credits,
                asset_server.load("audio/soundtracks/Monkeys Spinning Monkeys.ogg"),
            ),
            (
                SoundtrackKey::Gameplay,
                asset_server.load("audio/soundtracks/Fluffing A Duck.ogg"),
            ),
        ]
        .into()
    }
}

pub trait AssetKey: Sized {
    type Asset: Asset;
}

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct HandleMap<K: AssetKey>(HashMap<K, Handle<K::Asset>>);

impl<K: AssetKey, T> From<T> for HandleMap<K>
where
    T: Into<HashMap<K, Handle<K::Asset>>>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<K: AssetKey> HandleMap<K> {
    pub fn all_loaded(&self, asset_server: &AssetServer) -> bool {
        self.values()
            .all(|x| asset_server.is_loaded_with_dependencies(x))
    }
}
