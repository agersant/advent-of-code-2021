use itertools::Itertools;

static INPUT: &str = "420D610055D273AF1630010092019207300B278BE5932551E703E608400C335003900AF0402905009923003880856201E95C00B60198D400B50034400E20101DC00E10024C00F1003C400B000212697140249D049F0F8952A8C6009780272D5D074B5741F3F37730056C0149658965E9AED7CA8401A5CC45BB801F0999FFFEEE0D67050C010C0036278A62D4D737F359993398027800BECFD8467E3109945C1008210C9C442690A6F719C48A351006E9359C1C5003E739087E80F27EC29A0030322BD2553983D272C67508E5C0804639D4BD004C401B8B918E3600021D1061D47A30053801C89EF2C4CCFF39204C53C212DABED04C015983A9766200ACE7F95C80D802B2F3499E5A700267838803029FC56203A009CE134C773A2D3005A77F4EDC6B401600043A35C56840200F4668A71580043D92D5A02535BAF7F9A89CF97C9F59A4F02C400C249A8CF1A49331004CDA00ACA46517E8732E8D2DB90F3005E92362194EF5E630CA5E5EEAD1803E200CC228E70700010A89D0BE3A08033146164177005A5AEEB1DA463BDC667600189C9F53A6FF6D6677954B27745CA00BCAE53A6EEDC60074C920001B93CFB05140289E8FA4812E071EE447218CBE1AA149008DBA00A497F9486262325FE521898BC9669B382015365715953C5FC01AA8002111721D4221007E13C448BA600B4F77F694CE6C01393519CE474D46009D802C00085C578A71E4001098F518639CC301802B400E7CDDF4B525C8E9CA2188032600E44B8F1094C0198CB16A29180351EA1DC3091F47A5CA0054C4234BDBC2F338A77B84F201232C01700042A0DC7A8A0200CC578B10A65A000601048B24B25C56995A30056C013927D927C91AB43005D127FDC610EF55273F76C96641002A4F0F8C01CCC579A8D68E52587F982996F537D600";

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: u8,
    payload: Payload,
}

impl Packet {
    fn new_literal(version: u8, value: u64) -> Packet {
        Packet {
            version,
            payload: Payload::Literal(value),
        }
    }

    fn new_operator(version: u8, type_id: u8, sub_packets: Vec<Packet>) -> Packet {
        Packet {
            version,
            payload: match type_id {
                0 => Payload::Sum(sub_packets),
                1 => Payload::Product(sub_packets),
                2 => Payload::Minimum(sub_packets),
                3 => Payload::Maximum(sub_packets),
                5 => Payload::GreaterThan(sub_packets),
                6 => Payload::LessThan(sub_packets),
                _ => Payload::Equal(sub_packets),
            },
        }
    }

    fn sum_versions(&self) -> u64 {
        self.version as u64
            + match &self.payload {
                Payload::Literal(_) => 0_u64,
                Payload::Sum(sp)
                | Payload::Product(sp)
                | Payload::Minimum(sp)
                | Payload::Maximum(sp)
                | Payload::GreaterThan(sp)
                | Payload::LessThan(sp)
                | Payload::Equal(sp) => sp.iter().map(|p| p.sum_versions()).sum(),
            }
    }

