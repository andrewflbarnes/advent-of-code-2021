mod d4;
mod d5;

pub mod utils;

macro_rules! solution {
    ( $x:ident ) => {
        pub use crate::{$x::solve as $x};
    };
}

pub mod solutions {
    solution![d4];
    solution![d5];
}
