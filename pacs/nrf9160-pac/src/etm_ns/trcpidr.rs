#[doc = "Register `TRCPIDR[%s]` reader"]
pub type R = crate::R<TrcpidrSpec>;
#[doc = "Register `TRCPIDR[%s]` writer"]
pub type W = crate::W<TrcpidrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Description collection: Coresight peripheral identification registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcpidr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcpidr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcpidrSpec;
impl crate::RegisterSpec for TrcpidrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcpidr::R`](R) reader structure"]
impl crate::Readable for TrcpidrSpec {}
#[doc = "`write(|w| ..)` method takes [`trcpidr::W`](W) writer structure"]
impl crate::Writable for TrcpidrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCPIDR[%s] to value 0"]
impl crate::Resettable for TrcpidrSpec {}
