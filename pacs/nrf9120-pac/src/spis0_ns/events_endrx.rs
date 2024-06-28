#[doc = "Register `EVENTS_ENDRX` reader"]
pub type R = crate::R<EventsEndrxSpec>;
#[doc = "Register `EVENTS_ENDRX` writer"]
pub type W = crate::W<EventsEndrxSpec>;
#[doc = "End of RXD buffer reached\n\nValue on reset: 0"]
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
#[doc = "Field `EVENTS_ENDRX` reader - End of RXD buffer reached"]
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
#[doc = "Field `EVENTS_ENDRX` writer - End of RXD buffer reached"]
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
    #[doc = "Bit 0 - End of RXD buffer reached"]
    #[inline(always)]
    pub fn events_endrx(&self) -> EventsEndrxR {
        EventsEndrxR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of RXD buffer reached"]
    #[inline(always)]
    #[must_use]
    pub fn events_endrx(&mut self) -> EventsEndrxW<EventsEndrxSpec> {
        EventsEndrxW::new(self, 0)
    }
}
#[doc = "End of RXD buffer reached\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_endrx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_endrx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsEndrxSpec;
impl crate::RegisterSpec for EventsEndrxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_endrx::R`](R) reader structure"]
impl crate::Readable for EventsEndrxSpec {}
#[doc = "`write(|w| ..)` method takes [`events_endrx::W`](W) writer structure"]
impl crate::Writable for EventsEndrxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_ENDRX to value 0"]
impl crate::Resettable for EventsEndrxSpec {
    const RESET_VALUE: u32 = 0;
}
