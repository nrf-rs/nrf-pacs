#[doc = "Register `EVENTS_TXSTOPPED` reader"]
pub type R = crate::R<EventsTxstoppedSpec>;
#[doc = "Register `EVENTS_TXSTOPPED` writer"]
pub type W = crate::W<EventsTxstoppedSpec>;
#[doc = "Transmitter stopped\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsTxstopped {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsTxstopped> for bool {
    #[inline(always)]
    fn from(variant: EventsTxstopped) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_TXSTOPPED` reader - Transmitter stopped"]
pub type EventsTxstoppedR = crate::BitReader<EventsTxstopped>;
impl EventsTxstoppedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsTxstopped {
        match self.bits {
            false => EventsTxstopped::NotGenerated,
            true => EventsTxstopped::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsTxstopped::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsTxstopped::Generated
    }
}
#[doc = "Field `EVENTS_TXSTOPPED` writer - Transmitter stopped"]
pub type EventsTxstoppedW<'a, REG> = crate::BitWriter<'a, REG, EventsTxstopped>;
impl<'a, REG> EventsTxstoppedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsTxstopped::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsTxstopped::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Transmitter stopped"]
    #[inline(always)]
    pub fn events_txstopped(&self) -> EventsTxstoppedR {
        EventsTxstoppedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmitter stopped"]
    #[inline(always)]
    pub fn events_txstopped(&mut self) -> EventsTxstoppedW<'_, EventsTxstoppedSpec> {
        EventsTxstoppedW::new(self, 0)
    }
}
#[doc = "Transmitter stopped\n\nYou can [`read`](crate::Reg::read) this register and get [`events_txstopped::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_txstopped::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsTxstoppedSpec;
impl crate::RegisterSpec for EventsTxstoppedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_txstopped::R`](R) reader structure"]
impl crate::Readable for EventsTxstoppedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_txstopped::W`](W) writer structure"]
impl crate::Writable for EventsTxstoppedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_TXSTOPPED to value 0"]
impl crate::Resettable for EventsTxstoppedSpec {}
