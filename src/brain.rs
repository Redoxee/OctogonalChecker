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
            println!("{0}->{1} leads to {2} plays", Grid::get_coord_from_index(play.0), Grid::get_coord_from_index(play.1), resulting_boards.len());
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

        println!("{:?}", scores);
        println!("Of {0} plays, picked {1}->{2}", scores.len(), Grid::get_coord_from_index(scores[0].0.0), Grid::get_coord_from_index(scores[0].0.1));
        return Some(scores[0].0);
    }

    fn find_all_plays(board: &BoardState, player_side: PlayerSide, grid: &Grid) -> Vec<(usize, usize)> {
        let mut all_plays = Vec::new();

        let pawn_indexes = match player_side { PlayerSide::Top => board.top_pawns, PlayerSide::Bottom => board.bottom_pawns };
        for idx in 0..pawn_indexes.count {
            let pawn_index = pawn_indexes.tile_indexes[idx];
            match board.tiles[pawn_index] {
                Some(pawn) => {
                    if pawn.player != player_side {
                        panic!();
                    }
                    
                    let plays = board.get_possible_plays(pawn_index, player_side, grid);
                    for play in plays {
                        all_plays.push((pawn_index, play));
                    }
                },

                None => panic!("{0:?} pawn_indexes {1:?}, pawn_index {2}, tiles {3:?}", player_side, pawn_indexes, Grid::get_coord_from_index(pawn_index), board.tiles)
            }
        }

        return all_plays;
    }

    fn explore_layers(board: &BoardState, layer: u32, player_side: PlayerSide, grid: &Grid) -> Vec<BoardState> {
        let mut result = Vec::new();
        let current_plays = Brain::find_all_plays(board, player_side, grid);
        
        for play in current_plays {
            let mut next_board = board.make_move(play.0, play.1);
            let next_side = player_side.reverse();
            next_board.current_player = next_side;
            
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

        let my_pawns = match player_side { PlayerSide::Top => board.top_pawns, PlayerSide::Bottom => board.bottom_pawns };
        let their_pawns = match player_side { PlayerSide::Top => board.bottom_pawns, PlayerSide::Bottom => board.top_pawns };

        let mut score = 0;
        score = score + my_pawns.count as i32 * 5;
        score = score - their_pawns.count as i32 * 4;
        if my_pawns.count == 0 {
            score = score - 200;
        }

        if their_pawns.count == 0 {
            score = score + 100;
        }

        return score;
    }
}
