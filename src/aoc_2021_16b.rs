#[cfg(test)]
mod tests;

pub fn determine_numeric_result_of_packet_evaluation(raw_message: Vec<u8>) -> u64 {
    Parser::from(raw_message).parse().evaluate()
}

#[derive(Default, Debug)]
struct Parser {
    bits: Vec<u8>,
}

impl Parser {
    fn parse(&self) -> Message {
        let mut caret = 0;
        if let Some(packet) = self.next_packet(&mut caret) {
            return Message::from(packet);
        }
        Message::default()
    }

    fn next_packet(&self, caret: &mut usize) -> Option<Packet> {
        let (version, packet_type) = self.next_header(caret)?;
        match packet_type {
            PacketType::Sum => self.sum(caret, version),
            PacketType::Product => self.product(caret, version),
            PacketType::Min => self.min(caret, version),
            PacketType::Max => self.max(caret, version),
            PacketType::Literal => self.literal_value(caret, version),
            PacketType::GreaterThan => self.greater_than(caret, version),
            PacketType::LessThan => self.less_than(caret, version),
            PacketType::EqualTo => self.equal_to(caret, version),
        }
    }

    fn next_header(&self, caret: &mut usize) -> Option<(u8, PacketType)> {
        let version = self.next_bits_u8(caret, 3)?;
        let packet_type = PacketType::from(self.next_bits_u8(caret, 3)?);
        Some((version, packet_type))
    }

    fn sum(&self, caret: &mut usize, version: u8) -> Option<Packet> {
        let sub_packets = self.subpackets(caret);
        if sub_packets.is_empty() {
            None
        } else {
            Some(Packet::Sum {
                version,
                sub_packets,
            })
        }
    }

    fn product(&self, caret: &mut usize, version: u8) -> Option<Packet> {
        let sub_packets = self.subpackets(caret);
        if sub_packets.is_empty() {
            None
        } else {
            Some(Packet::Product {
                version,
                sub_packets,
            })
        }
    }

    fn min(&self, caret: &mut usize, version: u8) -> Option<Packet> {
        let sub_packets = self.subpackets(caret);
        if sub_packets.is_empty() {
            None
        } else {
            Some(Packet::Min {
                version,
                sub_packets,
            })
        }
    }

