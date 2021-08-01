#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Data Register 0"]
    pub adc_addr0: crate::Reg<adc_addr0::ADC_ADDR0_SPEC>,
    #[doc = "0x04 - ADC Data Register 1"]
    pub adc_addr1: crate::Reg<adc_addr1::ADC_ADDR1_SPEC>,
    #[doc = "0x08 - ADC Data Register 2"]
    pub adc_addr2: crate::Reg<adc_addr2::ADC_ADDR2_SPEC>,
    #[doc = "0x0c - ADC Data Register 3"]
    pub adc_addr3: crate::Reg<adc_addr3::ADC_ADDR3_SPEC>,
    #[doc = "0x10 - ADC Data Register 4"]
    pub adc_addr4: crate::Reg<adc_addr4::ADC_ADDR4_SPEC>,
    #[doc = "0x14 - ADC Data Register 5"]
    pub adc_addr5: crate::Reg<adc_addr5::ADC_ADDR5_SPEC>,
    #[doc = "0x18 - ADC Data Register 6"]
    pub adc_addr6: crate::Reg<adc_addr6::ADC_ADDR6_SPEC>,
    #[doc = "0x1c - ADC Data Register 7"]
    pub adc_addr7: crate::Reg<adc_addr7::ADC_ADDR7_SPEC>,
    #[doc = "0x20 - ADC Data Register 8"]
    pub adc_addr8: crate::Reg<adc_addr8::ADC_ADDR8_SPEC>,
    #[doc = "0x24 - ADC Data Register 9"]
    pub adc_addr9: crate::Reg<adc_addr9::ADC_ADDR9_SPEC>,
    #[doc = "0x28 - ADC Data Register 10"]
    pub adc_addr10: crate::Reg<adc_addr10::ADC_ADDR10_SPEC>,
    #[doc = "0x2c - ADC Data Register 11"]
    pub adc_addr11: crate::Reg<adc_addr11::ADC_ADDR11_SPEC>,
    #[doc = "0x30 - ADC Data Register 12"]
    pub adc_addr12: crate::Reg<adc_addr12::ADC_ADDR12_SPEC>,
    #[doc = "0x34 - ADC Data Register 13"]
    pub adc_addr13: crate::Reg<adc_addr13::ADC_ADDR13_SPEC>,
    #[doc = "0x38 - ADC Data Register 14"]
    pub adc_addr14: crate::Reg<adc_addr14::ADC_ADDR14_SPEC>,
    #[doc = "0x3c - ADC Data Register 15"]
    pub adc_addr15: crate::Reg<adc_addr15::ADC_ADDR15_SPEC>,
    _reserved16: [u8; 0x34],
    #[doc = "0x74 - ADC Data Register 29"]
    pub adc_addr29: crate::Reg<adc_addr29::ADC_ADDR29_SPEC>,
    _reserved17: [u8; 0x08],
    #[doc = "0x80 - ADC Control Register"]
    pub adc_adcr: crate::Reg<adc_adcr::ADC_ADCR_SPEC>,
    #[doc = "0x84 - ADC Channel Enable Register"]
    pub adc_adcher: crate::Reg<adc_adcher::ADC_ADCHER_SPEC>,
    #[doc = "0x88 - ADC Compare Register 0"]
    pub adc_adcmpr0: crate::Reg<adc_adcmpr0::ADC_ADCMPR0_SPEC>,
    #[doc = "0x8c - ADC Compare Register 1"]
    pub adc_adcmpr1: crate::Reg<adc_adcmpr1::ADC_ADCMPR1_SPEC>,
    #[doc = "0x90 - ADC Status Register0"]
    pub adc_adsr0: crate::Reg<adc_adsr0::ADC_ADSR0_SPEC>,
    #[doc = "0x94 - ADC Status Register1"]
    pub adc_adsr1: crate::Reg<adc_adsr1::ADC_ADSR1_SPEC>,
    #[doc = "0x98 - ADC Status Register2"]
    pub adc_adsr2: crate::Reg<adc_adsr2::ADC_ADSR2_SPEC>,
    _reserved24: [u8; 0x04],
    #[doc = "0xa0 - ADC Extend Sample Time Control Register"]
    pub adc_esmpctl: crate::Reg<adc_esmpctl::ADC_ESMPCTL_SPEC>,
    #[doc = "0xa4 - ADC Channel Floating Detect Control Register"]
    pub adc_cfdctl: crate::Reg<adc_cfdctl::ADC_CFDCTL_SPEC>,
    _reserved26: [u8; 0x58],
    #[doc = "0x100 - ADC PDMA Current Transfer Data Register"]
    pub adc_adpdma: crate::Reg<adc_adpdma::ADC_ADPDMA_SPEC>,
    _reserved27: [u8; 0x7c],
    #[doc = "0x180 - ADC Calibration Mode Register"]
    pub adc_adcalr: crate::Reg<adc_adcalr::ADC_ADCALR_SPEC>,
    #[doc = "0x184 - ADC Calibration Status Register"]
    pub adc_adcalstsr: crate::Reg<adc_adcalstsr::ADC_ADCALSTSR_SPEC>,
}
#[doc = "ADC_ADDR0 register accessor: an alias for `Reg<ADC_ADDR0_SPEC>`"]
pub type ADC_ADDR0 = crate::Reg<adc_addr0::ADC_ADDR0_SPEC>;
#[doc = "ADC Data Register 0"]
pub mod adc_addr0;
#[doc = "ADC_ADDR1 register accessor: an alias for `Reg<ADC_ADDR1_SPEC>`"]
pub type ADC_ADDR1 = crate::Reg<adc_addr1::ADC_ADDR1_SPEC>;
#[doc = "ADC Data Register 1"]
pub mod adc_addr1;
#[doc = "ADC_ADDR2 register accessor: an alias for `Reg<ADC_ADDR2_SPEC>`"]
pub type ADC_ADDR2 = crate::Reg<adc_addr2::ADC_ADDR2_SPEC>;
#[doc = "ADC Data Register 2"]
pub mod adc_addr2;
#[doc = "ADC_ADDR3 register accessor: an alias for `Reg<ADC_ADDR3_SPEC>`"]
pub type ADC_ADDR3 = crate::Reg<adc_addr3::ADC_ADDR3_SPEC>;
#[doc = "ADC Data Register 3"]
pub mod adc_addr3;
#[doc = "ADC_ADDR4 register accessor: an alias for `Reg<ADC_ADDR4_SPEC>`"]
pub type ADC_ADDR4 = crate::Reg<adc_addr4::ADC_ADDR4_SPEC>;
#[doc = "ADC Data Register 4"]
pub mod adc_addr4;
#[doc = "ADC_ADDR5 register accessor: an alias for `Reg<ADC_ADDR5_SPEC>`"]
pub type ADC_ADDR5 = crate::Reg<adc_addr5::ADC_ADDR5_SPEC>;
#[doc = "ADC Data Register 5"]
pub mod adc_addr5;
#[doc = "ADC_ADDR6 register accessor: an alias for `Reg<ADC_ADDR6_SPEC>`"]
pub type ADC_ADDR6 = crate::Reg<adc_addr6::ADC_ADDR6_SPEC>;
#[doc = "ADC Data Register 6"]
pub mod adc_addr6;
#[doc = "ADC_ADDR7 register accessor: an alias for `Reg<ADC_ADDR7_SPEC>`"]
pub type ADC_ADDR7 = crate::Reg<adc_addr7::ADC_ADDR7_SPEC>;
#[doc = "ADC Data Register 7"]
pub mod adc_addr7;
#[doc = "ADC_ADDR8 register accessor: an alias for `Reg<ADC_ADDR8_SPEC>`"]
pub type ADC_ADDR8 = crate::Reg<adc_addr8::ADC_ADDR8_SPEC>;
#[doc = "ADC Data Register 8"]
pub mod adc_addr8;
#[doc = "ADC_ADDR9 register accessor: an alias for `Reg<ADC_ADDR9_SPEC>`"]
pub type ADC_ADDR9 = crate::Reg<adc_addr9::ADC_ADDR9_SPEC>;
#[doc = "ADC Data Register 9"]
pub mod adc_addr9;
#[doc = "ADC_ADDR10 register accessor: an alias for `Reg<ADC_ADDR10_SPEC>`"]
pub type ADC_ADDR10 = crate::Reg<adc_addr10::ADC_ADDR10_SPEC>;
#[doc = "ADC Data Register 10"]
pub mod adc_addr10;
#[doc = "ADC_ADDR11 register accessor: an alias for `Reg<ADC_ADDR11_SPEC>`"]
pub type ADC_ADDR11 = crate::Reg<adc_addr11::ADC_ADDR11_SPEC>;
#[doc = "ADC Data Register 11"]
pub mod adc_addr11;
#[doc = "ADC_ADDR12 register accessor: an alias for `Reg<ADC_ADDR12_SPEC>`"]
pub type ADC_ADDR12 = crate::Reg<adc_addr12::ADC_ADDR12_SPEC>;
#[doc = "ADC Data Register 12"]
pub mod adc_addr12;
#[doc = "ADC_ADDR13 register accessor: an alias for `Reg<ADC_ADDR13_SPEC>`"]
pub type ADC_ADDR13 = crate::Reg<adc_addr13::ADC_ADDR13_SPEC>;
#[doc = "ADC Data Register 13"]
pub mod adc_addr13;
#[doc = "ADC_ADDR14 register accessor: an alias for `Reg<ADC_ADDR14_SPEC>`"]
pub type ADC_ADDR14 = crate::Reg<adc_addr14::ADC_ADDR14_SPEC>;
#[doc = "ADC Data Register 14"]
pub mod adc_addr14;
#[doc = "ADC_ADDR15 register accessor: an alias for `Reg<ADC_ADDR15_SPEC>`"]
pub type ADC_ADDR15 = crate::Reg<adc_addr15::ADC_ADDR15_SPEC>;
#[doc = "ADC Data Register 15"]
pub mod adc_addr15;
#[doc = "ADC_ADDR29 register accessor: an alias for `Reg<ADC_ADDR29_SPEC>`"]
pub type ADC_ADDR29 = crate::Reg<adc_addr29::ADC_ADDR29_SPEC>;
#[doc = "ADC Data Register 29"]
pub mod adc_addr29;
#[doc = "ADC_ADCR register accessor: an alias for `Reg<ADC_ADCR_SPEC>`"]
pub type ADC_ADCR = crate::Reg<adc_adcr::ADC_ADCR_SPEC>;
#[doc = "ADC Control Register"]
pub mod adc_adcr;
#[doc = "ADC_ADCHER register accessor: an alias for `Reg<ADC_ADCHER_SPEC>`"]
pub type ADC_ADCHER = crate::Reg<adc_adcher::ADC_ADCHER_SPEC>;
#[doc = "ADC Channel Enable Register"]
pub mod adc_adcher;
#[doc = "ADC_ADCMPR0 register accessor: an alias for `Reg<ADC_ADCMPR0_SPEC>`"]
pub type ADC_ADCMPR0 = crate::Reg<adc_adcmpr0::ADC_ADCMPR0_SPEC>;
#[doc = "ADC Compare Register 0"]
pub mod adc_adcmpr0;
#[doc = "ADC_ADCMPR1 register accessor: an alias for `Reg<ADC_ADCMPR1_SPEC>`"]
pub type ADC_ADCMPR1 = crate::Reg<adc_adcmpr1::ADC_ADCMPR1_SPEC>;
#[doc = "ADC Compare Register 1"]
pub mod adc_adcmpr1;
#[doc = "ADC_ADSR0 register accessor: an alias for `Reg<ADC_ADSR0_SPEC>`"]
pub type ADC_ADSR0 = crate::Reg<adc_adsr0::ADC_ADSR0_SPEC>;
#[doc = "ADC Status Register0"]
pub mod adc_adsr0;
#[doc = "ADC_ADSR1 register accessor: an alias for `Reg<ADC_ADSR1_SPEC>`"]
pub type ADC_ADSR1 = crate::Reg<adc_adsr1::ADC_ADSR1_SPEC>;
#[doc = "ADC Status Register1"]
pub mod adc_adsr1;
#[doc = "ADC_ADSR2 register accessor: an alias for `Reg<ADC_ADSR2_SPEC>`"]
pub type ADC_ADSR2 = crate::Reg<adc_adsr2::ADC_ADSR2_SPEC>;
#[doc = "ADC Status Register2"]
pub mod adc_adsr2;
#[doc = "ADC_ESMPCTL register accessor: an alias for `Reg<ADC_ESMPCTL_SPEC>`"]
pub type ADC_ESMPCTL = crate::Reg<adc_esmpctl::ADC_ESMPCTL_SPEC>;
#[doc = "ADC Extend Sample Time Control Register"]
pub mod adc_esmpctl;
#[doc = "ADC_CFDCTL register accessor: an alias for `Reg<ADC_CFDCTL_SPEC>`"]
pub type ADC_CFDCTL = crate::Reg<adc_cfdctl::ADC_CFDCTL_SPEC>;
#[doc = "ADC Channel Floating Detect Control Register"]
pub mod adc_cfdctl;
#[doc = "ADC_ADPDMA register accessor: an alias for `Reg<ADC_ADPDMA_SPEC>`"]
pub type ADC_ADPDMA = crate::Reg<adc_adpdma::ADC_ADPDMA_SPEC>;
#[doc = "ADC PDMA Current Transfer Data Register"]
pub mod adc_adpdma;
#[doc = "ADC_ADCALR register accessor: an alias for `Reg<ADC_ADCALR_SPEC>`"]
pub type ADC_ADCALR = crate::Reg<adc_adcalr::ADC_ADCALR_SPEC>;
#[doc = "ADC Calibration Mode Register"]
pub mod adc_adcalr;
#[doc = "ADC_ADCALSTSR register accessor: an alias for `Reg<ADC_ADCALSTSR_SPEC>`"]
pub type ADC_ADCALSTSR = crate::Reg<adc_adcalstsr::ADC_ADCALSTSR_SPEC>;
#[doc = "ADC Calibration Status Register"]
pub mod adc_adcalstsr;
