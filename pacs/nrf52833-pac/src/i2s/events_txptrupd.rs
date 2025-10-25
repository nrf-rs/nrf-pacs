#[doc = "Register `EVENTS_TXPTRUPD` reader"]
pub type R = crate::R<EventsTxptrupdSpec>;
#[doc = "Register `EVENTS_TXPTRUPD` writer"]
pub type W = crate::W<EventsTxptrupdSpec>;
#[doc = "The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsTxptrupd {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsTxptrupd> for bool {
    #[inline(always)]
    fn from(variant: EventsTxptrupd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_TXPTRUPD` reader - The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin."]
pub type EventsTxptrupdR = crate::BitReader<EventsTxptrupd>;
impl EventsTxptrupdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsTxptrupd {
        match self.bits {
            false => EventsTxptrupd::NotGenerated,
            true => EventsTxptrupd::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsTxptrupd::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsTxptrupd::Generated
    }
}
#[doc = "Field `EVENTS_TXPTRUPD` writer - The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin."]
pub type EventsTxptrupdW<'a, REG> = crate::BitWriter<'a, REG, EventsTxptrupd>;
impl<'a, REG> EventsTxptrupdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsTxptrupd::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsTxptrupd::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin."]
    #[inline(always)]
    pub fn events_txptrupd(&self) -> EventsTxptrupdR {
        EventsTxptrupdR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin."]
    #[inline(always)]
    pub fn events_txptrupd(&mut self) -> EventsTxptrupdW<'_, EventsTxptrupdSpec> {
        EventsTxptrupdW::new(self, 0)
    }
}
#[doc = "The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_txptrupd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_txptrupd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsTxptrupdSpec;
impl crate::RegisterSpec for EventsTxptrupdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_txptrupd::R`](R) reader structure"]
impl crate::Readable for EventsTxptrupdSpec {}
#[doc = "`write(|w| ..)` method takes [`events_txptrupd::W`](W) writer structure"]
impl crate::Writable for EventsTxptrupdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_TXPTRUPD to value 0"]
impl crate::Resettable for EventsTxptrupdSpec {}
