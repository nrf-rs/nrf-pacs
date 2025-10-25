#[doc = "Register `CRV` reader"]
pub type R = crate::R<CrvSpec>;
#[doc = "Register `CRV` writer"]
pub type W = crate::W<CrvSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Counter reload value in number of 32kiHz clock cycles.\n\nYou can [`read`](crate::Reg::read) this register and get [`crv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrvSpec;
impl crate::RegisterSpec for CrvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crv::R`](R) reader structure"]
impl crate::Readable for CrvSpec {}
#[doc = "`write(|w| ..)` method takes [`crv::W`](W) writer structure"]
impl crate::Writable for CrvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRV to value 0xffff_ffff"]
impl crate::Resettable for CrvSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
