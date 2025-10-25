#[doc = "Register `MCKEN` reader"]
pub type R = crate::R<MckenSpec>;
#[doc = "Register `MCKEN` writer"]
pub type W = crate::W<MckenSpec>;
#[doc = "Master clock generator enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mcken {
    #[doc = "0: Master clock generator disabled and PSEL.MCK not connected(available as GPIO)."]
    Disabled = 0,
    #[doc = "1: Master clock generator running and MCK output on PSEL.MCK."]
    Enabled = 1,
}
impl From<Mcken> for bool {
    #[inline(always)]
    fn from(variant: Mcken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCKEN` reader - Master clock generator enable"]
pub type MckenR = crate::BitReader<Mcken>;
impl MckenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mcken {
        match self.bits {
            false => Mcken::Disabled,
            true => Mcken::Enabled,
        }
    }
    #[doc = "Master clock generator disabled and PSEL.MCK not connected(available as GPIO)."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mcken::Disabled
    }
    #[doc = "Master clock generator running and MCK output on PSEL.MCK."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mcken::Enabled
    }
}
#[doc = "Field `MCKEN` writer - Master clock generator enable"]
pub type MckenW<'a, REG> = crate::BitWriter<'a, REG, Mcken>;
impl<'a, REG> MckenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master clock generator disabled and PSEL.MCK not connected(available as GPIO)."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mcken::Disabled)
    }
    #[doc = "Master clock generator running and MCK output on PSEL.MCK."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mcken::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Master clock generator enable"]
    #[inline(always)]
    pub fn mcken(&self) -> MckenR {
        MckenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master clock generator enable"]
    #[inline(always)]
    pub fn mcken(&mut self) -> MckenW<'_, MckenSpec> {
        MckenW::new(self, 0)
    }
}
#[doc = "Master clock generator enable\n\nYou can [`read`](crate::Reg::read) this register and get [`mcken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MckenSpec;
impl crate::RegisterSpec for MckenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcken::R`](R) reader structure"]
impl crate::Readable for MckenSpec {}
#[doc = "`write(|w| ..)` method takes [`mcken::W`](W) writer structure"]
impl crate::Writable for MckenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCKEN to value 0x01"]
impl crate::Resettable for MckenSpec {
    const RESET_VALUE: u32 = 0x01;
}
