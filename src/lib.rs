use anyhow::{bail, Result};
use std::fmt;

#[derive(Debug)]
pub struct GameBoard {
    slots: [[Option<GamePiece>; 8]; 8],
}

#[derive(Debug, Clone, Copy)]
pub struct GamePiece {
    kind: PieceKind,
    side: Side,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Position(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    // Starts at rows 6 and 7
    Black,

    // Starts at rows 0 and 1
    White,
}

impl GameBoard {
    #[must_use]
    pub fn new() -> Self {
        let mut slots = [[None; 8]; 8];

        // Spawn the pawns
        for col in 0..slots.len() {
            slots[1][col] = Some(GamePiece::new(PieceKind::Pawn, Side::White));
            slots[6][col] = Some(GamePiece::new(PieceKind::Pawn, Side::Black));
        }

        // Spawn the rooks
        slots[0][7] = Some(GamePiece::new(PieceKind::Rook, Side::White));
        slots[0][0] = Some(GamePiece::new(PieceKind::Rook, Side::White));
        slots[7][7] = Some(GamePiece::new(PieceKind::Rook, Side::Black));
        slots[7][0] = Some(GamePiece::new(PieceKind::Rook, Side::Black));

        // Spawn the knights
        slots[0][6] = Some(GamePiece::new(PieceKind::Knight, Side::White));
        slots[0][1] = Some(GamePiece::new(PieceKind::Knight, Side::White));
        slots[7][6] = Some(GamePiece::new(PieceKind::Knight, Side::Black));
        slots[7][1] = Some(GamePiece::new(PieceKind::Knight, Side::Black));

        // Spawn the bishops
        slots[0][5] = Some(GamePiece::new(PieceKind::Bishop, Side::White));
        slots[0][2] = Some(GamePiece::new(PieceKind::Bishop, Side::White));
        slots[7][5] = Some(GamePiece::new(PieceKind::Bishop, Side::Black));
        slots[7][2] = Some(GamePiece::new(PieceKind::Bishop, Side::Black));

        // Spawn the queens and kings
        slots[0][4] = Some(GamePiece::new(PieceKind::King, Side::White));
        slots[0][3] = Some(GamePiece::new(PieceKind::Queen, Side::White));
        slots[7][4] = Some(GamePiece::new(PieceKind::Queen, Side::Black));
        slots[7][3] = Some(GamePiece::new(PieceKind::King, Side::Black));

        Self { slots }
    }

