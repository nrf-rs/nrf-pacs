#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    supportedportsizes: Supportedportsizes,
    currentportsize: Currentportsize,
    _reserved2: [u8; 0xf8],
    supportedtriggermodes: Supportedtriggermodes,
    triggercountervalue: Triggercountervalue,
    triggermultiplier: Triggermultiplier,
    _reserved5: [u8; 0xf4],
    suppportedtestpatternmodes: Suppportedtestpatternmodes,
    currenttestpatternmodes: Currenttestpatternmodes,
    tprcr: Tprcr,
    _reserved8: [u8; 0xf4],
    ffsr: Ffsr,
    ffcr: Ffcr,
    fscr: Fscr,
    _reserved11: [u8; 0xf4],
    extctlinport: Extctlinport,
    extctloutport: Extctloutport,
    _reserved13: [u8; 0x0adc],
    ittrflinack: Ittrflinack,
    ittrflin: Ittrflin,
    itatbdata0: Itatbdata0,
    itatbctr2: Itatbctr2,
    itatbctr1: Itatbctr1,
    itatbctr0: Itatbctr0,
    _reserved19: [u8; 0x04],
    itctrl: Itctrl,
    _reserved20: [u8; 0x9c],
    claimset: Claimset,
    claimclr: Claimclr,
    _reserved22: [u8; 0x08],
    lar: Lar,
    lsr: Lsr,
    authstatus: Authstatus,
    _reserved25: [u8; 0x0c],
    devid: Devid,
    devtype: Devtype,
    pidr4: Pidr4,
    _reserved28: [u8; 0x0c],
    pidr_0: Pidr0,
    pidr_1: Pidr1,
    pidr_2: Pidr2,
    pidr_3: Pidr3,
    cidr_0: Cidr0,
    cidr_1: Cidr1,
    cidr_2: Cidr2,
    cidr_3: Cidr3,
}
impl RegisterBlock {
    #[doc = "0x00 - Each bit location is a single port size that is supported on the device."]
    #[inline(always)]
    pub const fn supportedportsizes(&self) -> &Supportedportsizes {
        &self.supportedportsizes
    }
    #[doc = "0x04 - Each bit location is a single port size. One bit can be set, and indicates the current port size."]
    #[inline(always)]
    pub const fn currentportsize(&self) -> &Currentportsize {
        &self.currentportsize
    }
    #[doc = "0x100 - The Supported_trigger_modes register indicates the implemented trigger counter multipliers and other supported features of the trigger system."]
    #[inline(always)]
    pub const fn supportedtriggermodes(&self) -> &Supportedtriggermodes {
        &self.supportedtriggermodes
    }
    #[doc = "0x104 - The Trigger_counter_value register enables delaying the indication of triggers to any external connected trace capture or storage devices."]
    #[inline(always)]
    pub const fn triggercountervalue(&self) -> &Triggercountervalue {
        &self.triggercountervalue
    }
    #[doc = "0x108 - The Trigger_multiplier register contains the selectors for the trigger counter multiplier."]
    #[inline(always)]
    pub const fn triggermultiplier(&self) -> &Triggermultiplier {
        &self.triggermultiplier
    }
    #[doc = "0x200 - The Supported_test_pattern_modes register provides a set of known bit sequences or patterns that can be output over the trace port and can be detected by the TPA or other associated trace capture device."]
    #[inline(always)]
    pub const fn suppportedtestpatternmodes(&self) -> &Suppportedtestpatternmodes {
        &self.suppportedtestpatternmodes
    }
    #[doc = "0x204 - Current_test_pattern_mode indicates the current test pattern or mode selected."]
    #[inline(always)]
    pub const fn currenttestpatternmodes(&self) -> &Currenttestpatternmodes {
        &self.currenttestpatternmodes
    }
    #[doc = "0x208 - The TPRCR register is an 8-bit counter start value that is decremented. A write sets the initial counter value and a read returns the programmed value."]
    #[inline(always)]
    pub const fn tprcr(&self) -> &Tprcr {
        &self.tprcr
    }
    #[doc = "0x300 - The FFSR register indicates the current status of the formatter and flush features available in the TPIU."]
    #[inline(always)]
    pub const fn ffsr(&self) -> &Ffsr {
        &self.ffsr
    }
    #[doc = "0x304 - The FFCR register controls the generation of stop, trigger, and flush events."]
    #[inline(always)]
    pub const fn ffcr(&self) -> &Ffcr {
        &self.ffcr
    }
    #[doc = "0x308 - The FSCR register enables the frequency of synchronization information to be optimized to suit the Trace Port Analyzer (TPA) capture buffer size."]
    #[inline(always)]
    pub const fn fscr(&self) -> &Fscr {
        &self.fscr
    }
    #[doc = "0x400 - Two ports can be used as a control and feedback mechanism for any serializers, pin sharing multiplexers, or other solutions that might be added to the trace output pins either for pin control or a high-speed trace port solution."]
    #[inline(always)]
    pub const fn extctlinport(&self) -> &Extctlinport {
        &self.extctlinport
    }
    #[doc = "0x404 - Two ports can be used as a control and feedback mechanism for any serializers, pin sharing multiplexers, or other solutions that might be added to the trace output pins either for pin control or a high speed trace port solution. These ports are raw register banks that sample or export the corresponding external pins."]
    #[inline(always)]
    pub const fn extctloutport(&self) -> &Extctloutport {
        &self.extctloutport
    }
    #[doc = "0xee4 - The ITTRFLINACK register enables control of the triginack and flushinack outputs from the TPIU."]
    #[inline(always)]
    pub const fn ittrflinack(&self) -> &Ittrflinack {
        &self.ittrflinack
    }
    #[doc = "0xee8 - The ITTRFLIN register contains the values of the flushin and trigin inputs to the TPIU."]
    #[inline(always)]
    pub const fn ittrflin(&self) -> &Ittrflin {
        &self.ittrflin
    }
    #[doc = "0xeec - The ITATBDATA0 register contains the value of the atdatas inputs to the TPIU. The values are valid only when atvalids is HIGH."]
    #[inline(always)]
    pub const fn itatbdata0(&self) -> &Itatbdata0 {
        &self.itatbdata0
    }
    #[doc = "0xef0 - Enables control of the atreadys and afvalids outputs of the TPIU."]
    #[inline(always)]
    pub const fn itatbctr2(&self) -> &Itatbctr2 {
        &self.itatbctr2
    }
    #[doc = "0xef4 - The ITATBCTR1 register contains the value of the atids input to the TPIU. This is only valid when atvalids is HIGH."]
    #[inline(always)]
    pub const fn itatbctr1(&self) -> &Itatbctr1 {
        &self.itatbctr1
    }
    #[doc = "0xef8 - The ITATBCTR0 register captures the values of the atvalids, afreadys, and atbytess inputs to the TPIU. To ensure the integration registers work correctly in a system, the value of atbytess is only valid when atvalids, bit\\[0\\], is HIGH."]
    #[inline(always)]
    pub const fn itatbctr0(&self) -> &Itatbctr0 {
        &self.itatbctr0
    }
    #[doc = "0xf00 - Used to enable topology detection. This register enables the component to switch from a functional mode, the default behavior, to integration mode where the inputs and outputs of the component can be directly controlled for integration testing and topology solving."]
    #[inline(always)]
    pub const fn itctrl(&self) -> &Itctrl {
        &self.itctrl
    }
    #[doc = "0xfa0 - Software can use the claim tag to coordinate application and debugger access to trace unit functionality. The claim tags have no effect on the operation of the component. The CLAIMSET register sets bits in the claim tag, and determines the number of claim bits implemented."]
    #[inline(always)]
    pub const fn claimset(&self) -> &Claimset {
        &self.claimset
    }
    #[doc = "0xfa4 - Software can use the claim tag to coordinate application and debugger access to trace unit functionality. The claim tags have no effect on the operation of the component. The CLAIMCLR register sets the bits in the claim tag to 0 and determines the current value of the claim tag."]
    #[inline(always)]
    pub const fn claimclr(&self) -> &Claimclr {
        &self.claimclr
    }
    #[doc = "0xfb0 - This is used to enable write access to device registers."]
    #[inline(always)]
    pub const fn lar(&self) -> &Lar {
        &self.lar
    }
    #[doc = "0xfb4 - This indicates the status of the lock control mechanism. This lock prevents accidental writes by code under debug. Accesses to the extended stimulus port registers are not affected by the lock mechanism. This register must always be present although there might not be any lock access control mechanism. The lock mechanism, where present and locked, must block write accesses to any control register, except the Lock Access Register. For most components this covers all registers except for the Lock Access Register."]
    #[inline(always)]
    pub const fn lsr(&self) -> &Lsr {
        &self.lsr
    }
    #[doc = "0xfb8 - Indicates the current level of tracing permitted by the system"]
    #[inline(always)]
    pub const fn authstatus(&self) -> &Authstatus {
        &self.authstatus
    }
    #[doc = "0xfc8 - Indicates the capabilities of the component."]
    #[inline(always)]
    pub const fn devid(&self) -> &Devid {
        &self.devid
    }
    #[doc = "0xfcc - The DEVTYPE register provides a debugger with information about the component when the Part Number field is not recognized. The debugger can then report this information."]
    #[inline(always)]
    pub const fn devtype(&self) -> &Devtype {
        &self.devtype
    }
    #[doc = "0xfd0 - Coresight peripheral identification registers."]
    #[inline(always)]
    pub const fn pidr4(&self) -> &Pidr4 {
        &self.pidr4
    }
    #[doc = "0xfe0 - Coresight peripheral identification registers."]
    #[inline(always)]
    pub const fn pidr_0(&self) -> &Pidr0 {
        &self.pidr_0
    }
    #[doc = "0xfe4 - Coresight peripheral identification registers."]
    #[inline(always)]
    pub const fn pidr_1(&self) -> &Pidr1 {
        &self.pidr_1
    }
    #[doc = "0xfe8 - Coresight peripheral identification registers."]
    #[inline(always)]
    pub const fn pidr_2(&self) -> &Pidr2 {
        &self.pidr_2
    }
    #[doc = "0xfec - Coresight peripheral identification registers."]
    #[inline(always)]
    pub const fn pidr_3(&self) -> &Pidr3 {
        &self.pidr_3
    }
    #[doc = "0xff0 - Coresight component identification registers."]
    #[inline(always)]
    pub const fn cidr_0(&self) -> &Cidr0 {
        &self.cidr_0
    }
    #[doc = "0xff4 - Coresight component identification registers."]
    #[inline(always)]
    pub const fn cidr_1(&self) -> &Cidr1 {
        &self.cidr_1
    }
    #[doc = "0xff8 - Coresight component identification registers."]
    #[inline(always)]
    pub const fn cidr_2(&self) -> &Cidr2 {
        &self.cidr_2
    }
    #[doc = "0xffc - Coresight component identification registers."]
    #[inline(always)]
    pub const fn cidr_3(&self) -> &Cidr3 {
        &self.cidr_3
    }
}
#[doc = "SUPPORTEDPORTSIZES (rw) register accessor: Each bit location is a single port size that is supported on the device.\n\nYou can [`read`](crate::Reg::read) this register and get [`supportedportsizes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`supportedportsizes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@supportedportsizes`] module"]
#[doc(alias = "SUPPORTEDPORTSIZES")]
pub type Supportedportsizes = crate::Reg<supportedportsizes::SupportedportsizesSpec>;
#[doc = "Each bit location is a single port size that is supported on the device."]
pub mod supportedportsizes;
#[doc = "CURRENTPORTSIZE (rw) register accessor: Each bit location is a single port size. One bit can be set, and indicates the current port size.\n\nYou can [`read`](crate::Reg::read) this register and get [`currentportsize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`currentportsize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@currentportsize`] module"]
#[doc(alias = "CURRENTPORTSIZE")]
pub type Currentportsize = crate::Reg<currentportsize::CurrentportsizeSpec>;
#[doc = "Each bit location is a single port size. One bit can be set, and indicates the current port size."]
pub mod currentportsize;
#[doc = "SUPPORTEDTRIGGERMODES (rw) register accessor: The Supported_trigger_modes register indicates the implemented trigger counter multipliers and other supported features of the trigger system.\n\nYou can [`read`](crate::Reg::read) this register and get [`supportedtriggermodes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`supportedtriggermodes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@supportedtriggermodes`] module"]
#[doc(alias = "SUPPORTEDTRIGGERMODES")]
pub type Supportedtriggermodes = crate::Reg<supportedtriggermodes::SupportedtriggermodesSpec>;
#[doc = "The Supported_trigger_modes register indicates the implemented trigger counter multipliers and other supported features of the trigger system."]
pub mod supportedtriggermodes;
#[doc = "TRIGGERCOUNTERVALUE (rw) register accessor: The Trigger_counter_value register enables delaying the indication of triggers to any external connected trace capture or storage devices.\n\nYou can [`read`](crate::Reg::read) this register and get [`triggercountervalue::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`triggercountervalue::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@triggercountervalue`] module"]
#[doc(alias = "TRIGGERCOUNTERVALUE")]
pub type Triggercountervalue = crate::Reg<triggercountervalue::TriggercountervalueSpec>;
#[doc = "The Trigger_counter_value register enables delaying the indication of triggers to any external connected trace capture or storage devices."]
pub mod triggercountervalue;
#[doc = "TRIGGERMULTIPLIER (rw) register accessor: The Trigger_multiplier register contains the selectors for the trigger counter multiplier.\n\nYou can [`read`](crate::Reg::read) this register and get [`triggermultiplier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`triggermultiplier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@triggermultiplier`] module"]
#[doc(alias = "TRIGGERMULTIPLIER")]
pub type Triggermultiplier = crate::Reg<triggermultiplier::TriggermultiplierSpec>;
#[doc = "The Trigger_multiplier register contains the selectors for the trigger counter multiplier."]
pub mod triggermultiplier;
#[doc = "SUPPPORTEDTESTPATTERNMODES (rw) register accessor: The Supported_test_pattern_modes register provides a set of known bit sequences or patterns that can be output over the trace port and can be detected by the TPA or other associated trace capture device.\n\nYou can [`read`](crate::Reg::read) this register and get [`suppportedtestpatternmodes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`suppportedtestpatternmodes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@suppportedtestpatternmodes`] module"]
#[doc(alias = "SUPPPORTEDTESTPATTERNMODES")]
pub type Suppportedtestpatternmodes =
    crate::Reg<suppportedtestpatternmodes::SuppportedtestpatternmodesSpec>;
