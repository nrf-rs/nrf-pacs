#[doc = "Register `IDFILTER0` reader"]
pub type R = crate::R<Idfilter0Spec>;
#[doc = "Register `IDFILTER0` writer"]
pub type W = crate::W<Idfilter0Spec>;
#[doc = "Enable or disable ID filtering for IDs 0x00_0x0F.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Id0_00_0f {
    #[doc = "0: Transactions with these IDs are passed on to ATB master port 0."]
    NotFiltered = 0,
    #[doc = "1: Transactions with these IDs are discarded by the replicator."]
    Selected = 1,
}
impl From<Id0_00_0f> for bool {
    #[inline(always)]
    fn from(variant: Id0_00_0f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ID0_00_0F` reader - Enable or disable ID filtering for IDs 0x00_0x0F."]
pub type Id0_00_0fR = crate::BitReader<Id0_00_0f>;
impl Id0_00_0fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Id0_00_0f {
        match self.bits {
            false => Id0_00_0f::NotFiltered,
            true => Id0_00_0f::Selected,
        }
    }
    #[doc = "Transactions with these IDs are passed on to ATB master port 0."]
    #[inline(always)]
    pub fn is_not_filtered(&self) -> bool {
        *self == Id0_00_0f::NotFiltered
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == Id0_00_0f::Selected
    }
}
#[doc = "Field `ID0_00_0F` writer - Enable or disable ID filtering for IDs 0x00_0x0F."]
pub type Id0_00_0fW<'a, REG> = crate::BitWriter<'a, REG, Id0_00_0f>;
impl<'a, REG> Id0_00_0fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transactions with these IDs are passed on to ATB master port 0."]
    #[inline(always)]
    pub fn not_filtered(self) -> &'a mut crate::W<REG> {
        self.variant(Id0_00_0f::NotFiltered)
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(Id0_00_0f::Selected)
    }
}
#[doc = "Enable or disable ID filtering for IDs 0x10_0x1F.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Id0_10_1f {
    #[doc = "0: Transactions with these IDs are passed on to ATB master port 0."]
    NotFiltered = 0,
    #[doc = "1: Transactions with these IDs are discarded by the replicator."]
    Selected = 1,
}
impl From<Id0_10_1f> for bool {
    #[inline(always)]
    fn from(variant: Id0_10_1f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ID0_10_1F` reader - Enable or disable ID filtering for IDs 0x10_0x1F."]
pub type Id0_10_1fR = crate::BitReader<Id0_10_1f>;
impl Id0_10_1fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Id0_10_1f {
        match self.bits {
            false => Id0_10_1f::NotFiltered,
            true => Id0_10_1f::Selected,
        }
    }
    #[doc = "Transactions with these IDs are passed on to ATB master port 0."]
    #[inline(always)]
    pub fn is_not_filtered(&self) -> bool {
        *self == Id0_10_1f::NotFiltered
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == Id0_10_1f::Selected
    }
}
#[doc = "Field `ID0_10_1F` writer - Enable or disable ID filtering for IDs 0x10_0x1F."]
pub type Id0_10_1fW<'a, REG> = crate::BitWriter<'a, REG, Id0_10_1f>;
impl<'a, REG> Id0_10_1fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transactions with these IDs are passed on to ATB master port 0."]
    #[inline(always)]
    pub fn not_filtered(self) -> &'a mut crate::W<REG> {
        self.variant(Id0_10_1f::NotFiltered)
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(Id0_10_1f::Selected)
    }
}
#[doc = "Enable or disable ID filtering for IDs 0x20_0x2F.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Id0_20_2f {
    #[doc = "0: Transactions with these IDs are passed on to ATB master port 0."]
    NotFiltered = 0,
    #[doc = "1: Transactions with these IDs are discarded by the replicator."]
    Selected = 1,
}
impl From<Id0_20_2f> for bool {
    #[inline(always)]
    fn from(variant: Id0_20_2f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ID0_20_2F` reader - Enable or disable ID filtering for IDs 0x20_0x2F."]
pub type Id0_20_2fR = crate::BitReader<Id0_20_2f>;
impl Id0_20_2fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Id0_20_2f {
        match self.bits {
            false => Id0_20_2f::NotFiltered,
            true => Id0_20_2f::Selected,
        }
    }
    #[doc = "Transactions with these IDs are passed on to ATB master port 0."]
    #[inline(always)]
    pub fn is_not_filtered(&self) -> bool {
        *self == Id0_20_2f::NotFiltered
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == Id0_20_2f::Selected
    }
}
#[doc = "Field `ID0_20_2F` writer - Enable or disable ID filtering for IDs 0x20_0x2F."]
pub type Id0_20_2fW<'a, REG> = crate::BitWriter<'a, REG, Id0_20_2f>;
impl<'a, REG> Id0_20_2fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transactions with these IDs are passed on to ATB master port 0."]
    #[inline(always)]
    pub fn not_filtered(self) -> &'a mut crate::W<REG> {
        self.variant(Id0_20_2f::NotFiltered)
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(Id0_20_2f::Selected)
    }
}
#[doc = "Enable or disable ID filtering for IDs 0x30_0x3F.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Id0_30_3f {
    #[doc = "0: Transactions with these IDs are passed on to ATB master port 0."]
    NotFiltered = 0,
    #[doc = "1: Transactions with these IDs are discarded by the replicator."]
    Selected = 1,
}
impl From<Id0_30_3f> for bool {
    #[inline(always)]
    fn from(variant: Id0_30_3f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ID0_30_3F` reader - Enable or disable ID filtering for IDs 0x30_0x3F."]
pub type Id0_30_3fR = crate::BitReader<Id0_30_3f>;
impl Id0_30_3fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Id0_30_3f {
        match self.bits {
            false => Id0_30_3f::NotFiltered,
            true => Id0_30_3f::Selected,
        }
    }
    #[doc = "Transactions with these IDs are passed on to ATB master port 0."]
    #[inline(always)]
    pub fn is_not_filtered(&self) -> bool {
        *self == Id0_30_3f::NotFiltered
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == Id0_30_3f::Selected
    }
}
#[doc = "Field `ID0_30_3F` writer - Enable or disable ID filtering for IDs 0x30_0x3F."]
pub type Id0_30_3fW<'a, REG> = crate::BitWriter<'a, REG, Id0_30_3f>;
impl<'a, REG> Id0_30_3fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transactions with these IDs are passed on to ATB master port 0."]
    #[inline(always)]
    pub fn not_filtered(self) -> &'a mut crate::W<REG> {
        self.variant(Id0_30_3f::NotFiltered)
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(Id0_30_3f::Selected)
    }
}
#[doc = "Enable or disable ID filtering for IDs 0x40_0x4F.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Id0_40_4f {
    #[doc = "0: Transactions with these IDs are passed on to ATB master port 0."]
    NotFiltered = 0,
    #[doc = "1: Transactions with these IDs are discarded by the replicator."]
    Selected = 1,
}
impl From<Id0_40_4f> for bool {
    #[inline(always)]
    fn from(variant: Id0_40_4f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ID0_40_4F` reader - Enable or disable ID filtering for IDs 0x40_0x4F."]
pub type Id0_40_4fR = crate::BitReader<Id0_40_4f>;
impl Id0_40_4fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Id0_40_4f {
        match self.bits {
            false => Id0_40_4f::NotFiltered,
            true => Id0_40_4f::Selected,
        }
    }
    #[doc = "Transactions with these IDs are passed on to ATB master port 0."]
    #[inline(always)]
    pub fn is_not_filtered(&self) -> bool {
        *self == Id0_40_4f::NotFiltered
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == Id0_40_4f::Selected
    }
}
#[doc = "Field `ID0_40_4F` writer - Enable or disable ID filtering for IDs 0x40_0x4F."]
pub type Id0_40_4fW<'a, REG> = crate::BitWriter<'a, REG, Id0_40_4f>;
impl<'a, REG> Id0_40_4fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transactions with these IDs are passed on to ATB master port 0."]
    #[inline(always)]
    pub fn not_filtered(self) -> &'a mut crate::W<REG> {
        self.variant(Id0_40_4f::NotFiltered)
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(Id0_40_4f::Selected)
    }
}
#[doc = "Enable or disable ID filtering for IDs 0x50_0x5F.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Id0_50_5f {
    #[doc = "0: Transactions with these IDs are passed on to ATB master port 0."]
    NotFiltered = 0,
    #[doc = "1: Transactions with these IDs are discarded by the replicator."]
    Selected = 1,
}
impl From<Id0_50_5f> for bool {
    #[inline(always)]
    fn from(variant: Id0_50_5f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ID0_50_5F` reader - Enable or disable ID filtering for IDs 0x50_0x5F."]
pub type Id0_50_5fR = crate::BitReader<Id0_50_5f>;
impl Id0_50_5fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Id0_50_5f {
        match self.bits {
            false => Id0_50_5f::NotFiltered,
            true => Id0_50_5f::Selected,
        }
    }
    #[doc = "Transactions with these IDs are passed on to ATB master port 0."]
    #[inline(always)]
    pub fn is_not_filtered(&self) -> bool {
        *self == Id0_50_5f::NotFiltered
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == Id0_50_5f::Selected
    }
}
#[doc = "Field `ID0_50_5F` writer - Enable or disable ID filtering for IDs 0x50_0x5F."]
pub type Id0_50_5fW<'a, REG> = crate::BitWriter<'a, REG, Id0_50_5f>;
impl<'a, REG> Id0_50_5fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transactions with these IDs are passed on to ATB master port 0."]
    #[inline(always)]
    pub fn not_filtered(self) -> &'a mut crate::W<REG> {
        self.variant(Id0_50_5f::NotFiltered)
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(Id0_50_5f::Selected)
    }
}
#[doc = "Enable or disable ID filtering for IDs 0x60_0x6F.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Id0_60_6f {
    #[doc = "0: Transactions with these IDs are passed on to ATB master port 0."]
    NotFiltered = 0,
    #[doc = "1: Transactions with these IDs are discarded by the replicator."]
    Selected = 1,
}
impl From<Id0_60_6f> for bool {
    #[inline(always)]
    fn from(variant: Id0_60_6f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ID0_60_6F` reader - Enable or disable ID filtering for IDs 0x60_0x6F."]
pub type Id0_60_6fR = crate::BitReader<Id0_60_6f>;
impl Id0_60_6fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Id0_60_6f {
        match self.bits {
            false => Id0_60_6f::NotFiltered,
            true => Id0_60_6f::Selected,
        }
    }
    #[doc = "Transactions with these IDs are passed on to ATB master port 0."]
    #[inline(always)]
    pub fn is_not_filtered(&self) -> bool {
        *self == Id0_60_6f::NotFiltered
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == Id0_60_6f::Selected
    }
}
#[doc = "Field `ID0_60_6F` writer - Enable or disable ID filtering for IDs 0x60_0x6F."]
pub type Id0_60_6fW<'a, REG> = crate::BitWriter<'a, REG, Id0_60_6f>;
impl<'a, REG> Id0_60_6fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transactions with these IDs are passed on to ATB master port 0."]
    #[inline(always)]
    pub fn not_filtered(self) -> &'a mut crate::W<REG> {
        self.variant(Id0_60_6f::NotFiltered)
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(Id0_60_6f::Selected)
    }
}
#[doc = "Enable or disable ID filtering for IDs 0x70_0x7F.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Id0_70_7f {
    #[doc = "0: Transactions with these IDs are passed on to ATB master port 0."]
    NotFiltered = 0,
    #[doc = "1: Transactions with these IDs are discarded by the replicator."]
    Selected = 1,
}
impl From<Id0_70_7f> for bool {
    #[inline(always)]
    fn from(variant: Id0_70_7f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ID0_70_7F` reader - Enable or disable ID filtering for IDs 0x70_0x7F."]
pub type Id0_70_7fR = crate::BitReader<Id0_70_7f>;
impl Id0_70_7fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Id0_70_7f {
        match self.bits {
            false => Id0_70_7f::NotFiltered,
            true => Id0_70_7f::Selected,
        }
    }
    #[doc = "Transactions with these IDs are passed on to ATB master port 0."]
    #[inline(always)]
    pub fn is_not_filtered(&self) -> bool {
        *self == Id0_70_7f::NotFiltered
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == Id0_70_7f::Selected
    }
}
#[doc = "Field `ID0_70_7F` writer - Enable or disable ID filtering for IDs 0x70_0x7F."]
pub type Id0_70_7fW<'a, REG> = crate::BitWriter<'a, REG, Id0_70_7f>;
impl<'a, REG> Id0_70_7fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transactions with these IDs are passed on to ATB master port 0."]
    #[inline(always)]
    pub fn not_filtered(self) -> &'a mut crate::W<REG> {
        self.variant(Id0_70_7f::NotFiltered)
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(Id0_70_7f::Selected)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable ID filtering for IDs 0x00_0x0F."]
    #[inline(always)]
    pub fn id0_00_0f(&self) -> Id0_00_0fR {
        Id0_00_0fR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable ID filtering for IDs 0x10_0x1F."]
    #[inline(always)]
    pub fn id0_10_1f(&self) -> Id0_10_1fR {
        Id0_10_1fR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable ID filtering for IDs 0x20_0x2F."]
    #[inline(always)]
    pub fn id0_20_2f(&self) -> Id0_20_2fR {
        Id0_20_2fR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable ID filtering for IDs 0x30_0x3F."]
    #[inline(always)]
    pub fn id0_30_3f(&self) -> Id0_30_3fR {
        Id0_30_3fR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable ID filtering for IDs 0x40_0x4F."]
    #[inline(always)]
    pub fn id0_40_4f(&self) -> Id0_40_4fR {
        Id0_40_4fR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable or disable ID filtering for IDs 0x50_0x5F."]
    #[inline(always)]
    pub fn id0_50_5f(&self) -> Id0_50_5fR {
        Id0_50_5fR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable or disable ID filtering for IDs 0x60_0x6F."]
    #[inline(always)]
    pub fn id0_60_6f(&self) -> Id0_60_6fR {
        Id0_60_6fR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable or disable ID filtering for IDs 0x70_0x7F."]
    #[inline(always)]
    pub fn id0_70_7f(&self) -> Id0_70_7fR {
        Id0_70_7fR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable ID filtering for IDs 0x00_0x0F."]
    #[inline(always)]
    pub fn id0_00_0f(&mut self) -> Id0_00_0fW<'_, Idfilter0Spec> {
        Id0_00_0fW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable or disable ID filtering for IDs 0x10_0x1F."]
    #[inline(always)]
    pub fn id0_10_1f(&mut self) -> Id0_10_1fW<'_, Idfilter0Spec> {
        Id0_10_1fW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable or disable ID filtering for IDs 0x20_0x2F."]
    #[inline(always)]
    pub fn id0_20_2f(&mut self) -> Id0_20_2fW<'_, Idfilter0Spec> {
        Id0_20_2fW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable or disable ID filtering for IDs 0x30_0x3F."]
    #[inline(always)]
    pub fn id0_30_3f(&mut self) -> Id0_30_3fW<'_, Idfilter0Spec> {
        Id0_30_3fW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable or disable ID filtering for IDs 0x40_0x4F."]
    #[inline(always)]
    pub fn id0_40_4f(&mut self) -> Id0_40_4fW<'_, Idfilter0Spec> {
        Id0_40_4fW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable or disable ID filtering for IDs 0x50_0x5F."]
    #[inline(always)]
    pub fn id0_50_5f(&mut self) -> Id0_50_5fW<'_, Idfilter0Spec> {
        Id0_50_5fW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable or disable ID filtering for IDs 0x60_0x6F."]
    #[inline(always)]
    pub fn id0_60_6f(&mut self) -> Id0_60_6fW<'_, Idfilter0Spec> {
        Id0_60_6fW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable or disable ID filtering for IDs 0x70_0x7F."]
    #[inline(always)]
    pub fn id0_70_7f(&mut self) -> Id0_70_7fW<'_, Idfilter0Spec> {
        Id0_70_7fW::new(self, 7)
    }
}
#[doc = "The IDFILTER0 register enables the programming of ID filtering for master port 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`idfilter0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idfilter0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idfilter0Spec;
impl crate::RegisterSpec for Idfilter0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idfilter0::R`](R) reader structure"]
impl crate::Readable for Idfilter0Spec {}
#[doc = "`write(|w| ..)` method takes [`idfilter0::W`](W) writer structure"]
impl crate::Writable for Idfilter0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDFILTER0 to value 0"]
impl crate::Resettable for Idfilter0Spec {}
