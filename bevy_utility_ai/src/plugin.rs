use crate::define_ai::AddedSystemTracker;
use crate::systems::make_decisions::{make_decisions_sys, EntityActionChangeEvent};
use crate::systems::update_action::{update_actions_sys, UpdateEntityActionInternal};
use crate::AIDefinitions;
use bevy::app::Update;
use bevy::ecs::schedule::{BoxedScheduleLabel, ScheduleLabel};
use bevy::prelude::{IntoSystemConfigs, IntoSystemSetConfig, Plugin, Resource, SystemSet};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum UtililityAISet {
    CalculateInputs,
    MakeDecisions,
    UpdateActions,
}

#[derive(Resource)]
pub(crate) struct UtilityAISettings {
    pub(crate) default_schedule: BoxedScheduleLabel,
}

pub struct UtilityAIPlugin {
    schedule: BoxedScheduleLabel,
}

impl UtilityAIPlugin {
    pub fn new(schedule: impl ScheduleLabel) -> Self {
        UtilityAIPlugin {
            schedule: schedule.dyn_clone(),
        }
    }
}

impl Default for UtilityAIPlugin {
    fn default() -> Self {
        UtilityAIPlugin {
            schedule: Box::new(Update),
        }
    }
}

impl Plugin for UtilityAIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<UpdateEntityActionInternal>()
            .add_event::<EntityActionChangeEvent>()
            .insert_resource(UtilityAISettings {
                default_schedule: self.schedule.dyn_clone(),
            })
            .init_resource::<AIDefinitions>()
            .init_resource::<AddedSystemTracker>()
            .add_systems(
                self.schedule.dyn_clone(),
                (make_decisions_sys, update_actions_sys).in_set(UtililityAISet::MakeDecisions),
            )
            .configure_set(
                self.schedule.dyn_clone(),
                UtililityAISet::CalculateInputs.before(UtililityAISet::MakeDecisions),
            )
            .configure_set(
                self.schedule.dyn_clone(),
                UtililityAISet::MakeDecisions.before(UtililityAISet::UpdateActions),
            );
    }
}
