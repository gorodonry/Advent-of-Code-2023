use std::fs;

fn main() {
    let file = fs::read_to_string("src/readings.txt").unwrap();
    let lines: Vec<&str> = file.split_terminator("\n").collect();

    let mut total: i64 = 0;
    for line in lines.into_iter() {
        let mut layers: Vec<Vec<i32>> = vec![line
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|&e| e.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()];

        while !all_zeroes(
            &layers[layers.len() - 1]
                .iter()
                .map(|&i| i as isize)
                .collect::<Vec<isize>>(),
        ) {
            let mut differences: Vec<i32> = Vec::new();
            let layer: &Vec<i32> = &layers[layers.len() - 1];

            for i in 0..(layer.len() - 1) {
                differences.push(layer[i + 1] - layer[i]);
            }

            layers.push(differences);
        }

        {
            let mut last_layer = layers.remove(layers.len() - 1);
            last_layer.push(0);
            layers.push(last_layer);
        }

        for i in (0..(layers.len() - 1)).rev() {
            let mut layer = layers.remove(i);
            layer.push(layer[layer.len() - 1] + layers[i][layer.len() - 1]);
            layers.insert(i, layer);
        }

        total += layers[0][layers[0].len() - 1] as i64;
    }

    println!("{}", total);
}

fn all_zeroes(ints: &Vec<isize>) -> bool {
    for int in ints.iter() {
        if *int != 0 {
            return false;
        }
    }

    true
}
