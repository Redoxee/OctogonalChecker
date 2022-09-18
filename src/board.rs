use crate::{
    pawn::*,
    tiles::*,
    grid::*,
    game::*,
};

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct BoardState {
    pub tiles: [Option<Pawn>; NUMBER_OF_TILES],
    pub current_player: PlayerSide,
    pub top_pawns: PawnArray,
    pub bottom_pawns: PawnArray,
}

impl BoardState {
    pub fn add_pawn(&mut self, coord: TileCoord, player: PlayerSide) {
        let tile_index = Grid::get_index_from_coord(coord).unwrap();
        match self.tiles[tile_index] {
            Some(_) => panic!(),
            None => (),
        }

        let pawn_array = match player {
            PlayerSide::Top => &mut self.top_pawns,
            PlayerSide::Bottom => &mut self.bottom_pawns,
        };

        self.tiles[tile_index] = Some(Pawn{player, table_index: pawn_array.count});
        (*pawn_array).tile_indexes[pawn_array.count] = tile_index;
        (*pawn_array).count = pawn_array.count + 1;
    }
    
    pub fn make_move(&self, source_index: usize, play_index: usize) -> BoardState {
        let mut board = self.clone();

        match board.tiles[play_index] {
            None=>(),
            Some(pawn) => {
                let pawn_array = match pawn.player {
                    PlayerSide::Top => &mut board.top_pawns,
                    PlayerSide::Bottom => &mut board.bottom_pawns
                };
                
                if pawn_array.count > 1 && pawn.table_index < pawn_array.count - 1 { 
                    let replacing_pawn_tile_index = pawn_array.tile_indexes[pawn_array.count - 1];
                    let replacing_pawn = match &mut board.tiles[replacing_pawn_tile_index] {
                        Some(other_pawn) => other_pawn,
                        None => panic!(),
                    };

                    (*replacing_pawn).table_index = pawn.table_index;
                    (*pawn_array).tile_indexes[pawn.table_index] = pawn_array.tile_indexes[pawn_array.count - 1];
                }
                
                (*pawn_array).count = pawn_array.count - 1;
            }
        }

        let pawn = board.tiles[source_index];
        board.tiles[play_index] = pawn;
        board.tiles[source_index] = Option::None;

        match pawn {
            Some(pawn) => {
                let tile_array = match pawn.player {
                    PlayerSide::Top => &mut board.top_pawns, 
                    PlayerSide::Bottom => &mut board.bottom_pawns
                };

                (*tile_array).tile_indexes[pawn.table_index] = play_index;
                assert!(pawn.table_index < tile_array.count);
            },
            None => {panic!();}
        }

        board.current_player = board.current_player.reverse();
        return board;
    }
    
    pub fn get_possible_moves(&self, tile_index: usize) -> Vec<usize> {
        let coord = Grid::get_coord_from_index(tile_index);
        let mut possible_plays = Vec::new();
        
        match Grid::get_tile_shape_from_index(tile_index) {
            TileShape::Quad => {
                if let Some(index) = Grid::get_index_from_coord(TileCoord{ x: coord.x + 1, y: coord.y }) {possible_plays.push(index)};
                if let Some(index) = Grid::get_index_from_coord(TileCoord{ x: coord.x - 1, y: coord.y }) {possible_plays.push(index)};
                if let Some(index) = Grid::get_index_from_coord(TileCoord{ x: coord.x + 1, y: coord.y - 1}) {possible_plays.push(index)};
                if let Some(index) = Grid::get_index_from_coord(TileCoord{ x: coord.x - 1, y: coord.y - 1}) {possible_plays.push(index)};
            },

            TileShape::Octo => {
                if let Some(index) = Grid::get_index_from_coord(TileCoord{ x: coord.x + 2, y: coord.y }) {possible_plays.push(index)};
                if let Some(index) = Grid::get_index_from_coord(TileCoord{ x: coord.x - 2, y: coord.y }) {possible_plays.push(index)};
                if let Some(index) = Grid::get_index_from_coord(TileCoord{ x: coord.x    , y: coord.y + 1}) {possible_plays.push(index)};
                if let Some(index) = Grid::get_index_from_coord(TileCoord{ x: coord.x    , y: coord.y - 1}) {possible_plays.push(index)};

                if let Some(index) = Grid::get_index_from_coord(TileCoord{ x: coord.x + 1, y: coord.y }) {possible_plays.push(index)};
                if let Some(index) = Grid::get_index_from_coord(TileCoord{ x: coord.x - 1, y: coord.y }) {possible_plays.push(index)};
                if let Some(index) = Grid::get_index_from_coord(TileCoord{ x: coord.x + 1, y: coord.y + 1}) {possible_plays.push(index)};
                if let Some(index) = Grid::get_index_from_coord(TileCoord{ x: coord.x - 1, y: coord.y + 1}) {possible_plays.push(index)};
            },
        }

        return possible_plays;
    }

    pub fn get_possible_plays(&self, tile_index: usize, player_side: PlayerSide) -> Vec<usize> {
        let mut possible_plays = self.get_possible_moves(tile_index);

        possible_plays.retain(|&index| match self.tiles[index] {
            Some(pawn) => { pawn.player != player_side },
            None => true
        });

        return possible_plays;
    }
}
