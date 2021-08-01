#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Analog Comparator 0 Control Register"]
    pub acmp_ctl0: crate::Reg<acmp_ctl0::ACMP_CTL0_SPEC>,
    #[doc = "0x04 - Analog Comparator 1 Control Register"]
    pub acmp_ctl1: crate::Reg<acmp_ctl1::ACMP_CTL1_SPEC>,
    #[doc = "0x08 - Analog Comparator Status Register"]
    pub acmp_status: crate::Reg<acmp_status::ACMP_STATUS_SPEC>,
    #[doc = "0x0c - Analog Comparator Reference Voltage Control Register"]
    pub acmp_vref: crate::Reg<acmp_vref::ACMP_VREF_SPEC>,
    #[doc = "0x10 - Analog Comparator Calibration Control Register"]
    pub acmp_calctl: crate::Reg<acmp_calctl::ACMP_CALCTL_SPEC>,
    #[doc = "0x14 - Analog Comparator Calibration Status Register"]
    pub acmp_calsr: crate::Reg<acmp_calsr::ACMP_CALSR_SPEC>,
}
#[doc = "ACMP_CTL0 register accessor: an alias for `Reg<ACMP_CTL0_SPEC>`"]
pub type ACMP_CTL0 = crate::Reg<acmp_ctl0::ACMP_CTL0_SPEC>;
#[doc = "Analog Comparator 0 Control Register"]
pub mod acmp_ctl0;
#[doc = "ACMP_CTL1 register accessor: an alias for `Reg<ACMP_CTL1_SPEC>`"]
pub type ACMP_CTL1 = crate::Reg<acmp_ctl1::ACMP_CTL1_SPEC>;
#[doc = "Analog Comparator 1 Control Register"]
pub mod acmp_ctl1;
#[doc = "ACMP_STATUS register accessor: an alias for `Reg<ACMP_STATUS_SPEC>`"]
pub type ACMP_STATUS = crate::Reg<acmp_status::ACMP_STATUS_SPEC>;
#[doc = "Analog Comparator Status Register"]
pub mod acmp_status;
#[doc = "ACMP_VREF register accessor: an alias for `Reg<ACMP_VREF_SPEC>`"]
pub type ACMP_VREF = crate::Reg<acmp_vref::ACMP_VREF_SPEC>;
#[doc = "Analog Comparator Reference Voltage Control Register"]
pub mod acmp_vref;
#[doc = "ACMP_CALCTL register accessor: an alias for `Reg<ACMP_CALCTL_SPEC>`"]
pub type ACMP_CALCTL = crate::Reg<acmp_calctl::ACMP_CALCTL_SPEC>;
#[doc = "Analog Comparator Calibration Control Register"]
pub mod acmp_calctl;
#[doc = "ACMP_CALSR register accessor: an alias for `Reg<ACMP_CALSR_SPEC>`"]
pub type ACMP_CALSR = crate::Reg<acmp_calsr::ACMP_CALSR_SPEC>;
#[doc = "Analog Comparator Calibration Status Register"]
pub mod acmp_calsr;