    fn eval(&self) -> u64 {
        match &self.payload {
            Payload::Literal(n) => *n,
            Payload::Sum(sp) => sp.iter().map(|p| p.eval()).sum(),
            Payload::Product(sp) => sp.iter().map(|p| p.eval()).product(),
            Payload::Minimum(sp) => sp.iter().map(|p| p.eval()).min().unwrap(),
            Payload::Maximum(sp) => sp.iter().map(|p| p.eval()).max().unwrap(),
            Payload::GreaterThan(sp) => {
                if sp[0].eval() > sp[1].eval() {
                    1
                } else {
                    0
                }
            }
            Payload::LessThan(sp) => {
                if sp[0].eval() < sp[1].eval() {
                    1
                } else {
                    0
                }
            }
            Payload::Equal(sp) => {
                if sp[0].eval() == sp[1].eval() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Payload {
    Literal(u64),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    Equal(Vec<Packet>),
}

fn decode_hex(hex_input: &str) -> String {
    hex_input
        .split("")
        .filter(|s| s.len() > 0)
        .map(|c| format!("{:0>4b}", u8::from_str_radix(c, 16).unwrap()))
        .join("")
}

fn decode(hex_input: &str) -> Packet {
    let (packet, _tail) = read_packet(&decode_hex(hex_input));
    packet
}

fn read_bits(input: &str) -> u32 {
    input
        .chars()
        .rev()
        .enumerate()
        .map(|(n, c)| match c {
            '0' => 0,
            _ => 1 << n,
        })
        .sum()
}

fn read_literal(mut input: &str) -> (u64, &str) {
    let mut res = 0;
    loop {
        let keep_reading = read_bits(&input[0..1]) == 1;
        let chunk = read_bits(&input[1..5]);
        res = (res << 4) + chunk as u64;
        input = &input[5..];
        if !keep_reading {
            break;
        }
    }
    (res, input)
}

fn read_packet(input: &str) -> (Packet, &str) {
    let version = read_bits(&input[0..3]) as u8;
    let type_id = read_bits(&input[3..6]) as u8;
    match type_id {
        4 => {
            let (value, tail) = read_literal(&input[6..]);
            (Packet::new_literal(version, value), tail)
        }
        _ => {
            let length_type_id = read_bits(&input[6..7]);
            if length_type_id == 0 {
                let sub_packets_length = read_bits(&input[7..22]) as usize;
                let mut sub_packets_str = &input[22..(22 + sub_packets_length)];
                let mut sub_packets = Vec::new();
                while !sub_packets_str.is_empty() {
                    let (sub_packet, tail) = read_packet(sub_packets_str);
                    sub_packets.push(sub_packet);
                    sub_packets_str = tail;
                }
                (
                    Packet::new_operator(version, type_id, sub_packets),
                    &input[(22 + sub_packets_length)..],
                )
            } else {
                let num_sub_packets = read_bits(&input[7..18]) as usize;
                let mut remaining_data = &input[18..];
                let mut sub_packets = Vec::new();
                while sub_packets.len() < num_sub_packets {
                    let (sub_packet, tail) = read_packet(remaining_data);
                    sub_packets.push(sub_packet);
                    remaining_data = tail;
                }
                (
                    Packet::new_operator(version, type_id, sub_packets),
                    remaining_data,
                )
            }
        }
    }
}

#[allow(dead_code)]
pub fn part1() {
    let transmission = decode(INPUT);
    println!("15.1 {}", transmission.sum_versions());
}

#[allow(dead_code)]
pub fn part2() {
    let transmission = decode(INPUT);
    println!("15.2 {}", transmission.eval());
}

#[test]
fn test_decode_hex() {
    assert_eq!(decode_hex("0"), "0000");
    assert_eq!(decode_hex("2"), "0010");
    assert_eq!(decode_hex("A"), "1010");
    assert_eq!(decode_hex("D2FE28"), "110100101111111000101000");
}

#[test]
fn test_read_bits() {
    assert_eq!(read_bits("000000"), 0);
    assert_eq!(read_bits("000010"), 2);
    assert_eq!(read_bits("000010"), 2);
    assert_eq!(read_bits("10111"), 23);
}

#[test]
fn test_decode_literal() {
    assert_eq!(decode("D2FE28"), Packet::new_literal(6, 2021));
}

#[test]
fn test_decode_total_length_operator() {
    assert_eq!(
        decode("38006F45291200"),
        Packet::new_operator(
            1,
            6,
            vec![Packet::new_literal(6, 10), Packet::new_literal(2, 20)]
        )
    );
}

#[test]
fn test_decode_num_packets_operator() {
    assert_eq!(
        decode("EE00D40C823060"),
        Packet::new_operator(
            7,
            3,
            vec![
                Packet::new_literal(2, 1),
                Packet::new_literal(4, 2),
                Packet::new_literal(1, 3),
            ]
        ),
    );
}

#[test]
fn test_sum_versions() {
    assert_eq!(decode("8A004A801A8002F478").sum_versions(), 16);
    assert_eq!(decode("620080001611562C8802118E34").sum_versions(), 12);
    assert_eq!(decode("C0015000016115A2E0802F182340").sum_versions(), 23);
    assert_eq!(decode("A0016C880162017C3686B18A3D4780").sum_versions(), 31);
}

#[test]
fn test_eval() {
    assert_eq!(decode("C200B40A82").eval(), 3);
    assert_eq!(decode("04005AC33890").eval(), 54);
    assert_eq!(decode("880086C3E88112").eval(), 7);
    assert_eq!(decode("CE00C43D881120").eval(), 9);
    assert_eq!(decode("D8005AC2A8F0").eval(), 1);
    assert_eq!(decode("F600BC2D8F").eval(), 0);
    assert_eq!(decode("9C005AC2F8F0").eval(), 0);
    assert_eq!(decode("9C0141080250320F1802104A08").eval(), 1);
}
