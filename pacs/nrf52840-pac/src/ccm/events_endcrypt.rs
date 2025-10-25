#[doc = "Register `EVENTS_ENDCRYPT` reader"]
pub type R = crate::R<EventsEndcryptSpec>;
#[doc = "Register `EVENTS_ENDCRYPT` writer"]
pub type W = crate::W<EventsEndcryptSpec>;
#[doc = "Encrypt/decrypt complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsEndcrypt {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsEndcrypt> for bool {
    #[inline(always)]
    fn from(variant: EventsEndcrypt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_ENDCRYPT` reader - Encrypt/decrypt complete"]
pub type EventsEndcryptR = crate::BitReader<EventsEndcrypt>;
impl EventsEndcryptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsEndcrypt {
        match self.bits {
            false => EventsEndcrypt::NotGenerated,
            true => EventsEndcrypt::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsEndcrypt::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsEndcrypt::Generated
    }
}
#[doc = "Field `EVENTS_ENDCRYPT` writer - Encrypt/decrypt complete"]
pub type EventsEndcryptW<'a, REG> = crate::BitWriter<'a, REG, EventsEndcrypt>;
impl<'a, REG> EventsEndcryptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEndcrypt::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEndcrypt::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Encrypt/decrypt complete"]
    #[inline(always)]
    pub fn events_endcrypt(&self) -> EventsEndcryptR {
        EventsEndcryptR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Encrypt/decrypt complete"]
    #[inline(always)]
    pub fn events_endcrypt(&mut self) -> EventsEndcryptW<'_, EventsEndcryptSpec> {
        EventsEndcryptW::new(self, 0)
    }
}
#[doc = "Encrypt/decrypt complete\n\nYou can [`read`](crate::Reg::read) this register and get [`events_endcrypt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_endcrypt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsEndcryptSpec;
impl crate::RegisterSpec for EventsEndcryptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_endcrypt::R`](R) reader structure"]
impl crate::Readable for EventsEndcryptSpec {}
#[doc = "`write(|w| ..)` method takes [`events_endcrypt::W`](W) writer structure"]
impl crate::Writable for EventsEndcryptSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_ENDCRYPT to value 0"]
impl crate::Resettable for EventsEndcryptSpec {}
