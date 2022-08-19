use crate::game::*;
use crate::grid::*;

pub struct Brain {
    pub player_side: PlayerSide,
}

impl Brain {
    pub fn enumerate_moves(&self, board: &BoardState, grid: &Grid) {
        for tile_index in 0..NUMBER_OF_TILES {
            match board.tiles[tile_index] {
                Some(pawn)=> {
                    if pawn.player != board.current_player {
                        break;
                    }
                    
                    let plays = board.get_possible_plays(tile_index, self.player_side, grid);
                    for play in plays {
                        println!("{0} -> {1}", grid.get_coord_from_index(tile_index), grid.get_coord_from_index(play));
                    }
                },
                None=>{},
            }
        }
    }
}
