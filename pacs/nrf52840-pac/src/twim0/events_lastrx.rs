#[doc = "Register `EVENTS_LASTRX` reader"]
pub type R = crate::R<EventsLastrxSpec>;
#[doc = "Register `EVENTS_LASTRX` writer"]
pub type W = crate::W<EventsLastrxSpec>;
#[doc = "Byte boundary, starting to receive the last byte\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsLastrx {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsLastrx> for bool {
    #[inline(always)]
    fn from(variant: EventsLastrx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_LASTRX` reader - Byte boundary, starting to receive the last byte"]
pub type EventsLastrxR = crate::BitReader<EventsLastrx>;
impl EventsLastrxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsLastrx {
        match self.bits {
            false => EventsLastrx::NotGenerated,
            true => EventsLastrx::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsLastrx::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsLastrx::Generated
    }
}
#[doc = "Field `EVENTS_LASTRX` writer - Byte boundary, starting to receive the last byte"]
pub type EventsLastrxW<'a, REG> = crate::BitWriter<'a, REG, EventsLastrx>;
impl<'a, REG> EventsLastrxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsLastrx::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsLastrx::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Byte boundary, starting to receive the last byte"]
    #[inline(always)]
    pub fn events_lastrx(&self) -> EventsLastrxR {
        EventsLastrxR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Byte boundary, starting to receive the last byte"]
    #[inline(always)]
    pub fn events_lastrx(&mut self) -> EventsLastrxW<'_, EventsLastrxSpec> {
        EventsLastrxW::new(self, 0)
    }
}
#[doc = "Byte boundary, starting to receive the last byte\n\nYou can [`read`](crate::Reg::read) this register and get [`events_lastrx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_lastrx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsLastrxSpec;
impl crate::RegisterSpec for EventsLastrxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_lastrx::R`](R) reader structure"]
impl crate::Readable for EventsLastrxSpec {}
#[doc = "`write(|w| ..)` method takes [`events_lastrx::W`](W) writer structure"]
impl crate::Writable for EventsLastrxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_LASTRX to value 0"]
impl crate::Resettable for EventsLastrxSpec {}
