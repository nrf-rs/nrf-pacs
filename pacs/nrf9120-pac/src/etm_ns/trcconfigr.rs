#[doc = "Register `TRCCONFIGR` reader"]
pub type R = crate::R<TrcconfigrSpec>;
#[doc = "Register `TRCCONFIGR` writer"]
pub type W = crate::W<TrcconfigrSpec>;
#[doc = "Instruction P0 load field. This field controls whether load instructions are traced as P0 instructions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Loadasp0inst {
    #[doc = "0: Do not trace load instructions as P0 instructions."]
    No = 0,
    #[doc = "1: Trace load instructions as P0 instructions."]
    Yes = 1,
}
impl From<Loadasp0inst> for bool {
    #[inline(always)]
    fn from(variant: Loadasp0inst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOADASP0INST` reader - Instruction P0 load field. This field controls whether load instructions are traced as P0 instructions."]
pub type Loadasp0instR = crate::BitReader<Loadasp0inst>;
impl Loadasp0instR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Loadasp0inst {
        match self.bits {
            false => Loadasp0inst::No,
            true => Loadasp0inst::Yes,
        }
    }
    #[doc = "Do not trace load instructions as P0 instructions."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Loadasp0inst::No
    }
    #[doc = "Trace load instructions as P0 instructions."]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Loadasp0inst::Yes
    }
}
#[doc = "Field `LOADASP0INST` writer - Instruction P0 load field. This field controls whether load instructions are traced as P0 instructions."]
pub type Loadasp0instW<'a, REG> = crate::BitWriter<'a, REG, Loadasp0inst>;
impl<'a, REG> Loadasp0instW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not trace load instructions as P0 instructions."]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Loadasp0inst::No)
    }
    #[doc = "Trace load instructions as P0 instructions."]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Loadasp0inst::Yes)
    }
}
#[doc = "Instruction P0 field. This field controls whether store instructions are traced as P0 instructions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Storeasp0inst {
    #[doc = "0: Do not trace store instructions as P0 instructions."]
    No = 0,
    #[doc = "1: Trace store instructions as P0 instructions."]
    Yes = 1,
}
impl From<Storeasp0inst> for bool {
    #[inline(always)]
    fn from(variant: Storeasp0inst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOREASP0INST` reader - Instruction P0 field. This field controls whether store instructions are traced as P0 instructions."]
pub type Storeasp0instR = crate::BitReader<Storeasp0inst>;
impl Storeasp0instR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Storeasp0inst {
        match self.bits {
            false => Storeasp0inst::No,
            true => Storeasp0inst::Yes,
        }
    }
    #[doc = "Do not trace store instructions as P0 instructions."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Storeasp0inst::No
    }
    #[doc = "Trace store instructions as P0 instructions."]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Storeasp0inst::Yes
    }
}
#[doc = "Field `STOREASP0INST` writer - Instruction P0 field. This field controls whether store instructions are traced as P0 instructions."]
pub type Storeasp0instW<'a, REG> = crate::BitWriter<'a, REG, Storeasp0inst>;
impl<'a, REG> Storeasp0instW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not trace store instructions as P0 instructions."]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Storeasp0inst::No)
    }
    #[doc = "Trace store instructions as P0 instructions."]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Storeasp0inst::Yes)
    }
}
#[doc = "Branch broadcast mode bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bb {
    #[doc = "0: Branch broadcast mode is disabled."]
    Disabled = 0,
    #[doc = "1: Branch broadcast mode is enabled."]
    Enabled = 1,
}
impl From<Bb> for bool {
    #[inline(always)]
    fn from(variant: Bb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BB` reader - Branch broadcast mode bit."]
pub type BbR = crate::BitReader<Bb>;
impl BbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bb {
        match self.bits {
            false => Bb::Disabled,
            true => Bb::Enabled,
        }
    }
    #[doc = "Branch broadcast mode is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Bb::Disabled
    }
    #[doc = "Branch broadcast mode is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Bb::Enabled
    }
}
#[doc = "Field `BB` writer - Branch broadcast mode bit."]
pub type BbW<'a, REG> = crate::BitWriter<'a, REG, Bb>;
impl<'a, REG> BbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Branch broadcast mode is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Bb::Disabled)
    }
    #[doc = "Branch broadcast mode is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Bb::Enabled)
    }
}
#[doc = "Cycle counting instruction trace bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cci {
    #[doc = "0: Cycle counting in the instruction trace is disabled."]
    Disabled = 0,
    #[doc = "1: Cycle counting in the instruction trace is enabled."]
    Enabled = 1,
}
impl From<Cci> for bool {
    #[inline(always)]
    fn from(variant: Cci) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCI` reader - Cycle counting instruction trace bit."]
pub type CciR = crate::BitReader<Cci>;
impl CciR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cci {
        match self.bits {
            false => Cci::Disabled,
            true => Cci::Enabled,
        }
    }
    #[doc = "Cycle counting in the instruction trace is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cci::Disabled
    }
    #[doc = "Cycle counting in the instruction trace is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cci::Enabled
    }
}
#[doc = "Field `CCI` writer - Cycle counting instruction trace bit."]
pub type CciW<'a, REG> = crate::BitWriter<'a, REG, Cci>;
impl<'a, REG> CciW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cycle counting in the instruction trace is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cci::Disabled)
    }
    #[doc = "Cycle counting in the instruction trace is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cci::Enabled)
    }
}
#[doc = "Context ID tracing bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cid {
    #[doc = "0: Context ID tracing is disabled."]
    Disabled = 0,
    #[doc = "1: Context ID tracing is enabled."]
    Enabled = 1,
}
impl From<Cid> for bool {
    #[inline(always)]
    fn from(variant: Cid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CID` reader - Context ID tracing bit."]
pub type CidR = crate::BitReader<Cid>;
impl CidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cid {
        match self.bits {
            false => Cid::Disabled,
            true => Cid::Enabled,
        }
    }
    #[doc = "Context ID tracing is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cid::Disabled
    }
    #[doc = "Context ID tracing is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cid::Enabled
    }
}
#[doc = "Field `CID` writer - Context ID tracing bit."]
pub type CidW<'a, REG> = crate::BitWriter<'a, REG, Cid>;
impl<'a, REG> CidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Context ID tracing is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cid::Disabled)
    }
    #[doc = "Context ID tracing is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cid::Enabled)
    }
}
#[doc = "Virtual context identifier tracing bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vmid {
    #[doc = "0: Virtual context identifier tracing is disabled."]
    Disabled = 0,
    #[doc = "1: Virtual context identifier tracing is enabled."]
    Enabled = 1,
}
impl From<Vmid> for bool {
    #[inline(always)]
    fn from(variant: Vmid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VMID` reader - Virtual context identifier tracing bit."]
pub type VmidR = crate::BitReader<Vmid>;
impl VmidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vmid {
        match self.bits {
            false => Vmid::Disabled,
            true => Vmid::Enabled,
        }
    }
    #[doc = "Virtual context identifier tracing is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Vmid::Disabled
    }
    #[doc = "Virtual context identifier tracing is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Vmid::Enabled
    }
}
#[doc = "Field `VMID` writer - Virtual context identifier tracing bit."]
pub type VmidW<'a, REG> = crate::BitWriter<'a, REG, Vmid>;
impl<'a, REG> VmidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Virtual context identifier tracing is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Vmid::Disabled)
    }
    #[doc = "Virtual context identifier tracing is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Vmid::Enabled)
    }
}
#[doc = "Conditional instruction tracing bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cond {
    #[doc = "0: Conditional instruction tracing is disabled."]
    Disabled = 0,
    #[doc = "1: Conditional load instructions are traced."]
    LoadOnly = 1,
    #[doc = "2: Conditional store instructions are traced."]
    StoreOnly = 2,
    #[doc = "3: Conditional load and store instructions are traced."]
    LoadAndStore = 3,
    #[doc = "7: All conditional instructions are traced."]
    All = 7,
}
impl From<Cond> for u8 {
    #[inline(always)]
    fn from(variant: Cond) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cond {
    type Ux = u8;
}
impl crate::IsEnum for Cond {}
#[doc = "Field `COND` reader - Conditional instruction tracing bit."]
pub type CondR = crate::FieldReader<Cond>;
impl CondR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cond> {
        match self.bits {
            0 => Some(Cond::Disabled),
            1 => Some(Cond::LoadOnly),
            2 => Some(Cond::StoreOnly),
            3 => Some(Cond::LoadAndStore),
            7 => Some(Cond::All),
            _ => None,
        }
    }
    #[doc = "Conditional instruction tracing is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cond::Disabled
    }
    #[doc = "Conditional load instructions are traced."]
    #[inline(always)]
    pub fn is_load_only(&self) -> bool {
        *self == Cond::LoadOnly
    }
    #[doc = "Conditional store instructions are traced."]
    #[inline(always)]
    pub fn is_store_only(&self) -> bool {
        *self == Cond::StoreOnly
    }
    #[doc = "Conditional load and store instructions are traced."]
    #[inline(always)]
    pub fn is_load_and_store(&self) -> bool {
        *self == Cond::LoadAndStore
    }
    #[doc = "All conditional instructions are traced."]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == Cond::All
    }
}
#[doc = "Field `COND` writer - Conditional instruction tracing bit."]
pub type CondW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cond>;
impl<'a, REG> CondW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Conditional instruction tracing is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cond::Disabled)
    }
    #[doc = "Conditional load instructions are traced."]
    #[inline(always)]
    pub fn load_only(self) -> &'a mut crate::W<REG> {
        self.variant(Cond::LoadOnly)
    }
    #[doc = "Conditional store instructions are traced."]
    #[inline(always)]
    pub fn store_only(self) -> &'a mut crate::W<REG> {
        self.variant(Cond::StoreOnly)
    }
    #[doc = "Conditional load and store instructions are traced."]
    #[inline(always)]
    pub fn load_and_store(self) -> &'a mut crate::W<REG> {
        self.variant(Cond::LoadAndStore)
    }
    #[doc = "All conditional instructions are traced."]
    #[inline(always)]
    pub fn all(self) -> &'a mut crate::W<REG> {
        self.variant(Cond::All)
    }
}
#[doc = "Global timestamp tracing bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ts {
    #[doc = "0: Global timestamp tracing is disabled."]
    Disabled = 0,
    #[doc = "1: Global timestamp tracing is enabled."]
    Enabled = 1,
}
impl From<Ts> for bool {
    #[inline(always)]
    fn from(variant: Ts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS` reader - Global timestamp tracing bit."]
pub type TsR = crate::BitReader<Ts>;
impl TsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ts {
        match self.bits {
            false => Ts::Disabled,
            true => Ts::Enabled,
        }
    }
    #[doc = "Global timestamp tracing is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ts::Disabled
    }
    #[doc = "Global timestamp tracing is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ts::Enabled
    }
}
#[doc = "Field `TS` writer - Global timestamp tracing bit."]
pub type TsW<'a, REG> = crate::BitWriter<'a, REG, Ts>;
impl<'a, REG> TsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Global timestamp tracing is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ts::Disabled)
    }
    #[doc = "Global timestamp tracing is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ts::Enabled)
    }
}
#[doc = "Return stack enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rs {
    #[doc = "0: Return stack is disabled."]
    Disabled = 0,
    #[doc = "1: Return stack is enabled."]
    Enabled = 1,
}
impl From<Rs> for bool {
    #[inline(always)]
    fn from(variant: Rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RS` reader - Return stack enable bit."]
pub type RsR = crate::BitReader<Rs>;
impl RsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rs {
        match self.bits {
            false => Rs::Disabled,
            true => Rs::Enabled,
        }
    }
    #[doc = "Return stack is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rs::Disabled
    }
    #[doc = "Return stack is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rs::Enabled
    }
}
#[doc = "Field `RS` writer - Return stack enable bit."]
pub type RsW<'a, REG> = crate::BitWriter<'a, REG, Rs>;
impl<'a, REG> RsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Return stack is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rs::Disabled)
    }
    #[doc = "Return stack is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rs::Enabled)
    }
}
#[doc = "Q element enable field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Qe {
    #[doc = "0: Q elements are disabled."]
    Disabled = 0,
    #[doc = "1: Q elements with instruction counts are enabled. Q elements without instruction counts are disabled."]
    OnlyWithoutInstCounts = 1,
    #[doc = "3: Q elements with and without instruction counts are enabled."]
    Enabled = 3,
}
impl From<Qe> for u8 {
    #[inline(always)]
    fn from(variant: Qe) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Qe {
    type Ux = u8;
}
impl crate::IsEnum for Qe {}
#[doc = "Field `QE` reader - Q element enable field."]
pub type QeR = crate::FieldReader<Qe>;
impl QeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Qe> {
        match self.bits {
            0 => Some(Qe::Disabled),
            1 => Some(Qe::OnlyWithoutInstCounts),
            3 => Some(Qe::Enabled),
            _ => None,
        }
    }
    #[doc = "Q elements are disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Qe::Disabled
    }
    #[doc = "Q elements with instruction counts are enabled. Q elements without instruction counts are disabled."]
    #[inline(always)]
    pub fn is_only_without_inst_counts(&self) -> bool {
        *self == Qe::OnlyWithoutInstCounts
    }
    #[doc = "Q elements with and without instruction counts are enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Qe::Enabled
    }
}
#[doc = "Field `QE` writer - Q element enable field."]
pub type QeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Qe>;
impl<'a, REG> QeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Q elements are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Qe::Disabled)
    }
    #[doc = "Q elements with instruction counts are enabled. Q elements without instruction counts are disabled."]
    #[inline(always)]
    pub fn only_without_inst_counts(self) -> &'a mut crate::W<REG> {
        self.variant(Qe::OnlyWithoutInstCounts)
    }
    #[doc = "Q elements with and without instruction counts are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Qe::Enabled)
    }
}
#[doc = "Control bit to select the Virtual context identifier value used by the trace unit, both for trace generation and in the Virtual context identifier comparators.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vmidopt {
    #[doc = "0: VTTBR_EL2.VMID is used. If the trace unit supports a Virtual context identifier larger than the VTTBR_EL2.VMID, the upper unused bits are always zero. If the trace unit supports a Virtual context identifier larger than 8 bits and if the VTCR_EL2.VS bit forces use of an 8-bit Virtual context identifier, bits \\[15:8\\] of the trace unit Virtual context identifier are always zero."]
    VttbrEl2 = 0,
    #[doc = "1: CONTEXTIDR_EL2 is used."]
    ContextidrEl2 = 1,
}
impl From<Vmidopt> for bool {
    #[inline(always)]
    fn from(variant: Vmidopt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VMIDOPT` reader - Control bit to select the Virtual context identifier value used by the trace unit, both for trace generation and in the Virtual context identifier comparators."]
pub type VmidoptR = crate::BitReader<Vmidopt>;
impl VmidoptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vmidopt {
        match self.bits {
            false => Vmidopt::VttbrEl2,
            true => Vmidopt::ContextidrEl2,
        }
    }
    #[doc = "VTTBR_EL2.VMID is used. If the trace unit supports a Virtual context identifier larger than the VTTBR_EL2.VMID, the upper unused bits are always zero. If the trace unit supports a Virtual context identifier larger than 8 bits and if the VTCR_EL2.VS bit forces use of an 8-bit Virtual context identifier, bits \\[15:8\\] of the trace unit Virtual context identifier are always zero."]
    #[inline(always)]
    pub fn is_vttbr_el2(&self) -> bool {
        *self == Vmidopt::VttbrEl2
    }
    #[doc = "CONTEXTIDR_EL2 is used."]
    #[inline(always)]
    pub fn is_contextidr_el2(&self) -> bool {
        *self == Vmidopt::ContextidrEl2
    }
}
#[doc = "Field `VMIDOPT` writer - Control bit to select the Virtual context identifier value used by the trace unit, both for trace generation and in the Virtual context identifier comparators."]
pub type VmidoptW<'a, REG> = crate::BitWriter<'a, REG, Vmidopt>;
impl<'a, REG> VmidoptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VTTBR_EL2.VMID is used. If the trace unit supports a Virtual context identifier larger than the VTTBR_EL2.VMID, the upper unused bits are always zero. If the trace unit supports a Virtual context identifier larger than 8 bits and if the VTCR_EL2.VS bit forces use of an 8-bit Virtual context identifier, bits \\[15:8\\] of the trace unit Virtual context identifier are always zero."]
    #[inline(always)]
    pub fn vttbr_el2(self) -> &'a mut crate::W<REG> {
        self.variant(Vmidopt::VttbrEl2)
    }
    #[doc = "CONTEXTIDR_EL2 is used."]
    #[inline(always)]
    pub fn contextidr_el2(self) -> &'a mut crate::W<REG> {
        self.variant(Vmidopt::ContextidrEl2)
    }
}
#[doc = "Data address tracing bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Da {
    #[doc = "0: Data address tracing is disabled."]
    Disabled = 0,
    #[doc = "1: Data address tracing is enabled."]
    Enabled = 1,
}
impl From<Da> for bool {
    #[inline(always)]
    fn from(variant: Da) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DA` reader - Data address tracing bit."]
pub type DaR = crate::BitReader<Da>;
impl DaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Da {
        match self.bits {
            false => Da::Disabled,
            true => Da::Enabled,
        }
    }
    #[doc = "Data address tracing is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Da::Disabled
    }
    #[doc = "Data address tracing is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Da::Enabled
    }
}
#[doc = "Field `DA` writer - Data address tracing bit."]
pub type DaW<'a, REG> = crate::BitWriter<'a, REG, Da>;
impl<'a, REG> DaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data address tracing is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Da::Disabled)
    }
    #[doc = "Data address tracing is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Da::Enabled)
    }
}
#[doc = "Data value tracing bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dv {
    #[doc = "0: Data value tracing is disabled."]
    Disabled = 0,
    #[doc = "1: Data value tracing is enabled."]
    Enabled = 1,
}
impl From<Dv> for bool {
    #[inline(always)]
    fn from(variant: Dv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DV` reader - Data value tracing bit."]
pub type DvR = crate::BitReader<Dv>;
impl DvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dv {
        match self.bits {
            false => Dv::Disabled,
            true => Dv::Enabled,
        }
    }
    #[doc = "Data value tracing is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dv::Disabled
    }
    #[doc = "Data value tracing is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dv::Enabled
    }
}
#[doc = "Field `DV` writer - Data value tracing bit."]
pub type DvW<'a, REG> = crate::BitWriter<'a, REG, Dv>;
impl<'a, REG> DvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data value tracing is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dv::Disabled)
    }
    #[doc = "Data value tracing is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dv::Enabled)
    }
}
impl R {
    #[doc = "Bit 1 - Instruction P0 load field. This field controls whether load instructions are traced as P0 instructions."]
    #[inline(always)]
    pub fn loadasp0inst(&self) -> Loadasp0instR {
        Loadasp0instR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Instruction P0 field. This field controls whether store instructions are traced as P0 instructions."]
    #[inline(always)]
    pub fn storeasp0inst(&self) -> Storeasp0instR {
        Storeasp0instR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Branch broadcast mode bit."]
    #[inline(always)]
    pub fn bb(&self) -> BbR {
        BbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Cycle counting instruction trace bit."]
    #[inline(always)]
    pub fn cci(&self) -> CciR {
        CciR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Context ID tracing bit."]
    #[inline(always)]
    pub fn cid(&self) -> CidR {
        CidR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Virtual context identifier tracing bit."]
    #[inline(always)]
    pub fn vmid(&self) -> VmidR {
        VmidR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Conditional instruction tracing bit."]
    #[inline(always)]
    pub fn cond(&self) -> CondR {
        CondR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Global timestamp tracing bit."]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Return stack enable bit."]
    #[inline(always)]
    pub fn rs(&self) -> RsR {
        RsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Q element enable field."]
    #[inline(always)]
    pub fn qe(&self) -> QeR {
        QeR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Control bit to select the Virtual context identifier value used by the trace unit, both for trace generation and in the Virtual context identifier comparators."]
    #[inline(always)]
    pub fn vmidopt(&self) -> VmidoptR {
        VmidoptR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Data address tracing bit."]
    #[inline(always)]
    pub fn da(&self) -> DaR {
        DaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Data value tracing bit."]
    #[inline(always)]
    pub fn dv(&self) -> DvR {
        DvR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Instruction P0 load field. This field controls whether load instructions are traced as P0 instructions."]
    #[inline(always)]
    pub fn loadasp0inst(&mut self) -> Loadasp0instW<'_, TrcconfigrSpec> {
        Loadasp0instW::new(self, 1)
    }
    #[doc = "Bit 2 - Instruction P0 field. This field controls whether store instructions are traced as P0 instructions."]
    #[inline(always)]
    pub fn storeasp0inst(&mut self) -> Storeasp0instW<'_, TrcconfigrSpec> {
        Storeasp0instW::new(self, 2)
    }
    #[doc = "Bit 3 - Branch broadcast mode bit."]
    #[inline(always)]
    pub fn bb(&mut self) -> BbW<'_, TrcconfigrSpec> {
        BbW::new(self, 3)
    }
    #[doc = "Bit 4 - Cycle counting instruction trace bit."]
    #[inline(always)]
    pub fn cci(&mut self) -> CciW<'_, TrcconfigrSpec> {
        CciW::new(self, 4)
    }
    #[doc = "Bit 6 - Context ID tracing bit."]
    #[inline(always)]
    pub fn cid(&mut self) -> CidW<'_, TrcconfigrSpec> {
        CidW::new(self, 6)
    }
    #[doc = "Bit 7 - Virtual context identifier tracing bit."]
    #[inline(always)]
    pub fn vmid(&mut self) -> VmidW<'_, TrcconfigrSpec> {
        VmidW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Conditional instruction tracing bit."]
    #[inline(always)]
    pub fn cond(&mut self) -> CondW<'_, TrcconfigrSpec> {
        CondW::new(self, 8)
    }
    #[doc = "Bit 11 - Global timestamp tracing bit."]
    #[inline(always)]
    pub fn ts(&mut self) -> TsW<'_, TrcconfigrSpec> {
        TsW::new(self, 11)
    }
    #[doc = "Bit 12 - Return stack enable bit."]
    #[inline(always)]
    pub fn rs(&mut self) -> RsW<'_, TrcconfigrSpec> {
        RsW::new(self, 12)
    }
    #[doc = "Bits 13:14 - Q element enable field."]
    #[inline(always)]
    pub fn qe(&mut self) -> QeW<'_, TrcconfigrSpec> {
        QeW::new(self, 13)
    }
    #[doc = "Bit 15 - Control bit to select the Virtual context identifier value used by the trace unit, both for trace generation and in the Virtual context identifier comparators."]
    #[inline(always)]
    pub fn vmidopt(&mut self) -> VmidoptW<'_, TrcconfigrSpec> {
        VmidoptW::new(self, 15)
    }
    #[doc = "Bit 16 - Data address tracing bit."]
    #[inline(always)]
    pub fn da(&mut self) -> DaW<'_, TrcconfigrSpec> {
        DaW::new(self, 16)
    }
    #[doc = "Bit 17 - Data value tracing bit."]
    #[inline(always)]
    pub fn dv(&mut self) -> DvW<'_, TrcconfigrSpec> {
        DvW::new(self, 17)
    }
}
#[doc = "Controls the tracing options This register must always be programmed as part of trace unit initialization. Might ignore writes when the trace unit is enabled or not idle.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcconfigr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcconfigr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcconfigrSpec;
impl crate::RegisterSpec for TrcconfigrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcconfigr::R`](R) reader structure"]
impl crate::Readable for TrcconfigrSpec {}
#[doc = "`write(|w| ..)` method takes [`trcconfigr::W`](W) writer structure"]
impl crate::Writable for TrcconfigrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCCONFIGR to value 0"]
impl crate::Resettable for TrcconfigrSpec {}
