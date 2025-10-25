#[doc = "Register `EVENTS_BB` reader"]
pub type R = crate::R<EventsBbSpec>;
#[doc = "Register `EVENTS_BB` writer"]
pub type W = crate::W<EventsBbSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "TWI byte boundary, generated before each byte that is sent or received\n\nYou can [`read`](crate::Reg::read) this register and get [`events_bb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_bb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsBbSpec;
impl crate::RegisterSpec for EventsBbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_bb::R`](R) reader structure"]
impl crate::Readable for EventsBbSpec {}
#[doc = "`write(|w| ..)` method takes [`events_bb::W`](W) writer structure"]
impl crate::Writable for EventsBbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_BB to value 0"]
impl crate::Resettable for EventsBbSpec {}
