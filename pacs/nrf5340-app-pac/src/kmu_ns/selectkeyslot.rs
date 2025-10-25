#[doc = "Register `SELECTKEYSLOT` reader"]
pub type R = crate::R<SelectkeyslotSpec>;
#[doc = "Register `SELECTKEYSLOT` writer"]
pub type W = crate::W<SelectkeyslotSpec>;
#[doc = "Field `ID` reader - Select key slot ID to be read over AHB, or pushed over secure APB, when TASKS_PUSH_KEYSLOT is started. NOTE: ID=0 is not a valid key slot ID. The 0 ID should be used when the KMU is idle or not in use. NOTE: Index N in UICR-&gt;KEYSLOT.KEY\\[N\\] and UICR-&gt;KEYSLOT.CONFIG\\[N\\] corresponds to KMU key slot ID=N+1."]
pub type IdR = crate::FieldReader;
#[doc = "Field `ID` writer - Select key slot ID to be read over AHB, or pushed over secure APB, when TASKS_PUSH_KEYSLOT is started. NOTE: ID=0 is not a valid key slot ID. The 0 ID should be used when the KMU is idle or not in use. NOTE: Index N in UICR-&gt;KEYSLOT.KEY\\[N\\] and UICR-&gt;KEYSLOT.CONFIG\\[N\\] corresponds to KMU key slot ID=N+1."]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Select key slot ID to be read over AHB, or pushed over secure APB, when TASKS_PUSH_KEYSLOT is started. NOTE: ID=0 is not a valid key slot ID. The 0 ID should be used when the KMU is idle or not in use. NOTE: Index N in UICR-&gt;KEYSLOT.KEY\\[N\\] and UICR-&gt;KEYSLOT.CONFIG\\[N\\] corresponds to KMU key slot ID=N+1."]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Select key slot ID to be read over AHB, or pushed over secure APB, when TASKS_PUSH_KEYSLOT is started. NOTE: ID=0 is not a valid key slot ID. The 0 ID should be used when the KMU is idle or not in use. NOTE: Index N in UICR-&gt;KEYSLOT.KEY\\[N\\] and UICR-&gt;KEYSLOT.CONFIG\\[N\\] corresponds to KMU key slot ID=N+1."]
    #[inline(always)]
    pub fn id(&mut self) -> IdW<'_, SelectkeyslotSpec> {
        IdW::new(self, 0)
    }
}
#[doc = "Select key slot to be read over AHB or pushed over secure APB when TASKS_PUSH_KEYSLOT is started\n\nYou can [`read`](crate::Reg::read) this register and get [`selectkeyslot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`selectkeyslot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SelectkeyslotSpec;
impl crate::RegisterSpec for SelectkeyslotSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`selectkeyslot::R`](R) reader structure"]
impl crate::Readable for SelectkeyslotSpec {}
#[doc = "`write(|w| ..)` method takes [`selectkeyslot::W`](W) writer structure"]
impl crate::Writable for SelectkeyslotSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SELECTKEYSLOT to value 0"]
impl crate::Resettable for SelectkeyslotSpec {}