    pub fn move_piece(&mut self, src: Position, dst: Position) -> Result<()> {
        if self.slots[dst.y() as usize][dst.x() as usize].is_some() {
            bail!("target slot already has a piece!")
        }

        let piece_slot = (src.y() as usize, src.x() as usize);
        if let Some(mut piece) = self.slots[piece_slot.0][piece_slot.1] {
            if matches!(piece.kind(), PieceKind::Bishop | PieceKind::Queen) {
                let hor_mul = if dst.x() as i8 - piece_slot.1 as i8 > 0 {
                    1
                } else {
                    -1
                };
                let ver_mul = if dst.y() as i8 - piece_slot.0 as i8 > 0 {
                    1
                } else {
                    -1
                };

                for i in 0..(dst.x() as i8 - piece_slot.0 as i8).abs() {
                    if self.slots[piece_slot.1 + (i * ver_mul) as usize]
                        [piece_slot.1 + (i * hor_mul) as usize]
                        .is_some()
                    {
                        bail!("piece in path of movement!");
                    }
                }
            }

            if matches!(piece.kind(), PieceKind::Rook | PieceKind::Queen) {
                if piece_slot.1 as u8 == dst.x() {
                    let mul = if dst.x() as i8 - piece_slot.1 as i8 > 0 {
                        1
                    } else {
                        -1
                    };
                    for i in 0..(dst.x() as i8 - piece_slot.1 as i8).abs() {
                        if self.slots[piece_slot.0][piece_slot.1 + (i * mul) as usize].is_some() {
                            bail!("piece in path of movement!");
                        }
                    }
                } else {
                    let mul = if dst.y() as i8 - piece_slot.0 as i8 > 0 {
                        1
                    } else {
                        -1
                    };
                    for i in 0..(dst.y() as i8 - piece_slot.0 as i8).abs() {
                        if self.slots[piece_slot.1][piece_slot.0 + (i * mul) as u8 as usize]
                            .is_some()
                        {
                            bail!("piece in path of movement!");
                        }
                    }
                }
            }

            let moved = (dst.x() as i8 - src.x() as i8, dst.y() as i8 - src.y() as i8);
            match piece.kind {
                PieceKind::Pawn => {
                    if moved.0 != 0 {
                        bail!("invalid move!");
                    }

                    match piece.side {
                        Side::White => {
                            if !(moved.1 == 2 && piece_slot.0 == 1 || moved.1 == 1) {
                                bail!("invalid move!")
                            }
                        }
                        Side::Black => {
                            if !(moved.1 == -2 && piece_slot.0 == 6 || moved.1 == -1) {
                                bail!("invalid move!")
                            }
                        }
                    }
                    match piece_slot.0 {
                        0 | 7 => piece.promote(PieceKind::Queen)?,
                        _ => (),
                    }
                }
                PieceKind::Rook => {
                    if moved.0 != 0 && moved.1 != 0 || moved.0 == 0 && moved.1 == 0 {
                        bail!("invalid move!")
                    }
                }
                PieceKind::Bishop => {
                    if moved.0.abs() != moved.1.abs() {
                        bail!("invalid move!")
                    }
                }
                PieceKind::Knight => {
                    if !matches!((moved.0.abs(), moved.1.abs()), (1, 2) | (2, 1)) {
                        bail!("invalid move!")
                    }
                }
                PieceKind::Queen => {
                    let legal = match moved {
                        (0, y) if y != 0 => true,
                        (x, 0) if x != 0 => true,
                        (x, y) if x != 0 && y != 0 && x.abs() == y.abs() => true,
                        _ => false,
                    };

                    if !legal {
                        bail!("invalid move!")
                    }
                }
                PieceKind::King => {
                    if !(moved.0.abs() <= 1 && moved.1.abs() <= 1) {
                        bail!("invalid move!")
                    }
                }
            }

            self.slots[dst.y() as usize][dst.x() as usize] = Some(
                self.slots[src.y() as usize][src.x() as usize]
                    .take()
                    .unwrap(),
            );
            Ok(())
        } else {
            bail!("selected slot is empty!")
        }
    }
}

impl Default for GameBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for GameBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();

        for row in self.slots {
            for slot in row {
                out.push(match slot {
                    None => ' ',
                    Some(piece) => match piece.kind() {
                        PieceKind::Pawn => 'p',
                        PieceKind::Bishop => 'b',
                        PieceKind::Knight => 'k',
                        PieceKind::Rook => 'r',
                        PieceKind::Queen => 'q',
                        PieceKind::King => 'K',
                    },
                });
            }
            out.push('\n');
        }

        write!(f, "{}", out)
    }
}

impl GamePiece {
    #[must_use]
    pub fn new(kind: PieceKind, side: Side) -> Self {
        Self { kind, side }
    }

    #[must_use]
    pub fn side(&self) -> Side {
        self.side
    }
    #[must_use]
    pub fn kind(&self) -> PieceKind {
        self.kind
    }

    fn promote(&mut self, target: PieceKind) -> Result<()> {
        match target {
            PieceKind::Pawn | PieceKind::King => bail!("illegal promotion!"),
            _ => {
                self.kind = target;
                Ok(())
            }
        }
    }
}

impl Position {
    pub fn new(row: u8, col: u8) -> Result<Self> {
        if row > 7 || col > 7 {
            bail!("invalid position!")
        }
        Ok(Self(row << 4 | col))
    }

    #[must_use]
    pub fn x(&self) -> u8 {
        self.0 >> 4
    }
    #[must_use]
    pub fn y(&self) -> u8 {
        self.0 & 0x0f
    }
}
