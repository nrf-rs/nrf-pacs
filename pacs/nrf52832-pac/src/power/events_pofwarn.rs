#[doc = "Register `EVENTS_POFWARN` reader"]
pub type R = crate::R<EventsPofwarnSpec>;
#[doc = "Register `EVENTS_POFWARN` writer"]
pub type W = crate::W<EventsPofwarnSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Power failure warning\n\nYou can [`read`](crate::Reg::read) this register and get [`events_pofwarn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_pofwarn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsPofwarnSpec;
impl crate::RegisterSpec for EventsPofwarnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_pofwarn::R`](R) reader structure"]
impl crate::Readable for EventsPofwarnSpec {}
#[doc = "`write(|w| ..)` method takes [`events_pofwarn::W`](W) writer structure"]
impl crate::Writable for EventsPofwarnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_POFWARN to value 0"]
impl crate::Resettable for EventsPofwarnSpec {}
