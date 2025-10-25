#[doc = "Register `EVENTS_RXERROR` reader"]
pub type R = crate::R<EventsRxerrorSpec>;
#[doc = "Register `EVENTS_RXERROR` writer"]
pub type W = crate::W<EventsRxerrorSpec>;
#[doc = "NFC RX frame error reported. The FRAMESTATUS.RX register contains details on the source of the error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsRxerror {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsRxerror> for bool {
    #[inline(always)]
    fn from(variant: EventsRxerror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_RXERROR` reader - NFC RX frame error reported. The FRAMESTATUS.RX register contains details on the source of the error."]
pub type EventsRxerrorR = crate::BitReader<EventsRxerror>;
impl EventsRxerrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsRxerror {
        match self.bits {
            false => EventsRxerror::NotGenerated,
            true => EventsRxerror::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsRxerror::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsRxerror::Generated
    }
}
#[doc = "Field `EVENTS_RXERROR` writer - NFC RX frame error reported. The FRAMESTATUS.RX register contains details on the source of the error."]
pub type EventsRxerrorW<'a, REG> = crate::BitWriter<'a, REG, EventsRxerror>;
impl<'a, REG> EventsRxerrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRxerror::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRxerror::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - NFC RX frame error reported. The FRAMESTATUS.RX register contains details on the source of the error."]
    #[inline(always)]
    pub fn events_rxerror(&self) -> EventsRxerrorR {
        EventsRxerrorR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NFC RX frame error reported. The FRAMESTATUS.RX register contains details on the source of the error."]
    #[inline(always)]
    pub fn events_rxerror(&mut self) -> EventsRxerrorW<'_, EventsRxerrorSpec> {
        EventsRxerrorW::new(self, 0)
    }
}
#[doc = "NFC RX frame error reported. The FRAMESTATUS.RX register contains details on the source of the error.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rxerror::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rxerror::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsRxerrorSpec;
impl crate::RegisterSpec for EventsRxerrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_rxerror::R`](R) reader structure"]
impl crate::Readable for EventsRxerrorSpec {}
#[doc = "`write(|w| ..)` method takes [`events_rxerror::W`](W) writer structure"]
impl crate::Writable for EventsRxerrorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_RXERROR to value 0"]
impl crate::Resettable for EventsRxerrorSpec {}
