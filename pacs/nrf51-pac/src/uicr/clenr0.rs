#[doc = "Register `CLENR0` reader"]
pub type R = crate::R<Clenr0Spec>;
#[doc = "Register `CLENR0` writer"]
pub type W = crate::W<Clenr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Length of code region 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`clenr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clenr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clenr0Spec;
impl crate::RegisterSpec for Clenr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clenr0::R`](R) reader structure"]
impl crate::Readable for Clenr0Spec {}
#[doc = "`write(|w| ..)` method takes [`clenr0::W`](W) writer structure"]
impl crate::Writable for Clenr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLENR0 to value 0xffff_ffff"]
impl crate::Resettable for Clenr0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
