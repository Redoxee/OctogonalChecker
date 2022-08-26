use crate::game::*;
use crate::grid::*;

pub struct Brain {
}

impl Brain {

    pub fn search_best_play(board: &BoardState, iteration: u32,grid: &Grid) -> Option<(usize, usize)>{
        let plays = Brain::find_all_plays(board, board.current_player, grid);
        let mut scores = Vec::new();
        for play in plays {
            let next_board = board.make_move(play.0, play.1);
            let resulting_boards = Brain::explore_layers(&next_board, iteration * 2, board.current_player, grid);
            println!("{0}->{1} leads to {2} plays", grid.get_coord_from_index(play.0), grid.get_coord_from_index(play.1), resulting_boards.len());
            let mut score = 0;
            let number_of_plays = resulting_boards.len() as i32;
            if number_of_plays == 0 {
                return None;
            }

            for board in resulting_boards {
                score = score + Brain::evaluate_play(board, board.current_player);
            }

            score = score / number_of_plays;
            scores.push((play, score));
        }

        scores.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());

        println!("Of {0} plays, picked {1}->{2}", scores.len(), grid.get_coord_from_index(scores[0].0.0), grid.get_coord_from_index(scores[0].0.1));
        return Some(scores[0].0);
    }

    fn find_all_plays(board: &BoardState, player_side: PlayerSide, grid: &Grid) -> Vec<(usize, usize)> {
        let mut all_plays = Vec::new();

        for tile_index in 0..NUMBER_OF_TILES {
            match board.tiles[tile_index] {
                Some(pawn)=> {
                    if pawn.player != player_side {
                        continue;
                    }
                    
                    let plays = board.get_possible_plays(tile_index, player_side, grid);
                    for play in plays {
                        all_plays.push((tile_index, play));
                    }
                },
                
                None=>{},
            }
        }

        return all_plays;
    }

    fn explore_layers(board: &BoardState, layer: u32, player_side: PlayerSide, grid: &Grid) -> Vec<BoardState> {
        // println!("Exploring layer {0}", layer);
        let mut result = Vec::new();
        let current_plays = Brain::find_all_plays(board, player_side, grid);
        
        for play in current_plays {
            let next_board = board.make_move(play.0, play.1);
            let next_side = player_side.reverse();
            // println!("Looking at {0}->{1}", grid.get_coord_from_index(play.0), grid.get_coord_from_index(play.1));
            if layer > 0 {
                let resulting_plays = Brain::explore_layers(&next_board, layer - 1, next_side, grid);
                result.extend(resulting_plays);
            }
            else
            {
                result.push(next_board);
            }
        }

        return result;
    }

    pub fn evaluate_play(board: BoardState, player_side: PlayerSide) -> i32 {
        if player_side != board.current_player {
            return -100;
        }

        let mut score = 0;
        let mut self_pawn_count = 0;
        let mut other_pawn_count = 0;
        for tile_index in 0..NUMBER_OF_TILES {
            match board.tiles[tile_index] {
                Some(pawn)=> {
                    if pawn.player == board.current_player {
                        score = score + 5;
                        self_pawn_count = self_pawn_count + 1;
                    }
                    else
                    {
                        score = score - 4;
                        other_pawn_count = other_pawn_count + 1;
                    }
                }
                None=> {}
            }
        }

        if self_pawn_count == 0 {
            score = score - 100;
        }

        if other_pawn_count == 0 {
            score = score + 100;
        }

        return score;
    }
}
