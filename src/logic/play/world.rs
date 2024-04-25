pub struct World {
    pub tiles: [[bool; 30]; 50],
}

impl World {
    pub fn new() -> Self {
        let mut tiles = [[false; 30]; 50];

        for x in 0..50 {
            for y in 0..30 {
                let dx = x as f32 - 25.0;
                let dy = y as f32 - 15.0;
                let distance = (dx * dx + dy * dy).sqrt();

                tiles[x][y] = ((distance < 30.0) && (distance > 20.0)) || (distance < 8.0);
            }
        }

        return Self {
            tiles
        };
    }
}