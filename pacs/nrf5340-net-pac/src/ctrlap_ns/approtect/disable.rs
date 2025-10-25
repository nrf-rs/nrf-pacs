#[doc = "Register `DISABLE` reader"]
pub type R = crate::R<DisableSpec>;
#[doc = "Register `DISABLE` writer"]
pub type W = crate::W<DisableSpec>;
#[doc = "Field `KEY` reader - If the value of the KEY field is non-zero, and the KEY fields match on both the CPU and debugger sides, disable APPROTECT and enable debug access to non-secure mode until the next pin reset, brown-out reset, power-on reset, or watchog timer reset. After reset the debugger side register has a fixed KEY value. To enable debug access, both CTRL-AP and UICR.APPROTECT protection needs to be disabled."]
pub type KeyR = crate::FieldReader<u32>;
#[doc = "Field `KEY` writer - If the value of the KEY field is non-zero, and the KEY fields match on both the CPU and debugger sides, disable APPROTECT and enable debug access to non-secure mode until the next pin reset, brown-out reset, power-on reset, or watchog timer reset. After reset the debugger side register has a fixed KEY value. To enable debug access, both CTRL-AP and UICR.APPROTECT protection needs to be disabled."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - If the value of the KEY field is non-zero, and the KEY fields match on both the CPU and debugger sides, disable APPROTECT and enable debug access to non-secure mode until the next pin reset, brown-out reset, power-on reset, or watchog timer reset. After reset the debugger side register has a fixed KEY value. To enable debug access, both CTRL-AP and UICR.APPROTECT protection needs to be disabled."]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - If the value of the KEY field is non-zero, and the KEY fields match on both the CPU and debugger sides, disable APPROTECT and enable debug access to non-secure mode until the next pin reset, brown-out reset, power-on reset, or watchog timer reset. After reset the debugger side register has a fixed KEY value. To enable debug access, both CTRL-AP and UICR.APPROTECT protection needs to be disabled."]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, DisableSpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "This register disables the APPROTECT register and enables debug access to non-secure mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`disable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DisableSpec;
impl crate::RegisterSpec for DisableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`disable::R`](R) reader structure"]
impl crate::Readable for DisableSpec {}
#[doc = "`write(|w| ..)` method takes [`disable::W`](W) writer structure"]
impl crate::Writable for DisableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DISABLE to value 0"]
impl crate::Resettable for DisableSpec {}
