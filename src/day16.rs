use std::panic;
use std::convert::TryFrom;


struct Packet {
    version: u8,
    packet_data: PacketData,
}
impl Packet {
    fn evaluate(&self) -> u64 {
        match &self.packet_data {
            PacketData::Literal { data } => *data,
            PacketData::Operator { operation, subpackets } => {
                let mut subpacket_data_iter = subpackets
                    .iter()
                    .map(Packet::evaluate);

                panic::catch_unwind(|| {
                    match operation {
                        Operation::Sum => subpacket_data_iter.sum(),
                        Operation::Product => subpacket_data_iter.product(),
                        Operation::Min => subpacket_data_iter.min().unwrap(),
                        Operation::Max => subpacket_data_iter.max().unwrap(),
                        op => {
                            let first = subpacket_data_iter.next().unwrap();
                            let second = subpacket_data_iter.next().unwrap();
                            assert!(subpacket_data_iter.next().is_none());
                            let satisfied = match op {
                                Operation::Greater => first > second,
                                Operation::Less => first < second,
                                Operation::Eq  => first == second,
                                _ => panic!(),
                            };
                            if satisfied { 1 } else { 0 }
                        }
                    }
                }).unwrap_or_else(|_| panic!("incorrect number of subpackets to perform {:?}", operation))
            },
        }
    }
}

#[derive(Debug)]
enum Operation {
    Sum, Product, Min, Max, Greater, Less, Eq
}
impl TryFrom<u8> for Operation {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Operation::Sum),
            1 => Ok(Operation::Product),
            2 => Ok(Operation::Min),
            3 => Ok(Operation::Max),
            5 => Ok(Operation::Greater),
            6 => Ok(Operation::Less),
            7 => Ok(Operation::Eq),
            _ => Err(()),
        }
    }
}

enum PacketData {
    Literal {
        data: u64
    },
    Operator {
        operation: Operation,
        subpackets: Vec<Packet>,
    },
}

#[derive(Clone, Copy)]
struct BitString<'a> {
    str: &'a str,
    offset: u8,
    version_number_sum: u16,
}
impl<'a> BitString<'a> {
    fn advance_by(&mut self, offset: usize) {
        let total_offset = (self.offset as usize) + offset;
        self.offset = (total_offset % 4) as u8;
        self.str = &self.str[(total_offset / 4)..];
    }

    fn peek_bits_with_u16(&mut self, num_of_bits: u8) -> Result<u16, char> {
        assert!(num_of_bits < 17, "num_of_bits must be at most 16");
        if num_of_bits == 0 { return Ok(0); }
        let len = 1 + (self.offset + num_of_bits - 1) / 4;  // round-up integer division
        let mask = 0xFFFFFFFF >> (32 - num_of_bits);
        let bits = (u32::from_str_radix(&self.str[0..(len as usize)], 16)
            .map_err(|_| {
                (&self.str[0..5]).chars().find(|c| {
                    !matches!(c, 'A'..='F' | '0'..='9')
                })
                .unwrap()
            })?
            >> (4*len - num_of_bits - self.offset))
            & mask;
        Ok(bits as u16)
    }

    fn cursor_eq(&self, other: &Self) -> bool {
        (self.str.as_ptr() == other.str.as_ptr()) && (self.offset == other.offset)
    }
}

impl<'a> TryFrom<&mut BitString<'a>> for Packet {
    type Error = char;
    fn try_from(value: &mut BitString<'a>) -> Result<Self, Self::Error> {
        let (version, type_id, length_type_id) = parse_header(value)?;
        let packet_data = parse_packet_data(value, type_id, length_type_id)?;
        value.version_number_sum += version as u16;
        Ok(Packet {
            version, packet_data
        })
    }
}

fn parse_header(input: &mut BitString) -> Result<(u8, u8, u8), char> {
    const ONE_BIT_MASK: u16 = 0x1;
    const THREE_BIT_MASK: u16 = 0x7;
    let bits = input.peek_bits_with_u16(7)?;

    let version = (bits >> 4) & THREE_BIT_MASK;
    let type_id = (bits >> 1) & THREE_BIT_MASK;
    let type_length_id = bits & ONE_BIT_MASK;
    if type_id == 4 { input.advance_by(6); }
    else { input.advance_by(7); }
    Ok((version as u8, type_id as u8, type_length_id as u8))
}

