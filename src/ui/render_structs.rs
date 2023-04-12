use crate::economy::components::CommodityArr;

#[derive(Debug)]
pub struct RenderCompany {
    pub wealth: f32,
    pub commodity_storage: CommodityArr<f32>,
}
