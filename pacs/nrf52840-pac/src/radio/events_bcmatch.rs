#[doc = "Register `EVENTS_BCMATCH` reader"]
pub type R = crate::R<EventsBcmatchSpec>;
#[doc = "Register `EVENTS_BCMATCH` writer"]
pub type W = crate::W<EventsBcmatchSpec>;
#[doc = "Bit counter reached bit count value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsBcmatch {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsBcmatch> for bool {
    #[inline(always)]
    fn from(variant: EventsBcmatch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_BCMATCH` reader - Bit counter reached bit count value"]
pub type EventsBcmatchR = crate::BitReader<EventsBcmatch>;
impl EventsBcmatchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsBcmatch {
        match self.bits {
            false => EventsBcmatch::NotGenerated,
            true => EventsBcmatch::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsBcmatch::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsBcmatch::Generated
    }
}
#[doc = "Field `EVENTS_BCMATCH` writer - Bit counter reached bit count value"]
pub type EventsBcmatchW<'a, REG> = crate::BitWriter<'a, REG, EventsBcmatch>;
impl<'a, REG> EventsBcmatchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsBcmatch::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsBcmatch::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Bit counter reached bit count value"]
    #[inline(always)]
    pub fn events_bcmatch(&self) -> EventsBcmatchR {
        EventsBcmatchR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit counter reached bit count value"]
    #[inline(always)]
    pub fn events_bcmatch(&mut self) -> EventsBcmatchW<'_, EventsBcmatchSpec> {
        EventsBcmatchW::new(self, 0)
    }
}
#[doc = "Bit counter reached bit count value\n\nYou can [`read`](crate::Reg::read) this register and get [`events_bcmatch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_bcmatch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsBcmatchSpec;
impl crate::RegisterSpec for EventsBcmatchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_bcmatch::R`](R) reader structure"]
impl crate::Readable for EventsBcmatchSpec {}
#[doc = "`write(|w| ..)` method takes [`events_bcmatch::W`](W) writer structure"]
impl crate::Writable for EventsBcmatchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_BCMATCH to value 0"]
impl crate::Resettable for EventsBcmatchSpec {}
