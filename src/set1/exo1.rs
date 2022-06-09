// Convert hex to base64

const base64: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn convert_hex_to_base64(hex: String) -> String {
    let binary: String = hex.chars().fold("".to_string(), |acc, x| {
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
    });

    let mut peek_binary = binary.chars().peekable();
    let mut res = String::from("");

    while peek_binary.peek().is_some() {
        let chunk: String = peek_binary
            .by_ref()
            .take(6)
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();
        let mut index = 0;
        for (i, char) in chunk.chars().enumerate() {
            index += 2_i32.pow((i).try_into().unwrap()) * char.to_digit(10).unwrap() as i32;
        }
        res.push(base64.chars().nth(index.try_into().unwrap()).unwrap());
    }

    String::from(res)
}

pub fn verify_exo1() {
    let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

    assert_eq!(
        &convert_hex_to_base64(hex_string.to_owned()),
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
    );
    println!("Exercise 1 completed");
}
