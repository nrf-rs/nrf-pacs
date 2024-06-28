#[doc = "Register `CODEPAGESIZE` reader"]
pub type R = crate::R<CodepagesizeSpec>;
#[doc = "Code memory page size\n\nValue on reset: 4096"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Codepagesize {
    #[doc = "4096: 4 kByte"]
    K4096 = 4096,
}
impl From<Codepagesize> for u32 {
    #[inline(always)]
    fn from(variant: Codepagesize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Codepagesize {
    type Ux = u32;
}
impl crate::IsEnum for Codepagesize {}
#[doc = "Field `CODEPAGESIZE` reader - Code memory page size"]
pub type CodepagesizeR = crate::FieldReader<Codepagesize>;
impl CodepagesizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Codepagesize> {
        match self.bits {
            4096 => Some(Codepagesize::K4096),
            _ => None,
        }
    }
    #[doc = "4 kByte"]
    #[inline(always)]
    pub fn is_k4096(&self) -> bool {
        *self == Codepagesize::K4096
    }
}
impl R {
    #[doc = "Bits 0:31 - Code memory page size"]
    #[inline(always)]
    pub fn codepagesize(&self) -> CodepagesizeR {
        CodepagesizeR::new(self.bits)
    }
}
#[doc = "Code memory page size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`codepagesize::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CodepagesizeSpec;
impl crate::RegisterSpec for CodepagesizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`codepagesize::R`](R) reader structure"]
impl crate::Readable for CodepagesizeSpec {}
#[doc = "`reset()` method sets CODEPAGESIZE to value 0x1000"]
impl crate::Resettable for CodepagesizeSpec {
    const RESET_VALUE: u32 = 0x1000;
}
