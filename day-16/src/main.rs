use std::fs;

const VERSION_START: usize = 0;
const TYPE_ID_START: usize = 3;
const LENGHT_TYPE_ID_START: usize = 6;
const LIT_SUB_PACKET_START: usize = 6;
const SUB_PACKET_LENGHT_START: usize = 7;

#[derive(Debug, Clone)]
pub struct Packet {
    version: usize,
    type_id: usize,
    lenght_type_id: usize,
    sub_packets: Vec<Packet>,
    value: usize,
}

fn main() {
    // let message = decode_hex("D2FE28");
    // let message = decode_hex("38006F45291200");
    // let message = decode_hex("EE00D40C823060");
    // let message = decode_hex("8A004A801A8002F478");
    // let message = decode_hex("620080001611562C8802118E34");
    // let message = decode_hex("C0015000016115A2E0802F182340");
    // let message = decode_hex("A0016C880162017C3686B18A3D4780");
    // let message = decode_hex("C200B40A82"); // -> 3
    // let message = decode_hex("04005AC33890"); // -> 54
    // let message = decode_hex("880086C3E88112"); // -> 7
    // let message = decode_hex("CE00C43D881120"); // -> 9
    // let message = decode_hex("D8005AC2A8F0"); // -> 1
    // let message = decode_hex("F600BC2D8F"); // -> 0
    // let message = decode_hex("9C005AC2F8F0"); // -> 0
    // let message = decode_hex("9C0141080250320F1802104A08"); // -> 1
    let message = decode_hex(&read_data("./input-full.txt"));
    let (packet, _striped_message, version_sum) = read_next_packet(message);
    println!("version sum {:?}", version_sum);
    println!("total {:?}", packet.value);
}

pub fn read_next_packet(message: String) -> (Packet, String, u16) {
    let version = bin_to_int(&message[VERSION_START..TYPE_ID_START]);
    let type_id = bin_to_int(&message[TYPE_ID_START..LENGHT_TYPE_ID_START]);
    let mut version_sum = version as u16;
    if type_id == 4 {
        // Handle literal type
        let mut sub_packets_message = &message[LIT_SUB_PACKET_START..message.len()];
        let mut value_parts: Vec<&str> = Vec::new();
        let mut is_last_packet = false;
        while sub_packets_message.clone().len() >= 5 && !is_last_packet {
            is_last_packet = &sub_packets_message[0..1] == "0";
            value_parts.push(&sub_packets_message[1..5]);
            sub_packets_message = &sub_packets_message[5..sub_packets_message.clone().len()];
        }
        let packet = Packet {
            version,
            type_id: type_id,
            lenght_type_id: 0,
            sub_packets: Vec::new(),
            value: bin_to_int(&value_parts.join("")),
        };
        (packet, sub_packets_message.to_string(), version_sum)
    } else {
        // Handle operator types
        let lenght_type_id = bin_to_int(&message[LENGHT_TYPE_ID_START..SUB_PACKET_LENGHT_START]);
        let sub_packet_lenght = if lenght_type_id == 0 { 15 } else { 11 };
        let sub_packets_offset_start = SUB_PACKET_LENGHT_START + sub_packet_lenght;

        let mut sub_packets: Vec<Packet> = Vec::new();
        let mut remaining_message = (&message[sub_packets_offset_start..message.len()]).to_string();
        if lenght_type_id == 0 {
            let bits_count = bin_to_int(
                &message[SUB_PACKET_LENGHT_START..SUB_PACKET_LENGHT_START + sub_packet_lenght],
            );
            let sub_packets_mess_length = remaining_message.clone().len();
            while sub_packets_mess_length - &remaining_message.len() < bits_count {
                let (sub_pack, mess, sub_ver_sum) = read_next_packet(remaining_message);
                version_sum += sub_ver_sum;
                sub_packets.push(sub_pack);
                remaining_message = mess;
            }
        } else {
            let sub_pack_number = bin_to_int(
                &message[SUB_PACKET_LENGHT_START..SUB_PACKET_LENGHT_START + sub_packet_lenght],
            );
            for _nb in 0..sub_pack_number {
                let (sub_pack, mess, sub_ver_sum) = read_next_packet(remaining_message.clone());
                version_sum += sub_ver_sum;
                sub_packets.push(sub_pack);
                remaining_message = mess;
            }
        }

        let packet = Packet {
            version,
            type_id: type_id,
            lenght_type_id,
            value: compute_operator_value(type_id, &sub_packets),
            sub_packets,
        };
        (packet, remaining_message, version_sum)
    }
}

pub fn read_data(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

pub fn decode_hex(s: &str) -> String {
    s.chars()
        .map(|c| to_binary(c).to_string())
        .collect::<Vec<String>>()
        .join("")
}

pub fn to_binary(c: char) -> &'static str {
    match c {
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
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

pub fn bin_to_int(bin_str: &str) -> usize {
    usize::from_str_radix(bin_str, 2).unwrap()
}

pub fn compute_operator_value(type_id: usize, sub_packets: &Vec<Packet>) -> usize {
    let mut value = 0;
    if type_id == 0 {
        value = sub_packets.iter().fold(0, |r, p| r + p.value);
    } else if type_id == 1 {
        value = sub_packets.iter().fold(1, |r, p| r * p.value);
    } else if type_id == 2 {
        value = sub_packets
            .iter()
            .fold(usize::MAX, |r, p| if r < p.value { r } else { p.value });
    } else if type_id == 3 {
        value = sub_packets
            .iter()
            .fold(0, |r, p| if r > p.value { r } else { p.value });
    } else if type_id == 5 {
        value = if sub_packets[0].value > sub_packets[1].value {
            1
        } else {
            0
        };
    } else if type_id == 6 {
        value = if sub_packets[0].value < sub_packets[1].value {
            1
        } else {
            0
        };
    } else if type_id == 7 {
        value = if sub_packets[0].value == sub_packets[1].value {
            1
        } else {
            0
        }
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_decode_hex() {
        assert_eq!(
            decode_hex("D2FE28"),
            String::from("110100101111111000101000")
        );
        assert_eq!(
            decode_hex("EE00D40C823060"),
            String::from("11101110000000001101010000001100100000100011000001100000")
        );
    }
}
