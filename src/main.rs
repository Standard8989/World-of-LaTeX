use bevy::{prelude::*, sprite::Anchor, window::PrimaryWindow};

const WINDOW_WIDTH: f32 = 700.;
const WINDOW_HEIGHT: f32 = 400.;

const HALF_WINDOW_WIDTH: f32 = WINDOW_WIDTH / 2.;
const HALF_WINDOW_HEIGHT: f32 = WINDOW_HEIGHT / 2.;

const TILE_SIZE: f32 = 10.;
const TILE_DEPTH: f32 = 0.;

const HP_BAR_TRANSLATION: Vec2 = Vec2::new(-HALF_WINDOW_WIDTH + 10., HALF_WINDOW_HEIGHT - 10.);
const HP_BAR_SCALE: f32 = 3.;
const HP_BAR_DEPTH: f32 = 2.;

const HP_BAR_WIDTH: f32 = 40.;
const HP_BAR_HEIGHT: f32 = 9.;
const HP_BAR_COUNT: u32 = 13;

const PLAYER_TRANSLATION: Vec2 = Vec2::new(-HALF_WINDOW_WIDTH + 50., 0.);
const PLAYER_SCALE: f32 = 0.1;
const PLAYER_DEPTH: f32 = 1.;

const PLAYER_SPEED: f32 = 200.;

const BULLET_SCALE: f32 = 0.07;
const BULLET_SPEED: f32 = 1000.;
const BULLET_Y_DIFF: f32 = 5.;
const BULLET_X_DIFF: f32 = 10.;
const BULLET_INTERVAL: f32 = 0.05;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "北方革命録".to_string(),
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(MyUiPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(ObjectPlugin)
        // .add_plugins(MyDebugPlugin)
        .run();
}

pub struct MyDebugPlugin;

impl Plugin for MyDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_window_size);
    }
}

fn print_window_size(windows: Query<&Window, With<PrimaryWindow>>) {
    for window in windows.iter() {
        let width = window.width();
        let height = window.height();
        println!("Window size: {} x {}", width, height);
    }
}

pub struct MyUiPlugin;

impl Plugin for MyUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_play_camera)
            .add_systems(Startup, spawn_hp_bar)
            // .add_systems(Startup, spawn_background)
            ;
    }
}

fn spawn_play_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_background(mut commands: Commands, asset: Res<ObjectAssets>) {
    let x_count = (WINDOW_WIDTH / TILE_SIZE).ceil() as usize;
    let y_count = (WINDOW_HEIGHT / TILE_SIZE).ceil() as usize;

    let x_diff = -HALF_WINDOW_WIDTH;
    let y_diff = -HALF_WINDOW_WIDTH;

    for x in 0..x_count {
        for y in 0..y_count {
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
                transform: Transform::from_translation(
                    Vec2::new(x as f32 * TILE_SIZE + x_diff, y as f32 * TILE_SIZE + y_diff)
                        .extend(TILE_DEPTH),
                ),
                texture: asset.background.clone(),
                ..default()
            });
        }
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct HpBarTag;

fn spawn_hp_bar(mut commands: Commands, asset: Res<ObjectAssets>) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                anchor: Anchor::TopLeft,
                rect: Some(Rect::new(
                    0.,
                    HP_BAR_COUNT as f32 * HP_BAR_HEIGHT,
                    HP_BAR_WIDTH,
                    (HP_BAR_COUNT + 1) as f32 * HP_BAR_HEIGHT,
                )),
                ..default()
            },
            transform: Transform::from_translation(HP_BAR_TRANSLATION.extend(HP_BAR_DEPTH))
                .with_scale(Vec2::splat(HP_BAR_SCALE).extend(1.)),
            texture: asset.hp_bar.clone(),
            ..default()
        },
        HpBarTag,
    ));
}

#[derive(Resource, Debug, Default)]
pub struct ObjectAssets {
    pub background: Handle<Image>,
    pub hp_bar: Handle<Image>,
    pub player: Handle<Image>,
    pub bullet: Handle<Image>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ObjectAssets>()
            .add_systems(PreStartup, load_assets);
    }
}

fn load_assets(mut object_assets: ResMut<ObjectAssets>, asset_server: Res<AssetServer>) {
    *object_assets = ObjectAssets {
        background: asset_server.load("background.png"),
        hp_bar: asset_server.load("HP_bar.png"),
        player: asset_server.load("player.png"),
        bullet: asset_server.load("bullet.png"),
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (move_player, shoot));
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct PlayerTag;

fn spawn_player(mut commands: Commands, asset: Res<ObjectAssets>) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                anchor: Anchor::Center,
                ..default()
            },
            transform: Transform::from_translation(PLAYER_TRANSLATION.extend(PLAYER_DEPTH))
                .with_scale(Vec2::splat(PLAYER_SCALE).extend(1.)),
            texture: asset.player.clone(),
            ..default()
        },
        PlayerTag,
        BulletInterval::new(BULLET_INTERVAL),
    ));
}

