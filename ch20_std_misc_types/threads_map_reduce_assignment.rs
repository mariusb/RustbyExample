use std::thread;

const N_CHUNKS: usize = 12; // Number of chunks to split the data into

fn main() {
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
383979667071060941727 83238747669219
52380795257888236525459303330302837
5849532713574   4041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

    let mut children = vec![];

    // Remove whitespace and collect all digits into a Vec<char>
    let digits: Vec<char> = data.chars().filter(|c| c.is_digit(10)).collect();
    let chunk_size = (digits.len() + N_CHUNKS - 1) / N_CHUNKS; // ceil division

    for i in 0..N_CHUNKS {
        let start = i * chunk_size;
        let end = ((i + 1) * chunk_size).min(digits.len());
        let chunk: Vec<char> = digits[start..end].to_vec();

        println!("data chunk {} is \"{}\"", i, chunk.iter().collect::<String>());

        children.push(thread::spawn(move || -> u32 {
            let result = chunk
                .iter()
                .map(|c| c.to_digit(10).expect("should be a digit"))
                .sum();

            println!("processed chunk {}, result={}", i, result);
            result
        }));
    }

    let final_result: u32 = children.into_iter().map(|c| c.join().unwrap()).sum();

    println!("Final sum result: {}", final_result);
}