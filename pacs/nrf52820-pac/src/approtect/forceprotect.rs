#[doc = "Register `FORCEPROTECT` reader"]
pub type R = crate::R<ForceprotectSpec>;
#[doc = "Register `FORCEPROTECT` writer"]
pub type W = crate::W<ForceprotectSpec>;
#[doc = "Write 0x0 to force enable APPROTECT mechanism\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Forceprotect {
    #[doc = "0: Software force enable APPROTECT mechanism"]
    Force = 0,
}
impl From<Forceprotect> for u8 {
    #[inline(always)]
    fn from(variant: Forceprotect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Forceprotect {
    type Ux = u8;
}
impl crate::IsEnum for Forceprotect {}
#[doc = "Field `FORCEPROTECT` reader - Write 0x0 to force enable APPROTECT mechanism"]
pub type ForceprotectR = crate::FieldReader<Forceprotect>;
impl ForceprotectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Forceprotect> {
        match self.bits {
            0 => Some(Forceprotect::Force),
            _ => None,
        }
    }
    #[doc = "Software force enable APPROTECT mechanism"]
    #[inline(always)]
    pub fn is_force(&self) -> bool {
        *self == Forceprotect::Force
    }
}
#[doc = "Field `FORCEPROTECT` writer - Write 0x0 to force enable APPROTECT mechanism"]
pub type ForceprotectW<'a, REG> = crate::FieldWriter<'a, REG, 8, Forceprotect>;
impl<'a, REG> ForceprotectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software force enable APPROTECT mechanism"]
    #[inline(always)]
    pub fn force(self) -> &'a mut crate::W<REG> {
        self.variant(Forceprotect::Force)
    }
}
impl R {
    #[doc = "Bits 0:7 - Write 0x0 to force enable APPROTECT mechanism"]
    #[inline(always)]
    pub fn forceprotect(&self) -> ForceprotectR {
        ForceprotectR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Write 0x0 to force enable APPROTECT mechanism"]
    #[inline(always)]
    pub fn forceprotect(&mut self) -> ForceprotectW<'_, ForceprotectSpec> {
        ForceprotectW::new(self, 0)
    }
}
#[doc = "Software force enable APPROTECT mechanism until next reset.\n\nYou can [`read`](crate::Reg::read) this register and get [`forceprotect::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`forceprotect::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ForceprotectSpec;
impl crate::RegisterSpec for ForceprotectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`forceprotect::R`](R) reader structure"]
impl crate::Readable for ForceprotectSpec {}
#[doc = "`write(|w| ..)` method takes [`forceprotect::W`](W) writer structure"]
impl crate::Writable for ForceprotectSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FORCEPROTECT to value 0xffff_ffff"]
impl crate::Resettable for ForceprotectSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
