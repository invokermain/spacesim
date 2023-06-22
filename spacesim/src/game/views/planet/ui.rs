use bevy::prelude::{Entity, Query, ResMut, State, With};
use bevy_egui::egui::{
    self,
    plot::{Legend, Line, Plot, PlotPoints},
    Ui,
};
use bevy_egui::EguiContexts;
use egui_extras::{Column, TableBuilder};
use spacesim_simulation::common::components::Name;
use spacesim_simulation::common::marker_components::{IsCompany, IsPlanet};
use spacesim_simulation::economy::components::{CommodityStorage, Wealth};
use spacesim_simulation::planet::components::Companies;
use strum::IntoEnumIterator;

use spacesim_simulation::economy::{
    commodity_type::{CommodityArr, CommodityType},
    market::Market,
};

use crate::game::state::GameViewState;
use crate::game::ui::widgets::view_selector::widget_view_selector;

use super::PlanetViewState;

pub(crate) fn planet_ui(
    mut q_egui_ctx: EguiContexts,
    q_planets: Query<(Entity, &Market, &Name, &Companies), With<IsPlanet>>,
    q_companies: Query<(&CommodityStorage, &Wealth), With<IsCompany>>,
    mut r_game_view_state: ResMut<State<GameViewState>>,
    mut r_planet_view_state: ResMut<PlanetViewState>,
) {
    if r_planet_view_state.selected_planet.is_none() {
        r_planet_view_state.selected_planet = q_planets.iter().next().map(|x| x.0);
    }

    let game_view_state = r_game_view_state.as_mut();

    widget_view_selector(q_egui_ctx.ctx_mut(), &mut game_view_state.0);

    egui::CentralPanel::default().show(q_egui_ctx.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            q_planets.iter().for_each(|res| {
                ui.selectable_value(
                    &mut r_planet_view_state.selected_planet,
                    Some(res.0),
                    res.2.value.to_string(),
                );
            })
        });
        ui.end_row();

        if let Some(planet_id) = r_planet_view_state.selected_planet {
            let (_, market, _, companies) = q_planets.get(planet_id).unwrap();
            ui.separator();
            ui.strong("MARKET");

            render_market(ui, market);

            ui.separator();

            for company_id in &companies.value {
                let (c_commodity_storage, c_wealth) = q_companies.get(*company_id).unwrap();
                ui.label(format!("Company {:?}", company_id));
                ui.label(format!("wealth: {:.1}", c_wealth.value));
                render_commodity_storage(ui, &c_commodity_storage.storage);
            }
        }
    });
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
                            ui.label(format!(
                                "{:.2}",
                                commodity_storage[commodity_type as usize]
                            ));
                        });
                    }
                });
            })
    });
}

pub(crate) fn render_market(ui: &mut Ui, market: &Market) {
    render_market_table(ui, market);

    ui.separator();

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
