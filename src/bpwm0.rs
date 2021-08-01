#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - BPWM Control Register 0"]
    pub bpwm_ctl0: crate::Reg<bpwm_ctl0::BPWM_CTL0_SPEC>,
    #[doc = "0x04 - BPWM Control Register 1"]
    pub bpwm_ctl1: crate::Reg<bpwm_ctl1::BPWM_CTL1_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - BPWM Clock Source Register"]
    pub bpwm_clksrc: crate::Reg<bpwm_clksrc::BPWM_CLKSRC_SPEC>,
    #[doc = "0x14 - BPWM Clock Prescale Register"]
    pub bpwm_clkpsc: crate::Reg<bpwm_clkpsc::BPWM_CLKPSC_SPEC>,
    _reserved4: [u8; 0x08],
    #[doc = "0x20 - BPWM Counter Enable Register"]
    pub bpwm_cnten: crate::Reg<bpwm_cnten::BPWM_CNTEN_SPEC>,
    #[doc = "0x24 - BPWM Clear Counter Register"]
    pub bpwm_cntclr: crate::Reg<bpwm_cntclr::BPWM_CNTCLR_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x30 - BPWM Period Register"]
    pub bpwm_period: crate::Reg<bpwm_period::BPWM_PERIOD_SPEC>,
    _reserved7: [u8; 0x1c],
    #[doc = "0x50 - BPWM Comparator Register 0"]
    pub bpwm_cmpdat0: crate::Reg<bpwm_cmpdat0::BPWM_CMPDAT0_SPEC>,
    #[doc = "0x54 - BPWM Comparator Register 1"]
    pub bpwm_cmpdat1: crate::Reg<bpwm_cmpdat1::BPWM_CMPDAT1_SPEC>,
    #[doc = "0x58 - BPWM Comparator Register 2"]
    pub bpwm_cmpdat2: crate::Reg<bpwm_cmpdat2::BPWM_CMPDAT2_SPEC>,
    #[doc = "0x5c - BPWM Comparator Register 3"]
    pub bpwm_cmpdat3: crate::Reg<bpwm_cmpdat3::BPWM_CMPDAT3_SPEC>,
    #[doc = "0x60 - BPWM Comparator Register 4"]
    pub bpwm_cmpdat4: crate::Reg<bpwm_cmpdat4::BPWM_CMPDAT4_SPEC>,
    #[doc = "0x64 - BPWM Comparator Register 5"]
    pub bpwm_cmpdat5: crate::Reg<bpwm_cmpdat5::BPWM_CMPDAT5_SPEC>,
    _reserved13: [u8; 0x28],
    #[doc = "0x90 - BPWM Counter Register"]
    pub bpwm_cnt: crate::Reg<bpwm_cnt::BPWM_CNT_SPEC>,
    _reserved14: [u8; 0x1c],
    #[doc = "0xb0 - BPWM Generation Register 0"]
    pub bpwm_wgctl0: crate::Reg<bpwm_wgctl0::BPWM_WGCTL0_SPEC>,
    #[doc = "0xb4 - BPWM Generation Register 1"]
    pub bpwm_wgctl1: crate::Reg<bpwm_wgctl1::BPWM_WGCTL1_SPEC>,
    #[doc = "0xb8 - BPWM Mask Enable Register"]
    pub bpwm_msken: crate::Reg<bpwm_msken::BPWM_MSKEN_SPEC>,
    #[doc = "0xbc - BPWM Mask Data Register"]
    pub bpwm_msk: crate::Reg<bpwm_msk::BPWM_MSK_SPEC>,
    _reserved18: [u8; 0x14],
    #[doc = "0xd4 - BPWM Pin Polar Inverse Register"]
    pub bpwm_polctl: crate::Reg<bpwm_polctl::BPWM_POLCTL_SPEC>,
    #[doc = "0xd8 - BPWM Output Enable Register"]
    pub bpwm_poen: crate::Reg<bpwm_poen::BPWM_POEN_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0xe0 - BPWM Interrupt Enable Register"]
    pub bpwm_inten: crate::Reg<bpwm_inten::BPWM_INTEN_SPEC>,
    _reserved21: [u8; 0x04],
    #[doc = "0xe8 - BPWM Interrupt Flag Register"]
    pub bpwm_intsts: crate::Reg<bpwm_intsts::BPWM_INTSTS_SPEC>,
    _reserved22: [u8; 0x0c],
    #[doc = "0xf8 - BPWM Trigger ADC Source Select Register 0"]
    pub bpwm_adcts0: crate::Reg<bpwm_adcts0::BPWM_ADCTS0_SPEC>,
    #[doc = "0xfc - BPWM Trigger ADC Source Select Register 1"]
    pub bpwm_adcts1: crate::Reg<bpwm_adcts1::BPWM_ADCTS1_SPEC>,
    _reserved24: [u8; 0x10],
    #[doc = "0x110 - BPWM Synchronous Start Control Register"]
    pub bpwm_ssctl: crate::Reg<bpwm_ssctl::BPWM_SSCTL_SPEC>,
    #[doc = "0x114 - BPWM Synchronous Start Trigger Register"]
    pub bpwm_sstrg: crate::Reg<bpwm_sstrg::BPWM_SSTRG_SPEC>,
    _reserved26: [u8; 0x08],
    #[doc = "0x120 - BPWM Status Register"]
    pub bpwm_status: crate::Reg<bpwm_status::BPWM_STATUS_SPEC>,
    _reserved27: [u8; 0xdc],
    #[doc = "0x200 - BPWM Capture Input Enable Register"]
    pub bpwm_capinen: crate::Reg<bpwm_capinen::BPWM_CAPINEN_SPEC>,
    #[doc = "0x204 - BPWM Capture Control Register"]
    pub bpwm_capctl: crate::Reg<bpwm_capctl::BPWM_CAPCTL_SPEC>,
    #[doc = "0x208 - BPWM Capture Status Register"]
    pub bpwm_capsts: crate::Reg<bpwm_capsts::BPWM_CAPSTS_SPEC>,
    #[doc = "0x20c - BPWM Rising Capture Data Register 0"]
    pub bpwm_rcapdat0: crate::Reg<bpwm_rcapdat0::BPWM_RCAPDAT0_SPEC>,
    #[doc = "0x210 - BPWM Falling Capture Data Register 0"]
    pub bpwm_fcapdat0: crate::Reg<bpwm_fcapdat0::BPWM_FCAPDAT0_SPEC>,
    #[doc = "0x214 - BPWM Rising Capture Data Register 1"]
    pub bpwm_rcapdat1: crate::Reg<bpwm_rcapdat1::BPWM_RCAPDAT1_SPEC>,
    #[doc = "0x218 - BPWM Falling Capture Data Register 1"]
    pub bpwm_fcapdat1: crate::Reg<bpwm_fcapdat1::BPWM_FCAPDAT1_SPEC>,
    #[doc = "0x21c - BPWM Rising Capture Data Register 2"]
    pub bpwm_rcapdat2: crate::Reg<bpwm_rcapdat2::BPWM_RCAPDAT2_SPEC>,
    #[doc = "0x220 - BPWM Falling Capture Data Register 2"]
    pub bpwm_fcapdat2: crate::Reg<bpwm_fcapdat2::BPWM_FCAPDAT2_SPEC>,
    #[doc = "0x224 - BPWM Rising Capture Data Register 3"]
    pub bpwm_rcapdat3: crate::Reg<bpwm_rcapdat3::BPWM_RCAPDAT3_SPEC>,
    #[doc = "0x228 - BPWM Falling Capture Data Register 3"]
    pub bpwm_fcapdat3: crate::Reg<bpwm_fcapdat3::BPWM_FCAPDAT3_SPEC>,
    #[doc = "0x22c - BPWM Rising Capture Data Register 4"]
    pub bpwm_rcapdat4: crate::Reg<bpwm_rcapdat4::BPWM_RCAPDAT4_SPEC>,
    #[doc = "0x230 - BPWM Falling Capture Data Register 4"]
    pub bpwm_fcapdat4: crate::Reg<bpwm_fcapdat4::BPWM_FCAPDAT4_SPEC>,
    #[doc = "0x234 - BPWM Rising Capture Data Register 5"]
    pub bpwm_rcapdat5: crate::Reg<bpwm_rcapdat5::BPWM_RCAPDAT5_SPEC>,
    #[doc = "0x238 - BPWM Falling Capture Data Register 5"]
    pub bpwm_fcapdat5: crate::Reg<bpwm_fcapdat5::BPWM_FCAPDAT5_SPEC>,
    _reserved42: [u8; 0x14],
    #[doc = "0x250 - BPWM Capture Interrupt Enable Register"]
    pub bpwm_capien: crate::Reg<bpwm_capien::BPWM_CAPIEN_SPEC>,
    #[doc = "0x254 - BPWM Capture Interrupt Flag Register"]
    pub bpwm_capif: crate::Reg<bpwm_capif::BPWM_CAPIF_SPEC>,
    _reserved44: [u8; 0xac],
    #[doc = "0x304 - BPWM PERIOD Buffer"]
    pub bpwm_pbuf: crate::Reg<bpwm_pbuf::BPWM_PBUF_SPEC>,
    _reserved45: [u8; 0x14],
    #[doc = "0x31c - BPWM CMPDAT 0 Buffer"]
    pub bpwm_cmpbuf0: crate::Reg<bpwm_cmpbuf0::BPWM_CMPBUF0_SPEC>,
    #[doc = "0x320 - BPWM CMPDAT 1 Buffer"]
    pub bpwm_cmpbuf1: crate::Reg<bpwm_cmpbuf1::BPWM_CMPBUF1_SPEC>,
    #[doc = "0x324 - BPWM CMPDAT 2 Buffer"]
    pub bpwm_cmpbuf2: crate::Reg<bpwm_cmpbuf2::BPWM_CMPBUF2_SPEC>,
    #[doc = "0x328 - BPWM CMPDAT 3 Buffer"]
    pub bpwm_cmpbuf3: crate::Reg<bpwm_cmpbuf3::BPWM_CMPBUF3_SPEC>,
    #[doc = "0x32c - BPWM CMPDAT 4 Buffer"]
    pub bpwm_cmpbuf4: crate::Reg<bpwm_cmpbuf4::BPWM_CMPBUF4_SPEC>,
    #[doc = "0x330 - BPWM CMPDAT 5 Buffer"]
    pub bpwm_cmpbuf5: crate::Reg<bpwm_cmpbuf5::BPWM_CMPBUF5_SPEC>,
}
#[doc = "BPWM_CTL0 register accessor: an alias for `Reg<BPWM_CTL0_SPEC>`"]
pub type BPWM_CTL0 = crate::Reg<bpwm_ctl0::BPWM_CTL0_SPEC>;
#[doc = "BPWM Control Register 0"]
pub mod bpwm_ctl0;
#[doc = "BPWM_CTL1 register accessor: an alias for `Reg<BPWM_CTL1_SPEC>`"]
pub type BPWM_CTL1 = crate::Reg<bpwm_ctl1::BPWM_CTL1_SPEC>;
#[doc = "BPWM Control Register 1"]
pub mod bpwm_ctl1;
#[doc = "BPWM_CLKSRC register accessor: an alias for `Reg<BPWM_CLKSRC_SPEC>`"]
pub type BPWM_CLKSRC = crate::Reg<bpwm_clksrc::BPWM_CLKSRC_SPEC>;
#[doc = "BPWM Clock Source Register"]
pub mod bpwm_clksrc;
#[doc = "BPWM_CLKPSC register accessor: an alias for `Reg<BPWM_CLKPSC_SPEC>`"]
pub type BPWM_CLKPSC = crate::Reg<bpwm_clkpsc::BPWM_CLKPSC_SPEC>;
#[doc = "BPWM Clock Prescale Register"]
pub mod bpwm_clkpsc;
#[doc = "BPWM_CNTEN register accessor: an alias for `Reg<BPWM_CNTEN_SPEC>`"]
pub type BPWM_CNTEN = crate::Reg<bpwm_cnten::BPWM_CNTEN_SPEC>;
#[doc = "BPWM Counter Enable Register"]
pub mod bpwm_cnten;
#[doc = "BPWM_CNTCLR register accessor: an alias for `Reg<BPWM_CNTCLR_SPEC>`"]
pub type BPWM_CNTCLR = crate::Reg<bpwm_cntclr::BPWM_CNTCLR_SPEC>;
#[doc = "BPWM Clear Counter Register"]
pub mod bpwm_cntclr;
#[doc = "BPWM_PERIOD register accessor: an alias for `Reg<BPWM_PERIOD_SPEC>`"]
pub type BPWM_PERIOD = crate::Reg<bpwm_period::BPWM_PERIOD_SPEC>;
#[doc = "BPWM Period Register"]
pub mod bpwm_period;
#[doc = "BPWM_CMPDAT0 register accessor: an alias for `Reg<BPWM_CMPDAT0_SPEC>`"]
pub type BPWM_CMPDAT0 = crate::Reg<bpwm_cmpdat0::BPWM_CMPDAT0_SPEC>;
#[doc = "BPWM Comparator Register 0"]
pub mod bpwm_cmpdat0;
#[doc = "BPWM_CMPDAT1 register accessor: an alias for `Reg<BPWM_CMPDAT1_SPEC>`"]
pub type BPWM_CMPDAT1 = crate::Reg<bpwm_cmpdat1::BPWM_CMPDAT1_SPEC>;
#[doc = "BPWM Comparator Register 1"]
pub mod bpwm_cmpdat1;
#[doc = "BPWM_CMPDAT2 register accessor: an alias for `Reg<BPWM_CMPDAT2_SPEC>`"]
pub type BPWM_CMPDAT2 = crate::Reg<bpwm_cmpdat2::BPWM_CMPDAT2_SPEC>;
#[doc = "BPWM Comparator Register 2"]
pub mod bpwm_cmpdat2;
#[doc = "BPWM_CMPDAT3 register accessor: an alias for `Reg<BPWM_CMPDAT3_SPEC>`"]
pub type BPWM_CMPDAT3 = crate::Reg<bpwm_cmpdat3::BPWM_CMPDAT3_SPEC>;
#[doc = "BPWM Comparator Register 3"]
pub mod bpwm_cmpdat3;
#[doc = "BPWM_CMPDAT4 register accessor: an alias for `Reg<BPWM_CMPDAT4_SPEC>`"]
pub type BPWM_CMPDAT4 = crate::Reg<bpwm_cmpdat4::BPWM_CMPDAT4_SPEC>;
#[doc = "BPWM Comparator Register 4"]
pub mod bpwm_cmpdat4;
#[doc = "BPWM_CMPDAT5 register accessor: an alias for `Reg<BPWM_CMPDAT5_SPEC>`"]
pub type BPWM_CMPDAT5 = crate::Reg<bpwm_cmpdat5::BPWM_CMPDAT5_SPEC>;
#[doc = "BPWM Comparator Register 5"]
pub mod bpwm_cmpdat5;
#[doc = "BPWM_CNT register accessor: an alias for `Reg<BPWM_CNT_SPEC>`"]
pub type BPWM_CNT = crate::Reg<bpwm_cnt::BPWM_CNT_SPEC>;
#[doc = "BPWM Counter Register"]
pub mod bpwm_cnt;
#[doc = "BPWM_WGCTL0 register accessor: an alias for `Reg<BPWM_WGCTL0_SPEC>`"]
pub type BPWM_WGCTL0 = crate::Reg<bpwm_wgctl0::BPWM_WGCTL0_SPEC>;
#[doc = "BPWM Generation Register 0"]
pub mod bpwm_wgctl0;
#[doc = "BPWM_WGCTL1 register accessor: an alias for `Reg<BPWM_WGCTL1_SPEC>`"]
pub type BPWM_WGCTL1 = crate::Reg<bpwm_wgctl1::BPWM_WGCTL1_SPEC>;
#[doc = "BPWM Generation Register 1"]
pub mod bpwm_wgctl1;
#[doc = "BPWM_MSKEN register accessor: an alias for `Reg<BPWM_MSKEN_SPEC>`"]
pub type BPWM_MSKEN = crate::Reg<bpwm_msken::BPWM_MSKEN_SPEC>;
#[doc = "BPWM Mask Enable Register"]
pub mod bpwm_msken;
#[doc = "BPWM_MSK register accessor: an alias for `Reg<BPWM_MSK_SPEC>`"]
pub type BPWM_MSK = crate::Reg<bpwm_msk::BPWM_MSK_SPEC>;
#[doc = "BPWM Mask Data Register"]
pub mod bpwm_msk;
#[doc = "BPWM_POLCTL register accessor: an alias for `Reg<BPWM_POLCTL_SPEC>`"]
pub type BPWM_POLCTL = crate::Reg<bpwm_polctl::BPWM_POLCTL_SPEC>;
#[doc = "BPWM Pin Polar Inverse Register"]
pub mod bpwm_polctl;
#[doc = "BPWM_POEN register accessor: an alias for `Reg<BPWM_POEN_SPEC>`"]
pub type BPWM_POEN = crate::Reg<bpwm_poen::BPWM_POEN_SPEC>;
#[doc = "BPWM Output Enable Register"]
pub mod bpwm_poen;
#[doc = "BPWM_INTEN register accessor: an alias for `Reg<BPWM_INTEN_SPEC>`"]
pub type BPWM_INTEN = crate::Reg<bpwm_inten::BPWM_INTEN_SPEC>;
#[doc = "BPWM Interrupt Enable Register"]
pub mod bpwm_inten;
#[doc = "BPWM_INTSTS register accessor: an alias for `Reg<BPWM_INTSTS_SPEC>`"]
pub type BPWM_INTSTS = crate::Reg<bpwm_intsts::BPWM_INTSTS_SPEC>;
#[doc = "BPWM Interrupt Flag Register"]
pub mod bpwm_intsts;
#[doc = "BPWM_ADCTS0 register accessor: an alias for `Reg<BPWM_ADCTS0_SPEC>`"]
pub type BPWM_ADCTS0 = crate::Reg<bpwm_adcts0::BPWM_ADCTS0_SPEC>;
#[doc = "BPWM Trigger ADC Source Select Register 0"]
pub mod bpwm_adcts0;
#[doc = "BPWM_ADCTS1 register accessor: an alias for `Reg<BPWM_ADCTS1_SPEC>`"]
pub type BPWM_ADCTS1 = crate::Reg<bpwm_adcts1::BPWM_ADCTS1_SPEC>;
#[doc = "BPWM Trigger ADC Source Select Register 1"]
pub mod bpwm_adcts1;
#[doc = "BPWM_SSCTL register accessor: an alias for `Reg<BPWM_SSCTL_SPEC>`"]
pub type BPWM_SSCTL = crate::Reg<bpwm_ssctl::BPWM_SSCTL_SPEC>;
#[doc = "BPWM Synchronous Start Control Register"]
pub mod bpwm_ssctl;
#[doc = "BPWM_SSTRG register accessor: an alias for `Reg<BPWM_SSTRG_SPEC>`"]
pub type BPWM_SSTRG = crate::Reg<bpwm_sstrg::BPWM_SSTRG_SPEC>;
#[doc = "BPWM Synchronous Start Trigger Register"]
pub mod bpwm_sstrg;
#[doc = "BPWM_STATUS register accessor: an alias for `Reg<BPWM_STATUS_SPEC>`"]
pub type BPWM_STATUS = crate::Reg<bpwm_status::BPWM_STATUS_SPEC>;
#[doc = "BPWM Status Register"]
pub mod bpwm_status;
#[doc = "BPWM_CAPINEN register accessor: an alias for `Reg<BPWM_CAPINEN_SPEC>`"]
pub type BPWM_CAPINEN = crate::Reg<bpwm_capinen::BPWM_CAPINEN_SPEC>;
#[doc = "BPWM Capture Input Enable Register"]
pub mod bpwm_capinen;
#[doc = "BPWM_CAPCTL register accessor: an alias for `Reg<BPWM_CAPCTL_SPEC>`"]
pub type BPWM_CAPCTL = crate::Reg<bpwm_capctl::BPWM_CAPCTL_SPEC>;
#[doc = "BPWM Capture Control Register"]
pub mod bpwm_capctl;
#[doc = "BPWM_CAPSTS register accessor: an alias for `Reg<BPWM_CAPSTS_SPEC>`"]
pub type BPWM_CAPSTS = crate::Reg<bpwm_capsts::BPWM_CAPSTS_SPEC>;
#[doc = "BPWM Capture Status Register"]
pub mod bpwm_capsts;
#[doc = "BPWM_RCAPDAT0 register accessor: an alias for `Reg<BPWM_RCAPDAT0_SPEC>`"]
pub type BPWM_RCAPDAT0 = crate::Reg<bpwm_rcapdat0::BPWM_RCAPDAT0_SPEC>;
#[doc = "BPWM Rising Capture Data Register 0"]
pub mod bpwm_rcapdat0;
#[doc = "BPWM_FCAPDAT0 register accessor: an alias for `Reg<BPWM_FCAPDAT0_SPEC>`"]
pub type BPWM_FCAPDAT0 = crate::Reg<bpwm_fcapdat0::BPWM_FCAPDAT0_SPEC>;
#[doc = "BPWM Falling Capture Data Register 0"]
pub mod bpwm_fcapdat0;
#[doc = "BPWM_RCAPDAT1 register accessor: an alias for `Reg<BPWM_RCAPDAT1_SPEC>`"]
pub type BPWM_RCAPDAT1 = crate::Reg<bpwm_rcapdat1::BPWM_RCAPDAT1_SPEC>;
#[doc = "BPWM Rising Capture Data Register 1"]
pub mod bpwm_rcapdat1;
#[doc = "BPWM_FCAPDAT1 register accessor: an alias for `Reg<BPWM_FCAPDAT1_SPEC>`"]
pub type BPWM_FCAPDAT1 = crate::Reg<bpwm_fcapdat1::BPWM_FCAPDAT1_SPEC>;
#[doc = "BPWM Falling Capture Data Register 1"]
pub mod bpwm_fcapdat1;
#[doc = "BPWM_RCAPDAT2 register accessor: an alias for `Reg<BPWM_RCAPDAT2_SPEC>`"]
pub type BPWM_RCAPDAT2 = crate::Reg<bpwm_rcapdat2::BPWM_RCAPDAT2_SPEC>;
#[doc = "BPWM Rising Capture Data Register 2"]
pub mod bpwm_rcapdat2;
#[doc = "BPWM_FCAPDAT2 register accessor: an alias for `Reg<BPWM_FCAPDAT2_SPEC>`"]
pub type BPWM_FCAPDAT2 = crate::Reg<bpwm_fcapdat2::BPWM_FCAPDAT2_SPEC>;
#[doc = "BPWM Falling Capture Data Register 2"]
pub mod bpwm_fcapdat2;
#[doc = "BPWM_RCAPDAT3 register accessor: an alias for `Reg<BPWM_RCAPDAT3_SPEC>`"]
pub type BPWM_RCAPDAT3 = crate::Reg<bpwm_rcapdat3::BPWM_RCAPDAT3_SPEC>;
#[doc = "BPWM Rising Capture Data Register 3"]
pub mod bpwm_rcapdat3;
#[doc = "BPWM_FCAPDAT3 register accessor: an alias for `Reg<BPWM_FCAPDAT3_SPEC>`"]
pub type BPWM_FCAPDAT3 = crate::Reg<bpwm_fcapdat3::BPWM_FCAPDAT3_SPEC>;
#[doc = "BPWM Falling Capture Data Register 3"]
pub mod bpwm_fcapdat3;
#[doc = "BPWM_RCAPDAT4 register accessor: an alias for `Reg<BPWM_RCAPDAT4_SPEC>`"]
pub type BPWM_RCAPDAT4 = crate::Reg<bpwm_rcapdat4::BPWM_RCAPDAT4_SPEC>;
#[doc = "BPWM Rising Capture Data Register 4"]
pub mod bpwm_rcapdat4;
#[doc = "BPWM_FCAPDAT4 register accessor: an alias for `Reg<BPWM_FCAPDAT4_SPEC>`"]
pub type BPWM_FCAPDAT4 = crate::Reg<bpwm_fcapdat4::BPWM_FCAPDAT4_SPEC>;
#[doc = "BPWM Falling Capture Data Register 4"]
pub mod bpwm_fcapdat4;
#[doc = "BPWM_RCAPDAT5 register accessor: an alias for `Reg<BPWM_RCAPDAT5_SPEC>`"]
pub type BPWM_RCAPDAT5 = crate::Reg<bpwm_rcapdat5::BPWM_RCAPDAT5_SPEC>;
#[doc = "BPWM Rising Capture Data Register 5"]
pub mod bpwm_rcapdat5;
#[doc = "BPWM_FCAPDAT5 register accessor: an alias for `Reg<BPWM_FCAPDAT5_SPEC>`"]
pub type BPWM_FCAPDAT5 = crate::Reg<bpwm_fcapdat5::BPWM_FCAPDAT5_SPEC>;
#[doc = "BPWM Falling Capture Data Register 5"]
pub mod bpwm_fcapdat5;
#[doc = "BPWM_CAPIEN register accessor: an alias for `Reg<BPWM_CAPIEN_SPEC>`"]
pub type BPWM_CAPIEN = crate::Reg<bpwm_capien::BPWM_CAPIEN_SPEC>;
#[doc = "BPWM Capture Interrupt Enable Register"]
pub mod bpwm_capien;
#[doc = "BPWM_CAPIF register accessor: an alias for `Reg<BPWM_CAPIF_SPEC>`"]
pub type BPWM_CAPIF = crate::Reg<bpwm_capif::BPWM_CAPIF_SPEC>;
#[doc = "BPWM Capture Interrupt Flag Register"]
pub mod bpwm_capif;
#[doc = "BPWM_PBUF register accessor: an alias for `Reg<BPWM_PBUF_SPEC>`"]
pub type BPWM_PBUF = crate::Reg<bpwm_pbuf::BPWM_PBUF_SPEC>;
#[doc = "BPWM PERIOD Buffer"]
pub mod bpwm_pbuf;
#[doc = "BPWM_CMPBUF0 register accessor: an alias for `Reg<BPWM_CMPBUF0_SPEC>`"]
pub type BPWM_CMPBUF0 = crate::Reg<bpwm_cmpbuf0::BPWM_CMPBUF0_SPEC>;
#[doc = "BPWM CMPDAT 0 Buffer"]
pub mod bpwm_cmpbuf0;
#[doc = "BPWM_CMPBUF1 register accessor: an alias for `Reg<BPWM_CMPBUF1_SPEC>`"]
pub type BPWM_CMPBUF1 = crate::Reg<bpwm_cmpbuf1::BPWM_CMPBUF1_SPEC>;
#[doc = "BPWM CMPDAT 1 Buffer"]
pub mod bpwm_cmpbuf1;
#[doc = "BPWM_CMPBUF2 register accessor: an alias for `Reg<BPWM_CMPBUF2_SPEC>`"]
pub type BPWM_CMPBUF2 = crate::Reg<bpwm_cmpbuf2::BPWM_CMPBUF2_SPEC>;
#[doc = "BPWM CMPDAT 2 Buffer"]
pub mod bpwm_cmpbuf2;
#[doc = "BPWM_CMPBUF3 register accessor: an alias for `Reg<BPWM_CMPBUF3_SPEC>`"]
pub type BPWM_CMPBUF3 = crate::Reg<bpwm_cmpbuf3::BPWM_CMPBUF3_SPEC>;
#[doc = "BPWM CMPDAT 3 Buffer"]
pub mod bpwm_cmpbuf3;
#[doc = "BPWM_CMPBUF4 register accessor: an alias for `Reg<BPWM_CMPBUF4_SPEC>`"]
pub type BPWM_CMPBUF4 = crate::Reg<bpwm_cmpbuf4::BPWM_CMPBUF4_SPEC>;
#[doc = "BPWM CMPDAT 4 Buffer"]
pub mod bpwm_cmpbuf4;
#[doc = "BPWM_CMPBUF5 register accessor: an alias for `Reg<BPWM_CMPBUF5_SPEC>`"]
pub type BPWM_CMPBUF5 = crate::Reg<bpwm_cmpbuf5::BPWM_CMPBUF5_SPEC>;
#[doc = "BPWM CMPDAT 5 Buffer"]
pub mod bpwm_cmpbuf5;
