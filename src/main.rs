use bevy::prelude::*;

struct Direction(bool);

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup_fox.system())
        .add_startup_system(setup_snail.system())
        .add_system(animate_sprite_system.system())
        .add_system(directed_animate_sprite_system.system())
        .run();
}

fn animate_sprite_system(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<Without<Direction, (&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query.iter() {
        if timer.finished {
            let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
            timer.reset();
        }
    }
}

fn directed_animate_sprite_system(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &mut Direction,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle, mut direction) in &mut query.iter() {
        if timer.finished {
            let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
            let max_frame_idx = texture_atlas.textures.len() - 1;

            direction.0 = if direction.0 && sprite.index as usize == max_frame_idx {
                !direction.0
            } else if !direction.0 && sprite.index == 0 {
                !direction.0
            } else {
                direction.0
            };

            sprite.index = match direction.0 {
                true => (sprite.index as usize + 1) as u32,
                false => (sprite.index as usize - 1) as u32,
            };
            timer.reset();
        }
    }
}

fn setup_fox(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Add a spritesheet
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/Fox-SpriteSheet.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 3, 3);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            scale: Scale(3.0),
            translation: Translation::new(-512.0, -100.0, 0.0),
            ..Default::default()
        })
        .with(Timer::from_seconds(0.2))
        .with(Direction(true));
}

fn setup_snail(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Add a spritesheet
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/Snail-SpriteSheet.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 2, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            scale: Scale(3.0),
            translation: Translation::new(100.0, 100.0, 0.0),
            ..Default::default()
        })
        .with(Timer::from_seconds(0.2));
}
