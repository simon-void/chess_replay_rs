use std::fmt;
use std::str;
use crate::base::position::Position;
use tinyvec::TinyVec;
use crate::base::{ChessError, ErrorKind};
use tinyvec::alloc::fmt::Formatter;
use crate::figure::FigureType;
use std::hash::{Hash, Hasher};
use serde::Serialize;
use crate::base::MoveType::{Castling, EnPassant, Normal, PawnPromotion};


#[derive(Debug, Copy, Clone, Serialize)]
pub struct Move {
    pub main_move: FromTo,
    // figure_captured and is_pawn_move are not as useful as normally for a chess engine
    // but still nice to have for 3-fold repetition computation
    pub figure_captured: Option<FigureType>,
    pub is_pawn_move: bool,
    pub move_type: MoveType, // TODO: make this a Box<MoveType> or Rc<MoveType> together with a static lifetime instance of Rc/Box<MoveType::Normal>
}

impl Move {
    pub fn new(
        main_move: FromTo,
        figure_caught: Option<FigureType>,
    ) -> Move {
        Move {
            main_move,
            figure_captured: figure_caught,
            is_pawn_move: false,
            move_type: Normal
        }
    }

    pub fn new_en_passant(main_move: FromTo) -> Move {
        Move {
            main_move,
            figure_captured: Some(FigureType::Pawn),
            is_pawn_move: true,
            move_type: EnPassant,
        }
    }

    pub fn new_castling(king_from: Position, rook_from: Position) -> Move {
        let king_to: Position = "".parse().unwrap();
        let rook_to: Position = "".parse().unwrap();
        let castling_type: CastlingType = CastlingType::KingSide;
        Move {
            main_move: FromTo::new(king_from, king_to),
            figure_captured: None,
            is_pawn_move: false,
            move_type: Castling(castling_type, FromTo::new(rook_from, rook_to)),
        }
    }

    pub fn did_catch_figure(&self) -> bool {
        self.figure_captured.is_some()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FromTo {
    pub from: Position,
    pub to: Position,
}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for FromTo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize((self.from.index<< 6) + self.to.index);
    }
}

impl FromTo {
    pub fn new(from: Position, to: Position) -> FromTo {
        FromTo {
            from,
            to,
        }
    }

    pub fn from_code(code: &str) -> FromTo {
        code.parse::<FromTo>().unwrap_or_else(|_| panic!("illegal Move code: {}", code))
    }

    pub fn toggle_rows(&self) -> FromTo {
        FromTo {
            from: self.from.toggle_row(),
            to: self.to.toggle_row(),
        }
    }
}

impl str::FromStr for FromTo {
    type Err = ChessError;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        Ok(FromTo {
            from: code[0..2].parse::<Position>()?,
            to: code[3..5].parse::<Position>()?,
        })
    }
}

impl fmt::Display for FromTo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.from, self.to)
    }
}

impl fmt::Debug for FromTo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Serialize for FromTo {
    fn serialize<S>(&self, serializer: S) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error> where
        S: serde::Serializer {
        serializer.serialize_str(&format!("{}", self))
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BasicMove {
    pub from_to: FromTo,
    pub promotion_type: Option<PromotionType>,
}

impl BasicMove {
    pub fn new(from_to: FromTo) -> BasicMove {
        BasicMove {
            from_to,
            promotion_type: None,
        }
    }

    pub fn new_with_promotion(from_to: FromTo, promotion_type: PromotionType) -> BasicMove {
        BasicMove {
            from_to,
            promotion_type: Some(promotion_type),
        }
    }
}

impl str::FromStr for BasicMove {
    type Err = ChessError;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        match code.len() {
            4 => {
                let from_to = code.parse::<FromTo>()?;
                Ok(BasicMove::new(from_to))
            }
            5 => {
                let from_to = code[0..4].parse::<FromTo>()?;
                let pawn_move_type = code[4..5].parse::<PromotionType>()?;
                Ok(BasicMove::new_with_promotion(from_to, pawn_move_type))
            }
            _ => {
                return Err(ChessError {
                    msg: format!("illegal move format: {}", code),
                    kind: ErrorKind::IllegalFormat,
                })
            }
        }
    }
}

impl fmt::Display for BasicMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.from_to)?;
        if let Some(promotion_type) = self.promotion_type {
            write!(f, "{}", promotion_type)?
        };
        Ok(())
    }
}

