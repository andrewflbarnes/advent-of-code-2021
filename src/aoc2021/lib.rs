mod d4;
mod d5;
mod d6;
mod d7;

pub mod utils;

macro_rules! solution {
    ( $x:ident ) => {
        pub use crate::{$x::solve as $x};
    };
}

pub mod solutions {
    solution![d4];
    solution![d5];
    solution![d6];
    solution![d7];
}