#[doc = "The Supported_test_pattern_modes register provides a set of known bit sequences or patterns that can be output over the trace port and can be detected by the TPA or other associated trace capture device."]
pub mod suppportedtestpatternmodes;
#[doc = "CURRENTTESTPATTERNMODES (rw) register accessor: Current_test_pattern_mode indicates the current test pattern or mode selected.\n\nYou can [`read`](crate::Reg::read) this register and get [`currenttestpatternmodes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`currenttestpatternmodes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@currenttestpatternmodes`] module"]
#[doc(alias = "CURRENTTESTPATTERNMODES")]
pub type Currenttestpatternmodes = crate::Reg<currenttestpatternmodes::CurrenttestpatternmodesSpec>;
#[doc = "Current_test_pattern_mode indicates the current test pattern or mode selected."]
pub mod currenttestpatternmodes;
#[doc = "TPRCR (rw) register accessor: The TPRCR register is an 8-bit counter start value that is decremented. A write sets the initial counter value and a read returns the programmed value.\n\nYou can [`read`](crate::Reg::read) this register and get [`tprcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tprcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tprcr`] module"]
#[doc(alias = "TPRCR")]
pub type Tprcr = crate::Reg<tprcr::TprcrSpec>;
#[doc = "The TPRCR register is an 8-bit counter start value that is decremented. A write sets the initial counter value and a read returns the programmed value."]
pub mod tprcr;
#[doc = "FFSR (rw) register accessor: The FFSR register indicates the current status of the formatter and flush features available in the TPIU.\n\nYou can [`read`](crate::Reg::read) this register and get [`ffsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffsr`] module"]
#[doc(alias = "FFSR")]
pub type Ffsr = crate::Reg<ffsr::FfsrSpec>;
#[doc = "The FFSR register indicates the current status of the formatter and flush features available in the TPIU."]
pub mod ffsr;
#[doc = "FFCR (rw) register accessor: The FFCR register controls the generation of stop, trigger, and flush events.\n\nYou can [`read`](crate::Reg::read) this register and get [`ffcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffcr`] module"]
#[doc(alias = "FFCR")]
pub type Ffcr = crate::Reg<ffcr::FfcrSpec>;
#[doc = "The FFCR register controls the generation of stop, trigger, and flush events."]
pub mod ffcr;
#[doc = "FSCR (rw) register accessor: The FSCR register enables the frequency of synchronization information to be optimized to suit the Trace Port Analyzer (TPA) capture buffer size.\n\nYou can [`read`](crate::Reg::read) this register and get [`fscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fscr`] module"]
#[doc(alias = "FSCR")]
pub type Fscr = crate::Reg<fscr::FscrSpec>;
#[doc = "The FSCR register enables the frequency of synchronization information to be optimized to suit the Trace Port Analyzer (TPA) capture buffer size."]
pub mod fscr;
#[doc = "EXTCTLINPORT (rw) register accessor: Two ports can be used as a control and feedback mechanism for any serializers, pin sharing multiplexers, or other solutions that might be added to the trace output pins either for pin control or a high-speed trace port solution.\n\nYou can [`read`](crate::Reg::read) this register and get [`extctlinport::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extctlinport::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extctlinport`] module"]
#[doc(alias = "EXTCTLINPORT")]
pub type Extctlinport = crate::Reg<extctlinport::ExtctlinportSpec>;
#[doc = "Two ports can be used as a control and feedback mechanism for any serializers, pin sharing multiplexers, or other solutions that might be added to the trace output pins either for pin control or a high-speed trace port solution."]
pub mod extctlinport;
#[doc = "EXTCTLOUTPORT (rw) register accessor: Two ports can be used as a control and feedback mechanism for any serializers, pin sharing multiplexers, or other solutions that might be added to the trace output pins either for pin control or a high speed trace port solution. These ports are raw register banks that sample or export the corresponding external pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`extctloutport::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extctloutport::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extctloutport`] module"]
#[doc(alias = "EXTCTLOUTPORT")]
pub type Extctloutport = crate::Reg<extctloutport::ExtctloutportSpec>;
#[doc = "Two ports can be used as a control and feedback mechanism for any serializers, pin sharing multiplexers, or other solutions that might be added to the trace output pins either for pin control or a high speed trace port solution. These ports are raw register banks that sample or export the corresponding external pins."]
pub mod extctloutport;
#[doc = "ITTRFLINACK (rw) register accessor: The ITTRFLINACK register enables control of the triginack and flushinack outputs from the TPIU.\n\nYou can [`read`](crate::Reg::read) this register and get [`ittrflinack::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ittrflinack::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ittrflinack`] module"]
#[doc(alias = "ITTRFLINACK")]
pub type Ittrflinack = crate::Reg<ittrflinack::IttrflinackSpec>;
#[doc = "The ITTRFLINACK register enables control of the triginack and flushinack outputs from the TPIU."]
pub mod ittrflinack;
#[doc = "ITTRFLIN (rw) register accessor: The ITTRFLIN register contains the values of the flushin and trigin inputs to the TPIU.\n\nYou can [`read`](crate::Reg::read) this register and get [`ittrflin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ittrflin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ittrflin`] module"]
#[doc(alias = "ITTRFLIN")]
pub type Ittrflin = crate::Reg<ittrflin::IttrflinSpec>;
#[doc = "The ITTRFLIN register contains the values of the flushin and trigin inputs to the TPIU."]
pub mod ittrflin;
#[doc = "ITATBDATA0 (rw) register accessor: The ITATBDATA0 register contains the value of the atdatas inputs to the TPIU. The values are valid only when atvalids is HIGH.\n\nYou can [`read`](crate::Reg::read) this register and get [`itatbdata0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itatbdata0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itatbdata0`] module"]
#[doc(alias = "ITATBDATA0")]
pub type Itatbdata0 = crate::Reg<itatbdata0::Itatbdata0Spec>;
#[doc = "The ITATBDATA0 register contains the value of the atdatas inputs to the TPIU. The values are valid only when atvalids is HIGH."]
pub mod itatbdata0;
#[doc = "ITATBCTR2 (rw) register accessor: Enables control of the atreadys and afvalids outputs of the TPIU.\n\nYou can [`read`](crate::Reg::read) this register and get [`itatbctr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itatbctr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itatbctr2`] module"]
#[doc(alias = "ITATBCTR2")]
pub type Itatbctr2 = crate::Reg<itatbctr2::Itatbctr2Spec>;
#[doc = "Enables control of the atreadys and afvalids outputs of the TPIU."]
pub mod itatbctr2;
#[doc = "ITATBCTR1 (rw) register accessor: The ITATBCTR1 register contains the value of the atids input to the TPIU. This is only valid when atvalids is HIGH.\n\nYou can [`read`](crate::Reg::read) this register and get [`itatbctr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itatbctr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itatbctr1`] module"]
#[doc(alias = "ITATBCTR1")]
pub type Itatbctr1 = crate::Reg<itatbctr1::Itatbctr1Spec>;
#[doc = "The ITATBCTR1 register contains the value of the atids input to the TPIU. This is only valid when atvalids is HIGH."]
pub mod itatbctr1;
#[doc = "ITATBCTR0 (rw) register accessor: The ITATBCTR0 register captures the values of the atvalids, afreadys, and atbytess inputs to the TPIU. To ensure the integration registers work correctly in a system, the value of atbytess is only valid when atvalids, bit\\[0\\], is HIGH.\n\nYou can [`read`](crate::Reg::read) this register and get [`itatbctr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itatbctr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itatbctr0`] module"]
#[doc(alias = "ITATBCTR0")]
pub type Itatbctr0 = crate::Reg<itatbctr0::Itatbctr0Spec>;
#[doc = "The ITATBCTR0 register captures the values of the atvalids, afreadys, and atbytess inputs to the TPIU. To ensure the integration registers work correctly in a system, the value of atbytess is only valid when atvalids, bit\\[0\\], is HIGH."]
pub mod itatbctr0;
#[doc = "ITCTRL (rw) register accessor: Used to enable topology detection. This register enables the component to switch from a functional mode, the default behavior, to integration mode where the inputs and outputs of the component can be directly controlled for integration testing and topology solving.\n\nYou can [`read`](crate::Reg::read) this register and get [`itctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itctrl`] module"]
#[doc(alias = "ITCTRL")]
pub type Itctrl = crate::Reg<itctrl::ItctrlSpec>;
#[doc = "Used to enable topology detection. This register enables the component to switch from a functional mode, the default behavior, to integration mode where the inputs and outputs of the component can be directly controlled for integration testing and topology solving."]
pub mod itctrl;
#[doc = "CLAIMSET (rw) register accessor: Software can use the claim tag to coordinate application and debugger access to trace unit functionality. The claim tags have no effect on the operation of the component. The CLAIMSET register sets bits in the claim tag, and determines the number of claim bits implemented.\n\nYou can [`read`](crate::Reg::read) this register and get [`claimset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`claimset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claimset`] module"]
#[doc(alias = "CLAIMSET")]
pub type Claimset = crate::Reg<claimset::ClaimsetSpec>;
#[doc = "Software can use the claim tag to coordinate application and debugger access to trace unit functionality. The claim tags have no effect on the operation of the component. The CLAIMSET register sets bits in the claim tag, and determines the number of claim bits implemented."]
pub mod claimset;
#[doc = "CLAIMCLR (rw) register accessor: Software can use the claim tag to coordinate application and debugger access to trace unit functionality. The claim tags have no effect on the operation of the component. The CLAIMCLR register sets the bits in the claim tag to 0 and determines the current value of the claim tag.\n\nYou can [`read`](crate::Reg::read) this register and get [`claimclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`claimclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claimclr`] module"]
#[doc(alias = "CLAIMCLR")]
pub type Claimclr = crate::Reg<claimclr::ClaimclrSpec>;
#[doc = "Software can use the claim tag to coordinate application and debugger access to trace unit functionality. The claim tags have no effect on the operation of the component. The CLAIMCLR register sets the bits in the claim tag to 0 and determines the current value of the claim tag."]
pub mod claimclr;
#[doc = "LAR (rw) register accessor: This is used to enable write access to device registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lar`] module"]
#[doc(alias = "LAR")]
pub type Lar = crate::Reg<lar::LarSpec>;
#[doc = "This is used to enable write access to device registers."]
pub mod lar;
#[doc = "LSR (rw) register accessor: This indicates the status of the lock control mechanism. This lock prevents accidental writes by code under debug. Accesses to the extended stimulus port registers are not affected by the lock mechanism. This register must always be present although there might not be any lock access control mechanism. The lock mechanism, where present and locked, must block write accesses to any control register, except the Lock Access Register. For most components this covers all registers except for the Lock Access Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsr`] module"]
#[doc(alias = "LSR")]
pub type Lsr = crate::Reg<lsr::LsrSpec>;
#[doc = "This indicates the status of the lock control mechanism. This lock prevents accidental writes by code under debug. Accesses to the extended stimulus port registers are not affected by the lock mechanism. This register must always be present although there might not be any lock access control mechanism. The lock mechanism, where present and locked, must block write accesses to any control register, except the Lock Access Register. For most components this covers all registers except for the Lock Access Register."]
pub mod lsr;
#[doc = "AUTHSTATUS (rw) register accessor: Indicates the current level of tracing permitted by the system\n\nYou can [`read`](crate::Reg::read) this register and get [`authstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`authstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@authstatus`] module"]
#[doc(alias = "AUTHSTATUS")]
pub type Authstatus = crate::Reg<authstatus::AuthstatusSpec>;
#[doc = "Indicates the current level of tracing permitted by the system"]
pub mod authstatus;
#[doc = "DEVID (r) register accessor: Indicates the capabilities of the component.\n\nYou can [`read`](crate::Reg::read) this register and get [`devid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devid`] module"]
#[doc(alias = "DEVID")]
pub type Devid = crate::Reg<devid::DevidSpec>;
#[doc = "Indicates the capabilities of the component."]
pub mod devid;
#[doc = "DEVTYPE (r) register accessor: The DEVTYPE register provides a debugger with information about the component when the Part Number field is not recognized. The debugger can then report this information.\n\nYou can [`read`](crate::Reg::read) this register and get [`devtype::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devtype`] module"]
#[doc(alias = "DEVTYPE")]
pub type Devtype = crate::Reg<devtype::DevtypeSpec>;
#[doc = "The DEVTYPE register provides a debugger with information about the component when the Part Number field is not recognized. The debugger can then report this information."]
pub mod devtype;
#[doc = "PIDR4 (rw) register accessor: Coresight peripheral identification registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr4`] module"]
#[doc(alias = "PIDR4")]
pub type Pidr4 = crate::Reg<pidr4::Pidr4Spec>;
#[doc = "Coresight peripheral identification registers."]
pub mod pidr4;
#[doc = "PIDR_0 (rw) register accessor: Coresight peripheral identification registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr_0`] module"]
#[doc(alias = "PIDR_0")]
pub type Pidr0 = crate::Reg<pidr_0::Pidr0Spec>;
#[doc = "Coresight peripheral identification registers."]
pub mod pidr_0;
#[doc = "PIDR_1 (rw) register accessor: Coresight peripheral identification registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr_1`] module"]
#[doc(alias = "PIDR_1")]
pub type Pidr1 = crate::Reg<pidr_1::Pidr1Spec>;
#[doc = "Coresight peripheral identification registers."]
pub mod pidr_1;
#[doc = "PIDR_2 (rw) register accessor: Coresight peripheral identification registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr_2`] module"]
#[doc(alias = "PIDR_2")]
pub type Pidr2 = crate::Reg<pidr_2::Pidr2Spec>;
#[doc = "Coresight peripheral identification registers."]
pub mod pidr_2;
#[doc = "PIDR_3 (rw) register accessor: Coresight peripheral identification registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr_3`] module"]
#[doc(alias = "PIDR_3")]
pub type Pidr3 = crate::Reg<pidr_3::Pidr3Spec>;
#[doc = "Coresight peripheral identification registers."]
pub mod pidr_3;
#[doc = "CIDR_0 (rw) register accessor: Coresight component identification registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cidr_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr_0`] module"]
#[doc(alias = "CIDR_0")]
pub type Cidr0 = crate::Reg<cidr_0::Cidr0Spec>;
#[doc = "Coresight component identification registers."]
pub mod cidr_0;
#[doc = "CIDR_1 (rw) register accessor: Coresight component identification registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cidr_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr_1`] module"]
#[doc(alias = "CIDR_1")]
pub type Cidr1 = crate::Reg<cidr_1::Cidr1Spec>;
#[doc = "Coresight component identification registers."]
pub mod cidr_1;
#[doc = "CIDR_2 (rw) register accessor: Coresight component identification registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cidr_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr_2`] module"]
#[doc(alias = "CIDR_2")]
pub type Cidr2 = crate::Reg<cidr_2::Cidr2Spec>;
#[doc = "Coresight component identification registers."]
pub mod cidr_2;
#[doc = "CIDR_3 (rw) register accessor: Coresight component identification registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cidr_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr_3`] module"]
#[doc(alias = "CIDR_3")]
pub type Cidr3 = crate::Reg<cidr_3::Cidr3Spec>;
#[doc = "Coresight component identification registers."]
pub mod cidr_3;
