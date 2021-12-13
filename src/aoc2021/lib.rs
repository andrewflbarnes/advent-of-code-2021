mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;

pub mod utils;

macro_rules! solution {
    ( $x:ident ) => {
        pub use crate::{$x::solve as $x};
    };
}

pub mod solutions {
    solution![d04];
    solution![d05];
    solution![d06];
    solution![d07];
    solution![d08];
    solution![d09];
    solution![d10];
    solution![d11];
    solution![d12];
    solution![d13];
}
