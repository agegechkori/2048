use crate::random::RandomNumberGenerator;
use crate::tile_generator::TileGenerator;

struct Board<R: RandomNumberGenerator> {
    cells: Vec<Vec<i32>>,
    generator: TileGenerator<R>,
}