fn move_player(
    mut player: Query<(&mut Transform, &Handle<Image>), With<PlayerTag>>,
    button: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    images: Res<Assets<Image>>,
) {
    let Ok((mut player_transform, image)) = player.get_single_mut() else {
        panic!("failed to get player_transform in move_player");
    };

    if button.pressed(KeyCode::ArrowUp) {
        player_transform.translation.y += PLAYER_SPEED * time.delta_seconds();
    }
    if button.pressed(KeyCode::ArrowDown) {
        player_transform.translation.y -= PLAYER_SPEED * time.delta_seconds();
    }
    if button.pressed(KeyCode::ArrowRight) {
        player_transform.translation.x += PLAYER_SPEED * time.delta_seconds();
    }
    if button.pressed(KeyCode::ArrowLeft) {
        player_transform.translation.x -= PLAYER_SPEED * time.delta_seconds();
    }

    if let Some(image) = images.get(image) {
        let size = image.size().as_vec2() * PLAYER_SCALE;

        player_transform.translation.x = player_transform.translation.x.clamp(
            -HALF_WINDOW_WIDTH + size.x / 2.,
            HALF_WINDOW_WIDTH - size.x / 2.,
        );
        player_transform.translation.y = player_transform.translation.y.clamp(
            -HALF_WINDOW_HEIGHT + size.y / 2.,
            HALF_WINDOW_HEIGHT - size.y / 2.,
        );
    } else {
        info!("failed to get image of player");
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct BulletTag;

#[derive(Debug, Clone, Copy, Resource, Component)]
struct BulletInterval {
    pub value: f32,
    pub interval: f32,
}

impl BulletInterval {
    fn new(interval: f32) -> Self {
        Self {
            value: 0.,
            interval,
        }
    }
}

fn shoot(
    mut commands: Commands,
    asset: Res<ObjectAssets>,
    mut player: Query<(&mut BulletInterval, &Transform), With<PlayerTag>>,
    button: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut interval, player_transform)) = player.get_single_mut() else {
        panic!("failed to get player_transform in shoot");
    };

    interval.value += time.delta_seconds();

    if button.pressed(KeyCode::Space) && interval.value >= interval.interval {
        interval.value -= interval.interval;

        let transform = Transform::from_translation(player_transform.translation)
            .with_scale(Vec2::splat(BULLET_SCALE).extend(1.));

        let up_transform = transform
            .with_translation(transform.translation + Vec3::new(BULLET_X_DIFF, BULLET_Y_DIFF, 0.));
        let down_transform = transform
            .with_translation(transform.translation + Vec3::new(BULLET_X_DIFF, -BULLET_Y_DIFF, 0.));

        commands.spawn((
            Velocity::new(Vec2::new(BULLET_SPEED, 0.), f32::INFINITY),
            SpriteBundle {
                sprite: Sprite {
                    anchor: Anchor::Center,
                    ..default()
                },
                transform: up_transform,
                texture: asset.bullet.clone(),
                ..default()
            },
            BulletTag,
        ));

        commands.spawn((
            Velocity::new(Vec2::new(BULLET_SPEED, 0.), f32::INFINITY),
            SpriteBundle {
                sprite: Sprite {
                    anchor: Anchor::Center,
                    ..default()
                },
                transform: down_transform,
                texture: asset.bullet.clone(),
                ..default()
            },
            BulletTag,
        ));
    }
}

#[derive(Debug, Component, Default)]
pub struct Velocity {
    pub value: Vec2,
    pub max: f32,
}

impl Velocity {
    pub fn new(value: Vec2, max: f32) -> Self {
        Self { value, max }
    }
}

#[derive(Debug, Component, Default)]
pub struct Acceleration {
    pub value: Vec2,
}

impl Acceleration {
    pub fn new(value: Vec2) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug, Default)]
// こやつは角速度
// 単位はrad
pub struct AngularVelocity {
    pub value: f32,
}

impl AngularVelocity {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}
pub struct ObjectPlugin;

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_direction, update_velocity, update_position).chain(),
        );
    }
}

fn update_velocity(mut query: Query<(&mut Velocity, &Acceleration)>, time: Res<Time>) {
    for (mut velocity, acceleration) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_seconds();

        velocity.value = velocity.value.clamp_length_max(velocity.max);
    }
}

fn update_position(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        let dp = velocity.value * time.delta_seconds();
        transform.translation.x += dp.x;
        transform.translation.y += dp.y;
    }
}

fn update_direction(
    mut query: Query<(&mut Transform, &AngularVelocity, &mut Velocity)>,
    time: Res<Time>,
) {
    return;

    for (mut transform, angular_velocity, mut velocity) in query.iter_mut() {}
}
