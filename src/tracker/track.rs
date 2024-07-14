use defmt::*;

// declare a track function with an enabled boolean parameter
pub fn track(enabled: bool) {
    // if enabled is true, log a message
    if enabled {
        debug!("tracking enabled");
    } else {
        // if enabled is false, log a different message
        debug!("tracking disabled");
    }
}