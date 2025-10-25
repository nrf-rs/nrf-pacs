#[doc = "Register `MODE` reader"]
pub type R = crate::R<ModeSpec>;
#[doc = "Register `MODE` writer"]
pub type W = crate::W<ModeSpec>;
#[doc = "Selects up mode or up-and-down mode for the counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Updown {
    #[doc = "0: Up counter, edge-aligned PWM duty cycle"]
    Up = 0,
    #[doc = "1: Up and down counter, center-aligned PWM duty cycle"]
    UpAndDown = 1,
}
impl From<Updown> for bool {
    #[inline(always)]
    fn from(variant: Updown) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPDOWN` reader - Selects up mode or up-and-down mode for the counter"]
pub type UpdownR = crate::BitReader<Updown>;
impl UpdownR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Updown {
        match self.bits {
            false => Updown::Up,
            true => Updown::UpAndDown,
        }
    }
    #[doc = "Up counter, edge-aligned PWM duty cycle"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Updown::Up
    }
    #[doc = "Up and down counter, center-aligned PWM duty cycle"]
    #[inline(always)]
    pub fn is_up_and_down(&self) -> bool {
        *self == Updown::UpAndDown
    }
}
#[doc = "Field `UPDOWN` writer - Selects up mode or up-and-down mode for the counter"]
pub type UpdownW<'a, REG> = crate::BitWriter<'a, REG, Updown>;
impl<'a, REG> UpdownW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Up counter, edge-aligned PWM duty cycle"]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Updown::Up)
    }
    #[doc = "Up and down counter, center-aligned PWM duty cycle"]
    #[inline(always)]
    pub fn up_and_down(self) -> &'a mut crate::W<REG> {
        self.variant(Updown::UpAndDown)
    }
}
impl R {
    #[doc = "Bit 0 - Selects up mode or up-and-down mode for the counter"]
    #[inline(always)]
    pub fn updown(&self) -> UpdownR {
        UpdownR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects up mode or up-and-down mode for the counter"]
    #[inline(always)]
    pub fn updown(&mut self) -> UpdownW<'_, ModeSpec> {
        UpdownW::new(self, 0)
    }
}
#[doc = "Selects operating mode of the wave counter\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeSpec;
impl crate::RegisterSpec for ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for ModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for ModeSpec {}
