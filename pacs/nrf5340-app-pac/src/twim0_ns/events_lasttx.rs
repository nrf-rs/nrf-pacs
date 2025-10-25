#[doc = "Register `EVENTS_LASTTX` reader"]
pub type R = crate::R<EventsLasttxSpec>;
#[doc = "Register `EVENTS_LASTTX` writer"]
pub type W = crate::W<EventsLasttxSpec>;
#[doc = "Byte boundary, starting to transmit the last byte\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsLasttx {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsLasttx> for bool {
    #[inline(always)]
    fn from(variant: EventsLasttx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_LASTTX` reader - Byte boundary, starting to transmit the last byte"]
pub type EventsLasttxR = crate::BitReader<EventsLasttx>;
impl EventsLasttxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsLasttx {
        match self.bits {
            false => EventsLasttx::NotGenerated,
            true => EventsLasttx::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsLasttx::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsLasttx::Generated
    }
}
#[doc = "Field `EVENTS_LASTTX` writer - Byte boundary, starting to transmit the last byte"]
pub type EventsLasttxW<'a, REG> = crate::BitWriter<'a, REG, EventsLasttx>;
impl<'a, REG> EventsLasttxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsLasttx::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsLasttx::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Byte boundary, starting to transmit the last byte"]
    #[inline(always)]
    pub fn events_lasttx(&self) -> EventsLasttxR {
        EventsLasttxR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Byte boundary, starting to transmit the last byte"]
    #[inline(always)]
    pub fn events_lasttx(&mut self) -> EventsLasttxW<'_, EventsLasttxSpec> {
        EventsLasttxW::new(self, 0)
    }
}
#[doc = "Byte boundary, starting to transmit the last byte\n\nYou can [`read`](crate::Reg::read) this register and get [`events_lasttx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_lasttx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsLasttxSpec;
impl crate::RegisterSpec for EventsLasttxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_lasttx::R`](R) reader structure"]
impl crate::Readable for EventsLasttxSpec {}
#[doc = "`write(|w| ..)` method takes [`events_lasttx::W`](W) writer structure"]
impl crate::Writable for EventsLasttxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_LASTTX to value 0"]
impl crate::Resettable for EventsLasttxSpec {}
