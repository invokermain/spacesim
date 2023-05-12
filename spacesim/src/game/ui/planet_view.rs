use bevy::prelude::{Entity, World};
use bevy_egui::egui::{
    self,
    plot::{Legend, Line, Plot, PlotPoints},
    Ui,
};
use egui_extras::{Column, TableBuilder};
use strum::IntoEnumIterator;

use spacesim_simulation::economy::{
    commodity_type::{CommodityArr, CommodityType},
    market::Market,
};

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
        render_commodity_storage(ui, &company.commodity_storage);
    }
}

pub(crate) fn render_commodity_storage(ui: &mut Ui, commodity_storage: &CommodityArr<f32>) {
    let id = ui.next_auto_id();
    ui.skip_ahead_auto_ids(1);

    ui.push_id(id, |ui| {
        let table = TableBuilder::new(ui)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .columns(Column::auto(), CommodityType::iter().len())
            .min_scrolled_height(0.0);

        table
            .header(20.0, |mut header| {
                CommodityType::iter().for_each(|commodity_type| {
                    header.col(|ui| {
                        ui.strong(format!("{:?}", commodity_type));
                    });
                });
            })
            .body(|mut body| {
                body.row(20.0, |mut row| {
                    for commodity_type in CommodityType::iter() {
                        row.col(|ui| {
                            ui.label(format!("{:.2}", commodity_storage[commodity_type as usize]));
                        });
                    }
                });
            })
    });
}

pub(crate) fn render_market(ui: &mut Ui, market: &Market) {
    render_market_table(ui, market);

    ui.columns(2, |ui_col| {
        // LEFT
        ui_col[0].label("Commodity Supply");
        Plot::new("market_supply_plot")
            .height(250.0)
            .legend(Legend::default())
            .show(&mut ui_col[0], |plot_ui| {
                for commodity_type in CommodityType::iter() {
                    plot_ui.line(
                        Line::new(PlotPoints::from_iter(
                            market.total_supply_history[commodity_type as usize]
                                .iter()
                                .rev()
                                .enumerate()
                                .map(|(x, total_supply)| [x as f64, *total_supply as f64]),
                        ))
                        .name(commodity_type),
                    );
                }
            });

        ui_col[0].label("Market Pressure");
        Plot::new("market_pressure_plot")
            .height(250.0)
            .legend(Legend::default())
            .show(&mut ui_col[0], |plot_ui| {
                for commodity_type in CommodityType::iter() {
                    plot_ui.line(
                        Line::new(PlotPoints::from_iter(
                            market.supply_history[commodity_type as usize]
                                .iter()
                                .zip(&market.demand_history[commodity_type as usize])
                                .rev()
                                .enumerate()
                                .map(|(x, (supply, demand))| {
                                    [x as f64, (*supply - *demand) as f64]
                                }),
                        ))
                        .name(commodity_type),
                    );
                }
            });

        // RIGHT
        ui_col[1].label("Commodity Purchase Price");
        Plot::new("market_purchase_price_plot")
            .height(250.0)
            .legend(Legend::default())
            .show(&mut ui_col[1], |plot_ui| {
                for commodity_type in CommodityType::iter() {
                    plot_ui.line(
                        Line::new(PlotPoints::from_iter(
                            market.purchase_price_history[commodity_type as usize]
                                .iter()
                                .rev()
                                .enumerate()
                                .map(|(x, price)| [x as f64, *price as f64]),
                        ))
                        .name(commodity_type),
                    );
                }
            });

        ui_col[1].label("Commodity Sale Price");
        Plot::new("market_sale_price_plot")
            .height(250.0)
            .legend(Legend::default())
            .show(&mut ui_col[1], |plot_ui| {
                for commodity_type in CommodityType::iter() {
                    plot_ui.line(
                        Line::new(PlotPoints::from_iter(
                            market.sale_price_history[commodity_type as usize]
                                .iter()
                                .rev()
                                .enumerate()
                                .map(|(x, price)| [x as f64, *price as f64]),
                        ))
                        .name(commodity_type),
                    );
                }
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
                ui.strong("Delta");
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
                            "{:.2}",
                            market.demand_price_modifier[commodity_type as usize] // * 100.0
                        ));
                    });
                });
            }
        });
}
