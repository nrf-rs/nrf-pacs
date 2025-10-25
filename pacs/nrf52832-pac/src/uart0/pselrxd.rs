#[doc = "Register `PSELRXD` reader"]
pub type R = crate::R<PselrxdSpec>;
#[doc = "Register `PSELRXD` writer"]
pub type W = crate::W<PselrxdSpec>;
#[doc = "Pin number configuration for UART RXD signal\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Pselrxd {
    #[doc = "4294967295: Disconnect"]
    Disconnected = 4294967295,
}
impl From<Pselrxd> for u32 {
    #[inline(always)]
    fn from(variant: Pselrxd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pselrxd {
    type Ux = u32;
}
impl crate::IsEnum for Pselrxd {}
#[doc = "Field `PSELRXD` reader - Pin number configuration for UART RXD signal"]
pub type PselrxdR = crate::FieldReader<Pselrxd>;
impl PselrxdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pselrxd> {
        match self.bits {
            4294967295 => Some(Pselrxd::Disconnected),
            _ => None,
        }
    }
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == Pselrxd::Disconnected
    }
}
#[doc = "Field `PSELRXD` writer - Pin number configuration for UART RXD signal"]
pub type PselrxdW<'a, REG> = crate::FieldWriter<'a, REG, 32, Pselrxd>;
impl<'a, REG> PselrxdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(Pselrxd::Disconnected)
    }
}
impl R {
    #[doc = "Bits 0:31 - Pin number configuration for UART RXD signal"]
    #[inline(always)]
    pub fn pselrxd(&self) -> PselrxdR {
        PselrxdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pin number configuration for UART RXD signal"]
    #[inline(always)]
    pub fn pselrxd(&mut self) -> PselrxdW<'_, PselrxdSpec> {
        PselrxdW::new(self, 0)
    }
}
#[doc = "Pin select for RXD\n\nYou can [`read`](crate::Reg::read) this register and get [`pselrxd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselrxd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PselrxdSpec;
impl crate::RegisterSpec for PselrxdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pselrxd::R`](R) reader structure"]
impl crate::Readable for PselrxdSpec {}
#[doc = "`write(|w| ..)` method takes [`pselrxd::W`](W) writer structure"]
impl crate::Writable for PselrxdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSELRXD to value 0xffff_ffff"]
impl crate::Resettable for PselrxdSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
