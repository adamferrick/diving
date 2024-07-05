use bevy::prelude::*;

pub mod circulation;
pub mod inhalation;

use crate::circulation::*;
use crate::respiration::decompression::GasExchangeInLungs;
use crate::respiration::equalization::BloodstreamPressure;
use crate::respiration::nitrogen::NitrogenHazard;
use crate::respiration::oxygen::OxygenHazard;
use crate::respiration::usage::GasUsageRate;
use crate::Depth;
use inhalation::*;

#[derive(Bundle, Default)]
pub struct BreatherBundle {
    pub depth: Depth,
    pub bloodstream_content: BloodstreamContent,
    pub bloodstream_pressure: BloodstreamPressure,
    pub gas_usage_rate: GasUsageRate,
    pub oxygen_hazard: OxygenHazard,
    pub nitrogen_hazard: NitrogenHazard,
    pub gas_exchange_in_lungs: GasExchangeInLungs,
}

pub fn respiration_plugin(app: &mut App) {
    app.add_plugins((inhalation_plugin, circulation_plugin));
}
