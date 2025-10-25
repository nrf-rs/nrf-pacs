#[doc = "Register `EVENTS_POFWARN` reader"]
pub type R = crate::R<EventsPofwarnSpec>;
#[doc = "Register `EVENTS_POFWARN` writer"]
pub type W = crate::W<EventsPofwarnSpec>;
#[doc = "Power failure warning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsPofwarn {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsPofwarn> for bool {
    #[inline(always)]
    fn from(variant: EventsPofwarn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_POFWARN` reader - Power failure warning"]
pub type EventsPofwarnR = crate::BitReader<EventsPofwarn>;
impl EventsPofwarnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsPofwarn {
        match self.bits {
            false => EventsPofwarn::NotGenerated,
            true => EventsPofwarn::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsPofwarn::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsPofwarn::Generated
    }
}
#[doc = "Field `EVENTS_POFWARN` writer - Power failure warning"]
pub type EventsPofwarnW<'a, REG> = crate::BitWriter<'a, REG, EventsPofwarn>;
impl<'a, REG> EventsPofwarnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsPofwarn::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsPofwarn::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Power failure warning"]
    #[inline(always)]
    pub fn events_pofwarn(&self) -> EventsPofwarnR {
        EventsPofwarnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power failure warning"]
    #[inline(always)]
    pub fn events_pofwarn(&mut self) -> EventsPofwarnW<'_, EventsPofwarnSpec> {
        EventsPofwarnW::new(self, 0)
    }
}
#[doc = "Power failure warning\n\nYou can [`read`](crate::Reg::read) this register and get [`events_pofwarn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_pofwarn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsPofwarnSpec;
impl crate::RegisterSpec for EventsPofwarnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_pofwarn::R`](R) reader structure"]
impl crate::Readable for EventsPofwarnSpec {}
#[doc = "`write(|w| ..)` method takes [`events_pofwarn::W`](W) writer structure"]
impl crate::Writable for EventsPofwarnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_POFWARN to value 0"]
impl crate::Resettable for EventsPofwarnSpec {}
