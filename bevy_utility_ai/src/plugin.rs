use crate::define_ai::AddedSystemTracker;
use crate::{
    systems::{make_decisions, update_action, UpdateEntityAction},
    AIDefinitions, AITargetEntitySets,
};
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
        app.add_event::<UpdateEntityAction>()
            .init_resource::<AIDefinitions>()
            .init_resource::<AITargetEntitySets>()
            .init_resource::<AddedSystemTracker>()
            .add_system(make_decisions.in_set(UtililityAISet::MakeDecisions))
            .add_system(update_action.in_set(UtililityAISet::UpdateActions))
            .configure_set(
                UtililityAISet::CalculateInputs.before(UtililityAISet::MakeDecisions),
            )
            .configure_set(
                UtililityAISet::MakeDecisions.before(UtililityAISet::UpdateActions),
            );
    }
}
