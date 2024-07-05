use bevy::prelude::*;

pub mod circulation;
pub mod inhalation;

use crate::circulation::*;
use crate::Depth;
use crate::respiration::equalization::BloodstreamPressure;
use crate::respiration::usage::GasUsageRate;
use crate::respiration::oxygen::OxygenHazard;
use crate::respiration::nitrogen::NitrogenHazard;
use crate::respiration::decompression::GasExchangeInLungs;
use inhalation::*;

#[derive(Bundle, Default)]
pub struct BreatherBundle {
    depth: Depth,
    bloodstream_content: BloodstreamContent,
    bloodstream_pressure: BloodstreamPressure,
    gas_usage_rate: GasUsageRate,
    oxygen_hazard: OxygenHazard,
    nitrogen_hazard: NitrogenHazard,
    gas_exchange_in_lungs: GasExchangeInLungs,
}

pub fn respiration_plugin(app: &mut App) {
    app.add_plugins((inhalation_plugin, circulation_plugin));
}
