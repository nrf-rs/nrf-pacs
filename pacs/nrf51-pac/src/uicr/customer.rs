#[doc = "Register `CUSTOMER[%s]` reader"]
pub type R = crate::R<CustomerSpec>;
#[doc = "Register `CUSTOMER[%s]` writer"]
pub type W = crate::W<CustomerSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Reserved for customer.\n\nYou can [`read`](crate::Reg::read) this register and get [`customer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`customer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CustomerSpec;
impl crate::RegisterSpec for CustomerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`customer::R`](R) reader structure"]
impl crate::Readable for CustomerSpec {}
#[doc = "`write(|w| ..)` method takes [`customer::W`](W) writer structure"]
impl crate::Writable for CustomerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CUSTOMER[%s] to value 0xffff_ffff"]
impl crate::Resettable for CustomerSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
