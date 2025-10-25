#[doc = "Register `EVENTS_ADDRESS` reader"]
pub type R = crate::R<EventsAddressSpec>;
#[doc = "Register `EVENTS_ADDRESS` writer"]
pub type W = crate::W<EventsAddressSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Address sent or received\n\nYou can [`read`](crate::Reg::read) this register and get [`events_address::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_address::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsAddressSpec;
impl crate::RegisterSpec for EventsAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_address::R`](R) reader structure"]
impl crate::Readable for EventsAddressSpec {}
#[doc = "`write(|w| ..)` method takes [`events_address::W`](W) writer structure"]
impl crate::Writable for EventsAddressSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_ADDRESS to value 0"]
impl crate::Resettable for EventsAddressSpec {}
