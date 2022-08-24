use crate::game::*;
use crate::grid::*;

pub struct Brain {
}

impl Brain {

    pub fn search_best_play(board: &BoardState, iteration: u32,grid: &Grid) -> (usize, usize){
        let plays = Brain::find_all_plays(board, board.current_player, grid);
        let mut scores = Vec::new();
        for play in plays {
            let resulting_boards = Brain::explore_all_plays(board, iteration, grid);
            let score = 0;
            for board in resulting_boards {
                score = score + Brain::evaluate_play(board, board.current_player);
            }

            score = score / resulting_boards.len() as i32;
            scores.push((play, score));
        }

        scores.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        return scores[0].0;
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

    pub fn explore_all_plays(board: &BoardState, iteration: u32, grid: &Grid) -> Vec<BoardState>{
        let possible_plays = Brain::explore_layer(board, iteration * 2, board.current_player, grid);
        for board in &possible_plays {
            Brain::evaluate_play(*board);
        }

        println!("found {0} results", possible_plays.len());
    }

    fn explore_layer(board: BoardState, layer: u32, player_side: PlayerSide, grid: &Grid) -> Vec<BoardState> {
        println!("Exploring layer {0}", layer);
        let mut result = Vec::new();
        for tile_index in 0..NUMBER_OF_TILES {
            match board.tiles[tile_index] {
                Some(pawn)=> {
                    if pawn.player != player_side {
                        continue;
                    }
                    
                    let plays = board.get_possible_plays(tile_index, player_side, grid);
                    println!("Possible play on layer {0} : {1}", layer, plays.len());

                    for play in plays {
                        let mut new_board = board.make_move(tile_index, play);
                        if layer > 0 {
                            new_board.current_player = new_board.current_player.reverse();
                            return Brain::explore_layer(new_board, layer - 1, player_side.reverse(), grid);
                        }
                        else {
                            result.push(new_board);
                        }
                    }
                },
                
                None=>{},
            }
        }

        println!("Stopped at layer {0}", layer);
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
