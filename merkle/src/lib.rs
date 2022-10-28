/*
   Appellation: algae-merkle <library>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
*/

pub mod crypto;
pub mod merkle;
pub mod proofs;

#[macro_export]
macro_rules! join {
    ( $( $x:expr ),* ) => {
        {
            let mut tmp = String::new();
            $(
                tmp.push_str($x);
            )*
            tmp
        }
    };
}
