#[doc = "Register `EVENTS_TXREADY` reader"]
pub type R = crate::R<EventsTxreadySpec>;
#[doc = "Register `EVENTS_TXREADY` writer"]
pub type W = crate::W<EventsTxreadySpec>;
#[doc = "RADIO has ramped up and is ready to be started TX path\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsTxready {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsTxready> for bool {
    #[inline(always)]
    fn from(variant: EventsTxready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_TXREADY` reader - RADIO has ramped up and is ready to be started TX path"]
pub type EventsTxreadyR = crate::BitReader<EventsTxready>;
impl EventsTxreadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsTxready {
        match self.bits {
            false => EventsTxready::NotGenerated,
            true => EventsTxready::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsTxready::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsTxready::Generated
    }
}
#[doc = "Field `EVENTS_TXREADY` writer - RADIO has ramped up and is ready to be started TX path"]
pub type EventsTxreadyW<'a, REG> = crate::BitWriter<'a, REG, EventsTxready>;
impl<'a, REG> EventsTxreadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsTxready::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsTxready::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - RADIO has ramped up and is ready to be started TX path"]
    #[inline(always)]
    pub fn events_txready(&self) -> EventsTxreadyR {
        EventsTxreadyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RADIO has ramped up and is ready to be started TX path"]
    #[inline(always)]
    pub fn events_txready(&mut self) -> EventsTxreadyW<'_, EventsTxreadySpec> {
        EventsTxreadyW::new(self, 0)
    }
}
#[doc = "RADIO has ramped up and is ready to be started TX path\n\nYou can [`read`](crate::Reg::read) this register and get [`events_txready::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_txready::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsTxreadySpec;
impl crate::RegisterSpec for EventsTxreadySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_txready::R`](R) reader structure"]
impl crate::Readable for EventsTxreadySpec {}
#[doc = "`write(|w| ..)` method takes [`events_txready::W`](W) writer structure"]
impl crate::Writable for EventsTxreadySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_TXREADY to value 0"]
impl crate::Resettable for EventsTxreadySpec {}
