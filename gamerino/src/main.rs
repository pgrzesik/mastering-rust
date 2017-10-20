enum Direction {
    West,
    East,
    North,
    South
}

#[derive(Debug, PartialEq)]
enum MovementError {
    NotBeingInSquare
}

#[derive(PartialEq, Debug)]
enum TerrainGround {
    Soil,
    Stone
}

#[derive(PartialEq, Debug)]
enum TerrainBlock {
    Tree,
    Soil,
    Stone
}

#[derive(PartialEq, Debug)]
enum Being {
    Orc,
    Human
}


struct Square {
    ground: TerrainGround,
    block: Option<TerrainBlock>,
    being: Option<Being>
}


struct Grid {
    size: (usize, usize),
    squares: Vec<Square>
}

impl Grid {
    fn generate_empty(x_size: usize, y_size: usize) -> Grid {
        let number_of_squares = x_size * y_size;
        let mut squares: Vec<Square> = Vec::with_capacity(number_of_squares);

        for _ in 0..number_of_squares {
            squares.push(Square {
                ground: TerrainGround::Soil,
                block: None,
                being: None
            })
        }

        Grid {
            size: (x_size, y_size),
            squares
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_empty_grid() {
        let grid = ::Grid::generate_empty(5, 13);
        assert_eq!(grid.size, (5, 13));
        let mut number_of_squares = 0;

        for square in &grid.squares {
            assert_eq!(square.ground, ::TerrainGround::Soil);
            assert_eq!(square.block, None);
            assert_eq!(square.being, None);
            number_of_squares += 1;
        }

        assert_eq!(grid.squares.len(), 5*13);
        assert_eq!(number_of_squares, 5*13);
    }
}

fn main() {
    println!("Hello, world!");
}
