#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Descriptor Table Control Register of PDMA Channel n"]
    pub pdma_dsct0_ctl: crate::Reg<pdma_dsct0_ctl::PDMA_DSCT0_CTL_SPEC>,
    #[doc = "0x04 - Source Address Register of PDMA Channel n"]
    pub pdma_dsct0_sa: crate::Reg<pdma_dsct0_sa::PDMA_DSCT0_SA_SPEC>,
    #[doc = "0x08 - Destination Address Register of PDMA Channel n"]
    pub pdma_dsct0_da: crate::Reg<pdma_dsct0_da::PDMA_DSCT0_DA_SPEC>,
    #[doc = "0x0c - Next Scatter-gather Descriptor Table Offset Address of PDMA Channel n"]
    pub pdma_dsct0_next: crate::Reg<pdma_dsct0_next::PDMA_DSCT0_NEXT_SPEC>,
    #[doc = "0x10 - Descriptor Table Control Register of PDMA Channel n"]
    pub pdma_dsct1_ctl: crate::Reg<pdma_dsct1_ctl::PDMA_DSCT1_CTL_SPEC>,
    #[doc = "0x14 - Source Address Register of PDMA Channel n"]
    pub pdma_dsct1_sa: crate::Reg<pdma_dsct1_sa::PDMA_DSCT1_SA_SPEC>,
    #[doc = "0x18 - Destination Address Register of PDMA Channel n"]
    pub pdma_dsct1_da: crate::Reg<pdma_dsct1_da::PDMA_DSCT1_DA_SPEC>,
    #[doc = "0x1c - Next Scatter-gather Descriptor Table Offset Address of PDMA Channel n"]
    pub pdma_dsct1_next: crate::Reg<pdma_dsct1_next::PDMA_DSCT1_NEXT_SPEC>,
    #[doc = "0x20 - Descriptor Table Control Register of PDMA Channel n"]
    pub pdma_dsct2_ctl: crate::Reg<pdma_dsct2_ctl::PDMA_DSCT2_CTL_SPEC>,
    #[doc = "0x24 - Source Address Register of PDMA Channel n"]
    pub pdma_dsct2_sa: crate::Reg<pdma_dsct2_sa::PDMA_DSCT2_SA_SPEC>,
    #[doc = "0x28 - Destination Address Register of PDMA Channel n"]
    pub pdma_dsct2_da: crate::Reg<pdma_dsct2_da::PDMA_DSCT2_DA_SPEC>,
    #[doc = "0x2c - Next Scatter-gather Descriptor Table Offset Address of PDMA Channel n"]
    pub pdma_dsct2_next: crate::Reg<pdma_dsct2_next::PDMA_DSCT2_NEXT_SPEC>,
    #[doc = "0x30 - Descriptor Table Control Register of PDMA Channel n"]
    pub pdma_dsct3_ctl: crate::Reg<pdma_dsct3_ctl::PDMA_DSCT3_CTL_SPEC>,
    #[doc = "0x34 - Source Address Register of PDMA Channel n"]
    pub pdma_dsct3_sa: crate::Reg<pdma_dsct3_sa::PDMA_DSCT3_SA_SPEC>,
    #[doc = "0x38 - Destination Address Register of PDMA Channel n"]
    pub pdma_dsct3_da: crate::Reg<pdma_dsct3_da::PDMA_DSCT3_DA_SPEC>,
    #[doc = "0x3c - Next Scatter-gather Descriptor Table Offset Address of PDMA Channel n"]
    pub pdma_dsct3_next: crate::Reg<pdma_dsct3_next::PDMA_DSCT3_NEXT_SPEC>,
    #[doc = "0x40 - Descriptor Table Control Register of PDMA Channel n"]
    pub pdma_dsct4_ctl: crate::Reg<pdma_dsct4_ctl::PDMA_DSCT4_CTL_SPEC>,
    #[doc = "0x44 - Source Address Register of PDMA Channel n"]
    pub pdma_dsct4_sa: crate::Reg<pdma_dsct4_sa::PDMA_DSCT4_SA_SPEC>,
    #[doc = "0x48 - Destination Address Register of PDMA Channel n"]
    pub pdma_dsct4_da: crate::Reg<pdma_dsct4_da::PDMA_DSCT4_DA_SPEC>,
    #[doc = "0x4c - Next Scatter-gather Descriptor Table Offset Address of PDMA Channel n"]
    pub pdma_dsct4_next: crate::Reg<pdma_dsct4_next::PDMA_DSCT4_NEXT_SPEC>,
    #[doc = "0x50 - Descriptor Table Control Register of PDMA Channel n"]
    pub pdma_dsct5_ctl: crate::Reg<pdma_dsct5_ctl::PDMA_DSCT5_CTL_SPEC>,
    #[doc = "0x54 - Source Address Register of PDMA Channel n"]
    pub pdma_dsct5_sa: crate::Reg<pdma_dsct5_sa::PDMA_DSCT5_SA_SPEC>,
    #[doc = "0x58 - Destination Address Register of PDMA Channel n"]
    pub pdma_dsct5_da: crate::Reg<pdma_dsct5_da::PDMA_DSCT5_DA_SPEC>,
    #[doc = "0x5c - Next Scatter-gather Descriptor Table Offset Address of PDMA Channel n"]
    pub pdma_dsct5_next: crate::Reg<pdma_dsct5_next::PDMA_DSCT5_NEXT_SPEC>,
    #[doc = "0x60 - Descriptor Table Control Register of PDMA Channel n"]
    pub pdma_dsct6_ctl: crate::Reg<pdma_dsct6_ctl::PDMA_DSCT6_CTL_SPEC>,
    #[doc = "0x64 - Source Address Register of PDMA Channel n"]
    pub pdma_dsct6_sa: crate::Reg<pdma_dsct6_sa::PDMA_DSCT6_SA_SPEC>,
    #[doc = "0x68 - Destination Address Register of PDMA Channel n"]
    pub pdma_dsct6_da: crate::Reg<pdma_dsct6_da::PDMA_DSCT6_DA_SPEC>,
    #[doc = "0x6c - Next Scatter-gather Descriptor Table Offset Address of PDMA Channel n"]
    pub pdma_dsct6_next: crate::Reg<pdma_dsct6_next::PDMA_DSCT6_NEXT_SPEC>,
    #[doc = "0x70 - Descriptor Table Control Register of PDMA Channel n"]
    pub pdma_dsct7_ctl: crate::Reg<pdma_dsct7_ctl::PDMA_DSCT7_CTL_SPEC>,
    #[doc = "0x74 - Source Address Register of PDMA Channel n"]
    pub pdma_dsct7_sa: crate::Reg<pdma_dsct7_sa::PDMA_DSCT7_SA_SPEC>,
    #[doc = "0x78 - Destination Address Register of PDMA Channel n"]
    pub pdma_dsct7_da: crate::Reg<pdma_dsct7_da::PDMA_DSCT7_DA_SPEC>,
    #[doc = "0x7c - Next Scatter-gather Descriptor Table Offset Address of PDMA Channel n"]
    pub pdma_dsct7_next: crate::Reg<pdma_dsct7_next::PDMA_DSCT7_NEXT_SPEC>,
    #[doc = "0x80 - Descriptor Table Control Register of PDMA Channel n"]
    pub pdma_dsct8_ctl: crate::Reg<pdma_dsct8_ctl::PDMA_DSCT8_CTL_SPEC>,
    #[doc = "0x84 - Source Address Register of PDMA Channel n"]
    pub pdma_dsct8_sa: crate::Reg<pdma_dsct8_sa::PDMA_DSCT8_SA_SPEC>,
    #[doc = "0x88 - Destination Address Register of PDMA Channel n"]
    pub pdma_dsct8_da: crate::Reg<pdma_dsct8_da::PDMA_DSCT8_DA_SPEC>,
    #[doc = "0x8c - Next Scatter-gather Descriptor Table Offset Address of PDMA Channel n"]
    pub pdma_dsct8_next: crate::Reg<pdma_dsct8_next::PDMA_DSCT8_NEXT_SPEC>,
    _reserved36: [u8; 0x70],
    #[doc = "0x100 - Current Scatter-gather Descriptor Table Address of PDMA Channel n"]
    pub pdma_curscat0: crate::Reg<pdma_curscat0::PDMA_CURSCAT0_SPEC>,
    #[doc = "0x104 - Current Scatter-gather Descriptor Table Address of PDMA Channel n"]
    pub pdma_curscat1: crate::Reg<pdma_curscat1::PDMA_CURSCAT1_SPEC>,
    #[doc = "0x108 - Current Scatter-gather Descriptor Table Address of PDMA Channel n"]
    pub pdma_curscat2: crate::Reg<pdma_curscat2::PDMA_CURSCAT2_SPEC>,
    #[doc = "0x10c - Current Scatter-gather Descriptor Table Address of PDMA Channel n"]
    pub pdma_curscat3: crate::Reg<pdma_curscat3::PDMA_CURSCAT3_SPEC>,
    #[doc = "0x110 - Current Scatter-gather Descriptor Table Address of PDMA Channel n"]
    pub pdma_curscat4: crate::Reg<pdma_curscat4::PDMA_CURSCAT4_SPEC>,
    #[doc = "0x114 - Current Scatter-gather Descriptor Table Address of PDMA Channel n"]
    pub pdma_curscat5: crate::Reg<pdma_curscat5::PDMA_CURSCAT5_SPEC>,
    #[doc = "0x118 - Current Scatter-gather Descriptor Table Address of PDMA Channel n"]
    pub pdma_curscat6: crate::Reg<pdma_curscat6::PDMA_CURSCAT6_SPEC>,
    #[doc = "0x11c - Current Scatter-gather Descriptor Table Address of PDMA Channel n"]
    pub pdma_curscat7: crate::Reg<pdma_curscat7::PDMA_CURSCAT7_SPEC>,
    #[doc = "0x120 - Current Scatter-gather Descriptor Table Address of PDMA Channel n"]
    pub pdma_curscat8: crate::Reg<pdma_curscat8::PDMA_CURSCAT8_SPEC>,
    _reserved45: [u8; 0x02dc],
    #[doc = "0x400 - PDMA Channel Control Register"]
    pub pdma_chctl: crate::Reg<pdma_chctl::PDMA_CHCTL_SPEC>,
    #[doc = "0x404 - PDMA Transfer Pause Control Register"]
    pub pdma_pause: crate::Reg<pdma_pause::PDMA_PAUSE_SPEC>,
    #[doc = "0x408 - PDMA Software Request Register"]
    pub pdma_swreq: crate::Reg<pdma_swreq::PDMA_SWREQ_SPEC>,
    #[doc = "0x40c - PDMA Channel Request Status Register"]
    pub pdma_trgsts: crate::Reg<pdma_trgsts::PDMA_TRGSTS_SPEC>,
    #[doc = "0x410 - PDMA Fixed Priority Setting Register"]
    pub pdma_priset: crate::Reg<pdma_priset::PDMA_PRISET_SPEC>,
    #[doc = "0x414 - PDMA Fixed Priority Clear Register"]
    pub pdma_priclr: crate::Reg<pdma_priclr::PDMA_PRICLR_SPEC>,
    #[doc = "0x418 - PDMA Interrupt Enable Register"]
    pub pdma_inten: crate::Reg<pdma_inten::PDMA_INTEN_SPEC>,
    #[doc = "0x41c - PDMA Interrupt Status Register"]
    pub pdma_intsts: crate::Reg<pdma_intsts::PDMA_INTSTS_SPEC>,
    #[doc = "0x420 - PDMA Channel Read/Write Target Abort Flag Register"]
    pub pdma_abtsts: crate::Reg<pdma_abtsts::PDMA_ABTSTS_SPEC>,
    #[doc = "0x424 - PDMA Channel Transfer Done Flag Register"]
    pub pdma_tdsts: crate::Reg<pdma_tdsts::PDMA_TDSTS_SPEC>,
    #[doc = "0x428 - PDMA Transfer Alignment Status Register"]
    pub pdma_align: crate::Reg<pdma_align::PDMA_ALIGN_SPEC>,
    #[doc = "0x42c - PDMA Transfer Active Flag Register"]
    pub pdma_tactsts: crate::Reg<pdma_tactsts::PDMA_TACTSTS_SPEC>,
    #[doc = "0x430 - PDMA Time-out Prescaler Register"]
    pub pdma_toutpsc: crate::Reg<pdma_toutpsc::PDMA_TOUTPSC_SPEC>,
    #[doc = "0x434 - PDMA Time-out Enable Register"]
    pub pdma_touten: crate::Reg<pdma_touten::PDMA_TOUTEN_SPEC>,
    #[doc = "0x438 - PDMA Time-out Interrupt Enable Register"]
    pub pdma_toutien: crate::Reg<pdma_toutien::PDMA_TOUTIEN_SPEC>,
    #[doc = "0x43c - PDMA Scatter-gather Descriptor Table Base Address Register"]
    pub pdma_scatba: crate::Reg<pdma_scatba::PDMA_SCATBA_SPEC>,
    #[doc = "0x440 - PDMA Time-out Counter Ch1 and Ch0 Register"]
    pub pdma_toc0_1: crate::Reg<pdma_toc0_1::PDMA_TOC0_1_SPEC>,
    _reserved62: [u8; 0x1c],
    #[doc = "0x460 - PDMA Channel Reset Register"]
    pub pdma_chrst: crate::Reg<pdma_chrst::PDMA_CHRST_SPEC>,
    _reserved63: [u8; 0x1c],
    #[doc = "0x480 - PDMA Request Source Select Register 0"]
    pub pdma_reqsel0_3: crate::Reg<pdma_reqsel0_3::PDMA_REQSEL0_3_SPEC>,
    #[doc = "0x484 - PDMA Request Source Select Register 1"]
    pub pdma_reqsel4_7: crate::Reg<pdma_reqsel4_7::PDMA_REQSEL4_7_SPEC>,
    #[doc = "0x488 - PDMA Request Source Select Register 2"]
    pub pdma_reqsel8: crate::Reg<pdma_reqsel8::PDMA_REQSEL8_SPEC>,
}
#[doc = "PDMA_DSCT0_CTL register accessor: an alias for `Reg<PDMA_DSCT0_CTL_SPEC>`"]
pub type PDMA_DSCT0_CTL = crate::Reg<pdma_dsct0_ctl::PDMA_DSCT0_CTL_SPEC>;
#[doc = "Descriptor Table Control Register of PDMA Channel n"]
pub mod pdma_dsct0_ctl;
#[doc = "PDMA_DSCT1_CTL register accessor: an alias for `Reg<PDMA_DSCT1_CTL_SPEC>`"]
pub type PDMA_DSCT1_CTL = crate::Reg<pdma_dsct1_ctl::PDMA_DSCT1_CTL_SPEC>;
#[doc = "Descriptor Table Control Register of PDMA Channel n"]
pub mod pdma_dsct1_ctl;
#[doc = "PDMA_DSCT2_CTL register accessor: an alias for `Reg<PDMA_DSCT2_CTL_SPEC>`"]
pub type PDMA_DSCT2_CTL = crate::Reg<pdma_dsct2_ctl::PDMA_DSCT2_CTL_SPEC>;
#[doc = "Descriptor Table Control Register of PDMA Channel n"]
pub mod pdma_dsct2_ctl;
#[doc = "PDMA_DSCT3_CTL register accessor: an alias for `Reg<PDMA_DSCT3_CTL_SPEC>`"]
pub type PDMA_DSCT3_CTL = crate::Reg<pdma_dsct3_ctl::PDMA_DSCT3_CTL_SPEC>;
#[doc = "Descriptor Table Control Register of PDMA Channel n"]
pub mod pdma_dsct3_ctl;
#[doc = "PDMA_DSCT4_CTL register accessor: an alias for `Reg<PDMA_DSCT4_CTL_SPEC>`"]
pub type PDMA_DSCT4_CTL = crate::Reg<pdma_dsct4_ctl::PDMA_DSCT4_CTL_SPEC>;
#[doc = "Descriptor Table Control Register of PDMA Channel n"]
pub mod pdma_dsct4_ctl;
#[doc = "PDMA_DSCT5_CTL register accessor: an alias for `Reg<PDMA_DSCT5_CTL_SPEC>`"]
pub type PDMA_DSCT5_CTL = crate::Reg<pdma_dsct5_ctl::PDMA_DSCT5_CTL_SPEC>;
#[doc = "Descriptor Table Control Register of PDMA Channel n"]
pub mod pdma_dsct5_ctl;
#[doc = "PDMA_DSCT6_CTL register accessor: an alias for `Reg<PDMA_DSCT6_CTL_SPEC>`"]
pub type PDMA_DSCT6_CTL = crate::Reg<pdma_dsct6_ctl::PDMA_DSCT6_CTL_SPEC>;
#[doc = "Descriptor Table Control Register of PDMA Channel n"]
pub mod pdma_dsct6_ctl;
#[doc = "PDMA_DSCT7_CTL register accessor: an alias for `Reg<PDMA_DSCT7_CTL_SPEC>`"]
pub type PDMA_DSCT7_CTL = crate::Reg<pdma_dsct7_ctl::PDMA_DSCT7_CTL_SPEC>;
#[doc = "Descriptor Table Control Register of PDMA Channel n"]
pub mod pdma_dsct7_ctl;
#[doc = "PDMA_DSCT8_CTL register accessor: an alias for `Reg<PDMA_DSCT8_CTL_SPEC>`"]
pub type PDMA_DSCT8_CTL = crate::Reg<pdma_dsct8_ctl::PDMA_DSCT8_CTL_SPEC>;
#[doc = "Descriptor Table Control Register of PDMA Channel n"]
pub mod pdma_dsct8_ctl;
#[doc = "PDMA_DSCT0_SA register accessor: an alias for `Reg<PDMA_DSCT0_SA_SPEC>`"]
pub type PDMA_DSCT0_SA = crate::Reg<pdma_dsct0_sa::PDMA_DSCT0_SA_SPEC>;
#[doc = "Source Address Register of PDMA Channel n"]
pub mod pdma_dsct0_sa;
#[doc = "PDMA_DSCT1_SA register accessor: an alias for `Reg<PDMA_DSCT1_SA_SPEC>`"]
pub type PDMA_DSCT1_SA = crate::Reg<pdma_dsct1_sa::PDMA_DSCT1_SA_SPEC>;
#[doc = "Source Address Register of PDMA Channel n"]
pub mod pdma_dsct1_sa;
#[doc = "PDMA_DSCT2_SA register accessor: an alias for `Reg<PDMA_DSCT2_SA_SPEC>`"]
pub type PDMA_DSCT2_SA = crate::Reg<pdma_dsct2_sa::PDMA_DSCT2_SA_SPEC>;
#[doc = "Source Address Register of PDMA Channel n"]
pub mod pdma_dsct2_sa;
#[doc = "PDMA_DSCT3_SA register accessor: an alias for `Reg<PDMA_DSCT3_SA_SPEC>`"]
pub type PDMA_DSCT3_SA = crate::Reg<pdma_dsct3_sa::PDMA_DSCT3_SA_SPEC>;
#[doc = "Source Address Register of PDMA Channel n"]
pub mod pdma_dsct3_sa;
#[doc = "PDMA_DSCT4_SA register accessor: an alias for `Reg<PDMA_DSCT4_SA_SPEC>`"]
pub type PDMA_DSCT4_SA = crate::Reg<pdma_dsct4_sa::PDMA_DSCT4_SA_SPEC>;
#[doc = "Source Address Register of PDMA Channel n"]
pub mod pdma_dsct4_sa;
#[doc = "PDMA_DSCT5_SA register accessor: an alias for `Reg<PDMA_DSCT5_SA_SPEC>`"]
pub type PDMA_DSCT5_SA = crate::Reg<pdma_dsct5_sa::PDMA_DSCT5_SA_SPEC>;
#[doc = "Source Address Register of PDMA Channel n"]
pub mod pdma_dsct5_sa;
#[doc = "PDMA_DSCT6_SA register accessor: an alias for `Reg<PDMA_DSCT6_SA_SPEC>`"]
pub type PDMA_DSCT6_SA = crate::Reg<pdma_dsct6_sa::PDMA_DSCT6_SA_SPEC>;
#[doc = "Source Address Register of PDMA Channel n"]
pub mod pdma_dsct6_sa;
#[doc = "PDMA_DSCT7_SA register accessor: an alias for `Reg<PDMA_DSCT7_SA_SPEC>`"]
pub type PDMA_DSCT7_SA = crate::Reg<pdma_dsct7_sa::PDMA_DSCT7_SA_SPEC>;
#[doc = "Source Address Register of PDMA Channel n"]
pub mod pdma_dsct7_sa;
#[doc = "PDMA_DSCT8_SA register accessor: an alias for `Reg<PDMA_DSCT8_SA_SPEC>`"]
pub type PDMA_DSCT8_SA = crate::Reg<pdma_dsct8_sa::PDMA_DSCT8_SA_SPEC>;
#[doc = "Source Address Register of PDMA Channel n"]
pub mod pdma_dsct8_sa;
#[doc = "PDMA_DSCT0_DA register accessor: an alias for `Reg<PDMA_DSCT0_DA_SPEC>`"]
pub type PDMA_DSCT0_DA = crate::Reg<pdma_dsct0_da::PDMA_DSCT0_DA_SPEC>;
#[doc = "Destination Address Register of PDMA Channel n"]
pub mod pdma_dsct0_da;
#[doc = "PDMA_DSCT1_DA register accessor: an alias for `Reg<PDMA_DSCT1_DA_SPEC>`"]
pub type PDMA_DSCT1_DA = crate::Reg<pdma_dsct1_da::PDMA_DSCT1_DA_SPEC>;
#[doc = "Destination Address Register of PDMA Channel n"]
pub mod pdma_dsct1_da;
#[doc = "PDMA_DSCT2_DA register accessor: an alias for `Reg<PDMA_DSCT2_DA_SPEC>`"]
pub type PDMA_DSCT2_DA = crate::Reg<pdma_dsct2_da::PDMA_DSCT2_DA_SPEC>;
#[doc = "Destination Address Register of PDMA Channel n"]
pub mod pdma_dsct2_da;
#[doc = "PDMA_DSCT3_DA register accessor: an alias for `Reg<PDMA_DSCT3_DA_SPEC>`"]
pub type PDMA_DSCT3_DA = crate::Reg<pdma_dsct3_da::PDMA_DSCT3_DA_SPEC>;
#[doc = "Destination Address Register of PDMA Channel n"]
pub mod pdma_dsct3_da;
#[doc = "PDMA_DSCT4_DA register accessor: an alias for `Reg<PDMA_DSCT4_DA_SPEC>`"]
pub type PDMA_DSCT4_DA = crate::Reg<pdma_dsct4_da::PDMA_DSCT4_DA_SPEC>;
#[doc = "Destination Address Register of PDMA Channel n"]
pub mod pdma_dsct4_da;
#[doc = "PDMA_DSCT5_DA register accessor: an alias for `Reg<PDMA_DSCT5_DA_SPEC>`"]
pub type PDMA_DSCT5_DA = crate::Reg<pdma_dsct5_da::PDMA_DSCT5_DA_SPEC>;
#[doc = "Destination Address Register of PDMA Channel n"]
pub mod pdma_dsct5_da;
#[doc = "PDMA_DSCT6_DA register accessor: an alias for `Reg<PDMA_DSCT6_DA_SPEC>`"]
pub type PDMA_DSCT6_DA = crate::Reg<pdma_dsct6_da::PDMA_DSCT6_DA_SPEC>;
#[doc = "Destination Address Register of PDMA Channel n"]
pub mod pdma_dsct6_da;
#[doc = "PDMA_DSCT7_DA register accessor: an alias for `Reg<PDMA_DSCT7_DA_SPEC>`"]
pub type PDMA_DSCT7_DA = crate::Reg<pdma_dsct7_da::PDMA_DSCT7_DA_SPEC>;
#[doc = "Destination Address Register of PDMA Channel n"]
pub mod pdma_dsct7_da;
#[doc = "PDMA_DSCT8_DA register accessor: an alias for `Reg<PDMA_DSCT8_DA_SPEC>`"]
pub type PDMA_DSCT8_DA = crate::Reg<pdma_dsct8_da::PDMA_DSCT8_DA_SPEC>;
#[doc = "Destination Address Register of PDMA Channel n"]
pub mod pdma_dsct8_da;
#[doc = "PDMA_DSCT0_NEXT register accessor: an alias for `Reg<PDMA_DSCT0_NEXT_SPEC>`"]
pub type PDMA_DSCT0_NEXT = crate::Reg<pdma_dsct0_next::PDMA_DSCT0_NEXT_SPEC>;
#[doc = "Next Scatter-gather Descriptor Table Offset Address of PDMA Channel n"]
pub mod pdma_dsct0_next;
#[doc = "PDMA_DSCT1_NEXT register accessor: an alias for `Reg<PDMA_DSCT1_NEXT_SPEC>`"]
pub type PDMA_DSCT1_NEXT = crate::Reg<pdma_dsct1_next::PDMA_DSCT1_NEXT_SPEC>;
#[doc = "Next Scatter-gather Descriptor Table Offset Address of PDMA Channel n"]
pub mod pdma_dsct1_next;
#[doc = "PDMA_DSCT2_NEXT register accessor: an alias for `Reg<PDMA_DSCT2_NEXT_SPEC>`"]
pub type PDMA_DSCT2_NEXT = crate::Reg<pdma_dsct2_next::PDMA_DSCT2_NEXT_SPEC>;
#[doc = "Next Scatter-gather Descriptor Table Offset Address of PDMA Channel n"]
pub mod pdma_dsct2_next;
#[doc = "PDMA_DSCT3_NEXT register accessor: an alias for `Reg<PDMA_DSCT3_NEXT_SPEC>`"]
pub type PDMA_DSCT3_NEXT = crate::Reg<pdma_dsct3_next::PDMA_DSCT3_NEXT_SPEC>;
#[doc = "Next Scatter-gather Descriptor Table Offset Address of PDMA Channel n"]
pub mod pdma_dsct3_next;
#[doc = "PDMA_DSCT4_NEXT register accessor: an alias for `Reg<PDMA_DSCT4_NEXT_SPEC>`"]
pub type PDMA_DSCT4_NEXT = crate::Reg<pdma_dsct4_next::PDMA_DSCT4_NEXT_SPEC>;
#[doc = "Next Scatter-gather Descriptor Table Offset Address of PDMA Channel n"]
pub mod pdma_dsct4_next;
#[doc = "PDMA_DSCT5_NEXT register accessor: an alias for `Reg<PDMA_DSCT5_NEXT_SPEC>`"]
pub type PDMA_DSCT5_NEXT = crate::Reg<pdma_dsct5_next::PDMA_DSCT5_NEXT_SPEC>;
#[doc = "Next Scatter-gather Descriptor Table Offset Address of PDMA Channel n"]
pub mod pdma_dsct5_next;
#[doc = "PDMA_DSCT6_NEXT register accessor: an alias for `Reg<PDMA_DSCT6_NEXT_SPEC>`"]
pub type PDMA_DSCT6_NEXT = crate::Reg<pdma_dsct6_next::PDMA_DSCT6_NEXT_SPEC>;
#[doc = "Next Scatter-gather Descriptor Table Offset Address of PDMA Channel n"]
pub mod pdma_dsct6_next;
#[doc = "PDMA_DSCT7_NEXT register accessor: an alias for `Reg<PDMA_DSCT7_NEXT_SPEC>`"]
pub type PDMA_DSCT7_NEXT = crate::Reg<pdma_dsct7_next::PDMA_DSCT7_NEXT_SPEC>;
#[doc = "Next Scatter-gather Descriptor Table Offset Address of PDMA Channel n"]
pub mod pdma_dsct7_next;
#[doc = "PDMA_DSCT8_NEXT register accessor: an alias for `Reg<PDMA_DSCT8_NEXT_SPEC>`"]
pub type PDMA_DSCT8_NEXT = crate::Reg<pdma_dsct8_next::PDMA_DSCT8_NEXT_SPEC>;
#[doc = "Next Scatter-gather Descriptor Table Offset Address of PDMA Channel n"]
pub mod pdma_dsct8_next;
#[doc = "PDMA_CURSCAT0 register accessor: an alias for `Reg<PDMA_CURSCAT0_SPEC>`"]
pub type PDMA_CURSCAT0 = crate::Reg<pdma_curscat0::PDMA_CURSCAT0_SPEC>;
#[doc = "Current Scatter-gather Descriptor Table Address of PDMA Channel n"]
pub mod pdma_curscat0;
#[doc = "PDMA_CURSCAT1 register accessor: an alias for `Reg<PDMA_CURSCAT1_SPEC>`"]
pub type PDMA_CURSCAT1 = crate::Reg<pdma_curscat1::PDMA_CURSCAT1_SPEC>;
#[doc = "Current Scatter-gather Descriptor Table Address of PDMA Channel n"]
pub mod pdma_curscat1;
#[doc = "PDMA_CURSCAT2 register accessor: an alias for `Reg<PDMA_CURSCAT2_SPEC>`"]
pub type PDMA_CURSCAT2 = crate::Reg<pdma_curscat2::PDMA_CURSCAT2_SPEC>;
#[doc = "Current Scatter-gather Descriptor Table Address of PDMA Channel n"]
pub mod pdma_curscat2;
#[doc = "PDMA_CURSCAT3 register accessor: an alias for `Reg<PDMA_CURSCAT3_SPEC>`"]
pub type PDMA_CURSCAT3 = crate::Reg<pdma_curscat3::PDMA_CURSCAT3_SPEC>;
#[doc = "Current Scatter-gather Descriptor Table Address of PDMA Channel n"]
pub mod pdma_curscat3;
#[doc = "PDMA_CURSCAT4 register accessor: an alias for `Reg<PDMA_CURSCAT4_SPEC>`"]
pub type PDMA_CURSCAT4 = crate::Reg<pdma_curscat4::PDMA_CURSCAT4_SPEC>;
#[doc = "Current Scatter-gather Descriptor Table Address of PDMA Channel n"]
pub mod pdma_curscat4;
#[doc = "PDMA_CURSCAT5 register accessor: an alias for `Reg<PDMA_CURSCAT5_SPEC>`"]
pub type PDMA_CURSCAT5 = crate::Reg<pdma_curscat5::PDMA_CURSCAT5_SPEC>;
#[doc = "Current Scatter-gather Descriptor Table Address of PDMA Channel n"]
pub mod pdma_curscat5;
#[doc = "PDMA_CURSCAT6 register accessor: an alias for `Reg<PDMA_CURSCAT6_SPEC>`"]
pub type PDMA_CURSCAT6 = crate::Reg<pdma_curscat6::PDMA_CURSCAT6_SPEC>;
#[doc = "Current Scatter-gather Descriptor Table Address of PDMA Channel n"]
pub mod pdma_curscat6;
#[doc = "PDMA_CURSCAT7 register accessor: an alias for `Reg<PDMA_CURSCAT7_SPEC>`"]
pub type PDMA_CURSCAT7 = crate::Reg<pdma_curscat7::PDMA_CURSCAT7_SPEC>;
#[doc = "Current Scatter-gather Descriptor Table Address of PDMA Channel n"]
pub mod pdma_curscat7;
#[doc = "PDMA_CURSCAT8 register accessor: an alias for `Reg<PDMA_CURSCAT8_SPEC>`"]
pub type PDMA_CURSCAT8 = crate::Reg<pdma_curscat8::PDMA_CURSCAT8_SPEC>;
#[doc = "Current Scatter-gather Descriptor Table Address of PDMA Channel n"]
pub mod pdma_curscat8;
#[doc = "PDMA_CHCTL register accessor: an alias for `Reg<PDMA_CHCTL_SPEC>`"]
pub type PDMA_CHCTL = crate::Reg<pdma_chctl::PDMA_CHCTL_SPEC>;
#[doc = "PDMA Channel Control Register"]
pub mod pdma_chctl;
#[doc = "PDMA_PAUSE register accessor: an alias for `Reg<PDMA_PAUSE_SPEC>`"]
pub type PDMA_PAUSE = crate::Reg<pdma_pause::PDMA_PAUSE_SPEC>;
#[doc = "PDMA Transfer Pause Control Register"]
pub mod pdma_pause;
#[doc = "PDMA_SWREQ register accessor: an alias for `Reg<PDMA_SWREQ_SPEC>`"]
pub type PDMA_SWREQ = crate::Reg<pdma_swreq::PDMA_SWREQ_SPEC>;
#[doc = "PDMA Software Request Register"]
pub mod pdma_swreq;
#[doc = "PDMA_TRGSTS register accessor: an alias for `Reg<PDMA_TRGSTS_SPEC>`"]
pub type PDMA_TRGSTS = crate::Reg<pdma_trgsts::PDMA_TRGSTS_SPEC>;
#[doc = "PDMA Channel Request Status Register"]
pub mod pdma_trgsts;
#[doc = "PDMA_PRISET register accessor: an alias for `Reg<PDMA_PRISET_SPEC>`"]
pub type PDMA_PRISET = crate::Reg<pdma_priset::PDMA_PRISET_SPEC>;
#[doc = "PDMA Fixed Priority Setting Register"]
pub mod pdma_priset;
#[doc = "PDMA_PRICLR register accessor: an alias for `Reg<PDMA_PRICLR_SPEC>`"]
pub type PDMA_PRICLR = crate::Reg<pdma_priclr::PDMA_PRICLR_SPEC>;
#[doc = "PDMA Fixed Priority Clear Register"]
pub mod pdma_priclr;
#[doc = "PDMA_INTEN register accessor: an alias for `Reg<PDMA_INTEN_SPEC>`"]
pub type PDMA_INTEN = crate::Reg<pdma_inten::PDMA_INTEN_SPEC>;
#[doc = "PDMA Interrupt Enable Register"]
pub mod pdma_inten;
#[doc = "PDMA_INTSTS register accessor: an alias for `Reg<PDMA_INTSTS_SPEC>`"]
pub type PDMA_INTSTS = crate::Reg<pdma_intsts::PDMA_INTSTS_SPEC>;
#[doc = "PDMA Interrupt Status Register"]
pub mod pdma_intsts;
#[doc = "PDMA_ABTSTS register accessor: an alias for `Reg<PDMA_ABTSTS_SPEC>`"]
pub type PDMA_ABTSTS = crate::Reg<pdma_abtsts::PDMA_ABTSTS_SPEC>;
#[doc = "PDMA Channel Read/Write Target Abort Flag Register"]
pub mod pdma_abtsts;
#[doc = "PDMA_TDSTS register accessor: an alias for `Reg<PDMA_TDSTS_SPEC>`"]
pub type PDMA_TDSTS = crate::Reg<pdma_tdsts::PDMA_TDSTS_SPEC>;
#[doc = "PDMA Channel Transfer Done Flag Register"]
pub mod pdma_tdsts;
#[doc = "PDMA_ALIGN register accessor: an alias for `Reg<PDMA_ALIGN_SPEC>`"]
pub type PDMA_ALIGN = crate::Reg<pdma_align::PDMA_ALIGN_SPEC>;
#[doc = "PDMA Transfer Alignment Status Register"]
pub mod pdma_align;
#[doc = "PDMA_TACTSTS register accessor: an alias for `Reg<PDMA_TACTSTS_SPEC>`"]
pub type PDMA_TACTSTS = crate::Reg<pdma_tactsts::PDMA_TACTSTS_SPEC>;
#[doc = "PDMA Transfer Active Flag Register"]
pub mod pdma_tactsts;
#[doc = "PDMA_TOUTPSC register accessor: an alias for `Reg<PDMA_TOUTPSC_SPEC>`"]
pub type PDMA_TOUTPSC = crate::Reg<pdma_toutpsc::PDMA_TOUTPSC_SPEC>;
#[doc = "PDMA Time-out Prescaler Register"]
pub mod pdma_toutpsc;
#[doc = "PDMA_TOUTEN register accessor: an alias for `Reg<PDMA_TOUTEN_SPEC>`"]
pub type PDMA_TOUTEN = crate::Reg<pdma_touten::PDMA_TOUTEN_SPEC>;
#[doc = "PDMA Time-out Enable Register"]
pub mod pdma_touten;
#[doc = "PDMA_TOUTIEN register accessor: an alias for `Reg<PDMA_TOUTIEN_SPEC>`"]
pub type PDMA_TOUTIEN = crate::Reg<pdma_toutien::PDMA_TOUTIEN_SPEC>;
#[doc = "PDMA Time-out Interrupt Enable Register"]
pub mod pdma_toutien;
#[doc = "PDMA_SCATBA register accessor: an alias for `Reg<PDMA_SCATBA_SPEC>`"]
pub type PDMA_SCATBA = crate::Reg<pdma_scatba::PDMA_SCATBA_SPEC>;
#[doc = "PDMA Scatter-gather Descriptor Table Base Address Register"]
pub mod pdma_scatba;
#[doc = "PDMA_TOC0_1 register accessor: an alias for `Reg<PDMA_TOC0_1_SPEC>`"]
pub type PDMA_TOC0_1 = crate::Reg<pdma_toc0_1::PDMA_TOC0_1_SPEC>;
#[doc = "PDMA Time-out Counter Ch1 and Ch0 Register"]
pub mod pdma_toc0_1;
#[doc = "PDMA_CHRST register accessor: an alias for `Reg<PDMA_CHRST_SPEC>`"]
pub type PDMA_CHRST = crate::Reg<pdma_chrst::PDMA_CHRST_SPEC>;
#[doc = "PDMA Channel Reset Register"]
pub mod pdma_chrst;
#[doc = "PDMA_REQSEL0_3 register accessor: an alias for `Reg<PDMA_REQSEL0_3_SPEC>`"]
pub type PDMA_REQSEL0_3 = crate::Reg<pdma_reqsel0_3::PDMA_REQSEL0_3_SPEC>;
#[doc = "PDMA Request Source Select Register 0"]
pub mod pdma_reqsel0_3;
#[doc = "PDMA_REQSEL4_7 register accessor: an alias for `Reg<PDMA_REQSEL4_7_SPEC>`"]
pub type PDMA_REQSEL4_7 = crate::Reg<pdma_reqsel4_7::PDMA_REQSEL4_7_SPEC>;
#[doc = "PDMA Request Source Select Register 1"]
pub mod pdma_reqsel4_7;
#[doc = "PDMA_REQSEL8 register accessor: an alias for `Reg<PDMA_REQSEL8_SPEC>`"]
pub type PDMA_REQSEL8 = crate::Reg<pdma_reqsel8::PDMA_REQSEL8_SPEC>;
#[doc = "PDMA Request Source Select Register 2"]
pub mod pdma_reqsel8;
