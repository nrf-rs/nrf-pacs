#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    acl: (),
}
impl RegisterBlock {
    #[doc = "0x800..0x860 - Unspecified"]
    #[inline(always)]
    pub const fn acl(&self, n: usize) -> &Acl {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2048)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x860 - Unspecified"]
    #[inline(always)]
    pub fn acl_iter(&self) -> impl Iterator<Item = &Acl> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2048)
                .add(16 * n)
                .cast()
        })
    }
}
#[doc = "Unspecified"]
pub use self::acl::Acl;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod acl;
