use rand::Rng;

fn gen_adjectives(adjectives:i32) -> Vec<char> {
    let adj_array = ['O','S','A','C','M'];
    let mut adj_array2 = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..adjectives {
        adj_array2.push(adj_array[rng.gen_range(0..5)]);
    }
    let length = adj_array2.len();
    for n in adj_array {
        for m in 0..length {
            if n == adj_array2[m] {
                adj_array2.push(n);
                // I absolutely despised having to create an additional vector. This was my solution.
                // This also serves to maintain the unconscious order of adjectives.
            }
        }
    }
    for _ in 0..length {
        adj_array2.remove(0);
    }
    return adj_array2;
}

fn get_starting_structure1(adjectives:i32, plural:bool) -> Vec<char> {
    let mut struct_array:Vec<char> = Vec::new();
    let mut rng = rand::thread_rng();
    if plural == false || rng.gen_bool(0.5) {
        struct_array.push('0');
        // Why is 'the' represented by a '0', you might ask? Because t is going to be used for transitive verbs. I hope.
    }
    struct_array.append(&mut gen_adjectives(adjectives));
    if plural == true {
        struct_array.push('p');
    } else {
        struct_array.push('s');
    }
    return struct_array;
}

fn get_ending_structure1(adverbs:i32, plural:bool, ex_struct:Vec<char>) -> Vec<char> {
    let mut struct_array = ex_struct;
    if plural == true {
        struct_array.push('v');
    }
    else {
        struct_array.push('V');
    }
    for _ in 0..adverbs{
        struct_array.push('a');
    }
    struct_array.push('x');
    return struct_array;
}

fn get_ending_structure2(adverbs:i32, plural:bool, ex_struct:Vec<char>) -> Vec<char> {
    let mut struct_array = ex_struct;
    for _ in 0..adverbs{
        struct_array.push('a');
    }
    if plural == true {
        struct_array.push('v');
    }
    else {
        struct_array.push('V');
    }
    struct_array.push('x');
    return struct_array;
}

fn get_ending_structure3(adverbs:i32, plural:bool, adjectives2:i32, plural2:bool, ex_struct:Vec<char>) -> Vec<char> {
    let mut rng = rand::thread_rng();
    let mut struct_array = ex_struct;
    if plural == true {
        struct_array.push('v');
    }
    else {
        struct_array.push('V');
    }
    for _ in 0..adverbs {
        struct_array.push('a');
    }
    if plural2 == false || rng.gen_bool(0.5) {
        struct_array.push('0');
    }
    struct_array.append(&mut gen_adjectives(adjectives2));
    if plural2 == true {
        struct_array.push('p');
    } else {
        struct_array.push('s');
    }
    struct_array.push('x');
    return struct_array;
}

fn get_ending_structure4(adverbs:i32, plural:bool, adjectives2:i32, plural2:bool, ex_struct:Vec<char>) -> Vec<char> {
    let mut rng = rand::thread_rng();
    let mut struct_array = ex_struct;
    for _ in 0..adverbs {
        struct_array.push('a');
    }
    if plural == true {
        struct_array.push('v');
    }
    else {
        struct_array.push('V');
    }
    if plural2 == false || rng.gen_bool(0.5) {
        struct_array.push('0');
    }
    struct_array.append(&mut gen_adjectives(adjectives2));
    if plural2 == true {
        struct_array.push('p');
    } else {
        struct_array.push('s');
    }
    struct_array.push('x');
    return struct_array;
}

pub fn get_structure(adjectives:i32, adverbs:i32, structure:i32, plural:bool, adjectives2:i32, plural2:bool) -> Vec<char>{
    return if structure == 1 {
        get_ending_structure1(adverbs, plural, get_starting_structure1(adjectives, plural))
    } else if structure == 2 {
        get_ending_structure2(adverbs, plural, get_starting_structure1(adjectives, plural))
    } else if structure == 3 {
        get_ending_structure3(adverbs, plural, adjectives2,plural2, get_starting_structure1(adjectives, plural))
    } else if structure == 4 {
        get_ending_structure4(adverbs, plural, adjectives2,plural2, get_starting_structure1(adjectives, plural))
    } else {
        Vec::from(['x'])
    }
}