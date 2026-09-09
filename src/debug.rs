use bevy::prelude::*;

const DEBUG_VIEW_TOGGLE_BUTTON: KeyCode = KeyCode::F1;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<DebugState>();

        app.add_systems(Update, toggle_debug_view);

        app.add_systems(
            Update,
            draw_bot_actions.run_if(in_state(DebugState::ViewEnabled)),
        );
    }
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DebugState {
    #[default]
    ViewDisabled,
    ViewEnabled,
}

fn toggle_debug_view(
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<DebugState>>,
    mut next_state: ResMut<NextState<DebugState>>,
) {
    if input.just_pressed(DEBUG_VIEW_TOGGLE_BUTTON) {
        let state_to_set = match **current_state {
            DebugState::ViewDisabled => DebugState::ViewEnabled,
            DebugState::ViewEnabled => DebugState::ViewDisabled,
        };

        next_state.set(state_to_set);
    }
}

fn draw_bot_actions(mut gizmos: Gizmos) {}
