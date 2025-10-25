#[doc = "Register `EVENTS_RXPTRUPD` reader"]
pub type R = crate::R<EventsRxptrupdSpec>;
#[doc = "Register `EVENTS_RXPTRUPD` writer"]
pub type W = crate::W<EventsRxptrupdSpec>;
#[doc = "The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words received on the SDIN pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsRxptrupd {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsRxptrupd> for bool {
    #[inline(always)]
    fn from(variant: EventsRxptrupd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_RXPTRUPD` reader - The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words received on the SDIN pin."]
pub type EventsRxptrupdR = crate::BitReader<EventsRxptrupd>;
impl EventsRxptrupdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsRxptrupd {
        match self.bits {
            false => EventsRxptrupd::NotGenerated,
            true => EventsRxptrupd::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsRxptrupd::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsRxptrupd::Generated
    }
}
#[doc = "Field `EVENTS_RXPTRUPD` writer - The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words received on the SDIN pin."]
pub type EventsRxptrupdW<'a, REG> = crate::BitWriter<'a, REG, EventsRxptrupd>;
impl<'a, REG> EventsRxptrupdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRxptrupd::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRxptrupd::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words received on the SDIN pin."]
    #[inline(always)]
    pub fn events_rxptrupd(&self) -> EventsRxptrupdR {
        EventsRxptrupdR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words received on the SDIN pin."]
    #[inline(always)]
    pub fn events_rxptrupd(&mut self) -> EventsRxptrupdW<'_, EventsRxptrupdSpec> {
        EventsRxptrupdW::new(self, 0)
    }
}
#[doc = "The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words received on the SDIN pin.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rxptrupd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rxptrupd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsRxptrupdSpec;
impl crate::RegisterSpec for EventsRxptrupdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_rxptrupd::R`](R) reader structure"]
impl crate::Readable for EventsRxptrupdSpec {}
#[doc = "`write(|w| ..)` method takes [`events_rxptrupd::W`](W) writer structure"]
impl crate::Writable for EventsRxptrupdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_RXPTRUPD to value 0"]
impl crate::Resettable for EventsRxptrupdSpec {}
