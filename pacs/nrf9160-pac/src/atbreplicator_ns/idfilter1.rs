#[doc = "Register `IDFILTER1` reader"]
pub type R = crate::R<Idfilter1Spec>;
#[doc = "Register `IDFILTER1` writer"]
pub type W = crate::W<Idfilter1Spec>;
#[doc = "Enable or disable ID filtering for IDs 0x00_0x0F.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Id1_00_0f {
    #[doc = "0: Transactions with these IDs are passed on to ATB master port 1."]
    NotFiltered = 0,
    #[doc = "1: Transactions with these IDs are discarded by the replicator."]
    Selected = 1,
}
impl From<Id1_00_0f> for bool {
    #[inline(always)]
    fn from(variant: Id1_00_0f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ID1_00_0F` reader - Enable or disable ID filtering for IDs 0x00_0x0F."]
pub type Id1_00_0fR = crate::BitReader<Id1_00_0f>;
impl Id1_00_0fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Id1_00_0f {
        match self.bits {
            false => Id1_00_0f::NotFiltered,
            true => Id1_00_0f::Selected,
        }
    }
    #[doc = "Transactions with these IDs are passed on to ATB master port 1."]
    #[inline(always)]
    pub fn is_not_filtered(&self) -> bool {
        *self == Id1_00_0f::NotFiltered
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == Id1_00_0f::Selected
    }
}
#[doc = "Field `ID1_00_0F` writer - Enable or disable ID filtering for IDs 0x00_0x0F."]
pub type Id1_00_0fW<'a, REG> = crate::BitWriter<'a, REG, Id1_00_0f>;
impl<'a, REG> Id1_00_0fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transactions with these IDs are passed on to ATB master port 1."]
    #[inline(always)]
    pub fn not_filtered(self) -> &'a mut crate::W<REG> {
        self.variant(Id1_00_0f::NotFiltered)
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(Id1_00_0f::Selected)
    }
}
#[doc = "Enable or disable ID filtering for IDs 0x10_0x1F.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Id1_10_1f {
    #[doc = "0: Transactions with these IDs are passed on to ATB master port 1."]
    NotFiltered = 0,
    #[doc = "1: Transactions with these IDs are discarded by the replicator."]
    Selected = 1,
}
impl From<Id1_10_1f> for bool {
    #[inline(always)]
    fn from(variant: Id1_10_1f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ID1_10_1F` reader - Enable or disable ID filtering for IDs 0x10_0x1F."]
pub type Id1_10_1fR = crate::BitReader<Id1_10_1f>;
impl Id1_10_1fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Id1_10_1f {
        match self.bits {
            false => Id1_10_1f::NotFiltered,
            true => Id1_10_1f::Selected,
        }
    }
    #[doc = "Transactions with these IDs are passed on to ATB master port 1."]
    #[inline(always)]
    pub fn is_not_filtered(&self) -> bool {
        *self == Id1_10_1f::NotFiltered
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == Id1_10_1f::Selected
    }
}
#[doc = "Field `ID1_10_1F` writer - Enable or disable ID filtering for IDs 0x10_0x1F."]
pub type Id1_10_1fW<'a, REG> = crate::BitWriter<'a, REG, Id1_10_1f>;
impl<'a, REG> Id1_10_1fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transactions with these IDs are passed on to ATB master port 1."]
    #[inline(always)]
    pub fn not_filtered(self) -> &'a mut crate::W<REG> {
        self.variant(Id1_10_1f::NotFiltered)
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(Id1_10_1f::Selected)
    }
}
#[doc = "Enable or disable ID filtering for IDs 0x20_0x2F.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Id1_20_2f {
    #[doc = "0: Transactions with these IDs are passed on to ATB master port 1."]
    NotFiltered = 0,
    #[doc = "1: Transactions with these IDs are discarded by the replicator."]
    Selected = 1,
}
impl From<Id1_20_2f> for bool {
    #[inline(always)]
    fn from(variant: Id1_20_2f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ID1_20_2F` reader - Enable or disable ID filtering for IDs 0x20_0x2F."]
pub type Id1_20_2fR = crate::BitReader<Id1_20_2f>;
impl Id1_20_2fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Id1_20_2f {
        match self.bits {
            false => Id1_20_2f::NotFiltered,
            true => Id1_20_2f::Selected,
        }
    }
    #[doc = "Transactions with these IDs are passed on to ATB master port 1."]
    #[inline(always)]
    pub fn is_not_filtered(&self) -> bool {
        *self == Id1_20_2f::NotFiltered
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == Id1_20_2f::Selected
    }
}
#[doc = "Field `ID1_20_2F` writer - Enable or disable ID filtering for IDs 0x20_0x2F."]
pub type Id1_20_2fW<'a, REG> = crate::BitWriter<'a, REG, Id1_20_2f>;
impl<'a, REG> Id1_20_2fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transactions with these IDs are passed on to ATB master port 1."]
    #[inline(always)]
    pub fn not_filtered(self) -> &'a mut crate::W<REG> {
        self.variant(Id1_20_2f::NotFiltered)
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(Id1_20_2f::Selected)
    }
}
#[doc = "Enable or disable ID filtering for IDs 0x30_0x3F.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Id1_30_3f {
    #[doc = "0: Transactions with these IDs are passed on to ATB master port 1."]
    NotFiltered = 0,
    #[doc = "1: Transactions with these IDs are discarded by the replicator."]
    Selected = 1,
}
impl From<Id1_30_3f> for bool {
    #[inline(always)]
    fn from(variant: Id1_30_3f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ID1_30_3F` reader - Enable or disable ID filtering for IDs 0x30_0x3F."]
pub type Id1_30_3fR = crate::BitReader<Id1_30_3f>;
impl Id1_30_3fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Id1_30_3f {
        match self.bits {
            false => Id1_30_3f::NotFiltered,
            true => Id1_30_3f::Selected,
        }
    }
    #[doc = "Transactions with these IDs are passed on to ATB master port 1."]
    #[inline(always)]
    pub fn is_not_filtered(&self) -> bool {
        *self == Id1_30_3f::NotFiltered
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == Id1_30_3f::Selected
    }
}
#[doc = "Field `ID1_30_3F` writer - Enable or disable ID filtering for IDs 0x30_0x3F."]
pub type Id1_30_3fW<'a, REG> = crate::BitWriter<'a, REG, Id1_30_3f>;
impl<'a, REG> Id1_30_3fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transactions with these IDs are passed on to ATB master port 1."]
    #[inline(always)]
    pub fn not_filtered(self) -> &'a mut crate::W<REG> {
        self.variant(Id1_30_3f::NotFiltered)
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(Id1_30_3f::Selected)
    }
}
#[doc = "Enable or disable ID filtering for IDs 0x40_0x4F.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Id1_40_4f {
    #[doc = "0: Transactions with these IDs are passed on to ATB master port 1."]
    NotFiltered = 0,
    #[doc = "1: Transactions with these IDs are discarded by the replicator."]
    Selected = 1,
}
impl From<Id1_40_4f> for bool {
    #[inline(always)]
    fn from(variant: Id1_40_4f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ID1_40_4F` reader - Enable or disable ID filtering for IDs 0x40_0x4F."]
pub type Id1_40_4fR = crate::BitReader<Id1_40_4f>;
impl Id1_40_4fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Id1_40_4f {
        match self.bits {
            false => Id1_40_4f::NotFiltered,
            true => Id1_40_4f::Selected,
        }
    }
    #[doc = "Transactions with these IDs are passed on to ATB master port 1."]
    #[inline(always)]
    pub fn is_not_filtered(&self) -> bool {
        *self == Id1_40_4f::NotFiltered
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == Id1_40_4f::Selected
    }
}
#[doc = "Field `ID1_40_4F` writer - Enable or disable ID filtering for IDs 0x40_0x4F."]
pub type Id1_40_4fW<'a, REG> = crate::BitWriter<'a, REG, Id1_40_4f>;
impl<'a, REG> Id1_40_4fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transactions with these IDs are passed on to ATB master port 1."]
    #[inline(always)]
    pub fn not_filtered(self) -> &'a mut crate::W<REG> {
        self.variant(Id1_40_4f::NotFiltered)
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(Id1_40_4f::Selected)
    }
}
#[doc = "Enable or disable ID filtering for IDs 0x50_0x5F.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Id1_50_5f {
    #[doc = "0: Transactions with these IDs are passed on to ATB master port 1."]
    NotFiltered = 0,
    #[doc = "1: Transactions with these IDs are discarded by the replicator."]
    Selected = 1,
}
impl From<Id1_50_5f> for bool {
    #[inline(always)]
    fn from(variant: Id1_50_5f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ID1_50_5F` reader - Enable or disable ID filtering for IDs 0x50_0x5F."]
pub type Id1_50_5fR = crate::BitReader<Id1_50_5f>;
impl Id1_50_5fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Id1_50_5f {
        match self.bits {
            false => Id1_50_5f::NotFiltered,
            true => Id1_50_5f::Selected,
        }
    }
    #[doc = "Transactions with these IDs are passed on to ATB master port 1."]
    #[inline(always)]
    pub fn is_not_filtered(&self) -> bool {
        *self == Id1_50_5f::NotFiltered
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == Id1_50_5f::Selected
    }
}
#[doc = "Field `ID1_50_5F` writer - Enable or disable ID filtering for IDs 0x50_0x5F."]
pub type Id1_50_5fW<'a, REG> = crate::BitWriter<'a, REG, Id1_50_5f>;
impl<'a, REG> Id1_50_5fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transactions with these IDs are passed on to ATB master port 1."]
    #[inline(always)]
    pub fn not_filtered(self) -> &'a mut crate::W<REG> {
        self.variant(Id1_50_5f::NotFiltered)
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(Id1_50_5f::Selected)
    }
}
#[doc = "Enable or disable ID filtering for IDs 0x60_0x6F.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Id1_60_6f {
    #[doc = "0: Transactions with these IDs are passed on to ATB master port 1."]
    NotFiltered = 0,
    #[doc = "1: Transactions with these IDs are discarded by the replicator."]
    Selected = 1,
}
impl From<Id1_60_6f> for bool {
    #[inline(always)]
    fn from(variant: Id1_60_6f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ID1_60_6F` reader - Enable or disable ID filtering for IDs 0x60_0x6F."]
pub type Id1_60_6fR = crate::BitReader<Id1_60_6f>;
impl Id1_60_6fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Id1_60_6f {
        match self.bits {
            false => Id1_60_6f::NotFiltered,
            true => Id1_60_6f::Selected,
        }
    }
    #[doc = "Transactions with these IDs are passed on to ATB master port 1."]
    #[inline(always)]
    pub fn is_not_filtered(&self) -> bool {
        *self == Id1_60_6f::NotFiltered
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == Id1_60_6f::Selected
    }
}
#[doc = "Field `ID1_60_6F` writer - Enable or disable ID filtering for IDs 0x60_0x6F."]
pub type Id1_60_6fW<'a, REG> = crate::BitWriter<'a, REG, Id1_60_6f>;
impl<'a, REG> Id1_60_6fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transactions with these IDs are passed on to ATB master port 1."]
    #[inline(always)]
    pub fn not_filtered(self) -> &'a mut crate::W<REG> {
        self.variant(Id1_60_6f::NotFiltered)
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(Id1_60_6f::Selected)
    }
}
#[doc = "Enable or disable ID filtering for IDs 0x70_0x7F.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Id1_70_7f {
    #[doc = "0: Transactions with these IDs are passed on to ATB master port 1."]
    NotFiltered = 0,
    #[doc = "1: Transactions with these IDs are discarded by the replicator."]
    Selected = 1,
}
impl From<Id1_70_7f> for bool {
    #[inline(always)]
    fn from(variant: Id1_70_7f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ID1_70_7F` reader - Enable or disable ID filtering for IDs 0x70_0x7F."]
pub type Id1_70_7fR = crate::BitReader<Id1_70_7f>;
impl Id1_70_7fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Id1_70_7f {
        match self.bits {
            false => Id1_70_7f::NotFiltered,
            true => Id1_70_7f::Selected,
        }
    }
    #[doc = "Transactions with these IDs are passed on to ATB master port 1."]
    #[inline(always)]
    pub fn is_not_filtered(&self) -> bool {
        *self == Id1_70_7f::NotFiltered
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == Id1_70_7f::Selected
    }
}
#[doc = "Field `ID1_70_7F` writer - Enable or disable ID filtering for IDs 0x70_0x7F."]
pub type Id1_70_7fW<'a, REG> = crate::BitWriter<'a, REG, Id1_70_7f>;
impl<'a, REG> Id1_70_7fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transactions with these IDs are passed on to ATB master port 1."]
    #[inline(always)]
    pub fn not_filtered(self) -> &'a mut crate::W<REG> {
        self.variant(Id1_70_7f::NotFiltered)
    }
    #[doc = "Transactions with these IDs are discarded by the replicator."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(Id1_70_7f::Selected)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable ID filtering for IDs 0x00_0x0F."]
    #[inline(always)]
    pub fn id1_00_0f(&self) -> Id1_00_0fR {
        Id1_00_0fR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable ID filtering for IDs 0x10_0x1F."]
    #[inline(always)]
    pub fn id1_10_1f(&self) -> Id1_10_1fR {
        Id1_10_1fR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable ID filtering for IDs 0x20_0x2F."]
    #[inline(always)]
    pub fn id1_20_2f(&self) -> Id1_20_2fR {
        Id1_20_2fR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable ID filtering for IDs 0x30_0x3F."]
    #[inline(always)]
    pub fn id1_30_3f(&self) -> Id1_30_3fR {
        Id1_30_3fR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable ID filtering for IDs 0x40_0x4F."]
    #[inline(always)]
    pub fn id1_40_4f(&self) -> Id1_40_4fR {
        Id1_40_4fR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable or disable ID filtering for IDs 0x50_0x5F."]
    #[inline(always)]
    pub fn id1_50_5f(&self) -> Id1_50_5fR {
        Id1_50_5fR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable or disable ID filtering for IDs 0x60_0x6F."]
    #[inline(always)]
    pub fn id1_60_6f(&self) -> Id1_60_6fR {
        Id1_60_6fR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable or disable ID filtering for IDs 0x70_0x7F."]
    #[inline(always)]
    pub fn id1_70_7f(&self) -> Id1_70_7fR {
        Id1_70_7fR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable ID filtering for IDs 0x00_0x0F."]
    #[inline(always)]
    pub fn id1_00_0f(&mut self) -> Id1_00_0fW<'_, Idfilter1Spec> {
        Id1_00_0fW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable or disable ID filtering for IDs 0x10_0x1F."]
    #[inline(always)]
    pub fn id1_10_1f(&mut self) -> Id1_10_1fW<'_, Idfilter1Spec> {
        Id1_10_1fW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable or disable ID filtering for IDs 0x20_0x2F."]
    #[inline(always)]
    pub fn id1_20_2f(&mut self) -> Id1_20_2fW<'_, Idfilter1Spec> {
        Id1_20_2fW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable or disable ID filtering for IDs 0x30_0x3F."]
    #[inline(always)]
    pub fn id1_30_3f(&mut self) -> Id1_30_3fW<'_, Idfilter1Spec> {
        Id1_30_3fW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable or disable ID filtering for IDs 0x40_0x4F."]
    #[inline(always)]
    pub fn id1_40_4f(&mut self) -> Id1_40_4fW<'_, Idfilter1Spec> {
        Id1_40_4fW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable or disable ID filtering for IDs 0x50_0x5F."]
    #[inline(always)]
    pub fn id1_50_5f(&mut self) -> Id1_50_5fW<'_, Idfilter1Spec> {
        Id1_50_5fW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable or disable ID filtering for IDs 0x60_0x6F."]
    #[inline(always)]
    pub fn id1_60_6f(&mut self) -> Id1_60_6fW<'_, Idfilter1Spec> {
        Id1_60_6fW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable or disable ID filtering for IDs 0x70_0x7F."]
    #[inline(always)]
    pub fn id1_70_7f(&mut self) -> Id1_70_7fW<'_, Idfilter1Spec> {
        Id1_70_7fW::new(self, 7)
    }
}
#[doc = "The IDFILTER1 register enables the programming of ID filtering for master port 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`idfilter1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idfilter1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idfilter1Spec;
impl crate::RegisterSpec for Idfilter1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idfilter1::R`](R) reader structure"]
impl crate::Readable for Idfilter1Spec {}
#[doc = "`write(|w| ..)` method takes [`idfilter1::W`](W) writer structure"]
impl crate::Writable for Idfilter1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDFILTER1 to value 0"]
impl crate::Resettable for Idfilter1Spec {}
