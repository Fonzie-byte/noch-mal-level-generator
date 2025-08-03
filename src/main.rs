#[derive(Debug)]
enum Color {
    Green,
    Yellow,
    Cyan,
    Pink,
    Orange,
}

#[derive(Debug)]
struct Column {
    letter: char,
    cells: [Color; 7],
    star_position: u8,
}

#[derive(Debug)]
struct Board {
    columns: [Column; 15],
}

fn main() {
    let board = Board {
        columns: [
            Column {
                letter: 'A',
                cells: [
                    Color::Green,
                    Color::Pink,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Orange,
                    Color::Yellow,
                ],
                star_position: 2,
            },
            Column {
                letter: 'A',
                cells: [
                    Color::Green,
                    Color::Pink,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Orange,
                    Color::Yellow,
                ],
                star_position: 2,
            },
            Column {
                letter: 'A',
                cells: [
                    Color::Green,
                    Color::Pink,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Orange,
                    Color::Yellow,
                ],
                star_position: 2,
            },
            Column {
                letter: 'A',
                cells: [
                    Color::Green,
                    Color::Pink,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Orange,
                    Color::Yellow,
                ],
                star_position: 2,
            },
            Column {
                letter: 'A',
                cells: [
                    Color::Green,
                    Color::Pink,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Orange,
                    Color::Yellow,
                ],
                star_position: 2,
            },
            Column {
                letter: 'A',
                cells: [
                    Color::Green,
                    Color::Pink,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Orange,
                    Color::Yellow,
                ],
                star_position: 2,
            },
            Column {
                letter: 'A',
                cells: [
                    Color::Green,
                    Color::Pink,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Orange,
                    Color::Yellow,
                ],
                star_position: 2,
            },
            Column {
                letter: 'A',
                cells: [
                    Color::Green,
                    Color::Pink,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Orange,
                    Color::Yellow,
                ],
                star_position: 2,
            },
            Column {
                letter: 'A',
                cells: [
                    Color::Green,
                    Color::Pink,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Orange,
                    Color::Yellow,
                ],
                star_position: 2,
            },
            Column {
                letter: 'A',
                cells: [
                    Color::Green,
                    Color::Pink,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Orange,
                    Color::Yellow,
                ],
                star_position: 2,
            },
            Column {
                letter: 'A',
                cells: [
                    Color::Green,
                    Color::Pink,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Orange,
                    Color::Yellow,
                ],
                star_position: 2,
            },
            Column {
                letter: 'A',
                cells: [
                    Color::Green,
                    Color::Pink,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Orange,
                    Color::Yellow,
                ],
                star_position: 2,
            },
            Column {
                letter: 'A',
                cells: [
                    Color::Green,
                    Color::Pink,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Orange,
                    Color::Yellow,
                ],
                star_position: 2,
            },
            Column {
                letter: 'A',
                cells: [
                    Color::Green,
                    Color::Pink,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Orange,
                    Color::Yellow,
                ],
                star_position: 2,
            },
            Column {
                letter: 'A',
                cells: [
                    Color::Green,
                    Color::Pink,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Cyan,
                    Color::Orange,
                    Color::Yellow,
                ],
                star_position: 2,
            },
        ],
    };

    dbg!(board);
}
