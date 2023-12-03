// prompt

// small bag of cubes green, blue, or red

//
// Each time you play this game,
// he will hide a secret number
// of cubes of each color in the
// bag, and your goal is to figure out
// information about the number of cubes.


/*
only 12 red cubes, 13 green cubes, and 14 blue cubes.
What is the sum of the IDs of those games?
*/
use std::io;
use day_2::{check_reveal, get_cube_reveals, parse_input_data, Reveal};

fn main() -> io::Result<()> {
    let mut lines = parse_input_data();
    let mut sum_ids: u128 = 0;
    for line in lines.iter() {
        let reveals: Vec<Reveal> = get_cube_reveals(line);
        for reveal in reveals {
            if check_reveal(&reveal) {
                sum_ids += reveal.game_id as u128;
            }
        }

    }
    println!("{}", sum_ids);

    Ok(())
}