impl fmt::Debug for BasicMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

// Default is needed, so that Move can be stored in a TinyVec
impl Default for BasicMove {
    fn default() -> Self {
        BasicMove::new(FromTo::new(
            // default values should never be used, so illegal values are fine
            // (they are necessary for TinyVec)
            Position::new_unchecked(9, 9),
            Position::new_unchecked(9, 9),
        ))
    }
}

impl Serialize for BasicMove {
    fn serialize<S>(&self, serializer: S) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error> where
        S: serde::Serializer {
        serializer.serialize_str(&format!("{}", self))
    }
}

pub const EXPECTED_MAX_NUMBER_OF_MOVES: usize = 80;

#[derive(Clone)]
pub struct MoveArray {
    array: [BasicMove; EXPECTED_MAX_NUMBER_OF_MOVES]
}

impl tinyvec::Array for MoveArray {
    type Item = BasicMove;
    const CAPACITY: usize = EXPECTED_MAX_NUMBER_OF_MOVES;

    fn as_slice(&self) -> &[Self::Item] {
        &self.array
    }

    fn as_slice_mut(&mut self) -> &mut [Self::Item] {
        &mut self.array
    }

    fn default() -> Self {
        MoveArray {
            array: [BasicMove::default(); EXPECTED_MAX_NUMBER_OF_MOVES]
        }
    }
}

pub type Moves = TinyVec<MoveArray>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PromotionType {
    Rook,
    Knight,
    Bishop,
    Queen,
}

impl PromotionType {
    pub fn get_figure_type(&self) -> FigureType {
        match self {
            PromotionType::Rook => {FigureType::Rook}
            PromotionType::Knight => {FigureType::Knight}
            PromotionType::Bishop => {FigureType::Bishop}
            PromotionType::Queen => {FigureType::Queen}
        }
    }
}

impl str::FromStr for PromotionType {
    type Err = ChessError;

    fn from_str(s: &str) -> Result<PromotionType, Self::Err> {
        match s {
            "Q" => Ok(PromotionType::Queen),
            "R" => Ok(PromotionType::Rook),
            "K" => Ok(PromotionType::Knight),
            "B" => Ok(PromotionType::Bishop),
            _ => Err(ChessError{
                msg: format!("unknown pawn promotion type: {}. Only 'QRKB' are allowed.", s),
                kind: ErrorKind::IllegalFormat
            }),
        }
    }
}

impl fmt::Display for PromotionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let code = match self {
            PromotionType::Queen => "Q",
            PromotionType::Rook => "R",
            PromotionType::Knight => "K",
            PromotionType::Bishop => "B",
        };
        write!(f, "{}", code)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CastlingType {
    KingSide,
    QueenSide,
}

impl fmt::Display for MoveType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let code = match self {
            Normal => "-",
            PawnPromotion(PromotionType::Queen) => "Q",
            PawnPromotion(PromotionType::Rook) => "R",
            PawnPromotion(PromotionType::Knight) => "K",
            PawnPromotion(PromotionType::Bishop) => "B",
            EnPassant => "e",
            Castling(CastlingType::KingSide, _) => "c",
            Castling(CastlingType::QueenSide, _) => "C",
        };
        write!(f, "{}", code)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MoveType {
    Normal,
    PawnPromotion(PromotionType),
    EnPassant,
    Castling(CastlingType, /*rookMove:*/FromTo) // TODO: is CastlingType needed?
}
