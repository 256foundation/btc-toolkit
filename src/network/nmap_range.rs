use std::net::Ipv4Addr;

// Parses a NMAP Network Range `192.6.1-8.1-50`
pub fn parse_nmap_range(range_str: &str) -> Vec<Ipv4Addr> {
    let mut result = Vec::new();

    // Split into octets
    let octets: Vec<&str> = range_str.split('.').collect();
    if octets.len() != 4 {
        return result; // Invalid format
    }

    // Parse each octet into a list of values
    let mut octet_values: Vec<Vec<u8>> = Vec::new();
    for octet in octets {
        let mut values = Vec::new();

        // Split by comma if multiple ranges/values
        for part in octet.split(',') {
            if part.contains('-') {
                // Handle range like "1-8"
                let range_parts: Vec<&str> = part.split('-').collect();
                if range_parts.len() == 2 {
                    if let (Ok(start), Ok(end)) =
                        (range_parts[0].parse::<u8>(), range_parts[1].parse::<u8>())
                    {
                        for i in start..=end {
                            values.push(i);
                        }
                    }
                }
            } else {
                // Handle single value
                if let Ok(val) = part.parse::<u8>() {
                    values.push(val);
                }
            }
        }

        octet_values.push(values);
    }

    // Generate all combinations
    generate_ip_addresses(&mut result, &octet_values, 0, [0, 0, 0, 0]);

    result
}

// Recursive function to generate all IP combinations
fn generate_ip_addresses(
    result: &mut Vec<Ipv4Addr>,
    octet_values: &[Vec<u8>],
    depth: usize,
    mut current: [u8; 4],
) {
    if depth == 4 {
        result.push(Ipv4Addr::new(
            current[0], current[1], current[2], current[3],
        ));
        return;
    }

    for &value in &octet_values[depth] {
        current[depth] = value;
        generate_ip_addresses(result, octet_values, depth + 1, current);
    }
}
