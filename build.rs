use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let chars = [
        (' '),
        (','),
        ('.'),
        ('0'),
        ('1'),
        ('2'),
        ('3'),
        ('4'),
        ('5'),
        ('6'),
        ('7'),
        ('8'),
        ('9'),
        ('A'),
        ('B'),
        ('C'),
        ('D'),
        ('E'),
        ('F'),
        ('G'),
        ('H'),
        ('I'),
        ('J'),
        ('K'),
        ('L'),
        ('M'),
        ('N'),
        ('O'),
        ('P'),
        ('Q'),
        ('R'),
        ('S'),
        ('T'),
        ('U'),
        ('V'),
        ('W'),
        ('X'),
        ('Y'),
        ('Z'),
    ];
    let mut map: HashMap<char, Vec<u8>> = HashMap::new();
    map.insert('0', fs::read("codes/0.png").unwrap());
    map.insert('1', fs::read("codes/0.png").unwrap());
    map.insert('2', fs::read("codes/0.png").unwrap());
    map.insert('3', fs::read("codes/0.png").unwrap());
    map.insert('4', fs::read("codes/0.png").unwrap());
    map.insert('5', fs::read("codes/0.png").unwrap());
    map.insert('6', fs::read("codes/0.png").unwrap());
    map.insert('7', fs::read("codes/0.png").unwrap());
    map.insert('8', fs::read("codes/0.png").unwrap());
    map.insert('9', fs::read("codes/0.png").unwrap());
    map.insert('A', fs::read("codes/0.png").unwrap());
    map.insert('B', fs::read("codes/0.png").unwrap());
    map.insert('C', fs::read("codes/0.png").unwrap());
    map.insert('D', fs::read("codes/0.png").unwrap());
    map.insert('E', fs::read("codes/0.png").unwrap());
    map.insert('F', fs::read("codes/0.png").unwrap());
    map.insert('G', fs::read("codes/0.png").unwrap());
    map.insert('H', fs::read("codes/0.png").unwrap());
    map.insert('I', fs::read("codes/0.png").unwrap());
    map.insert('J', fs::read("codes/0.png").unwrap());
    map.insert('K', fs::read("codes/0.png").unwrap());
    map.insert('L', fs::read("codes/0.png").unwrap());
    map.insert('M', fs::read("codes/0.png").unwrap());
    map.insert('N', fs::read("codes/0.png").unwrap());
    map.insert('O', fs::read("codes/0.png").unwrap());
    map.insert('P', fs::read("codes/0.png").unwrap());
    map.insert('Q', fs::read("codes/0.png").unwrap());
    map.insert('R', fs::read("codes/0.png").unwrap());
    map.insert('S', fs::read("codes/0.png").unwrap());
    map.insert('T', fs::read("codes/0.png").unwrap());
    map.insert('U', fs::read("codes/0.png").unwrap());
    map.insert('V', fs::read("codes/0.png").unwrap());
    map.insert('W', fs::read("codes/0.png").unwrap());
    map.insert('X', fs::read("codes/0.png").unwrap());
    map.insert('Y', fs::read("codes/0.png").unwrap());
    map.insert('Z', fs::read("codes/0.png").unwrap());
    map.insert(' ', fs::read("codes/0.png").unwrap());
    map.insert('.', fs::read("codes/0.png").unwrap());
    map.insert(',', fs::read("codes/0.png").unwrap());

    let mut output = String::new();

    output.push_str("use std::collections::HashMap;\n\n");
    output.push_str("pub fn precompiled_map() -> HashMap<String, Vec<u8>> {\n");
    output.push_str("    let mut map = HashMap::new();\n");
    for (key, value) in &map {
        output.push_str(&format!(
            "    map.insert(\"{}\".to_string(), vec!{:?});\n",
            key, value
        ));
    }
    output.push_str("    map\n}\n");
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = format!("{}/precompiled_map.rs", out_dir);
    fs::write(dest_path, output).unwrap();
}
