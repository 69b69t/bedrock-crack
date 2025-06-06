use bedrockgen::BedrockGenerator;
use bedrockgen::overworld::OverworldBedrock;

fn similarity_check(first: &[bool], second: &[bool]) -> f64 {
    let length = first.len();
    assert_eq!(length, second.len());

    let mut counter = 0;
    for i in 0..length {
        if first[i] == second[i] {
            counter += 1;
        }
    }

    (counter as f64) / (length as f64)
}

fn main() {
    //bits 0-24 and 17 - (17 + 24) on low matter
    //bits 17 - (17 + 24) on high matter
    //
    //realistically we can assume only ~5 bits are necessary per position.
    //this is because more bits just mean more accuracy to a float
    //those are 0 and 17 on low
    //and 17 on high

    //lets make a counter that corresponds to different bits in the three state positions.
    //counter will have 3 * bruteforce bits

    //generate this many bedrock at y = -60
    let check_size: i32 = 256;

    //number of bits we are using to bruteforce a state
    let bruteforce_bit_count = 10;

    //we are bruteforcing in 3 positions, multiply by 3
    let counter_max = 3 * bruteforce_bit_count;

    //test against seed 0
    let control_state = OverworldBedrock::new(694201337);
    let check_against = control_state.generate_range(0, -60, 0, check_size, -59, check_size);

    //dummy overworld bedrock thing to hack the states
    let mut modified_state = OverworldBedrock::new(0);
    for counter in 0..(1 << counter_max) {
        //extract bits from counter
        let low0 = (counter) & ((1 << bruteforce_bit_count) - 1);
        let low17 = (counter >> (bruteforce_bit_count)) & ((1 << bruteforce_bit_count) - 1);
        let high17 = (counter >> (2 * bruteforce_bit_count)) & ((1 << bruteforce_bit_count) - 1);
        
        //put extracted bits into state
        modified_state.xr.low = (low0 << (64 - bruteforce_bit_count)) | (low17 << (47 - bruteforce_bit_count));
        modified_state.xr.high = high17 << (47 - bruteforce_bit_count);
        //println!("{:#066b} {:#066b}", modified_state.xr.low, modified_state.xr.high);

        //now we can check state for any funny business
        let bedrock_pattern = modified_state.generate_range(0, -60, 0, check_size, -59, check_size);
        let similarity = similarity_check(&check_against, &bedrock_pattern);
        if 0.993 < similarity {
            print!("{similarity}");
            println!(" high:{} low:{}", modified_state.xr.high, modified_state.xr.low);
        }
    }

    
}

/*
pairs from andrew
3387475 11495857
14391324 19455217
15330447 23598152
12707052 24163979
2210519 26160814
10605566 27291435
3351313 28018135
22930187 29437475
21888275 30384970
28178283 31347541
6245239 31735938
24064455 31930598
2630413 32604855
20122818 34220079
1548823 37085592
21814723 37097492
23641604 37103284
18770175 38280548
18277579 38346655
31288489 39126476
*/
