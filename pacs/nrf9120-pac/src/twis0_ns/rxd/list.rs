#[doc = "Register `LIST` reader"]
pub type R = crate::R<ListSpec>;
#[doc = "Register `LIST` writer"]
pub type W = crate::W<ListSpec>;
#[doc = "List type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum List {
    #[doc = "0: Disable EasyDMA list"]
    Disabled = 0,
    #[doc = "1: Use array list"]
    ArrayList = 1,
}
impl From<List> for u8 {
    #[inline(always)]
    fn from(variant: List) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for List {
    type Ux = u8;
}
impl crate::IsEnum for List {}
#[doc = "Field `LIST` reader - List type"]
pub type ListR = crate::FieldReader<List>;
impl ListR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<List> {
        match self.bits {
            0 => Some(List::Disabled),
            1 => Some(List::ArrayList),
            _ => None,
        }
    }
    #[doc = "Disable EasyDMA list"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == List::Disabled
    }
    #[doc = "Use array list"]
    #[inline(always)]
    pub fn is_array_list(&self) -> bool {
        *self == List::ArrayList
    }
}
#[doc = "Field `LIST` writer - List type"]
pub type ListW<'a, REG> = crate::FieldWriter<'a, REG, 2, List>;
impl<'a, REG> ListW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable EasyDMA list"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(List::Disabled)
    }
    #[doc = "Use array list"]
    #[inline(always)]
    pub fn array_list(self) -> &'a mut crate::W<REG> {
        self.variant(List::ArrayList)
    }
}
impl R {
    #[doc = "Bits 0:1 - List type"]
    #[inline(always)]
    pub fn list(&self) -> ListR {
        ListR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - List type"]
    #[inline(always)]
    #[must_use]
    pub fn list(&mut self) -> ListW<ListSpec> {
        ListW::new(self, 0)
    }
}
#[doc = "EasyDMA list type\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`list::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`list::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ListSpec;
impl crate::RegisterSpec for ListSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`list::R`](R) reader structure"]
impl crate::Readable for ListSpec {}
#[doc = "`write(|w| ..)` method takes [`list::W`](W) writer structure"]
impl crate::Writable for ListSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LIST to value 0"]
impl crate::Resettable for ListSpec {
    const RESET_VALUE: u32 = 0;
}
