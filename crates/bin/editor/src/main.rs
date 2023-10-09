use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_flycam::NoCameraAndGrabPlugin;
use bevy_iron_voxel::VoxelWorldPlugin;

pub mod theme;
pub mod main_menu;
pub mod voxel_edit_mode;
pub mod voxel_edit_options;

fn main() {
    let mut app = App::new();
    app

        //Configure Bevy
        .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Ironverse".into(),
            mode: bevy::window::WindowMode::BorderlessFullscreen,
            resolution: (800., 600.).into(),
            present_mode: PresentMode::AutoVsync,
            fit_canvas_to_parent: true,
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
        }))

        //Add Global stuff
        .add_state::<AppState>()
        .add_plugins(bevy_iron_ui::UiPlugin{theme: theme::theme})
        .add_plugins(NoCameraAndGrabPlugin)
        .add_plugins(VoxelWorldPlugin)
        .add_systems(Startup, |mut next_state: ResMut<NextState<AppState>>| {
            next_state.set(AppState::MainMenu);
        })
        .init_resource::<voxel_edit_mode::HotbarVoxels>()
        .add_systems(Update, voxel_edit_mode::update_hotbar)
        //.add_systems(Startup, setup_camera)

        //AppState: MainMenu
        .add_systems(OnEnter(AppState::MainMenu), main_menu::on_enter)
        .add_systems(Update, main_menu::button_actions.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, main_menu::controls.run_if(in_state(AppState::MainMenu)))

        //AppState: VoxelEditMode
        .add_systems(OnEnter(AppState::VoxelEditMode), voxel_edit_mode::on_enter)
        .add_systems(Update, voxel_edit_mode::controls.run_if(in_state(AppState::VoxelEditMode)))
        .add_systems(OnExit(AppState::VoxelEditMode), voxel_edit_mode::on_exit)

        //AppState: VoxelEditOptions
        .add_systems(OnEnter(AppState::VoxelEditOptions), voxel_edit_options::on_enter)
        .add_systems(Update, voxel_edit_options::controls.run_if(in_state(AppState::VoxelEditOptions)))
        .add_systems(Update, voxel_edit_options::update_color_picker.run_if(in_state(AppState::VoxelEditOptions)))

        .run();
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    VoxelEditMode,
    VoxelEditOptions,
}

/* fn setup_camera(mut commands: Commands, mut bevy_voxel_res: ResMut<BevyVoxelResource>, game_res: Res<GameResource>) {
    //Add Player Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        FlyCam,
        create_new_player(bevy_voxel_res, &game_res)
    ));
} */