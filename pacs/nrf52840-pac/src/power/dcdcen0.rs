#[doc = "Register `DCDCEN0` reader"]
pub type R = crate::R<Dcdcen0Spec>;
#[doc = "Register `DCDCEN0` writer"]
pub type W = crate::W<Dcdcen0Spec>;
#[doc = "Enable DC/DC converter for REG0 stage.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcdcen {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Dcdcen> for bool {
    #[inline(always)]
    fn from(variant: Dcdcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCDCEN` reader - Enable DC/DC converter for REG0 stage."]
pub type DcdcenR = crate::BitReader<Dcdcen>;
impl DcdcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcdcen {
        match self.bits {
            false => Dcdcen::Disabled,
            true => Dcdcen::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dcdcen::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dcdcen::Enabled
    }
}
#[doc = "Field `DCDCEN` writer - Enable DC/DC converter for REG0 stage."]
pub type DcdcenW<'a, REG> = crate::BitWriter<'a, REG, Dcdcen>;
impl<'a, REG> DcdcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dcdcen::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dcdcen::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable DC/DC converter for REG0 stage."]
    #[inline(always)]
    pub fn dcdcen(&self) -> DcdcenR {
        DcdcenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable DC/DC converter for REG0 stage."]
    #[inline(always)]
    pub fn dcdcen(&mut self) -> DcdcenW<'_, Dcdcen0Spec> {
        DcdcenW::new(self, 0)
    }
}
#[doc = "Enable DC/DC converter for REG0 stage\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdcen0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdcen0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dcdcen0Spec;
impl crate::RegisterSpec for Dcdcen0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdcen0::R`](R) reader structure"]
impl crate::Readable for Dcdcen0Spec {}
#[doc = "`write(|w| ..)` method takes [`dcdcen0::W`](W) writer structure"]
impl crate::Writable for Dcdcen0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCDCEN0 to value 0"]
impl crate::Resettable for Dcdcen0Spec {}
