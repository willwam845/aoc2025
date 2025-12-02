mod util;

use seq_macro::seq;

type AocFn = fn(&str) -> either::Either<u64, String>;

seq! {
    N in 01..=01 {
        #(
            pub mod day~N;
        )*
        pub static FUNCS: &[(AocFn, AocFn)] = &[
            #(
                (day~N::part1 as _, day~N::part2 as _),
            )*
        ];
    }
}