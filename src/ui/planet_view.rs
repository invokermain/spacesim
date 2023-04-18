use bevy::prelude::{Entity, World};
use bevy_egui::egui::{self, Ui};
use egui_extras::{Column, TableBuilder};
use strum::IntoEnumIterator;

use crate::economy::{components::CommodityType, market::Market};

use super::query_layer::{get_planet_companies, get_system_planets};

#[derive(Default)]
pub(crate) struct PlanetViewState {
    selected: Option<Entity>,
}

pub(crate) fn planet_view(ui: &mut Ui, world: &mut World) {
    let planets = get_system_planets(world);
    render_planet(ui, world, planets[0].entity);
}

pub(crate) fn render_planet(ui: &mut Ui, world: &mut World, planet_id: Entity) {
    ui.strong(format!("Planet {:?}", planet_id));
    ui.separator();
    ui.strong("MARKET");

    ui.separator();

    for company in get_planet_companies(planet_id, world).iter() {
        ui.label(format!("Company {:?}", company.entity));
        ui.label(format!("wealth: {:.1}", company.wealth));
        ui.label(format!("storage: {:?}", company.commodity_storage));
    }
}

pub(crate) fn render_market(ui: &mut Ui, market: &Market) {
    let table = TableBuilder::new(ui)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .columns(Column::auto(), 3)
        .min_scrolled_height(0.0);

    table
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.strong("Commodity");
            });
            header.col(|ui| {
                ui.strong("Supply");
            });
            header.col(|ui| {
                ui.strong("Demand");
            });
        })
        .body(|mut body| {
            for commodity_type in CommodityType::iter() {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label(format!("{:?}", commodity_type));
                    });
                    row.col(|ui| {
                        ui.label(format!(
                            "{:.1}",
                            market.total_supply[commodity_type as usize]
                        ));
                    });
                });
            }
        });

    ui.label(format!("supply_pressure: {:?}", market.supply_pressure));
    ui.label(format!("demand_pressure: {:?}", market.demand_pressure));
    ui.label(format!(
        "price_modifier: {:?}",
        market.demand_price_modifier
    ));

    market
        .transaction_history
        .iter()
        .take(10)
        .for_each(|transaction| {
            ui.label(format!("{:.2} | {:?}", transaction.0, transaction.1));
        });
}
