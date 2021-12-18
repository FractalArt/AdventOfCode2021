//! # Advent of Code 2021 - Day 16
//!
//! This module contains the solution of the [sixteenth day's challenges](https://adventofcode.com/2021/day/16).
use itertools::Itertools;
use std::borrow::Borrow;

/// Packets can be either of type `Literal` or `Operator`, where operators contain sub-packages.
#[derive(Debug, PartialEq)]
enum PacketType {
    Literal(usize),
    Operator(Vec<Packet>),
}

/// A package has a `version`, a `type_id` and a `package_type`.
#[derive(Debug, PartialEq)]
struct Packet {
    version: usize,
    type_id: usize,
    package_type: PacketType,
}

/// Convert the hexadecimal input format to binary.
fn convert_hex_to_binary(hex: &str) -> Vec<char> {
    hex.chars()
        .flat_map(|c| match c {
            '0' => ['0', '0', '0', '0'],
            '1' => ['0', '0', '0', '1'],
            '2' => ['0', '0', '1', '0'],
            '3' => ['0', '0', '1', '1'],
            '4' => ['0', '1', '0', '0'],
            '5' => ['0', '1', '0', '1'],
            '6' => ['0', '1', '1', '0'],
            '7' => ['0', '1', '1', '1'],
            '8' => ['1', '0', '0', '0'],
            '9' => ['1', '0', '0', '1'],
            'A' => ['1', '0', '1', '0'],
            'B' => ['1', '0', '1', '1'],
            'C' => ['1', '1', '0', '0'],
            'D' => ['1', '1', '0', '1'],
            'E' => ['1', '1', '1', '0'],
            _ => ['1', '1', '1', '1'],
        })
        .collect()
}

/// Parse a literal type packet.
fn parse_literal(start_index: usize, binary: &[char]) -> (Packet, usize) {
    let version =
        usize::from_str_radix(&binary[start_index..start_index + 3].iter().join(""), 2).unwrap();
    let type_id =
        usize::from_str_radix(&binary[start_index + 3..start_index + 6].iter().join(""), 2)
            .unwrap();

    let mut index = start_index + 6;
    let mut number = vec![];

    loop {
        if binary[index] == '1' {
            (index + 1..=index + 4).for_each(|i| number.push(binary[i]));
            index += 5;
        } else {
            (index + 1..=index + 4).for_each(|i| number.push(binary[i]));
            index += 5;
            break;
        }
    }

    let number = usize::from_str_radix(&number.iter().join(""), 2).unwrap();

    (
        Packet {
            version,
            type_id,
            package_type: PacketType::Literal(number),
        },
        index,
    )
}

/// Parse an operator packet with length-type id 0.
fn parse_operator_0(start_index: usize, binary: &[char]) -> (Packet, usize) {
    let version =
        usize::from_str_radix(&binary[start_index..start_index + 3].iter().join(""), 2).unwrap();
    let type_id =
        usize::from_str_radix(&binary[start_index + 3..start_index + 6].iter().join(""), 2)
            .unwrap();

    debug_assert_eq!(binary[start_index + 6], '0');

    // Next 15 bits indicate the total length of subpackets
    let n_bits = usize::from_str_radix(
        &binary[start_index + 7..start_index + 22].iter().join(""),
        2,
    )
    .unwrap();

    let next_package_start_index = start_index + 22 + n_bits;

    let mut index = start_index + 22;
    let mut sub_packets = vec![];

    loop {
        let (packet, next_index) = parse_packet(index, binary);
        sub_packets.push(packet);
        index = next_index;
        if next_index == next_package_start_index {
            break;
        }
    }

    (
        Packet {
            version,
            type_id,
            package_type: PacketType::Operator(sub_packets),
        },
        next_package_start_index,
    )
}

