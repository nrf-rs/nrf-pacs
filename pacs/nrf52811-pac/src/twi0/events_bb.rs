#[doc = "Register `EVENTS_BB` reader"]
pub type R = crate::R<EventsBbSpec>;
#[doc = "Register `EVENTS_BB` writer"]
pub type W = crate::W<EventsBbSpec>;
#[doc = "TWI byte boundary, generated before each byte that is sent or received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsBb {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsBb> for bool {
    #[inline(always)]
    fn from(variant: EventsBb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_BB` reader - TWI byte boundary, generated before each byte that is sent or received"]
pub type EventsBbR = crate::BitReader<EventsBb>;
impl EventsBbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsBb {
        match self.bits {
            false => EventsBb::NotGenerated,
            true => EventsBb::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsBb::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsBb::Generated
    }
}
#[doc = "Field `EVENTS_BB` writer - TWI byte boundary, generated before each byte that is sent or received"]
pub type EventsBbW<'a, REG> = crate::BitWriter<'a, REG, EventsBb>;
impl<'a, REG> EventsBbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsBb::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsBb::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - TWI byte boundary, generated before each byte that is sent or received"]
    #[inline(always)]
    pub fn events_bb(&self) -> EventsBbR {
        EventsBbR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TWI byte boundary, generated before each byte that is sent or received"]
    #[inline(always)]
    pub fn events_bb(&mut self) -> EventsBbW<'_, EventsBbSpec> {
        EventsBbW::new(self, 0)
    }
}
#[doc = "TWI byte boundary, generated before each byte that is sent or received\n\nYou can [`read`](crate::Reg::read) this register and get [`events_bb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_bb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsBbSpec;
impl crate::RegisterSpec for EventsBbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_bb::R`](R) reader structure"]
impl crate::Readable for EventsBbSpec {}
#[doc = "`write(|w| ..)` method takes [`events_bb::W`](W) writer structure"]
impl crate::Writable for EventsBbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_BB to value 0"]
impl crate::Resettable for EventsBbSpec {}
