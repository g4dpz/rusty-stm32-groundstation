use defmt::*;

// declare a track function with an enabled boolean parameter
pub fn parse(a: heapless::String<64>) {
    debug!("parsing: {:?}", a);
    let mut iter = a.split('|');
    match iter.next() {
        Some("A") => {
        }
        Some("B") => {
        }
        Some("C") => {
        }
        Some("D") => {
        }
        Some("E") => {
        }
        Some("F") => {
        }
        Some("G") => {
        }
        Some("H") => {
        }
        Some("I") => {
        }
        Some("J") => {
        }
        Some("K") => {
        }
        Some("L") => {
            // print all the remaining elements
            debug!("Location changed");
            for element in iter {
                debug!("element: {:?}", element);
            }
        }
        Some("M") => {
        }
        Some("N") => {
        }
        Some("O") => {
        }
        Some("P") => {
        }
        Some("Q") => {
        }
        Some("R") => {
        }
        Some("S") => {
        }
        Some("T") => {
        }
        Some("U") => {
        }
        Some("V") => {
        }
        Some("W") => {
        }
        Some("X") => {
        }
        Some("Y") => {
        }
        Some("Z") => {
        }
        Some(&_) => {
            debug!("Unknown command");
        }
        None => {
            debug!("No command");
        }
    }
}

