// Fixed XOR

const hex_values: &str = "0123456789abcdef";

fn hex_to_bits(hex: String) -> String {
    hex.chars().fold("".to_string(), |acc, x| {
        let bits = match x {
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
            'a' => "1010",
            'b' => "1011",
            'c' => "1100",
            'd' => "1101",
            'e' => "1110",
            'f' => "1111",
            _ => "",
        };
        format!("{acc}{bits}")
    })
}

fn xor_hexes(first_hex: String, second_hex: String) -> String {
    let first_bits = hex_to_bits(first_hex);
    let second_bits = hex_to_bits(second_hex);

    let mut xor_binary = String::from("");

    for (i, it) in first_bits.chars().zip(second_bits.chars()) {
        // here
        if i != it {
            xor_binary.push('1')
        } else {
            xor_binary.push('0')
        }
    }

    let mut res = String::from("");
    let mut peek_binary = xor_binary.chars().peekable();

    while peek_binary.peek().is_some() {
        let chunk: String = peek_binary
            .by_ref()
            .take(4)
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();
        let mut index = 0;
        for (i, char) in chunk.chars().enumerate() {
            index += 2_i32.pow((i).try_into().unwrap()) * char.to_digit(10).unwrap() as i32;
        }

        res.push(hex_values.chars().nth(index.try_into().unwrap()).unwrap());
    }

    res
}

pub fn verify_exo2() {
    let first_hex = "1c0111001f010100061a024b53535009181c";
    let second_hex = "686974207468652062756c6c277320657965";

    assert_eq!(
        &xor_hexes(first_hex.to_owned(), second_hex.to_owned()),
        "746865206b696420646f6e277420706c6179"
    );
    println!("Exercise 2 completed");
}
