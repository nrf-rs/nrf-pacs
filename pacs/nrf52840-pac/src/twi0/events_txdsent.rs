#[doc = "Register `EVENTS_TXDSENT` reader"]
pub type R = crate::R<EventsTxdsentSpec>;
#[doc = "Register `EVENTS_TXDSENT` writer"]
pub type W = crate::W<EventsTxdsentSpec>;
#[doc = "TWI TXD byte sent\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsTxdsent {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsTxdsent> for bool {
    #[inline(always)]
    fn from(variant: EventsTxdsent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_TXDSENT` reader - TWI TXD byte sent"]
pub type EventsTxdsentR = crate::BitReader<EventsTxdsent>;
impl EventsTxdsentR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsTxdsent {
        match self.bits {
            false => EventsTxdsent::NotGenerated,
            true => EventsTxdsent::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsTxdsent::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsTxdsent::Generated
    }
}
#[doc = "Field `EVENTS_TXDSENT` writer - TWI TXD byte sent"]
pub type EventsTxdsentW<'a, REG> = crate::BitWriter<'a, REG, EventsTxdsent>;
impl<'a, REG> EventsTxdsentW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsTxdsent::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsTxdsent::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - TWI TXD byte sent"]
    #[inline(always)]
    pub fn events_txdsent(&self) -> EventsTxdsentR {
        EventsTxdsentR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TWI TXD byte sent"]
    #[inline(always)]
    pub fn events_txdsent(&mut self) -> EventsTxdsentW<'_, EventsTxdsentSpec> {
        EventsTxdsentW::new(self, 0)
    }
}
#[doc = "TWI TXD byte sent\n\nYou can [`read`](crate::Reg::read) this register and get [`events_txdsent::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_txdsent::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsTxdsentSpec;
impl crate::RegisterSpec for EventsTxdsentSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_txdsent::R`](R) reader structure"]
impl crate::Readable for EventsTxdsentSpec {}
#[doc = "`write(|w| ..)` method takes [`events_txdsent::W`](W) writer structure"]
impl crate::Writable for EventsTxdsentSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_TXDSENT to value 0"]
impl crate::Resettable for EventsTxdsentSpec {}
