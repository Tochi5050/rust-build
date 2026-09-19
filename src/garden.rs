pub mod vegetables;

use crate::garden::vegetables::Asparagus;

pub fn garde_fn () -> String {
    let asparagus: Asparagus = Asparagus {
         color: String::from("green"),
         size: 10
    };

    asparagus.color
}
