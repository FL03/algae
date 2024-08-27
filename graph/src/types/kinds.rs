/*
    Appellation: kinds <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/


macro_rules! uninit {
    (@impl $vis:vis enum $name:ident) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis enum $name {}

        impl $name {
            pub fn phantom() -> ::core::marker::PhantomData::<Self> {
                ::core::marker::PhantomData::<Self>
            }
        }
    };

    ($($vis:vis $name:ident),* $(,)?) => {
        $(
            uninit!(@impl $vis enum $name);
        )*
    };
}

uninit! {
    pub Directed, 
    pub Undirected,
}