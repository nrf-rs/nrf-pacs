#[doc = "Register `DETECTMODE` reader"]
pub type R = crate::R<DetectmodeSpec>;
#[doc = "Register `DETECTMODE` writer"]
pub type W = crate::W<DetectmodeSpec>;
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
    pub fn detectmode(&mut self) -> DetectmodeW<'_, DetectmodeSpec> {
        DetectmodeW::new(self, 0)
    }
}
#[doc = "Select between default DETECT signal behavior and LDETECT mode\n\nYou can [`read`](crate::Reg::read) this register and get [`detectmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`detectmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DetectmodeSpec;
impl crate::RegisterSpec for DetectmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`detectmode::R`](R) reader structure"]
impl crate::Readable for DetectmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`detectmode::W`](W) writer structure"]
impl crate::Writable for DetectmodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DETECTMODE to value 0"]
impl crate::Resettable for DetectmodeSpec {}
