pub(super) fn encode(data: Vec<u8>) -> String {
    let mut data = data.clone();
    let align = data.len() % 3;

    if align > 0 {
        for _ in 0..(align - 3) {
            data.push(0)
        }
    }

    let mut query = String::new();

    data.chunks(3).for_each(|bs| {
        bytes_to_chars(bs).map(|cs| {
            cs.iter().for_each(|ch| query.push(to_alphabet_char(*ch)))
        });
    });

    query
}

fn to_alphabet_char(ch: u8) -> char {
    match ch {
        0 ..=9  => format!("{}", ch).parse().unwrap(),
        10..=35 => char::from('A' as u8 + ch - 10),
        36..=61 => char::from('a' as u8 + ch - 36),
        62      => '-',
        63      => '_',
        _       => '?'
    }
}

fn bytes_to_chars(bytes: &[u8]) -> Result<[u8; 4], ()> {
    if let [b1, b2, b3] = bytes {
        let c1 = b1 >> 2;
        let c2 = ((b1 & 0x3) << 4) | (b2 >> 4);
        let c3 = ((b2 & 0xF) << 2) | (b3 >> 6);
        let c4 = b3 & 0x3F;

        return Ok([c1, c2, c3, c4])
    }
    Err(())
}