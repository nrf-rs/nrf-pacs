#[doc = "Register `TRCCIDR[%s]` reader"]
pub type R = crate::R<TrccidrSpec>;
#[doc = "Register `TRCCIDR[%s]` writer"]
pub type W = crate::W<TrccidrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Description collection: Coresight component identification registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`trccidr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trccidr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrccidrSpec;
impl crate::RegisterSpec for TrccidrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trccidr::R`](R) reader structure"]
impl crate::Readable for TrccidrSpec {}
#[doc = "`write(|w| ..)` method takes [`trccidr::W`](W) writer structure"]
impl crate::Writable for TrccidrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCCIDR[%s] to value 0"]
impl crate::Resettable for TrccidrSpec {}
