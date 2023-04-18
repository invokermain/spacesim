use bevy::prelude::{Entity, World};
use bevy_egui::egui::{
    self,
    plot::{BarChart, Line, Plot, PlotPoints},
    Ui,
};
use egui_extras::{Column, TableBuilder};
use strum::IntoEnumIterator;

use crate::economy::{components::CommodityType, market::Market};

use super::{
    query_layer::{get_planet, get_planet_companies, get_system_info},
    render_structs::{RenderPlanet, RenderSystemInfo},
};

pub(crate) struct PlanetViewState {
    selected_planet: Option<Entity>,
    system_info: RenderSystemInfo,
}

impl Default for PlanetViewState {
    fn default() -> Self {
        Self {
            selected_planet: Default::default(),
            system_info: RenderSystemInfo { planets: vec![] },
        }
    }
}

pub(crate) fn planet_view(ui: &mut Ui, world: &mut World, state: &mut PlanetViewState) {
    state.system_info = get_system_info(world);

    if state.selected_planet.is_none() {
        state.selected_planet = state.system_info.planets.first().copied();
    }

    ui.horizontal(|ui| {
        state.system_info.planets.iter().for_each(|planet| {
            ui.selectable_value(
                &mut state.selected_planet,
                Some(*planet),
                format!("{:?}", planet),
            );
        })
    });
    ui.end_row();

    if let Some(planet_id) = state.selected_planet {
        let planet = get_planet(world, planet_id);
        render_planet(ui, world, &planet);
    }
}

pub(crate) fn render_planet(ui: &mut Ui, world: &mut World, planet: &RenderPlanet) {
    ui.strong(format!("Planet {:?}", planet.entity));
    ui.separator();
    ui.strong("MARKET");

    render_market(ui, &planet.market);

    ui.separator();

    for company in get_planet_companies(planet.entity, world).iter() {
        ui.label(format!("Company {:?}", company.entity));
        ui.label(format!("wealth: {:.1}", company.wealth));
        ui.label(format!("storage: {:?}", company.commodity_storage));
    }
}

pub(crate) fn render_market(ui: &mut Ui, market: &Market) {
    ui.columns(3, |columns| {
        columns.iter_mut().for_each(|col| col.set_max_height(100.0));

        render_market_table(&mut columns[0], market);

        let plot = Plot::new("market_production_plot");
        plot.show(&mut columns[1], |plot_ui| {
            plot_ui.line(Line::new(
                market
                    .production_history
                    .iter()
                    .map(|f| [f.0 as f64, f.1.units as f64])
                    .collect::<PlotPoints>(),
            ));
        });

        let plot = Plot::new("market_consumption_plot");
        plot.show(&mut columns[2], |plot_ui| {
            plot_ui.bar_chart(BarChart::new(
                market
                    .consumption_history
                    .iter()
                    .map(|f| [f.0 as f64, f.1.units as f64])
                    .collect::<PlotPoints>(),
            ));
        });
    });

    market
        .transaction_history
        .iter()
        .take(10)
        .for_each(|transaction| {
            ui.label(format!("{:.2} | {:?}", transaction.0, transaction.1));
        });
}

pub(crate) fn render_market_table(ui: &mut Ui, market: &Market) {
    let table = TableBuilder::new(ui)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .columns(Column::auto(), 4)
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
                ui.strong("Pressure");
            });
            header.col(|ui| {
                ui.strong("Price Modifier");
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
                    row.col(|ui| {
                        ui.label(format!(
                            "{:.1}",
                            market.supply_pressure[commodity_type as usize]
                                - market.demand_pressure[commodity_type as usize]
                        ));
                    });
                    row.col(|ui| {
                        ui.label(format!(
                            "{:.0}%",
                            market.demand_price_modifier[commodity_type as usize] * 100.0
                        ));
                    });
                });
            }
        });
}
