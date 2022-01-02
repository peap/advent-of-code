use std::cmp::Ordering;

use common::{default_puzzle, Puzzle};

fn parse_bits(bits: Vec<char>) -> u64 {
    let mut n: u64 = 0;
    for bc in bits.iter() {
        let bit = bc.to_digit(2).unwrap() as u64;
        n = (n << 1) | bit;
    }
    n
}

fn nums_to_bits(nums: Vec<u8>) -> Vec<char> {
    // Convert numbers to binary characters of width 4.
    let mut bits: Vec<char> = vec![];
    for n in nums.iter() {
        let mut chars: Vec<char> = format!("{:04b}", n).chars().collect();
        bits.append(&mut chars);
    }
    bits
}

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    bit_length: u64,
    payload: u64,
    subpackets: Vec<Packet>,
}

impl Packet {
    fn new(nums: Vec<u8>) -> Self {
        let bits = nums_to_bits(nums);
        let mut biterator = bits.iter();
        Self::from_iterator(&mut biterator)
    }

    fn from_iterator<'a, I>(biterator: &mut I) -> Self
    where
        I: Iterator<Item = &'a char>,
    {
        let mut bit_length = 0;
        let mut payload: u64 = 0;
        let mut subpackets = vec![];

        // Read the version: first 3 bits
        let version = parse_bits(biterator.by_ref().take(3).copied().collect()) as u8;
        bit_length += 3;

        // Read the type ID: next 3 bits
        let type_id = parse_bits(biterator.by_ref().take(3).copied().collect()) as u8;
        bit_length += 3;

        // Read the payload: next N bits.
        if type_id == 4 {
            let (count, _payload) = Self::read_literal(biterator.by_ref());
            bit_length += count;
            payload = _payload;
        } else {
            let (count, _subpackets) = Self::read_subpacket(biterator.by_ref());
            bit_length += count;
            subpackets = _subpackets;
        }

        // Make the packet.
        Packet {
            version,
            type_id,
            bit_length,
            payload,
            subpackets,
        }
    }

    fn read_literal<'a, I>(biterator: &mut I) -> (u64, u64)
    where
        I: Iterator<Item = &'a char>,
    {
        let mut count = 0;
        let mut done = false;
        let mut val_bits = vec![];
        while !done {
            // Read five bits at a time; a leading zero indicates it's the last group.
            let prefix = biterator.next().unwrap();
            count += 1;
            let mut group = biterator.by_ref().take(4).copied().collect();
            count += 4;
            val_bits.append(&mut group);
            if *prefix == '0' {
                done = true;
            }
        }
        (count, parse_bits(val_bits))
    }

    fn read_subpacket<'a, I>(biterator: &mut I) -> (u64, Vec<Self>)
    where
        I: Iterator<Item = &'a char>,
    {
        // First bit is the length type ID.
        let mut count = 0;
        let mut subpackets: Vec<Self> = vec![];
        let prefix = biterator.next().unwrap();
        count += 1;

        // Length type 0: next 15 bits are a number representing the total length in bits.
        if *prefix == '0' {
            let length = parse_bits(biterator.by_ref().take(15).copied().collect());
            count += 15;
            let subpacket_bits: Vec<char> =
                biterator.by_ref().take(length as usize).copied().collect();
            count += length;
            let mut sub_biterator = subpacket_bits.iter();
            loop {
                subpackets.push(Packet::from_iterator(&mut sub_biterator));
                if sub_biterator.clone().next().is_none() {
                    break;
                }
            }
        // Length type 1: next 11 bits are a number representing the number of sub-packets.
        } else {
            let subpacket_count = parse_bits(biterator.by_ref().take(11).copied().collect());
            count += 11;
            for _ in 0..subpacket_count {
                let subpacket = Packet::from_iterator(biterator);
                count += subpacket.bit_length;
                subpackets.push(subpacket);
            }
        }
        (count, subpackets)
    }

    fn sum_of_versions(&self) -> u64 {
        self.version as u64
            + self
                .subpackets
                .iter()
                .fold(0, |acc, p| acc + p.sum_of_versions())
    }

