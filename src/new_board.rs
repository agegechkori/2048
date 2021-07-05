use crate::random::RandomNumberGenerator;
use crate::tile_generator::RandomTileGenerator;

struct Board<R: RandomNumberGenerator> {
    cells: Vec<Vec<i32>>,
    generator: RandomTileGenerator<R>,
}