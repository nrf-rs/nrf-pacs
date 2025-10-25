#[doc = "Register `EVENTS_RXFRAMEEND` reader"]
pub type R = crate::R<EventsRxframeendSpec>;
#[doc = "Register `EVENTS_RXFRAMEEND` writer"]
pub type W = crate::W<EventsRxframeendSpec>;
#[doc = "Received data has been checked (CRC, parity) and transferred to RAM, and EasyDMA has ended accessing the RX buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsRxframeend {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsRxframeend> for bool {
    #[inline(always)]
    fn from(variant: EventsRxframeend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_RXFRAMEEND` reader - Received data has been checked (CRC, parity) and transferred to RAM, and EasyDMA has ended accessing the RX buffer"]
pub type EventsRxframeendR = crate::BitReader<EventsRxframeend>;
impl EventsRxframeendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsRxframeend {
        match self.bits {
            false => EventsRxframeend::NotGenerated,
            true => EventsRxframeend::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsRxframeend::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsRxframeend::Generated
    }
}
#[doc = "Field `EVENTS_RXFRAMEEND` writer - Received data has been checked (CRC, parity) and transferred to RAM, and EasyDMA has ended accessing the RX buffer"]
pub type EventsRxframeendW<'a, REG> = crate::BitWriter<'a, REG, EventsRxframeend>;
impl<'a, REG> EventsRxframeendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRxframeend::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRxframeend::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Received data has been checked (CRC, parity) and transferred to RAM, and EasyDMA has ended accessing the RX buffer"]
    #[inline(always)]
    pub fn events_rxframeend(&self) -> EventsRxframeendR {
        EventsRxframeendR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Received data has been checked (CRC, parity) and transferred to RAM, and EasyDMA has ended accessing the RX buffer"]
    #[inline(always)]
    pub fn events_rxframeend(&mut self) -> EventsRxframeendW<'_, EventsRxframeendSpec> {
        EventsRxframeendW::new(self, 0)
    }
}
#[doc = "Received data has been checked (CRC, parity) and transferred to RAM, and EasyDMA has ended accessing the RX buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rxframeend::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rxframeend::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsRxframeendSpec;
impl crate::RegisterSpec for EventsRxframeendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_rxframeend::R`](R) reader structure"]
impl crate::Readable for EventsRxframeendSpec {}
#[doc = "`write(|w| ..)` method takes [`events_rxframeend::W`](W) writer structure"]
impl crate::Writable for EventsRxframeendSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_RXFRAMEEND to value 0"]
impl crate::Resettable for EventsRxframeendSpec {}
