#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0600],
    ram: (),
}
impl RegisterBlock {
    #[doc = "0x600..0x630 - Unspecified"]
    #[inline(always)]
    pub const fn ram(&self, n: usize) -> &Ram {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1536)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x600..0x630 - Unspecified"]
    #[inline(always)]
    pub fn ram_iter(&self) -> impl Iterator<Item = &Ram> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1536)
                .add(16 * n)
                .cast()
        })
    }
}
#[doc = "Unspecified"]
pub use self::ram::Ram;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod ram;
