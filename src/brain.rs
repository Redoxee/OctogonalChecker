use std::collections::HashSet;

use crate::game::*;
use crate::grid::*;

pub struct Brain {
}

impl Brain {

    pub fn search_best_play(board: &BoardState, iteration: u32) -> Option<(usize, usize)>{
        let plays = Brain::find_all_plays(board, board.current_player);
        let mut scores = Vec::new();
        for play in plays {
            let next_board = board.make_move(play.0, play.1);
            let  predicted_result = Brain::explore_branch(next_board, iteration * 2 - 1);
            
            scores.push((play, predicted_result.1));
        }

        scores.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());

        println!("{:?}", scores);
        println!("Of {0} plays, picked {1}->{2}", scores.len(), Grid::get_coord_from_index(scores[0].0.0), Grid::get_coord_from_index(scores[0].0.1));
        return Some(scores[0].0);
    }

    fn explore_branch(board: BoardState, layer: u32) -> (BoardState, i32){
        let mut result = (board, -100);
        for _ in 0..layer {
            let plays = Brain::find_all_plays(&result.0, result.0.current_player);
            if plays.len() == 0 {
                break;
            }

            let mut ranked_plays = Vec::new();
            for play in plays {
                let next_board = result.0.make_move(play.0, play.1);
                let evaluation = Brain::evaluate_play(next_board, board.current_player);
                ranked_plays.push((next_board, evaluation));
            }

            ranked_plays.sort_by(|left, right| left.1.cmp(&right.1).reverse());
            result = ranked_plays[0];
            result.0.current_player = result.0.current_player.reverse();
        }

        return result;
    }

    fn find_all_plays(board: &BoardState, player_side: PlayerSide) -> Vec<(usize, usize)> {
        let mut all_plays = Vec::new();

        let pawn_indexes = match player_side { PlayerSide::Top => board.top_pawns, PlayerSide::Bottom => board.bottom_pawns };
        for idx in 0..pawn_indexes.count {
            let pawn_index = pawn_indexes.tile_indexes[idx];
            match board.tiles[pawn_index] {
                Some(pawn) => {
                    if pawn.player != player_side {
                        panic!();
                    }
                    
                    let plays = board.get_possible_plays(pawn_index, player_side);
                    for play in plays {
                        all_plays.push((pawn_index, play));
                    }
                },

                None => panic!("{0:?} pawn_indexes {1:?}, pawn_index {2}, tiles {3:?}", player_side, pawn_indexes, Grid::get_coord_from_index(pawn_index), board.tiles)
            }
        }

        return all_plays;
    }


    pub fn get_two_layer_moves(board: BoardState, tile_index: usize) -> (Vec<usize>, Vec<usize>){
        let first_layer = board.get_possible_moves(tile_index);
        let mut second_layer:HashSet<usize> = HashSet::new();
        for play_index in &first_layer {
            let layer = board.get_possible_moves(*play_index);
            second_layer.extend(layer.iter());
        }

        let first_layer_map : HashSet<usize> = (&first_layer).into_iter().map(|element| *element).collect();
        let second_layer = &second_layer - &first_layer_map;
        let mut second_layer : Vec<usize> = second_layer.into_iter().filter(|element| element != &tile_index).collect();
        second_layer.sort_by(|left, right|left.cmp(right));
        return (first_layer, second_layer);
    }

    pub fn evaluate_play(board: BoardState, player_side: PlayerSide) -> i32 {
        let my_pawns = match player_side { PlayerSide::Top => board.top_pawns, PlayerSide::Bottom => board.bottom_pawns };
        let their_pawns = match player_side { PlayerSide::Top => board.bottom_pawns, PlayerSide::Bottom => board.top_pawns };

        let mut score = 0;
        score = score + my_pawns.count as i32 * 20;
        score = score - their_pawns.count as i32 * 20;

        for index in 0..my_pawns.count {
            let two_layers = Brain::get_two_layer_moves(board, my_pawns.tile_indexes[index]);
            for tile_index in two_layers.0 {
                match board.tiles[tile_index] {
                    None => {},
                    Some(pawn) => {
                        if pawn.player == player_side {
                            score = score + 5;
                        }
                        else {
                            score = score - 6;
                        }
                    }
                }
            }

            for tile_index in two_layers.1 {
                match board.tiles[tile_index] {
                    None => {},
                    Some(pawn) => {
                        if pawn.player != player_side {
                            score = score + 6;
                        }
                    }
                }
            }
        }

        if my_pawns.count == 0 {
            score = score - 200;
        }

        if their_pawns.count == 0 {
            score = score + 200;
        }

        return score;
    }
}
