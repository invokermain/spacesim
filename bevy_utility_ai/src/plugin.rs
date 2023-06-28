use crate::define_ai::AddedSystemTracker;
use crate::systems::make_decisions::{make_decisions_sys, EntityActionChangeEvent};
use crate::systems::update_action::{update_actions_sys, UpdateEntityActionInternal};
use crate::AIDefinitions;
use bevy::prelude::{IntoSystemConfig, IntoSystemSetConfig, Plugin, SystemSet};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum UtililityAISet {
    CalculateInputs,
    MakeDecisions,
    UpdateActions,
}

pub struct UtilityAIPlugin;

impl Plugin for UtilityAIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<UpdateEntityActionInternal>()
            .add_event::<EntityActionChangeEvent>()
            .init_resource::<AIDefinitions>()
            .init_resource::<AddedSystemTracker>()
            .add_system(make_decisions_sys.in_set(UtililityAISet::MakeDecisions))
            .add_system(update_actions_sys.in_set(UtililityAISet::UpdateActions))
            .configure_set(
                UtililityAISet::CalculateInputs.before(UtililityAISet::MakeDecisions),
            )
            .configure_set(
                UtililityAISet::MakeDecisions.before(UtililityAISet::UpdateActions),
            );
    }
}
