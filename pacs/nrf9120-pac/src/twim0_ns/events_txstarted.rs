#[doc = "Register `EVENTS_TXSTARTED` reader"]
pub type R = crate::R<EventsTxstartedSpec>;
#[doc = "Register `EVENTS_TXSTARTED` writer"]
pub type W = crate::W<EventsTxstartedSpec>;
#[doc = "Transmit sequence started\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsTxstarted {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsTxstarted> for bool {
    #[inline(always)]
    fn from(variant: EventsTxstarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_TXSTARTED` reader - Transmit sequence started"]
pub type EventsTxstartedR = crate::BitReader<EventsTxstarted>;
impl EventsTxstartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsTxstarted {
        match self.bits {
            false => EventsTxstarted::NotGenerated,
            true => EventsTxstarted::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsTxstarted::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsTxstarted::Generated
    }
}
#[doc = "Field `EVENTS_TXSTARTED` writer - Transmit sequence started"]
pub type EventsTxstartedW<'a, REG> = crate::BitWriter<'a, REG, EventsTxstarted>;
impl<'a, REG> EventsTxstartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsTxstarted::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsTxstarted::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Transmit sequence started"]
    #[inline(always)]
    pub fn events_txstarted(&self) -> EventsTxstartedR {
        EventsTxstartedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit sequence started"]
    #[inline(always)]
    #[must_use]
    pub fn events_txstarted(&mut self) -> EventsTxstartedW<EventsTxstartedSpec> {
        EventsTxstartedW::new(self, 0)
    }
}
#[doc = "Transmit sequence started\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_txstarted::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_txstarted::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsTxstartedSpec;
impl crate::RegisterSpec for EventsTxstartedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_txstarted::R`](R) reader structure"]
impl crate::Readable for EventsTxstartedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_txstarted::W`](W) writer structure"]
impl crate::Writable for EventsTxstartedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_TXSTARTED to value 0"]
impl crate::Resettable for EventsTxstartedSpec {
    const RESET_VALUE: u32 = 0;
}
