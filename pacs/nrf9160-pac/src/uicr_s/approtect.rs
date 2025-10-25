#[doc = "Register `APPROTECT` reader"]
pub type R = crate::R<ApprotectSpec>;
#[doc = "Register `APPROTECT` writer"]
pub type W = crate::W<ApprotectSpec>;
#[doc = "Blocks debugger read/write access to all CPU registers and memory mapped addresses\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Pall {
    #[doc = "4294967295: Unprotected"]
    Unprotected = 4294967295,
    #[doc = "0: Protected"]
    Protected = 0,
}
impl From<Pall> for u32 {
    #[inline(always)]
    fn from(variant: Pall) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pall {
    type Ux = u32;
}
impl crate::IsEnum for Pall {}
#[doc = "Field `PALL` reader - Blocks debugger read/write access to all CPU registers and memory mapped addresses"]
pub type PallR = crate::FieldReader<Pall>;
impl PallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pall> {
        match self.bits {
            4294967295 => Some(Pall::Unprotected),
            0 => Some(Pall::Protected),
            _ => None,
        }
    }
    #[doc = "Unprotected"]
    #[inline(always)]
    pub fn is_unprotected(&self) -> bool {
        *self == Pall::Unprotected
    }
    #[doc = "Protected"]
    #[inline(always)]
    pub fn is_protected(&self) -> bool {
        *self == Pall::Protected
    }
}
#[doc = "Field `PALL` writer - Blocks debugger read/write access to all CPU registers and memory mapped addresses"]
pub type PallW<'a, REG> = crate::FieldWriter<'a, REG, 32, Pall>;
impl<'a, REG> PallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Unprotected"]
    #[inline(always)]
    pub fn unprotected(self) -> &'a mut crate::W<REG> {
        self.variant(Pall::Unprotected)
    }
    #[doc = "Protected"]
    #[inline(always)]
    pub fn protected(self) -> &'a mut crate::W<REG> {
        self.variant(Pall::Protected)
    }
}
impl R {
    #[doc = "Bits 0:31 - Blocks debugger read/write access to all CPU registers and memory mapped addresses"]
    #[inline(always)]
    pub fn pall(&self) -> PallR {
        PallR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Blocks debugger read/write access to all CPU registers and memory mapped addresses"]
    #[inline(always)]
    pub fn pall(&mut self) -> PallW<'_, ApprotectSpec> {
        PallW::new(self, 0)
    }
}
#[doc = "Access port protection\n\nYou can [`read`](crate::Reg::read) this register and get [`approtect::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`approtect::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApprotectSpec;
impl crate::RegisterSpec for ApprotectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`approtect::R`](R) reader structure"]
impl crate::Readable for ApprotectSpec {}
#[doc = "`write(|w| ..)` method takes [`approtect::W`](W) writer structure"]
impl crate::Writable for ApprotectSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APPROTECT to value 0xffff_ffff"]
impl crate::Resettable for ApprotectSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
