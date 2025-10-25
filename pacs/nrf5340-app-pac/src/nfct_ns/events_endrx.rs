#[doc = "Register `EVENTS_ENDRX` reader"]
pub type R = crate::R<EventsEndrxSpec>;
#[doc = "Register `EVENTS_ENDRX` writer"]
pub type W = crate::W<EventsEndrxSpec>;
#[doc = "RX buffer (as defined by PACKETPTR and MAXLEN) in Data RAM full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsEndrx {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsEndrx> for bool {
    #[inline(always)]
    fn from(variant: EventsEndrx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_ENDRX` reader - RX buffer (as defined by PACKETPTR and MAXLEN) in Data RAM full."]
pub type EventsEndrxR = crate::BitReader<EventsEndrx>;
impl EventsEndrxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsEndrx {
        match self.bits {
            false => EventsEndrx::NotGenerated,
            true => EventsEndrx::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsEndrx::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsEndrx::Generated
    }
}
#[doc = "Field `EVENTS_ENDRX` writer - RX buffer (as defined by PACKETPTR and MAXLEN) in Data RAM full."]
pub type EventsEndrxW<'a, REG> = crate::BitWriter<'a, REG, EventsEndrx>;
impl<'a, REG> EventsEndrxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEndrx::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEndrx::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - RX buffer (as defined by PACKETPTR and MAXLEN) in Data RAM full."]
    #[inline(always)]
    pub fn events_endrx(&self) -> EventsEndrxR {
        EventsEndrxR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX buffer (as defined by PACKETPTR and MAXLEN) in Data RAM full."]
    #[inline(always)]
    pub fn events_endrx(&mut self) -> EventsEndrxW<'_, EventsEndrxSpec> {
        EventsEndrxW::new(self, 0)
    }
}
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
