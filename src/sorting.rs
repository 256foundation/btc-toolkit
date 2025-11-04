use asic_rs::data::miner::MinerData;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl SortDirection {
    /// Returns the opposite direction.
    #[must_use]
    pub const fn toggle(self) -> Self {
        match self {
            Self::Ascending => Self::Descending,
            Self::Descending => Self::Ascending,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortColumn {
    IpAddress,
    Model,
    Make,
    Firmware,
    FirmwareVersion,
}

/// Sorts a slice of miners by the specified column and direction.
///
/// This function performs an in-place sort, modifying the input slice.
pub fn sort_miners_by_column(
    miners: &mut [MinerData],
    column: SortColumn,
    direction: SortDirection,
) {
    match column {
        SortColumn::IpAddress => {
            miners.sort_by(|a, b| compare_with_direction(a.ip, b.ip, direction));
        }
        SortColumn::Model => {
            miners.sort_by(|a, b| {
                let a_model = format!("{}", a.device_info.model);
                let b_model = format!("{}", b.device_info.model);
                compare_with_direction(a_model, b_model, direction)
            });
        }
        SortColumn::Make => {
            miners.sort_by(|a, b| {
                let a_make = format!("{}", a.device_info.make);
                let b_make = format!("{}", b.device_info.make);
                compare_with_direction(a_make, b_make, direction)
            });
        }
        SortColumn::Firmware => {
            miners.sort_by(|a, b| {
                let a_firmware = format!("{}", a.device_info.firmware);
                let b_firmware = format!("{}", b.device_info.firmware);
                compare_with_direction(a_firmware, b_firmware, direction)
            });
        }
        SortColumn::FirmwareVersion => {
            miners.sort_by(|a, b| {
                let a_version = a.firmware_version.as_deref().unwrap_or("");
                let b_version = b.firmware_version.as_deref().unwrap_or("");
                compare_with_direction(a_version, b_version, direction)
            });
        }
    }
}

fn compare_with_direction<T: Ord>(a: T, b: T, direction: SortDirection) -> Ordering {
    match direction {
        SortDirection::Ascending => a.cmp(&b),
        SortDirection::Descending => b.cmp(&a),
    }
}