    fn max(&self, caret: &mut usize, version: u8) -> Option<Packet> {
        let sub_packets = self.subpackets(caret);
        if sub_packets.is_empty() {
            None
        } else {
            Some(Packet::Max {
                version,
                sub_packets,
            })
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

    fn greater_than(&self, caret: &mut usize, version: u8) -> Option<Packet> {
        let sub_packets = self.subpackets(caret);
        if sub_packets.len() != 2 {
            None
        } else {
            Some(Packet::GreaterThan {
                version,
                left_packet: Box::new(sub_packets[0].clone()),
                right_packet: Box::new(sub_packets[1].clone()),
            })
        }
    }

    fn less_than(&self, caret: &mut usize, version: u8) -> Option<Packet> {
        let sub_packets = self.subpackets(caret);
        if sub_packets.len() != 2 {
            None
        } else {
            Some(Packet::LessThan {
                version,
                left_packet: Box::new(sub_packets[0].clone()),
                right_packet: Box::new(sub_packets[1].clone()),
            })
        }
    }

    fn equal_to(&self, caret: &mut usize, version: u8) -> Option<Packet> {
        let sub_packets = self.subpackets(caret);
        if sub_packets.len() != 2 {
            None
        } else {
            Some(Packet::EqualTo {
                version,
                left_packet: Box::new(sub_packets[0].clone()),
                right_packet: Box::new(sub_packets[1].clone()),
            })
        }
    }

    fn subpackets(&self, caret: &mut usize) -> Vec<Packet> {
        let is_length_subpackets = self.next_bit_bool(caret);
        match is_length_subpackets {
            Some(true) => self.subpackets_by_total_number(caret),
            Some(false) => self.subpackets_by_total_length(caret),
            _ => Vec::new(),
        }
    }

    fn subpackets_by_total_number(&self, caret: &mut usize) -> Vec<Packet> {
        if let Some(num_subpackets) = self.next_bits_u64(caret, 11) {
            let mut sub_packets = Vec::new();
            for _ in 0..num_subpackets {
                if let Some(next_packet) = self.next_packet(caret) {
                    sub_packets.push(next_packet);
                } else {
                    return Vec::new();
                }
            }
            sub_packets
        } else {
            Vec::new()
        }
    }

    fn subpackets_by_total_length(&self, caret: &mut usize) -> Vec<Packet> {
        if let Some(sub_packets_length) = self.next_bits_u64(caret, 15) {
            let caret_end = *caret + sub_packets_length as usize;
            let mut sub_packets = Vec::new();
            while *caret < caret_end {
                if let Some(next_packet) = self.next_packet(caret) {
                    sub_packets.push(next_packet);
                } else {
                    return Vec::new();
                }
            }
            sub_packets
        } else {
            Vec::new()
        }
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
    root_packet: Option<Packet>,
}

impl Message {
    fn evaluate(&self) -> u64 {
        self.root_packet
            .iter()
            .map(|packet| packet.value())
            .next()
            .unwrap()
    }
}

impl From<Packet> for Message {
    fn from(packet: Packet) -> Self {
        Self {
            root_packet: Some(packet),
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Packet {
    Sum {
        version: u8,
        sub_packets: Vec<Packet>,
    },
    Product {
        version: u8,
        sub_packets: Vec<Packet>,
    },
    Min {
        version: u8,
        sub_packets: Vec<Packet>,
    },
    Max {
        version: u8,
        sub_packets: Vec<Packet>,
    },
    Literal {
        version: u8,
        value: u64,
    },
    GreaterThan {
        version: u8,
        left_packet: Box<Packet>,
        right_packet: Box<Packet>,
    },
    LessThan {
        version: u8,
        left_packet: Box<Packet>,
        right_packet: Box<Packet>,
    },
    EqualTo {
        version: u8,
        left_packet: Box<Packet>,
        right_packet: Box<Packet>,
    },
}

impl Packet {
    fn value(&self) -> u64 {
        match self {
            Packet::Sum { sub_packets, .. } => {
                sub_packets.iter().map(|packet| packet.value()).sum()
            }
            Packet::Product { sub_packets, .. } => {
                sub_packets.iter().map(|packet| packet.value()).product()
            }
            Packet::Min { sub_packets, .. } => sub_packets
                .iter()
                .map(|packet| packet.value())
                .min()
                .unwrap_or(0),
            Packet::Max { sub_packets, .. } => sub_packets
                .iter()
                .map(|packet| packet.value())
                .max()
                .unwrap_or(0),
            Packet::Literal { value, .. } => *value,
            Packet::GreaterThan {
                left_packet,
                right_packet,
                ..
            } => {
                if left_packet.value() > right_packet.value() {
                    1
                } else {
                    0
                }
            }
            Packet::LessThan {
                left_packet,
                right_packet,
                ..
            } => {
                if left_packet.value() < right_packet.value() {
                    1
                } else {
                    0
                }
            }
            Packet::EqualTo {
                left_packet,
                right_packet,
                ..
            } => {
                if left_packet.value() == right_packet.value() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

enum PacketType {
    Literal,
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl From<u8> for PacketType {
    fn from(packet_type_number: u8) -> Self {
        match packet_type_number {
            0 => PacketType::Sum,
            1 => PacketType::Product,
            2 => PacketType::Min,
            3 => PacketType::Max,
            4 => PacketType::Literal,
            5 => PacketType::GreaterThan,
            6 => PacketType::LessThan,
            7 => PacketType::EqualTo,
            _ => panic!("Invalid Out-of-Range Packet Type Number"),
        }
    }
}
