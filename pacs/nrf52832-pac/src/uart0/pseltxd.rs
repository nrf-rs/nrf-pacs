#[doc = "Register `PSELTXD` reader"]
pub type R = crate::R<PseltxdSpec>;
#[doc = "Register `PSELTXD` writer"]
pub type W = crate::W<PseltxdSpec>;
#[doc = "Pin number configuration for UART TXD signal\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Pseltxd {
    #[doc = "4294967295: Disconnect"]
    Disconnected = 4294967295,
}
impl From<Pseltxd> for u32 {
    #[inline(always)]
    fn from(variant: Pseltxd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pseltxd {
    type Ux = u32;
}
impl crate::IsEnum for Pseltxd {}
#[doc = "Field `PSELTXD` reader - Pin number configuration for UART TXD signal"]
pub type PseltxdR = crate::FieldReader<Pseltxd>;
impl PseltxdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pseltxd> {
        match self.bits {
            4294967295 => Some(Pseltxd::Disconnected),
            _ => None,
        }
    }
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == Pseltxd::Disconnected
    }
}
#[doc = "Field `PSELTXD` writer - Pin number configuration for UART TXD signal"]
pub type PseltxdW<'a, REG> = crate::FieldWriter<'a, REG, 32, Pseltxd>;
impl<'a, REG> PseltxdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(Pseltxd::Disconnected)
    }
}
impl R {
    #[doc = "Bits 0:31 - Pin number configuration for UART TXD signal"]
    #[inline(always)]
    pub fn pseltxd(&self) -> PseltxdR {
        PseltxdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pin number configuration for UART TXD signal"]
    #[inline(always)]
    pub fn pseltxd(&mut self) -> PseltxdW<'_, PseltxdSpec> {
        PseltxdW::new(self, 0)
    }
}
#[doc = "Pin select for TXD\n\nYou can [`read`](crate::Reg::read) this register and get [`pseltxd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pseltxd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PseltxdSpec;
impl crate::RegisterSpec for PseltxdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pseltxd::R`](R) reader structure"]
impl crate::Readable for PseltxdSpec {}
#[doc = "`write(|w| ..)` method takes [`pseltxd::W`](W) writer structure"]
impl crate::Writable for PseltxdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSELTXD to value 0xffff_ffff"]
impl crate::Resettable for PseltxdSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
