#[doc = "Register `ENABLE` reader"]
pub type R = crate::R<EnableSpec>;
#[doc = "Register `ENABLE` writer"]
pub type W = crate::W<EnableSpec>;
#[doc = "Enable or disable UARTE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Enable {
    #[doc = "0: Disable UARTE"]
    Disabled = 0,
    #[doc = "8: Enable UARTE"]
    Enabled = 8,
}
impl From<Enable> for u8 {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Enable {
    type Ux = u8;
}
impl crate::IsEnum for Enable {}
#[doc = "Field `ENABLE` reader - Enable or disable UARTE"]
pub type EnableR = crate::FieldReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Enable> {
        match self.bits {
            0 => Some(Enable::Disabled),
            8 => Some(Enable::Enabled),
            _ => None,
        }
    }
    #[doc = "Disable UARTE"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enable::Disabled
    }
    #[doc = "Enable UARTE"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enable::Enabled
    }
}
#[doc = "Field `ENABLE` writer - Enable or disable UARTE"]
pub type EnableW<'a, REG> = crate::FieldWriter<'a, REG, 4, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable UARTE"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disabled)
    }
    #[doc = "Enable UARTE"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:3 - Enable or disable UARTE"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Enable or disable UARTE"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, EnableSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "Enable UART\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnableSpec;
impl crate::RegisterSpec for EnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable::R`](R) reader structure"]
impl crate::Readable for EnableSpec {}
#[doc = "`write(|w| ..)` method takes [`enable::W`](W) writer structure"]
impl crate::Writable for EnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENABLE to value 0"]
impl crate::Resettable for EnableSpec {}
