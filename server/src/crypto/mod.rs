

pub fn decrypt (input: &[u8]) -> Vec<u8> {
    let mut iter = input.iter();
    let mut output: Vec<u8> = Vec::new();

    while let Some(next) = iter.next() {
        let val = if *next == 7 {
            iter.next().unwrap() ^ 15
        } else {
            *next
        };
        output.push(val);
    }

    output
}

pub fn encrypt (input: &[u8]) -> Vec<u8> {
    let mut iter = input.iter();
    let mut output: Vec<u8> = Vec::new();

    while let Some(next) = iter.next() {
        if *next <= 7 {
            output.push(7);
            output.push(*next ^ 15);
        }
        output.push(*next);
    }

    output
}
