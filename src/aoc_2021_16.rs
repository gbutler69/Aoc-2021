#[cfg(test)]
mod tests;

pub fn determine_packet_and_subpacket_version_numbers(raw_message: Vec<u8>) -> Vec<u8> {
    Parser::from(raw_message).parse().packet_versions()
}

#[derive(Default, Debug)]
struct Parser {
    bits: Vec<u8>,
}

impl Parser {
    fn parse(&self) -> Message {
        let mut msg = Message::default();
        let mut caret = 0;
        while let Some(packet) = self.next_packet(&mut caret) {
            msg.add(packet);
        }
        msg
    }

    fn next_packet(&self, caret: &mut usize) -> Option<Packet> {
        let (version, packet_type) = self.next_header(caret)?;
        match packet_type {
            PacketType::Literal => self.literal_value(caret, version),
            PacketType::Operator => self.operator(caret, version),
        }
    }

    fn literal_value(&self, caret: &mut usize, version: u8) -> Option<Packet> {
        let mut more = true;
        let mut value = 0;
        while more {
            more = self.next_bit_bool(caret)?;
            value <<= 4;
            value += self.next_bits_u8(caret, 4)? as u64;
        }
        Some(Packet::Literal { version, value })
    }

    fn operator(&self, caret: &mut usize, version: u8) -> Option<Packet> {
        let is_length_subpackets = self.next_bit_bool(caret)?;
        match is_length_subpackets {
            true => self.operator_by_subpackets(caret, version),
            false => self.operator_by_total_length(caret, version),
        }
    }

    fn operator_by_subpackets(&self, caret: &mut usize, version: u8) -> Option<Packet> {
        let num_subpackets = self.next_bits_u64(caret, 11)?;
        let mut sub_packets = Vec::new();
        for _ in 0..num_subpackets {
            let next_packet = self.next_packet(caret)?;
            sub_packets.push(next_packet);
        }
        Some(Packet::Operator {
            version,
            sub_packets,
        })
    }

    fn operator_by_total_length(&self, caret: &mut usize, version: u8) -> Option<Packet> {
        let sub_packets_length = self.next_bits_u64(caret, 15)?;
        let caret_end = *caret + sub_packets_length as usize;
        let mut sub_packets = Vec::new();
        while *caret < caret_end {
            let next_packet = self.next_packet(caret)?;
            sub_packets.push(next_packet);
        }
        Some(Packet::Operator {
            version,
            sub_packets,
        })
    }

    fn next_header(&self, caret: &mut usize) -> Option<(u8, PacketType)> {
        let version = self.next_bits_u8(caret, 3)?;
        let packet_type = PacketType::from(self.next_bits_u8(caret, 3)?);
        Some((version, packet_type))
    }

    fn next_bit_bool(&self, caret: &mut usize) -> Option<bool> {
        self.next_bit_u8(caret).map(|v| v == 0x01)
    }

    fn next_bits_u8(&self, caret: &mut usize, num_bits: u8) -> Option<u8> {
        assert!(num_bits as u32 <= u8::BITS);
        self.next_bits_u64(caret, num_bits).map(|v| v as u8)
    }

    fn next_bits_u64(&self, caret: &mut usize, num_bits: u8) -> Option<u64> {
        assert!(num_bits as u32 <= u64::BITS);
        let mut value = 0;
        for _ in 0..num_bits {
            value <<= 1;
            value |= self.next_bit_u8(caret)? as u64;
        }
        Some(value)
    }

    fn next_bit_u8(&self, caret: &mut usize) -> Option<u8> {
        if *caret >= self.bits.len() * 8 {
            None
        } else {
            let byte = *caret / 8;
            let bit_num = 7 - *caret % 8_usize;
            let bit_value = (self.bits[byte] >> bit_num) & 0x01;
            *caret += 1;
            Some(bit_value)
        }
    }
}

impl From<Vec<u8>> for Parser {
    fn from(raw_message: Vec<u8>) -> Self {
        Self { bits: raw_message }
    }
}

#[derive(Debug, Default)]
struct Message {
    root_packets: Vec<Packet>,
}

impl Message {
    fn packet_versions(&self) -> Vec<u8> {
        self.root_packets
            .iter()
            .flat_map(Packet::versions)
            .collect()
    }

    fn add(&mut self, packet: Packet) {
        self.root_packets.push(packet);
    }

    #[cfg(test)]
    fn root_value(&self) -> Option<u64> {
        self.root_packets
            .iter()
            .filter_map(|p| {
                if let Packet::Literal { value, .. } = *p {
                    Some(value)
                } else {
                    None
                }
            })
            .next()
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum Packet {
    Literal {
        version: u8,
        value: u64,
    },
    Operator {
        version: u8,
        sub_packets: Vec<Packet>,
    },
}

impl Packet {
    fn versions(&self) -> Vec<u8> {
        match self {
            Packet::Literal { version, .. } => vec![*version],
            Packet::Operator {
                version,
                sub_packets,
            } => {
                let mut versions = versions(sub_packets);
                versions.push(*version);
                versions
            }
        }
    }
}

fn versions(packets: &[Packet]) -> Vec<u8> {
    let mut collected_versions = Vec::new();
    for packet in packets {
        match packet {
            Packet::Literal { version, .. } => collected_versions.push(*version),
            Packet::Operator {
                version,
                sub_packets,
            } => {
                collected_versions.append(&mut versions(sub_packets));
                collected_versions.push(*version);
            }
        }
    }
    collected_versions
}

enum PacketType {
    Literal,
    Operator,
}

impl From<u8> for PacketType {
    fn from(packet_type_number: u8) -> Self {
        match packet_type_number {
            4 => PacketType::Literal,
            0..=3 | 5..=7 => PacketType::Operator,
            _ => panic!("Invalid Out-of-Range Packet Type Number"),
        }
    }
}
