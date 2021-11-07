use super::tile::Tile;

pub fn parse(input: &str) -> Vec<Vec<Tile>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().map(Tile::from).collect::<Vec<Tile>>())
        .collect::<Vec<Vec<Tile>>>()
}

pub fn cache_key(map: &[Vec<Tile>]) -> String {
    draw(map, "")
}

pub fn draw(map: &[Vec<Tile>], delimiter: &str) -> String {
    map.iter()
        .map(|row| {
            row.iter()
                .map(|&tile| {
                    let c: char = tile.into();
                    c
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(delimiter)
}
