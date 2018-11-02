pub(crate) trait CastTo<T: Copy> {
    fn cast_to(self) -> T;
}

macro_rules! impl_cast_to {
    ($typ:ty) => {
        impl CastTo<$typ> for $typ {
            fn cast_to(self) -> $typ {
                self
            }
        }
    };

    ($src:ty, $dst:ty) => {
        impl CastTo<$dst> for $src {
            fn cast_to(self) -> $dst {
                self as $dst
            }
        }
    };
}

impl_cast_to!(u8);
impl_cast_to!(u8, u16);
impl_cast_to!(u8, u32);
impl_cast_to!(u8, u64);
impl_cast_to!(u8, i8);
impl_cast_to!(u8, i16);
impl_cast_to!(u8, i32);
impl_cast_to!(u8, i64);

impl_cast_to!(u16, u8);
impl_cast_to!(u16);
impl_cast_to!(u16, u32);
impl_cast_to!(u16, u64);
impl_cast_to!(u16, i8);
impl_cast_to!(u16, i16);
impl_cast_to!(u16, i32);
impl_cast_to!(u16, i64);

impl_cast_to!(u32, u8);
impl_cast_to!(u32, u16);
impl_cast_to!(u32);
impl_cast_to!(u32, u64);
impl_cast_to!(u32, i8);
impl_cast_to!(u32, i16);
impl_cast_to!(u32, i32);
impl_cast_to!(u32, i64);

impl_cast_to!(u64, u8);
impl_cast_to!(u64, u16);
impl_cast_to!(u64, u32);
impl_cast_to!(u64);
impl_cast_to!(u64, i8);
impl_cast_to!(u64, i16);
impl_cast_to!(u64, i32);
impl_cast_to!(u64, i64);

impl_cast_to!(i8, u8);
impl_cast_to!(i8, u16);
impl_cast_to!(i8, u32);
impl_cast_to!(i8, u64);
impl_cast_to!(i8);
impl_cast_to!(i8, i16);
impl_cast_to!(i8, i32);
impl_cast_to!(i8, i64);

impl_cast_to!(i16, u8);
impl_cast_to!(i16, u16);
impl_cast_to!(i16, u32);
impl_cast_to!(i16, u64);
impl_cast_to!(i16, i8);
impl_cast_to!(i16);
impl_cast_to!(i16, i32);
impl_cast_to!(i16, i64);

impl_cast_to!(i32, u8);
impl_cast_to!(i32, u16);
impl_cast_to!(i32, u32);
impl_cast_to!(i32, u64);
impl_cast_to!(i32, i8);
impl_cast_to!(i32, i16);
impl_cast_to!(i32);
impl_cast_to!(i32, i64);

impl_cast_to!(i64, u8);
impl_cast_to!(i64, u16);
impl_cast_to!(i64, u32);
impl_cast_to!(i64, u64);
impl_cast_to!(i64, i8);
impl_cast_to!(i64, i16);
impl_cast_to!(i64, i32);
impl_cast_to!(i64);

impl_cast_to!(f64);
impl_cast_to!(f64, f32);

impl_cast_to!(f32);
impl_cast_to!(f32, f64);