fn parse_packet_data(input: &mut BitString, type_id: u8, length_type_id: u8) -> Result<PacketData, char> {
    if type_id == 4 {
        return parse_literal_data(input);
    }
    if length_type_id == 0 {
        return parse_operator_data_mode0(input, type_id)
    }
    parse_operator_data_mode1(input, type_id)
}

fn parse_literal_data(input: &mut BitString) -> Result<PacketData, char> {
    let mut still_ongoing = true;
    let mut counter = 0;
    let mut data: u64 = 0;

    while still_ongoing {
        assert!(counter != 16, "more than 16 hex digits appeared in some packet data \
            (17th digit at {{ {:?}, offset: {} }}", input.str, input.offset);

        let bits = input.peek_bits_with_u16(5)?;
        input.advance_by(5);
        data <<= 4;
        data |= (bits & 0xF) as u64;

        still_ongoing = (bits >> 4) == 1;
        counter += 1;
    }
    Ok(PacketData::Literal { data })
}

fn parse_operator_data_mode0(input: &mut BitString, type_id: u8) -> Result<PacketData, char> {
    let num_of_bits = input.peek_bits_with_u16(15)?;
    input.advance_by(15);
    let mut target = *input;
    target.advance_by(num_of_bits as usize);

    let mut subpackets = Vec::new();
    while !input.cursor_eq(&target) {
        subpackets.push(input.try_into()?);
    }
    Ok(PacketData::Operator { operation: type_id.try_into().unwrap(), subpackets })
}
fn parse_operator_data_mode1(input: &mut BitString, type_id: u8) -> Result<PacketData, char> {
    let num_of_packets = input.peek_bits_with_u16(11)?;
    input.advance_by(11);

    let mut subpackets = Vec::with_capacity(num_of_packets as usize);
    for _ in 0..num_of_packets {
        subpackets.push(input.try_into()?);
    }
    Ok(PacketData::Operator { operation: type_id.try_into().unwrap(), subpackets })
}


pub fn day16_main(file_data: &str) -> (u16, u64) {
    let mut bit_string = BitString {
        str: file_data,
        offset: 0,
        version_number_sum: 0
    };
    let packet: Packet = (&mut bit_string).try_into().unwrap_or_else(|c| {
        panic!("invalid hex digit '{}' found in packet", c);
    });
    let packet_value = packet.evaluate();
    println!("[Part 1] The sum of all packet version numbers is {}.", bit_string.version_number_sum);
    println!("[Part 2] The packet evaluates to {}.", packet_value);

    (bit_string.version_number_sum, packet_value)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_sum_test() {
        assert_eq!(day16_main("D2FE28").0, 6);
        assert_eq!(day16_main("38006F45291200").0, 1+6+2);
        assert_eq!(day16_main("EE00D40C823060").0, 7+2+4+1);
        assert_eq!(day16_main("8A004A801A8002F478").0, 16);
        assert_eq!(day16_main("620080001611562C8802118E34").0, 12);
        assert_eq!(day16_main("C0015000016115A2E0802F182340").0, 23);
        assert_eq!(day16_main("A0016C880162017C3686B18A3D4780").0, 31);
    }

    #[test]
    fn packet_value_test() {
        assert_eq!(day16_main("C200B40A82").1, 3);
        assert_eq!(day16_main("04005AC33890").1, 54);
        assert_eq!(day16_main("880086C3E88112").1, 7);
        assert_eq!(day16_main("CE00C43D881120").1, 9);
        assert_eq!(day16_main("D8005AC2A8F0").1, 1);
        assert_eq!(day16_main("F600BC2D8F").1, 0);
        assert_eq!(day16_main("9C005AC2F8F0").1, 0);
        assert_eq!(day16_main("9C0141080250320F1802104A08").1, 1);
    }

}