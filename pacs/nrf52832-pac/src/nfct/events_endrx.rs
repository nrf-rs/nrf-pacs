#[doc = "Register `EVENTS_ENDRX` reader"]
pub type R = crate::R<EventsEndrxSpec>;
#[doc = "Register `EVENTS_ENDRX` writer"]
pub type W = crate::W<EventsEndrxSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "RX buffer (as defined by PACKETPTR and MAXLEN) in Data RAM full.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_endrx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_endrx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsEndrxSpec;
impl crate::RegisterSpec for EventsEndrxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_endrx::R`](R) reader structure"]
impl crate::Readable for EventsEndrxSpec {}
#[doc = "`write(|w| ..)` method takes [`events_endrx::W`](W) writer structure"]
impl crate::Writable for EventsEndrxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_ENDRX to value 0"]
impl crate::Resettable for EventsEndrxSpec {}
