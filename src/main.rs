use bevy::{
    prelude::*,
    render::{camera::RenderTarget, render_resource::*, view::RenderLayers},
    window::PrimaryWindow,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<SceneTime>()
        .add_systems(Startup, init_rt)
        .add_systems(FixedUpdate, reset_scene)
        .run();
}

pub const RENDER_LAYER_PRIMARY: usize = 0;
pub const RENDER_LAYER_SELECTION: usize = 1;

#[derive(Component)]
struct TestEnt;

#[derive(Resource)]
struct SceneTime {
    time: f32,
}

impl Default for SceneTime {
    fn default() -> Self {
        SceneTime { time: 0.0 }
    }
}

#[derive(Resource)]
struct TestRT {
    render_target: Handle<Image>,
}

fn init_rt(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let size = Extent3d {
        width: window.resolution.width() as u32,
        height: window.resolution.height() as u32,
        ..default()
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_SRC
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    image.resize(size);
    let render_target = images.add(image);
    commands.insert_resource(TestRT { render_target });
}

fn reset_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,

    test_rt: Res<TestRT>,
    mut scene_time: ResMut<SceneTime>,
    test_ents: Query<Entity, With<TestEnt>>,
) {
    scene_time.time -= 1.0 / 60.0;
    if scene_time.time > 0.0 {
        return;
    }

    for entity in &test_ents {
        commands.entity(entity).despawn_recursive();
    }

    commands.insert_resource(SceneTime { time: 3.0 });

    info!("Recreating scene!");

    // Normal camera.
    commands.spawn((Camera2d, TestEnt));

    // Render target camera.
    commands.spawn((
        Camera2d,
        Camera {
            order: -1,
            is_active: true,
            target: RenderTarget::Image(test_rt.render_target.clone()),
            clear_color: ClearColorConfig::Custom(Color::srgba(0.0, 0.0, 0.0, 1.0)),
            ..default()
        },
        RenderLayers::layer(RENDER_LAYER_SELECTION),
        TestEnt,
    ));

    let b1_pos = Transform::from_translation(Vec3::new(-200.0, 0.0, 0.0));
    commands.spawn((
        Sprite::from_image(asset_server.load("branding/bevy_bird_dark.png")),
        b1_pos,
        RenderLayers::layer(RENDER_LAYER_PRIMARY),
        TestEnt,
    ));

    let b2_pos = Transform::from_translation(Vec3::new(200.0, 0.0, 0.0));
    commands.spawn((
        Sprite::from_image(asset_server.load("branding/bevy_bird_dark.png")),
        b2_pos,
        RenderLayers::layer(RENDER_LAYER_PRIMARY).with(RENDER_LAYER_SELECTION),
        TestEnt,
    ));

    commands.spawn((
        ImageNode {
            image: test_rt.render_target.clone(),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            width: Val::Px(480.0),
            height: Val::Px(270.0),
            right: Val::Px(50.0),
            bottom: Val::Px(50.0),
            ..default()
        },
        TestEnt,
    ));

    commands.spawn((
        Text::new("the left bird should never be visible in the lower right camera's view!\n it is on a different render layer"),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 14.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
        TestEnt,
    ));
}
