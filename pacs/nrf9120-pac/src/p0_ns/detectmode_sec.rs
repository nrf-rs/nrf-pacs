#[doc = "Register `DETECTMODE_SEC` reader"]
pub type R = crate::R<DetectmodeSecSpec>;
#[doc = "Register `DETECTMODE_SEC` writer"]
pub type W = crate::W<DetectmodeSecSpec>;
#[doc = "Select between default DETECT signal behavior and LDETECT mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Detectmode {
    #[doc = "0: DETECT directly connected to PIN DETECT signals"]
    Default = 0,
    #[doc = "1: Use the latched LDETECT behavior"]
    Ldetect = 1,
}
impl From<Detectmode> for bool {
    #[inline(always)]
    fn from(variant: Detectmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DETECTMODE` reader - Select between default DETECT signal behavior and LDETECT mode"]
pub type DetectmodeR = crate::BitReader<Detectmode>;
impl DetectmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Detectmode {
        match self.bits {
            false => Detectmode::Default,
            true => Detectmode::Ldetect,
        }
    }
    #[doc = "DETECT directly connected to PIN DETECT signals"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Detectmode::Default
    }
    #[doc = "Use the latched LDETECT behavior"]
    #[inline(always)]
    pub fn is_ldetect(&self) -> bool {
        *self == Detectmode::Ldetect
    }
}
#[doc = "Field `DETECTMODE` writer - Select between default DETECT signal behavior and LDETECT mode"]
pub type DetectmodeW<'a, REG> = crate::BitWriter<'a, REG, Detectmode>;
impl<'a, REG> DetectmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DETECT directly connected to PIN DETECT signals"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Detectmode::Default)
    }
    #[doc = "Use the latched LDETECT behavior"]
    #[inline(always)]
    pub fn ldetect(self) -> &'a mut crate::W<REG> {
        self.variant(Detectmode::Ldetect)
    }
}
impl R {
    #[doc = "Bit 0 - Select between default DETECT signal behavior and LDETECT mode"]
    #[inline(always)]
    pub fn detectmode(&self) -> DetectmodeR {
        DetectmodeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select between default DETECT signal behavior and LDETECT mode"]
    #[inline(always)]
    #[must_use]
    pub fn detectmode(&mut self) -> DetectmodeW<DetectmodeSecSpec> {
        DetectmodeW::new(self, 0)
    }
}
#[doc = "Select between default DETECT signal behavior and LDETECT mode (For secure pin only)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`detectmode_sec::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`detectmode_sec::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DetectmodeSecSpec;
impl crate::RegisterSpec for DetectmodeSecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`detectmode_sec::R`](R) reader structure"]
impl crate::Readable for DetectmodeSecSpec {}
#[doc = "`write(|w| ..)` method takes [`detectmode_sec::W`](W) writer structure"]
impl crate::Writable for DetectmodeSecSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DETECTMODE_SEC to value 0"]
impl crate::Resettable for DetectmodeSecSpec {
    const RESET_VALUE: u32 = 0;
}
