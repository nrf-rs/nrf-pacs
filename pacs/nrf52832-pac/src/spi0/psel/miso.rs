#[doc = "Register `MISO` reader"]
pub type R = crate::R<MisoSpec>;
#[doc = "Register `MISO` writer"]
pub type W = crate::W<MisoSpec>;
#[doc = "Pin number configuration for SPI MISO signal\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Pselmiso {
    #[doc = "4294967295: Disconnect"]
    Disconnected = 4294967295,
}
impl From<Pselmiso> for u32 {
    #[inline(always)]
    fn from(variant: Pselmiso) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pselmiso {
    type Ux = u32;
}
impl crate::IsEnum for Pselmiso {}
#[doc = "Field `PSELMISO` reader - Pin number configuration for SPI MISO signal"]
pub type PselmisoR = crate::FieldReader<Pselmiso>;
impl PselmisoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pselmiso> {
        match self.bits {
            4294967295 => Some(Pselmiso::Disconnected),
            _ => None,
        }
    }
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == Pselmiso::Disconnected
    }
}
#[doc = "Field `PSELMISO` writer - Pin number configuration for SPI MISO signal"]
pub type PselmisoW<'a, REG> = crate::FieldWriter<'a, REG, 32, Pselmiso>;
impl<'a, REG> PselmisoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(Pselmiso::Disconnected)
    }
}
impl R {
    #[doc = "Bits 0:31 - Pin number configuration for SPI MISO signal"]
    #[inline(always)]
    pub fn pselmiso(&self) -> PselmisoR {
        PselmisoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pin number configuration for SPI MISO signal"]
    #[inline(always)]
    pub fn pselmiso(&mut self) -> PselmisoW<'_, MisoSpec> {
        PselmisoW::new(self, 0)
    }
}
#[doc = "Pin select for MISO\n\nYou can [`read`](crate::Reg::read) this register and get [`miso::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miso::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisoSpec;
impl crate::RegisterSpec for MisoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`miso::R`](R) reader structure"]
impl crate::Readable for MisoSpec {}
#[doc = "`write(|w| ..)` method takes [`miso::W`](W) writer structure"]
impl crate::Writable for MisoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MISO to value 0xffff_ffff"]
impl crate::Resettable for MisoSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
