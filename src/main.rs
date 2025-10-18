use std::{collections::HashMap, io};

fn main() {
    let mut has_winner: bool = false;
    let mut player_turn: u8 = 1;
    let mut player_one_selections: Vec<u8> = vec![];
    let mut player_two_selections: Vec<u8> = vec![];
    let mut marked_positions: HashMap<u8, String> = HashMap::new();

    display_board(&marked_positions);

    while !has_winner {
        if player_turn == 1 {
            let selection = make_selection(&player_turn, &mut player_one_selections, &marked_positions);
            marked_positions.insert(selection, "X".to_string());
            display_board(&marked_positions);
            if player_one_selections.len() >= 3 {
                has_winner = check_if_winner(&player_one_selections);
                if has_winner {
                    println!("Player 1 is the winner!");
                }
            }
            player_turn = 2;
        } else {
            let selection = make_selection(&player_turn, &mut player_two_selections, &marked_positions);
            marked_positions.insert(selection, "O".to_string());
            display_board(&marked_positions);
            if player_two_selections.len() >= 3 {
                has_winner = check_if_winner(&player_two_selections);
                if has_winner {
                    println!("Player 2 is the winner!");
                }
            }
            player_turn = 1;
        }

        if player_one_selections.len() + player_two_selections.len() == 9 && !has_winner {
            println!("Game has finished in draw.");
            has_winner = true;
        }
    }
}

fn make_selection(player: &u8, selection_list: &mut Vec<u8>, marked_positions: &HashMap<u8, String>) -> u8 {
    println!("Player {} to act: type 1 to 9", player);
    let mut selection = String::new();
    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line");

    let parsed_selection = match selection.trim().parse::<u8>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number between 1 and 9.");
            return make_selection(player, selection_list, marked_positions);
        }
    };

    if parsed_selection < 1 || parsed_selection > 9 {
        println!("Selection should be between 1 and 9");
        return make_selection(player, selection_list, marked_positions);
    }
    if marked_positions.contains_key(&parsed_selection) {
        println!("This position is already marked.");
        return make_selection(player, selection_list, marked_positions);
    }

    selection_list.push(parsed_selection);
    parsed_selection
}

fn display_board(marked_positions: &HashMap<u8, String>) {   
   println!("   |   |   "); 
   println!(" {} | {} | {} ",    
   marked_positions.get(&1).unwrap_or(&" ".to_string()), 
   marked_positions.get(&2).unwrap_or(&" ".to_string()), 
   marked_positions.get(&3).unwrap_or(&" ".to_string()));
   println!("___|___|___");
   println!("   |   |   ");
   println!(" {} | {} | {} ",    
   marked_positions.get(&4).unwrap_or(&" ".to_string()), 
   marked_positions.get(&5).unwrap_or(&" ".to_string()), 
   marked_positions.get(&6).unwrap_or(&" ".to_string()));
   println!("___|___|___");
   println!("   |   |   ");
   println!(" {} | {} | {} ",    
   marked_positions.get(&7).unwrap_or(&" ".to_string()), 
   marked_positions.get(&8).unwrap_or(&" ".to_string()), 
   marked_positions.get(&9).unwrap_or(&" ".to_string()));
   println!("   |   |   ");
}

fn check_if_winner(selection_list: &Vec<u8>) -> bool { 
    let winning_combinations: [[u8; 3]; 8] = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
        [1, 4, 7],
        [2, 5, 8],
        [3, 6, 9],
        [1, 5, 9],
        [3, 5, 7]
    ];

    let mut winner = false;

    for combination in winning_combinations {
        if combination.iter().all(|pos| selection_list.contains(pos)) {
            winner = true;
            break;
        }
    }
    winner
}