/// Parse an operator packet with length-type id 1.
fn parse_operator_1(start_index: usize, binary: &[char]) -> (Packet, usize) {
    let version =
        usize::from_str_radix(&binary[start_index..start_index + 3].iter().join(""), 2).unwrap();
    let type_id =
        usize::from_str_radix(&binary[start_index + 3..start_index + 6].iter().join(""), 2)
            .unwrap();

    debug_assert_eq!(binary[start_index + 6], '1');

    // Next 11 bits indicate the total number of subpackets
    let n_packets = usize::from_str_radix(
        &binary[start_index + 7..start_index + 18].iter().join(""),
        2,
    )
    .unwrap();

    let mut index = start_index + 18;
    let mut sub_packets = vec![];

    loop {
        let (packet, next_index) = parse_packet(index, binary);
        sub_packets.push(packet);
        index = next_index;
        if sub_packets.len() == n_packets {
            break;
        }
    }

    (
        Packet {
            version,
            type_id,
            package_type: PacketType::Operator(sub_packets),
        },
        index,
    )
}

/// Parse a general operator type packet.
fn parse_operator(start_index: usize, binary: &[char]) -> (Packet, usize) {
    // Check the length type id
    match binary[start_index + 6] {
        '0' => parse_operator_0(start_index, binary),
        _ => parse_operator_1(start_index, binary),
    }
}

/// Parse a generic packet.
fn parse_packet(start_index: usize, binary: &[char]) -> (Packet, usize) {
    // if start_index + 6 > binary.len() { return (Packet{ version: 0, type_id: 0, package_type: PacketType::Literal(0) } ,0); }
    match binary[start_index + 3..start_index + 6] {
        ['1', '0', '0'] => parse_literal(start_index, binary),
        _ => parse_operator(start_index, binary),
    }
}

/// All versions in the packet and its sub-packets.
fn sum_versions(packet: &Packet) -> usize {
    match packet.package_type.borrow() {
        PacketType::Literal(_) => packet.version,
        PacketType::Operator(packets) => {
            packet.version + packets.iter().map(sum_versions).sum::<usize>()
        }
    }
}

/// Sum the versions of all packets and sub-packets in the input.
pub fn day_16_1(data: &[String]) -> usize {
    let binary = convert_hex_to_binary(&data[0]);
    let (packet, _) = parse_packet(0, &binary);
    sum_versions(&packet)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_convert_hex_to_binary() {
        assert_eq!(
            convert_hex_to_binary("38006F45291200"),
            vec![
                '0', '0', '1', '1', '1', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0',
                '0', '1', '1', '0', '1', '1', '1', '1', '0', '1', '0', '0', '0', '1', '0', '1',
                '0', '0', '1', '0', '1', '0', '0', '1', '0', '0', '0', '1', '0', '0', '1', '0',
                '0', '0', '0', '0', '0', '0', '0', '0'
            ]
        );
    }

    #[test]
    fn test_parse_literal() {
        assert_eq!(
            parse_literal(
                0,
                &[
                    '1', '1', '0', '1', '0', '0', '1', '0', '1', '1', '1', '1', '1', '1', '1', '0',
                    '0', '0', '1', '0', '1', '0', '0', '0'
                ]
            ),
            (
                Packet {
                    version: 6,
                    type_id: 4,
                    package_type: PacketType::Literal(2021)
                },
                21
            )
        );
    }

    #[test]
    fn test_parse_operator_0() {
        let binary = [
            '0', '0', '1', '1', '1', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0',
            '1', '1', '0', '1', '1', '1', '1', '0', '1', '0', '0', '0', '1', '0', '1', '0', '0',
            '1', '0', '1', '0', '0', '1', '0', '0', '0', '1', '0', '0', '1', '0', '0', '0', '0',
            '0', '0', '0', '0', '0',
        ];
        let (packet, _) = parse_packet(0, &binary);
        assert_eq!(packet.version, 1);
        assert_eq!(packet.type_id, 6);
        if let PacketType::Operator(packets) = packet.package_type {
            assert_eq!(packets.len(), 2);
        }
    }

    #[test]
    fn test_day_16_1() {
        let data = vec!["D2FE28".to_string()];
        assert_eq!(day_16_1(&data), 6);

        let data = vec!["620080001611562C8802118E34".to_string()];
        assert_eq!(day_16_1(&data), 12);

        let data = vec!["C0015000016115A2E0802F182340".to_string()];
        assert_eq!(day_16_1(&data), 23);

        let data = vec!["A0016C880162017C3686B18A3D4780".to_string()];
        assert_eq!(day_16_1(&data), 31);
    }
}
