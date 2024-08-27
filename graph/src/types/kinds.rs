

pub enum Directed {}

pub enum Undirected {}

macro_rules! graph_ty {
    (pub enum $name:ident) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum $name {}

        impl $name {
            pub fn phantom() -> ::core::marker::PhantomData::<Self> {
                ::core::marker::PhantomData::<Self>
            }
        }
    };
}