    fn evaluate(&self) -> u64 {
        let sub_evaluations = self.subpackets.iter().map(|s| s.evaluate());
        match self.type_id {
            0 => sub_evaluations.sum(),
            1 => sub_evaluations.product(),
            2 => sub_evaluations.min().unwrap(),
            3 => sub_evaluations.max().unwrap(),
            4 => self.payload,
            t => {
                let first = self.subpackets[0].evaluate();
                let second = self.subpackets[1].evaluate();
                (match first.cmp(&second) {
                    Ordering::Greater => t == 5,
                    Ordering::Less => t == 6,
                    Ordering::Equal => t == 7,
                }) as u64
            }
        }
    }
}

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("Packet Decoder");
    puzzle.set_part1("sum of version numbers", |reader| {
        let nums: Vec<u8> = reader.digit_line(16);
        let packet = Packet::new(nums);
        packet.sum_of_versions()
    });
    puzzle.set_part2("packet evaluates to", |reader| {
        let nums: Vec<u8> = reader.digit_line(16);
        let packet = Packet::new(nums);
        packet.evaluate()
    });
    puzzle
}

fn main() {
    get_puzzle().run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bits() {
        assert_eq!(0, parse_bits(vec![]));
        assert_eq!(0, parse_bits(vec!['0']));
        assert_eq!(0, parse_bits(vec!['0', '0']));
        assert_eq!(0, parse_bits(vec!['0', '0', '0', '0', '0']));
        assert_eq!(1, parse_bits(vec!['1']));
        assert_eq!(2, parse_bits(vec!['1', '0']));
        assert_eq!(8, parse_bits(vec!['1', '0', '0', '0']));
    }

    fn hex_to_nums(hex: &'static str) -> Vec<u8> {
        hex.chars().map(|c| c.to_digit(16).unwrap() as u8).collect()
    }

    #[test]
    fn test_example1() {
        let nums = hex_to_nums("D2FE28");
        assert_eq!(nums, vec![13, 2, 15, 14, 2, 8]);
        let packet = Packet::new(nums);
        assert_eq!(packet.version, 6);
        assert_eq!(packet.type_id, 4);
        assert_eq!(packet.bit_length, 21); // last three zeroes ignored
        assert_eq!(packet.payload, 2021);
        assert!(packet.subpackets.is_empty());
        assert_eq!(packet.sum_of_versions(), 6);
    }

    #[test]
    fn test_example2() {
        let packet = Packet::new(hex_to_nums("38006F45291200"));
        assert_eq!(packet.version, 1);
        assert_eq!(packet.type_id, 6);
        assert_eq!(packet.bit_length, 49); // last seven zeroes ignored
        assert_eq!(packet.payload, 0);
        assert_eq!(packet.subpackets.len(), 2);
        assert_eq!(packet.sum_of_versions(), 1 + 6 + 2);
    }

    #[test]
    fn test_example3() {
        let packet = Packet::new(hex_to_nums("EE00D40C823060"));
        assert_eq!(packet.sum_of_versions(), 7 + 2 + 4 + 1);
    }

    #[test]
    fn test_example4() {
        let packet = Packet::new(hex_to_nums("8A004A801A8002F478"));
        assert_eq!(packet.sum_of_versions(), 16);
    }

    #[test]
    fn test_example5() {
        let packet = Packet::new(hex_to_nums("620080001611562C8802118E34"));
        assert_eq!(packet.sum_of_versions(), 12);
    }

    #[test]
    fn test_example6() {
        let packet = Packet::new(hex_to_nums("C0015000016115A2E0802F182340"));
        assert_eq!(packet.sum_of_versions(), 23);
    }

    #[test]
    fn test_example7() {
        let packet = Packet::new(hex_to_nums("A0016C880162017C3686B18A3D4780"));
        assert_eq!(packet.sum_of_versions(), 31);
    }

    #[test]
    fn test_example_evaluations() {
        assert_eq!(Packet::new(hex_to_nums("D2FE28")).evaluate(), 2021);
        assert_eq!(Packet::new(hex_to_nums("C200B40A82")).evaluate(), 3);
        assert_eq!(Packet::new(hex_to_nums("04005AC33890")).evaluate(), 54);
        assert_eq!(Packet::new(hex_to_nums("880086C3E88112")).evaluate(), 7);
        assert_eq!(Packet::new(hex_to_nums("CE00C43D881120")).evaluate(), 9);
        assert_eq!(Packet::new(hex_to_nums("D8005AC2A8F0")).evaluate(), 1);
        assert_eq!(Packet::new(hex_to_nums("F600BC2D8F")).evaluate(), 0);
        assert_eq!(Packet::new(hex_to_nums("9C005AC2F8F0")).evaluate(), 0);
        assert_eq!(
            Packet::new(hex_to_nums("9C0141080250320F1802104A08")).evaluate(),
            1
        );
    }

    #[test]
    fn test_part1() {
        get_puzzle().test_part1(821);
    }

    #[test]
    fn test_part2() {
        get_puzzle().test_part2(2056021084691);
    }
}
