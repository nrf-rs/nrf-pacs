#[doc = "Register `DIS` writer"]
pub type W = crate::W<DisSpec>;
impl core::fmt::Debug for crate::generic::Reg<DisSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Disable channel group.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dis::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DisSpec;
impl crate::RegisterSpec for DisSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dis::W`](W) writer structure"]
impl crate::Writable for DisSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIS to value 0"]
impl crate::Resettable for DisSpec {}
