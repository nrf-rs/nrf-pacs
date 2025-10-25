#[doc = "Register `SCK` reader"]
pub type R = crate::R<SckSpec>;
#[doc = "Register `SCK` writer"]
pub type W = crate::W<SckSpec>;
#[doc = "Pin number configuration for SPI SCK signal\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Pselsck {
    #[doc = "4294967295: Disconnect"]
    Disconnected = 4294967295,
}
impl From<Pselsck> for u32 {
    #[inline(always)]
    fn from(variant: Pselsck) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pselsck {
    type Ux = u32;
}
impl crate::IsEnum for Pselsck {}
#[doc = "Field `PSELSCK` reader - Pin number configuration for SPI SCK signal"]
pub type PselsckR = crate::FieldReader<Pselsck>;
impl PselsckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pselsck> {
        match self.bits {
            4294967295 => Some(Pselsck::Disconnected),
            _ => None,
        }
    }
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == Pselsck::Disconnected
    }
}
#[doc = "Field `PSELSCK` writer - Pin number configuration for SPI SCK signal"]
pub type PselsckW<'a, REG> = crate::FieldWriter<'a, REG, 32, Pselsck>;
impl<'a, REG> PselsckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(Pselsck::Disconnected)
    }
}
impl R {
    #[doc = "Bits 0:31 - Pin number configuration for SPI SCK signal"]
    #[inline(always)]
    pub fn pselsck(&self) -> PselsckR {
        PselsckR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pin number configuration for SPI SCK signal"]
    #[inline(always)]
    pub fn pselsck(&mut self) -> PselsckW<'_, SckSpec> {
        PselsckW::new(self, 0)
    }
}
#[doc = "Pin select for SCK\n\nYou can [`read`](crate::Reg::read) this register and get [`sck::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sck::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SckSpec;
impl crate::RegisterSpec for SckSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sck::R`](R) reader structure"]
impl crate::Readable for SckSpec {}
#[doc = "`write(|w| ..)` method takes [`sck::W`](W) writer structure"]
impl crate::Writable for SckSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCK to value 0xffff_ffff"]
impl crate::Resettable for SckSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
