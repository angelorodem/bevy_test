use bevy::{asset::LoadState, gltf::Gltf, prelude::*};

use crate::{character::NameComponent, states::GameState};

#[derive(Resource, Debug, Default)]
pub struct PlayerSceneAssets {
    pub player: Handle<Scene>,
    pub player_glb: Handle<Gltf>,
    pub player_death_animation: Handle<AnimationClip>,
    pub player_run_animation: Handle<AnimationClip>,
    pub player_walk_animation: Handle<AnimationClip>,
    pub player_take_damage_animation: Handle<AnimationClip>,
    pub player_idle_animations: Vec<Handle<AnimationClip>>,
    pub chess_texture: Handle<Image>,
}

#[derive(Resource, Debug, Default)]
pub struct SkeletonSceneAssets {
    pub skeleton: Handle<Scene>,
    pub skeleton_attack_animation: Handle<AnimationClip>,
    pub skeleton_death_animation: Handle<AnimationClip>,
    pub skeleton_run_animation: Handle<AnimationClip>,
    pub skeleton_walk_animation: Handle<AnimationClip>,
    pub skeleton_take_damage_animation: Handle<AnimationClip>,
    pub skeleton_idle_animations: Vec<Handle<AnimationClip>>,
}

// Animation Entity link to link entity root to animation player
#[derive(Component)]
pub struct AnimationEntityLink(pub Entity);

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerSceneAssets>()
            .init_resource::<SkeletonSceneAssets>()
            .add_systems(PreStartup, (load_player_assets, load_skeleton_assets))
            .add_systems(Update, link_animators)
            .add_systems(
                Update,
                check_loading_status.run_if(in_state(GameState::Loading)),
            );
    }
}

fn check_loading_status(
    mut game_state: ResMut<NextState<GameState>>,
    player_assets: ResMut<PlayerSceneAssets>,
    asset_server: Res<AssetServer>,
    mut skeleton_assets: ResMut<SkeletonSceneAssets>,
) {
    if asset_server.is_loaded_with_dependencies(&player_assets.player_glb) {
        println!("Loaded, Start game");
        game_state.set(GameState::Playing);
    } else {
        println!("Loading");
    }
}

fn link_animators(
    players: Query<(Entity, &NameComponent), Without<AnimationEntityLink>>,
    animation_players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
    children: Query<&Children>,
    mut commands: Commands,
) {
    // Get all players in the scene (not animation players, just the game Character)
    for (player, name) in players.iter() {
        // Loop through all the player children in search for animation player
        for entity_child in children.iter_descendants(player) {
            if let Ok(_) = animation_players.get(entity_child) {
                // If animation player was found for the Child id, add it to the main node as an Entity ref
                commands
                    .entity(player)
                    .insert(AnimationEntityLink(entity_child.clone()));
                println!("Found animation player for {}!", name.0);
                break;
            }
        }
    }
}

fn load_player_assets(mut scene_assets: ResMut<PlayerSceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = PlayerSceneAssets {
        player: asset_server.load("Steve.glb#Scene0"),
        player_glb: asset_server.load("Steve.glb"),
        player_death_animation: asset_server.load("Steve.glb#Animation0"),
        player_run_animation: asset_server.load("Steve.glb#Animation12"),
        player_walk_animation: asset_server.load("Steve.glb#Animation14"),
        player_take_damage_animation: asset_server.load("Steve.glb#Animation2"),
        player_idle_animations: vec![
            asset_server.load("Steve.glb#Animation4"),
            asset_server.load("Steve.glb#Animation3"),
            asset_server.load("Steve.glb#Animation5"),
        ],
        chess_texture: asset_server.load("chess.jpg"),
    };
}

fn load_skeleton_assets(
    mut scene_assets: ResMut<SkeletonSceneAssets>,
    asset_server: Res<AssetServer>,
) {
    println!("Loading skeleton assets");
    *scene_assets = SkeletonSceneAssets {
        skeleton: asset_server.load("Skeleton.glb#Scene0"),
        skeleton_attack_animation: asset_server.load("Skeleton.glb#Animation0"),
        skeleton_death_animation: asset_server.load("Skeleton.glb#Animation1"),
        skeleton_run_animation: asset_server.load("Skeleton.glb#Animation5"),
        skeleton_walk_animation: asset_server.load("Skeleton.glb#Animation6"),
        skeleton_take_damage_animation: asset_server.load("Skeleton.glb#Animation2"),
        skeleton_idle_animations: vec![asset_server.load("Skeleton.glb#Animation3")],
    }
}
