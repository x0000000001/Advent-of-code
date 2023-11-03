use crate::Solution;

type InputType = Vec<String>;

struct Packet {
    version: u8,
    type_id: u8,
    value: i64,
    inside_packets: Vec<Packet>,
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "version : {}, type id : {}, value : {}",
            self.version, self.type_id, self.value
        )
    }
}

fn read_int(bits: &[bool]) -> i64 {
    let mut i = 0;

    for b in bits {
        i *= 2;

        if *b {
            i += 1;
        }
    }

    i
}

//returns as second argument the number of bits read
fn read_litteral(bits: &[bool]) -> (i64, i64) {
    let mut val_bits: Vec<bool> = Vec::new();

    let mut i = 0;
    let mut read = true;

    while read {
        if !bits[i] {
            read = false;
        }
        i += 1;
        for _ in 0..4 {
            val_bits.push(bits[i]);
            i += 1;
        }
    }

    (read_int(&val_bits), i as i64)
}

//returns as well length of packets read
fn read_packet<'a>(source: &[bool]) -> (Packet, i64) {
    let version: u8 = read_int(&source[0..3]) as u8;
    let type_id: u8 = read_int(&source[3..6]) as u8;

    //index in packet bits
    let mut i = 6;

    if type_id == 4 {
        let (val, bits_read) = read_litteral(&source[i..]);

        let y = (
            Packet {
                version: version,
                type_id: type_id,
                value: val,
                inside_packets: Vec::new(),
            },
            bits_read + 6,
        );

        // println!("{}", y.0);

        return y;
    } else {
        let length_type_id = source[6];
        let mut subpackets_count = 0;
        let mut bits_count = 0;
        let mut subpackets_total = i64::MAX;
        let mut bits_total = i64::MAX;

        let mut inside_packets: Vec<Packet> = Vec::new();

        if length_type_id {
            subpackets_total = read_int(&source[7..18]);
            i = 18;
        } else {
            bits_total = read_int(&source[7..22]);
            i = 22;
        }

        while bits_count < bits_total && subpackets_count < subpackets_total {
            let (packet, bits_read) = read_packet(&source[i..]);
            i += bits_read as usize;
            bits_count += bits_read;
            subpackets_count += 1;

            inside_packets.push(packet);
        }

        let y = (
            Packet {
                version: version,
                type_id: type_id,
                value: 0,
                inside_packets: inside_packets,
            },
            i as i64,
        );

        // println!("{}", y.0);
        return y;
    }
}

fn sum_version_numbers(packet: Packet) -> i64 {
    let mut sum: i64 = packet.version as i64;

    for p in packet.inside_packets {
        sum += sum_version_numbers(p);
    }

    sum
}

pub fn part1(s: String) -> Solution {
    let input = &mut parse(s)[0];

    let hexa: String = input
        .chars()
        .map(|c| match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' | 'a' => "1010",
            'B' | 'b' => "1011",
            'C' | 'c' => "1100",
            'D' | 'd' => "1101",
            'E' | 'e' => "1110",
            'F' | 'f' => "1111",
            _ => "",
        })
        .flat_map(|c| c.chars())
        .collect();

    let bits: Vec<bool> = hexa
        .chars()
        .map(|c| match c {
            '0' => false,
            _ => true,
        })
        .collect();

    // println!("{:?}", bits.iter().map(|el| if *el { 1} else { 0}).collect::<Vec<u8>>());

    let (packet, _) = read_packet(&bits);

    Solution::from(sum_version_numbers(packet))
}

fn get_packet_result(packet: Packet) -> i64 {
    match packet.type_id {
        0 => packet
            .inside_packets
            .into_iter()
            .map(|p| get_packet_result(p))
            .sum(),
        1 => packet
            .inside_packets
            .into_iter()
            .map(|p| get_packet_result(p))
            .product(),
        2 => packet
            .inside_packets
            .into_iter()
            .map(|p| get_packet_result(p))
            .min()
            .unwrap_or(0),
        3 => packet
            .inside_packets
            .into_iter()
            .map(|p| get_packet_result(p))
            .max()
            .unwrap_or(0),
        4 => packet.value,
        5 => {
            let results: Vec<i64> = packet
                .inside_packets
                .into_iter()
                .map(|p| get_packet_result(p))
                .collect();
            if results[0] > results[1] {
                1
            } else {
                0
            }
        }
        6 => {
            let results: Vec<i64> = packet
                .inside_packets
                .into_iter()
                .map(|p| get_packet_result(p))
                .collect();
            if results[0] < results[1] {
                1
            } else {
                0
            }
        }
        7 => {
            let results: Vec<i64> = packet
                .inside_packets
                .into_iter()
                .map(|p| get_packet_result(p))
                .collect();
            if results[0] == results[1] {
                1
            } else {
                0
            }
        }
        _ => 0,
    }
}

pub fn part2(s: String) -> Solution {
    let input = &mut parse(s)[0];

    let hexa: String = input
        .chars()
        .map(|c| match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' | 'a' => "1010",
            'B' | 'b' => "1011",
            'C' | 'c' => "1100",
            'D' | 'd' => "1101",
            'E' | 'e' => "1110",
            'F' | 'f' => "1111",
            _ => "",
        })
        .flat_map(|c| c.chars())
        .collect();

    let bits: Vec<bool> = hexa
        .chars()
        .map(|c| match c {
            '0' => false,
            _ => true,
        })
        .collect();

    // println!("{:?}", bits.iter().map(|el| if *el { 1} else { 0}).collect::<Vec<u8>>());

    let (packet, _) = read_packet(&bits);

    Solution::from(get_packet_result(packet))
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect()
}
