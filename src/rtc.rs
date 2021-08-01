#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC Initiation Register"]
    pub rtc_init: crate::Reg<rtc_init::RTC_INIT_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - RTC Frequency Compensation Register"]
    pub rtc_freqadj: crate::Reg<rtc_freqadj::RTC_FREQADJ_SPEC>,
    #[doc = "0x0c - RTC Time Loading Register"]
    pub rtc_time: crate::Reg<rtc_time::RTC_TIME_SPEC>,
    #[doc = "0x10 - RTC Calendar Loading Register"]
    pub rtc_cal: crate::Reg<rtc_cal::RTC_CAL_SPEC>,
    #[doc = "0x14 - RTC Time Scale Selection Register"]
    pub rtc_clkfmt: crate::Reg<rtc_clkfmt::RTC_CLKFMT_SPEC>,
    #[doc = "0x18 - RTC Day of the Week Register"]
    pub rtc_weekday: crate::Reg<rtc_weekday::RTC_WEEKDAY_SPEC>,
    #[doc = "0x1c - RTC Time Alarm Register"]
    pub rtc_talm: crate::Reg<rtc_talm::RTC_TALM_SPEC>,
    #[doc = "0x20 - RTC Calendar Alarm Register"]
    pub rtc_calm: crate::Reg<rtc_calm::RTC_CALM_SPEC>,
    #[doc = "0x24 - RTC Leap Year Indicator Register"]
    pub rtc_leapyear: crate::Reg<rtc_leapyear::RTC_LEAPYEAR_SPEC>,
    #[doc = "0x28 - RTC Interrupt Enable Register"]
    pub rtc_inten: crate::Reg<rtc_inten::RTC_INTEN_SPEC>,
    #[doc = "0x2c - RTC Interrupt Status Register"]
    pub rtc_intsts: crate::Reg<rtc_intsts::RTC_INTSTS_SPEC>,
    #[doc = "0x30 - RTC Time Tick Register"]
    pub rtc_tick: crate::Reg<rtc_tick::RTC_TICK_SPEC>,
    #[doc = "0x34 - RTC Time Alarm Mask Register"]
    pub rtc_tamsk: crate::Reg<rtc_tamsk::RTC_TAMSK_SPEC>,
    #[doc = "0x38 - RTC Calendar Alarm Mask Register"]
    pub rtc_camsk: crate::Reg<rtc_camsk::RTC_CAMSK_SPEC>,
    _reserved14: [u8; 0xc4],
    #[doc = "0x100 - RTC 32K.768 KHz LXT Control Register"]
    pub rtc_lxtctl: crate::Reg<rtc_lxtctl::RTC_LXTCTL_SPEC>,
}
#[doc = "RTC_INIT register accessor: an alias for `Reg<RTC_INIT_SPEC>`"]
pub type RTC_INIT = crate::Reg<rtc_init::RTC_INIT_SPEC>;
#[doc = "RTC Initiation Register"]
pub mod rtc_init;
#[doc = "RTC_FREQADJ register accessor: an alias for `Reg<RTC_FREQADJ_SPEC>`"]
pub type RTC_FREQADJ = crate::Reg<rtc_freqadj::RTC_FREQADJ_SPEC>;
#[doc = "RTC Frequency Compensation Register"]
pub mod rtc_freqadj;
#[doc = "RTC_TIME register accessor: an alias for `Reg<RTC_TIME_SPEC>`"]
pub type RTC_TIME = crate::Reg<rtc_time::RTC_TIME_SPEC>;
#[doc = "RTC Time Loading Register"]
pub mod rtc_time;
#[doc = "RTC_CAL register accessor: an alias for `Reg<RTC_CAL_SPEC>`"]
pub type RTC_CAL = crate::Reg<rtc_cal::RTC_CAL_SPEC>;
#[doc = "RTC Calendar Loading Register"]
pub mod rtc_cal;
#[doc = "RTC_CLKFMT register accessor: an alias for `Reg<RTC_CLKFMT_SPEC>`"]
pub type RTC_CLKFMT = crate::Reg<rtc_clkfmt::RTC_CLKFMT_SPEC>;
#[doc = "RTC Time Scale Selection Register"]
pub mod rtc_clkfmt;
#[doc = "RTC_WEEKDAY register accessor: an alias for `Reg<RTC_WEEKDAY_SPEC>`"]
pub type RTC_WEEKDAY = crate::Reg<rtc_weekday::RTC_WEEKDAY_SPEC>;
#[doc = "RTC Day of the Week Register"]
pub mod rtc_weekday;
#[doc = "RTC_TALM register accessor: an alias for `Reg<RTC_TALM_SPEC>`"]
pub type RTC_TALM = crate::Reg<rtc_talm::RTC_TALM_SPEC>;
#[doc = "RTC Time Alarm Register"]
pub mod rtc_talm;
#[doc = "RTC_CALM register accessor: an alias for `Reg<RTC_CALM_SPEC>`"]
pub type RTC_CALM = crate::Reg<rtc_calm::RTC_CALM_SPEC>;
#[doc = "RTC Calendar Alarm Register"]
pub mod rtc_calm;
#[doc = "RTC_LEAPYEAR register accessor: an alias for `Reg<RTC_LEAPYEAR_SPEC>`"]
pub type RTC_LEAPYEAR = crate::Reg<rtc_leapyear::RTC_LEAPYEAR_SPEC>;
#[doc = "RTC Leap Year Indicator Register"]
pub mod rtc_leapyear;
#[doc = "RTC_INTEN register accessor: an alias for `Reg<RTC_INTEN_SPEC>`"]
pub type RTC_INTEN = crate::Reg<rtc_inten::RTC_INTEN_SPEC>;
#[doc = "RTC Interrupt Enable Register"]
pub mod rtc_inten;
#[doc = "RTC_INTSTS register accessor: an alias for `Reg<RTC_INTSTS_SPEC>`"]
pub type RTC_INTSTS = crate::Reg<rtc_intsts::RTC_INTSTS_SPEC>;
#[doc = "RTC Interrupt Status Register"]
pub mod rtc_intsts;
#[doc = "RTC_TICK register accessor: an alias for `Reg<RTC_TICK_SPEC>`"]
pub type RTC_TICK = crate::Reg<rtc_tick::RTC_TICK_SPEC>;
#[doc = "RTC Time Tick Register"]
pub mod rtc_tick;
#[doc = "RTC_TAMSK register accessor: an alias for `Reg<RTC_TAMSK_SPEC>`"]
pub type RTC_TAMSK = crate::Reg<rtc_tamsk::RTC_TAMSK_SPEC>;
#[doc = "RTC Time Alarm Mask Register"]
pub mod rtc_tamsk;
#[doc = "RTC_CAMSK register accessor: an alias for `Reg<RTC_CAMSK_SPEC>`"]
pub type RTC_CAMSK = crate::Reg<rtc_camsk::RTC_CAMSK_SPEC>;
#[doc = "RTC Calendar Alarm Mask Register"]
pub mod rtc_camsk;
#[doc = "RTC_LXTCTL register accessor: an alias for `Reg<RTC_LXTCTL_SPEC>`"]
pub type RTC_LXTCTL = crate::Reg<rtc_lxtctl::RTC_LXTCTL_SPEC>;
#[doc = "RTC 32K.768 KHz LXT Control Register"]
pub mod rtc_lxtctl;
