#[doc = "Register `CODESIZE` reader"]
pub type R = crate::R<CodesizeSpec>;
#[doc = "Code memory size in number of pages Total code space is: CODEPAGESIZE * CODESIZE\n\nValue on reset: 256"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Codesize {
    #[doc = "256: 256 pages"]
    P256 = 256,
}
impl From<Codesize> for u32 {
    #[inline(always)]
    fn from(variant: Codesize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Codesize {
    type Ux = u32;
}
impl crate::IsEnum for Codesize {}
#[doc = "Field `CODESIZE` reader - Code memory size in number of pages Total code space is: CODEPAGESIZE * CODESIZE"]
pub type CodesizeR = crate::FieldReader<Codesize>;
impl CodesizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Codesize> {
        match self.bits {
            256 => Some(Codesize::P256),
            _ => None,
        }
    }
    #[doc = "256 pages"]
    #[inline(always)]
    pub fn is_p256(&self) -> bool {
        *self == Codesize::P256
    }
}
impl R {
    #[doc = "Bits 0:31 - Code memory size in number of pages Total code space is: CODEPAGESIZE * CODESIZE"]
    #[inline(always)]
    pub fn codesize(&self) -> CodesizeR {
        CodesizeR::new(self.bits)
    }
}
#[doc = "Code memory size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`codesize::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CodesizeSpec;
impl crate::RegisterSpec for CodesizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`codesize::R`](R) reader structure"]
impl crate::Readable for CodesizeSpec {}
#[doc = "`reset()` method sets CODESIZE to value 0x0100"]
impl crate::Resettable for CodesizeSpec {
    const RESET_VALUE: u32 = 0x0100;
}
