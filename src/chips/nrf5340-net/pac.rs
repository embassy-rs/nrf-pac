#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (828b7b8 2025-09-01))"]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "5 - CLOCK_POWER"]
    CLOCK_POWER = 5,
    #[doc = "8 - RADIO"]
    RADIO = 8,
    #[doc = "9 - RNG"]
    RNG = 9,
    #[doc = "10 - GPIOTE"]
    GPIOTE = 10,
    #[doc = "11 - WDT"]
    WDT = 11,
    #[doc = "12 - TIMER0"]
    TIMER0 = 12,
    #[doc = "13 - ECB"]
    ECB = 13,
    #[doc = "14 - AAR_CCM"]
    AAR_CCM = 14,
    #[doc = "16 - TEMP"]
    TEMP = 16,
    #[doc = "17 - RTC0"]
    RTC0 = 17,
    #[doc = "18 - IPC"]
    IPC = 18,
    #[doc = "19 - SERIAL0"]
    SERIAL0 = 19,
    #[doc = "20 - EGU0"]
    EGU0 = 20,
    #[doc = "22 - RTC1"]
    RTC1 = 22,
    #[doc = "24 - TIMER1"]
    TIMER1 = 24,
    #[doc = "25 - TIMER2"]
    TIMER2 = 25,
    #[doc = "26 - SWI0"]
    SWI0 = 26,
    #[doc = "27 - SWI1"]
    SWI1 = 27,
    #[doc = "28 - SWI2"]
    SWI2 = 28,
    #[doc = "29 - SWI3"]
    SWI3 = 29,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors {
    unsafe extern "C" {
        fn CLOCK_POWER();
        fn RADIO();
        fn RNG();
        fn GPIOTE();
        fn WDT();
        fn TIMER0();
        fn ECB();
        fn AAR_CCM();
        fn TEMP();
        fn RTC0();
        fn IPC();
        fn SERIAL0();
        fn EGU0();
        fn RTC1();
        fn TIMER1();
        fn TIMER2();
        fn SWI0();
        fn SWI1();
        fn SWI2();
        fn SWI3();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 30] = [
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector {
            _handler: CLOCK_POWER,
        },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: RADIO },
        Vector { _handler: RNG },
        Vector { _handler: GPIOTE },
        Vector { _handler: WDT },
        Vector { _handler: TIMER0 },
        Vector { _handler: ECB },
        Vector { _handler: AAR_CCM },
        Vector { _reserved: 0 },
        Vector { _handler: TEMP },
        Vector { _handler: RTC0 },
        Vector { _handler: IPC },
        Vector { _handler: SERIAL0 },
        Vector { _handler: EGU0 },
        Vector { _reserved: 0 },
        Vector { _handler: RTC1 },
        Vector { _reserved: 0 },
        Vector { _handler: TIMER1 },
        Vector { _handler: TIMER2 },
        Vector { _handler: SWI0 },
        Vector { _handler: SWI1 },
        Vector { _handler: SWI2 },
        Vector { _handler: SWI3 },
    ];
}
#[doc = "Factory Information Configuration Registers"]
pub const FICR_NS: ficr::Ficr = unsafe { ficr::Ficr::from_ptr(0x01ff_0000usize as _) };
#[doc = "User Information Configuration Registers"]
pub const UICR_NS: uicr::Uicr = unsafe { uicr::Uicr::from_ptr(0x01ff_8000usize as _) };
#[doc = "MUTEX 0"]
pub const APPMUTEX_NS: mutex::Mutex = unsafe { mutex::Mutex::from_ptr(0x4003_0000usize as _) };
#[doc = "Domain configuration management"]
pub const DCNF_NS: dcnf::Dcnf = unsafe { dcnf::Dcnf::from_ptr(0x4100_0000usize as _) };
#[doc = "Voltage request control"]
pub const VREQCTRL_NS: vreqctrl::Vreqctrl =
    unsafe { vreqctrl::Vreqctrl::from_ptr(0x4100_4000usize as _) };
#[doc = "Clock management"]
pub const CLOCK_NS: clock::Clock = unsafe { clock::Clock::from_ptr(0x4100_5000usize as _) };
#[doc = "Power control"]
pub const POWER_NS: power::Power = unsafe { power::Power::from_ptr(0x4100_5000usize as _) };
#[doc = "Reset control"]
pub const RESET_NS: reset::Reset = unsafe { reset::Reset::from_ptr(0x4100_5000usize as _) };
#[doc = "Control access port"]
pub const CTRLAP_NS: ctrlapperi::Ctrlapperi =
    unsafe { ctrlapperi::Ctrlapperi::from_ptr(0x4100_6000usize as _) };
#[doc = "2.4 GHz radio"]
pub const RADIO_NS: radio::Radio = unsafe { radio::Radio::from_ptr(0x4100_8000usize as _) };
#[doc = "Random Number Generator"]
pub const RNG_NS: rng::Rng = unsafe { rng::Rng::from_ptr(0x4100_9000usize as _) };
#[doc = "GPIO Tasks and Events"]
pub const GPIOTE_NS: gpiote::Gpiote = unsafe { gpiote::Gpiote::from_ptr(0x4100_a000usize as _) };
#[doc = "Watchdog Timer"]
pub const WDT_NS: wdt::Wdt = unsafe { wdt::Wdt::from_ptr(0x4100_b000usize as _) };
#[doc = "Timer/Counter 0"]
pub const TIMER0_NS: timer::Timer = unsafe { timer::Timer::from_ptr(0x4100_c000usize as _) };
#[doc = "AES ECB Mode Encryption"]
pub const ECB_NS: ecb::Ecb = unsafe { ecb::Ecb::from_ptr(0x4100_d000usize as _) };
#[doc = "Accelerated Address Resolver"]
pub const AAR_NS: aar::Aar = unsafe { aar::Aar::from_ptr(0x4100_e000usize as _) };
#[doc = "AES CCM mode encryption"]
pub const CCM_NS: ccm::Ccm = unsafe { ccm::Ccm::from_ptr(0x4100_e000usize as _) };
#[doc = "Distributed programmable peripheral interconnect controller"]
pub const DPPIC_NS: dppic::Dppic = unsafe { dppic::Dppic::from_ptr(0x4100_f000usize as _) };
#[doc = "Temperature Sensor"]
pub const TEMP_NS: temp::Temp = unsafe { temp::Temp::from_ptr(0x4101_0000usize as _) };
#[doc = "Real-time counter 0"]
pub const RTC0_NS: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4101_1000usize as _) };
#[doc = "Interprocessor communication"]
pub const IPC_NS: ipc::Ipc = unsafe { ipc::Ipc::from_ptr(0x4101_2000usize as _) };
#[doc = "Serial Peripheral Interface Master with EasyDMA"]
pub const SPIM0_NS: spim::Spim = unsafe { spim::Spim::from_ptr(0x4101_3000usize as _) };
#[doc = "SPI Slave"]
pub const SPIS0_NS: spis::Spis = unsafe { spis::Spis::from_ptr(0x4101_3000usize as _) };
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA"]
pub const TWIM0_NS: twim::Twim = unsafe { twim::Twim::from_ptr(0x4101_3000usize as _) };
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA"]
pub const TWIS0_NS: twis::Twis = unsafe { twis::Twis::from_ptr(0x4101_3000usize as _) };
#[doc = "UART with EasyDMA"]
pub const UARTE0_NS: uarte::Uarte = unsafe { uarte::Uarte::from_ptr(0x4101_3000usize as _) };
#[doc = "Event generator unit"]
pub const EGU0_NS: egu::Egu = unsafe { egu::Egu::from_ptr(0x4101_4000usize as _) };
#[doc = "Real-time counter 1"]
pub const RTC1_NS: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4101_6000usize as _) };
#[doc = "Timer/Counter 1"]
pub const TIMER1_NS: timer::Timer = unsafe { timer::Timer::from_ptr(0x4101_8000usize as _) };
#[doc = "Timer/Counter 2"]
pub const TIMER2_NS: timer::Timer = unsafe { timer::Timer::from_ptr(0x4101_9000usize as _) };
#[doc = "Software interrupt 0"]
pub const SWI0_NS: swi::Swi = unsafe { swi::Swi::from_ptr(0x4101_a000usize as _) };
#[doc = "Software interrupt 1"]
pub const SWI1_NS: swi::Swi = unsafe { swi::Swi::from_ptr(0x4101_b000usize as _) };
#[doc = "Software interrupt 2"]
pub const SWI2_NS: swi::Swi = unsafe { swi::Swi::from_ptr(0x4101_c000usize as _) };
#[doc = "Software interrupt 3"]
pub const SWI3_NS: swi::Swi = unsafe { swi::Swi::from_ptr(0x4101_d000usize as _) };
#[doc = "Access control lists"]
pub const ACL_NS: acl::Acl = unsafe { acl::Acl::from_ptr(0x4108_0000usize as _) };
#[doc = "Non-volatile memory controller"]
pub const NVMC_NS: nvmc::Nvmc = unsafe { nvmc::Nvmc::from_ptr(0x4108_0000usize as _) };
#[doc = "Volatile Memory controller"]
pub const VMC_NS: vmc::Vmc = unsafe { vmc::Vmc::from_ptr(0x4108_1000usize as _) };
#[doc = "GPIO Port 0"]
pub const P0_NS: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x418c_0500usize as _) };
#[doc = "GPIO Port 1"]
pub const P1_NS: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x418c_0800usize as _) };
#[doc = "MUTEX 1"]
pub const APPMUTEX_S: mutex::Mutex = unsafe { mutex::Mutex::from_ptr(0x5003_0000usize as _) };
#[doc = "Cross-Trigger Interface control. NOTE: this is not a separate peripheral, but describes CM33 functionality."]
pub const CTI_NS: cti::Cti = unsafe { cti::Cti::from_ptr(0xe004_2000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub mod aar {
    #[doc = "Accelerated Address Resolver"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Aar {
        ptr: *mut u8,
    }
    unsafe impl Send for Aar {}
    unsafe impl Sync for Aar {}
    impl Aar {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start resolving addresses based on IRKs specified in the IRK data structure"]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Stop resolving addresses"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "Subscribe configuration for task START"]
        #[inline(always)]
        pub const fn subscribe_start(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
        }
        #[doc = "Subscribe configuration for task STOP"]
        #[inline(always)]
        pub const fn subscribe_stop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
        }
        #[doc = "Address resolution procedure complete"]
        #[inline(always)]
        pub const fn events_end(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
        }
        #[doc = "Address resolved"]
        #[inline(always)]
        pub const fn events_resolved(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
        }
        #[doc = "Address not resolved"]
        #[inline(always)]
        pub const fn events_notresolved(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
        }
        #[doc = "Publish configuration for event END"]
        #[inline(always)]
        pub const fn publish_end(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
        }
        #[doc = "Publish configuration for event RESOLVED"]
        #[inline(always)]
        pub const fn publish_resolved(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
        }
        #[doc = "Publish configuration for event NOTRESOLVED"]
        #[inline(always)]
        pub const fn publish_notresolved(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
        }
        #[doc = "Resolution status"]
        #[inline(always)]
        pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
        }
        #[doc = "Enable AAR"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
        }
        #[doc = "Number of IRKs"]
        #[inline(always)]
        pub const fn nirk(self) -> crate::common::Reg<regs::Nirk, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
        }
        #[doc = "Pointer to IRK data structure"]
        #[inline(always)]
        pub const fn irkptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
        }
        #[doc = "Pointer to the resolvable address"]
        #[inline(always)]
        pub const fn addrptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0510usize) as _) }
        }
        #[doc = "Pointer to data area used for temporary storage"]
        #[inline(always)]
        pub const fn scratchptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0514usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Enable AAR"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable AAR"]
            #[must_use]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable AAR"]
            #[inline(always)]
            pub const fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        impl core::fmt::Debug for Enable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Enable")
                    .field("enable", &self.enable())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Enable {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Enable {{ enable: {:?} }}", self.enable())
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event END"]
            #[must_use]
            #[inline(always)]
            pub const fn end(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event END"]
            #[inline(always)]
            pub const fn set_end(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to disable interrupt for event RESOLVED"]
            #[must_use]
            #[inline(always)]
            pub const fn resolved(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event RESOLVED"]
            #[inline(always)]
            pub const fn set_resolved(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to disable interrupt for event NOTRESOLVED"]
            #[must_use]
            #[inline(always)]
            pub const fn notresolved(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event NOTRESOLVED"]
            #[inline(always)]
            pub const fn set_notresolved(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("end", &self.end())
                    .field("resolved", &self.resolved())
                    .field("notresolved", &self.notresolved())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Int {{ end: {=bool:?}, resolved: {=bool:?}, notresolved: {=bool:?} }}",
                    self.end(),
                    self.resolved(),
                    self.notresolved()
                )
            }
        }
        #[doc = "Number of IRKs"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Nirk(pub u32);
        impl Nirk {
            #[doc = "Number of Identity Root Keys available in the IRK data structure"]
            #[must_use]
            #[inline(always)]
            pub const fn nirk(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x1f;
                val as u8
            }
            #[doc = "Number of Identity Root Keys available in the IRK data structure"]
            #[inline(always)]
            pub const fn set_nirk(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
            }
        }
        impl Default for Nirk {
            #[inline(always)]
            fn default() -> Nirk {
                Nirk(0)
            }
        }
        impl core::fmt::Debug for Nirk {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Nirk").field("nirk", &self.nirk()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Nirk {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Nirk {{ nirk: {=u8:?} }}", self.nirk())
            }
        }
        #[doc = "Resolution status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Status(pub u32);
        impl Status {
            #[doc = "The IRK that was used last time an address was resolved"]
            #[must_use]
            #[inline(always)]
            pub const fn status(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "The IRK that was used last time an address was resolved"]
            #[inline(always)]
            pub const fn set_status(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Status {
            #[inline(always)]
            fn default() -> Status {
                Status(0)
            }
        }
        impl core::fmt::Debug for Status {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Status")
                    .field("status", &self.status())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Status {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Status {{ status: {=u8:?} }}", self.status())
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Enable {
            #[doc = "Disable"]
            DISABLED = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            #[doc = "Enable"]
            ENABLED = 0x03,
        }
        impl Enable {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Enable {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Enable {
            #[inline(always)]
            fn from(val: u8) -> Enable {
                Enable::from_bits(val)
            }
        }
        impl From<Enable> for u8 {
            #[inline(always)]
            fn from(val: Enable) -> u8 {
                Enable::to_bits(val)
            }
        }
    }
}
pub mod acl {
    #[doc = "Access control lists"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acl {
        ptr: *mut u8,
    }
    unsafe impl Send for Acl {}
    unsafe impl Sync for Acl {}
    impl Acl {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn acl(self, n: usize) -> AclAcl {
            assert!(n < 8usize);
            unsafe { AclAcl::from_ptr(self.ptr.wrapping_add(0x0800usize + n * 16usize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AclAcl {
        ptr: *mut u8,
    }
    unsafe impl Send for AclAcl {}
    unsafe impl Sync for AclAcl {}
    impl AclAcl {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster: Start address of region to protect. The start address must be word-aligned."]
        #[inline(always)]
        pub const fn addr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Description cluster: Size of region to protect counting from address ACL\\[n\\].ADDR. Writing a '0' has no effect."]
        #[inline(always)]
        pub const fn size(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Description cluster: Access permissions for region n as defined by start address ACL\\[n\\].ADDR and size ACL\\[n\\].SIZE"]
        #[inline(always)]
        pub const fn perm(self) -> crate::common::Reg<regs::Perm, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Description cluster: Access permissions for region n as defined by start address ACL\\[n\\].ADDR and size ACL\\[n\\].SIZE"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Perm(pub u32);
        impl Perm {
            #[doc = "Configure write and erase permissions for region n. Writing a '0' has no effect."]
            #[must_use]
            #[inline(always)]
            pub const fn write(&self) -> super::vals::Write {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Write::from_bits(val as u8)
            }
            #[doc = "Configure write and erase permissions for region n. Writing a '0' has no effect."]
            #[inline(always)]
            pub const fn set_write(&mut self, val: super::vals::Write) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Configure read permissions for region n. Writing a '0' has no effect."]
            #[must_use]
            #[inline(always)]
            pub const fn read(&self) -> super::vals::Read {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Read::from_bits(val as u8)
            }
            #[doc = "Configure read permissions for region n. Writing a '0' has no effect."]
            #[inline(always)]
            pub const fn set_read(&mut self, val: super::vals::Read) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Perm {
            #[inline(always)]
            fn default() -> Perm {
                Perm(0)
            }
        }
        impl core::fmt::Debug for Perm {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Perm")
                    .field("write", &self.write())
                    .field("read", &self.read())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Perm {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Perm {{ write: {:?}, read: {:?} }}",
                    self.write(),
                    self.read()
                )
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Read {
            #[doc = "Allow read instructions to region n."]
            ENABLE = 0x0,
            #[doc = "Block read instructions to region n."]
            DISABLE = 0x01,
        }
        impl Read {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Read {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Read {
            #[inline(always)]
            fn from(val: u8) -> Read {
                Read::from_bits(val)
            }
        }
        impl From<Read> for u8 {
            #[inline(always)]
            fn from(val: Read) -> u8 {
                Read::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Write {
            #[doc = "Allow write and erase instructions to region n."]
            ENABLE = 0x0,
            #[doc = "Block write and erase instructions to region n."]
            DISABLE = 0x01,
        }
        impl Write {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Write {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Write {
            #[inline(always)]
            fn from(val: u8) -> Write {
                Write::from_bits(val)
            }
        }
        impl From<Write> for u8 {
            #[inline(always)]
            fn from(val: Write) -> u8 {
                Write::to_bits(val)
            }
        }
    }
}
pub mod ccm {
    #[doc = "AES CCM mode encryption"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccm {
        ptr: *mut u8,
    }
    unsafe impl Send for Ccm {}
    unsafe impl Sync for Ccm {}
    impl Ccm {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start generation of keystream. This operation will stop by itself when completed."]
        #[inline(always)]
        pub const fn tasks_ksgen(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Start encryption/decryption. This operation will stop by itself when completed."]
        #[inline(always)]
        pub const fn tasks_crypt(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Stop encryption/decryption"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "Override DATARATE setting in MODE register with the contents of the RATEOVERRIDE register for any ongoing encryption/decryption"]
        #[inline(always)]
        pub const fn tasks_rateoverride(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
        #[doc = "Subscribe configuration for task KSGEN"]
        #[inline(always)]
        pub const fn subscribe_ksgen(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
        }
        #[doc = "Subscribe configuration for task CRYPT"]
        #[inline(always)]
        pub const fn subscribe_crypt(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
        }
        #[doc = "Subscribe configuration for task STOP"]
        #[inline(always)]
        pub const fn subscribe_stop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
        }
        #[doc = "Subscribe configuration for task RATEOVERRIDE"]
        #[inline(always)]
        pub const fn subscribe_rateoverride(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
        }
        #[doc = "Keystream generation complete"]
        #[inline(always)]
        pub const fn events_endksgen(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
        }
        #[doc = "Encrypt/decrypt complete"]
        #[inline(always)]
        pub const fn events_endcrypt(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
        }
        #[doc = "Deprecated register - CCM error event"]
        #[inline(always)]
        pub const fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
        }
        #[doc = "Publish configuration for event ENDKSGEN"]
        #[inline(always)]
        pub const fn publish_endksgen(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
        }
        #[doc = "Publish configuration for event ENDCRYPT"]
        #[inline(always)]
        pub const fn publish_endcrypt(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
        }
        #[doc = "Deprecated register - Publish configuration for event ERROR"]
        #[inline(always)]
        pub const fn publish_error(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
        }
        #[doc = "MIC check result"]
        #[inline(always)]
        pub const fn micstatus(self) -> crate::common::Reg<regs::Micstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
        }
        #[doc = "Enable"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
        }
        #[doc = "Operation mode"]
        #[inline(always)]
        pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
        }
        #[doc = "Pointer to data structure holding the AES key and the NONCE vector"]
        #[inline(always)]
        pub const fn cnfptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
        }
        #[doc = "Input pointer"]
        #[inline(always)]
        pub const fn inptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x050cusize) as _) }
        }
        #[doc = "Output pointer"]
        #[inline(always)]
        pub const fn outptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0510usize) as _) }
        }
        #[doc = "Pointer to data area used for temporary storage"]
        #[inline(always)]
        pub const fn scratchptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0514usize) as _) }
        }
        #[doc = "Length of keystream generated when MODE.LENGTH = Extended"]
        #[inline(always)]
        pub const fn maxpacketsize(
            self,
        ) -> crate::common::Reg<regs::Maxpacketsize, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0518usize) as _) }
        }
        #[doc = "Data rate override setting."]
        #[inline(always)]
        pub const fn rateoverride(
            self,
        ) -> crate::common::Reg<regs::Rateoverride, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x051cusize) as _) }
        }
        #[doc = "Header (S0) mask."]
        #[inline(always)]
        pub const fn headermask(self) -> crate::common::Reg<regs::Headermask, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0520usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Enable"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable CCM"]
            #[must_use]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable CCM"]
            #[inline(always)]
            pub const fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        impl core::fmt::Debug for Enable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Enable")
                    .field("enable", &self.enable())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Enable {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Enable {{ enable: {:?} }}", self.enable())
            }
        }
        #[doc = "Header (S0) mask."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Headermask(pub u32);
        impl Headermask {
            #[doc = "Header (S0) mask"]
            #[must_use]
            #[inline(always)]
            pub const fn headermask(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Header (S0) mask"]
            #[inline(always)]
            pub const fn set_headermask(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Headermask {
            #[inline(always)]
            fn default() -> Headermask {
                Headermask(0)
            }
        }
        impl core::fmt::Debug for Headermask {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Headermask")
                    .field("headermask", &self.headermask())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Headermask {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Headermask {{ headermask: {=u8:?} }}", self.headermask())
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event ENDKSGEN"]
            #[must_use]
            #[inline(always)]
            pub const fn endksgen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ENDKSGEN"]
            #[inline(always)]
            pub const fn set_endksgen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to disable interrupt for event ENDCRYPT"]
            #[must_use]
            #[inline(always)]
            pub const fn endcrypt(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ENDCRYPT"]
            #[inline(always)]
            pub const fn set_endcrypt(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Deprecated intclrfield - Write '1' to disable interrupt for event ERROR"]
            #[must_use]
            #[inline(always)]
            pub const fn error(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Deprecated intclrfield - Write '1' to disable interrupt for event ERROR"]
            #[inline(always)]
            pub const fn set_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("endksgen", &self.endksgen())
                    .field("endcrypt", &self.endcrypt())
                    .field("error", &self.error())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Int {{ endksgen: {=bool:?}, endcrypt: {=bool:?}, error: {=bool:?} }}",
                    self.endksgen(),
                    self.endcrypt(),
                    self.error()
                )
            }
        }
        #[doc = "Length of keystream generated when MODE.LENGTH = Extended"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Maxpacketsize(pub u32);
        impl Maxpacketsize {
            #[doc = "Length of keystream generated when MODE.LENGTH = Extended. This value must be greater than or equal to the subsequent packet payload to be encrypted/decrypted."]
            #[must_use]
            #[inline(always)]
            pub const fn maxpacketsize(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Length of keystream generated when MODE.LENGTH = Extended. This value must be greater than or equal to the subsequent packet payload to be encrypted/decrypted."]
            #[inline(always)]
            pub const fn set_maxpacketsize(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Maxpacketsize {
            #[inline(always)]
            fn default() -> Maxpacketsize {
                Maxpacketsize(0)
            }
        }
        impl core::fmt::Debug for Maxpacketsize {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Maxpacketsize")
                    .field("maxpacketsize", &self.maxpacketsize())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Maxpacketsize {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Maxpacketsize {{ maxpacketsize: {=u8:?} }}",
                    self.maxpacketsize()
                )
            }
        }
        #[doc = "MIC check result"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Micstatus(pub u32);
        impl Micstatus {
            #[doc = "The result of the MIC check performed during the previous decryption operation"]
            #[must_use]
            #[inline(always)]
            pub const fn micstatus(&self) -> super::vals::Micstatus {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Micstatus::from_bits(val as u8)
            }
            #[doc = "The result of the MIC check performed during the previous decryption operation"]
            #[inline(always)]
            pub const fn set_micstatus(&mut self, val: super::vals::Micstatus) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Micstatus {
            #[inline(always)]
            fn default() -> Micstatus {
                Micstatus(0)
            }
        }
        impl core::fmt::Debug for Micstatus {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Micstatus")
                    .field("micstatus", &self.micstatus())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Micstatus {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Micstatus {{ micstatus: {:?} }}", self.micstatus())
            }
        }
        #[doc = "Operation mode"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Mode(pub u32);
        impl Mode {
            #[doc = "The mode of operation to be used. Settings in this register apply whenever either the KSGEN task or the CRYPT task is triggered."]
            #[must_use]
            #[inline(always)]
            pub const fn mode(&self) -> super::vals::Mode {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Mode::from_bits(val as u8)
            }
            #[doc = "The mode of operation to be used. Settings in this register apply whenever either the KSGEN task or the CRYPT task is triggered."]
            #[inline(always)]
            pub const fn set_mode(&mut self, val: super::vals::Mode) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Radio data rate that the CCM shall run synchronous with"]
            #[must_use]
            #[inline(always)]
            pub const fn datarate(&self) -> super::vals::Datarate {
                let val = (self.0 >> 16usize) & 0x03;
                super::vals::Datarate::from_bits(val as u8)
            }
            #[doc = "Radio data rate that the CCM shall run synchronous with"]
            #[inline(always)]
            pub const fn set_datarate(&mut self, val: super::vals::Datarate) {
                self.0 =
                    (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
            }
            #[doc = "Packet length configuration"]
            #[must_use]
            #[inline(always)]
            pub const fn length(&self) -> super::vals::Length {
                let val = (self.0 >> 24usize) & 0x01;
                super::vals::Length::from_bits(val as u8)
            }
            #[doc = "Packet length configuration"]
            #[inline(always)]
            pub const fn set_length(&mut self, val: super::vals::Length) {
                self.0 =
                    (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
            }
        }
        impl Default for Mode {
            #[inline(always)]
            fn default() -> Mode {
                Mode(0)
            }
        }
        impl core::fmt::Debug for Mode {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Mode")
                    .field("mode", &self.mode())
                    .field("datarate", &self.datarate())
                    .field("length", &self.length())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Mode {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Mode {{ mode: {:?}, datarate: {:?}, length: {:?} }}",
                    self.mode(),
                    self.datarate(),
                    self.length()
                )
            }
        }
        #[doc = "Data rate override setting."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rateoverride(pub u32);
        impl Rateoverride {
            #[doc = "Data rate override setting"]
            #[must_use]
            #[inline(always)]
            pub const fn rateoverride(&self) -> super::vals::Rateoverride {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Rateoverride::from_bits(val as u8)
            }
            #[doc = "Data rate override setting"]
            #[inline(always)]
            pub const fn set_rateoverride(&mut self, val: super::vals::Rateoverride) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Rateoverride {
            #[inline(always)]
            fn default() -> Rateoverride {
                Rateoverride(0)
            }
        }
        impl core::fmt::Debug for Rateoverride {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rateoverride")
                    .field("rateoverride", &self.rateoverride())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rateoverride {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Rateoverride {{ rateoverride: {:?} }}",
                    self.rateoverride()
                )
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event ENDKSGEN and task CRYPT"]
            #[must_use]
            #[inline(always)]
            pub const fn endksgen_crypt(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event ENDKSGEN and task CRYPT"]
            #[inline(always)]
            pub const fn set_endksgen_crypt(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        impl core::fmt::Debug for Shorts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Shorts")
                    .field("endksgen_crypt", &self.endksgen_crypt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Shorts {{ endksgen_crypt: {=bool:?} }}",
                    self.endksgen_crypt()
                )
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Datarate {
            #[doc = "1 Mbps"]
            _1MBIT = 0x0,
            #[doc = "2 Mbps"]
            _2MBIT = 0x01,
            #[doc = "125 kbps"]
            _125KBPS = 0x02,
            #[doc = "500 kbps"]
            _500KBPS = 0x03,
        }
        impl Datarate {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Datarate {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Datarate {
            #[inline(always)]
            fn from(val: u8) -> Datarate {
                Datarate::from_bits(val)
            }
        }
        impl From<Datarate> for u8 {
            #[inline(always)]
            fn from(val: Datarate) -> u8 {
                Datarate::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Enable {
            #[doc = "Disable"]
            DISABLED = 0x0,
            _RESERVED_1 = 0x01,
            #[doc = "Enable"]
            ENABLED = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Enable {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Enable {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Enable {
            #[inline(always)]
            fn from(val: u8) -> Enable {
                Enable::from_bits(val)
            }
        }
        impl From<Enable> for u8 {
            #[inline(always)]
            fn from(val: Enable) -> u8 {
                Enable::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Length {
            #[doc = "Default length. Effective length of LENGTH field in encrypted/decrypted packet is 5 bits. A keystream for packet payloads up to 27 bytes will be generated."]
            DEFAULT = 0x0,
            #[doc = "Extended length. Effective length of LENGTH field in encrypted/decrypted packet is 8 bits. A keystream for packet payloads up to MAXPACKETSIZE bytes will be generated."]
            EXTENDED = 0x01,
        }
        impl Length {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Length {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Length {
            #[inline(always)]
            fn from(val: u8) -> Length {
                Length::from_bits(val)
            }
        }
        impl From<Length> for u8 {
            #[inline(always)]
            fn from(val: Length) -> u8 {
                Length::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Micstatus {
            #[doc = "MIC check failed"]
            CHECK_FAILED = 0x0,
            #[doc = "MIC check passed"]
            CHECK_PASSED = 0x01,
        }
        impl Micstatus {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Micstatus {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Micstatus {
            #[inline(always)]
            fn from(val: u8) -> Micstatus {
                Micstatus::from_bits(val)
            }
        }
        impl From<Micstatus> for u8 {
            #[inline(always)]
            fn from(val: Micstatus) -> u8 {
                Micstatus::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Mode {
            #[doc = "AES CCM packet encryption mode"]
            ENCRYPTION = 0x0,
            #[doc = "AES CCM packet decryption mode"]
            DECRYPTION = 0x01,
        }
        impl Mode {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Mode {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Mode {
            #[inline(always)]
            fn from(val: u8) -> Mode {
                Mode::from_bits(val)
            }
        }
        impl From<Mode> for u8 {
            #[inline(always)]
            fn from(val: Mode) -> u8 {
                Mode::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rateoverride {
            #[doc = "1 Mbps"]
            _1MBIT = 0x0,
            #[doc = "2 Mbps"]
            _2MBIT = 0x01,
            #[doc = "125 kbps"]
            _125KBPS = 0x02,
            #[doc = "500 kbps"]
            _500KBPS = 0x03,
        }
        impl Rateoverride {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rateoverride {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rateoverride {
            #[inline(always)]
            fn from(val: u8) -> Rateoverride {
                Rateoverride::from_bits(val)
            }
        }
        impl From<Rateoverride> for u8 {
            #[inline(always)]
            fn from(val: Rateoverride) -> u8 {
                Rateoverride::to_bits(val)
            }
        }
    }
}
pub mod clock {
    #[doc = "Clock management"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clock {
        ptr: *mut u8,
    }
    unsafe impl Send for Clock {}
    unsafe impl Sync for Clock {}
    impl Clock {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start HFCLK128M/HFCLK64M source as selected in HFCLKSRC"]
        #[inline(always)]
        pub const fn tasks_hfclkstart(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Stop HFCLK128M/HFCLK64M source"]
        #[inline(always)]
        pub const fn tasks_hfclkstop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Start LFCLK source as selected in LFCLKSRC"]
        #[inline(always)]
        pub const fn tasks_lfclkstart(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "Stop LFCLK source"]
        #[inline(always)]
        pub const fn tasks_lfclkstop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
        #[doc = "Start calibration of LFRC oscillator"]
        #[inline(always)]
        pub const fn tasks_cal(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
        }
        #[doc = "Subscribe configuration for task HFCLKSTART"]
        #[inline(always)]
        pub const fn subscribe_hfclkstart(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
        }
        #[doc = "Subscribe configuration for task HFCLKSTOP"]
        #[inline(always)]
        pub const fn subscribe_hfclkstop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
        }
        #[doc = "Subscribe configuration for task LFCLKSTART"]
        #[inline(always)]
        pub const fn subscribe_lfclkstart(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
        }
        #[doc = "Subscribe configuration for task LFCLKSTOP"]
        #[inline(always)]
        pub const fn subscribe_lfclkstop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
        }
        #[doc = "Subscribe configuration for task CAL"]
        #[inline(always)]
        pub const fn subscribe_cal(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
        }
        #[doc = "HFCLK128M/HFCLK64M source started"]
        #[inline(always)]
        pub const fn events_hfclkstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
        }
        #[doc = "LFCLK source started"]
        #[inline(always)]
        pub const fn events_lfclkstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
        }
        #[doc = "Calibration of LFRC oscillator complete event"]
        #[inline(always)]
        pub const fn events_done(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
        }
        #[doc = "Publish configuration for event HFCLKSTARTED"]
        #[inline(always)]
        pub const fn publish_hfclkstarted(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
        }
        #[doc = "Publish configuration for event LFCLKSTARTED"]
        #[inline(always)]
        pub const fn publish_lfclkstarted(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
        }
        #[doc = "Publish configuration for event DONE"]
        #[inline(always)]
        pub const fn publish_done(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
        }
        #[doc = "Pending interrupts"]
        #[inline(always)]
        pub const fn intpend(self) -> crate::common::Reg<regs::Int, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x030cusize) as _) }
        }
        #[doc = "Status indicating that HFCLKSTART task has been triggered"]
        #[inline(always)]
        pub const fn hfclkrun(self) -> crate::common::Reg<regs::Hfclkrun, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0408usize) as _) }
        }
        #[doc = "Status indicating which HFCLK128M/HFCLK64M source is running This register value in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance."]
        #[inline(always)]
        pub const fn hfclkstat(self) -> crate::common::Reg<regs::Hfclkstat, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x040cusize) as _) }
        }
        #[doc = "Status indicating that LFCLKSTART task has been triggered"]
        #[inline(always)]
        pub const fn lfclkrun(self) -> crate::common::Reg<regs::Lfclkrun, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0414usize) as _) }
        }
        #[doc = "Status indicating which LFCLK source is running This register value in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance."]
        #[inline(always)]
        pub const fn lfclkstat(self) -> crate::common::Reg<regs::Lfclkstat, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0418usize) as _) }
        }
        #[doc = "Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
        #[inline(always)]
        pub const fn lfclksrccopy(
            self,
        ) -> crate::common::Reg<regs::Lfclksrccopy, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x041cusize) as _) }
        }
        #[doc = "Clock source for HFCLK128M/HFCLK64M"]
        #[inline(always)]
        pub const fn hfclksrc(self) -> crate::common::Reg<regs::Hfclksrc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0514usize) as _) }
        }
        #[doc = "Clock source for LFCLK"]
        #[inline(always)]
        pub const fn lfclksrc(self) -> crate::common::Reg<regs::Lfclksrc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0518usize) as _) }
        }
        #[doc = "HFCLK128M frequency configuration"]
        #[inline(always)]
        pub const fn hfclkctrl(self) -> crate::common::Reg<regs::Hfclkctrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0558usize) as _) }
        }
        #[doc = "Automatic or manual control of HFCLK128M/HFCLK64M"]
        #[inline(always)]
        pub const fn hfclkalwaysrun(
            self,
        ) -> crate::common::Reg<regs::Hfclkalwaysrun, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0570usize) as _) }
        }
        #[doc = "Automatic or manual control of LFCLK"]
        #[inline(always)]
        pub const fn lfclkalwaysrun(
            self,
        ) -> crate::common::Reg<regs::Lfclkalwaysrun, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0574usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Automatic or manual control of HFCLK128M/HFCLK64M"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Hfclkalwaysrun(pub u32);
        impl Hfclkalwaysrun {
            #[doc = "Ensure clock is always running"]
            #[must_use]
            #[inline(always)]
            pub const fn alwaysrun(&self) -> super::vals::HfclkalwaysrunAlwaysrun {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::HfclkalwaysrunAlwaysrun::from_bits(val as u8)
            }
            #[doc = "Ensure clock is always running"]
            #[inline(always)]
            pub const fn set_alwaysrun(&mut self, val: super::vals::HfclkalwaysrunAlwaysrun) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Hfclkalwaysrun {
            #[inline(always)]
            fn default() -> Hfclkalwaysrun {
                Hfclkalwaysrun(0)
            }
        }
        impl core::fmt::Debug for Hfclkalwaysrun {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Hfclkalwaysrun")
                    .field("alwaysrun", &self.alwaysrun())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Hfclkalwaysrun {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Hfclkalwaysrun {{ alwaysrun: {:?} }}", self.alwaysrun())
            }
        }
        #[doc = "HFCLK128M frequency configuration"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Hfclkctrl(pub u32);
        impl Hfclkctrl {
            #[doc = "High frequency clock HCLK"]
            #[must_use]
            #[inline(always)]
            pub const fn hclk(&self) -> super::vals::Hclk {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Hclk::from_bits(val as u8)
            }
            #[doc = "High frequency clock HCLK"]
            #[inline(always)]
            pub const fn set_hclk(&mut self, val: super::vals::Hclk) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Hfclkctrl {
            #[inline(always)]
            fn default() -> Hfclkctrl {
                Hfclkctrl(0)
            }
        }
        impl core::fmt::Debug for Hfclkctrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Hfclkctrl")
                    .field("hclk", &self.hclk())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Hfclkctrl {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Hfclkctrl {{ hclk: {:?} }}", self.hclk())
            }
        }
        #[doc = "Status indicating that HFCLKSTART task has been triggered"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Hfclkrun(pub u32);
        impl Hfclkrun {
            #[doc = "HFCLKSTART task triggered or not"]
            #[must_use]
            #[inline(always)]
            pub const fn status(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "HFCLKSTART task triggered or not"]
            #[inline(always)]
            pub const fn set_status(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Hfclkrun {
            #[inline(always)]
            fn default() -> Hfclkrun {
                Hfclkrun(0)
            }
        }
        impl core::fmt::Debug for Hfclkrun {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Hfclkrun")
                    .field("status", &self.status())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Hfclkrun {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Hfclkrun {{ status: {=bool:?} }}", self.status())
            }
        }
        #[doc = "Clock source for HFCLK128M/HFCLK64M"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Hfclksrc(pub u32);
        impl Hfclksrc {
            #[doc = "Select which HFCLK source is started by the HFCLKSTART task"]
            #[must_use]
            #[inline(always)]
            pub const fn src(&self) -> super::vals::HfclksrcSrc {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::HfclksrcSrc::from_bits(val as u8)
            }
            #[doc = "Select which HFCLK source is started by the HFCLKSTART task"]
            #[inline(always)]
            pub const fn set_src(&mut self, val: super::vals::HfclksrcSrc) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Hfclksrc {
            #[inline(always)]
            fn default() -> Hfclksrc {
                Hfclksrc(0)
            }
        }
        impl core::fmt::Debug for Hfclksrc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Hfclksrc")
                    .field("src", &self.src())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Hfclksrc {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Hfclksrc {{ src: {:?} }}", self.src())
            }
        }
        #[doc = "Status indicating which HFCLK128M/HFCLK64M source is running This register value in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Hfclkstat(pub u32);
        impl Hfclkstat {
            #[doc = "Active clock source"]
            #[must_use]
            #[inline(always)]
            pub const fn src(&self) -> super::vals::HfclkstatSrc {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::HfclkstatSrc::from_bits(val as u8)
            }
            #[doc = "Active clock source"]
            #[inline(always)]
            pub const fn set_src(&mut self, val: super::vals::HfclkstatSrc) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "ALWAYSRUN activated"]
            #[must_use]
            #[inline(always)]
            pub const fn alwaysrunning(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "ALWAYSRUN activated"]
            #[inline(always)]
            pub const fn set_alwaysrunning(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "HFCLK state"]
            #[must_use]
            #[inline(always)]
            pub const fn state(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "HFCLK state"]
            #[inline(always)]
            pub const fn set_state(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
        }
        impl Default for Hfclkstat {
            #[inline(always)]
            fn default() -> Hfclkstat {
                Hfclkstat(0)
            }
        }
        impl core::fmt::Debug for Hfclkstat {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Hfclkstat")
                    .field("src", &self.src())
                    .field("alwaysrunning", &self.alwaysrunning())
                    .field("state", &self.state())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Hfclkstat {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Hfclkstat {{ src: {:?}, alwaysrunning: {=bool:?}, state: {=bool:?} }}",
                    self.src(),
                    self.alwaysrunning(),
                    self.state()
                )
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Enable or disable interrupt for event HFCLKSTARTED"]
            #[must_use]
            #[inline(always)]
            pub const fn hfclkstarted(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event HFCLKSTARTED"]
            #[inline(always)]
            pub const fn set_hfclkstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable interrupt for event LFCLKSTARTED"]
            #[must_use]
            #[inline(always)]
            pub const fn lfclkstarted(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event LFCLKSTARTED"]
            #[inline(always)]
            pub const fn set_lfclkstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for event DONE"]
            #[must_use]
            #[inline(always)]
            pub const fn done(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event DONE"]
            #[inline(always)]
            pub const fn set_done(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("hfclkstarted", &self.hfclkstarted())
                    .field("lfclkstarted", &self.lfclkstarted())
                    .field("done", &self.done())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Int {{ hfclkstarted: {=bool:?}, lfclkstarted: {=bool:?}, done: {=bool:?} }}",
                    self.hfclkstarted(),
                    self.lfclkstarted(),
                    self.done()
                )
            }
        }
        #[doc = "Automatic or manual control of LFCLK"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Lfclkalwaysrun(pub u32);
        impl Lfclkalwaysrun {
            #[doc = "Ensure clock is always running"]
            #[must_use]
            #[inline(always)]
            pub const fn alwaysrun(&self) -> super::vals::LfclkalwaysrunAlwaysrun {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::LfclkalwaysrunAlwaysrun::from_bits(val as u8)
            }
            #[doc = "Ensure clock is always running"]
            #[inline(always)]
            pub const fn set_alwaysrun(&mut self, val: super::vals::LfclkalwaysrunAlwaysrun) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Lfclkalwaysrun {
            #[inline(always)]
            fn default() -> Lfclkalwaysrun {
                Lfclkalwaysrun(0)
            }
        }
        impl core::fmt::Debug for Lfclkalwaysrun {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Lfclkalwaysrun")
                    .field("alwaysrun", &self.alwaysrun())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Lfclkalwaysrun {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Lfclkalwaysrun {{ alwaysrun: {:?} }}", self.alwaysrun())
            }
        }
        #[doc = "Status indicating that LFCLKSTART task has been triggered"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Lfclkrun(pub u32);
        impl Lfclkrun {
            #[doc = "LFCLKSTART task triggered or not"]
            #[must_use]
            #[inline(always)]
            pub const fn status(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "LFCLKSTART task triggered or not"]
            #[inline(always)]
            pub const fn set_status(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Lfclkrun {
            #[inline(always)]
            fn default() -> Lfclkrun {
                Lfclkrun(0)
            }
        }
        impl core::fmt::Debug for Lfclkrun {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Lfclkrun")
                    .field("status", &self.status())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Lfclkrun {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Lfclkrun {{ status: {=bool:?} }}", self.status())
            }
        }
        #[doc = "Clock source for LFCLK"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Lfclksrc(pub u32);
        impl Lfclksrc {
            #[doc = "Select which LFCLK source is started by the LFCLKSTART task"]
            #[must_use]
            #[inline(always)]
            pub const fn src(&self) -> super::vals::Lfclksrc {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Lfclksrc::from_bits(val as u8)
            }
            #[doc = "Select which LFCLK source is started by the LFCLKSTART task"]
            #[inline(always)]
            pub const fn set_src(&mut self, val: super::vals::Lfclksrc) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Lfclksrc {
            #[inline(always)]
            fn default() -> Lfclksrc {
                Lfclksrc(0)
            }
        }
        impl core::fmt::Debug for Lfclksrc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Lfclksrc")
                    .field("src", &self.src())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Lfclksrc {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Lfclksrc {{ src: {:?} }}", self.src())
            }
        }
        #[doc = "Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Lfclksrccopy(pub u32);
        impl Lfclksrccopy {
            #[doc = "Clock source"]
            #[must_use]
            #[inline(always)]
            pub const fn src(&self) -> super::vals::Lfclksrc {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Lfclksrc::from_bits(val as u8)
            }
            #[doc = "Clock source"]
            #[inline(always)]
            pub const fn set_src(&mut self, val: super::vals::Lfclksrc) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Lfclksrccopy {
            #[inline(always)]
            fn default() -> Lfclksrccopy {
                Lfclksrccopy(0)
            }
        }
        impl core::fmt::Debug for Lfclksrccopy {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Lfclksrccopy")
                    .field("src", &self.src())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Lfclksrccopy {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Lfclksrccopy {{ src: {:?} }}", self.src())
            }
        }
        #[doc = "Status indicating which LFCLK source is running This register value in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Lfclkstat(pub u32);
        impl Lfclkstat {
            #[doc = "Active clock source"]
            #[must_use]
            #[inline(always)]
            pub const fn src(&self) -> super::vals::Lfclksrc {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Lfclksrc::from_bits(val as u8)
            }
            #[doc = "Active clock source"]
            #[inline(always)]
            pub const fn set_src(&mut self, val: super::vals::Lfclksrc) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
            #[doc = "ALWAYSRUN activated"]
            #[must_use]
            #[inline(always)]
            pub const fn alwaysrunning(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "ALWAYSRUN activated"]
            #[inline(always)]
            pub const fn set_alwaysrunning(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "LFCLK state"]
            #[must_use]
            #[inline(always)]
            pub const fn state(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "LFCLK state"]
            #[inline(always)]
            pub const fn set_state(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
        }
        impl Default for Lfclkstat {
            #[inline(always)]
            fn default() -> Lfclkstat {
                Lfclkstat(0)
            }
        }
        impl core::fmt::Debug for Lfclkstat {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Lfclkstat")
                    .field("src", &self.src())
                    .field("alwaysrunning", &self.alwaysrunning())
                    .field("state", &self.state())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Lfclkstat {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Lfclkstat {{ src: {:?}, alwaysrunning: {=bool:?}, state: {=bool:?} }}",
                    self.src(),
                    self.alwaysrunning(),
                    self.state()
                )
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Hclk {
            #[doc = "Divide HFCLK by 1"]
            DIV1 = 0x0,
            #[doc = "Divide HFCLK by 2"]
            DIV2 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Hclk {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Hclk {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Hclk {
            #[inline(always)]
            fn from(val: u8) -> Hclk {
                Hclk::from_bits(val)
            }
        }
        impl From<Hclk> for u8 {
            #[inline(always)]
            fn from(val: Hclk) -> u8 {
                Hclk::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum HfclkalwaysrunAlwaysrun {
            #[doc = "Use automatic clock control"]
            AUTOMATIC = 0x0,
            #[doc = "Ensure clock is always running"]
            ALWAYS_RUN = 0x01,
        }
        impl HfclkalwaysrunAlwaysrun {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> HfclkalwaysrunAlwaysrun {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for HfclkalwaysrunAlwaysrun {
            #[inline(always)]
            fn from(val: u8) -> HfclkalwaysrunAlwaysrun {
                HfclkalwaysrunAlwaysrun::from_bits(val)
            }
        }
        impl From<HfclkalwaysrunAlwaysrun> for u8 {
            #[inline(always)]
            fn from(val: HfclkalwaysrunAlwaysrun) -> u8 {
                HfclkalwaysrunAlwaysrun::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum HfclksrcSrc {
            #[doc = "HFCLKSTART task starts HFINT oscillator"]
            HFINT = 0x0,
            #[doc = "HFCLKSTART task starts HFXO oscillator"]
            HFXO = 0x01,
        }
        impl HfclksrcSrc {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> HfclksrcSrc {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for HfclksrcSrc {
            #[inline(always)]
            fn from(val: u8) -> HfclksrcSrc {
                HfclksrcSrc::from_bits(val)
            }
        }
        impl From<HfclksrcSrc> for u8 {
            #[inline(always)]
            fn from(val: HfclksrcSrc) -> u8 {
                HfclksrcSrc::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum HfclkstatSrc {
            #[doc = "Clock source: HFINT - 128 MHz on-chip oscillator"]
            HFINT = 0x0,
            #[doc = "Clock source: HFXO - 128 MHz clock derived from external 32 MHz crystal oscillator"]
            HFXO = 0x01,
        }
        impl HfclkstatSrc {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> HfclkstatSrc {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for HfclkstatSrc {
            #[inline(always)]
            fn from(val: u8) -> HfclkstatSrc {
                HfclkstatSrc::from_bits(val)
            }
        }
        impl From<HfclkstatSrc> for u8 {
            #[inline(always)]
            fn from(val: HfclkstatSrc) -> u8 {
                HfclkstatSrc::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum LfclkalwaysrunAlwaysrun {
            #[doc = "Use automatic clock control"]
            AUTOMATIC = 0x0,
            #[doc = "Ensure clock is always running"]
            ALWAYS_RUN = 0x01,
        }
        impl LfclkalwaysrunAlwaysrun {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> LfclkalwaysrunAlwaysrun {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for LfclkalwaysrunAlwaysrun {
            #[inline(always)]
            fn from(val: u8) -> LfclkalwaysrunAlwaysrun {
                LfclkalwaysrunAlwaysrun::from_bits(val)
            }
        }
        impl From<LfclkalwaysrunAlwaysrun> for u8 {
            #[inline(always)]
            fn from(val: LfclkalwaysrunAlwaysrun) -> u8 {
                LfclkalwaysrunAlwaysrun::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Lfclksrc {
            _RESERVED_0 = 0x0,
            #[doc = "32.768 kHz RC oscillator"]
            LFRC = 0x01,
            #[doc = "32.768 kHz crystal oscillator"]
            LFXO = 0x02,
            #[doc = "32.768 kHz synthesized from HFCLK"]
            LFSYNT = 0x03,
        }
        impl Lfclksrc {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Lfclksrc {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Lfclksrc {
            #[inline(always)]
            fn from(val: u8) -> Lfclksrc {
                Lfclksrc::from_bits(val)
            }
        }
        impl From<Lfclksrc> for u8 {
            #[inline(always)]
            fn from(val: Lfclksrc) -> u8 {
                Lfclksrc::to_bits(val)
            }
        }
    }
}
pub mod common {
    use core::marker::PhantomData;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct RW;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct R;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct W;
    mod sealed {
        use super::*;
        pub trait Access {}
        impl Access for R {}
        impl Access for W {}
        impl Access for RW {}
    }
    pub trait Access: sealed::Access + Copy {}
    impl Access for R {}
    impl Access for W {}
    impl Access for RW {}
    pub trait Read: Access {}
    impl Read for RW {}
    impl Read for R {}
    pub trait Write: Access {}
    impl Write for RW {}
    impl Write for W {}
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct Reg<T: Copy, A: Access> {
        ptr: *mut u8,
        phantom: PhantomData<*mut (T, A)>,
    }
    unsafe impl<T: Copy, A: Access> Send for Reg<T, A> {}
    unsafe impl<T: Copy, A: Access> Sync for Reg<T, A> {}
    impl<T: Copy, A: Access> Reg<T, A> {
        #[allow(clippy::missing_safety_doc)]
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut T) -> Self {
            Self {
                ptr: ptr as _,
                phantom: PhantomData,
            }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut T {
            self.ptr as _
        }
    }
    impl<T: Copy, A: Read> Reg<T, A> {
        #[inline(always)]
        pub fn read(&self) -> T {
            unsafe { (self.ptr as *mut T).read_volatile() }
        }
    }
    impl<T: Copy, A: Write> Reg<T, A> {
        #[inline(always)]
        pub fn write_value(&self, val: T) {
            unsafe { (self.ptr as *mut T).write_volatile(val) }
        }
    }
    impl<T: Default + Copy, A: Write> Reg<T, A> {
        #[inline(always)]
        pub fn write(&self, f: impl FnOnce(&mut T)) {
            let mut val = Default::default();
            f(&mut val);
            self.write_value(val);
        }
    }
    impl<T: Copy, A: Read + Write> Reg<T, A> {
        #[inline(always)]
        pub fn modify(&self, f: impl FnOnce(&mut T)) {
            let mut val = self.read();
            f(&mut val);
            self.write_value(val);
        }
    }
}
pub mod cti {
    #[doc = "Cross-Trigger Interface control. NOTE: this is not a separate peripheral, but describes CM33 functionality."]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cti {
        ptr: *mut u8,
    }
    unsafe impl Send for Cti {}
    unsafe impl Sync for Cti {}
    impl Cti {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "CTI Control register"]
        #[inline(always)]
        pub const fn cticontrol(self) -> crate::common::Reg<regs::Cticontrol, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "CTI Interrupt Acknowledge register"]
        #[inline(always)]
        pub const fn ctiintack(self) -> crate::common::Reg<regs::Ctiintack, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
        }
        #[doc = "CTI Application Trigger Set register"]
        #[inline(always)]
        pub const fn ctiappset(self) -> crate::common::Reg<regs::Ctiappset, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
        }
        #[doc = "CTI Application Trigger Clear register"]
        #[inline(always)]
        pub const fn ctiappclear(self) -> crate::common::Reg<regs::Ctiappclear, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
        }
        #[doc = "CTI Application Pulse register"]
        #[inline(always)]
        pub const fn ctiapppulse(self) -> crate::common::Reg<regs::Ctiapppulse, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
        }
        #[doc = "Description collection: CTI Trigger input"]
        #[inline(always)]
        pub const fn ctiinen(
            self,
            n: usize,
        ) -> crate::common::Reg<regs::Ctiinen, crate::common::RW> {
            assert!(n < 8usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _)
            }
        }
        #[doc = "Description collection: CTI Trigger output"]
        #[inline(always)]
        pub const fn ctiouten(
            self,
            n: usize,
        ) -> crate::common::Reg<regs::Ctiouten, crate::common::RW> {
            assert!(n < 8usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 4usize) as _)
            }
        }
        #[doc = "CTI Trigger In Status register"]
        #[inline(always)]
        pub const fn ctitriginstatus(
            self,
        ) -> crate::common::Reg<regs::Ctitriginstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
        }
        #[doc = "CTI Trigger Out Status register"]
        #[inline(always)]
        pub const fn ctitrigoutstatus(
            self,
        ) -> crate::common::Reg<regs::Ctitrigoutstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
        }
        #[doc = "CTI Channel In Status register"]
        #[inline(always)]
        pub const fn ctichinstatus(
            self,
        ) -> crate::common::Reg<regs::Ctichinstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
        }
        #[doc = "Enable CTI Channel Gate register"]
        #[inline(always)]
        pub const fn ctigate(self) -> crate::common::Reg<regs::Ctigate, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
        }
        #[doc = "Device Architecture register"]
        #[inline(always)]
        pub const fn devarch(self) -> crate::common::Reg<regs::Devarch, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fbcusize) as _) }
        }
        #[doc = "Device Configuration register"]
        #[inline(always)]
        pub const fn devid(self) -> crate::common::Reg<regs::Devid, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc8usize) as _) }
        }
        #[doc = "Device Type Identifier register"]
        #[inline(always)]
        pub const fn devtype(self) -> crate::common::Reg<regs::Devtype, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fccusize) as _) }
        }
        #[doc = "Peripheral ID4 Register"]
        #[inline(always)]
        pub const fn pidr4(self) -> crate::common::Reg<regs::Pidr4, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd0usize) as _) }
        }
        #[doc = "Peripheral ID5 register"]
        #[inline(always)]
        pub const fn pidr5(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd4usize) as _) }
        }
        #[doc = "Peripheral ID6 register"]
        #[inline(always)]
        pub const fn pidr6(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd8usize) as _) }
        }
        #[doc = "Peripheral ID7 register"]
        #[inline(always)]
        pub const fn pidr7(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fdcusize) as _) }
        }
        #[doc = "Peripheral ID0 Register"]
        #[inline(always)]
        pub const fn pidr0(self) -> crate::common::Reg<regs::Pidr0, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe0usize) as _) }
        }
        #[doc = "Peripheral ID1 Register"]
        #[inline(always)]
        pub const fn pidr1(self) -> crate::common::Reg<regs::Pidr1, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe4usize) as _) }
        }
        #[doc = "Peripheral ID2 Register"]
        #[inline(always)]
        pub const fn pidr2(self) -> crate::common::Reg<regs::Pidr2, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe8usize) as _) }
        }
        #[doc = "Peripheral ID3 Register"]
        #[inline(always)]
        pub const fn pidr3(self) -> crate::common::Reg<regs::Pidr3, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fecusize) as _) }
        }
        #[doc = "Component ID0 Register"]
        #[inline(always)]
        pub const fn cidr0(self) -> crate::common::Reg<regs::Cidr0, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff0usize) as _) }
        }
        #[doc = "Component ID1 Register"]
        #[inline(always)]
        pub const fn cidr1(self) -> crate::common::Reg<regs::Cidr1, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff4usize) as _) }
        }
        #[doc = "Component ID2 Register"]
        #[inline(always)]
        pub const fn cidr2(self) -> crate::common::Reg<regs::Cidr2, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
        }
        #[doc = "Component ID3 Register"]
        #[inline(always)]
        pub const fn cidr3(self) -> crate::common::Reg<regs::Cidr3, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Component ID0 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cidr0(pub u32);
        impl Cidr0 {
            #[doc = "Preamble\\[0\\]. Contains bits\\[7:0\\] of the component identification code."]
            #[must_use]
            #[inline(always)]
            pub const fn prmbl_0(&self) -> super::vals::Prmbl0 {
                let val = (self.0 >> 0usize) & 0xff;
                super::vals::Prmbl0::from_bits(val as u8)
            }
            #[doc = "Preamble\\[0\\]. Contains bits\\[7:0\\] of the component identification code."]
            #[inline(always)]
            pub const fn set_prmbl_0(&mut self, val: super::vals::Prmbl0) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Cidr0 {
            #[inline(always)]
            fn default() -> Cidr0 {
                Cidr0(0)
            }
        }
        impl core::fmt::Debug for Cidr0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Cidr0")
                    .field("prmbl_0", &self.prmbl_0())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Cidr0 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Cidr0 {{ prmbl_0: {:?} }}", self.prmbl_0())
            }
        }
        #[doc = "Component ID1 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cidr1(pub u32);
        impl Cidr1 {
            #[doc = "Preamble\\[1\\]. Contains bits\\[11:8\\] of the component identification code."]
            #[must_use]
            #[inline(always)]
            pub const fn prmbl_1(&self) -> super::vals::Prmbl1 {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Prmbl1::from_bits(val as u8)
            }
            #[doc = "Preamble\\[1\\]. Contains bits\\[11:8\\] of the component identification code."]
            #[inline(always)]
            pub const fn set_prmbl_1(&mut self, val: super::vals::Prmbl1) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
            #[doc = "Class of the component, for example, whether the component is a ROM table or a generic CoreSight component. Contains bits\\[15:12\\] of the component identification code"]
            #[must_use]
            #[inline(always)]
            pub const fn class(&self) -> super::vals::Class {
                let val = (self.0 >> 4usize) & 0x0f;
                super::vals::Class::from_bits(val as u8)
            }
            #[doc = "Class of the component, for example, whether the component is a ROM table or a generic CoreSight component. Contains bits\\[15:12\\] of the component identification code"]
            #[inline(always)]
            pub const fn set_class(&mut self, val: super::vals::Class) {
                self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
            }
        }
        impl Default for Cidr1 {
            #[inline(always)]
            fn default() -> Cidr1 {
                Cidr1(0)
            }
        }
        impl core::fmt::Debug for Cidr1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Cidr1")
                    .field("prmbl_1", &self.prmbl_1())
                    .field("class", &self.class())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Cidr1 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Cidr1 {{ prmbl_1: {:?}, class: {:?} }}",
                    self.prmbl_1(),
                    self.class()
                )
            }
        }
        #[doc = "Component ID2 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cidr2(pub u32);
        impl Cidr2 {
            #[doc = "Preamble\\[2\\]. Contains bits\\[23:16\\] of the component identification code."]
            #[must_use]
            #[inline(always)]
            pub const fn prmbl_2(&self) -> super::vals::Prmbl2 {
                let val = (self.0 >> 0usize) & 0xff;
                super::vals::Prmbl2::from_bits(val as u8)
            }
            #[doc = "Preamble\\[2\\]. Contains bits\\[23:16\\] of the component identification code."]
            #[inline(always)]
            pub const fn set_prmbl_2(&mut self, val: super::vals::Prmbl2) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Cidr2 {
            #[inline(always)]
            fn default() -> Cidr2 {
                Cidr2(0)
            }
        }
        impl core::fmt::Debug for Cidr2 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Cidr2")
                    .field("prmbl_2", &self.prmbl_2())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Cidr2 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Cidr2 {{ prmbl_2: {:?} }}", self.prmbl_2())
            }
        }
        #[doc = "Component ID3 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cidr3(pub u32);
        impl Cidr3 {
            #[doc = "Preamble\\[3\\]. Contains bits\\[31:24\\] of the component identification code."]
            #[must_use]
            #[inline(always)]
            pub const fn prmbl_3(&self) -> super::vals::Prmbl3 {
                let val = (self.0 >> 0usize) & 0xff;
                super::vals::Prmbl3::from_bits(val as u8)
            }
            #[doc = "Preamble\\[3\\]. Contains bits\\[31:24\\] of the component identification code."]
            #[inline(always)]
            pub const fn set_prmbl_3(&mut self, val: super::vals::Prmbl3) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Cidr3 {
            #[inline(always)]
            fn default() -> Cidr3 {
                Cidr3(0)
            }
        }
        impl core::fmt::Debug for Cidr3 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Cidr3")
                    .field("prmbl_3", &self.prmbl_3())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Cidr3 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Cidr3 {{ prmbl_3: {:?} }}", self.prmbl_3())
            }
        }
        #[doc = "CTI Application Trigger Clear register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctiappclear(pub u32);
        impl Ctiappclear {
            #[doc = "Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
            #[must_use]
            #[inline(always)]
            pub const fn appclear_0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
            #[inline(always)]
            pub const fn set_appclear_0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
            #[must_use]
            #[inline(always)]
            pub const fn appclear_1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
            #[inline(always)]
            pub const fn set_appclear_1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
            #[must_use]
            #[inline(always)]
            pub const fn appclear_2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
            #[inline(always)]
            pub const fn set_appclear_2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
            #[must_use]
            #[inline(always)]
            pub const fn appclear_3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
            #[inline(always)]
            pub const fn set_appclear_3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Ctiappclear {
            #[inline(always)]
            fn default() -> Ctiappclear {
                Ctiappclear(0)
            }
        }
        impl core::fmt::Debug for Ctiappclear {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctiappclear")
                    .field("appclear_0", &self.appclear_0())
                    .field("appclear_1", &self.appclear_1())
                    .field("appclear_2", &self.appclear_2())
                    .field("appclear_3", &self.appclear_3())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctiappclear {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Ctiappclear {{ appclear_0: {=bool:?}, appclear_1: {=bool:?}, appclear_2: {=bool:?}, appclear_3: {=bool:?} }}" , self . appclear_0 () , self . appclear_1 () , self . appclear_2 () , self . appclear_3 ())
            }
        }
        #[doc = "CTI Application Pulse register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctiapppulse(pub u32);
        impl Ctiapppulse {
            #[doc = "Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
            #[must_use]
            #[inline(always)]
            pub const fn appulse_0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
            #[inline(always)]
            pub const fn set_appulse_0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
            #[must_use]
            #[inline(always)]
            pub const fn appulse_1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
            #[inline(always)]
            pub const fn set_appulse_1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
            #[must_use]
            #[inline(always)]
            pub const fn appulse_2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
            #[inline(always)]
            pub const fn set_appulse_2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
            #[must_use]
            #[inline(always)]
            pub const fn appulse_3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
            #[inline(always)]
            pub const fn set_appulse_3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Ctiapppulse {
            #[inline(always)]
            fn default() -> Ctiapppulse {
                Ctiapppulse(0)
            }
        }
        impl core::fmt::Debug for Ctiapppulse {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctiapppulse")
                    .field("appulse_0", &self.appulse_0())
                    .field("appulse_1", &self.appulse_1())
                    .field("appulse_2", &self.appulse_2())
                    .field("appulse_3", &self.appulse_3())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctiapppulse {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Ctiapppulse {{ appulse_0: {=bool:?}, appulse_1: {=bool:?}, appulse_2: {=bool:?}, appulse_3: {=bool:?} }}" , self . appulse_0 () , self . appulse_1 () , self . appulse_2 () , self . appulse_3 ())
            }
        }
        #[doc = "CTI Application Trigger Set register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctiappset(pub u32);
        impl Ctiappset {
            #[doc = "Application trigger event for channel 0."]
            #[must_use]
            #[inline(always)]
            pub const fn appset_0(&self) -> super::vals::Appset0 {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Appset0::from_bits(val as u8)
            }
            #[doc = "Application trigger event for channel 0."]
            #[inline(always)]
            pub const fn set_appset_0(&mut self, val: super::vals::Appset0) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Application trigger event for channel 1."]
            #[must_use]
            #[inline(always)]
            pub const fn appset_1(&self) -> super::vals::Appset1 {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Appset1::from_bits(val as u8)
            }
            #[doc = "Application trigger event for channel 1."]
            #[inline(always)]
            pub const fn set_appset_1(&mut self, val: super::vals::Appset1) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Application trigger event for channel 2."]
            #[must_use]
            #[inline(always)]
            pub const fn appset_2(&self) -> super::vals::Appset2 {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Appset2::from_bits(val as u8)
            }
            #[doc = "Application trigger event for channel 2."]
            #[inline(always)]
            pub const fn set_appset_2(&mut self, val: super::vals::Appset2) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
            #[doc = "Application trigger event for channel 3."]
            #[must_use]
            #[inline(always)]
            pub const fn appset_3(&self) -> super::vals::Appset3 {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Appset3::from_bits(val as u8)
            }
            #[doc = "Application trigger event for channel 3."]
            #[inline(always)]
            pub const fn set_appset_3(&mut self, val: super::vals::Appset3) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Ctiappset {
            #[inline(always)]
            fn default() -> Ctiappset {
                Ctiappset(0)
            }
        }
        impl core::fmt::Debug for Ctiappset {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctiappset")
                    .field("appset_0", &self.appset_0())
                    .field("appset_1", &self.appset_1())
                    .field("appset_2", &self.appset_2())
                    .field("appset_3", &self.appset_3())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctiappset {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Ctiappset {{ appset_0: {:?}, appset_1: {:?}, appset_2: {:?}, appset_3: {:?} }}" , self . appset_0 () , self . appset_1 () , self . appset_2 () , self . appset_3 ())
            }
        }
        #[doc = "CTI Channel In Status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctichinstatus(pub u32);
        impl Ctichinstatus {
            #[doc = "Shows the status of the ctitrigin 0 input."]
            #[must_use]
            #[inline(always)]
            pub const fn ctichinstatus_0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shows the status of the ctitrigin 0 input."]
            #[inline(always)]
            pub const fn set_ctichinstatus_0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Shows the status of the ctitrigin 1 input."]
            #[must_use]
            #[inline(always)]
            pub const fn ctichinstatus_1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Shows the status of the ctitrigin 1 input."]
            #[inline(always)]
            pub const fn set_ctichinstatus_1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Shows the status of the ctitrigin 2 input."]
            #[must_use]
            #[inline(always)]
            pub const fn ctichinstatus_2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Shows the status of the ctitrigin 2 input."]
            #[inline(always)]
            pub const fn set_ctichinstatus_2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Shows the status of the ctitrigin 3 input."]
            #[must_use]
            #[inline(always)]
            pub const fn ctichinstatus_3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Shows the status of the ctitrigin 3 input."]
            #[inline(always)]
            pub const fn set_ctichinstatus_3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Ctichinstatus {
            #[inline(always)]
            fn default() -> Ctichinstatus {
                Ctichinstatus(0)
            }
        }
        impl core::fmt::Debug for Ctichinstatus {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctichinstatus")
                    .field("ctichinstatus_0", &self.ctichinstatus_0())
                    .field("ctichinstatus_1", &self.ctichinstatus_1())
                    .field("ctichinstatus_2", &self.ctichinstatus_2())
                    .field("ctichinstatus_3", &self.ctichinstatus_3())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctichinstatus {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Ctichinstatus {{ ctichinstatus_0: {=bool:?}, ctichinstatus_1: {=bool:?}, ctichinstatus_2: {=bool:?}, ctichinstatus_3: {=bool:?} }}" , self . ctichinstatus_0 () , self . ctichinstatus_1 () , self . ctichinstatus_2 () , self . ctichinstatus_3 ())
            }
        }
        #[doc = "CTI Control register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cticontrol(pub u32);
        impl Cticontrol {
            #[doc = "Enables or disables the CTI."]
            #[must_use]
            #[inline(always)]
            pub const fn glben(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enables or disables the CTI."]
            #[inline(always)]
            pub const fn set_glben(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Cticontrol {
            #[inline(always)]
            fn default() -> Cticontrol {
                Cticontrol(0)
            }
        }
        impl core::fmt::Debug for Cticontrol {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Cticontrol")
                    .field("glben", &self.glben())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Cticontrol {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Cticontrol {{ glben: {=bool:?} }}", self.glben())
            }
        }
        #[doc = "Enable CTI Channel Gate register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctigate(pub u32);
        impl Ctigate {
            #[doc = "Enable ctichout0."]
            #[must_use]
            #[inline(always)]
            pub const fn ctigateen_0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable ctichout0."]
            #[inline(always)]
            pub const fn set_ctigateen_0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable ctichout1."]
            #[must_use]
            #[inline(always)]
            pub const fn ctigateen_1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable ctichout1."]
            #[inline(always)]
            pub const fn set_ctigateen_1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable ctichout2."]
            #[must_use]
            #[inline(always)]
            pub const fn ctigateen_2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable ctichout2."]
            #[inline(always)]
            pub const fn set_ctigateen_2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable ctichout3."]
            #[must_use]
            #[inline(always)]
            pub const fn ctigateen_3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable ctichout3."]
            #[inline(always)]
            pub const fn set_ctigateen_3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Ctigate {
            #[inline(always)]
            fn default() -> Ctigate {
                Ctigate(0)
            }
        }
        impl core::fmt::Debug for Ctigate {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctigate")
                    .field("ctigateen_0", &self.ctigateen_0())
                    .field("ctigateen_1", &self.ctigateen_1())
                    .field("ctigateen_2", &self.ctigateen_2())
                    .field("ctigateen_3", &self.ctigateen_3())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctigate {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Ctigate {{ ctigateen_0: {=bool:?}, ctigateen_1: {=bool:?}, ctigateen_2: {=bool:?}, ctigateen_3: {=bool:?} }}" , self . ctigateen_0 () , self . ctigateen_1 () , self . ctigateen_2 () , self . ctigateen_3 ())
            }
        }
        #[doc = "Description collection: CTI Trigger input"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctiinen(pub u32);
        impl Ctiinen {
            #[doc = "Enables a cross trigger event to channel 0 when a ctitrigin input is activated."]
            #[must_use]
            #[inline(always)]
            pub const fn triginen_0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enables a cross trigger event to channel 0 when a ctitrigin input is activated."]
            #[inline(always)]
            pub const fn set_triginen_0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enables a cross trigger event to channel 1 when a ctitrigin input is activated."]
            #[must_use]
            #[inline(always)]
            pub const fn triginen_1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enables a cross trigger event to channel 1 when a ctitrigin input is activated."]
            #[inline(always)]
            pub const fn set_triginen_1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enables a cross trigger event to channel 2 when a ctitrigin input is activated."]
            #[must_use]
            #[inline(always)]
            pub const fn triginen_2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enables a cross trigger event to channel 2 when a ctitrigin input is activated."]
            #[inline(always)]
            pub const fn set_triginen_2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enables a cross trigger event to channel 3 when a ctitrigin input is activated."]
            #[must_use]
            #[inline(always)]
            pub const fn triginen_3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enables a cross trigger event to channel 3 when a ctitrigin input is activated."]
            #[inline(always)]
            pub const fn set_triginen_3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Ctiinen {
            #[inline(always)]
            fn default() -> Ctiinen {
                Ctiinen(0)
            }
        }
        impl core::fmt::Debug for Ctiinen {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctiinen")
                    .field("triginen_0", &self.triginen_0())
                    .field("triginen_1", &self.triginen_1())
                    .field("triginen_2", &self.triginen_2())
                    .field("triginen_3", &self.triginen_3())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctiinen {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Ctiinen {{ triginen_0: {=bool:?}, triginen_1: {=bool:?}, triginen_2: {=bool:?}, triginen_3: {=bool:?} }}" , self . triginen_0 () , self . triginen_1 () , self . triginen_2 () , self . triginen_3 ())
            }
        }
        #[doc = "CTI Interrupt Acknowledge register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctiintack(pub u32);
        impl Ctiintack {
            #[doc = "Processor debug request"]
            #[must_use]
            #[inline(always)]
            pub const fn debugreq(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Processor debug request"]
            #[inline(always)]
            pub const fn set_debugreq(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Processor Restart"]
            #[must_use]
            #[inline(always)]
            pub const fn cpurestart(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Processor Restart"]
            #[inline(always)]
            pub const fn set_cpurestart(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "N/A"]
            #[must_use]
            #[inline(always)]
            pub const fn unused0(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "N/A"]
            #[inline(always)]
            pub const fn set_unused0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "N/A"]
            #[must_use]
            #[inline(always)]
            pub const fn unused1(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "N/A"]
            #[inline(always)]
            pub const fn set_unused1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "N/A"]
            #[must_use]
            #[inline(always)]
            pub const fn unused2(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "N/A"]
            #[inline(always)]
            pub const fn set_unused2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "N/A"]
            #[must_use]
            #[inline(always)]
            pub const fn unused3(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "N/A"]
            #[inline(always)]
            pub const fn set_unused3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "N/A"]
            #[must_use]
            #[inline(always)]
            pub const fn unused4(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "N/A"]
            #[inline(always)]
            pub const fn set_unused4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "N/A"]
            #[must_use]
            #[inline(always)]
            pub const fn unused5(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "N/A"]
            #[inline(always)]
            pub const fn set_unused5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
        }
        impl Default for Ctiintack {
            #[inline(always)]
            fn default() -> Ctiintack {
                Ctiintack(0)
            }
        }
        impl core::fmt::Debug for Ctiintack {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctiintack")
                    .field("debugreq", &self.debugreq())
                    .field("cpurestart", &self.cpurestart())
                    .field("unused0", &self.unused0())
                    .field("unused1", &self.unused1())
                    .field("unused2", &self.unused2())
                    .field("unused3", &self.unused3())
                    .field("unused4", &self.unused4())
                    .field("unused5", &self.unused5())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctiintack {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Ctiintack {{ debugreq: {=bool:?}, cpurestart: {=bool:?}, unused0: {=bool:?}, unused1: {=bool:?}, unused2: {=bool:?}, unused3: {=bool:?}, unused4: {=bool:?}, unused5: {=bool:?} }}" , self . debugreq () , self . cpurestart () , self . unused0 () , self . unused1 () , self . unused2 () , self . unused3 () , self . unused4 () , self . unused5 ())
            }
        }
        #[doc = "Description collection: CTI Trigger output"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctiouten(pub u32);
        impl Ctiouten {
            #[doc = "Enables a cross trigger event to ctitrigout when channel 0 is activated."]
            #[must_use]
            #[inline(always)]
            pub const fn trigouten_0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enables a cross trigger event to ctitrigout when channel 0 is activated."]
            #[inline(always)]
            pub const fn set_trigouten_0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enables a cross trigger event to ctitrigout when channel 1 is activated."]
            #[must_use]
            #[inline(always)]
            pub const fn trigouten_1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enables a cross trigger event to ctitrigout when channel 1 is activated."]
            #[inline(always)]
            pub const fn set_trigouten_1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enables a cross trigger event to ctitrigout when channel 2 is activated."]
            #[must_use]
            #[inline(always)]
            pub const fn trigouten_2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enables a cross trigger event to ctitrigout when channel 2 is activated."]
            #[inline(always)]
            pub const fn set_trigouten_2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enables a cross trigger event to ctitrigout when channel 3 is activated."]
            #[must_use]
            #[inline(always)]
            pub const fn trigouten_3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enables a cross trigger event to ctitrigout when channel 3 is activated."]
            #[inline(always)]
            pub const fn set_trigouten_3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Ctiouten {
            #[inline(always)]
            fn default() -> Ctiouten {
                Ctiouten(0)
            }
        }
        impl core::fmt::Debug for Ctiouten {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctiouten")
                    .field("trigouten_0", &self.trigouten_0())
                    .field("trigouten_1", &self.trigouten_1())
                    .field("trigouten_2", &self.trigouten_2())
                    .field("trigouten_3", &self.trigouten_3())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctiouten {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Ctiouten {{ trigouten_0: {=bool:?}, trigouten_1: {=bool:?}, trigouten_2: {=bool:?}, trigouten_3: {=bool:?} }}" , self . trigouten_0 () , self . trigouten_1 () , self . trigouten_2 () , self . trigouten_3 ())
            }
        }
        #[doc = "CTI Trigger In Status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctitriginstatus(pub u32);
        impl Ctitriginstatus {
            #[doc = "Processor Halted"]
            #[must_use]
            #[inline(always)]
            pub const fn cpuhalted(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Processor Halted"]
            #[inline(always)]
            pub const fn set_cpuhalted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "DWT Comparator Output 0"]
            #[must_use]
            #[inline(always)]
            pub const fn dwtcompout0(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "DWT Comparator Output 0"]
            #[inline(always)]
            pub const fn set_dwtcompout0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "DWT Comparator Output 1"]
            #[must_use]
            #[inline(always)]
            pub const fn dwtcompout1(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "DWT Comparator Output 1"]
            #[inline(always)]
            pub const fn set_dwtcompout1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "DWT Comparator Output 2"]
            #[must_use]
            #[inline(always)]
            pub const fn dwtcompout2(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "DWT Comparator Output 2"]
            #[inline(always)]
            pub const fn set_dwtcompout2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "N/A"]
            #[must_use]
            #[inline(always)]
            pub const fn unused0(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "N/A"]
            #[inline(always)]
            pub const fn set_unused0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "N/A"]
            #[must_use]
            #[inline(always)]
            pub const fn unused1(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "N/A"]
            #[inline(always)]
            pub const fn set_unused1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "N/A"]
            #[must_use]
            #[inline(always)]
            pub const fn unused2(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "N/A"]
            #[inline(always)]
            pub const fn set_unused2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "N/A"]
            #[must_use]
            #[inline(always)]
            pub const fn unused3(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "N/A"]
            #[inline(always)]
            pub const fn set_unused3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
        }
        impl Default for Ctitriginstatus {
            #[inline(always)]
            fn default() -> Ctitriginstatus {
                Ctitriginstatus(0)
            }
        }
        impl core::fmt::Debug for Ctitriginstatus {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctitriginstatus")
                    .field("cpuhalted", &self.cpuhalted())
                    .field("dwtcompout0", &self.dwtcompout0())
                    .field("dwtcompout1", &self.dwtcompout1())
                    .field("dwtcompout2", &self.dwtcompout2())
                    .field("unused0", &self.unused0())
                    .field("unused1", &self.unused1())
                    .field("unused2", &self.unused2())
                    .field("unused3", &self.unused3())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctitriginstatus {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Ctitriginstatus {{ cpuhalted: {=bool:?}, dwtcompout0: {=bool:?}, dwtcompout1: {=bool:?}, dwtcompout2: {=bool:?}, unused0: {=bool:?}, unused1: {=bool:?}, unused2: {=bool:?}, unused3: {=bool:?} }}" , self . cpuhalted () , self . dwtcompout0 () , self . dwtcompout1 () , self . dwtcompout2 () , self . unused0 () , self . unused1 () , self . unused2 () , self . unused3 ())
            }
        }
        #[doc = "CTI Trigger Out Status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctitrigoutstatus(pub u32);
        impl Ctitrigoutstatus {
            #[doc = "Processor debug request"]
            #[must_use]
            #[inline(always)]
            pub const fn debugreq(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Processor debug request"]
            #[inline(always)]
            pub const fn set_debugreq(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Processor Restart"]
            #[must_use]
            #[inline(always)]
            pub const fn cpurestart(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Processor Restart"]
            #[inline(always)]
            pub const fn set_cpurestart(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "N/A"]
            #[must_use]
            #[inline(always)]
            pub const fn unused0(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "N/A"]
            #[inline(always)]
            pub const fn set_unused0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "N/A"]
            #[must_use]
            #[inline(always)]
            pub const fn unused1(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "N/A"]
            #[inline(always)]
            pub const fn set_unused1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "N/A"]
            #[must_use]
            #[inline(always)]
            pub const fn unused2(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "N/A"]
            #[inline(always)]
            pub const fn set_unused2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "N/A"]
            #[must_use]
            #[inline(always)]
            pub const fn unused3(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "N/A"]
            #[inline(always)]
            pub const fn set_unused3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "N/A"]
            #[must_use]
            #[inline(always)]
            pub const fn unused4(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "N/A"]
            #[inline(always)]
            pub const fn set_unused4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "N/A"]
            #[must_use]
            #[inline(always)]
            pub const fn unused5(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "N/A"]
            #[inline(always)]
            pub const fn set_unused5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
        }
        impl Default for Ctitrigoutstatus {
            #[inline(always)]
            fn default() -> Ctitrigoutstatus {
                Ctitrigoutstatus(0)
            }
        }
        impl core::fmt::Debug for Ctitrigoutstatus {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctitrigoutstatus")
                    .field("debugreq", &self.debugreq())
                    .field("cpurestart", &self.cpurestart())
                    .field("unused0", &self.unused0())
                    .field("unused1", &self.unused1())
                    .field("unused2", &self.unused2())
                    .field("unused3", &self.unused3())
                    .field("unused4", &self.unused4())
                    .field("unused5", &self.unused5())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctitrigoutstatus {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Ctitrigoutstatus {{ debugreq: {=bool:?}, cpurestart: {=bool:?}, unused0: {=bool:?}, unused1: {=bool:?}, unused2: {=bool:?}, unused3: {=bool:?}, unused4: {=bool:?}, unused5: {=bool:?} }}" , self . debugreq () , self . cpurestart () , self . unused0 () , self . unused1 () , self . unused2 () , self . unused3 () , self . unused4 () , self . unused5 ())
            }
        }
        #[doc = "Device Architecture register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Devarch(pub u32);
        impl Devarch {
            #[doc = "Contains the CTI device architecture."]
            #[must_use]
            #[inline(always)]
            pub const fn architecture(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Contains the CTI device architecture."]
            #[inline(always)]
            pub const fn set_architecture(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Devarch {
            #[inline(always)]
            fn default() -> Devarch {
                Devarch(0)
            }
        }
        impl core::fmt::Debug for Devarch {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Devarch")
                    .field("architecture", &self.architecture())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Devarch {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Devarch {{ architecture: {=bool:?} }}",
                    self.architecture()
                )
            }
        }
        #[doc = "Device Configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Devid(pub u32);
        impl Devid {
            #[doc = "Indicates the number of multiplexers available on Trigger Inputs and Trigger Outputs that are using asicctl. The default value of 0b00000 indicates that no multiplexing is present."]
            #[must_use]
            #[inline(always)]
            pub const fn extmuxnum(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x1f;
                val as u8
            }
            #[doc = "Indicates the number of multiplexers available on Trigger Inputs and Trigger Outputs that are using asicctl. The default value of 0b00000 indicates that no multiplexing is present."]
            #[inline(always)]
            pub const fn set_extmuxnum(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
            }
            #[doc = "Number of ECT triggers available."]
            #[must_use]
            #[inline(always)]
            pub const fn numtrig(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "Number of ECT triggers available."]
            #[inline(always)]
            pub const fn set_numtrig(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[doc = "Number of ECT channels available."]
            #[must_use]
            #[inline(always)]
            pub const fn numch(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0x0f;
                val as u8
            }
            #[doc = "Number of ECT channels available."]
            #[inline(always)]
            pub const fn set_numch(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
            }
        }
        impl Default for Devid {
            #[inline(always)]
            fn default() -> Devid {
                Devid(0)
            }
        }
        impl core::fmt::Debug for Devid {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Devid")
                    .field("extmuxnum", &self.extmuxnum())
                    .field("numtrig", &self.numtrig())
                    .field("numch", &self.numch())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Devid {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Devid {{ extmuxnum: {=u8:?}, numtrig: {=u8:?}, numch: {=u8:?} }}",
                    self.extmuxnum(),
                    self.numtrig(),
                    self.numch()
                )
            }
        }
        #[doc = "Device Type Identifier register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Devtype(pub u32);
        impl Devtype {
            #[doc = "Major classification of the type of the debug component as specified in the Arm Architecture Specification for this debug and trace component."]
            #[must_use]
            #[inline(always)]
            pub const fn major(&self) -> super::vals::Major {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Major::from_bits(val as u8)
            }
            #[doc = "Major classification of the type of the debug component as specified in the Arm Architecture Specification for this debug and trace component."]
            #[inline(always)]
            pub const fn set_major(&mut self, val: super::vals::Major) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
            #[doc = "Sub-classification of the type of the debug component as specified in the Arm Architecture Specification within the major classification as specified in the MAJOR field."]
            #[must_use]
            #[inline(always)]
            pub const fn sub(&self) -> super::vals::Sub {
                let val = (self.0 >> 4usize) & 0x0f;
                super::vals::Sub::from_bits(val as u8)
            }
            #[doc = "Sub-classification of the type of the debug component as specified in the Arm Architecture Specification within the major classification as specified in the MAJOR field."]
            #[inline(always)]
            pub const fn set_sub(&mut self, val: super::vals::Sub) {
                self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
            }
        }
        impl Default for Devtype {
            #[inline(always)]
            fn default() -> Devtype {
                Devtype(0)
            }
        }
        impl core::fmt::Debug for Devtype {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Devtype")
                    .field("major", &self.major())
                    .field("sub", &self.sub())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Devtype {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Devtype {{ major: {:?}, sub: {:?} }}",
                    self.major(),
                    self.sub()
                )
            }
        }
        #[doc = "Peripheral ID0 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pidr0(pub u32);
        impl Pidr0 {
            #[doc = "Bits\\[7:0\\] of the 12-bit part number of the component. The designer of the component assigns this part number."]
            #[must_use]
            #[inline(always)]
            pub const fn part_0(&self) -> super::vals::Part0 {
                let val = (self.0 >> 0usize) & 0xff;
                super::vals::Part0::from_bits(val as u8)
            }
            #[doc = "Bits\\[7:0\\] of the 12-bit part number of the component. The designer of the component assigns this part number."]
            #[inline(always)]
            pub const fn set_part_0(&mut self, val: super::vals::Part0) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Pidr0 {
            #[inline(always)]
            fn default() -> Pidr0 {
                Pidr0(0)
            }
        }
        impl core::fmt::Debug for Pidr0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Pidr0")
                    .field("part_0", &self.part_0())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Pidr0 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Pidr0 {{ part_0: {:?} }}", self.part_0())
            }
        }
        #[doc = "Peripheral ID1 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pidr1(pub u32);
        impl Pidr1 {
            #[doc = "Bits\\[11:8\\] of the 12-bit part number of the component. The designer of the component assigns this part number."]
            #[must_use]
            #[inline(always)]
            pub const fn part_1(&self) -> super::vals::Part1 {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Part1::from_bits(val as u8)
            }
            #[doc = "Bits\\[11:8\\] of the 12-bit part number of the component. The designer of the component assigns this part number."]
            #[inline(always)]
            pub const fn set_part_1(&mut self, val: super::vals::Part1) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
            #[doc = "Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
            #[must_use]
            #[inline(always)]
            pub const fn des_0(&self) -> super::vals::Des0 {
                let val = (self.0 >> 4usize) & 0x0f;
                super::vals::Des0::from_bits(val as u8)
            }
            #[doc = "Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
            #[inline(always)]
            pub const fn set_des_0(&mut self, val: super::vals::Des0) {
                self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
            }
        }
        impl Default for Pidr1 {
            #[inline(always)]
            fn default() -> Pidr1 {
                Pidr1(0)
            }
        }
        impl core::fmt::Debug for Pidr1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Pidr1")
                    .field("part_1", &self.part_1())
                    .field("des_0", &self.des_0())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Pidr1 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Pidr1 {{ part_1: {:?}, des_0: {:?} }}",
                    self.part_1(),
                    self.des_0()
                )
            }
        }
        #[doc = "Peripheral ID2 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pidr2(pub u32);
        impl Pidr2 {
            #[doc = "Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
            #[must_use]
            #[inline(always)]
            pub const fn des_1(&self) -> super::vals::Des1 {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::Des1::from_bits(val as u8)
            }
            #[doc = "Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
            #[inline(always)]
            pub const fn set_des_1(&mut self, val: super::vals::Des1) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
            }
            #[doc = "Always 1. Indicates that the JEDEC-assigned designer ID is used."]
            #[must_use]
            #[inline(always)]
            pub const fn jedec(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Always 1. Indicates that the JEDEC-assigned designer ID is used."]
            #[inline(always)]
            pub const fn set_jedec(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Peripheral revision"]
            #[must_use]
            #[inline(always)]
            pub const fn revision(&self) -> super::vals::Revision {
                let val = (self.0 >> 4usize) & 0x0f;
                super::vals::Revision::from_bits(val as u8)
            }
            #[doc = "Peripheral revision"]
            #[inline(always)]
            pub const fn set_revision(&mut self, val: super::vals::Revision) {
                self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
            }
        }
        impl Default for Pidr2 {
            #[inline(always)]
            fn default() -> Pidr2 {
                Pidr2(0)
            }
        }
        impl core::fmt::Debug for Pidr2 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Pidr2")
                    .field("des_1", &self.des_1())
                    .field("jedec", &self.jedec())
                    .field("revision", &self.revision())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Pidr2 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Pidr2 {{ des_1: {:?}, jedec: {=bool:?}, revision: {:?} }}",
                    self.des_1(),
                    self.jedec(),
                    self.revision()
                )
            }
        }
        #[doc = "Peripheral ID3 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pidr3(pub u32);
        impl Pidr3 {
            #[doc = "Customer Modified. Indicates whether the customer has modified the behavior of the component. In most cases, this field is 0b0000. Customers change this value when they make authorized modifications to this component."]
            #[must_use]
            #[inline(always)]
            pub const fn cmod(&self) -> super::vals::Cmod {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Cmod::from_bits(val as u8)
            }
            #[doc = "Customer Modified. Indicates whether the customer has modified the behavior of the component. In most cases, this field is 0b0000. Customers change this value when they make authorized modifications to this component."]
            #[inline(always)]
            pub const fn set_cmod(&mut self, val: super::vals::Cmod) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
            #[doc = "Indicates minor errata fixes specific to the revision of the component being used, for example metal fixes after implementation. In most cases, this field is 0b0000. Arm recommends that the component designers ensure that a metal fix can change this field if required, for example, by driving it from registers that reset to 0b0000."]
            #[must_use]
            #[inline(always)]
            pub const fn revand(&self) -> super::vals::Revand {
                let val = (self.0 >> 4usize) & 0x0f;
                super::vals::Revand::from_bits(val as u8)
            }
            #[doc = "Indicates minor errata fixes specific to the revision of the component being used, for example metal fixes after implementation. In most cases, this field is 0b0000. Arm recommends that the component designers ensure that a metal fix can change this field if required, for example, by driving it from registers that reset to 0b0000."]
            #[inline(always)]
            pub const fn set_revand(&mut self, val: super::vals::Revand) {
                self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
            }
        }
        impl Default for Pidr3 {
            #[inline(always)]
            fn default() -> Pidr3 {
                Pidr3(0)
            }
        }
        impl core::fmt::Debug for Pidr3 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Pidr3")
                    .field("cmod", &self.cmod())
                    .field("revand", &self.revand())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Pidr3 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Pidr3 {{ cmod: {:?}, revand: {:?} }}",
                    self.cmod(),
                    self.revand()
                )
            }
        }
        #[doc = "Peripheral ID4 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pidr4(pub u32);
        impl Pidr4 {
            #[doc = "Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
            #[must_use]
            #[inline(always)]
            pub const fn des_2(&self) -> super::vals::Des2 {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Des2::from_bits(val as u8)
            }
            #[doc = "Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
            #[inline(always)]
            pub const fn set_des_2(&mut self, val: super::vals::Des2) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
            #[doc = "Always 0b0000. Indicates that the device only occupies 4KB of memory."]
            #[must_use]
            #[inline(always)]
            pub const fn size(&self) -> u8 {
                let val = (self.0 >> 4usize) & 0x0f;
                val as u8
            }
            #[doc = "Always 0b0000. Indicates that the device only occupies 4KB of memory."]
            #[inline(always)]
            pub const fn set_size(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
            }
        }
        impl Default for Pidr4 {
            #[inline(always)]
            fn default() -> Pidr4 {
                Pidr4(0)
            }
        }
        impl core::fmt::Debug for Pidr4 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Pidr4")
                    .field("des_2", &self.des_2())
                    .field("size", &self.size())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Pidr4 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Pidr4 {{ des_2: {:?}, size: {=u8:?} }}",
                    self.des_2(),
                    self.size()
                )
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Appset0 {
            #[doc = "Application trigger 0 is inactive."]
            INACTIVE = 0x0,
            #[doc = "Application trigger 0 is active."]
            R_ACTIVE_W_ACTIVATE = 0x01,
        }
        impl Appset0 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Appset0 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Appset0 {
            #[inline(always)]
            fn from(val: u8) -> Appset0 {
                Appset0::from_bits(val)
            }
        }
        impl From<Appset0> for u8 {
            #[inline(always)]
            fn from(val: Appset0) -> u8 {
                Appset0::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Appset1 {
            #[doc = "Application trigger 1 is inactive."]
            INACTIVE = 0x0,
            #[doc = "Application trigger 1 is active."]
            R_ACTIVE_W_ACTIVATE = 0x01,
        }
        impl Appset1 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Appset1 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Appset1 {
            #[inline(always)]
            fn from(val: u8) -> Appset1 {
                Appset1::from_bits(val)
            }
        }
        impl From<Appset1> for u8 {
            #[inline(always)]
            fn from(val: Appset1) -> u8 {
                Appset1::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Appset2 {
            #[doc = "Application trigger 2 is inactive."]
            INACTIVE = 0x0,
            #[doc = "Application trigger 2 is active."]
            R_ACTIVE_W_ACTIVATE = 0x01,
        }
        impl Appset2 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Appset2 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Appset2 {
            #[inline(always)]
            fn from(val: u8) -> Appset2 {
                Appset2::from_bits(val)
            }
        }
        impl From<Appset2> for u8 {
            #[inline(always)]
            fn from(val: Appset2) -> u8 {
                Appset2::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Appset3 {
            #[doc = "Application trigger 3 is inactive."]
            INACTIVE = 0x0,
            #[doc = "Application trigger 3 is active."]
            R_ACTIVE_W_ACTIVATE = 0x01,
        }
        impl Appset3 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Appset3 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Appset3 {
            #[inline(always)]
            fn from(val: u8) -> Appset3 {
                Appset3::from_bits(val)
            }
        }
        impl From<Appset3> for u8 {
            #[inline(always)]
            fn from(val: Appset3) -> u8 {
                Appset3::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Class {
            _RESERVED_0 = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
            _RESERVED_8 = 0x08,
            #[doc = "Indicates that the component is a CoreSight component."]
            CORESIGHT = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Class {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Class {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Class {
            #[inline(always)]
            fn from(val: u8) -> Class {
                Class::from_bits(val)
            }
        }
        impl From<Class> for u8 {
            #[inline(always)]
            fn from(val: Class) -> u8 {
                Class::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Cmod {
            #[doc = "Indicates that the customer has not modified this component."]
            UNMODIFIED = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
            _RESERVED_8 = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Cmod {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Cmod {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Cmod {
            #[inline(always)]
            fn from(val: u8) -> Cmod {
                Cmod::from_bits(val)
            }
        }
        impl From<Cmod> for u8 {
            #[inline(always)]
            fn from(val: Cmod) -> u8 {
                Cmod::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Des0 {
            _RESERVED_0 = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
            _RESERVED_8 = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            #[doc = "Arm. Bits\\[3:0\\] of the JEDEC JEP106 Identity Code"]
            ARM = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Des0 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Des0 {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Des0 {
            #[inline(always)]
            fn from(val: u8) -> Des0 {
                Des0::from_bits(val)
            }
        }
        impl From<Des0> for u8 {
            #[inline(always)]
            fn from(val: Des0) -> u8 {
                Des0::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Des1 {
            _RESERVED_0 = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            #[doc = "Arm. Bits\\[6:4\\] of the JEDEC JEP106 Identity Code"]
            ARM = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl Des1 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Des1 {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Des1 {
            #[inline(always)]
            fn from(val: u8) -> Des1 {
                Des1::from_bits(val)
            }
        }
        impl From<Des1> for u8 {
            #[inline(always)]
            fn from(val: Des1) -> u8 {
                Des1::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Des2 {
            _RESERVED_0 = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            #[doc = "JEDEC continuation code."]
            CODE = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
            _RESERVED_8 = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Des2 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Des2 {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Des2 {
            #[inline(always)]
            fn from(val: u8) -> Des2 {
                Des2::from_bits(val)
            }
        }
        impl From<Des2> for u8 {
            #[inline(always)]
            fn from(val: Des2) -> u8 {
                Des2::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Major {
            _RESERVED_0 = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            #[doc = "Indicates that this component allows a debugger to control other components in an Arm CoreSight SoC-400 system."]
            CONTROLLER = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
            _RESERVED_8 = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Major {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Major {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Major {
            #[inline(always)]
            fn from(val: u8) -> Major {
                Major::from_bits(val)
            }
        }
        impl From<Major> for u8 {
            #[inline(always)]
            fn from(val: Major) -> u8 {
                Major::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Part0(u8);
        impl Part0 {
            #[doc = "Indicates bits\\[7:0\\] of the part number of the component."]
            pub const PARTNUMBER_L: Self = Self(0x21);
        }
        impl Part0 {
            pub const fn from_bits(val: u8) -> Part0 {
                Self(val & 0xff)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl core::fmt::Debug for Part0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x21 => f.write_str("PARTNUMBER_L"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Part0 {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x21 => defmt::write!(f, "PARTNUMBER_L"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u8> for Part0 {
            #[inline(always)]
            fn from(val: u8) -> Part0 {
                Part0::from_bits(val)
            }
        }
        impl From<Part0> for u8 {
            #[inline(always)]
            fn from(val: Part0) -> u8 {
                Part0::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Part1 {
            _RESERVED_0 = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
            _RESERVED_8 = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            #[doc = "Indicates bits\\[11:8\\] of the part number of the component."]
            PARTNUMBER_H = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Part1 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Part1 {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Part1 {
            #[inline(always)]
            fn from(val: u8) -> Part1 {
                Part1::from_bits(val)
            }
        }
        impl From<Part1> for u8 {
            #[inline(always)]
            fn from(val: Part1) -> u8 {
                Part1::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Prmbl0(u8);
        impl Prmbl0 {
            #[doc = "Bits\\[7:0\\] of the identification code."]
            pub const VALUE: Self = Self(0x0d);
        }
        impl Prmbl0 {
            pub const fn from_bits(val: u8) -> Prmbl0 {
                Self(val & 0xff)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl core::fmt::Debug for Prmbl0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0d => f.write_str("VALUE"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Prmbl0 {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0d => defmt::write!(f, "VALUE"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u8> for Prmbl0 {
            #[inline(always)]
            fn from(val: u8) -> Prmbl0 {
                Prmbl0::from_bits(val)
            }
        }
        impl From<Prmbl0> for u8 {
            #[inline(always)]
            fn from(val: Prmbl0) -> u8 {
                Prmbl0::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Prmbl1 {
            #[doc = "Bits\\[11:8\\] of the identification code."]
            VALUE = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
            _RESERVED_8 = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Prmbl1 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Prmbl1 {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Prmbl1 {
            #[inline(always)]
            fn from(val: u8) -> Prmbl1 {
                Prmbl1::from_bits(val)
            }
        }
        impl From<Prmbl1> for u8 {
            #[inline(always)]
            fn from(val: Prmbl1) -> u8 {
                Prmbl1::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Prmbl2(u8);
        impl Prmbl2 {
            #[doc = "Bits\\[23:16\\] of the identification code."]
            pub const VALUE: Self = Self(0x05);
        }
        impl Prmbl2 {
            pub const fn from_bits(val: u8) -> Prmbl2 {
                Self(val & 0xff)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl core::fmt::Debug for Prmbl2 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x05 => f.write_str("VALUE"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Prmbl2 {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x05 => defmt::write!(f, "VALUE"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u8> for Prmbl2 {
            #[inline(always)]
            fn from(val: u8) -> Prmbl2 {
                Prmbl2::from_bits(val)
            }
        }
        impl From<Prmbl2> for u8 {
            #[inline(always)]
            fn from(val: Prmbl2) -> u8 {
                Prmbl2::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Prmbl3(u8);
        impl Prmbl3 {
            #[doc = "Bits\\[31:24\\] of the identification code."]
            pub const VALUE: Self = Self(0xb1);
        }
        impl Prmbl3 {
            pub const fn from_bits(val: u8) -> Prmbl3 {
                Self(val & 0xff)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl core::fmt::Debug for Prmbl3 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0xb1 => f.write_str("VALUE"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Prmbl3 {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0xb1 => defmt::write!(f, "VALUE"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u8> for Prmbl3 {
            #[inline(always)]
            fn from(val: u8) -> Prmbl3 {
                Prmbl3::from_bits(val)
            }
        }
        impl From<Prmbl3> for u8 {
            #[inline(always)]
            fn from(val: Prmbl3) -> u8 {
                Prmbl3::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Revand {
            #[doc = "Indicates that there are no errata fixes to this component."]
            NO_ERRATA = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
            _RESERVED_8 = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Revand {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Revand {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Revand {
            #[inline(always)]
            fn from(val: u8) -> Revand {
                Revand::from_bits(val)
            }
        }
        impl From<Revand> for u8 {
            #[inline(always)]
            fn from(val: Revand) -> u8 {
                Revand::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Revision {
            #[doc = "This device is at r0p0"]
            REV0P0 = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
            _RESERVED_8 = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Revision {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Revision {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Revision {
            #[inline(always)]
            fn from(val: u8) -> Revision {
                Revision::from_bits(val)
            }
        }
        impl From<Revision> for u8 {
            #[inline(always)]
            fn from(val: Revision) -> u8 {
                Revision::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Sub {
            _RESERVED_0 = 0x0,
            #[doc = "Indicates that this component is a sub-triggering component."]
            CROSSTRIGGER = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
            _RESERVED_8 = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Sub {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Sub {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Sub {
            #[inline(always)]
            fn from(val: u8) -> Sub {
                Sub::from_bits(val)
            }
        }
        impl From<Sub> for u8 {
            #[inline(always)]
            fn from(val: Sub) -> u8 {
                Sub::to_bits(val)
            }
        }
    }
}
pub mod ctrlapperi {
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Approtect {
        ptr: *mut u8,
    }
    unsafe impl Send for Approtect {}
    unsafe impl Sync for Approtect {}
    impl Approtect {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "This register locks the APPROTECT.DISABLE register from being written to until next reset."]
        #[inline(always)]
        pub const fn lock(self) -> crate::common::Reg<regs::ApprotectLock, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "This register disables the APPROTECT register and enables debug access to non-secure mode."]
        #[inline(always)]
        pub const fn disable(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
    }
    #[doc = "Control access port"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrlapperi {
        ptr: *mut u8,
    }
    unsafe impl Send for Ctrlapperi {}
    unsafe impl Sync for Ctrlapperi {}
    impl Ctrlapperi {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn mailbox(self) -> Mailbox {
            unsafe { Mailbox::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn eraseprotect(self) -> Eraseprotect {
            unsafe { Eraseprotect::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn approtect(self) -> Approtect {
            unsafe { Approtect::from_ptr(self.ptr.wrapping_add(0x0540usize) as _) }
        }
        #[doc = "Status bits for CTRL-AP peripheral."]
        #[inline(always)]
        pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eraseprotect {
        ptr: *mut u8,
    }
    unsafe impl Send for Eraseprotect {}
    unsafe impl Sync for Eraseprotect {}
    impl Eraseprotect {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "This register locks the ERASEPROTECT.DISABLE register from being written until next reset."]
        #[inline(always)]
        pub const fn lock(self) -> crate::common::Reg<regs::EraseprotectLock, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "This register disables the ERASEPROTECT register and performs an ERASEALL operation."]
        #[inline(always)]
        pub const fn disable(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mailbox {
        ptr: *mut u8,
    }
    unsafe impl Send for Mailbox {}
    unsafe impl Sync for Mailbox {}
    impl Mailbox {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Data sent from the debugger to the CPU."]
        #[inline(always)]
        pub const fn rxdata(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "This register shows a status that indicates if data sent from the debugger to the CPU has been read."]
        #[inline(always)]
        pub const fn rxstatus(self) -> crate::common::Reg<regs::Rxstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Data sent from the CPU to the debugger."]
        #[inline(always)]
        pub const fn txdata(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
        }
        #[doc = "This register shows a status that indicates if the data sent from the CPU to the debugger has been read."]
        #[inline(always)]
        pub const fn txstatus(self) -> crate::common::Reg<regs::Txstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "This register locks the APPROTECT.DISABLE register from being written to until next reset."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct ApprotectLock(pub u32);
        impl ApprotectLock {
            #[doc = "Lock the APPROTECT.DISABLE register from being written to until next reset"]
            #[must_use]
            #[inline(always)]
            pub const fn lock(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Lock the APPROTECT.DISABLE register from being written to until next reset"]
            #[inline(always)]
            pub const fn set_lock(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for ApprotectLock {
            #[inline(always)]
            fn default() -> ApprotectLock {
                ApprotectLock(0)
            }
        }
        impl core::fmt::Debug for ApprotectLock {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("ApprotectLock")
                    .field("lock", &self.lock())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for ApprotectLock {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "ApprotectLock {{ lock: {=bool:?} }}", self.lock())
            }
        }
        #[doc = "This register locks the ERASEPROTECT.DISABLE register from being written until next reset."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct EraseprotectLock(pub u32);
        impl EraseprotectLock {
            #[doc = "Lock ERASEPROTECT.DISABLE register from being written until next reset"]
            #[must_use]
            #[inline(always)]
            pub const fn lock(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Lock ERASEPROTECT.DISABLE register from being written until next reset"]
            #[inline(always)]
            pub const fn set_lock(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for EraseprotectLock {
            #[inline(always)]
            fn default() -> EraseprotectLock {
                EraseprotectLock(0)
            }
        }
        impl core::fmt::Debug for EraseprotectLock {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("EraseprotectLock")
                    .field("lock", &self.lock())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for EraseprotectLock {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "EraseprotectLock {{ lock: {=bool:?} }}", self.lock())
            }
        }
        #[doc = "This register shows a status that indicates if data sent from the debugger to the CPU has been read."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxstatus(pub u32);
        impl Rxstatus {
            #[doc = "Status of data in register RXDATA"]
            #[must_use]
            #[inline(always)]
            pub const fn rxstatus(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Status of data in register RXDATA"]
            #[inline(always)]
            pub const fn set_rxstatus(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Rxstatus {
            #[inline(always)]
            fn default() -> Rxstatus {
                Rxstatus(0)
            }
        }
        impl core::fmt::Debug for Rxstatus {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rxstatus")
                    .field("rxstatus", &self.rxstatus())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rxstatus {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Rxstatus {{ rxstatus: {=bool:?} }}", self.rxstatus())
            }
        }
        #[doc = "Status bits for CTRL-AP peripheral."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Status(pub u32);
        impl Status {
            #[doc = "Status bit for UICR part of access port protection at last reset."]
            #[must_use]
            #[inline(always)]
            pub const fn uicrapprotect(&self) -> super::vals::Uicrapprotect {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Uicrapprotect::from_bits(val as u8)
            }
            #[doc = "Status bit for UICR part of access port protection at last reset."]
            #[inline(always)]
            pub const fn set_uicrapprotect(&mut self, val: super::vals::Uicrapprotect) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Status bit for device debug interface mode"]
            #[must_use]
            #[inline(always)]
            pub const fn dbgifacemode(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Status bit for device debug interface mode"]
            #[inline(always)]
            pub const fn set_dbgifacemode(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Status {
            #[inline(always)]
            fn default() -> Status {
                Status(0)
            }
        }
        impl core::fmt::Debug for Status {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Status")
                    .field("uicrapprotect", &self.uicrapprotect())
                    .field("dbgifacemode", &self.dbgifacemode())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Status {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Status {{ uicrapprotect: {:?}, dbgifacemode: {=bool:?} }}",
                    self.uicrapprotect(),
                    self.dbgifacemode()
                )
            }
        }
        #[doc = "This register shows a status that indicates if the data sent from the CPU to the debugger has been read."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Txstatus(pub u32);
        impl Txstatus {
            #[doc = "Status of data in register TXDATA"]
            #[must_use]
            #[inline(always)]
            pub const fn txstatus(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Status of data in register TXDATA"]
            #[inline(always)]
            pub const fn set_txstatus(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Txstatus {
            #[inline(always)]
            fn default() -> Txstatus {
                Txstatus(0)
            }
        }
        impl core::fmt::Debug for Txstatus {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Txstatus")
                    .field("txstatus", &self.txstatus())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Txstatus {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Txstatus {{ txstatus: {=bool:?} }}", self.txstatus())
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Uicrapprotect {
            #[doc = "APPROTECT was enabled in UICR"]
            ENABLED = 0x0,
            #[doc = "APPROTECT wasdisabled in UICR"]
            DISABLED = 0x01,
        }
        impl Uicrapprotect {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Uicrapprotect {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Uicrapprotect {
            #[inline(always)]
            fn from(val: u8) -> Uicrapprotect {
                Uicrapprotect::from_bits(val)
            }
        }
        impl From<Uicrapprotect> for u8 {
            #[inline(always)]
            fn from(val: Uicrapprotect) -> u8 {
                Uicrapprotect::to_bits(val)
            }
        }
    }
}
pub mod dcnf {
    #[doc = "Domain configuration management"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcnf {
        ptr: *mut u8,
    }
    unsafe impl Send for Dcnf {}
    unsafe impl Sync for Dcnf {}
    impl Dcnf {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "CPU ID of this subsystem"]
        #[inline(always)]
        pub const fn cpuid(self) -> crate::common::Reg<regs::Cpuid, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "CPU ID of this subsystem"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cpuid(pub u32);
        impl Cpuid {
            #[doc = "CPU ID"]
            #[must_use]
            #[inline(always)]
            pub const fn cpuid(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "CPU ID"]
            #[inline(always)]
            pub const fn set_cpuid(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Cpuid {
            #[inline(always)]
            fn default() -> Cpuid {
                Cpuid(0)
            }
        }
        impl core::fmt::Debug for Cpuid {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Cpuid")
                    .field("cpuid", &self.cpuid())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Cpuid {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Cpuid {{ cpuid: {=u8:?} }}", self.cpuid())
            }
        }
    }
}
pub mod dppic {
    #[doc = "Distributed programmable peripheral interconnect controller"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dppic {
        ptr: *mut u8,
    }
    unsafe impl Send for Dppic {}
    unsafe impl Sync for Dppic {}
    impl Dppic {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Channel group tasks"]
        #[inline(always)]
        pub const fn tasks_chg(self, n: usize) -> TasksChg {
            assert!(n < 6usize);
            unsafe { TasksChg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 8usize) as _) }
        }
        #[doc = "Subscribe configuration for tasks"]
        #[inline(always)]
        pub const fn subscribe_chg(self, n: usize) -> SubscribeChg {
            assert!(n < 6usize);
            unsafe { SubscribeChg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 8usize) as _) }
        }
        #[doc = "Channel enable register"]
        #[inline(always)]
        pub const fn chen(self) -> crate::common::Reg<regs::Chen, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
        }
        #[doc = "Channel enable set register"]
        #[inline(always)]
        pub const fn chenset(self) -> crate::common::Reg<regs::Chen, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
        }
        #[doc = "Channel enable clear register"]
        #[inline(always)]
        pub const fn chenclr(self) -> crate::common::Reg<regs::Chen, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
        }
        #[doc = "Description collection: Channel group n Note: Writes to this register are ignored if either SUBSCRIBE_CHG\\[n\\].EN or SUBSCRIBE_CHG\\[n\\].DIS is enabled"]
        #[inline(always)]
        pub const fn chg(self, n: usize) -> crate::common::Reg<regs::Chg, crate::common::RW> {
            assert!(n < 6usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize + n * 4usize) as _)
            }
        }
    }
    #[doc = "Subscribe configuration for tasks"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SubscribeChg {
        ptr: *mut u8,
    }
    unsafe impl Send for SubscribeChg {}
    unsafe impl Sync for SubscribeChg {}
    impl SubscribeChg {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster: Subscribe configuration for task CHG\\[n\\].EN"]
        #[inline(always)]
        pub const fn en(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Description cluster: Subscribe configuration for task CHG\\[n\\].DIS"]
        #[inline(always)]
        pub const fn dis(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
    }
    #[doc = "Channel group tasks"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TasksChg {
        ptr: *mut u8,
    }
    unsafe impl Send for TasksChg {}
    unsafe impl Sync for TasksChg {}
    impl TasksChg {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster: Enable channel group n"]
        #[inline(always)]
        pub const fn en(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Description cluster: Disable channel group n"]
        #[inline(always)]
        pub const fn dis(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Channel enable register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Chen(pub u32);
        impl Chen {
            #[doc = "Enable or disable channel 0"]
            #[must_use]
            #[inline(always)]
            pub const fn ch(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable channel 0"]
            #[inline(always)]
            pub const fn set_ch(&mut self, n: usize, val: bool) {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Chen {
            #[inline(always)]
            fn default() -> Chen {
                Chen(0)
            }
        }
        impl core::fmt::Debug for Chen {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Chen")
                    .field("ch[0]", &self.ch(0usize))
                    .field("ch[1]", &self.ch(1usize))
                    .field("ch[2]", &self.ch(2usize))
                    .field("ch[3]", &self.ch(3usize))
                    .field("ch[4]", &self.ch(4usize))
                    .field("ch[5]", &self.ch(5usize))
                    .field("ch[6]", &self.ch(6usize))
                    .field("ch[7]", &self.ch(7usize))
                    .field("ch[8]", &self.ch(8usize))
                    .field("ch[9]", &self.ch(9usize))
                    .field("ch[10]", &self.ch(10usize))
                    .field("ch[11]", &self.ch(11usize))
                    .field("ch[12]", &self.ch(12usize))
                    .field("ch[13]", &self.ch(13usize))
                    .field("ch[14]", &self.ch(14usize))
                    .field("ch[15]", &self.ch(15usize))
                    .field("ch[16]", &self.ch(16usize))
                    .field("ch[17]", &self.ch(17usize))
                    .field("ch[18]", &self.ch(18usize))
                    .field("ch[19]", &self.ch(19usize))
                    .field("ch[20]", &self.ch(20usize))
                    .field("ch[21]", &self.ch(21usize))
                    .field("ch[22]", &self.ch(22usize))
                    .field("ch[23]", &self.ch(23usize))
                    .field("ch[24]", &self.ch(24usize))
                    .field("ch[25]", &self.ch(25usize))
                    .field("ch[26]", &self.ch(26usize))
                    .field("ch[27]", &self.ch(27usize))
                    .field("ch[28]", &self.ch(28usize))
                    .field("ch[29]", &self.ch(29usize))
                    .field("ch[30]", &self.ch(30usize))
                    .field("ch[31]", &self.ch(31usize))
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Chen {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Chen {{ ch[0]: {=bool:?}, ch[1]: {=bool:?}, ch[2]: {=bool:?}, ch[3]: {=bool:?}, ch[4]: {=bool:?}, ch[5]: {=bool:?}, ch[6]: {=bool:?}, ch[7]: {=bool:?}, ch[8]: {=bool:?}, ch[9]: {=bool:?}, ch[10]: {=bool:?}, ch[11]: {=bool:?}, ch[12]: {=bool:?}, ch[13]: {=bool:?}, ch[14]: {=bool:?}, ch[15]: {=bool:?}, ch[16]: {=bool:?}, ch[17]: {=bool:?}, ch[18]: {=bool:?}, ch[19]: {=bool:?}, ch[20]: {=bool:?}, ch[21]: {=bool:?}, ch[22]: {=bool:?}, ch[23]: {=bool:?}, ch[24]: {=bool:?}, ch[25]: {=bool:?}, ch[26]: {=bool:?}, ch[27]: {=bool:?}, ch[28]: {=bool:?}, ch[29]: {=bool:?}, ch[30]: {=bool:?}, ch[31]: {=bool:?} }}" , self . ch (0usize) , self . ch (1usize) , self . ch (2usize) , self . ch (3usize) , self . ch (4usize) , self . ch (5usize) , self . ch (6usize) , self . ch (7usize) , self . ch (8usize) , self . ch (9usize) , self . ch (10usize) , self . ch (11usize) , self . ch (12usize) , self . ch (13usize) , self . ch (14usize) , self . ch (15usize) , self . ch (16usize) , self . ch (17usize) , self . ch (18usize) , self . ch (19usize) , self . ch (20usize) , self . ch (21usize) , self . ch (22usize) , self . ch (23usize) , self . ch (24usize) , self . ch (25usize) , self . ch (26usize) , self . ch (27usize) , self . ch (28usize) , self . ch (29usize) , self . ch (30usize) , self . ch (31usize))
            }
        }
        #[doc = "Description collection: Channel group n Note: Writes to this register are ignored if either SUBSCRIBE_CHG\\[n\\].EN or SUBSCRIBE_CHG\\[n\\].DIS is enabled"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Chg(pub u32);
        impl Chg {
            #[doc = "Include or exclude channel 0"]
            #[must_use]
            #[inline(always)]
            pub const fn ch(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Include or exclude channel 0"]
            #[inline(always)]
            pub const fn set_ch(&mut self, n: usize, val: bool) {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Chg {
            #[inline(always)]
            fn default() -> Chg {
                Chg(0)
            }
        }
        impl core::fmt::Debug for Chg {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Chg")
                    .field("ch[0]", &self.ch(0usize))
                    .field("ch[1]", &self.ch(1usize))
                    .field("ch[2]", &self.ch(2usize))
                    .field("ch[3]", &self.ch(3usize))
                    .field("ch[4]", &self.ch(4usize))
                    .field("ch[5]", &self.ch(5usize))
                    .field("ch[6]", &self.ch(6usize))
                    .field("ch[7]", &self.ch(7usize))
                    .field("ch[8]", &self.ch(8usize))
                    .field("ch[9]", &self.ch(9usize))
                    .field("ch[10]", &self.ch(10usize))
                    .field("ch[11]", &self.ch(11usize))
                    .field("ch[12]", &self.ch(12usize))
                    .field("ch[13]", &self.ch(13usize))
                    .field("ch[14]", &self.ch(14usize))
                    .field("ch[15]", &self.ch(15usize))
                    .field("ch[16]", &self.ch(16usize))
                    .field("ch[17]", &self.ch(17usize))
                    .field("ch[18]", &self.ch(18usize))
                    .field("ch[19]", &self.ch(19usize))
                    .field("ch[20]", &self.ch(20usize))
                    .field("ch[21]", &self.ch(21usize))
                    .field("ch[22]", &self.ch(22usize))
                    .field("ch[23]", &self.ch(23usize))
                    .field("ch[24]", &self.ch(24usize))
                    .field("ch[25]", &self.ch(25usize))
                    .field("ch[26]", &self.ch(26usize))
                    .field("ch[27]", &self.ch(27usize))
                    .field("ch[28]", &self.ch(28usize))
                    .field("ch[29]", &self.ch(29usize))
                    .field("ch[30]", &self.ch(30usize))
                    .field("ch[31]", &self.ch(31usize))
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Chg {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Chg {{ ch[0]: {=bool:?}, ch[1]: {=bool:?}, ch[2]: {=bool:?}, ch[3]: {=bool:?}, ch[4]: {=bool:?}, ch[5]: {=bool:?}, ch[6]: {=bool:?}, ch[7]: {=bool:?}, ch[8]: {=bool:?}, ch[9]: {=bool:?}, ch[10]: {=bool:?}, ch[11]: {=bool:?}, ch[12]: {=bool:?}, ch[13]: {=bool:?}, ch[14]: {=bool:?}, ch[15]: {=bool:?}, ch[16]: {=bool:?}, ch[17]: {=bool:?}, ch[18]: {=bool:?}, ch[19]: {=bool:?}, ch[20]: {=bool:?}, ch[21]: {=bool:?}, ch[22]: {=bool:?}, ch[23]: {=bool:?}, ch[24]: {=bool:?}, ch[25]: {=bool:?}, ch[26]: {=bool:?}, ch[27]: {=bool:?}, ch[28]: {=bool:?}, ch[29]: {=bool:?}, ch[30]: {=bool:?}, ch[31]: {=bool:?} }}" , self . ch (0usize) , self . ch (1usize) , self . ch (2usize) , self . ch (3usize) , self . ch (4usize) , self . ch (5usize) , self . ch (6usize) , self . ch (7usize) , self . ch (8usize) , self . ch (9usize) , self . ch (10usize) , self . ch (11usize) , self . ch (12usize) , self . ch (13usize) , self . ch (14usize) , self . ch (15usize) , self . ch (16usize) , self . ch (17usize) , self . ch (18usize) , self . ch (19usize) , self . ch (20usize) , self . ch (21usize) , self . ch (22usize) , self . ch (23usize) , self . ch (24usize) , self . ch (25usize) , self . ch (26usize) , self . ch (27usize) , self . ch (28usize) , self . ch (29usize) , self . ch (30usize) , self . ch (31usize))
            }
        }
    }
}
pub mod ecb {
    #[doc = "AES ECB Mode Encryption"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ecb {
        ptr: *mut u8,
    }
    unsafe impl Send for Ecb {}
    unsafe impl Sync for Ecb {}
    impl Ecb {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start ECB block encrypt"]
        #[inline(always)]
        pub const fn tasks_startecb(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Abort a possible executing ECB operation"]
        #[inline(always)]
        pub const fn tasks_stopecb(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Subscribe configuration for task STARTECB"]
        #[inline(always)]
        pub const fn subscribe_startecb(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
        }
        #[doc = "Subscribe configuration for task STOPECB"]
        #[inline(always)]
        pub const fn subscribe_stopecb(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
        }
        #[doc = "ECB block encrypt complete"]
        #[inline(always)]
        pub const fn events_endecb(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
        }
        #[doc = "ECB block encrypt aborted because of a STOPECB task or due to an error"]
        #[inline(always)]
        pub const fn events_errorecb(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
        }
        #[doc = "Publish configuration for event ENDECB"]
        #[inline(always)]
        pub const fn publish_endecb(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
        }
        #[doc = "Publish configuration for event ERRORECB"]
        #[inline(always)]
        pub const fn publish_errorecb(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
        }
        #[doc = "ECB block encrypt memory pointers"]
        #[inline(always)]
        pub const fn ecbdataptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event ENDECB"]
            #[must_use]
            #[inline(always)]
            pub const fn endecb(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ENDECB"]
            #[inline(always)]
            pub const fn set_endecb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to disable interrupt for event ERRORECB"]
            #[must_use]
            #[inline(always)]
            pub const fn errorecb(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ERRORECB"]
            #[inline(always)]
            pub const fn set_errorecb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("endecb", &self.endecb())
                    .field("errorecb", &self.errorecb())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Int {{ endecb: {=bool:?}, errorecb: {=bool:?} }}",
                    self.endecb(),
                    self.errorecb()
                )
            }
        }
    }
}
pub mod egu {
    #[doc = "Event generator unit"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Egu {
        ptr: *mut u8,
    }
    unsafe impl Send for Egu {}
    unsafe impl Sync for Egu {}
    impl Egu {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description collection: Trigger n for triggering the corresponding TRIGGERED\\[n\\] event"]
        #[inline(always)]
        pub const fn tasks_trigger(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
            assert!(n < 16usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _)
            }
        }
        #[doc = "Description collection: Subscribe configuration for task TRIGGER\\[n\\]"]
        #[inline(always)]
        pub const fn subscribe_trigger(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            assert!(n < 16usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _)
            }
        }
        #[doc = "Description collection: Event number n generated by triggering the corresponding TRIGGER\\[n\\] task"]
        #[inline(always)]
        pub const fn events_triggered(
            self,
            n: usize,
        ) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 16usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
            }
        }
        #[doc = "Description collection: Publish configuration for event TRIGGERED\\[n\\]"]
        #[inline(always)]
        pub const fn publish_triggered(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            assert!(n < 16usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize + n * 4usize) as _)
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Enable or disable interrupt for event TRIGGERED\\[0\\]"]
            #[must_use]
            #[inline(always)]
            pub const fn triggered(&self, n: usize) -> bool {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event TRIGGERED\\[0\\]"]
            #[inline(always)]
            pub const fn set_triggered(&mut self, n: usize, val: bool) {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("triggered[0]", &self.triggered(0usize))
                    .field("triggered[1]", &self.triggered(1usize))
                    .field("triggered[2]", &self.triggered(2usize))
                    .field("triggered[3]", &self.triggered(3usize))
                    .field("triggered[4]", &self.triggered(4usize))
                    .field("triggered[5]", &self.triggered(5usize))
                    .field("triggered[6]", &self.triggered(6usize))
                    .field("triggered[7]", &self.triggered(7usize))
                    .field("triggered[8]", &self.triggered(8usize))
                    .field("triggered[9]", &self.triggered(9usize))
                    .field("triggered[10]", &self.triggered(10usize))
                    .field("triggered[11]", &self.triggered(11usize))
                    .field("triggered[12]", &self.triggered(12usize))
                    .field("triggered[13]", &self.triggered(13usize))
                    .field("triggered[14]", &self.triggered(14usize))
                    .field("triggered[15]", &self.triggered(15usize))
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Int {{ triggered[0]: {=bool:?}, triggered[1]: {=bool:?}, triggered[2]: {=bool:?}, triggered[3]: {=bool:?}, triggered[4]: {=bool:?}, triggered[5]: {=bool:?}, triggered[6]: {=bool:?}, triggered[7]: {=bool:?}, triggered[8]: {=bool:?}, triggered[9]: {=bool:?}, triggered[10]: {=bool:?}, triggered[11]: {=bool:?}, triggered[12]: {=bool:?}, triggered[13]: {=bool:?}, triggered[14]: {=bool:?}, triggered[15]: {=bool:?} }}" , self . triggered (0usize) , self . triggered (1usize) , self . triggered (2usize) , self . triggered (3usize) , self . triggered (4usize) , self . triggered (5usize) , self . triggered (6usize) , self . triggered (7usize) , self . triggered (8usize) , self . triggered (9usize) , self . triggered (10usize) , self . triggered (11usize) , self . triggered (12usize) , self . triggered (13usize) , self . triggered (14usize) , self . triggered (15usize))
            }
        }
    }
}
pub mod ficr {
    #[doc = "Factory Information Configuration Registers"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ficr {
        ptr: *mut u8,
    }
    unsafe impl Send for Ficr {}
    unsafe impl Sync for Ficr {}
    impl Ficr {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Device info"]
        #[inline(always)]
        pub const fn info(self) -> Info {
            unsafe { Info::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
        }
        #[doc = "Description collection: Encryption Root, word n"]
        #[inline(always)]
        pub const fn er(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
            assert!(n < 4usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize + n * 4usize) as _)
            }
        }
        #[doc = "Description collection: Identity Root, word n"]
        #[inline(always)]
        pub const fn ir(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
            assert!(n < 4usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0290usize + n * 4usize) as _)
            }
        }
        #[doc = "Device address type"]
        #[inline(always)]
        pub const fn deviceaddrtype(
            self,
        ) -> crate::common::Reg<regs::Deviceaddrtype, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a0usize) as _) }
        }
        #[doc = "Description collection: Device address n"]
        #[inline(always)]
        pub const fn deviceaddr(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
            assert!(n < 2usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a4usize + n * 4usize) as _)
            }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn trimcnf(self, n: usize) -> Trimcnf {
            assert!(n < 32usize);
            unsafe { Trimcnf::from_ptr(self.ptr.wrapping_add(0x0300usize + n * 8usize) as _) }
        }
    }
    #[doc = "Device info"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Info {
        ptr: *mut u8,
    }
    unsafe impl Send for Info {}
    unsafe impl Sync for Info {}
    impl Info {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Configuration identifier"]
        #[inline(always)]
        pub const fn configid(self) -> crate::common::Reg<regs::Configid, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Description collection: Device identifier"]
        #[inline(always)]
        pub const fn deviceid(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
            assert!(n < 2usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize + n * 4usize) as _)
            }
        }
        #[doc = "Part code"]
        #[inline(always)]
        pub const fn part(self) -> crate::common::Reg<regs::Part, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
        #[doc = "Part Variant, Hardware version and Production configuration"]
        #[inline(always)]
        pub const fn variant(self) -> crate::common::Reg<regs::Variant, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
        }
        #[doc = "Package option"]
        #[inline(always)]
        pub const fn package(self) -> crate::common::Reg<regs::Package, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
        }
        #[doc = "RAM variant"]
        #[inline(always)]
        pub const fn ram(self) -> crate::common::Reg<regs::Ram, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
        }
        #[doc = "Flash variant"]
        #[inline(always)]
        pub const fn flash(self) -> crate::common::Reg<regs::Flash, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
        }
        #[doc = "Code memory page size in bytes"]
        #[inline(always)]
        pub const fn codepagesize(
            self,
        ) -> crate::common::Reg<regs::Codepagesize, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
        }
        #[doc = "Code memory size"]
        #[inline(always)]
        pub const fn codesize(self) -> crate::common::Reg<regs::Codesize, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
        }
        #[doc = "Device type"]
        #[inline(always)]
        pub const fn devicetype(self) -> crate::common::Reg<regs::Devicetype, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trimcnf {
        ptr: *mut u8,
    }
    unsafe impl Send for Trimcnf {}
    unsafe impl Sync for Trimcnf {}
    impl Trimcnf {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster: Address"]
        #[inline(always)]
        pub const fn addr(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Description cluster: Data"]
        #[inline(always)]
        pub const fn data(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Code memory page size in bytes"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Codepagesize(pub u32);
        impl Codepagesize {
            #[doc = "Code memory page size in bytes"]
            #[must_use]
            #[inline(always)]
            pub const fn codepagesize(&self) -> super::vals::Codepagesize {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Codepagesize::from_bits(val as u32)
            }
            #[doc = "Code memory page size in bytes"]
            #[inline(always)]
            pub const fn set_codepagesize(&mut self, val: super::vals::Codepagesize) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Codepagesize {
            #[inline(always)]
            fn default() -> Codepagesize {
                Codepagesize(0)
            }
        }
        impl core::fmt::Debug for Codepagesize {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Codepagesize")
                    .field("codepagesize", &self.codepagesize())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Codepagesize {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Codepagesize {{ codepagesize: {:?} }}",
                    self.codepagesize()
                )
            }
        }
        #[doc = "Code memory size"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Codesize(pub u32);
        impl Codesize {
            #[doc = "Code memory size in number of pages"]
            #[must_use]
            #[inline(always)]
            pub const fn codesize(&self) -> super::vals::Codesize {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Codesize::from_bits(val as u32)
            }
            #[doc = "Code memory size in number of pages"]
            #[inline(always)]
            pub const fn set_codesize(&mut self, val: super::vals::Codesize) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Codesize {
            #[inline(always)]
            fn default() -> Codesize {
                Codesize(0)
            }
        }
        impl core::fmt::Debug for Codesize {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Codesize")
                    .field("codesize", &self.codesize())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Codesize {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Codesize {{ codesize: {:?} }}", self.codesize())
            }
        }
        #[doc = "Configuration identifier"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Configid(pub u32);
        impl Configid {
            #[doc = "Identification number for the HW"]
            #[must_use]
            #[inline(always)]
            pub const fn hwid(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Identification number for the HW"]
            #[inline(always)]
            pub const fn set_hwid(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Configid {
            #[inline(always)]
            fn default() -> Configid {
                Configid(0)
            }
        }
        impl core::fmt::Debug for Configid {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Configid")
                    .field("hwid", &self.hwid())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Configid {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Configid {{ hwid: {=u16:?} }}", self.hwid())
            }
        }
        #[doc = "Device address type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Deviceaddrtype(pub u32);
        impl Deviceaddrtype {
            #[doc = "Device address type"]
            #[must_use]
            #[inline(always)]
            pub const fn deviceaddrtype(&self) -> super::vals::Deviceaddrtype {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Deviceaddrtype::from_bits(val as u8)
            }
            #[doc = "Device address type"]
            #[inline(always)]
            pub const fn set_deviceaddrtype(&mut self, val: super::vals::Deviceaddrtype) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Deviceaddrtype {
            #[inline(always)]
            fn default() -> Deviceaddrtype {
                Deviceaddrtype(0)
            }
        }
        impl core::fmt::Debug for Deviceaddrtype {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Deviceaddrtype")
                    .field("deviceaddrtype", &self.deviceaddrtype())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Deviceaddrtype {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Deviceaddrtype {{ deviceaddrtype: {:?} }}",
                    self.deviceaddrtype()
                )
            }
        }
        #[doc = "Device type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Devicetype(pub u32);
        impl Devicetype {
            #[doc = "Device type"]
            #[must_use]
            #[inline(always)]
            pub const fn devicetype(&self) -> super::vals::Devicetype {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Devicetype::from_bits(val as u32)
            }
            #[doc = "Device type"]
            #[inline(always)]
            pub const fn set_devicetype(&mut self, val: super::vals::Devicetype) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Devicetype {
            #[inline(always)]
            fn default() -> Devicetype {
                Devicetype(0)
            }
        }
        impl core::fmt::Debug for Devicetype {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Devicetype")
                    .field("devicetype", &self.devicetype())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Devicetype {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Devicetype {{ devicetype: {:?} }}", self.devicetype())
            }
        }
        #[doc = "Flash variant"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Flash(pub u32);
        impl Flash {
            #[doc = "Flash variant"]
            #[must_use]
            #[inline(always)]
            pub const fn flash(&self) -> super::vals::Flash {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Flash::from_bits(val as u32)
            }
            #[doc = "Flash variant"]
            #[inline(always)]
            pub const fn set_flash(&mut self, val: super::vals::Flash) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Flash {
            #[inline(always)]
            fn default() -> Flash {
                Flash(0)
            }
        }
        impl core::fmt::Debug for Flash {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Flash")
                    .field("flash", &self.flash())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Flash {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Flash {{ flash: {:?} }}", self.flash())
            }
        }
        #[doc = "Package option"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Package(pub u32);
        impl Package {
            #[doc = "Package option"]
            #[must_use]
            #[inline(always)]
            pub const fn package(&self) -> super::vals::Package {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Package::from_bits(val as u32)
            }
            #[doc = "Package option"]
            #[inline(always)]
            pub const fn set_package(&mut self, val: super::vals::Package) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Package {
            #[inline(always)]
            fn default() -> Package {
                Package(0)
            }
        }
        impl core::fmt::Debug for Package {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Package")
                    .field("package", &self.package())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Package {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Package {{ package: {:?} }}", self.package())
            }
        }
        #[doc = "Part code"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Part(pub u32);
        impl Part {
            #[doc = "Part code"]
            #[must_use]
            #[inline(always)]
            pub const fn part(&self) -> super::vals::Part {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Part::from_bits(val as u32)
            }
            #[doc = "Part code"]
            #[inline(always)]
            pub const fn set_part(&mut self, val: super::vals::Part) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Part {
            #[inline(always)]
            fn default() -> Part {
                Part(0)
            }
        }
        impl core::fmt::Debug for Part {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Part").field("part", &self.part()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Part {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Part {{ part: {:?} }}", self.part())
            }
        }
        #[doc = "RAM variant"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ram(pub u32);
        impl Ram {
            #[doc = "RAM variant"]
            #[must_use]
            #[inline(always)]
            pub const fn ram(&self) -> super::vals::Ram {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Ram::from_bits(val as u32)
            }
            #[doc = "RAM variant"]
            #[inline(always)]
            pub const fn set_ram(&mut self, val: super::vals::Ram) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Ram {
            #[inline(always)]
            fn default() -> Ram {
                Ram(0)
            }
        }
        impl core::fmt::Debug for Ram {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ram").field("ram", &self.ram()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ram {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Ram {{ ram: {:?} }}", self.ram())
            }
        }
        #[doc = "Part Variant, Hardware version and Production configuration"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Variant(pub u32);
        impl Variant {
            #[doc = "Part Variant, Hardware version and Production configuration, encoded as ASCII"]
            #[must_use]
            #[inline(always)]
            pub const fn variant(&self) -> super::vals::Variant {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Variant::from_bits(val as u32)
            }
            #[doc = "Part Variant, Hardware version and Production configuration, encoded as ASCII"]
            #[inline(always)]
            pub const fn set_variant(&mut self, val: super::vals::Variant) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Variant {
            #[inline(always)]
            fn default() -> Variant {
                Variant(0)
            }
        }
        impl core::fmt::Debug for Variant {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Variant")
                    .field("variant", &self.variant())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Variant {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Variant {{ variant: {:?} }}", self.variant())
            }
        }
    }
    pub mod vals {
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Codepagesize(u32);
        impl Codepagesize {
            #[doc = "2 kByte"]
            pub const K2048: Self = Self(0x0800);
        }
        impl Codepagesize {
            pub const fn from_bits(val: u32) -> Codepagesize {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Codepagesize {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0800 => f.write_str("K2048"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Codepagesize {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0800 => defmt::write!(f, "K2048"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for Codepagesize {
            #[inline(always)]
            fn from(val: u32) -> Codepagesize {
                Codepagesize::from_bits(val)
            }
        }
        impl From<Codepagesize> for u32 {
            #[inline(always)]
            fn from(val: Codepagesize) -> u32 {
                Codepagesize::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Codesize(u32);
        impl Codesize {
            #[doc = "128 pages"]
            pub const P128: Self = Self(0x80);
        }
        impl Codesize {
            pub const fn from_bits(val: u32) -> Codesize {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Codesize {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x80 => f.write_str("P128"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Codesize {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x80 => defmt::write!(f, "P128"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for Codesize {
            #[inline(always)]
            fn from(val: u32) -> Codesize {
                Codesize::from_bits(val)
            }
        }
        impl From<Codesize> for u32 {
            #[inline(always)]
            fn from(val: Codesize) -> u32 {
                Codesize::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Deviceaddrtype {
            #[doc = "Public address"]
            PUBLIC = 0x0,
            #[doc = "Random address"]
            RANDOM = 0x01,
        }
        impl Deviceaddrtype {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Deviceaddrtype {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Deviceaddrtype {
            #[inline(always)]
            fn from(val: u8) -> Deviceaddrtype {
                Deviceaddrtype::from_bits(val)
            }
        }
        impl From<Deviceaddrtype> for u8 {
            #[inline(always)]
            fn from(val: Deviceaddrtype) -> u8 {
                Deviceaddrtype::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Devicetype(u32);
        impl Devicetype {
            #[doc = "Device is an physical DIE"]
            pub const DIE: Self = Self(0x0);
            #[doc = "Device is an FPGA"]
            pub const FPGA: Self = Self(0xffff_ffff);
        }
        impl Devicetype {
            pub const fn from_bits(val: u32) -> Devicetype {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Devicetype {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0 => f.write_str("DIE"),
                    0xffff_ffff => f.write_str("FPGA"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Devicetype {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0 => defmt::write!(f, "DIE"),
                    0xffff_ffff => defmt::write!(f, "FPGA"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for Devicetype {
            #[inline(always)]
            fn from(val: u32) -> Devicetype {
                Devicetype::from_bits(val)
            }
        }
        impl From<Devicetype> for u32 {
            #[inline(always)]
            fn from(val: Devicetype) -> u32 {
                Devicetype::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Flash(u32);
        impl Flash {
            #[doc = "128 kByte FLASH"]
            pub const K128: Self = Self(0x80);
            #[doc = "256 kByte FLASH"]
            pub const K256: Self = Self(0x0100);
            #[doc = "512 kByte FLASH"]
            pub const K512: Self = Self(0x0200);
            #[doc = "1 MByte FLASH"]
            pub const K1024: Self = Self(0x0400);
            #[doc = "2 MByte FLASH"]
            pub const K2048: Self = Self(0x0800);
            #[doc = "Unspecified"]
            pub const UNSPECIFIED: Self = Self(0xffff_ffff);
        }
        impl Flash {
            pub const fn from_bits(val: u32) -> Flash {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Flash {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x80 => f.write_str("K128"),
                    0x0100 => f.write_str("K256"),
                    0x0200 => f.write_str("K512"),
                    0x0400 => f.write_str("K1024"),
                    0x0800 => f.write_str("K2048"),
                    0xffff_ffff => f.write_str("UNSPECIFIED"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Flash {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x80 => defmt::write!(f, "K128"),
                    0x0100 => defmt::write!(f, "K256"),
                    0x0200 => defmt::write!(f, "K512"),
                    0x0400 => defmt::write!(f, "K1024"),
                    0x0800 => defmt::write!(f, "K2048"),
                    0xffff_ffff => defmt::write!(f, "UNSPECIFIED"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for Flash {
            #[inline(always)]
            fn from(val: u32) -> Flash {
                Flash::from_bits(val)
            }
        }
        impl From<Flash> for u32 {
            #[inline(always)]
            fn from(val: Flash) -> u32 {
                Flash::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Package(u32);
        impl Package {
            #[doc = "QKxx - 94-pin aQFN"]
            pub const QK: Self = Self(0x2000);
            #[doc = "CLxx - WLCSP"]
            pub const CL: Self = Self(0x2005);
            #[doc = "Unspecified"]
            pub const UNSPECIFIED: Self = Self(0xffff_ffff);
        }
        impl Package {
            pub const fn from_bits(val: u32) -> Package {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Package {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x2000 => f.write_str("QK"),
                    0x2005 => f.write_str("CL"),
                    0xffff_ffff => f.write_str("UNSPECIFIED"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Package {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x2000 => defmt::write!(f, "QK"),
                    0x2005 => defmt::write!(f, "CL"),
                    0xffff_ffff => defmt::write!(f, "UNSPECIFIED"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for Package {
            #[inline(always)]
            fn from(val: u32) -> Package {
                Package::from_bits(val)
            }
        }
        impl From<Package> for u32 {
            #[inline(always)]
            fn from(val: Package) -> u32 {
                Package::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Part(u32);
        impl Part {
            #[doc = "nRF5340"]
            pub const N5340: Self = Self(0x5340);
            #[doc = "Unspecified"]
            pub const UNSPECIFIED: Self = Self(0xffff_ffff);
        }
        impl Part {
            pub const fn from_bits(val: u32) -> Part {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Part {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x5340 => f.write_str("N5340"),
                    0xffff_ffff => f.write_str("UNSPECIFIED"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Part {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x5340 => defmt::write!(f, "N5340"),
                    0xffff_ffff => defmt::write!(f, "UNSPECIFIED"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for Part {
            #[inline(always)]
            fn from(val: u32) -> Part {
                Part::from_bits(val)
            }
        }
        impl From<Part> for u32 {
            #[inline(always)]
            fn from(val: Part) -> u32 {
                Part::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ram(u32);
        impl Ram {
            #[doc = "16 kByte RAM"]
            pub const K16: Self = Self(0x10);
            #[doc = "32 kByte RAM"]
            pub const K32: Self = Self(0x20);
            #[doc = "64 kByte RAM"]
            pub const K64: Self = Self(0x40);
            #[doc = "128 kByte RAM"]
            pub const K128: Self = Self(0x80);
            #[doc = "256 kByte RAM"]
            pub const K256: Self = Self(0x0100);
            #[doc = "512 kByte RAM"]
            pub const K512: Self = Self(0x0200);
            #[doc = "Unspecified"]
            pub const UNSPECIFIED: Self = Self(0xffff_ffff);
        }
        impl Ram {
            pub const fn from_bits(val: u32) -> Ram {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Ram {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x10 => f.write_str("K16"),
                    0x20 => f.write_str("K32"),
                    0x40 => f.write_str("K64"),
                    0x80 => f.write_str("K128"),
                    0x0100 => f.write_str("K256"),
                    0x0200 => f.write_str("K512"),
                    0xffff_ffff => f.write_str("UNSPECIFIED"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ram {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x10 => defmt::write!(f, "K16"),
                    0x20 => defmt::write!(f, "K32"),
                    0x40 => defmt::write!(f, "K64"),
                    0x80 => defmt::write!(f, "K128"),
                    0x0100 => defmt::write!(f, "K256"),
                    0x0200 => defmt::write!(f, "K512"),
                    0xffff_ffff => defmt::write!(f, "UNSPECIFIED"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for Ram {
            #[inline(always)]
            fn from(val: u32) -> Ram {
                Ram::from_bits(val)
            }
        }
        impl From<Ram> for u32 {
            #[inline(always)]
            fn from(val: Ram) -> u32 {
                Ram::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Variant(u32);
        impl Variant {
            #[doc = "CLAA"]
            pub const CLAA: Self = Self(0x434c_4141);
            #[doc = "QKAA"]
            pub const QKAA: Self = Self(0x514b_4141);
            #[doc = "Unspecified"]
            pub const UNSPECIFIED: Self = Self(0xffff_ffff);
        }
        impl Variant {
            pub const fn from_bits(val: u32) -> Variant {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Variant {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x434c_4141 => f.write_str("CLAA"),
                    0x514b_4141 => f.write_str("QKAA"),
                    0xffff_ffff => f.write_str("UNSPECIFIED"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Variant {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x434c_4141 => defmt::write!(f, "CLAA"),
                    0x514b_4141 => defmt::write!(f, "QKAA"),
                    0xffff_ffff => defmt::write!(f, "UNSPECIFIED"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for Variant {
            #[inline(always)]
            fn from(val: u32) -> Variant {
                Variant::from_bits(val)
            }
        }
        impl From<Variant> for u32 {
            #[inline(always)]
            fn from(val: Variant) -> u32 {
                Variant::to_bits(val)
            }
        }
    }
}
pub mod gpio {
    #[doc = "GPIO Port 0"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpio {
        ptr: *mut u8,
    }
    unsafe impl Send for Gpio {}
    unsafe impl Sync for Gpio {}
    impl Gpio {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Write GPIO port"]
        #[inline(always)]
        pub const fn out(self) -> crate::common::Reg<regs::Out, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Set individual bits in GPIO port"]
        #[inline(always)]
        pub const fn outset(self) -> crate::common::Reg<regs::Outset, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "Clear individual bits in GPIO port"]
        #[inline(always)]
        pub const fn outclr(self) -> crate::common::Reg<regs::Outclr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
        #[doc = "Read GPIO port"]
        #[inline(always)]
        pub const fn in_(self) -> crate::common::Reg<regs::In, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
        }
        #[doc = "Direction of GPIO pins"]
        #[inline(always)]
        pub const fn dir(self) -> crate::common::Reg<regs::Dir, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
        }
        #[doc = "DIR set register"]
        #[inline(always)]
        pub const fn dirset(self) -> crate::common::Reg<regs::Dirset, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
        }
        #[doc = "DIR clear register"]
        #[inline(always)]
        pub const fn dirclr(self) -> crate::common::Reg<regs::Dirclr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
        }
        #[doc = "Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF\\[n\\].SENSE registers"]
        #[inline(always)]
        pub const fn latch(self) -> crate::common::Reg<regs::Latch, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
        }
        #[doc = "Select between default DETECT signal behavior and LDETECT mode (For non-secure pin only)"]
        #[inline(always)]
        pub const fn detectmode(self) -> crate::common::Reg<regs::Detectmode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
        }
        #[doc = "Select between default DETECT signal behavior and LDETECT mode (For secure pin only)"]
        #[inline(always)]
        pub const fn detectmode_sec(
            self,
        ) -> crate::common::Reg<regs::DetectmodeSec, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
        }
        #[doc = "Description collection: Configuration of GPIO pins"]
        #[inline(always)]
        pub const fn pin_cnf(
            self,
            n: usize,
        ) -> crate::common::Reg<regs::PinCnf, crate::common::RW> {
            assert!(n < 32usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
            }
        }
    }
    pub mod regs {
        #[doc = "Select between default DETECT signal behavior and LDETECT mode (For non-secure pin only)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Detectmode(pub u32);
        impl Detectmode {
            #[doc = "Select between default DETECT signal behavior and LDETECT mode"]
            #[must_use]
            #[inline(always)]
            pub const fn detectmode(&self) -> super::vals::Detectmode {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Detectmode::from_bits(val as u8)
            }
            #[doc = "Select between default DETECT signal behavior and LDETECT mode"]
            #[inline(always)]
            pub const fn set_detectmode(&mut self, val: super::vals::Detectmode) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Detectmode {
            #[inline(always)]
            fn default() -> Detectmode {
                Detectmode(0)
            }
        }
        impl core::fmt::Debug for Detectmode {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Detectmode")
                    .field("detectmode", &self.detectmode())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Detectmode {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Detectmode {{ detectmode: {:?} }}", self.detectmode())
            }
        }
        #[doc = "Select between default DETECT signal behavior and LDETECT mode (For secure pin only)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DetectmodeSec(pub u32);
        impl DetectmodeSec {
            #[doc = "Select between default DETECT signal behavior and LDETECT mode"]
            #[must_use]
            #[inline(always)]
            pub const fn detectmode(&self) -> super::vals::Detectmode {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Detectmode::from_bits(val as u8)
            }
            #[doc = "Select between default DETECT signal behavior and LDETECT mode"]
            #[inline(always)]
            pub const fn set_detectmode(&mut self, val: super::vals::Detectmode) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for DetectmodeSec {
            #[inline(always)]
            fn default() -> DetectmodeSec {
                DetectmodeSec(0)
            }
        }
        impl core::fmt::Debug for DetectmodeSec {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DetectmodeSec")
                    .field("detectmode", &self.detectmode())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DetectmodeSec {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "DetectmodeSec {{ detectmode: {:?} }}", self.detectmode())
            }
        }
        #[doc = "Direction of GPIO pins"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dir(pub u32);
        impl Dir {
            #[doc = "Pin 0"]
            #[must_use]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> super::vals::Dir {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Dir::from_bits(val as u8)
            }
            #[doc = "Pin 0"]
            #[inline(always)]
            pub const fn set_pin(&mut self, n: usize, val: super::vals::Dir) {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
            }
        }
        impl Default for Dir {
            #[inline(always)]
            fn default() -> Dir {
                Dir(0)
            }
        }
        impl core::fmt::Debug for Dir {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dir")
                    .field("pin[0]", &self.pin(0usize))
                    .field("pin[1]", &self.pin(1usize))
                    .field("pin[2]", &self.pin(2usize))
                    .field("pin[3]", &self.pin(3usize))
                    .field("pin[4]", &self.pin(4usize))
                    .field("pin[5]", &self.pin(5usize))
                    .field("pin[6]", &self.pin(6usize))
                    .field("pin[7]", &self.pin(7usize))
                    .field("pin[8]", &self.pin(8usize))
                    .field("pin[9]", &self.pin(9usize))
                    .field("pin[10]", &self.pin(10usize))
                    .field("pin[11]", &self.pin(11usize))
                    .field("pin[12]", &self.pin(12usize))
                    .field("pin[13]", &self.pin(13usize))
                    .field("pin[14]", &self.pin(14usize))
                    .field("pin[15]", &self.pin(15usize))
                    .field("pin[16]", &self.pin(16usize))
                    .field("pin[17]", &self.pin(17usize))
                    .field("pin[18]", &self.pin(18usize))
                    .field("pin[19]", &self.pin(19usize))
                    .field("pin[20]", &self.pin(20usize))
                    .field("pin[21]", &self.pin(21usize))
                    .field("pin[22]", &self.pin(22usize))
                    .field("pin[23]", &self.pin(23usize))
                    .field("pin[24]", &self.pin(24usize))
                    .field("pin[25]", &self.pin(25usize))
                    .field("pin[26]", &self.pin(26usize))
                    .field("pin[27]", &self.pin(27usize))
                    .field("pin[28]", &self.pin(28usize))
                    .field("pin[29]", &self.pin(29usize))
                    .field("pin[30]", &self.pin(30usize))
                    .field("pin[31]", &self.pin(31usize))
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dir {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Dir {{ pin[0]: {:?}, pin[1]: {:?}, pin[2]: {:?}, pin[3]: {:?}, pin[4]: {:?}, pin[5]: {:?}, pin[6]: {:?}, pin[7]: {:?}, pin[8]: {:?}, pin[9]: {:?}, pin[10]: {:?}, pin[11]: {:?}, pin[12]: {:?}, pin[13]: {:?}, pin[14]: {:?}, pin[15]: {:?}, pin[16]: {:?}, pin[17]: {:?}, pin[18]: {:?}, pin[19]: {:?}, pin[20]: {:?}, pin[21]: {:?}, pin[22]: {:?}, pin[23]: {:?}, pin[24]: {:?}, pin[25]: {:?}, pin[26]: {:?}, pin[27]: {:?}, pin[28]: {:?}, pin[29]: {:?}, pin[30]: {:?}, pin[31]: {:?} }}" , self . pin (0usize) , self . pin (1usize) , self . pin (2usize) , self . pin (3usize) , self . pin (4usize) , self . pin (5usize) , self . pin (6usize) , self . pin (7usize) , self . pin (8usize) , self . pin (9usize) , self . pin (10usize) , self . pin (11usize) , self . pin (12usize) , self . pin (13usize) , self . pin (14usize) , self . pin (15usize) , self . pin (16usize) , self . pin (17usize) , self . pin (18usize) , self . pin (19usize) , self . pin (20usize) , self . pin (21usize) , self . pin (22usize) , self . pin (23usize) , self . pin (24usize) , self . pin (25usize) , self . pin (26usize) , self . pin (27usize) , self . pin (28usize) , self . pin (29usize) , self . pin (30usize) , self . pin (31usize))
            }
        }
        #[doc = "DIR clear register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dirclr(pub u32);
        impl Dirclr {
            #[doc = "Set as input pin 0"]
            #[must_use]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Set as input pin 0"]
            #[inline(always)]
            pub const fn set_pin(&mut self, n: usize, val: bool) {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Dirclr {
            #[inline(always)]
            fn default() -> Dirclr {
                Dirclr(0)
            }
        }
        impl core::fmt::Debug for Dirclr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dirclr")
                    .field("pin[0]", &self.pin(0usize))
                    .field("pin[1]", &self.pin(1usize))
                    .field("pin[2]", &self.pin(2usize))
                    .field("pin[3]", &self.pin(3usize))
                    .field("pin[4]", &self.pin(4usize))
                    .field("pin[5]", &self.pin(5usize))
                    .field("pin[6]", &self.pin(6usize))
                    .field("pin[7]", &self.pin(7usize))
                    .field("pin[8]", &self.pin(8usize))
                    .field("pin[9]", &self.pin(9usize))
                    .field("pin[10]", &self.pin(10usize))
                    .field("pin[11]", &self.pin(11usize))
                    .field("pin[12]", &self.pin(12usize))
                    .field("pin[13]", &self.pin(13usize))
                    .field("pin[14]", &self.pin(14usize))
                    .field("pin[15]", &self.pin(15usize))
                    .field("pin[16]", &self.pin(16usize))
                    .field("pin[17]", &self.pin(17usize))
                    .field("pin[18]", &self.pin(18usize))
                    .field("pin[19]", &self.pin(19usize))
                    .field("pin[20]", &self.pin(20usize))
                    .field("pin[21]", &self.pin(21usize))
                    .field("pin[22]", &self.pin(22usize))
                    .field("pin[23]", &self.pin(23usize))
                    .field("pin[24]", &self.pin(24usize))
                    .field("pin[25]", &self.pin(25usize))
                    .field("pin[26]", &self.pin(26usize))
                    .field("pin[27]", &self.pin(27usize))
                    .field("pin[28]", &self.pin(28usize))
                    .field("pin[29]", &self.pin(29usize))
                    .field("pin[30]", &self.pin(30usize))
                    .field("pin[31]", &self.pin(31usize))
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dirclr {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Dirclr {{ pin[0]: {=bool:?}, pin[1]: {=bool:?}, pin[2]: {=bool:?}, pin[3]: {=bool:?}, pin[4]: {=bool:?}, pin[5]: {=bool:?}, pin[6]: {=bool:?}, pin[7]: {=bool:?}, pin[8]: {=bool:?}, pin[9]: {=bool:?}, pin[10]: {=bool:?}, pin[11]: {=bool:?}, pin[12]: {=bool:?}, pin[13]: {=bool:?}, pin[14]: {=bool:?}, pin[15]: {=bool:?}, pin[16]: {=bool:?}, pin[17]: {=bool:?}, pin[18]: {=bool:?}, pin[19]: {=bool:?}, pin[20]: {=bool:?}, pin[21]: {=bool:?}, pin[22]: {=bool:?}, pin[23]: {=bool:?}, pin[24]: {=bool:?}, pin[25]: {=bool:?}, pin[26]: {=bool:?}, pin[27]: {=bool:?}, pin[28]: {=bool:?}, pin[29]: {=bool:?}, pin[30]: {=bool:?}, pin[31]: {=bool:?} }}" , self . pin (0usize) , self . pin (1usize) , self . pin (2usize) , self . pin (3usize) , self . pin (4usize) , self . pin (5usize) , self . pin (6usize) , self . pin (7usize) , self . pin (8usize) , self . pin (9usize) , self . pin (10usize) , self . pin (11usize) , self . pin (12usize) , self . pin (13usize) , self . pin (14usize) , self . pin (15usize) , self . pin (16usize) , self . pin (17usize) , self . pin (18usize) , self . pin (19usize) , self . pin (20usize) , self . pin (21usize) , self . pin (22usize) , self . pin (23usize) , self . pin (24usize) , self . pin (25usize) , self . pin (26usize) , self . pin (27usize) , self . pin (28usize) , self . pin (29usize) , self . pin (30usize) , self . pin (31usize))
            }
        }
        #[doc = "DIR set register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dirset(pub u32);
        impl Dirset {
            #[doc = "Set as output pin 0"]
            #[must_use]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Set as output pin 0"]
            #[inline(always)]
            pub const fn set_pin(&mut self, n: usize, val: bool) {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Dirset {
            #[inline(always)]
            fn default() -> Dirset {
                Dirset(0)
            }
        }
        impl core::fmt::Debug for Dirset {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dirset")
                    .field("pin[0]", &self.pin(0usize))
                    .field("pin[1]", &self.pin(1usize))
                    .field("pin[2]", &self.pin(2usize))
                    .field("pin[3]", &self.pin(3usize))
                    .field("pin[4]", &self.pin(4usize))
                    .field("pin[5]", &self.pin(5usize))
                    .field("pin[6]", &self.pin(6usize))
                    .field("pin[7]", &self.pin(7usize))
                    .field("pin[8]", &self.pin(8usize))
                    .field("pin[9]", &self.pin(9usize))
                    .field("pin[10]", &self.pin(10usize))
                    .field("pin[11]", &self.pin(11usize))
                    .field("pin[12]", &self.pin(12usize))
                    .field("pin[13]", &self.pin(13usize))
                    .field("pin[14]", &self.pin(14usize))
                    .field("pin[15]", &self.pin(15usize))
                    .field("pin[16]", &self.pin(16usize))
                    .field("pin[17]", &self.pin(17usize))
                    .field("pin[18]", &self.pin(18usize))
                    .field("pin[19]", &self.pin(19usize))
                    .field("pin[20]", &self.pin(20usize))
                    .field("pin[21]", &self.pin(21usize))
                    .field("pin[22]", &self.pin(22usize))
                    .field("pin[23]", &self.pin(23usize))
                    .field("pin[24]", &self.pin(24usize))
                    .field("pin[25]", &self.pin(25usize))
                    .field("pin[26]", &self.pin(26usize))
                    .field("pin[27]", &self.pin(27usize))
                    .field("pin[28]", &self.pin(28usize))
                    .field("pin[29]", &self.pin(29usize))
                    .field("pin[30]", &self.pin(30usize))
                    .field("pin[31]", &self.pin(31usize))
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dirset {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Dirset {{ pin[0]: {=bool:?}, pin[1]: {=bool:?}, pin[2]: {=bool:?}, pin[3]: {=bool:?}, pin[4]: {=bool:?}, pin[5]: {=bool:?}, pin[6]: {=bool:?}, pin[7]: {=bool:?}, pin[8]: {=bool:?}, pin[9]: {=bool:?}, pin[10]: {=bool:?}, pin[11]: {=bool:?}, pin[12]: {=bool:?}, pin[13]: {=bool:?}, pin[14]: {=bool:?}, pin[15]: {=bool:?}, pin[16]: {=bool:?}, pin[17]: {=bool:?}, pin[18]: {=bool:?}, pin[19]: {=bool:?}, pin[20]: {=bool:?}, pin[21]: {=bool:?}, pin[22]: {=bool:?}, pin[23]: {=bool:?}, pin[24]: {=bool:?}, pin[25]: {=bool:?}, pin[26]: {=bool:?}, pin[27]: {=bool:?}, pin[28]: {=bool:?}, pin[29]: {=bool:?}, pin[30]: {=bool:?}, pin[31]: {=bool:?} }}" , self . pin (0usize) , self . pin (1usize) , self . pin (2usize) , self . pin (3usize) , self . pin (4usize) , self . pin (5usize) , self . pin (6usize) , self . pin (7usize) , self . pin (8usize) , self . pin (9usize) , self . pin (10usize) , self . pin (11usize) , self . pin (12usize) , self . pin (13usize) , self . pin (14usize) , self . pin (15usize) , self . pin (16usize) , self . pin (17usize) , self . pin (18usize) , self . pin (19usize) , self . pin (20usize) , self . pin (21usize) , self . pin (22usize) , self . pin (23usize) , self . pin (24usize) , self . pin (25usize) , self . pin (26usize) , self . pin (27usize) , self . pin (28usize) , self . pin (29usize) , self . pin (30usize) , self . pin (31usize))
            }
        }
        #[doc = "Read GPIO port"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct In(pub u32);
        impl In {
            #[doc = "Pin 0"]
            #[must_use]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Pin 0"]
            #[inline(always)]
            pub const fn set_pin(&mut self, n: usize, val: bool) {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for In {
            #[inline(always)]
            fn default() -> In {
                In(0)
            }
        }
        impl core::fmt::Debug for In {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("In")
                    .field("pin[0]", &self.pin(0usize))
                    .field("pin[1]", &self.pin(1usize))
                    .field("pin[2]", &self.pin(2usize))
                    .field("pin[3]", &self.pin(3usize))
                    .field("pin[4]", &self.pin(4usize))
                    .field("pin[5]", &self.pin(5usize))
                    .field("pin[6]", &self.pin(6usize))
                    .field("pin[7]", &self.pin(7usize))
                    .field("pin[8]", &self.pin(8usize))
                    .field("pin[9]", &self.pin(9usize))
                    .field("pin[10]", &self.pin(10usize))
                    .field("pin[11]", &self.pin(11usize))
                    .field("pin[12]", &self.pin(12usize))
                    .field("pin[13]", &self.pin(13usize))
                    .field("pin[14]", &self.pin(14usize))
                    .field("pin[15]", &self.pin(15usize))
                    .field("pin[16]", &self.pin(16usize))
                    .field("pin[17]", &self.pin(17usize))
                    .field("pin[18]", &self.pin(18usize))
                    .field("pin[19]", &self.pin(19usize))
                    .field("pin[20]", &self.pin(20usize))
                    .field("pin[21]", &self.pin(21usize))
                    .field("pin[22]", &self.pin(22usize))
                    .field("pin[23]", &self.pin(23usize))
                    .field("pin[24]", &self.pin(24usize))
                    .field("pin[25]", &self.pin(25usize))
                    .field("pin[26]", &self.pin(26usize))
                    .field("pin[27]", &self.pin(27usize))
                    .field("pin[28]", &self.pin(28usize))
                    .field("pin[29]", &self.pin(29usize))
                    .field("pin[30]", &self.pin(30usize))
                    .field("pin[31]", &self.pin(31usize))
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for In {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "In {{ pin[0]: {=bool:?}, pin[1]: {=bool:?}, pin[2]: {=bool:?}, pin[3]: {=bool:?}, pin[4]: {=bool:?}, pin[5]: {=bool:?}, pin[6]: {=bool:?}, pin[7]: {=bool:?}, pin[8]: {=bool:?}, pin[9]: {=bool:?}, pin[10]: {=bool:?}, pin[11]: {=bool:?}, pin[12]: {=bool:?}, pin[13]: {=bool:?}, pin[14]: {=bool:?}, pin[15]: {=bool:?}, pin[16]: {=bool:?}, pin[17]: {=bool:?}, pin[18]: {=bool:?}, pin[19]: {=bool:?}, pin[20]: {=bool:?}, pin[21]: {=bool:?}, pin[22]: {=bool:?}, pin[23]: {=bool:?}, pin[24]: {=bool:?}, pin[25]: {=bool:?}, pin[26]: {=bool:?}, pin[27]: {=bool:?}, pin[28]: {=bool:?}, pin[29]: {=bool:?}, pin[30]: {=bool:?}, pin[31]: {=bool:?} }}" , self . pin (0usize) , self . pin (1usize) , self . pin (2usize) , self . pin (3usize) , self . pin (4usize) , self . pin (5usize) , self . pin (6usize) , self . pin (7usize) , self . pin (8usize) , self . pin (9usize) , self . pin (10usize) , self . pin (11usize) , self . pin (12usize) , self . pin (13usize) , self . pin (14usize) , self . pin (15usize) , self . pin (16usize) , self . pin (17usize) , self . pin (18usize) , self . pin (19usize) , self . pin (20usize) , self . pin (21usize) , self . pin (22usize) , self . pin (23usize) , self . pin (24usize) , self . pin (25usize) , self . pin (26usize) , self . pin (27usize) , self . pin (28usize) , self . pin (29usize) , self . pin (30usize) , self . pin (31usize))
            }
        }
        #[doc = "Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF\\[n\\].SENSE registers"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Latch(pub u32);
        impl Latch {
            #[doc = "Status on whether PIN\\[0\\] has met criteria set in PIN_CNF\\[0\\].SENSE register. Write '1' to clear."]
            #[must_use]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Status on whether PIN\\[0\\] has met criteria set in PIN_CNF\\[0\\].SENSE register. Write '1' to clear."]
            #[inline(always)]
            pub const fn set_pin(&mut self, n: usize, val: bool) {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Latch {
            #[inline(always)]
            fn default() -> Latch {
                Latch(0)
            }
        }
        impl core::fmt::Debug for Latch {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Latch")
                    .field("pin[0]", &self.pin(0usize))
                    .field("pin[1]", &self.pin(1usize))
                    .field("pin[2]", &self.pin(2usize))
                    .field("pin[3]", &self.pin(3usize))
                    .field("pin[4]", &self.pin(4usize))
                    .field("pin[5]", &self.pin(5usize))
                    .field("pin[6]", &self.pin(6usize))
                    .field("pin[7]", &self.pin(7usize))
                    .field("pin[8]", &self.pin(8usize))
                    .field("pin[9]", &self.pin(9usize))
                    .field("pin[10]", &self.pin(10usize))
                    .field("pin[11]", &self.pin(11usize))
                    .field("pin[12]", &self.pin(12usize))
                    .field("pin[13]", &self.pin(13usize))
                    .field("pin[14]", &self.pin(14usize))
                    .field("pin[15]", &self.pin(15usize))
                    .field("pin[16]", &self.pin(16usize))
                    .field("pin[17]", &self.pin(17usize))
                    .field("pin[18]", &self.pin(18usize))
                    .field("pin[19]", &self.pin(19usize))
                    .field("pin[20]", &self.pin(20usize))
                    .field("pin[21]", &self.pin(21usize))
                    .field("pin[22]", &self.pin(22usize))
                    .field("pin[23]", &self.pin(23usize))
                    .field("pin[24]", &self.pin(24usize))
                    .field("pin[25]", &self.pin(25usize))
                    .field("pin[26]", &self.pin(26usize))
                    .field("pin[27]", &self.pin(27usize))
                    .field("pin[28]", &self.pin(28usize))
                    .field("pin[29]", &self.pin(29usize))
                    .field("pin[30]", &self.pin(30usize))
                    .field("pin[31]", &self.pin(31usize))
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Latch {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Latch {{ pin[0]: {=bool:?}, pin[1]: {=bool:?}, pin[2]: {=bool:?}, pin[3]: {=bool:?}, pin[4]: {=bool:?}, pin[5]: {=bool:?}, pin[6]: {=bool:?}, pin[7]: {=bool:?}, pin[8]: {=bool:?}, pin[9]: {=bool:?}, pin[10]: {=bool:?}, pin[11]: {=bool:?}, pin[12]: {=bool:?}, pin[13]: {=bool:?}, pin[14]: {=bool:?}, pin[15]: {=bool:?}, pin[16]: {=bool:?}, pin[17]: {=bool:?}, pin[18]: {=bool:?}, pin[19]: {=bool:?}, pin[20]: {=bool:?}, pin[21]: {=bool:?}, pin[22]: {=bool:?}, pin[23]: {=bool:?}, pin[24]: {=bool:?}, pin[25]: {=bool:?}, pin[26]: {=bool:?}, pin[27]: {=bool:?}, pin[28]: {=bool:?}, pin[29]: {=bool:?}, pin[30]: {=bool:?}, pin[31]: {=bool:?} }}" , self . pin (0usize) , self . pin (1usize) , self . pin (2usize) , self . pin (3usize) , self . pin (4usize) , self . pin (5usize) , self . pin (6usize) , self . pin (7usize) , self . pin (8usize) , self . pin (9usize) , self . pin (10usize) , self . pin (11usize) , self . pin (12usize) , self . pin (13usize) , self . pin (14usize) , self . pin (15usize) , self . pin (16usize) , self . pin (17usize) , self . pin (18usize) , self . pin (19usize) , self . pin (20usize) , self . pin (21usize) , self . pin (22usize) , self . pin (23usize) , self . pin (24usize) , self . pin (25usize) , self . pin (26usize) , self . pin (27usize) , self . pin (28usize) , self . pin (29usize) , self . pin (30usize) , self . pin (31usize))
            }
        }
        #[doc = "Write GPIO port"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Out(pub u32);
        impl Out {
            #[doc = "Pin 0"]
            #[must_use]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Pin 0"]
            #[inline(always)]
            pub const fn set_pin(&mut self, n: usize, val: bool) {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Out {
            #[inline(always)]
            fn default() -> Out {
                Out(0)
            }
        }
        impl core::fmt::Debug for Out {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Out")
                    .field("pin[0]", &self.pin(0usize))
                    .field("pin[1]", &self.pin(1usize))
                    .field("pin[2]", &self.pin(2usize))
                    .field("pin[3]", &self.pin(3usize))
                    .field("pin[4]", &self.pin(4usize))
                    .field("pin[5]", &self.pin(5usize))
                    .field("pin[6]", &self.pin(6usize))
                    .field("pin[7]", &self.pin(7usize))
                    .field("pin[8]", &self.pin(8usize))
                    .field("pin[9]", &self.pin(9usize))
                    .field("pin[10]", &self.pin(10usize))
                    .field("pin[11]", &self.pin(11usize))
                    .field("pin[12]", &self.pin(12usize))
                    .field("pin[13]", &self.pin(13usize))
                    .field("pin[14]", &self.pin(14usize))
                    .field("pin[15]", &self.pin(15usize))
                    .field("pin[16]", &self.pin(16usize))
                    .field("pin[17]", &self.pin(17usize))
                    .field("pin[18]", &self.pin(18usize))
                    .field("pin[19]", &self.pin(19usize))
                    .field("pin[20]", &self.pin(20usize))
                    .field("pin[21]", &self.pin(21usize))
                    .field("pin[22]", &self.pin(22usize))
                    .field("pin[23]", &self.pin(23usize))
                    .field("pin[24]", &self.pin(24usize))
                    .field("pin[25]", &self.pin(25usize))
                    .field("pin[26]", &self.pin(26usize))
                    .field("pin[27]", &self.pin(27usize))
                    .field("pin[28]", &self.pin(28usize))
                    .field("pin[29]", &self.pin(29usize))
                    .field("pin[30]", &self.pin(30usize))
                    .field("pin[31]", &self.pin(31usize))
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Out {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Out {{ pin[0]: {=bool:?}, pin[1]: {=bool:?}, pin[2]: {=bool:?}, pin[3]: {=bool:?}, pin[4]: {=bool:?}, pin[5]: {=bool:?}, pin[6]: {=bool:?}, pin[7]: {=bool:?}, pin[8]: {=bool:?}, pin[9]: {=bool:?}, pin[10]: {=bool:?}, pin[11]: {=bool:?}, pin[12]: {=bool:?}, pin[13]: {=bool:?}, pin[14]: {=bool:?}, pin[15]: {=bool:?}, pin[16]: {=bool:?}, pin[17]: {=bool:?}, pin[18]: {=bool:?}, pin[19]: {=bool:?}, pin[20]: {=bool:?}, pin[21]: {=bool:?}, pin[22]: {=bool:?}, pin[23]: {=bool:?}, pin[24]: {=bool:?}, pin[25]: {=bool:?}, pin[26]: {=bool:?}, pin[27]: {=bool:?}, pin[28]: {=bool:?}, pin[29]: {=bool:?}, pin[30]: {=bool:?}, pin[31]: {=bool:?} }}" , self . pin (0usize) , self . pin (1usize) , self . pin (2usize) , self . pin (3usize) , self . pin (4usize) , self . pin (5usize) , self . pin (6usize) , self . pin (7usize) , self . pin (8usize) , self . pin (9usize) , self . pin (10usize) , self . pin (11usize) , self . pin (12usize) , self . pin (13usize) , self . pin (14usize) , self . pin (15usize) , self . pin (16usize) , self . pin (17usize) , self . pin (18usize) , self . pin (19usize) , self . pin (20usize) , self . pin (21usize) , self . pin (22usize) , self . pin (23usize) , self . pin (24usize) , self . pin (25usize) , self . pin (26usize) , self . pin (27usize) , self . pin (28usize) , self . pin (29usize) , self . pin (30usize) , self . pin (31usize))
            }
        }
        #[doc = "Clear individual bits in GPIO port"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Outclr(pub u32);
        impl Outclr {
            #[doc = "Pin 0"]
            #[must_use]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Pin 0"]
            #[inline(always)]
            pub const fn set_pin(&mut self, n: usize, val: bool) {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Outclr {
            #[inline(always)]
            fn default() -> Outclr {
                Outclr(0)
            }
        }
        impl core::fmt::Debug for Outclr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Outclr")
                    .field("pin[0]", &self.pin(0usize))
                    .field("pin[1]", &self.pin(1usize))
                    .field("pin[2]", &self.pin(2usize))
                    .field("pin[3]", &self.pin(3usize))
                    .field("pin[4]", &self.pin(4usize))
                    .field("pin[5]", &self.pin(5usize))
                    .field("pin[6]", &self.pin(6usize))
                    .field("pin[7]", &self.pin(7usize))
                    .field("pin[8]", &self.pin(8usize))
                    .field("pin[9]", &self.pin(9usize))
                    .field("pin[10]", &self.pin(10usize))
                    .field("pin[11]", &self.pin(11usize))
                    .field("pin[12]", &self.pin(12usize))
                    .field("pin[13]", &self.pin(13usize))
                    .field("pin[14]", &self.pin(14usize))
                    .field("pin[15]", &self.pin(15usize))
                    .field("pin[16]", &self.pin(16usize))
                    .field("pin[17]", &self.pin(17usize))
                    .field("pin[18]", &self.pin(18usize))
                    .field("pin[19]", &self.pin(19usize))
                    .field("pin[20]", &self.pin(20usize))
                    .field("pin[21]", &self.pin(21usize))
                    .field("pin[22]", &self.pin(22usize))
                    .field("pin[23]", &self.pin(23usize))
                    .field("pin[24]", &self.pin(24usize))
                    .field("pin[25]", &self.pin(25usize))
                    .field("pin[26]", &self.pin(26usize))
                    .field("pin[27]", &self.pin(27usize))
                    .field("pin[28]", &self.pin(28usize))
                    .field("pin[29]", &self.pin(29usize))
                    .field("pin[30]", &self.pin(30usize))
                    .field("pin[31]", &self.pin(31usize))
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Outclr {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Outclr {{ pin[0]: {=bool:?}, pin[1]: {=bool:?}, pin[2]: {=bool:?}, pin[3]: {=bool:?}, pin[4]: {=bool:?}, pin[5]: {=bool:?}, pin[6]: {=bool:?}, pin[7]: {=bool:?}, pin[8]: {=bool:?}, pin[9]: {=bool:?}, pin[10]: {=bool:?}, pin[11]: {=bool:?}, pin[12]: {=bool:?}, pin[13]: {=bool:?}, pin[14]: {=bool:?}, pin[15]: {=bool:?}, pin[16]: {=bool:?}, pin[17]: {=bool:?}, pin[18]: {=bool:?}, pin[19]: {=bool:?}, pin[20]: {=bool:?}, pin[21]: {=bool:?}, pin[22]: {=bool:?}, pin[23]: {=bool:?}, pin[24]: {=bool:?}, pin[25]: {=bool:?}, pin[26]: {=bool:?}, pin[27]: {=bool:?}, pin[28]: {=bool:?}, pin[29]: {=bool:?}, pin[30]: {=bool:?}, pin[31]: {=bool:?} }}" , self . pin (0usize) , self . pin (1usize) , self . pin (2usize) , self . pin (3usize) , self . pin (4usize) , self . pin (5usize) , self . pin (6usize) , self . pin (7usize) , self . pin (8usize) , self . pin (9usize) , self . pin (10usize) , self . pin (11usize) , self . pin (12usize) , self . pin (13usize) , self . pin (14usize) , self . pin (15usize) , self . pin (16usize) , self . pin (17usize) , self . pin (18usize) , self . pin (19usize) , self . pin (20usize) , self . pin (21usize) , self . pin (22usize) , self . pin (23usize) , self . pin (24usize) , self . pin (25usize) , self . pin (26usize) , self . pin (27usize) , self . pin (28usize) , self . pin (29usize) , self . pin (30usize) , self . pin (31usize))
            }
        }
        #[doc = "Set individual bits in GPIO port"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Outset(pub u32);
        impl Outset {
            #[doc = "Pin 0"]
            #[must_use]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Pin 0"]
            #[inline(always)]
            pub const fn set_pin(&mut self, n: usize, val: bool) {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Outset {
            #[inline(always)]
            fn default() -> Outset {
                Outset(0)
            }
        }
        impl core::fmt::Debug for Outset {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Outset")
                    .field("pin[0]", &self.pin(0usize))
                    .field("pin[1]", &self.pin(1usize))
                    .field("pin[2]", &self.pin(2usize))
                    .field("pin[3]", &self.pin(3usize))
                    .field("pin[4]", &self.pin(4usize))
                    .field("pin[5]", &self.pin(5usize))
                    .field("pin[6]", &self.pin(6usize))
                    .field("pin[7]", &self.pin(7usize))
                    .field("pin[8]", &self.pin(8usize))
                    .field("pin[9]", &self.pin(9usize))
                    .field("pin[10]", &self.pin(10usize))
                    .field("pin[11]", &self.pin(11usize))
                    .field("pin[12]", &self.pin(12usize))
                    .field("pin[13]", &self.pin(13usize))
                    .field("pin[14]", &self.pin(14usize))
                    .field("pin[15]", &self.pin(15usize))
                    .field("pin[16]", &self.pin(16usize))
                    .field("pin[17]", &self.pin(17usize))
                    .field("pin[18]", &self.pin(18usize))
                    .field("pin[19]", &self.pin(19usize))
                    .field("pin[20]", &self.pin(20usize))
                    .field("pin[21]", &self.pin(21usize))
                    .field("pin[22]", &self.pin(22usize))
                    .field("pin[23]", &self.pin(23usize))
                    .field("pin[24]", &self.pin(24usize))
                    .field("pin[25]", &self.pin(25usize))
                    .field("pin[26]", &self.pin(26usize))
                    .field("pin[27]", &self.pin(27usize))
                    .field("pin[28]", &self.pin(28usize))
                    .field("pin[29]", &self.pin(29usize))
                    .field("pin[30]", &self.pin(30usize))
                    .field("pin[31]", &self.pin(31usize))
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Outset {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Outset {{ pin[0]: {=bool:?}, pin[1]: {=bool:?}, pin[2]: {=bool:?}, pin[3]: {=bool:?}, pin[4]: {=bool:?}, pin[5]: {=bool:?}, pin[6]: {=bool:?}, pin[7]: {=bool:?}, pin[8]: {=bool:?}, pin[9]: {=bool:?}, pin[10]: {=bool:?}, pin[11]: {=bool:?}, pin[12]: {=bool:?}, pin[13]: {=bool:?}, pin[14]: {=bool:?}, pin[15]: {=bool:?}, pin[16]: {=bool:?}, pin[17]: {=bool:?}, pin[18]: {=bool:?}, pin[19]: {=bool:?}, pin[20]: {=bool:?}, pin[21]: {=bool:?}, pin[22]: {=bool:?}, pin[23]: {=bool:?}, pin[24]: {=bool:?}, pin[25]: {=bool:?}, pin[26]: {=bool:?}, pin[27]: {=bool:?}, pin[28]: {=bool:?}, pin[29]: {=bool:?}, pin[30]: {=bool:?}, pin[31]: {=bool:?} }}" , self . pin (0usize) , self . pin (1usize) , self . pin (2usize) , self . pin (3usize) , self . pin (4usize) , self . pin (5usize) , self . pin (6usize) , self . pin (7usize) , self . pin (8usize) , self . pin (9usize) , self . pin (10usize) , self . pin (11usize) , self . pin (12usize) , self . pin (13usize) , self . pin (14usize) , self . pin (15usize) , self . pin (16usize) , self . pin (17usize) , self . pin (18usize) , self . pin (19usize) , self . pin (20usize) , self . pin (21usize) , self . pin (22usize) , self . pin (23usize) , self . pin (24usize) , self . pin (25usize) , self . pin (26usize) , self . pin (27usize) , self . pin (28usize) , self . pin (29usize) , self . pin (30usize) , self . pin (31usize))
            }
        }
        #[doc = "Description collection: Configuration of GPIO pins"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct PinCnf(pub u32);
        impl PinCnf {
            #[doc = "Pin direction. Same physical register as DIR register"]
            #[must_use]
            #[inline(always)]
            pub const fn dir(&self) -> super::vals::Dir {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Dir::from_bits(val as u8)
            }
            #[doc = "Pin direction. Same physical register as DIR register"]
            #[inline(always)]
            pub const fn set_dir(&mut self, val: super::vals::Dir) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Connect or disconnect input buffer"]
            #[must_use]
            #[inline(always)]
            pub const fn input(&self) -> super::vals::Input {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Input::from_bits(val as u8)
            }
            #[doc = "Connect or disconnect input buffer"]
            #[inline(always)]
            pub const fn set_input(&mut self, val: super::vals::Input) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Pull configuration"]
            #[must_use]
            #[inline(always)]
            pub const fn pull(&self) -> super::vals::Pull {
                let val = (self.0 >> 2usize) & 0x03;
                super::vals::Pull::from_bits(val as u8)
            }
            #[doc = "Pull configuration"]
            #[inline(always)]
            pub const fn set_pull(&mut self, val: super::vals::Pull) {
                self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
            }
            #[doc = "Drive configuration"]
            #[must_use]
            #[inline(always)]
            pub const fn drive(&self) -> super::vals::Drive {
                let val = (self.0 >> 8usize) & 0x0f;
                super::vals::Drive::from_bits(val as u8)
            }
            #[doc = "Drive configuration"]
            #[inline(always)]
            pub const fn set_drive(&mut self, val: super::vals::Drive) {
                self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
            }
            #[doc = "Pin sensing mechanism"]
            #[must_use]
            #[inline(always)]
            pub const fn sense(&self) -> super::vals::Sense {
                let val = (self.0 >> 16usize) & 0x03;
                super::vals::Sense::from_bits(val as u8)
            }
            #[doc = "Pin sensing mechanism"]
            #[inline(always)]
            pub const fn set_sense(&mut self, val: super::vals::Sense) {
                self.0 =
                    (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
            }
            #[doc = "Select which MCU/Subsystem controls this pin Note: this field is only accessible from secure code."]
            #[must_use]
            #[inline(always)]
            pub const fn mcusel(&self) -> super::vals::Mcusel {
                let val = (self.0 >> 28usize) & 0x07;
                super::vals::Mcusel::from_bits(val as u8)
            }
            #[doc = "Select which MCU/Subsystem controls this pin Note: this field is only accessible from secure code."]
            #[inline(always)]
            pub const fn set_mcusel(&mut self, val: super::vals::Mcusel) {
                self.0 =
                    (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
            }
        }
        impl Default for PinCnf {
            #[inline(always)]
            fn default() -> PinCnf {
                PinCnf(0)
            }
        }
        impl core::fmt::Debug for PinCnf {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("PinCnf")
                    .field("dir", &self.dir())
                    .field("input", &self.input())
                    .field("pull", &self.pull())
                    .field("drive", &self.drive())
                    .field("sense", &self.sense())
                    .field("mcusel", &self.mcusel())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for PinCnf {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "PinCnf {{ dir: {:?}, input: {:?}, pull: {:?}, drive: {:?}, sense: {:?}, mcusel: {:?} }}" , self . dir () , self . input () , self . pull () , self . drive () , self . sense () , self . mcusel ())
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Detectmode {
            #[doc = "DETECT directly connected to PIN DETECT signals"]
            DEFAULT = 0x0,
            #[doc = "Use the latched LDETECT behavior"]
            LDETECT = 0x01,
        }
        impl Detectmode {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Detectmode {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Detectmode {
            #[inline(always)]
            fn from(val: u8) -> Detectmode {
                Detectmode::from_bits(val)
            }
        }
        impl From<Detectmode> for u8 {
            #[inline(always)]
            fn from(val: Detectmode) -> u8 {
                Detectmode::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Dir {
            #[doc = "Configure pin as an input pin"]
            INPUT = 0x0,
            #[doc = "Configure pin as an output pin"]
            OUTPUT = 0x01,
        }
        impl Dir {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Dir {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Dir {
            #[inline(always)]
            fn from(val: u8) -> Dir {
                Dir::from_bits(val)
            }
        }
        impl From<Dir> for u8 {
            #[inline(always)]
            fn from(val: Dir) -> u8 {
                Dir::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Drive {
            #[doc = "Standard '0', standard '1'"]
            S0S1 = 0x0,
            #[doc = "High drive '0', standard '1'"]
            H0S1 = 0x01,
            #[doc = "Standard '0', high drive '1'"]
            S0H1 = 0x02,
            #[doc = "High drive '0', high 'drive '1''"]
            H0H1 = 0x03,
            #[doc = "Disconnect '0', standard '1' (normally used for wired-or connections)"]
            D0S1 = 0x04,
            #[doc = "Disconnect '0', high drive '1' (normally used for wired-or connections)"]
            D0H1 = 0x05,
            #[doc = "Standard '0', disconnect '1' (normally used for wired-and connections)"]
            S0D1 = 0x06,
            #[doc = "High drive '0', disconnect '1' (normally used for wired-and connections)"]
            H0D1 = 0x07,
            _RESERVED_8 = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            #[doc = "Extra high drive '0', extra high drive '1'"]
            E0E1 = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Drive {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Drive {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Drive {
            #[inline(always)]
            fn from(val: u8) -> Drive {
                Drive::from_bits(val)
            }
        }
        impl From<Drive> for u8 {
            #[inline(always)]
            fn from(val: Drive) -> u8 {
                Drive::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Input {
            #[doc = "Connect input buffer"]
            CONNECT = 0x0,
            #[doc = "Disconnect input buffer"]
            DISCONNECT = 0x01,
        }
        impl Input {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Input {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Input {
            #[inline(always)]
            fn from(val: u8) -> Input {
                Input::from_bits(val)
            }
        }
        impl From<Input> for u8 {
            #[inline(always)]
            fn from(val: Input) -> u8 {
                Input::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Mcusel {
            #[doc = "Application MCU"]
            APP_MCU = 0x0,
            #[doc = "Network MCU"]
            NETWORK_MCU = 0x01,
            _RESERVED_2 = 0x02,
            #[doc = "Peripheral with dedicated pins"]
            PERIPHERAL = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            #[doc = "Trace and Debug Subsystem"]
            TND = 0x07,
        }
        impl Mcusel {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Mcusel {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Mcusel {
            #[inline(always)]
            fn from(val: u8) -> Mcusel {
                Mcusel::from_bits(val)
            }
        }
        impl From<Mcusel> for u8 {
            #[inline(always)]
            fn from(val: Mcusel) -> u8 {
                Mcusel::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Pull {
            #[doc = "No pull"]
            DISABLED = 0x0,
            #[doc = "Pull down on pin"]
            PULLDOWN = 0x01,
            _RESERVED_2 = 0x02,
            #[doc = "Pull up on pin"]
            PULLUP = 0x03,
        }
        impl Pull {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Pull {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Pull {
            #[inline(always)]
            fn from(val: u8) -> Pull {
                Pull::from_bits(val)
            }
        }
        impl From<Pull> for u8 {
            #[inline(always)]
            fn from(val: Pull) -> u8 {
                Pull::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Sense {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            _RESERVED_1 = 0x01,
            #[doc = "Sense for high level"]
            HIGH = 0x02,
            #[doc = "Sense for low level"]
            LOW = 0x03,
        }
        impl Sense {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Sense {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Sense {
            #[inline(always)]
            fn from(val: u8) -> Sense {
                Sense::from_bits(val)
            }
        }
        impl From<Sense> for u8 {
            #[inline(always)]
            fn from(val: Sense) -> u8 {
                Sense::to_bits(val)
            }
        }
    }
}
pub mod gpiote {
    #[doc = "GPIO Tasks and Events"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpiote {
        ptr: *mut u8,
    }
    unsafe impl Send for Gpiote {}
    unsafe impl Sync for Gpiote {}
    impl Gpiote {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is configured in CONFIG\\[n\\].POLARITY."]
        #[inline(always)]
        pub const fn tasks_out(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
            assert!(n < 8usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _)
            }
        }
        #[doc = "Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it high."]
        #[inline(always)]
        pub const fn tasks_set(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
            assert!(n < 8usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize + n * 4usize) as _)
            }
        }
        #[doc = "Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it low."]
        #[inline(always)]
        pub const fn tasks_clr(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
            assert!(n < 8usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize + n * 4usize) as _)
            }
        }
        #[doc = "Description collection: Subscribe configuration for task OUT\\[n\\]"]
        #[inline(always)]
        pub const fn subscribe_out(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            assert!(n < 8usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _)
            }
        }
        #[doc = "Description collection: Subscribe configuration for task SET\\[n\\]"]
        #[inline(always)]
        pub const fn subscribe_set(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            assert!(n < 8usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize + n * 4usize) as _)
            }
        }
        #[doc = "Description collection: Subscribe configuration for task CLR\\[n\\]"]
        #[inline(always)]
        pub const fn subscribe_clr(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            assert!(n < 8usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize + n * 4usize) as _)
            }
        }
        #[doc = "Description collection: Event generated from pin specified in CONFIG\\[n\\].PSEL"]
        #[inline(always)]
        pub const fn events_in(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 8usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
            }
        }
        #[doc = "Event generated from multiple input GPIO pins with SENSE mechanism enabled"]
        #[inline(always)]
        pub const fn events_port(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
        }
        #[doc = "Description collection: Publish configuration for event IN\\[n\\]"]
        #[inline(always)]
        pub const fn publish_in(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            assert!(n < 8usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize + n * 4usize) as _)
            }
        }
        #[doc = "Publish configuration for event PORT"]
        #[inline(always)]
        pub const fn publish_port(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
        }
        #[doc = "Latency selection for Event mode (MODE=Event) with rising or falling edge detection on the pin."]
        #[inline(always)]
        pub const fn latency(self) -> crate::common::Reg<regs::Latency, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
        }
        #[doc = "Description collection: Configuration for OUT\\[n\\], SET\\[n\\], and CLR\\[n\\] tasks and IN\\[n\\] event"]
        #[inline(always)]
        pub const fn config(self, n: usize) -> crate::common::Reg<regs::Config, crate::common::RW> {
            assert!(n < 8usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0510usize + n * 4usize) as _)
            }
        }
    }
    pub mod regs {
        #[doc = "Description collection: Configuration for OUT\\[n\\], SET\\[n\\], and CLR\\[n\\] tasks and IN\\[n\\] event"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Mode"]
            #[must_use]
            #[inline(always)]
            pub const fn mode(&self) -> super::vals::Mode {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Mode::from_bits(val as u8)
            }
            #[doc = "Mode"]
            #[inline(always)]
            pub const fn set_mode(&mut self, val: super::vals::Mode) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
            #[doc = "GPIO number associated with SET\\[n\\], CLR\\[n\\], and OUT\\[n\\] tasks and IN\\[n\\] event"]
            #[must_use]
            #[inline(always)]
            pub const fn psel(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0x1f;
                val as u8
            }
            #[doc = "GPIO number associated with SET\\[n\\], CLR\\[n\\], and OUT\\[n\\] tasks and IN\\[n\\] event"]
            #[inline(always)]
            pub const fn set_psel(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
            }
            #[doc = "Port number"]
            #[must_use]
            #[inline(always)]
            pub const fn port(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Port number"]
            #[inline(always)]
            pub const fn set_port(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "When In task mode: Operation to be performed on output when OUT\\[n\\] task is triggered. When In event mode: Operation on input that shall trigger IN\\[n\\] event."]
            #[must_use]
            #[inline(always)]
            pub const fn polarity(&self) -> super::vals::Polarity {
                let val = (self.0 >> 16usize) & 0x03;
                super::vals::Polarity::from_bits(val as u8)
            }
            #[doc = "When In task mode: Operation to be performed on output when OUT\\[n\\] task is triggered. When In event mode: Operation on input that shall trigger IN\\[n\\] event."]
            #[inline(always)]
            pub const fn set_polarity(&mut self, val: super::vals::Polarity) {
                self.0 =
                    (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
            }
            #[doc = "When in task mode: Initial value of the output when the GPIOTE channel is configured. When in event mode: No effect."]
            #[must_use]
            #[inline(always)]
            pub const fn outinit(&self) -> super::vals::Outinit {
                let val = (self.0 >> 20usize) & 0x01;
                super::vals::Outinit::from_bits(val as u8)
            }
            #[doc = "When in task mode: Initial value of the output when the GPIOTE channel is configured. When in event mode: No effect."]
            #[inline(always)]
            pub const fn set_outinit(&mut self, val: super::vals::Outinit) {
                self.0 =
                    (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        impl core::fmt::Debug for Config {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Config")
                    .field("mode", &self.mode())
                    .field("psel", &self.psel())
                    .field("port", &self.port())
                    .field("polarity", &self.polarity())
                    .field("outinit", &self.outinit())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Config {{ mode: {:?}, psel: {=u8:?}, port: {=bool:?}, polarity: {:?}, outinit: {:?} }}" , self . mode () , self . psel () , self . port () , self . polarity () , self . outinit ())
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event IN\\[0\\]"]
            #[must_use]
            #[inline(always)]
            pub const fn in_(&self, n: usize) -> bool {
                assert!(n < 8usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event IN\\[0\\]"]
            #[inline(always)]
            pub const fn set_in_(&mut self, n: usize, val: bool) {
                assert!(n < 8usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Write '1' to disable interrupt for event PORT"]
            #[must_use]
            #[inline(always)]
            pub const fn port(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event PORT"]
            #[inline(always)]
            pub const fn set_port(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("in_[0]", &self.in_(0usize))
                    .field("in_[1]", &self.in_(1usize))
                    .field("in_[2]", &self.in_(2usize))
                    .field("in_[3]", &self.in_(3usize))
                    .field("in_[4]", &self.in_(4usize))
                    .field("in_[5]", &self.in_(5usize))
                    .field("in_[6]", &self.in_(6usize))
                    .field("in_[7]", &self.in_(7usize))
                    .field("port", &self.port())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Int {{ in_[0]: {=bool:?}, in_[1]: {=bool:?}, in_[2]: {=bool:?}, in_[3]: {=bool:?}, in_[4]: {=bool:?}, in_[5]: {=bool:?}, in_[6]: {=bool:?}, in_[7]: {=bool:?}, port: {=bool:?} }}" , self . in_ (0usize) , self . in_ (1usize) , self . in_ (2usize) , self . in_ (3usize) , self . in_ (4usize) , self . in_ (5usize) , self . in_ (6usize) , self . in_ (7usize) , self . port ())
            }
        }
        #[doc = "Latency selection for Event mode (MODE=Event) with rising or falling edge detection on the pin."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Latency(pub u32);
        impl Latency {
            #[doc = "Latency setting"]
            #[must_use]
            #[inline(always)]
            pub const fn latency(&self) -> super::vals::Latency {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Latency::from_bits(val as u8)
            }
            #[doc = "Latency setting"]
            #[inline(always)]
            pub const fn set_latency(&mut self, val: super::vals::Latency) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Latency {
            #[inline(always)]
            fn default() -> Latency {
                Latency(0)
            }
        }
        impl core::fmt::Debug for Latency {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Latency")
                    .field("latency", &self.latency())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Latency {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Latency {{ latency: {:?} }}", self.latency())
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Latency {
            #[doc = "Low power setting, for signals with minimum hold time tGPIOTE,HOLD,LP; refer to Electrical specification section"]
            LOW_POWER = 0x0,
            #[doc = "Low latency setting, for signals with minimum hold time tGPIOTE,HOLD,LL; refer to Electrical specification section"]
            LOW_LATENCY = 0x01,
        }
        impl Latency {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Latency {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Latency {
            #[inline(always)]
            fn from(val: u8) -> Latency {
                Latency::from_bits(val)
            }
        }
        impl From<Latency> for u8 {
            #[inline(always)]
            fn from(val: Latency) -> u8 {
                Latency::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Mode {
            #[doc = "Disabled. Pin specified by PSEL will not be acquired by the GPIOTE module."]
            DISABLED = 0x0,
            #[doc = "Event mode"]
            EVENT = 0x01,
            _RESERVED_2 = 0x02,
            #[doc = "Task mode"]
            TASK = 0x03,
        }
        impl Mode {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Mode {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Mode {
            #[inline(always)]
            fn from(val: u8) -> Mode {
                Mode::from_bits(val)
            }
        }
        impl From<Mode> for u8 {
            #[inline(always)]
            fn from(val: Mode) -> u8 {
                Mode::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Outinit {
            #[doc = "Task mode: Initial value of pin before task triggering is low"]
            LOW = 0x0,
            #[doc = "Task mode: Initial value of pin before task triggering is high"]
            HIGH = 0x01,
        }
        impl Outinit {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Outinit {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Outinit {
            #[inline(always)]
            fn from(val: u8) -> Outinit {
                Outinit::from_bits(val)
            }
        }
        impl From<Outinit> for u8 {
            #[inline(always)]
            fn from(val: Outinit) -> u8 {
                Outinit::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Polarity {
            #[doc = "Task mode: No effect on pin from OUT\\[n\\] task. Event mode: no IN\\[n\\] event generated on pin activity."]
            NONE = 0x0,
            #[doc = "Task mode: Set pin from OUT\\[n\\] task. Event mode: Generate IN\\[n\\] event when rising edge on pin."]
            LO_TO_HI = 0x01,
            #[doc = "Task mode: Clear pin from OUT\\[n\\] task. Event mode: Generate IN\\[n\\] event when falling edge on pin."]
            HI_TO_LO = 0x02,
            #[doc = "Task mode: Toggle pin from OUT\\[n\\]. Event mode: Generate IN\\[n\\] when any change on pin."]
            TOGGLE = 0x03,
        }
        impl Polarity {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Polarity {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Polarity {
            #[inline(always)]
            fn from(val: u8) -> Polarity {
                Polarity::from_bits(val)
            }
        }
        impl From<Polarity> for u8 {
            #[inline(always)]
            fn from(val: Polarity) -> u8 {
                Polarity::to_bits(val)
            }
        }
    }
}
pub mod ipc {
    #[doc = "Interprocessor communication"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipc {
        ptr: *mut u8,
    }
    unsafe impl Send for Ipc {}
    unsafe impl Sync for Ipc {}
    impl Ipc {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description collection: Trigger events on IPC channel enabled in SEND_CNF\\[n\\]"]
        #[inline(always)]
        pub const fn tasks_send(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
            assert!(n < 16usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _)
            }
        }
        #[doc = "Description collection: Subscribe configuration for task SEND\\[n\\]"]
        #[inline(always)]
        pub const fn subscribe_send(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            assert!(n < 16usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _)
            }
        }
        #[doc = "Description collection: Event received on one or more of the enabled IPC channels in RECEIVE_CNF\\[n\\]"]
        #[inline(always)]
        pub const fn events_receive(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 16usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
            }
        }
        #[doc = "Description collection: Publish configuration for event RECEIVE\\[n\\]"]
        #[inline(always)]
        pub const fn publish_receive(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            assert!(n < 16usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize + n * 4usize) as _)
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
        }
        #[doc = "Pending interrupts"]
        #[inline(always)]
        pub const fn intpend(self) -> crate::common::Reg<regs::Int, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x030cusize) as _) }
        }
        #[doc = "Description collection: Send event configuration for TASKS_SEND\\[n\\]"]
        #[inline(always)]
        pub const fn send_cnf(
            self,
            n: usize,
        ) -> crate::common::Reg<regs::SendCnf, crate::common::RW> {
            assert!(n < 16usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0510usize + n * 4usize) as _)
            }
        }
        #[doc = "Description collection: Receive event configuration for EVENTS_RECEIVE\\[n\\]"]
        #[inline(always)]
        pub const fn receive_cnf(
            self,
            n: usize,
        ) -> crate::common::Reg<regs::ReceiveCnf, crate::common::RW> {
            assert!(n < 16usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0590usize + n * 4usize) as _)
            }
        }
        #[doc = "Description collection: General purpose memory"]
        #[inline(always)]
        pub const fn gpmem(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 2usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0610usize + n * 4usize) as _)
            }
        }
    }
    pub mod regs {
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Enable or disable interrupt for event RECEIVE\\[0\\]"]
            #[must_use]
            #[inline(always)]
            pub const fn receive0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[0\\]"]
            #[inline(always)]
            pub const fn set_receive0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[1\\]"]
            #[must_use]
            #[inline(always)]
            pub const fn receive1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[1\\]"]
            #[inline(always)]
            pub const fn set_receive1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[2\\]"]
            #[must_use]
            #[inline(always)]
            pub const fn receive2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[2\\]"]
            #[inline(always)]
            pub const fn set_receive2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[3\\]"]
            #[must_use]
            #[inline(always)]
            pub const fn receive3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[3\\]"]
            #[inline(always)]
            pub const fn set_receive3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[4\\]"]
            #[must_use]
            #[inline(always)]
            pub const fn receive4(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[4\\]"]
            #[inline(always)]
            pub const fn set_receive4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[5\\]"]
            #[must_use]
            #[inline(always)]
            pub const fn receive5(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[5\\]"]
            #[inline(always)]
            pub const fn set_receive5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[6\\]"]
            #[must_use]
            #[inline(always)]
            pub const fn receive6(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[6\\]"]
            #[inline(always)]
            pub const fn set_receive6(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[7\\]"]
            #[must_use]
            #[inline(always)]
            pub const fn receive7(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[7\\]"]
            #[inline(always)]
            pub const fn set_receive7(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[8\\]"]
            #[must_use]
            #[inline(always)]
            pub const fn receive8(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[8\\]"]
            #[inline(always)]
            pub const fn set_receive8(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[9\\]"]
            #[must_use]
            #[inline(always)]
            pub const fn receive9(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[9\\]"]
            #[inline(always)]
            pub const fn set_receive9(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[10\\]"]
            #[must_use]
            #[inline(always)]
            pub const fn receive10(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[10\\]"]
            #[inline(always)]
            pub const fn set_receive10(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[11\\]"]
            #[must_use]
            #[inline(always)]
            pub const fn receive11(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[11\\]"]
            #[inline(always)]
            pub const fn set_receive11(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[12\\]"]
            #[must_use]
            #[inline(always)]
            pub const fn receive12(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[12\\]"]
            #[inline(always)]
            pub const fn set_receive12(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[13\\]"]
            #[must_use]
            #[inline(always)]
            pub const fn receive13(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[13\\]"]
            #[inline(always)]
            pub const fn set_receive13(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[14\\]"]
            #[must_use]
            #[inline(always)]
            pub const fn receive14(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[14\\]"]
            #[inline(always)]
            pub const fn set_receive14(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[15\\]"]
            #[must_use]
            #[inline(always)]
            pub const fn receive15(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[15\\]"]
            #[inline(always)]
            pub const fn set_receive15(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("receive0", &self.receive0())
                    .field("receive1", &self.receive1())
                    .field("receive2", &self.receive2())
                    .field("receive3", &self.receive3())
                    .field("receive4", &self.receive4())
                    .field("receive5", &self.receive5())
                    .field("receive6", &self.receive6())
                    .field("receive7", &self.receive7())
                    .field("receive8", &self.receive8())
                    .field("receive9", &self.receive9())
                    .field("receive10", &self.receive10())
                    .field("receive11", &self.receive11())
                    .field("receive12", &self.receive12())
                    .field("receive13", &self.receive13())
                    .field("receive14", &self.receive14())
                    .field("receive15", &self.receive15())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Int {{ receive0: {=bool:?}, receive1: {=bool:?}, receive2: {=bool:?}, receive3: {=bool:?}, receive4: {=bool:?}, receive5: {=bool:?}, receive6: {=bool:?}, receive7: {=bool:?}, receive8: {=bool:?}, receive9: {=bool:?}, receive10: {=bool:?}, receive11: {=bool:?}, receive12: {=bool:?}, receive13: {=bool:?}, receive14: {=bool:?}, receive15: {=bool:?} }}" , self . receive0 () , self . receive1 () , self . receive2 () , self . receive3 () , self . receive4 () , self . receive5 () , self . receive6 () , self . receive7 () , self . receive8 () , self . receive9 () , self . receive10 () , self . receive11 () , self . receive12 () , self . receive13 () , self . receive14 () , self . receive15 ())
            }
        }
        #[doc = "Description collection: Receive event configuration for EVENTS_RECEIVE\\[n\\]"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct ReceiveCnf(pub u32);
        impl ReceiveCnf {
            #[doc = "Enable subscription to IPC channel 0"]
            #[must_use]
            #[inline(always)]
            pub const fn chen0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable subscription to IPC channel 0"]
            #[inline(always)]
            pub const fn set_chen0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable subscription to IPC channel 1"]
            #[must_use]
            #[inline(always)]
            pub const fn chen1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable subscription to IPC channel 1"]
            #[inline(always)]
            pub const fn set_chen1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable subscription to IPC channel 2"]
            #[must_use]
            #[inline(always)]
            pub const fn chen2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable subscription to IPC channel 2"]
            #[inline(always)]
            pub const fn set_chen2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable subscription to IPC channel 3"]
            #[must_use]
            #[inline(always)]
            pub const fn chen3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable subscription to IPC channel 3"]
            #[inline(always)]
            pub const fn set_chen3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable subscription to IPC channel 4"]
            #[must_use]
            #[inline(always)]
            pub const fn chen4(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable subscription to IPC channel 4"]
            #[inline(always)]
            pub const fn set_chen4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable subscription to IPC channel 5"]
            #[must_use]
            #[inline(always)]
            pub const fn chen5(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable subscription to IPC channel 5"]
            #[inline(always)]
            pub const fn set_chen5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable subscription to IPC channel 6"]
            #[must_use]
            #[inline(always)]
            pub const fn chen6(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable subscription to IPC channel 6"]
            #[inline(always)]
            pub const fn set_chen6(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Enable subscription to IPC channel 7"]
            #[must_use]
            #[inline(always)]
            pub const fn chen7(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable subscription to IPC channel 7"]
            #[inline(always)]
            pub const fn set_chen7(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Enable subscription to IPC channel 8"]
            #[must_use]
            #[inline(always)]
            pub const fn chen8(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Enable subscription to IPC channel 8"]
            #[inline(always)]
            pub const fn set_chen8(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Enable subscription to IPC channel 9"]
            #[must_use]
            #[inline(always)]
            pub const fn chen9(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Enable subscription to IPC channel 9"]
            #[inline(always)]
            pub const fn set_chen9(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Enable subscription to IPC channel 10"]
            #[must_use]
            #[inline(always)]
            pub const fn chen10(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Enable subscription to IPC channel 10"]
            #[inline(always)]
            pub const fn set_chen10(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Enable subscription to IPC channel 11"]
            #[must_use]
            #[inline(always)]
            pub const fn chen11(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "Enable subscription to IPC channel 11"]
            #[inline(always)]
            pub const fn set_chen11(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "Enable subscription to IPC channel 12"]
            #[must_use]
            #[inline(always)]
            pub const fn chen12(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Enable subscription to IPC channel 12"]
            #[inline(always)]
            pub const fn set_chen12(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "Enable subscription to IPC channel 13"]
            #[must_use]
            #[inline(always)]
            pub const fn chen13(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Enable subscription to IPC channel 13"]
            #[inline(always)]
            pub const fn set_chen13(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Enable subscription to IPC channel 14"]
            #[must_use]
            #[inline(always)]
            pub const fn chen14(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Enable subscription to IPC channel 14"]
            #[inline(always)]
            pub const fn set_chen14(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Enable subscription to IPC channel 15"]
            #[must_use]
            #[inline(always)]
            pub const fn chen15(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "Enable subscription to IPC channel 15"]
            #[inline(always)]
            pub const fn set_chen15(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
        }
        impl Default for ReceiveCnf {
            #[inline(always)]
            fn default() -> ReceiveCnf {
                ReceiveCnf(0)
            }
        }
        impl core::fmt::Debug for ReceiveCnf {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("ReceiveCnf")
                    .field("chen0", &self.chen0())
                    .field("chen1", &self.chen1())
                    .field("chen2", &self.chen2())
                    .field("chen3", &self.chen3())
                    .field("chen4", &self.chen4())
                    .field("chen5", &self.chen5())
                    .field("chen6", &self.chen6())
                    .field("chen7", &self.chen7())
                    .field("chen8", &self.chen8())
                    .field("chen9", &self.chen9())
                    .field("chen10", &self.chen10())
                    .field("chen11", &self.chen11())
                    .field("chen12", &self.chen12())
                    .field("chen13", &self.chen13())
                    .field("chen14", &self.chen14())
                    .field("chen15", &self.chen15())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for ReceiveCnf {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "ReceiveCnf {{ chen0: {=bool:?}, chen1: {=bool:?}, chen2: {=bool:?}, chen3: {=bool:?}, chen4: {=bool:?}, chen5: {=bool:?}, chen6: {=bool:?}, chen7: {=bool:?}, chen8: {=bool:?}, chen9: {=bool:?}, chen10: {=bool:?}, chen11: {=bool:?}, chen12: {=bool:?}, chen13: {=bool:?}, chen14: {=bool:?}, chen15: {=bool:?} }}" , self . chen0 () , self . chen1 () , self . chen2 () , self . chen3 () , self . chen4 () , self . chen5 () , self . chen6 () , self . chen7 () , self . chen8 () , self . chen9 () , self . chen10 () , self . chen11 () , self . chen12 () , self . chen13 () , self . chen14 () , self . chen15 ())
            }
        }
        #[doc = "Description collection: Send event configuration for TASKS_SEND\\[n\\]"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SendCnf(pub u32);
        impl SendCnf {
            #[doc = "Enable broadcasting on IPC channel 0"]
            #[must_use]
            #[inline(always)]
            pub const fn chen0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable broadcasting on IPC channel 0"]
            #[inline(always)]
            pub const fn set_chen0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable broadcasting on IPC channel 1"]
            #[must_use]
            #[inline(always)]
            pub const fn chen1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable broadcasting on IPC channel 1"]
            #[inline(always)]
            pub const fn set_chen1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable broadcasting on IPC channel 2"]
            #[must_use]
            #[inline(always)]
            pub const fn chen2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable broadcasting on IPC channel 2"]
            #[inline(always)]
            pub const fn set_chen2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable broadcasting on IPC channel 3"]
            #[must_use]
            #[inline(always)]
            pub const fn chen3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable broadcasting on IPC channel 3"]
            #[inline(always)]
            pub const fn set_chen3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable broadcasting on IPC channel 4"]
            #[must_use]
            #[inline(always)]
            pub const fn chen4(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable broadcasting on IPC channel 4"]
            #[inline(always)]
            pub const fn set_chen4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable broadcasting on IPC channel 5"]
            #[must_use]
            #[inline(always)]
            pub const fn chen5(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable broadcasting on IPC channel 5"]
            #[inline(always)]
            pub const fn set_chen5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable broadcasting on IPC channel 6"]
            #[must_use]
            #[inline(always)]
            pub const fn chen6(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable broadcasting on IPC channel 6"]
            #[inline(always)]
            pub const fn set_chen6(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Enable broadcasting on IPC channel 7"]
            #[must_use]
            #[inline(always)]
            pub const fn chen7(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable broadcasting on IPC channel 7"]
            #[inline(always)]
            pub const fn set_chen7(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Enable broadcasting on IPC channel 8"]
            #[must_use]
            #[inline(always)]
            pub const fn chen8(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Enable broadcasting on IPC channel 8"]
            #[inline(always)]
            pub const fn set_chen8(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Enable broadcasting on IPC channel 9"]
            #[must_use]
            #[inline(always)]
            pub const fn chen9(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Enable broadcasting on IPC channel 9"]
            #[inline(always)]
            pub const fn set_chen9(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Enable broadcasting on IPC channel 10"]
            #[must_use]
            #[inline(always)]
            pub const fn chen10(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Enable broadcasting on IPC channel 10"]
            #[inline(always)]
            pub const fn set_chen10(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Enable broadcasting on IPC channel 11"]
            #[must_use]
            #[inline(always)]
            pub const fn chen11(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "Enable broadcasting on IPC channel 11"]
            #[inline(always)]
            pub const fn set_chen11(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "Enable broadcasting on IPC channel 12"]
            #[must_use]
            #[inline(always)]
            pub const fn chen12(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Enable broadcasting on IPC channel 12"]
            #[inline(always)]
            pub const fn set_chen12(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "Enable broadcasting on IPC channel 13"]
            #[must_use]
            #[inline(always)]
            pub const fn chen13(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Enable broadcasting on IPC channel 13"]
            #[inline(always)]
            pub const fn set_chen13(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Enable broadcasting on IPC channel 14"]
            #[must_use]
            #[inline(always)]
            pub const fn chen14(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Enable broadcasting on IPC channel 14"]
            #[inline(always)]
            pub const fn set_chen14(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Enable broadcasting on IPC channel 15"]
            #[must_use]
            #[inline(always)]
            pub const fn chen15(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "Enable broadcasting on IPC channel 15"]
            #[inline(always)]
            pub const fn set_chen15(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
        }
        impl Default for SendCnf {
            #[inline(always)]
            fn default() -> SendCnf {
                SendCnf(0)
            }
        }
        impl core::fmt::Debug for SendCnf {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SendCnf")
                    .field("chen0", &self.chen0())
                    .field("chen1", &self.chen1())
                    .field("chen2", &self.chen2())
                    .field("chen3", &self.chen3())
                    .field("chen4", &self.chen4())
                    .field("chen5", &self.chen5())
                    .field("chen6", &self.chen6())
                    .field("chen7", &self.chen7())
                    .field("chen8", &self.chen8())
                    .field("chen9", &self.chen9())
                    .field("chen10", &self.chen10())
                    .field("chen11", &self.chen11())
                    .field("chen12", &self.chen12())
                    .field("chen13", &self.chen13())
                    .field("chen14", &self.chen14())
                    .field("chen15", &self.chen15())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SendCnf {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "SendCnf {{ chen0: {=bool:?}, chen1: {=bool:?}, chen2: {=bool:?}, chen3: {=bool:?}, chen4: {=bool:?}, chen5: {=bool:?}, chen6: {=bool:?}, chen7: {=bool:?}, chen8: {=bool:?}, chen9: {=bool:?}, chen10: {=bool:?}, chen11: {=bool:?}, chen12: {=bool:?}, chen13: {=bool:?}, chen14: {=bool:?}, chen15: {=bool:?} }}" , self . chen0 () , self . chen1 () , self . chen2 () , self . chen3 () , self . chen4 () , self . chen5 () , self . chen6 () , self . chen7 () , self . chen8 () , self . chen9 () , self . chen10 () , self . chen11 () , self . chen12 () , self . chen13 () , self . chen14 () , self . chen15 ())
            }
        }
    }
}
pub mod mutex {
    #[doc = "MUTEX 0"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mutex {
        ptr: *mut u8,
    }
    unsafe impl Send for Mutex {}
    unsafe impl Sync for Mutex {}
    impl Mutex {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description collection: Mutex register"]
        #[inline(always)]
        pub const fn mutex(self, n: usize) -> crate::common::Reg<regs::Mutex, crate::common::RW> {
            assert!(n < 16usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 4usize) as _)
            }
        }
    }
    pub mod regs {
        #[doc = "Description collection: Mutex register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Mutex(pub u32);
        impl Mutex {
            #[doc = "Mutex register n"]
            #[must_use]
            #[inline(always)]
            pub const fn mutex(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Mutex register n"]
            #[inline(always)]
            pub const fn set_mutex(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Mutex {
            #[inline(always)]
            fn default() -> Mutex {
                Mutex(0)
            }
        }
        impl core::fmt::Debug for Mutex {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Mutex")
                    .field("mutex", &self.mutex())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Mutex {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Mutex {{ mutex: {=bool:?} }}", self.mutex())
            }
        }
    }
}
pub mod nvmc {
    #[doc = "Non-volatile memory controller"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nvmc {
        ptr: *mut u8,
    }
    unsafe impl Send for Nvmc {}
    unsafe impl Sync for Nvmc {}
    impl Nvmc {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Ready flag"]
        #[inline(always)]
        pub const fn ready(self) -> crate::common::Reg<regs::Ready, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
        }
        #[doc = "Ready flag"]
        #[inline(always)]
        pub const fn readynext(self) -> crate::common::Reg<regs::Readynext, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0408usize) as _) }
        }
        #[doc = "Configuration register"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
        }
        #[doc = "Register for erasing all non-volatile user memory"]
        #[inline(always)]
        pub const fn eraseall(self) -> crate::common::Reg<regs::Eraseall, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x050cusize) as _) }
        }
        #[doc = "Register for partial erase configuration"]
        #[inline(always)]
        pub const fn erasepagepartialcfg(
            self,
        ) -> crate::common::Reg<regs::Erasepagepartialcfg, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x051cusize) as _) }
        }
        #[doc = "I-code cache configuration register"]
        #[inline(always)]
        pub const fn icachecnf(self) -> crate::common::Reg<regs::Icachecnf, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0540usize) as _) }
        }
        #[doc = "I-code cache hit counter"]
        #[inline(always)]
        pub const fn ihit(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0548usize) as _) }
        }
        #[doc = "I-code cache miss counter"]
        #[inline(always)]
        pub const fn imiss(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x054cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated."]
            #[must_use]
            #[inline(always)]
            pub const fn wen(&self) -> super::vals::Wen {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::Wen::from_bits(val as u8)
            }
            #[doc = "Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated."]
            #[inline(always)]
            pub const fn set_wen(&mut self, val: super::vals::Wen) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        impl core::fmt::Debug for Config {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Config").field("wen", &self.wen()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Config {{ wen: {:?} }}", self.wen())
            }
        }
        #[doc = "Register for erasing all non-volatile user memory"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Eraseall(pub u32);
        impl Eraseall {
            #[doc = "Erase all non-volatile memory including UICR registers. Before the non-volatile memory can be erased, erasing must be enabled by setting CONFIG.WEN=Een."]
            #[must_use]
            #[inline(always)]
            pub const fn eraseall(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Erase all non-volatile memory including UICR registers. Before the non-volatile memory can be erased, erasing must be enabled by setting CONFIG.WEN=Een."]
            #[inline(always)]
            pub const fn set_eraseall(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Eraseall {
            #[inline(always)]
            fn default() -> Eraseall {
                Eraseall(0)
            }
        }
        impl core::fmt::Debug for Eraseall {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Eraseall")
                    .field("eraseall", &self.eraseall())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Eraseall {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Eraseall {{ eraseall: {=bool:?} }}", self.eraseall())
            }
        }
        #[doc = "Register for partial erase configuration"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Erasepagepartialcfg(pub u32);
        impl Erasepagepartialcfg {
            #[doc = "Duration of the partial erase in milliseconds"]
            #[must_use]
            #[inline(always)]
            pub const fn duration(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "Duration of the partial erase in milliseconds"]
            #[inline(always)]
            pub const fn set_duration(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
            }
        }
        impl Default for Erasepagepartialcfg {
            #[inline(always)]
            fn default() -> Erasepagepartialcfg {
                Erasepagepartialcfg(0)
            }
        }
        impl core::fmt::Debug for Erasepagepartialcfg {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Erasepagepartialcfg")
                    .field("duration", &self.duration())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Erasepagepartialcfg {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Erasepagepartialcfg {{ duration: {=u8:?} }}",
                    self.duration()
                )
            }
        }
        #[doc = "I-code cache configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Icachecnf(pub u32);
        impl Icachecnf {
            #[doc = "Cache enable"]
            #[must_use]
            #[inline(always)]
            pub const fn cacheen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Cache enable"]
            #[inline(always)]
            pub const fn set_cacheen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Cache profiling enable"]
            #[must_use]
            #[inline(always)]
            pub const fn cacheprofen(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Cache profiling enable"]
            #[inline(always)]
            pub const fn set_cacheprofen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for Icachecnf {
            #[inline(always)]
            fn default() -> Icachecnf {
                Icachecnf(0)
            }
        }
        impl core::fmt::Debug for Icachecnf {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Icachecnf")
                    .field("cacheen", &self.cacheen())
                    .field("cacheprofen", &self.cacheprofen())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Icachecnf {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Icachecnf {{ cacheen: {=bool:?}, cacheprofen: {=bool:?} }}",
                    self.cacheen(),
                    self.cacheprofen()
                )
            }
        }
        #[doc = "Ready flag"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ready(pub u32);
        impl Ready {
            #[doc = "NVMC is ready or busy"]
            #[must_use]
            #[inline(always)]
            pub const fn ready(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "NVMC is ready or busy"]
            #[inline(always)]
            pub const fn set_ready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Ready {
            #[inline(always)]
            fn default() -> Ready {
                Ready(0)
            }
        }
        impl core::fmt::Debug for Ready {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ready")
                    .field("ready", &self.ready())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ready {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Ready {{ ready: {=bool:?} }}", self.ready())
            }
        }
        #[doc = "Ready flag"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Readynext(pub u32);
        impl Readynext {
            #[doc = "NVMC can accept a new write operation"]
            #[must_use]
            #[inline(always)]
            pub const fn readynext(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "NVMC can accept a new write operation"]
            #[inline(always)]
            pub const fn set_readynext(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Readynext {
            #[inline(always)]
            fn default() -> Readynext {
                Readynext(0)
            }
        }
        impl core::fmt::Debug for Readynext {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Readynext")
                    .field("readynext", &self.readynext())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Readynext {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Readynext {{ readynext: {=bool:?} }}", self.readynext())
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Wen {
            #[doc = "Read only access"]
            REN = 0x0,
            #[doc = "Write enabled"]
            WEN = 0x01,
            #[doc = "Erase enabled"]
            EEN = 0x02,
            _RESERVED_3 = 0x03,
            #[doc = "Partial erase enabled"]
            PEEN = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl Wen {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Wen {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Wen {
            #[inline(always)]
            fn from(val: u8) -> Wen {
                Wen::from_bits(val)
            }
        }
        impl From<Wen> for u8 {
            #[inline(always)]
            fn from(val: Wen) -> u8 {
                Wen::to_bits(val)
            }
        }
    }
}
pub mod power {
    #[doc = "Power control"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Power {
        ptr: *mut u8,
    }
    unsafe impl Send for Power {}
    unsafe impl Sync for Power {}
    impl Power {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Enable Constant Latency mode"]
        #[inline(always)]
        pub const fn tasks_constlat(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
        }
        #[doc = "Enable Low-Power mode (variable latency)"]
        #[inline(always)]
        pub const fn tasks_lowpwr(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
        }
        #[doc = "Subscribe configuration for task CONSTLAT"]
        #[inline(always)]
        pub const fn subscribe_constlat(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
        }
        #[doc = "Subscribe configuration for task LOWPWR"]
        #[inline(always)]
        pub const fn subscribe_lowpwr(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
        }
        #[doc = "Power failure warning"]
        #[inline(always)]
        pub const fn events_pofwarn(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
        }
        #[doc = "CPU entered WFI/WFE sleep"]
        #[inline(always)]
        pub const fn events_sleepenter(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
        }
        #[doc = "CPU exited WFI/WFE sleep"]
        #[inline(always)]
        pub const fn events_sleepexit(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
        }
        #[doc = "Publish configuration for event POFWARN"]
        #[inline(always)]
        pub const fn publish_pofwarn(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
        }
        #[doc = "Publish configuration for event SLEEPENTER"]
        #[inline(always)]
        pub const fn publish_sleepenter(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
        }
        #[doc = "Publish configuration for event SLEEPEXIT"]
        #[inline(always)]
        pub const fn publish_sleepexit(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
        }
        #[doc = "Description collection: General purpose retention register"]
        #[inline(always)]
        pub const fn gpregret(
            self,
            n: usize,
        ) -> crate::common::Reg<regs::Gpregret, crate::common::RW> {
            assert!(n < 2usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x051cusize + n * 4usize) as _)
            }
        }
    }
    pub mod regs {
        #[doc = "Description collection: General purpose retention register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Gpregret(pub u32);
        impl Gpregret {
            #[doc = "General purpose retention register"]
            #[must_use]
            #[inline(always)]
            pub const fn gpregret(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "General purpose retention register"]
            #[inline(always)]
            pub const fn set_gpregret(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Gpregret {
            #[inline(always)]
            fn default() -> Gpregret {
                Gpregret(0)
            }
        }
        impl core::fmt::Debug for Gpregret {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Gpregret")
                    .field("gpregret", &self.gpregret())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Gpregret {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Gpregret {{ gpregret: {=u8:?} }}", self.gpregret())
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Enable or disable interrupt for event POFWARN"]
            #[must_use]
            #[inline(always)]
            pub const fn pofwarn(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event POFWARN"]
            #[inline(always)]
            pub const fn set_pofwarn(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable or disable interrupt for event SLEEPENTER"]
            #[must_use]
            #[inline(always)]
            pub const fn sleepenter(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event SLEEPENTER"]
            #[inline(always)]
            pub const fn set_sleepenter(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable or disable interrupt for event SLEEPEXIT"]
            #[must_use]
            #[inline(always)]
            pub const fn sleepexit(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event SLEEPEXIT"]
            #[inline(always)]
            pub const fn set_sleepexit(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("pofwarn", &self.pofwarn())
                    .field("sleepenter", &self.sleepenter())
                    .field("sleepexit", &self.sleepexit())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Int {{ pofwarn: {=bool:?}, sleepenter: {=bool:?}, sleepexit: {=bool:?} }}",
                    self.pofwarn(),
                    self.sleepenter(),
                    self.sleepexit()
                )
            }
        }
    }
}
pub mod radio {
    #[doc = "DFE packet EasyDMA channel"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dfepacket {
        ptr: *mut u8,
    }
    unsafe impl Send for Dfepacket {}
    unsafe impl Sync for Dfepacket {}
    impl Dfepacket {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Data pointer"]
        #[inline(always)]
        pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Maximum number of buffer words to transfer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::Maxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Number of samples transferred in the last transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::Amount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psel {
        ptr: *mut u8,
    }
    unsafe impl Send for Psel {}
    unsafe impl Sync for Psel {}
    impl Psel {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description collection: Pin select for DFE pin n"]
        #[inline(always)]
        pub const fn dfegpio(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            assert!(n < 8usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _)
            }
        }
    }
    #[doc = "2.4 GHz radio"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Radio {
        ptr: *mut u8,
    }
    unsafe impl Send for Radio {}
    unsafe impl Sync for Radio {}
    impl Radio {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Enable RADIO in TX mode"]
        #[inline(always)]
        pub const fn tasks_txen(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Enable RADIO in RX mode"]
        #[inline(always)]
        pub const fn tasks_rxen(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Start RADIO"]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "Stop RADIO"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
        #[doc = "Disable RADIO"]
        #[inline(always)]
        pub const fn tasks_disable(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
        }
        #[doc = "Start the RSSI and take one single sample of the receive signal strength"]
        #[inline(always)]
        pub const fn tasks_rssistart(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
        }
        #[doc = "Stop the RSSI measurement"]
        #[inline(always)]
        pub const fn tasks_rssistop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
        }
        #[doc = "Start the bit counter"]
        #[inline(always)]
        pub const fn tasks_bcstart(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
        }
        #[doc = "Stop the bit counter"]
        #[inline(always)]
        pub const fn tasks_bcstop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
        }
        #[doc = "Start the energy detect measurement used in IEEE 802.15.4 mode"]
        #[inline(always)]
        pub const fn tasks_edstart(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
        }
        #[doc = "Stop the energy detect measurement"]
        #[inline(always)]
        pub const fn tasks_edstop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
        }
        #[doc = "Start the clear channel assessment used in IEEE 802.15.4 mode"]
        #[inline(always)]
        pub const fn tasks_ccastart(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
        }
        #[doc = "Stop the clear channel assessment"]
        #[inline(always)]
        pub const fn tasks_ccastop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
        }
        #[doc = "Subscribe configuration for task TXEN"]
        #[inline(always)]
        pub const fn subscribe_txen(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
        }
        #[doc = "Subscribe configuration for task RXEN"]
        #[inline(always)]
        pub const fn subscribe_rxen(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
        }
        #[doc = "Subscribe configuration for task START"]
        #[inline(always)]
        pub const fn subscribe_start(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
        }
        #[doc = "Subscribe configuration for task STOP"]
        #[inline(always)]
        pub const fn subscribe_stop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
        }
        #[doc = "Subscribe configuration for task DISABLE"]
        #[inline(always)]
        pub const fn subscribe_disable(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
        }
        #[doc = "Subscribe configuration for task RSSISTART"]
        #[inline(always)]
        pub const fn subscribe_rssistart(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
        }
        #[doc = "Subscribe configuration for task RSSISTOP"]
        #[inline(always)]
        pub const fn subscribe_rssistop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
        }
        #[doc = "Subscribe configuration for task BCSTART"]
        #[inline(always)]
        pub const fn subscribe_bcstart(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
        }
        #[doc = "Subscribe configuration for task BCSTOP"]
        #[inline(always)]
        pub const fn subscribe_bcstop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
        }
        #[doc = "Subscribe configuration for task EDSTART"]
        #[inline(always)]
        pub const fn subscribe_edstart(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
        }
        #[doc = "Subscribe configuration for task EDSTOP"]
        #[inline(always)]
        pub const fn subscribe_edstop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
        }
        #[doc = "Subscribe configuration for task CCASTART"]
        #[inline(always)]
        pub const fn subscribe_ccastart(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
        }
        #[doc = "Subscribe configuration for task CCASTOP"]
        #[inline(always)]
        pub const fn subscribe_ccastop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
        }
        #[doc = "RADIO has ramped up and is ready to be started"]
        #[inline(always)]
        pub const fn events_ready(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
        }
        #[doc = "Address sent or received"]
        #[inline(always)]
        pub const fn events_address(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
        }
        #[doc = "Packet payload sent or received"]
        #[inline(always)]
        pub const fn events_payload(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
        }
        #[doc = "Packet sent or received"]
        #[inline(always)]
        pub const fn events_end(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
        }
        #[doc = "RADIO has been disabled"]
        #[inline(always)]
        pub const fn events_disabled(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
        }
        #[doc = "A device address match occurred on the last received packet"]
        #[inline(always)]
        pub const fn events_devmatch(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
        }
        #[doc = "No device address match occurred on the last received packet"]
        #[inline(always)]
        pub const fn events_devmiss(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
        }
        #[doc = "Sampling of receive signal strength complete"]
        #[inline(always)]
        pub const fn events_rssiend(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
        }
        #[doc = "Bit counter reached bit count value"]
        #[inline(always)]
        pub const fn events_bcmatch(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
        }
        #[doc = "Packet received with CRC ok"]
        #[inline(always)]
        pub const fn events_crcok(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
        }
        #[doc = "Packet received with CRC error"]
        #[inline(always)]
        pub const fn events_crcerror(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
        }
        #[doc = "IEEE 802.15.4 length field received"]
        #[inline(always)]
        pub const fn events_framestart(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
        }
        #[doc = "Sampling of energy detection complete. A new ED sample is ready for readout from the RADIO.EDSAMPLE register."]
        #[inline(always)]
        pub const fn events_edend(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
        }
        #[doc = "The sampling of energy detection has stopped"]
        #[inline(always)]
        pub const fn events_edstopped(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
        }
        #[doc = "Wireless medium in idle - clear to send"]
        #[inline(always)]
        pub const fn events_ccaidle(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
        }
        #[doc = "Wireless medium busy - do not send"]
        #[inline(always)]
        pub const fn events_ccabusy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
        }
        #[doc = "The CCA has stopped"]
        #[inline(always)]
        pub const fn events_ccastopped(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
        }
        #[doc = "Ble_LR CI field received, receive mode is changed from Ble_LR125Kbit to Ble_LR500Kbit."]
        #[inline(always)]
        pub const fn events_rateboost(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
        }
        #[doc = "RADIO has ramped up and is ready to be started TX path"]
        #[inline(always)]
        pub const fn events_txready(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
        }
        #[doc = "RADIO has ramped up and is ready to be started RX path"]
        #[inline(always)]
        pub const fn events_rxready(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
        }
        #[doc = "MAC header match found"]
        #[inline(always)]
        pub const fn events_mhrmatch(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
        }
        #[doc = "Preamble indicator"]
        #[inline(always)]
        pub const fn events_sync(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
        }
        #[doc = "Generated when last bit is sent on air, or received from air"]
        #[inline(always)]
        pub const fn events_phyend(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
        }
        #[doc = "CTE is present (early warning right after receiving CTEInfo byte)"]
        #[inline(always)]
        pub const fn events_ctepresent(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
        }
        #[doc = "Publish configuration for event READY"]
        #[inline(always)]
        pub const fn publish_ready(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
        }
        #[doc = "Publish configuration for event ADDRESS"]
        #[inline(always)]
        pub const fn publish_address(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
        }
        #[doc = "Publish configuration for event PAYLOAD"]
        #[inline(always)]
        pub const fn publish_payload(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
        }
        #[doc = "Publish configuration for event END"]
        #[inline(always)]
        pub const fn publish_end(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
        }
        #[doc = "Publish configuration for event DISABLED"]
        #[inline(always)]
        pub const fn publish_disabled(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
        }
        #[doc = "Publish configuration for event DEVMATCH"]
        #[inline(always)]
        pub const fn publish_devmatch(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
        }
        #[doc = "Publish configuration for event DEVMISS"]
        #[inline(always)]
        pub const fn publish_devmiss(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
        }
        #[doc = "Publish configuration for event RSSIEND"]
        #[inline(always)]
        pub const fn publish_rssiend(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
        }
        #[doc = "Publish configuration for event BCMATCH"]
        #[inline(always)]
        pub const fn publish_bcmatch(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
        }
        #[doc = "Publish configuration for event CRCOK"]
        #[inline(always)]
        pub const fn publish_crcok(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
        }
        #[doc = "Publish configuration for event CRCERROR"]
        #[inline(always)]
        pub const fn publish_crcerror(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
        }
        #[doc = "Publish configuration for event FRAMESTART"]
        #[inline(always)]
        pub const fn publish_framestart(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
        }
        #[doc = "Publish configuration for event EDEND"]
        #[inline(always)]
        pub const fn publish_edend(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
        }
        #[doc = "Publish configuration for event EDSTOPPED"]
        #[inline(always)]
        pub const fn publish_edstopped(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
        }
        #[doc = "Publish configuration for event CCAIDLE"]
        #[inline(always)]
        pub const fn publish_ccaidle(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
        }
        #[doc = "Publish configuration for event CCABUSY"]
        #[inline(always)]
        pub const fn publish_ccabusy(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
        }
        #[doc = "Publish configuration for event CCASTOPPED"]
        #[inline(always)]
        pub const fn publish_ccastopped(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
        }
        #[doc = "Publish configuration for event RATEBOOST"]
        #[inline(always)]
        pub const fn publish_rateboost(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
        }
        #[doc = "Publish configuration for event TXREADY"]
        #[inline(always)]
        pub const fn publish_txready(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
        }
        #[doc = "Publish configuration for event RXREADY"]
        #[inline(always)]
        pub const fn publish_rxready(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
        }
        #[doc = "Publish configuration for event MHRMATCH"]
        #[inline(always)]
        pub const fn publish_mhrmatch(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01dcusize) as _) }
        }
        #[doc = "Publish configuration for event SYNC"]
        #[inline(always)]
        pub const fn publish_sync(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
        }
        #[doc = "Publish configuration for event PHYEND"]
        #[inline(always)]
        pub const fn publish_phyend(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
        }
        #[doc = "Publish configuration for event CTEPRESENT"]
        #[inline(always)]
        pub const fn publish_ctepresent(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
        }
        #[doc = "CRC status"]
        #[inline(always)]
        pub const fn crcstatus(self) -> crate::common::Reg<regs::Crcstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
        }
        #[doc = "Received address"]
        #[inline(always)]
        pub const fn rxmatch(self) -> crate::common::Reg<regs::Rxmatch, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0408usize) as _) }
        }
        #[doc = "CRC field of previously received packet"]
        #[inline(always)]
        pub const fn rxcrc(self) -> crate::common::Reg<regs::Rxcrc, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x040cusize) as _) }
        }
        #[doc = "Device address match index"]
        #[inline(always)]
        pub const fn dai(self) -> crate::common::Reg<regs::Dai, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
        }
        #[doc = "Payload status"]
        #[inline(always)]
        pub const fn pdustat(self) -> crate::common::Reg<regs::Pdustat, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0414usize) as _) }
        }
        #[doc = "CTEInfo parsed from received packet"]
        #[inline(always)]
        pub const fn ctestatus(self) -> crate::common::Reg<regs::Ctestatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x044cusize) as _) }
        }
        #[doc = "DFE status information"]
        #[inline(always)]
        pub const fn dfestatus(self) -> crate::common::Reg<regs::Dfestatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0458usize) as _) }
        }
        #[doc = "Packet pointer"]
        #[inline(always)]
        pub const fn packetptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
        }
        #[doc = "Frequency"]
        #[inline(always)]
        pub const fn frequency(self) -> crate::common::Reg<regs::Frequency, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
        }
        #[doc = "Output power"]
        #[inline(always)]
        pub const fn txpower(self) -> crate::common::Reg<regs::Txpower, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x050cusize) as _) }
        }
        #[doc = "Data rate and modulation"]
        #[inline(always)]
        pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0510usize) as _) }
        }
        #[doc = "Packet configuration register 0"]
        #[inline(always)]
        pub const fn pcnf0(self) -> crate::common::Reg<regs::Pcnf0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0514usize) as _) }
        }
        #[doc = "Packet configuration register 1"]
        #[inline(always)]
        pub const fn pcnf1(self) -> crate::common::Reg<regs::Pcnf1, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0518usize) as _) }
        }
        #[doc = "Base address 0"]
        #[inline(always)]
        pub const fn base0(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x051cusize) as _) }
        }
        #[doc = "Base address 1"]
        #[inline(always)]
        pub const fn base1(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0520usize) as _) }
        }
        #[doc = "Prefixes bytes for logical addresses 0-3"]
        #[inline(always)]
        pub const fn prefix0(self) -> crate::common::Reg<regs::Prefix0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0524usize) as _) }
        }
        #[doc = "Prefixes bytes for logical addresses 4-7"]
        #[inline(always)]
        pub const fn prefix1(self) -> crate::common::Reg<regs::Prefix1, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0528usize) as _) }
        }
        #[doc = "Transmit address select"]
        #[inline(always)]
        pub const fn txaddress(self) -> crate::common::Reg<regs::Txaddress, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x052cusize) as _) }
        }
        #[doc = "Receive address select"]
        #[inline(always)]
        pub const fn rxaddresses(self) -> crate::common::Reg<regs::Rxaddresses, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0530usize) as _) }
        }
        #[doc = "CRC configuration"]
        #[inline(always)]
        pub const fn crccnf(self) -> crate::common::Reg<regs::Crccnf, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0534usize) as _) }
        }
        #[doc = "CRC polynomial"]
        #[inline(always)]
        pub const fn crcpoly(self) -> crate::common::Reg<regs::Crcpoly, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0538usize) as _) }
        }
        #[doc = "CRC initial value"]
        #[inline(always)]
        pub const fn crcinit(self) -> crate::common::Reg<regs::Crcinit, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x053cusize) as _) }
        }
        #[doc = "Interframe spacing in us"]
        #[inline(always)]
        pub const fn tifs(self) -> crate::common::Reg<regs::Tifs, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0544usize) as _) }
        }
        #[doc = "RSSI sample"]
        #[inline(always)]
        pub const fn rssisample(self) -> crate::common::Reg<regs::Rssisample, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0548usize) as _) }
        }
        #[doc = "Current radio state"]
        #[inline(always)]
        pub const fn state(self) -> crate::common::Reg<regs::State, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0550usize) as _) }
        }
        #[doc = "Data whitening initial value"]
        #[inline(always)]
        pub const fn datawhiteiv(self) -> crate::common::Reg<regs::Datawhiteiv, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0554usize) as _) }
        }
        #[doc = "Bit counter compare"]
        #[inline(always)]
        pub const fn bcc(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0560usize) as _) }
        }
        #[doc = "Description collection: Device address base segment n"]
        #[inline(always)]
        pub const fn dab(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 8usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize + n * 4usize) as _)
            }
        }
        #[doc = "Description collection: Device address prefix n"]
        #[inline(always)]
        pub const fn dap(self, n: usize) -> crate::common::Reg<regs::Dap, crate::common::RW> {
            assert!(n < 8usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0620usize + n * 4usize) as _)
            }
        }
        #[doc = "Device address match configuration"]
        #[inline(always)]
        pub const fn dacnf(self) -> crate::common::Reg<regs::Dacnf, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0640usize) as _) }
        }
        #[doc = "Search pattern configuration"]
        #[inline(always)]
        pub const fn mhrmatchconf(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0644usize) as _) }
        }
        #[doc = "Pattern mask"]
        #[inline(always)]
        pub const fn mhrmatchmas(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0648usize) as _) }
        }
        #[doc = "Radio mode configuration register 0"]
        #[inline(always)]
        pub const fn modecnf0(self) -> crate::common::Reg<regs::Modecnf0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0650usize) as _) }
        }
        #[doc = "IEEE 802.15.4 start of frame delimiter"]
        #[inline(always)]
        pub const fn sfd(self) -> crate::common::Reg<regs::Sfd, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0660usize) as _) }
        }
        #[doc = "IEEE 802.15.4 energy detect loop count"]
        #[inline(always)]
        pub const fn edcnt(self) -> crate::common::Reg<regs::Edcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0664usize) as _) }
        }
        #[doc = "IEEE 802.15.4 energy detect level"]
        #[inline(always)]
        pub const fn edsample(self) -> crate::common::Reg<regs::Edsample, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0668usize) as _) }
        }
        #[doc = "IEEE 802.15.4 clear channel assessment control"]
        #[inline(always)]
        pub const fn ccactrl(self) -> crate::common::Reg<regs::Ccactrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x066cusize) as _) }
        }
        #[doc = "Whether to use Angle-of-Arrival (AOA) or Angle-of-Departure (AOD)"]
        #[inline(always)]
        pub const fn dfemode(self) -> crate::common::Reg<regs::Dfemode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0900usize) as _) }
        }
        #[doc = "Configuration for CTE inline mode"]
        #[inline(always)]
        pub const fn cteinlineconf(
            self,
        ) -> crate::common::Reg<regs::Cteinlineconf, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0904usize) as _) }
        }
        #[doc = "Various configuration for Direction finding"]
        #[inline(always)]
        pub const fn dfectrl1(self) -> crate::common::Reg<regs::Dfectrl1, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0910usize) as _) }
        }
        #[doc = "Start offset for Direction finding"]
        #[inline(always)]
        pub const fn dfectrl2(self) -> crate::common::Reg<regs::Dfectrl2, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0914usize) as _) }
        }
        #[doc = "GPIO patterns to be used for each antenna"]
        #[inline(always)]
        pub const fn switchpattern(
            self,
        ) -> crate::common::Reg<regs::Switchpattern, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0928usize) as _) }
        }
        #[doc = "Clear the GPIO pattern array for antenna control"]
        #[inline(always)]
        pub const fn clearpattern(
            self,
        ) -> crate::common::Reg<regs::Clearpattern, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x092cusize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn psel(self) -> Psel {
            unsafe { Psel::from_ptr(self.ptr.wrapping_add(0x0930usize) as _) }
        }
        #[doc = "DFE packet EasyDMA channel"]
        #[inline(always)]
        pub const fn dfepacket(self) -> Dfepacket {
            unsafe { Dfepacket::from_ptr(self.ptr.wrapping_add(0x0950usize) as _) }
        }
        #[doc = "Peripheral power control"]
        #[inline(always)]
        pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Number of samples transferred in the last transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Amount(pub u32);
        impl Amount {
            #[doc = "Number of samples transferred in the last transaction"]
            #[must_use]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Number of samples transferred in the last transaction"]
            #[inline(always)]
            pub const fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Amount {
            #[inline(always)]
            fn default() -> Amount {
                Amount(0)
            }
        }
        impl core::fmt::Debug for Amount {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Amount")
                    .field("amount", &self.amount())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Amount {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Amount {{ amount: {=u16:?} }}", self.amount())
            }
        }
        #[doc = "IEEE 802.15.4 clear channel assessment control"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ccactrl(pub u32);
        impl Ccactrl {
            #[doc = "CCA mode of operation"]
            #[must_use]
            #[inline(always)]
            pub const fn ccamode(&self) -> super::vals::Ccamode {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::Ccamode::from_bits(val as u8)
            }
            #[doc = "CCA mode of operation"]
            #[inline(always)]
            pub const fn set_ccamode(&mut self, val: super::vals::Ccamode) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
            }
            #[doc = "CCA energy busy threshold. Used in all the CCA modes except CarrierMode."]
            #[must_use]
            #[inline(always)]
            pub const fn ccaedthres(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "CCA energy busy threshold. Used in all the CCA modes except CarrierMode."]
            #[inline(always)]
            pub const fn set_ccaedthres(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[doc = "CCA correlator busy threshold. Only relevant to CarrierMode, CarrierAndEdMode, and CarrierOrEdMode."]
            #[must_use]
            #[inline(always)]
            pub const fn ccacorrthres(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0xff;
                val as u8
            }
            #[doc = "CCA correlator busy threshold. Only relevant to CarrierMode, CarrierAndEdMode, and CarrierOrEdMode."]
            #[inline(always)]
            pub const fn set_ccacorrthres(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
            }
            #[doc = "Limit for occurances above CCACORRTHRES. When not equal to zero the corrolator based signal detect is enabled."]
            #[must_use]
            #[inline(always)]
            pub const fn ccacorrcnt(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0xff;
                val as u8
            }
            #[doc = "Limit for occurances above CCACORRTHRES. When not equal to zero the corrolator based signal detect is enabled."]
            #[inline(always)]
            pub const fn set_ccacorrcnt(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
            }
        }
        impl Default for Ccactrl {
            #[inline(always)]
            fn default() -> Ccactrl {
                Ccactrl(0)
            }
        }
        impl core::fmt::Debug for Ccactrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ccactrl")
                    .field("ccamode", &self.ccamode())
                    .field("ccaedthres", &self.ccaedthres())
                    .field("ccacorrthres", &self.ccacorrthres())
                    .field("ccacorrcnt", &self.ccacorrcnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ccactrl {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Ccactrl {{ ccamode: {:?}, ccaedthres: {=u8:?}, ccacorrthres: {=u8:?}, ccacorrcnt: {=u8:?} }}" , self . ccamode () , self . ccaedthres () , self . ccacorrthres () , self . ccacorrcnt ())
            }
        }
        #[doc = "Clear the GPIO pattern array for antenna control"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Clearpattern(pub u32);
        impl Clearpattern {
            #[doc = "Clears GPIO pattern array for antenna control"]
            #[must_use]
            #[inline(always)]
            pub const fn clearpattern(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Clears GPIO pattern array for antenna control"]
            #[inline(always)]
            pub const fn set_clearpattern(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Clearpattern {
            #[inline(always)]
            fn default() -> Clearpattern {
                Clearpattern(0)
            }
        }
        impl core::fmt::Debug for Clearpattern {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Clearpattern")
                    .field("clearpattern", &self.clearpattern())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Clearpattern {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Clearpattern {{ clearpattern: {=bool:?} }}",
                    self.clearpattern()
                )
            }
        }
        #[doc = "CRC configuration"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Crccnf(pub u32);
        impl Crccnf {
            #[doc = "CRC length in number of bytes For MODE Ble_LR125Kbit and Ble_LR500Kbit, only LEN set to 3 is supported"]
            #[must_use]
            #[inline(always)]
            pub const fn len(&self) -> super::vals::Len {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Len::from_bits(val as u8)
            }
            #[doc = "CRC length in number of bytes For MODE Ble_LR125Kbit and Ble_LR500Kbit, only LEN set to 3 is supported"]
            #[inline(always)]
            pub const fn set_len(&mut self, val: super::vals::Len) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
            #[doc = "Include or exclude packet address field out of CRC calculation."]
            #[must_use]
            #[inline(always)]
            pub const fn skipaddr(&self) -> super::vals::Skipaddr {
                let val = (self.0 >> 8usize) & 0x03;
                super::vals::Skipaddr::from_bits(val as u8)
            }
            #[doc = "Include or exclude packet address field out of CRC calculation."]
            #[inline(always)]
            pub const fn set_skipaddr(&mut self, val: super::vals::Skipaddr) {
                self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
            }
        }
        impl Default for Crccnf {
            #[inline(always)]
            fn default() -> Crccnf {
                Crccnf(0)
            }
        }
        impl core::fmt::Debug for Crccnf {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Crccnf")
                    .field("len", &self.len())
                    .field("skipaddr", &self.skipaddr())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Crccnf {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Crccnf {{ len: {:?}, skipaddr: {:?} }}",
                    self.len(),
                    self.skipaddr()
                )
            }
        }
        #[doc = "CRC initial value"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Crcinit(pub u32);
        impl Crcinit {
            #[doc = "CRC initial value"]
            #[must_use]
            #[inline(always)]
            pub const fn crcinit(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x00ff_ffff;
                val as u32
            }
            #[doc = "CRC initial value"]
            #[inline(always)]
            pub const fn set_crcinit(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
            }
        }
        impl Default for Crcinit {
            #[inline(always)]
            fn default() -> Crcinit {
                Crcinit(0)
            }
        }
        impl core::fmt::Debug for Crcinit {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Crcinit")
                    .field("crcinit", &self.crcinit())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Crcinit {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Crcinit {{ crcinit: {=u32:?} }}", self.crcinit())
            }
        }
        #[doc = "CRC polynomial"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Crcpoly(pub u32);
        impl Crcpoly {
            #[doc = "CRC polynomial"]
            #[must_use]
            #[inline(always)]
            pub const fn crcpoly(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x00ff_ffff;
                val as u32
            }
            #[doc = "CRC polynomial"]
            #[inline(always)]
            pub const fn set_crcpoly(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
            }
        }
        impl Default for Crcpoly {
            #[inline(always)]
            fn default() -> Crcpoly {
                Crcpoly(0)
            }
        }
        impl core::fmt::Debug for Crcpoly {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Crcpoly")
                    .field("crcpoly", &self.crcpoly())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Crcpoly {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Crcpoly {{ crcpoly: {=u32:?} }}", self.crcpoly())
            }
        }
        #[doc = "CRC status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Crcstatus(pub u32);
        impl Crcstatus {
            #[doc = "CRC status of packet received"]
            #[must_use]
            #[inline(always)]
            pub const fn crcstatus(&self) -> super::vals::Crcstatus {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Crcstatus::from_bits(val as u8)
            }
            #[doc = "CRC status of packet received"]
            #[inline(always)]
            pub const fn set_crcstatus(&mut self, val: super::vals::Crcstatus) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Crcstatus {
            #[inline(always)]
            fn default() -> Crcstatus {
                Crcstatus(0)
            }
        }
        impl core::fmt::Debug for Crcstatus {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Crcstatus")
                    .field("crcstatus", &self.crcstatus())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Crcstatus {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Crcstatus {{ crcstatus: {:?} }}", self.crcstatus())
            }
        }
        #[doc = "Configuration for CTE inline mode"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cteinlineconf(pub u32);
        impl Cteinlineconf {
            #[doc = "Enable parsing of CTEInfo from received packet in BLE modes"]
            #[must_use]
            #[inline(always)]
            pub const fn cteinlinectrlen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable parsing of CTEInfo from received packet in BLE modes"]
            #[inline(always)]
            pub const fn set_cteinlinectrlen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "CTEInfo is S1 byte or not"]
            #[must_use]
            #[inline(always)]
            pub const fn cteinfoins1(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "CTEInfo is S1 byte or not"]
            #[inline(always)]
            pub const fn set_cteinfoins1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Sampling/switching if CRC is not OK"]
            #[must_use]
            #[inline(always)]
            pub const fn cteerrorhandling(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Sampling/switching if CRC is not OK"]
            #[inline(always)]
            pub const fn set_cteerrorhandling(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Max range of CTETime"]
            #[must_use]
            #[inline(always)]
            pub const fn ctetimevalidrange(&self) -> super::vals::Ctetimevalidrange {
                let val = (self.0 >> 6usize) & 0x03;
                super::vals::Ctetimevalidrange::from_bits(val as u8)
            }
            #[doc = "Max range of CTETime"]
            #[inline(always)]
            pub const fn set_ctetimevalidrange(&mut self, val: super::vals::Ctetimevalidrange) {
                self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
            }
            #[doc = "Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set."]
            #[must_use]
            #[inline(always)]
            pub const fn cteinlinerxmode1us(&self) -> super::vals::Cteinlinerxmode1us {
                let val = (self.0 >> 10usize) & 0x07;
                super::vals::Cteinlinerxmode1us::from_bits(val as u8)
            }
            #[doc = "Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set."]
            #[inline(always)]
            pub const fn set_cteinlinerxmode1us(&mut self, val: super::vals::Cteinlinerxmode1us) {
                self.0 =
                    (self.0 & !(0x07 << 10usize)) | (((val.to_bits() as u32) & 0x07) << 10usize);
            }
            #[doc = "Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set."]
            #[must_use]
            #[inline(always)]
            pub const fn cteinlinerxmode2us(&self) -> super::vals::Cteinlinerxmode2us {
                let val = (self.0 >> 13usize) & 0x07;
                super::vals::Cteinlinerxmode2us::from_bits(val as u8)
            }
            #[doc = "Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set."]
            #[inline(always)]
            pub const fn set_cteinlinerxmode2us(&mut self, val: super::vals::Cteinlinerxmode2us) {
                self.0 =
                    (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u32) & 0x07) << 13usize);
            }
            #[doc = "S0 bit pattern to match"]
            #[must_use]
            #[inline(always)]
            pub const fn s0conf(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0xff;
                val as u8
            }
            #[doc = "S0 bit pattern to match"]
            #[inline(always)]
            pub const fn set_s0conf(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
            }
            #[doc = "S0 bit mask to set which bit to match"]
            #[must_use]
            #[inline(always)]
            pub const fn s0mask(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0xff;
                val as u8
            }
            #[doc = "S0 bit mask to set which bit to match"]
            #[inline(always)]
            pub const fn set_s0mask(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
            }
        }
        impl Default for Cteinlineconf {
            #[inline(always)]
            fn default() -> Cteinlineconf {
                Cteinlineconf(0)
            }
        }
        impl core::fmt::Debug for Cteinlineconf {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Cteinlineconf")
                    .field("cteinlinectrlen", &self.cteinlinectrlen())
                    .field("cteinfoins1", &self.cteinfoins1())
                    .field("cteerrorhandling", &self.cteerrorhandling())
                    .field("ctetimevalidrange", &self.ctetimevalidrange())
                    .field("cteinlinerxmode1us", &self.cteinlinerxmode1us())
                    .field("cteinlinerxmode2us", &self.cteinlinerxmode2us())
                    .field("s0conf", &self.s0conf())
                    .field("s0mask", &self.s0mask())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Cteinlineconf {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Cteinlineconf {{ cteinlinectrlen: {=bool:?}, cteinfoins1: {=bool:?}, cteerrorhandling: {=bool:?}, ctetimevalidrange: {:?}, cteinlinerxmode1us: {:?}, cteinlinerxmode2us: {:?}, s0conf: {=u8:?}, s0mask: {=u8:?} }}" , self . cteinlinectrlen () , self . cteinfoins1 () , self . cteerrorhandling () , self . ctetimevalidrange () , self . cteinlinerxmode1us () , self . cteinlinerxmode2us () , self . s0conf () , self . s0mask ())
            }
        }
        #[doc = "CTEInfo parsed from received packet"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctestatus(pub u32);
        impl Ctestatus {
            #[doc = "CTETime parsed from packet"]
            #[must_use]
            #[inline(always)]
            pub const fn ctetime(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x1f;
                val as u8
            }
            #[doc = "CTETime parsed from packet"]
            #[inline(always)]
            pub const fn set_ctetime(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
            }
            #[doc = "RFU parsed from packet"]
            #[must_use]
            #[inline(always)]
            pub const fn rfu(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "RFU parsed from packet"]
            #[inline(always)]
            pub const fn set_rfu(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "CTEType parsed from packet"]
            #[must_use]
            #[inline(always)]
            pub const fn ctetype(&self) -> u8 {
                let val = (self.0 >> 6usize) & 0x03;
                val as u8
            }
            #[doc = "CTEType parsed from packet"]
            #[inline(always)]
            pub const fn set_ctetype(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
            }
        }
        impl Default for Ctestatus {
            #[inline(always)]
            fn default() -> Ctestatus {
                Ctestatus(0)
            }
        }
        impl core::fmt::Debug for Ctestatus {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctestatus")
                    .field("ctetime", &self.ctetime())
                    .field("rfu", &self.rfu())
                    .field("ctetype", &self.ctetype())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctestatus {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Ctestatus {{ ctetime: {=u8:?}, rfu: {=bool:?}, ctetype: {=u8:?} }}",
                    self.ctetime(),
                    self.rfu(),
                    self.ctetype()
                )
            }
        }
        #[doc = "Device address match configuration"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dacnf(pub u32);
        impl Dacnf {
            #[doc = "Enable or disable device address matching using device address 0"]
            #[must_use]
            #[inline(always)]
            pub const fn ena0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 0"]
            #[inline(always)]
            pub const fn set_ena0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable device address matching using device address 1"]
            #[must_use]
            #[inline(always)]
            pub const fn ena1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 1"]
            #[inline(always)]
            pub const fn set_ena1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable device address matching using device address 2"]
            #[must_use]
            #[inline(always)]
            pub const fn ena2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 2"]
            #[inline(always)]
            pub const fn set_ena2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable or disable device address matching using device address 3"]
            #[must_use]
            #[inline(always)]
            pub const fn ena3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 3"]
            #[inline(always)]
            pub const fn set_ena3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable or disable device address matching using device address 4"]
            #[must_use]
            #[inline(always)]
            pub const fn ena4(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 4"]
            #[inline(always)]
            pub const fn set_ena4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable or disable device address matching using device address 5"]
            #[must_use]
            #[inline(always)]
            pub const fn ena5(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 5"]
            #[inline(always)]
            pub const fn set_ena5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable or disable device address matching using device address 6"]
            #[must_use]
            #[inline(always)]
            pub const fn ena6(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 6"]
            #[inline(always)]
            pub const fn set_ena6(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Enable or disable device address matching using device address 7"]
            #[must_use]
            #[inline(always)]
            pub const fn ena7(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 7"]
            #[inline(always)]
            pub const fn set_ena7(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "TxAdd for device address 0"]
            #[must_use]
            #[inline(always)]
            pub const fn txadd0(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 0"]
            #[inline(always)]
            pub const fn set_txadd0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "TxAdd for device address 1"]
            #[must_use]
            #[inline(always)]
            pub const fn txadd1(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 1"]
            #[inline(always)]
            pub const fn set_txadd1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "TxAdd for device address 2"]
            #[must_use]
            #[inline(always)]
            pub const fn txadd2(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 2"]
            #[inline(always)]
            pub const fn set_txadd2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "TxAdd for device address 3"]
            #[must_use]
            #[inline(always)]
            pub const fn txadd3(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 3"]
            #[inline(always)]
            pub const fn set_txadd3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "TxAdd for device address 4"]
            #[must_use]
            #[inline(always)]
            pub const fn txadd4(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 4"]
            #[inline(always)]
            pub const fn set_txadd4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "TxAdd for device address 5"]
            #[must_use]
            #[inline(always)]
            pub const fn txadd5(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 5"]
            #[inline(always)]
            pub const fn set_txadd5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "TxAdd for device address 6"]
            #[must_use]
            #[inline(always)]
            pub const fn txadd6(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 6"]
            #[inline(always)]
            pub const fn set_txadd6(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "TxAdd for device address 7"]
            #[must_use]
            #[inline(always)]
            pub const fn txadd7(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 7"]
            #[inline(always)]
            pub const fn set_txadd7(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
        }
        impl Default for Dacnf {
            #[inline(always)]
            fn default() -> Dacnf {
                Dacnf(0)
            }
        }
        impl core::fmt::Debug for Dacnf {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dacnf")
                    .field("ena0", &self.ena0())
                    .field("ena1", &self.ena1())
                    .field("ena2", &self.ena2())
                    .field("ena3", &self.ena3())
                    .field("ena4", &self.ena4())
                    .field("ena5", &self.ena5())
                    .field("ena6", &self.ena6())
                    .field("ena7", &self.ena7())
                    .field("txadd0", &self.txadd0())
                    .field("txadd1", &self.txadd1())
                    .field("txadd2", &self.txadd2())
                    .field("txadd3", &self.txadd3())
                    .field("txadd4", &self.txadd4())
                    .field("txadd5", &self.txadd5())
                    .field("txadd6", &self.txadd6())
                    .field("txadd7", &self.txadd7())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dacnf {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Dacnf {{ ena0: {=bool:?}, ena1: {=bool:?}, ena2: {=bool:?}, ena3: {=bool:?}, ena4: {=bool:?}, ena5: {=bool:?}, ena6: {=bool:?}, ena7: {=bool:?}, txadd0: {=bool:?}, txadd1: {=bool:?}, txadd2: {=bool:?}, txadd3: {=bool:?}, txadd4: {=bool:?}, txadd5: {=bool:?}, txadd6: {=bool:?}, txadd7: {=bool:?} }}" , self . ena0 () , self . ena1 () , self . ena2 () , self . ena3 () , self . ena4 () , self . ena5 () , self . ena6 () , self . ena7 () , self . txadd0 () , self . txadd1 () , self . txadd2 () , self . txadd3 () , self . txadd4 () , self . txadd5 () , self . txadd6 () , self . txadd7 ())
            }
        }
        #[doc = "Device address match index"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dai(pub u32);
        impl Dai {
            #[doc = "Device address match index"]
            #[must_use]
            #[inline(always)]
            pub const fn dai(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x07;
                val as u8
            }
            #[doc = "Device address match index"]
            #[inline(always)]
            pub const fn set_dai(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
            }
        }
        impl Default for Dai {
            #[inline(always)]
            fn default() -> Dai {
                Dai(0)
            }
        }
        impl core::fmt::Debug for Dai {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dai").field("dai", &self.dai()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dai {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Dai {{ dai: {=u8:?} }}", self.dai())
            }
        }
        #[doc = "Description collection: Device address prefix n"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dap(pub u32);
        impl Dap {
            #[doc = "Device address prefix n"]
            #[must_use]
            #[inline(always)]
            pub const fn dap(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Device address prefix n"]
            #[inline(always)]
            pub const fn set_dap(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Dap {
            #[inline(always)]
            fn default() -> Dap {
                Dap(0)
            }
        }
        impl core::fmt::Debug for Dap {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dap").field("dap", &self.dap()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dap {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Dap {{ dap: {=u16:?} }}", self.dap())
            }
        }
        #[doc = "Data whitening initial value"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Datawhiteiv(pub u32);
        impl Datawhiteiv {
            #[doc = "Data whitening initial value. Bit 6 is hardwired to '1', writing '0' to it has no effect, and it will always be read back and used by the device as '1'."]
            #[must_use]
            #[inline(always)]
            pub const fn datawhiteiv(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "Data whitening initial value. Bit 6 is hardwired to '1', writing '0' to it has no effect, and it will always be read back and used by the device as '1'."]
            #[inline(always)]
            pub const fn set_datawhiteiv(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
            }
        }
        impl Default for Datawhiteiv {
            #[inline(always)]
            fn default() -> Datawhiteiv {
                Datawhiteiv(0)
            }
        }
        impl core::fmt::Debug for Datawhiteiv {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Datawhiteiv")
                    .field("datawhiteiv", &self.datawhiteiv())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Datawhiteiv {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Datawhiteiv {{ datawhiteiv: {=u8:?} }}",
                    self.datawhiteiv()
                )
            }
        }
        #[doc = "Various configuration for Direction finding"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dfectrl1(pub u32);
        impl Dfectrl1 {
            #[doc = "Length of the AoA/AoD procedure in number of 8 us units"]
            #[must_use]
            #[inline(always)]
            pub const fn numberof8us(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x3f;
                val as u8
            }
            #[doc = "Length of the AoA/AoD procedure in number of 8 us units"]
            #[inline(always)]
            pub const fn set_numberof8us(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
            }
            #[doc = "Add CTE extension and do antenna switching/sampling in this extension"]
            #[must_use]
            #[inline(always)]
            pub const fn dfeinextension(&self) -> super::vals::Dfeinextension {
                let val = (self.0 >> 7usize) & 0x01;
                super::vals::Dfeinextension::from_bits(val as u8)
            }
            #[doc = "Add CTE extension and do antenna switching/sampling in this extension"]
            #[inline(always)]
            pub const fn set_dfeinextension(&mut self, val: super::vals::Dfeinextension) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
            }
            #[doc = "Interval between every time the antenna is changed in the SWITCHING state"]
            #[must_use]
            #[inline(always)]
            pub const fn tswitchspacing(&self) -> super::vals::Tswitchspacing {
                let val = (self.0 >> 8usize) & 0x07;
                super::vals::Tswitchspacing::from_bits(val as u8)
            }
            #[doc = "Interval between every time the antenna is changed in the SWITCHING state"]
            #[inline(always)]
            pub const fn set_tswitchspacing(&mut self, val: super::vals::Tswitchspacing) {
                self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
            }
            #[doc = "Interval between samples in the REFERENCE period"]
            #[must_use]
            #[inline(always)]
            pub const fn tsamplespacingref(&self) -> super::vals::Tsamplespacingref {
                let val = (self.0 >> 12usize) & 0x07;
                super::vals::Tsamplespacingref::from_bits(val as u8)
            }
            #[doc = "Interval between samples in the REFERENCE period"]
            #[inline(always)]
            pub const fn set_tsamplespacingref(&mut self, val: super::vals::Tsamplespacingref) {
                self.0 =
                    (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
            }
            #[doc = "Whether to sample I/Q or magnitude/phase"]
            #[must_use]
            #[inline(always)]
            pub const fn sampletype(&self) -> super::vals::Sampletype {
                let val = (self.0 >> 15usize) & 0x01;
                super::vals::Sampletype::from_bits(val as u8)
            }
            #[doc = "Whether to sample I/Q or magnitude/phase"]
            #[inline(always)]
            pub const fn set_sampletype(&mut self, val: super::vals::Sampletype) {
                self.0 =
                    (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
            }
            #[doc = "Interval between samples in the SWITCHING period when CTEINLINECTRLEN is 0"]
            #[must_use]
            #[inline(always)]
            pub const fn tsamplespacing(&self) -> super::vals::Tsamplespacing {
                let val = (self.0 >> 16usize) & 0x07;
                super::vals::Tsamplespacing::from_bits(val as u8)
            }
            #[doc = "Interval between samples in the SWITCHING period when CTEINLINECTRLEN is 0"]
            #[inline(always)]
            pub const fn set_tsamplespacing(&mut self, val: super::vals::Tsamplespacing) {
                self.0 =
                    (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
            }
            #[doc = "Repeat each individual antenna pattern N times sequentially, i.e. P0, P0, P1, P1, P2, P2, P3, P3, etc."]
            #[must_use]
            #[inline(always)]
            pub const fn repeatpattern(&self) -> super::vals::Repeatpattern {
                let val = (self.0 >> 20usize) & 0x0f;
                super::vals::Repeatpattern::from_bits(val as u8)
            }
            #[doc = "Repeat each individual antenna pattern N times sequentially, i.e. P0, P0, P1, P1, P2, P2, P3, P3, etc."]
            #[inline(always)]
            pub const fn set_repeatpattern(&mut self, val: super::vals::Repeatpattern) {
                self.0 =
                    (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
            }
            #[doc = "Gain will be lowered by the specified number of gain steps at the start of CTE"]
            #[must_use]
            #[inline(always)]
            pub const fn agcbackoffgain(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0x0f;
                val as u8
            }
            #[doc = "Gain will be lowered by the specified number of gain steps at the start of CTE"]
            #[inline(always)]
            pub const fn set_agcbackoffgain(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
            }
        }
        impl Default for Dfectrl1 {
            #[inline(always)]
            fn default() -> Dfectrl1 {
                Dfectrl1(0)
            }
        }
        impl core::fmt::Debug for Dfectrl1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dfectrl1")
                    .field("numberof8us", &self.numberof8us())
                    .field("dfeinextension", &self.dfeinextension())
                    .field("tswitchspacing", &self.tswitchspacing())
                    .field("tsamplespacingref", &self.tsamplespacingref())
                    .field("sampletype", &self.sampletype())
                    .field("tsamplespacing", &self.tsamplespacing())
                    .field("repeatpattern", &self.repeatpattern())
                    .field("agcbackoffgain", &self.agcbackoffgain())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dfectrl1 {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Dfectrl1 {{ numberof8us: {=u8:?}, dfeinextension: {:?}, tswitchspacing: {:?}, tsamplespacingref: {:?}, sampletype: {:?}, tsamplespacing: {:?}, repeatpattern: {:?}, agcbackoffgain: {=u8:?} }}" , self . numberof8us () , self . dfeinextension () , self . tswitchspacing () , self . tsamplespacingref () , self . sampletype () , self . tsamplespacing () , self . repeatpattern () , self . agcbackoffgain ())
            }
        }
        #[doc = "Start offset for Direction finding"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dfectrl2(pub u32);
        impl Dfectrl2 {
            #[doc = "Signed value offset after the end of the CRC before starting switching in number of 16 MHz clock cycles"]
            #[must_use]
            #[inline(always)]
            pub const fn tswitchoffset(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x1fff;
                val as u16
            }
            #[doc = "Signed value offset after the end of the CRC before starting switching in number of 16 MHz clock cycles"]
            #[inline(always)]
            pub const fn set_tswitchoffset(&mut self, val: u16) {
                self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
            }
            #[doc = "Signed value offset in number of 16 MHz clock cycles for fine tuning of the sampling instant for all IQ samples. With TSAMPLEOFFSET=0 the first sample is taken immediately at the start of the reference period"]
            #[must_use]
            #[inline(always)]
            pub const fn tsampleoffset(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0x0fff;
                val as u16
            }
            #[doc = "Signed value offset in number of 16 MHz clock cycles for fine tuning of the sampling instant for all IQ samples. With TSAMPLEOFFSET=0 the first sample is taken immediately at the start of the reference period"]
            #[inline(always)]
            pub const fn set_tsampleoffset(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
            }
        }
        impl Default for Dfectrl2 {
            #[inline(always)]
            fn default() -> Dfectrl2 {
                Dfectrl2(0)
            }
        }
        impl core::fmt::Debug for Dfectrl2 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dfectrl2")
                    .field("tswitchoffset", &self.tswitchoffset())
                    .field("tsampleoffset", &self.tsampleoffset())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dfectrl2 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Dfectrl2 {{ tswitchoffset: {=u16:?}, tsampleoffset: {=u16:?} }}",
                    self.tswitchoffset(),
                    self.tsampleoffset()
                )
            }
        }
        #[doc = "Whether to use Angle-of-Arrival (AOA) or Angle-of-Departure (AOD)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dfemode(pub u32);
        impl Dfemode {
            #[doc = "Direction finding operation mode"]
            #[must_use]
            #[inline(always)]
            pub const fn dfeopmode(&self) -> super::vals::Dfeopmode {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Dfeopmode::from_bits(val as u8)
            }
            #[doc = "Direction finding operation mode"]
            #[inline(always)]
            pub const fn set_dfeopmode(&mut self, val: super::vals::Dfeopmode) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Dfemode {
            #[inline(always)]
            fn default() -> Dfemode {
                Dfemode(0)
            }
        }
        impl core::fmt::Debug for Dfemode {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dfemode")
                    .field("dfeopmode", &self.dfeopmode())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dfemode {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Dfemode {{ dfeopmode: {:?} }}", self.dfeopmode())
            }
        }
        #[doc = "DFE status information"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dfestatus(pub u32);
        impl Dfestatus {
            #[doc = "Internal state of switching state machine"]
            #[must_use]
            #[inline(always)]
            pub const fn switchingstate(&self) -> super::vals::Switchingstate {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::Switchingstate::from_bits(val as u8)
            }
            #[doc = "Internal state of switching state machine"]
            #[inline(always)]
            pub const fn set_switchingstate(&mut self, val: super::vals::Switchingstate) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
            }
            #[doc = "Internal state of sampling state machine"]
            #[must_use]
            #[inline(always)]
            pub const fn samplingstate(&self) -> super::vals::Samplingstate {
                let val = (self.0 >> 4usize) & 0x01;
                super::vals::Samplingstate::from_bits(val as u8)
            }
            #[doc = "Internal state of sampling state machine"]
            #[inline(always)]
            pub const fn set_samplingstate(&mut self, val: super::vals::Samplingstate) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
            }
        }
        impl Default for Dfestatus {
            #[inline(always)]
            fn default() -> Dfestatus {
                Dfestatus(0)
            }
        }
        impl core::fmt::Debug for Dfestatus {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dfestatus")
                    .field("switchingstate", &self.switchingstate())
                    .field("samplingstate", &self.samplingstate())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dfestatus {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Dfestatus {{ switchingstate: {:?}, samplingstate: {:?} }}",
                    self.switchingstate(),
                    self.samplingstate()
                )
            }
        }
        #[doc = "IEEE 802.15.4 energy detect loop count"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Edcnt(pub u32);
        impl Edcnt {
            #[doc = "IEEE 802.15.4 energy detect loop count"]
            #[must_use]
            #[inline(always)]
            pub const fn edcnt(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x001f_ffff;
                val as u32
            }
            #[doc = "IEEE 802.15.4 energy detect loop count"]
            #[inline(always)]
            pub const fn set_edcnt(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0x001f_ffff << 0usize)) | (((val as u32) & 0x001f_ffff) << 0usize);
            }
        }
        impl Default for Edcnt {
            #[inline(always)]
            fn default() -> Edcnt {
                Edcnt(0)
            }
        }
        impl core::fmt::Debug for Edcnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Edcnt")
                    .field("edcnt", &self.edcnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Edcnt {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Edcnt {{ edcnt: {=u32:?} }}", self.edcnt())
            }
        }
        #[doc = "IEEE 802.15.4 energy detect level"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Edsample(pub u32);
        impl Edsample {
            #[doc = "IEEE 802.15.4 energy detect level"]
            #[must_use]
            #[inline(always)]
            pub const fn edlvl(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "IEEE 802.15.4 energy detect level"]
            #[inline(always)]
            pub const fn set_edlvl(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Edsample {
            #[inline(always)]
            fn default() -> Edsample {
                Edsample(0)
            }
        }
        impl core::fmt::Debug for Edsample {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Edsample")
                    .field("edlvl", &self.edlvl())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Edsample {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Edsample {{ edlvl: {=u8:?} }}", self.edlvl())
            }
        }
        #[doc = "Frequency"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Frequency(pub u32);
        impl Frequency {
            #[doc = "Radio channel frequency"]
            #[must_use]
            #[inline(always)]
            pub const fn frequency(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "Radio channel frequency"]
            #[inline(always)]
            pub const fn set_frequency(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
            }
            #[doc = "Channel map selection"]
            #[must_use]
            #[inline(always)]
            pub const fn map(&self) -> super::vals::Map {
                let val = (self.0 >> 8usize) & 0x01;
                super::vals::Map::from_bits(val as u8)
            }
            #[doc = "Channel map selection"]
            #[inline(always)]
            pub const fn set_map(&mut self, val: super::vals::Map) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
            }
        }
        impl Default for Frequency {
            #[inline(always)]
            fn default() -> Frequency {
                Frequency(0)
            }
        }
        impl core::fmt::Debug for Frequency {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Frequency")
                    .field("frequency", &self.frequency())
                    .field("map", &self.map())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Frequency {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Frequency {{ frequency: {=u8:?}, map: {:?} }}",
                    self.frequency(),
                    self.map()
                )
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event READY"]
            #[must_use]
            #[inline(always)]
            pub const fn ready(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event READY"]
            #[inline(always)]
            pub const fn set_ready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to disable interrupt for event ADDRESS"]
            #[must_use]
            #[inline(always)]
            pub const fn address(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ADDRESS"]
            #[inline(always)]
            pub const fn set_address(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to disable interrupt for event PAYLOAD"]
            #[must_use]
            #[inline(always)]
            pub const fn payload(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event PAYLOAD"]
            #[inline(always)]
            pub const fn set_payload(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Write '1' to disable interrupt for event END"]
            #[must_use]
            #[inline(always)]
            pub const fn end(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event END"]
            #[inline(always)]
            pub const fn set_end(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Write '1' to disable interrupt for event DISABLED"]
            #[must_use]
            #[inline(always)]
            pub const fn disabled(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event DISABLED"]
            #[inline(always)]
            pub const fn set_disabled(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Write '1' to disable interrupt for event DEVMATCH"]
            #[must_use]
            #[inline(always)]
            pub const fn devmatch(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event DEVMATCH"]
            #[inline(always)]
            pub const fn set_devmatch(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Write '1' to disable interrupt for event DEVMISS"]
            #[must_use]
            #[inline(always)]
            pub const fn devmiss(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event DEVMISS"]
            #[inline(always)]
            pub const fn set_devmiss(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Write '1' to disable interrupt for event RSSIEND"]
            #[must_use]
            #[inline(always)]
            pub const fn rssiend(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event RSSIEND"]
            #[inline(always)]
            pub const fn set_rssiend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Write '1' to disable interrupt for event BCMATCH"]
            #[must_use]
            #[inline(always)]
            pub const fn bcmatch(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event BCMATCH"]
            #[inline(always)]
            pub const fn set_bcmatch(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Write '1' to disable interrupt for event CRCOK"]
            #[must_use]
            #[inline(always)]
            pub const fn crcok(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event CRCOK"]
            #[inline(always)]
            pub const fn set_crcok(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "Write '1' to disable interrupt for event CRCERROR"]
            #[must_use]
            #[inline(always)]
            pub const fn crcerror(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event CRCERROR"]
            #[inline(always)]
            pub const fn set_crcerror(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Write '1' to disable interrupt for event FRAMESTART"]
            #[must_use]
            #[inline(always)]
            pub const fn framestart(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event FRAMESTART"]
            #[inline(always)]
            pub const fn set_framestart(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Write '1' to disable interrupt for event EDEND"]
            #[must_use]
            #[inline(always)]
            pub const fn edend(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event EDEND"]
            #[inline(always)]
            pub const fn set_edend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
            #[doc = "Write '1' to disable interrupt for event EDSTOPPED"]
            #[must_use]
            #[inline(always)]
            pub const fn edstopped(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event EDSTOPPED"]
            #[inline(always)]
            pub const fn set_edstopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Write '1' to disable interrupt for event CCAIDLE"]
            #[must_use]
            #[inline(always)]
            pub const fn ccaidle(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event CCAIDLE"]
            #[inline(always)]
            pub const fn set_ccaidle(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Write '1' to disable interrupt for event CCABUSY"]
            #[must_use]
            #[inline(always)]
            pub const fn ccabusy(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event CCABUSY"]
            #[inline(always)]
            pub const fn set_ccabusy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Write '1' to disable interrupt for event CCASTOPPED"]
            #[must_use]
            #[inline(always)]
            pub const fn ccastopped(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event CCASTOPPED"]
            #[inline(always)]
            pub const fn set_ccastopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "Write '1' to disable interrupt for event RATEBOOST"]
            #[must_use]
            #[inline(always)]
            pub const fn rateboost(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event RATEBOOST"]
            #[inline(always)]
            pub const fn set_rateboost(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "Write '1' to disable interrupt for event TXREADY"]
            #[must_use]
            #[inline(always)]
            pub const fn txready(&self) -> bool {
                let val = (self.0 >> 21usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event TXREADY"]
            #[inline(always)]
            pub const fn set_txready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
            }
            #[doc = "Write '1' to disable interrupt for event RXREADY"]
            #[must_use]
            #[inline(always)]
            pub const fn rxready(&self) -> bool {
                let val = (self.0 >> 22usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event RXREADY"]
            #[inline(always)]
            pub const fn set_rxready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
            }
            #[doc = "Write '1' to disable interrupt for event MHRMATCH"]
            #[must_use]
            #[inline(always)]
            pub const fn mhrmatch(&self) -> bool {
                let val = (self.0 >> 23usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event MHRMATCH"]
            #[inline(always)]
            pub const fn set_mhrmatch(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
            }
            #[doc = "Write '1' to disable interrupt for event SYNC"]
            #[must_use]
            #[inline(always)]
            pub const fn sync(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event SYNC"]
            #[inline(always)]
            pub const fn set_sync(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
            #[doc = "Write '1' to disable interrupt for event PHYEND"]
            #[must_use]
            #[inline(always)]
            pub const fn phyend(&self) -> bool {
                let val = (self.0 >> 27usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event PHYEND"]
            #[inline(always)]
            pub const fn set_phyend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
            }
            #[doc = "Write '1' to disable interrupt for event CTEPRESENT"]
            #[must_use]
            #[inline(always)]
            pub const fn ctepresent(&self) -> bool {
                let val = (self.0 >> 28usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event CTEPRESENT"]
            #[inline(always)]
            pub const fn set_ctepresent(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("ready", &self.ready())
                    .field("address", &self.address())
                    .field("payload", &self.payload())
                    .field("end", &self.end())
                    .field("disabled", &self.disabled())
                    .field("devmatch", &self.devmatch())
                    .field("devmiss", &self.devmiss())
                    .field("rssiend", &self.rssiend())
                    .field("bcmatch", &self.bcmatch())
                    .field("crcok", &self.crcok())
                    .field("crcerror", &self.crcerror())
                    .field("framestart", &self.framestart())
                    .field("edend", &self.edend())
                    .field("edstopped", &self.edstopped())
                    .field("ccaidle", &self.ccaidle())
                    .field("ccabusy", &self.ccabusy())
                    .field("ccastopped", &self.ccastopped())
                    .field("rateboost", &self.rateboost())
                    .field("txready", &self.txready())
                    .field("rxready", &self.rxready())
                    .field("mhrmatch", &self.mhrmatch())
                    .field("sync", &self.sync())
                    .field("phyend", &self.phyend())
                    .field("ctepresent", &self.ctepresent())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Int {{ ready: {=bool:?}, address: {=bool:?}, payload: {=bool:?}, end: {=bool:?}, disabled: {=bool:?}, devmatch: {=bool:?}, devmiss: {=bool:?}, rssiend: {=bool:?}, bcmatch: {=bool:?}, crcok: {=bool:?}, crcerror: {=bool:?}, framestart: {=bool:?}, edend: {=bool:?}, edstopped: {=bool:?}, ccaidle: {=bool:?}, ccabusy: {=bool:?}, ccastopped: {=bool:?}, rateboost: {=bool:?}, txready: {=bool:?}, rxready: {=bool:?}, mhrmatch: {=bool:?}, sync: {=bool:?}, phyend: {=bool:?}, ctepresent: {=bool:?} }}" , self . ready () , self . address () , self . payload () , self . end () , self . disabled () , self . devmatch () , self . devmiss () , self . rssiend () , self . bcmatch () , self . crcok () , self . crcerror () , self . framestart () , self . edend () , self . edstopped () , self . ccaidle () , self . ccabusy () , self . ccastopped () , self . rateboost () , self . txready () , self . rxready () , self . mhrmatch () , self . sync () , self . phyend () , self . ctepresent ())
            }
        }
        #[doc = "Maximum number of buffer words to transfer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Maxcnt(pub u32);
        impl Maxcnt {
            #[doc = "Maximum number of buffer words to transfer"]
            #[must_use]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "Maximum number of buffer words to transfer"]
            #[inline(always)]
            pub const fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for Maxcnt {
            #[inline(always)]
            fn default() -> Maxcnt {
                Maxcnt(0)
            }
        }
        impl core::fmt::Debug for Maxcnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Maxcnt")
                    .field("maxcnt", &self.maxcnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Maxcnt {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Maxcnt {{ maxcnt: {=u16:?} }}", self.maxcnt())
            }
        }
        #[doc = "Data rate and modulation"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Mode(pub u32);
        impl Mode {
            #[doc = "Radio data rate and modulation setting. The radio supports frequency-shift keying (FSK) modulation."]
            #[must_use]
            #[inline(always)]
            pub const fn mode(&self) -> super::vals::Mode {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Mode::from_bits(val as u8)
            }
            #[doc = "Radio data rate and modulation setting. The radio supports frequency-shift keying (FSK) modulation."]
            #[inline(always)]
            pub const fn set_mode(&mut self, val: super::vals::Mode) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Mode {
            #[inline(always)]
            fn default() -> Mode {
                Mode(0)
            }
        }
        impl core::fmt::Debug for Mode {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Mode").field("mode", &self.mode()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Mode {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Mode {{ mode: {:?} }}", self.mode())
            }
        }
        #[doc = "Radio mode configuration register 0"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Modecnf0(pub u32);
        impl Modecnf0 {
            #[doc = "Radio ramp-up time"]
            #[must_use]
            #[inline(always)]
            pub const fn ru(&self) -> super::vals::Ru {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Ru::from_bits(val as u8)
            }
            #[doc = "Radio ramp-up time"]
            #[inline(always)]
            pub const fn set_ru(&mut self, val: super::vals::Ru) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Default TX value"]
            #[must_use]
            #[inline(always)]
            pub const fn dtx(&self) -> super::vals::Dtx {
                let val = (self.0 >> 8usize) & 0x03;
                super::vals::Dtx::from_bits(val as u8)
            }
            #[doc = "Default TX value"]
            #[inline(always)]
            pub const fn set_dtx(&mut self, val: super::vals::Dtx) {
                self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
            }
        }
        impl Default for Modecnf0 {
            #[inline(always)]
            fn default() -> Modecnf0 {
                Modecnf0(0)
            }
        }
        impl core::fmt::Debug for Modecnf0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Modecnf0")
                    .field("ru", &self.ru())
                    .field("dtx", &self.dtx())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Modecnf0 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Modecnf0 {{ ru: {:?}, dtx: {:?} }}",
                    self.ru(),
                    self.dtx()
                )
            }
        }
        #[doc = "Packet configuration register 0"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pcnf0(pub u32);
        impl Pcnf0 {
            #[doc = "Length on air of LENGTH field in number of bits"]
            #[must_use]
            #[inline(always)]
            pub const fn lflen(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "Length on air of LENGTH field in number of bits"]
            #[inline(always)]
            pub const fn set_lflen(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
            #[doc = "Length on air of S0 field in number of bytes"]
            #[must_use]
            #[inline(always)]
            pub const fn s0len(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Length on air of S0 field in number of bytes"]
            #[inline(always)]
            pub const fn set_s0len(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Length on air of S1 field in number of bits"]
            #[must_use]
            #[inline(always)]
            pub const fn s1len(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0x0f;
                val as u8
            }
            #[doc = "Length on air of S1 field in number of bits"]
            #[inline(always)]
            pub const fn set_s1len(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
            }
            #[doc = "Include or exclude S1 field in RAM"]
            #[must_use]
            #[inline(always)]
            pub const fn s1incl(&self) -> super::vals::S1incl {
                let val = (self.0 >> 20usize) & 0x01;
                super::vals::S1incl::from_bits(val as u8)
            }
            #[doc = "Include or exclude S1 field in RAM"]
            #[inline(always)]
            pub const fn set_s1incl(&mut self, val: super::vals::S1incl) {
                self.0 =
                    (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
            }
            #[doc = "Length of code indicator - Long Range"]
            #[must_use]
            #[inline(always)]
            pub const fn cilen(&self) -> u8 {
                let val = (self.0 >> 22usize) & 0x03;
                val as u8
            }
            #[doc = "Length of code indicator - Long Range"]
            #[inline(always)]
            pub const fn set_cilen(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
            }
            #[doc = "Length of preamble on air. Decision point: TASKS_START task"]
            #[must_use]
            #[inline(always)]
            pub const fn plen(&self) -> super::vals::Plen {
                let val = (self.0 >> 24usize) & 0x03;
                super::vals::Plen::from_bits(val as u8)
            }
            #[doc = "Length of preamble on air. Decision point: TASKS_START task"]
            #[inline(always)]
            pub const fn set_plen(&mut self, val: super::vals::Plen) {
                self.0 =
                    (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
            }
            #[doc = "Indicates if LENGTH field contains CRC or not"]
            #[must_use]
            #[inline(always)]
            pub const fn crcinc(&self) -> super::vals::Crcinc {
                let val = (self.0 >> 26usize) & 0x01;
                super::vals::Crcinc::from_bits(val as u8)
            }
            #[doc = "Indicates if LENGTH field contains CRC or not"]
            #[inline(always)]
            pub const fn set_crcinc(&mut self, val: super::vals::Crcinc) {
                self.0 =
                    (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
            }
            #[doc = "Length of TERM field in Long Range operation"]
            #[must_use]
            #[inline(always)]
            pub const fn termlen(&self) -> u8 {
                let val = (self.0 >> 29usize) & 0x03;
                val as u8
            }
            #[doc = "Length of TERM field in Long Range operation"]
            #[inline(always)]
            pub const fn set_termlen(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
            }
        }
        impl Default for Pcnf0 {
            #[inline(always)]
            fn default() -> Pcnf0 {
                Pcnf0(0)
            }
        }
        impl core::fmt::Debug for Pcnf0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Pcnf0")
                    .field("lflen", &self.lflen())
                    .field("s0len", &self.s0len())
                    .field("s1len", &self.s1len())
                    .field("s1incl", &self.s1incl())
                    .field("cilen", &self.cilen())
                    .field("plen", &self.plen())
                    .field("crcinc", &self.crcinc())
                    .field("termlen", &self.termlen())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Pcnf0 {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Pcnf0 {{ lflen: {=u8:?}, s0len: {=bool:?}, s1len: {=u8:?}, s1incl: {:?}, cilen: {=u8:?}, plen: {:?}, crcinc: {:?}, termlen: {=u8:?} }}" , self . lflen () , self . s0len () , self . s1len () , self . s1incl () , self . cilen () , self . plen () , self . crcinc () , self . termlen ())
            }
        }
        #[doc = "Packet configuration register 1"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pcnf1(pub u32);
        impl Pcnf1 {
            #[doc = "Maximum length of packet payload. If the packet payload is larger than MAXLEN, the radio will truncate the payload to MAXLEN."]
            #[must_use]
            #[inline(always)]
            pub const fn maxlen(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Maximum length of packet payload. If the packet payload is larger than MAXLEN, the radio will truncate the payload to MAXLEN."]
            #[inline(always)]
            pub const fn set_maxlen(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "Static length in number of bytes"]
            #[must_use]
            #[inline(always)]
            pub const fn statlen(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "Static length in number of bytes"]
            #[inline(always)]
            pub const fn set_statlen(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[doc = "Base address length in number of bytes"]
            #[must_use]
            #[inline(always)]
            pub const fn balen(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0x07;
                val as u8
            }
            #[doc = "Base address length in number of bytes"]
            #[inline(always)]
            pub const fn set_balen(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
            }
            #[doc = "On-air endianness of packet, this applies to the S0, LENGTH, S1, and the PAYLOAD fields."]
            #[must_use]
            #[inline(always)]
            pub const fn endian(&self) -> super::vals::Endian {
                let val = (self.0 >> 24usize) & 0x01;
                super::vals::Endian::from_bits(val as u8)
            }
            #[doc = "On-air endianness of packet, this applies to the S0, LENGTH, S1, and the PAYLOAD fields."]
            #[inline(always)]
            pub const fn set_endian(&mut self, val: super::vals::Endian) {
                self.0 =
                    (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
            }
            #[doc = "Enable or disable packet whitening"]
            #[must_use]
            #[inline(always)]
            pub const fn whiteen(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable packet whitening"]
            #[inline(always)]
            pub const fn set_whiteen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
        }
        impl Default for Pcnf1 {
            #[inline(always)]
            fn default() -> Pcnf1 {
                Pcnf1(0)
            }
        }
        impl core::fmt::Debug for Pcnf1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Pcnf1")
                    .field("maxlen", &self.maxlen())
                    .field("statlen", &self.statlen())
                    .field("balen", &self.balen())
                    .field("endian", &self.endian())
                    .field("whiteen", &self.whiteen())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Pcnf1 {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Pcnf1 {{ maxlen: {=u8:?}, statlen: {=u8:?}, balen: {=u8:?}, endian: {:?}, whiteen: {=bool:?} }}" , self . maxlen () , self . statlen () , self . balen () , self . endian () , self . whiteen ())
            }
        }
        #[doc = "Payload status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pdustat(pub u32);
        impl Pdustat {
            #[doc = "Status on payload length vs. PCNF1.MAXLEN"]
            #[must_use]
            #[inline(always)]
            pub const fn pdustat(&self) -> super::vals::Pdustat {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Pdustat::from_bits(val as u8)
            }
            #[doc = "Status on payload length vs. PCNF1.MAXLEN"]
            #[inline(always)]
            pub const fn set_pdustat(&mut self, val: super::vals::Pdustat) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Status on what rate packet is received with in Long Range"]
            #[must_use]
            #[inline(always)]
            pub const fn cistat(&self) -> super::vals::Cistat {
                let val = (self.0 >> 1usize) & 0x03;
                super::vals::Cistat::from_bits(val as u8)
            }
            #[doc = "Status on what rate packet is received with in Long Range"]
            #[inline(always)]
            pub const fn set_cistat(&mut self, val: super::vals::Cistat) {
                self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
            }
        }
        impl Default for Pdustat {
            #[inline(always)]
            fn default() -> Pdustat {
                Pdustat(0)
            }
        }
        impl core::fmt::Debug for Pdustat {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Pdustat")
                    .field("pdustat", &self.pdustat())
                    .field("cistat", &self.cistat())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Pdustat {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Pdustat {{ pdustat: {:?}, cistat: {:?} }}",
                    self.pdustat(),
                    self.cistat()
                )
            }
        }
        #[doc = "Peripheral power control"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Power(pub u32);
        impl Power {
            #[doc = "Peripheral power control. The peripheral and its registers will be reset to its initial state by switching the peripheral off and then back on again."]
            #[must_use]
            #[inline(always)]
            pub const fn power(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Peripheral power control. The peripheral and its registers will be reset to its initial state by switching the peripheral off and then back on again."]
            #[inline(always)]
            pub const fn set_power(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Power {
            #[inline(always)]
            fn default() -> Power {
                Power(0)
            }
        }
        impl core::fmt::Debug for Power {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Power")
                    .field("power", &self.power())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Power {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Power {{ power: {=bool:?} }}", self.power())
            }
        }
        #[doc = "Prefixes bytes for logical addresses 0-3"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Prefix0(pub u32);
        impl Prefix0 {
            #[doc = "Address prefix 0."]
            #[must_use]
            #[inline(always)]
            pub const fn ap0(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 0."]
            #[inline(always)]
            pub const fn set_ap0(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "Address prefix 1."]
            #[must_use]
            #[inline(always)]
            pub const fn ap1(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 1."]
            #[inline(always)]
            pub const fn set_ap1(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[doc = "Address prefix 2."]
            #[must_use]
            #[inline(always)]
            pub const fn ap2(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 2."]
            #[inline(always)]
            pub const fn set_ap2(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
            }
            #[doc = "Address prefix 3."]
            #[must_use]
            #[inline(always)]
            pub const fn ap3(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 3."]
            #[inline(always)]
            pub const fn set_ap3(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
            }
        }
        impl Default for Prefix0 {
            #[inline(always)]
            fn default() -> Prefix0 {
                Prefix0(0)
            }
        }
        impl core::fmt::Debug for Prefix0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Prefix0")
                    .field("ap0", &self.ap0())
                    .field("ap1", &self.ap1())
                    .field("ap2", &self.ap2())
                    .field("ap3", &self.ap3())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Prefix0 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Prefix0 {{ ap0: {=u8:?}, ap1: {=u8:?}, ap2: {=u8:?}, ap3: {=u8:?} }}",
                    self.ap0(),
                    self.ap1(),
                    self.ap2(),
                    self.ap3()
                )
            }
        }
        #[doc = "Prefixes bytes for logical addresses 4-7"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Prefix1(pub u32);
        impl Prefix1 {
            #[doc = "Address prefix 4."]
            #[must_use]
            #[inline(always)]
            pub const fn ap4(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 4."]
            #[inline(always)]
            pub const fn set_ap4(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "Address prefix 5."]
            #[must_use]
            #[inline(always)]
            pub const fn ap5(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 5."]
            #[inline(always)]
            pub const fn set_ap5(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[doc = "Address prefix 6."]
            #[must_use]
            #[inline(always)]
            pub const fn ap6(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 6."]
            #[inline(always)]
            pub const fn set_ap6(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
            }
            #[doc = "Address prefix 7."]
            #[must_use]
            #[inline(always)]
            pub const fn ap7(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 7."]
            #[inline(always)]
            pub const fn set_ap7(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
            }
        }
        impl Default for Prefix1 {
            #[inline(always)]
            fn default() -> Prefix1 {
                Prefix1(0)
            }
        }
        impl core::fmt::Debug for Prefix1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Prefix1")
                    .field("ap4", &self.ap4())
                    .field("ap5", &self.ap5())
                    .field("ap6", &self.ap6())
                    .field("ap7", &self.ap7())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Prefix1 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Prefix1 {{ ap4: {=u8:?}, ap5: {=u8:?}, ap6: {=u8:?}, ap7: {=u8:?} }}",
                    self.ap4(),
                    self.ap5(),
                    self.ap6(),
                    self.ap7()
                )
            }
        }
        #[doc = "RSSI sample"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rssisample(pub u32);
        impl Rssisample {
            #[doc = "RSSI sample."]
            #[must_use]
            #[inline(always)]
            pub const fn rssisample(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "RSSI sample."]
            #[inline(always)]
            pub const fn set_rssisample(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
            }
        }
        impl Default for Rssisample {
            #[inline(always)]
            fn default() -> Rssisample {
                Rssisample(0)
            }
        }
        impl core::fmt::Debug for Rssisample {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rssisample")
                    .field("rssisample", &self.rssisample())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rssisample {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Rssisample {{ rssisample: {=u8:?} }}", self.rssisample())
            }
        }
        #[doc = "Receive address select"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxaddresses(pub u32);
        impl Rxaddresses {
            #[doc = "Enable or disable reception on logical address 0."]
            #[must_use]
            #[inline(always)]
            pub const fn addr0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable reception on logical address 0."]
            #[inline(always)]
            pub const fn set_addr0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable reception on logical address 1."]
            #[must_use]
            #[inline(always)]
            pub const fn addr1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable reception on logical address 1."]
            #[inline(always)]
            pub const fn set_addr1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable reception on logical address 2."]
            #[must_use]
            #[inline(always)]
            pub const fn addr2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable reception on logical address 2."]
            #[inline(always)]
            pub const fn set_addr2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable or disable reception on logical address 3."]
            #[must_use]
            #[inline(always)]
            pub const fn addr3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable reception on logical address 3."]
            #[inline(always)]
            pub const fn set_addr3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable or disable reception on logical address 4."]
            #[must_use]
            #[inline(always)]
            pub const fn addr4(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable reception on logical address 4."]
            #[inline(always)]
            pub const fn set_addr4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable or disable reception on logical address 5."]
            #[must_use]
            #[inline(always)]
            pub const fn addr5(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable reception on logical address 5."]
            #[inline(always)]
            pub const fn set_addr5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable or disable reception on logical address 6."]
            #[must_use]
            #[inline(always)]
            pub const fn addr6(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable reception on logical address 6."]
            #[inline(always)]
            pub const fn set_addr6(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Enable or disable reception on logical address 7."]
            #[must_use]
            #[inline(always)]
            pub const fn addr7(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable reception on logical address 7."]
            #[inline(always)]
            pub const fn set_addr7(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
        }
        impl Default for Rxaddresses {
            #[inline(always)]
            fn default() -> Rxaddresses {
                Rxaddresses(0)
            }
        }
        impl core::fmt::Debug for Rxaddresses {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rxaddresses")
                    .field("addr0", &self.addr0())
                    .field("addr1", &self.addr1())
                    .field("addr2", &self.addr2())
                    .field("addr3", &self.addr3())
                    .field("addr4", &self.addr4())
                    .field("addr5", &self.addr5())
                    .field("addr6", &self.addr6())
                    .field("addr7", &self.addr7())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rxaddresses {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Rxaddresses {{ addr0: {=bool:?}, addr1: {=bool:?}, addr2: {=bool:?}, addr3: {=bool:?}, addr4: {=bool:?}, addr5: {=bool:?}, addr6: {=bool:?}, addr7: {=bool:?} }}" , self . addr0 () , self . addr1 () , self . addr2 () , self . addr3 () , self . addr4 () , self . addr5 () , self . addr6 () , self . addr7 ())
            }
        }
        #[doc = "CRC field of previously received packet"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxcrc(pub u32);
        impl Rxcrc {
            #[doc = "CRC field of previously received packet"]
            #[must_use]
            #[inline(always)]
            pub const fn rxcrc(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x00ff_ffff;
                val as u32
            }
            #[doc = "CRC field of previously received packet"]
            #[inline(always)]
            pub const fn set_rxcrc(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
            }
        }
        impl Default for Rxcrc {
            #[inline(always)]
            fn default() -> Rxcrc {
                Rxcrc(0)
            }
        }
        impl core::fmt::Debug for Rxcrc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rxcrc")
                    .field("rxcrc", &self.rxcrc())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rxcrc {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Rxcrc {{ rxcrc: {=u32:?} }}", self.rxcrc())
            }
        }
        #[doc = "Received address"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxmatch(pub u32);
        impl Rxmatch {
            #[doc = "Received address"]
            #[must_use]
            #[inline(always)]
            pub const fn rxmatch(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x07;
                val as u8
            }
            #[doc = "Received address"]
            #[inline(always)]
            pub const fn set_rxmatch(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
            }
        }
        impl Default for Rxmatch {
            #[inline(always)]
            fn default() -> Rxmatch {
                Rxmatch(0)
            }
        }
        impl core::fmt::Debug for Rxmatch {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rxmatch")
                    .field("rxmatch", &self.rxmatch())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rxmatch {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Rxmatch {{ rxmatch: {=u8:?} }}", self.rxmatch())
            }
        }
        #[doc = "IEEE 802.15.4 start of frame delimiter"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sfd(pub u32);
        impl Sfd {
            #[doc = "IEEE 802.15.4 start of frame delimiter"]
            #[must_use]
            #[inline(always)]
            pub const fn sfd(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "IEEE 802.15.4 start of frame delimiter"]
            #[inline(always)]
            pub const fn set_sfd(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Sfd {
            #[inline(always)]
            fn default() -> Sfd {
                Sfd(0)
            }
        }
        impl core::fmt::Debug for Sfd {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sfd").field("sfd", &self.sfd()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sfd {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Sfd {{ sfd: {=u8:?} }}", self.sfd())
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event READY and task START"]
            #[must_use]
            #[inline(always)]
            pub const fn ready_start(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event READY and task START"]
            #[inline(always)]
            pub const fn set_ready_start(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Shortcut between event END and task DISABLE"]
            #[must_use]
            #[inline(always)]
            pub const fn end_disable(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event END and task DISABLE"]
            #[inline(always)]
            pub const fn set_end_disable(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Shortcut between event DISABLED and task TXEN"]
            #[must_use]
            #[inline(always)]
            pub const fn disabled_txen(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event DISABLED and task TXEN"]
            #[inline(always)]
            pub const fn set_disabled_txen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Shortcut between event DISABLED and task RXEN"]
            #[must_use]
            #[inline(always)]
            pub const fn disabled_rxen(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event DISABLED and task RXEN"]
            #[inline(always)]
            pub const fn set_disabled_rxen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Shortcut between event ADDRESS and task RSSISTART"]
            #[must_use]
            #[inline(always)]
            pub const fn address_rssistart(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event ADDRESS and task RSSISTART"]
            #[inline(always)]
            pub const fn set_address_rssistart(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Shortcut between event END and task START"]
            #[must_use]
            #[inline(always)]
            pub const fn end_start(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event END and task START"]
            #[inline(always)]
            pub const fn set_end_start(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Shortcut between event ADDRESS and task BCSTART"]
            #[must_use]
            #[inline(always)]
            pub const fn address_bcstart(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event ADDRESS and task BCSTART"]
            #[inline(always)]
            pub const fn set_address_bcstart(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Shortcut between event DISABLED and task RSSISTOP"]
            #[must_use]
            #[inline(always)]
            pub const fn disabled_rssistop(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event DISABLED and task RSSISTOP"]
            #[inline(always)]
            pub const fn set_disabled_rssistop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Shortcut between event RXREADY and task CCASTART"]
            #[must_use]
            #[inline(always)]
            pub const fn rxready_ccastart(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event RXREADY and task CCASTART"]
            #[inline(always)]
            pub const fn set_rxready_ccastart(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "Shortcut between event CCAIDLE and task TXEN"]
            #[must_use]
            #[inline(always)]
            pub const fn ccaidle_txen(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event CCAIDLE and task TXEN"]
            #[inline(always)]
            pub const fn set_ccaidle_txen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "Shortcut between event CCABUSY and task DISABLE"]
            #[must_use]
            #[inline(always)]
            pub const fn ccabusy_disable(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event CCABUSY and task DISABLE"]
            #[inline(always)]
            pub const fn set_ccabusy_disable(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Shortcut between event FRAMESTART and task BCSTART"]
            #[must_use]
            #[inline(always)]
            pub const fn framestart_bcstart(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event FRAMESTART and task BCSTART"]
            #[inline(always)]
            pub const fn set_framestart_bcstart(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Shortcut between event READY and task EDSTART"]
            #[must_use]
            #[inline(always)]
            pub const fn ready_edstart(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event READY and task EDSTART"]
            #[inline(always)]
            pub const fn set_ready_edstart(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
            #[doc = "Shortcut between event EDEND and task DISABLE"]
            #[must_use]
            #[inline(always)]
            pub const fn edend_disable(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event EDEND and task DISABLE"]
            #[inline(always)]
            pub const fn set_edend_disable(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Shortcut between event CCAIDLE and task STOP"]
            #[must_use]
            #[inline(always)]
            pub const fn ccaidle_stop(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event CCAIDLE and task STOP"]
            #[inline(always)]
            pub const fn set_ccaidle_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Shortcut between event TXREADY and task START"]
            #[must_use]
            #[inline(always)]
            pub const fn txready_start(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event TXREADY and task START"]
            #[inline(always)]
            pub const fn set_txready_start(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Shortcut between event RXREADY and task START"]
            #[must_use]
            #[inline(always)]
            pub const fn rxready_start(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event RXREADY and task START"]
            #[inline(always)]
            pub const fn set_rxready_start(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "Shortcut between event PHYEND and task DISABLE"]
            #[must_use]
            #[inline(always)]
            pub const fn phyend_disable(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event PHYEND and task DISABLE"]
            #[inline(always)]
            pub const fn set_phyend_disable(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "Shortcut between event PHYEND and task START"]
            #[must_use]
            #[inline(always)]
            pub const fn phyend_start(&self) -> bool {
                let val = (self.0 >> 21usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event PHYEND and task START"]
            #[inline(always)]
            pub const fn set_phyend_start(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        impl core::fmt::Debug for Shorts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Shorts")
                    .field("ready_start", &self.ready_start())
                    .field("end_disable", &self.end_disable())
                    .field("disabled_txen", &self.disabled_txen())
                    .field("disabled_rxen", &self.disabled_rxen())
                    .field("address_rssistart", &self.address_rssistart())
                    .field("end_start", &self.end_start())
                    .field("address_bcstart", &self.address_bcstart())
                    .field("disabled_rssistop", &self.disabled_rssistop())
                    .field("rxready_ccastart", &self.rxready_ccastart())
                    .field("ccaidle_txen", &self.ccaidle_txen())
                    .field("ccabusy_disable", &self.ccabusy_disable())
                    .field("framestart_bcstart", &self.framestart_bcstart())
                    .field("ready_edstart", &self.ready_edstart())
                    .field("edend_disable", &self.edend_disable())
                    .field("ccaidle_stop", &self.ccaidle_stop())
                    .field("txready_start", &self.txready_start())
                    .field("rxready_start", &self.rxready_start())
                    .field("phyend_disable", &self.phyend_disable())
                    .field("phyend_start", &self.phyend_start())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Shorts {{ ready_start: {=bool:?}, end_disable: {=bool:?}, disabled_txen: {=bool:?}, disabled_rxen: {=bool:?}, address_rssistart: {=bool:?}, end_start: {=bool:?}, address_bcstart: {=bool:?}, disabled_rssistop: {=bool:?}, rxready_ccastart: {=bool:?}, ccaidle_txen: {=bool:?}, ccabusy_disable: {=bool:?}, framestart_bcstart: {=bool:?}, ready_edstart: {=bool:?}, edend_disable: {=bool:?}, ccaidle_stop: {=bool:?}, txready_start: {=bool:?}, rxready_start: {=bool:?}, phyend_disable: {=bool:?}, phyend_start: {=bool:?} }}" , self . ready_start () , self . end_disable () , self . disabled_txen () , self . disabled_rxen () , self . address_rssistart () , self . end_start () , self . address_bcstart () , self . disabled_rssistop () , self . rxready_ccastart () , self . ccaidle_txen () , self . ccabusy_disable () , self . framestart_bcstart () , self . ready_edstart () , self . edend_disable () , self . ccaidle_stop () , self . txready_start () , self . rxready_start () , self . phyend_disable () , self . phyend_start ())
            }
        }
        #[doc = "Current radio state"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct State(pub u32);
        impl State {
            #[doc = "Current radio state"]
            #[must_use]
            #[inline(always)]
            pub const fn state(&self) -> super::vals::State {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::State::from_bits(val as u8)
            }
            #[doc = "Current radio state"]
            #[inline(always)]
            pub const fn set_state(&mut self, val: super::vals::State) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for State {
            #[inline(always)]
            fn default() -> State {
                State(0)
            }
        }
        impl core::fmt::Debug for State {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("State")
                    .field("state", &self.state())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for State {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "State {{ state: {:?} }}", self.state())
            }
        }
        #[doc = "GPIO patterns to be used for each antenna"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Switchpattern(pub u32);
        impl Switchpattern {
            #[doc = "Fill array of GPIO patterns for antenna control."]
            #[must_use]
            #[inline(always)]
            pub const fn switchpattern(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Fill array of GPIO patterns for antenna control."]
            #[inline(always)]
            pub const fn set_switchpattern(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Switchpattern {
            #[inline(always)]
            fn default() -> Switchpattern {
                Switchpattern(0)
            }
        }
        impl core::fmt::Debug for Switchpattern {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Switchpattern")
                    .field("switchpattern", &self.switchpattern())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Switchpattern {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Switchpattern {{ switchpattern: {=u8:?} }}",
                    self.switchpattern()
                )
            }
        }
        #[doc = "Interframe spacing in us"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Tifs(pub u32);
        impl Tifs {
            #[doc = "Interframe spacing in us."]
            #[must_use]
            #[inline(always)]
            pub const fn tifs(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Interframe spacing in us."]
            #[inline(always)]
            pub const fn set_tifs(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
        }
        impl Default for Tifs {
            #[inline(always)]
            fn default() -> Tifs {
                Tifs(0)
            }
        }
        impl core::fmt::Debug for Tifs {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Tifs").field("tifs", &self.tifs()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Tifs {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Tifs {{ tifs: {=u16:?} }}", self.tifs())
            }
        }
        #[doc = "Transmit address select"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Txaddress(pub u32);
        impl Txaddress {
            #[doc = "Transmit address select"]
            #[must_use]
            #[inline(always)]
            pub const fn txaddress(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x07;
                val as u8
            }
            #[doc = "Transmit address select"]
            #[inline(always)]
            pub const fn set_txaddress(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
            }
        }
        impl Default for Txaddress {
            #[inline(always)]
            fn default() -> Txaddress {
                Txaddress(0)
            }
        }
        impl core::fmt::Debug for Txaddress {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Txaddress")
                    .field("txaddress", &self.txaddress())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Txaddress {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Txaddress {{ txaddress: {=u8:?} }}", self.txaddress())
            }
        }
        #[doc = "Output power"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Txpower(pub u32);
        impl Txpower {
            #[doc = "RADIO output power"]
            #[must_use]
            #[inline(always)]
            pub const fn txpower(&self) -> super::vals::Txpower {
                let val = (self.0 >> 0usize) & 0xff;
                super::vals::Txpower::from_bits(val as u8)
            }
            #[doc = "RADIO output power"]
            #[inline(always)]
            pub const fn set_txpower(&mut self, val: super::vals::Txpower) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Txpower {
            #[inline(always)]
            fn default() -> Txpower {
                Txpower(0)
            }
        }
        impl core::fmt::Debug for Txpower {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Txpower")
                    .field("txpower", &self.txpower())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Txpower {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Txpower {{ txpower: {:?} }}", self.txpower())
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Ccamode {
            #[doc = "Energy above threshold"]
            ED_MODE = 0x0,
            #[doc = "Carrier seen"]
            CARRIER_MODE = 0x01,
            #[doc = "Energy above threshold AND carrier seen"]
            CARRIER_AND_ED_MODE = 0x02,
            #[doc = "Energy above threshold OR carrier seen"]
            CARRIER_OR_ED_MODE = 0x03,
            #[doc = "Energy above threshold test mode that will abort when first ED measurement over threshold is seen. No averaging."]
            ED_MODE_TEST1 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl Ccamode {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Ccamode {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Ccamode {
            #[inline(always)]
            fn from(val: u8) -> Ccamode {
                Ccamode::from_bits(val)
            }
        }
        impl From<Ccamode> for u8 {
            #[inline(always)]
            fn from(val: Ccamode) -> u8 {
                Ccamode::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Cistat {
            #[doc = "Frame is received at 125 kbps"]
            LR125KBIT = 0x0,
            #[doc = "Frame is received at 500 kbps"]
            LR500KBIT = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Cistat {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Cistat {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Cistat {
            #[inline(always)]
            fn from(val: u8) -> Cistat {
                Cistat::from_bits(val)
            }
        }
        impl From<Cistat> for u8 {
            #[inline(always)]
            fn from(val: Cistat) -> u8 {
                Cistat::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Crcinc {
            #[doc = "LENGTH does not contain CRC"]
            EXCLUDE = 0x0,
            #[doc = "LENGTH includes CRC"]
            INCLUDE = 0x01,
        }
        impl Crcinc {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Crcinc {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Crcinc {
            #[inline(always)]
            fn from(val: u8) -> Crcinc {
                Crcinc::from_bits(val)
            }
        }
        impl From<Crcinc> for u8 {
            #[inline(always)]
            fn from(val: Crcinc) -> u8 {
                Crcinc::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Crcstatus {
            #[doc = "Packet received with CRC error"]
            CRCERROR = 0x0,
            #[doc = "Packet received with CRC ok"]
            CRCOK = 0x01,
        }
        impl Crcstatus {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Crcstatus {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Crcstatus {
            #[inline(always)]
            fn from(val: u8) -> Crcstatus {
                Crcstatus::from_bits(val)
            }
        }
        impl From<Crcstatus> for u8 {
            #[inline(always)]
            fn from(val: Crcstatus) -> u8 {
                Crcstatus::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Cteinlinerxmode1us {
            _RESERVED_0 = 0x0,
            #[doc = "4 us"]
            _4US = 0x01,
            #[doc = "2 us"]
            _2US = 0x02,
            #[doc = "1 us"]
            _1US = 0x03,
            #[doc = "0.5 us"]
            _500NS = 0x04,
            #[doc = "0.25 us"]
            _250NS = 0x05,
            #[doc = "0.125 us"]
            _125NS = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl Cteinlinerxmode1us {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Cteinlinerxmode1us {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Cteinlinerxmode1us {
            #[inline(always)]
            fn from(val: u8) -> Cteinlinerxmode1us {
                Cteinlinerxmode1us::from_bits(val)
            }
        }
        impl From<Cteinlinerxmode1us> for u8 {
            #[inline(always)]
            fn from(val: Cteinlinerxmode1us) -> u8 {
                Cteinlinerxmode1us::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Cteinlinerxmode2us {
            _RESERVED_0 = 0x0,
            #[doc = "4 us"]
            _4US = 0x01,
            #[doc = "2 us"]
            _2US = 0x02,
            #[doc = "1 us"]
            _1US = 0x03,
            #[doc = "0.5 us"]
            _500NS = 0x04,
            #[doc = "0.25 us"]
            _250NS = 0x05,
            #[doc = "0.125 us"]
            _125NS = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl Cteinlinerxmode2us {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Cteinlinerxmode2us {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Cteinlinerxmode2us {
            #[inline(always)]
            fn from(val: u8) -> Cteinlinerxmode2us {
                Cteinlinerxmode2us::from_bits(val)
            }
        }
        impl From<Cteinlinerxmode2us> for u8 {
            #[inline(always)]
            fn from(val: Cteinlinerxmode2us) -> u8 {
                Cteinlinerxmode2us::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Ctetimevalidrange {
            #[doc = "20 in 8 us unit (default) Set to 20 if parsed CTETime is larger than 20"]
            _20 = 0x0,
            #[doc = "31 in 8 us unit"]
            _31 = 0x01,
            #[doc = "63 in 8 us unit"]
            _63 = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Ctetimevalidrange {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Ctetimevalidrange {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Ctetimevalidrange {
            #[inline(always)]
            fn from(val: u8) -> Ctetimevalidrange {
                Ctetimevalidrange::from_bits(val)
            }
        }
        impl From<Ctetimevalidrange> for u8 {
            #[inline(always)]
            fn from(val: Ctetimevalidrange) -> u8 {
                Ctetimevalidrange::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Dfeinextension {
            #[doc = "Antenna switching/sampling is done in the packet payload"]
            PAYLOAD = 0x0,
            #[doc = "AoA/AoD procedure triggered at end of CRC"]
            CRC = 0x01,
        }
        impl Dfeinextension {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Dfeinextension {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Dfeinextension {
            #[inline(always)]
            fn from(val: u8) -> Dfeinextension {
                Dfeinextension::from_bits(val)
            }
        }
        impl From<Dfeinextension> for u8 {
            #[inline(always)]
            fn from(val: Dfeinextension) -> u8 {
                Dfeinextension::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Dfeopmode {
            #[doc = "Direction finding mode disabled"]
            DISABLED = 0x0,
            _RESERVED_1 = 0x01,
            #[doc = "Direction finding mode set to AoD"]
            AO_D = 0x02,
            #[doc = "Direction finding mode set to AoA"]
            AO_A = 0x03,
        }
        impl Dfeopmode {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Dfeopmode {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Dfeopmode {
            #[inline(always)]
            fn from(val: u8) -> Dfeopmode {
                Dfeopmode::from_bits(val)
            }
        }
        impl From<Dfeopmode> for u8 {
            #[inline(always)]
            fn from(val: Dfeopmode) -> u8 {
                Dfeopmode::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Dtx {
            #[doc = "Transmit '1'"]
            B1 = 0x0,
            #[doc = "Transmit '0'"]
            B0 = 0x01,
            #[doc = "Transmit center frequency"]
            CENTER = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Dtx {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Dtx {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Dtx {
            #[inline(always)]
            fn from(val: u8) -> Dtx {
                Dtx::from_bits(val)
            }
        }
        impl From<Dtx> for u8 {
            #[inline(always)]
            fn from(val: Dtx) -> u8 {
                Dtx::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Endian {
            #[doc = "Least significant bit on air first"]
            LITTLE = 0x0,
            #[doc = "Most significant bit on air first"]
            BIG = 0x01,
        }
        impl Endian {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Endian {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Endian {
            #[inline(always)]
            fn from(val: u8) -> Endian {
                Endian::from_bits(val)
            }
        }
        impl From<Endian> for u8 {
            #[inline(always)]
            fn from(val: Endian) -> u8 {
                Endian::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Len {
            #[doc = "CRC length is zero and CRC calculation is disabled"]
            DISABLED = 0x0,
            #[doc = "CRC length is one byte and CRC calculation is enabled"]
            ONE = 0x01,
            #[doc = "CRC length is two bytes and CRC calculation is enabled"]
            TWO = 0x02,
            #[doc = "CRC length is three bytes and CRC calculation is enabled"]
            THREE = 0x03,
        }
        impl Len {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Len {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Len {
            #[inline(always)]
            fn from(val: u8) -> Len {
                Len::from_bits(val)
            }
        }
        impl From<Len> for u8 {
            #[inline(always)]
            fn from(val: Len) -> u8 {
                Len::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Map {
            #[doc = "Channel map between 2400 MHz and 2500 MHz"]
            DEFAULT = 0x0,
            #[doc = "Channel map between 2360 MHz and 2460 MHz"]
            LOW = 0x01,
        }
        impl Map {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Map {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Map {
            #[inline(always)]
            fn from(val: u8) -> Map {
                Map::from_bits(val)
            }
        }
        impl From<Map> for u8 {
            #[inline(always)]
            fn from(val: Map) -> u8 {
                Map::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Mode {
            #[doc = "1 Mbps Nordic proprietary radio mode"]
            NRF_1MBIT = 0x0,
            #[doc = "2 Mbps Nordic proprietary radio mode"]
            NRF_2MBIT = 0x01,
            _RESERVED_2 = 0x02,
            #[doc = "1 Mbps BLE"]
            BLE_1MBIT = 0x03,
            #[doc = "2 Mbps BLE"]
            BLE_2MBIT = 0x04,
            #[doc = "Long Range 125 kbps TX, 125 kbps and 500 kbps RX"]
            BLE_LR125KBIT = 0x05,
            #[doc = "Long Range 500 kbps TX, 125 kbps and 500 kbps RX"]
            BLE_LR500KBIT = 0x06,
            _RESERVED_7 = 0x07,
            _RESERVED_8 = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            #[doc = "IEEE 802.15.4-2006 250 kbps"]
            IEEE802154_250KBIT = 0x0f,
        }
        impl Mode {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Mode {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Mode {
            #[inline(always)]
            fn from(val: u8) -> Mode {
                Mode::from_bits(val)
            }
        }
        impl From<Mode> for u8 {
            #[inline(always)]
            fn from(val: Mode) -> u8 {
                Mode::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Pdustat {
            #[doc = "Payload less than PCNF1.MAXLEN"]
            LESS_THAN = 0x0,
            #[doc = "Payload greater than PCNF1.MAXLEN"]
            GREATER_THAN = 0x01,
        }
        impl Pdustat {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Pdustat {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Pdustat {
            #[inline(always)]
            fn from(val: u8) -> Pdustat {
                Pdustat::from_bits(val)
            }
        }
        impl From<Pdustat> for u8 {
            #[inline(always)]
            fn from(val: Pdustat) -> u8 {
                Pdustat::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Plen {
            #[doc = "8-bit preamble"]
            _8BIT = 0x0,
            #[doc = "16-bit preamble"]
            _16BIT = 0x01,
            #[doc = "32-bit zero preamble - used for IEEE 802.15.4"]
            _32BIT_ZERO = 0x02,
            #[doc = "Preamble - used for Bluetooth LE Long Range"]
            LONG_RANGE = 0x03,
        }
        impl Plen {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Plen {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Plen {
            #[inline(always)]
            fn from(val: u8) -> Plen {
                Plen::from_bits(val)
            }
        }
        impl From<Plen> for u8 {
            #[inline(always)]
            fn from(val: Plen) -> u8 {
                Plen::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Repeatpattern {
            #[doc = "Do not repeat (1 time in total)"]
            NO_REPEAT = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
            _RESERVED_8 = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Repeatpattern {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Repeatpattern {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Repeatpattern {
            #[inline(always)]
            fn from(val: u8) -> Repeatpattern {
                Repeatpattern::from_bits(val)
            }
        }
        impl From<Repeatpattern> for u8 {
            #[inline(always)]
            fn from(val: Repeatpattern) -> u8 {
                Repeatpattern::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Ru {
            #[doc = "Default ramp-up time (tRXEN and tTXEN), compatible with firmware written for nRF51"]
            DEFAULT = 0x0,
            #[doc = "Fast ramp-up (tRXEN,FAST and tTXEN,FAST), see electrical specifications for more information"]
            FAST = 0x01,
        }
        impl Ru {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Ru {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Ru {
            #[inline(always)]
            fn from(val: u8) -> Ru {
                Ru::from_bits(val)
            }
        }
        impl From<Ru> for u8 {
            #[inline(always)]
            fn from(val: Ru) -> u8 {
                Ru::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum S1incl {
            #[doc = "Include S1 field in RAM only if S1LEN &gt; 0"]
            AUTOMATIC = 0x0,
            #[doc = "Always include S1 field in RAM independent of S1LEN"]
            INCLUDE = 0x01,
        }
        impl S1incl {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> S1incl {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for S1incl {
            #[inline(always)]
            fn from(val: u8) -> S1incl {
                S1incl::from_bits(val)
            }
        }
        impl From<S1incl> for u8 {
            #[inline(always)]
            fn from(val: S1incl) -> u8 {
                S1incl::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Sampletype {
            #[doc = "Complex samples in I and Q"]
            IQ = 0x0,
            #[doc = "Complex samples as magnitude and phase"]
            MAG_PHASE = 0x01,
        }
        impl Sampletype {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Sampletype {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Sampletype {
            #[inline(always)]
            fn from(val: u8) -> Sampletype {
                Sampletype::from_bits(val)
            }
        }
        impl From<Sampletype> for u8 {
            #[inline(always)]
            fn from(val: Sampletype) -> u8 {
                Sampletype::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Samplingstate {
            #[doc = "Sampling state Idle"]
            IDLE = 0x0,
            #[doc = "Sampling state Sampling"]
            SAMPLING = 0x01,
        }
        impl Samplingstate {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Samplingstate {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Samplingstate {
            #[inline(always)]
            fn from(val: u8) -> Samplingstate {
                Samplingstate::from_bits(val)
            }
        }
        impl From<Samplingstate> for u8 {
            #[inline(always)]
            fn from(val: Samplingstate) -> u8 {
                Samplingstate::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Skipaddr {
            #[doc = "CRC calculation includes address field"]
            INCLUDE = 0x0,
            #[doc = "CRC calculation does not include address field. The CRC calculation will start at the first byte after the address."]
            SKIP = 0x01,
            #[doc = "CRC calculation as per 802.15.4 standard. Starting at first byte after length field."]
            IEEE802154 = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Skipaddr {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Skipaddr {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Skipaddr {
            #[inline(always)]
            fn from(val: u8) -> Skipaddr {
                Skipaddr::from_bits(val)
            }
        }
        impl From<Skipaddr> for u8 {
            #[inline(always)]
            fn from(val: Skipaddr) -> u8 {
                Skipaddr::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum State {
            #[doc = "RADIO is in the Disabled state"]
            DISABLED = 0x0,
            #[doc = "RADIO is in the RXRU state"]
            RX_RU = 0x01,
            #[doc = "RADIO is in the RXIDLE state"]
            RX_IDLE = 0x02,
            #[doc = "RADIO is in the RX state"]
            RX = 0x03,
            #[doc = "RADIO is in the RXDISABLED state"]
            RX_DISABLE = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
            _RESERVED_8 = 0x08,
            #[doc = "RADIO is in the TXRU state"]
            TX_RU = 0x09,
            #[doc = "RADIO is in the TXIDLE state"]
            TX_IDLE = 0x0a,
            #[doc = "RADIO is in the TX state"]
            TX = 0x0b,
            #[doc = "RADIO is in the TXDISABLED state"]
            TX_DISABLE = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl State {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> State {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for State {
            #[inline(always)]
            fn from(val: u8) -> State {
                State::from_bits(val)
            }
        }
        impl From<State> for u8 {
            #[inline(always)]
            fn from(val: State) -> u8 {
                State::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Switchingstate {
            #[doc = "Switching state Idle"]
            IDLE = 0x0,
            #[doc = "Switching state Offset"]
            OFFSET = 0x01,
            #[doc = "Switching state Guard"]
            GUARD = 0x02,
            #[doc = "Switching state Ref"]
            REF = 0x03,
            #[doc = "Switching state Switching"]
            SWITCHING = 0x04,
            #[doc = "Switching state Ending"]
            ENDING = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl Switchingstate {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Switchingstate {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Switchingstate {
            #[inline(always)]
            fn from(val: u8) -> Switchingstate {
                Switchingstate::from_bits(val)
            }
        }
        impl From<Switchingstate> for u8 {
            #[inline(always)]
            fn from(val: Switchingstate) -> u8 {
                Switchingstate::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Tsamplespacing {
            _RESERVED_0 = 0x0,
            #[doc = "4 us"]
            _4US = 0x01,
            #[doc = "2 us"]
            _2US = 0x02,
            #[doc = "1 us"]
            _1US = 0x03,
            #[doc = "0.5 us"]
            _500NS = 0x04,
            #[doc = "0.25 us"]
            _250NS = 0x05,
            #[doc = "0.125 us"]
            _125NS = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl Tsamplespacing {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Tsamplespacing {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Tsamplespacing {
            #[inline(always)]
            fn from(val: u8) -> Tsamplespacing {
                Tsamplespacing::from_bits(val)
            }
        }
        impl From<Tsamplespacing> for u8 {
            #[inline(always)]
            fn from(val: Tsamplespacing) -> u8 {
                Tsamplespacing::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Tsamplespacingref {
            _RESERVED_0 = 0x0,
            #[doc = "4 us"]
            _4US = 0x01,
            #[doc = "2 us"]
            _2US = 0x02,
            #[doc = "1 us"]
            _1US = 0x03,
            #[doc = "0.5 us"]
            _500NS = 0x04,
            #[doc = "0.25 us"]
            _250NS = 0x05,
            #[doc = "0.125 us"]
            _125NS = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl Tsamplespacingref {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Tsamplespacingref {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Tsamplespacingref {
            #[inline(always)]
            fn from(val: u8) -> Tsamplespacingref {
                Tsamplespacingref::from_bits(val)
            }
        }
        impl From<Tsamplespacingref> for u8 {
            #[inline(always)]
            fn from(val: Tsamplespacingref) -> u8 {
                Tsamplespacingref::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Tswitchspacing {
            _RESERVED_0 = 0x0,
            #[doc = "4 us"]
            _4US = 0x01,
            #[doc = "2 us"]
            _2US = 0x02,
            #[doc = "1 us"]
            _1US = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl Tswitchspacing {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Tswitchspacing {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Tswitchspacing {
            #[inline(always)]
            fn from(val: u8) -> Tswitchspacing {
                Tswitchspacing::from_bits(val)
            }
        }
        impl From<Tswitchspacing> for u8 {
            #[inline(always)]
            fn from(val: Tswitchspacing) -> u8 {
                Tswitchspacing::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Txpower(u8);
        impl Txpower {
            #[doc = "0 dBm"]
            pub const _0_DBM: Self = Self(0x0);
            #[doc = "-40 dBm"]
            pub const NEG40_DBM: Self = Self(0xd8);
            #[doc = "Deprecated enumerator - -40 dBm"]
            pub const NEG30_DBM: Self = Self(0xe2);
            #[doc = "-20 dBm"]
            pub const NEG20_DBM: Self = Self(0xec);
            #[doc = "-16 dBm"]
            pub const NEG16_DBM: Self = Self(0xf0);
            #[doc = "-12 dBm"]
            pub const NEG12_DBM: Self = Self(0xf4);
            #[doc = "-8 dBm"]
            pub const NEG8_DBM: Self = Self(0xf8);
            #[doc = "-7 dBm"]
            pub const NEG7_DBM: Self = Self(0xf9);
            #[doc = "-6 dBm"]
            pub const NEG6_DBM: Self = Self(0xfa);
            #[doc = "-5 dBm"]
            pub const NEG5_DBM: Self = Self(0xfb);
            #[doc = "-4 dBm"]
            pub const NEG4_DBM: Self = Self(0xfc);
            #[doc = "-3 dBm"]
            pub const NEG3_DBM: Self = Self(0xfd);
            #[doc = "-2 dBm"]
            pub const NEG2_DBM: Self = Self(0xfe);
            #[doc = "-1 dBm"]
            pub const NEG1_DBM: Self = Self(0xff);
        }
        impl Txpower {
            pub const fn from_bits(val: u8) -> Txpower {
                Self(val & 0xff)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl core::fmt::Debug for Txpower {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0 => f.write_str("_0_DBM"),
                    0xd8 => f.write_str("NEG40_DBM"),
                    0xe2 => f.write_str("NEG30_DBM"),
                    0xec => f.write_str("NEG20_DBM"),
                    0xf0 => f.write_str("NEG16_DBM"),
                    0xf4 => f.write_str("NEG12_DBM"),
                    0xf8 => f.write_str("NEG8_DBM"),
                    0xf9 => f.write_str("NEG7_DBM"),
                    0xfa => f.write_str("NEG6_DBM"),
                    0xfb => f.write_str("NEG5_DBM"),
                    0xfc => f.write_str("NEG4_DBM"),
                    0xfd => f.write_str("NEG3_DBM"),
                    0xfe => f.write_str("NEG2_DBM"),
                    0xff => f.write_str("NEG1_DBM"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Txpower {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0 => defmt::write!(f, "_0_DBM"),
                    0xd8 => defmt::write!(f, "NEG40_DBM"),
                    0xe2 => defmt::write!(f, "NEG30_DBM"),
                    0xec => defmt::write!(f, "NEG20_DBM"),
                    0xf0 => defmt::write!(f, "NEG16_DBM"),
                    0xf4 => defmt::write!(f, "NEG12_DBM"),
                    0xf8 => defmt::write!(f, "NEG8_DBM"),
                    0xf9 => defmt::write!(f, "NEG7_DBM"),
                    0xfa => defmt::write!(f, "NEG6_DBM"),
                    0xfb => defmt::write!(f, "NEG5_DBM"),
                    0xfc => defmt::write!(f, "NEG4_DBM"),
                    0xfd => defmt::write!(f, "NEG3_DBM"),
                    0xfe => defmt::write!(f, "NEG2_DBM"),
                    0xff => defmt::write!(f, "NEG1_DBM"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u8> for Txpower {
            #[inline(always)]
            fn from(val: u8) -> Txpower {
                Txpower::from_bits(val)
            }
        }
        impl From<Txpower> for u8 {
            #[inline(always)]
            fn from(val: Txpower) -> u8 {
                Txpower::to_bits(val)
            }
        }
    }
}
pub mod reset {
    #[doc = "Reset control"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Reset {
        ptr: *mut u8,
    }
    unsafe impl Send for Reset {}
    unsafe impl Sync for Reset {}
    impl Reset {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Reset reason"]
        #[inline(always)]
        pub const fn resetreas(self) -> crate::common::Reg<regs::Resetreas, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Reset reason"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Resetreas(pub u32);
        impl Resetreas {
            #[doc = "Reset from pin reset detected"]
            #[must_use]
            #[inline(always)]
            pub const fn resetpin(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from pin reset detected"]
            #[inline(always)]
            pub const fn set_resetpin(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Reset from application watchdog timer 0 detected"]
            #[must_use]
            #[inline(always)]
            pub const fn dog0(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from application watchdog timer 0 detected"]
            #[inline(always)]
            pub const fn set_dog0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Reset from application CTRL-AP detected"]
            #[must_use]
            #[inline(always)]
            pub const fn ctrlap(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from application CTRL-AP detected"]
            #[inline(always)]
            pub const fn set_ctrlap(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Reset from application soft reset detected"]
            #[must_use]
            #[inline(always)]
            pub const fn sreq(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from application soft reset detected"]
            #[inline(always)]
            pub const fn set_sreq(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Reset from application CPU lockup detected"]
            #[must_use]
            #[inline(always)]
            pub const fn lockup(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from application CPU lockup detected"]
            #[inline(always)]
            pub const fn set_lockup(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Reset due to wakeup from System OFF mode when wakeup is triggered by DETECT signal from GPIO"]
            #[must_use]
            #[inline(always)]
            pub const fn off(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Reset due to wakeup from System OFF mode when wakeup is triggered by DETECT signal from GPIO"]
            #[inline(always)]
            pub const fn set_off(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Reset due to wakeup from System OFF mode when wakeup is triggered by ANADETECT signal from LPCOMP"]
            #[must_use]
            #[inline(always)]
            pub const fn lpcomp(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Reset due to wakeup from System OFF mode when wakeup is triggered by ANADETECT signal from LPCOMP"]
            #[inline(always)]
            pub const fn set_lpcomp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Reset due to wakeup from System OFF mode when wakeup is triggered by entering the Debug Interface mode"]
            #[must_use]
            #[inline(always)]
            pub const fn dif(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Reset due to wakeup from System OFF mode when wakeup is triggered by entering the Debug Interface mode"]
            #[inline(always)]
            pub const fn set_dif(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Reset from network soft reset detected"]
            #[must_use]
            #[inline(always)]
            pub const fn lsreq(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from network soft reset detected"]
            #[inline(always)]
            pub const fn set_lsreq(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Reset from network CPU lockup detected"]
            #[must_use]
            #[inline(always)]
            pub const fn llockup(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from network CPU lockup detected"]
            #[inline(always)]
            pub const fn set_llockup(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Reset from network watchdog timer detected"]
            #[must_use]
            #[inline(always)]
            pub const fn ldog(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from network watchdog timer detected"]
            #[inline(always)]
            pub const fn set_ldog(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Force-OFF reset from application core detected"]
            #[must_use]
            #[inline(always)]
            pub const fn mforceoff(&self) -> bool {
                let val = (self.0 >> 23usize) & 0x01;
                val != 0
            }
            #[doc = "Force-OFF reset from application core detected"]
            #[inline(always)]
            pub const fn set_mforceoff(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
            }
            #[doc = "Reset after wakeup from System OFF mode due to NFC field being detected"]
            #[must_use]
            #[inline(always)]
            pub const fn nfc(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "Reset after wakeup from System OFF mode due to NFC field being detected"]
            #[inline(always)]
            pub const fn set_nfc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
            #[doc = "Reset from application watchdog timer 1 detected"]
            #[must_use]
            #[inline(always)]
            pub const fn dog1(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from application watchdog timer 1 detected"]
            #[inline(always)]
            pub const fn set_dog1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
            #[doc = "Reset after wakeup from System OFF mode due to VBUS rising into valid range"]
            #[must_use]
            #[inline(always)]
            pub const fn vbus(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "Reset after wakeup from System OFF mode due to VBUS rising into valid range"]
            #[inline(always)]
            pub const fn set_vbus(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
            #[doc = "Reset from network CTRL-AP detected"]
            #[must_use]
            #[inline(always)]
            pub const fn lctrlap(&self) -> bool {
                let val = (self.0 >> 27usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from network CTRL-AP detected"]
            #[inline(always)]
            pub const fn set_lctrlap(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
            }
        }
        impl Default for Resetreas {
            #[inline(always)]
            fn default() -> Resetreas {
                Resetreas(0)
            }
        }
        impl core::fmt::Debug for Resetreas {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Resetreas")
                    .field("resetpin", &self.resetpin())
                    .field("dog0", &self.dog0())
                    .field("ctrlap", &self.ctrlap())
                    .field("sreq", &self.sreq())
                    .field("lockup", &self.lockup())
                    .field("off", &self.off())
                    .field("lpcomp", &self.lpcomp())
                    .field("dif", &self.dif())
                    .field("lsreq", &self.lsreq())
                    .field("llockup", &self.llockup())
                    .field("ldog", &self.ldog())
                    .field("mforceoff", &self.mforceoff())
                    .field("nfc", &self.nfc())
                    .field("dog1", &self.dog1())
                    .field("vbus", &self.vbus())
                    .field("lctrlap", &self.lctrlap())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Resetreas {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Resetreas {{ resetpin: {=bool:?}, dog0: {=bool:?}, ctrlap: {=bool:?}, sreq: {=bool:?}, lockup: {=bool:?}, off: {=bool:?}, lpcomp: {=bool:?}, dif: {=bool:?}, lsreq: {=bool:?}, llockup: {=bool:?}, ldog: {=bool:?}, mforceoff: {=bool:?}, nfc: {=bool:?}, dog1: {=bool:?}, vbus: {=bool:?}, lctrlap: {=bool:?} }}" , self . resetpin () , self . dog0 () , self . ctrlap () , self . sreq () , self . lockup () , self . off () , self . lpcomp () , self . dif () , self . lsreq () , self . llockup () , self . ldog () , self . mforceoff () , self . nfc () , self . dog1 () , self . vbus () , self . lctrlap ())
            }
        }
    }
}
pub mod rng {
    #[doc = "Random Number Generator"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rng {
        ptr: *mut u8,
    }
    unsafe impl Send for Rng {}
    unsafe impl Sync for Rng {}
    impl Rng {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Task starting the random number generator"]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Task stopping the random number generator"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Subscribe configuration for task START"]
        #[inline(always)]
        pub const fn subscribe_start(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
        }
        #[doc = "Subscribe configuration for task STOP"]
        #[inline(always)]
        pub const fn subscribe_stop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
        }
        #[doc = "Event being generated for every new random number written to the VALUE register"]
        #[inline(always)]
        pub const fn events_valrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
        }
        #[doc = "Publish configuration for event VALRDY"]
        #[inline(always)]
        pub const fn publish_valrdy(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
        }
        #[doc = "Configuration register"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
        }
        #[doc = "Output random number"]
        #[inline(always)]
        pub const fn value(self) -> crate::common::Reg<regs::Value, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Bias correction"]
            #[must_use]
            #[inline(always)]
            pub const fn dercen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Bias correction"]
            #[inline(always)]
            pub const fn set_dercen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        impl core::fmt::Debug for Config {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Config")
                    .field("dercen", &self.dercen())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Config {{ dercen: {=bool:?} }}", self.dercen())
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event VALRDY"]
            #[must_use]
            #[inline(always)]
            pub const fn valrdy(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event VALRDY"]
            #[inline(always)]
            pub const fn set_valrdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("valrdy", &self.valrdy())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Int {{ valrdy: {=bool:?} }}", self.valrdy())
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event VALRDY and task STOP"]
            #[must_use]
            #[inline(always)]
            pub const fn valrdy_stop(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event VALRDY and task STOP"]
            #[inline(always)]
            pub const fn set_valrdy_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        impl core::fmt::Debug for Shorts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Shorts")
                    .field("valrdy_stop", &self.valrdy_stop())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Shorts {{ valrdy_stop: {=bool:?} }}", self.valrdy_stop())
            }
        }
        #[doc = "Output random number"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Value(pub u32);
        impl Value {
            #[doc = "Generated random number"]
            #[must_use]
            #[inline(always)]
            pub const fn value(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Generated random number"]
            #[inline(always)]
            pub const fn set_value(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Value {
            #[inline(always)]
            fn default() -> Value {
                Value(0)
            }
        }
        impl core::fmt::Debug for Value {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Value")
                    .field("value", &self.value())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Value {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Value {{ value: {=u8:?} }}", self.value())
            }
        }
    }
}
pub mod rtc {
    #[doc = "Real-time counter 0"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rtc {
        ptr: *mut u8,
    }
    unsafe impl Send for Rtc {}
    unsafe impl Sync for Rtc {}
    impl Rtc {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start RTC counter"]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Stop RTC counter"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Clear RTC counter"]
        #[inline(always)]
        pub const fn tasks_clear(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "Set counter to 0xFFFFF0"]
        #[inline(always)]
        pub const fn tasks_trigovrflw(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
        #[doc = "Description collection: Capture RTC counter to CC\\[n\\] register"]
        #[inline(always)]
        pub const fn tasks_capture(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
            assert!(n < 4usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _)
            }
        }
        #[doc = "Subscribe configuration for task START"]
        #[inline(always)]
        pub const fn subscribe_start(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
        }
        #[doc = "Subscribe configuration for task STOP"]
        #[inline(always)]
        pub const fn subscribe_stop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
        }
        #[doc = "Subscribe configuration for task CLEAR"]
        #[inline(always)]
        pub const fn subscribe_clear(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
        }
        #[doc = "Subscribe configuration for task TRIGOVRFLW"]
        #[inline(always)]
        pub const fn subscribe_trigovrflw(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
        }
        #[doc = "Description collection: Subscribe configuration for task CAPTURE\\[n\\]"]
        #[inline(always)]
        pub const fn subscribe_capture(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            assert!(n < 4usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize + n * 4usize) as _)
            }
        }
        #[doc = "Event on counter increment"]
        #[inline(always)]
        pub const fn events_tick(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
        }
        #[doc = "Event on counter overflow"]
        #[inline(always)]
        pub const fn events_ovrflw(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
        }
        #[doc = "Description collection: Compare event on CC\\[n\\] match"]
        #[inline(always)]
        pub const fn events_compare(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 4usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize + n * 4usize) as _)
            }
        }
        #[doc = "Publish configuration for event TICK"]
        #[inline(always)]
        pub const fn publish_tick(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
        }
        #[doc = "Publish configuration for event OVRFLW"]
        #[inline(always)]
        pub const fn publish_ovrflw(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
        }
        #[doc = "Description collection: Publish configuration for event COMPARE\\[n\\]"]
        #[inline(always)]
        pub const fn publish_compare(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            assert!(n < 4usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize + n * 4usize) as _)
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
        }
        #[doc = "Enable or disable event routing"]
        #[inline(always)]
        pub const fn evten(self) -> crate::common::Reg<regs::Evt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0340usize) as _) }
        }
        #[doc = "Enable event routing"]
        #[inline(always)]
        pub const fn evtenset(self) -> crate::common::Reg<regs::Evt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0344usize) as _) }
        }
        #[doc = "Disable event routing"]
        #[inline(always)]
        pub const fn evtenclr(self) -> crate::common::Reg<regs::Evt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0348usize) as _) }
        }
        #[doc = "Current counter value"]
        #[inline(always)]
        pub const fn counter(self) -> crate::common::Reg<regs::Counter, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
        }
        #[doc = "12-bit prescaler for counter frequency (32768 / (PRESCALER + 1)). Must be written when RTC is stopped."]
        #[inline(always)]
        pub const fn prescaler(self) -> crate::common::Reg<regs::Prescaler, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
        }
        #[doc = "Description collection: Compare register n"]
        #[inline(always)]
        pub const fn cc(self, n: usize) -> crate::common::Reg<regs::Cc, crate::common::RW> {
            assert!(n < 4usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0540usize + n * 4usize) as _)
            }
        }
    }
    pub mod regs {
        #[doc = "Description collection: Compare register n"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cc(pub u32);
        impl Cc {
            #[doc = "Compare value"]
            #[must_use]
            #[inline(always)]
            pub const fn compare(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x00ff_ffff;
                val as u32
            }
            #[doc = "Compare value"]
            #[inline(always)]
            pub const fn set_compare(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
            }
        }
        impl Default for Cc {
            #[inline(always)]
            fn default() -> Cc {
                Cc(0)
            }
        }
        impl core::fmt::Debug for Cc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Cc")
                    .field("compare", &self.compare())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Cc {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Cc {{ compare: {=u32:?} }}", self.compare())
            }
        }
        #[doc = "Current counter value"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Counter(pub u32);
        impl Counter {
            #[doc = "Counter value"]
            #[must_use]
            #[inline(always)]
            pub const fn counter(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x00ff_ffff;
                val as u32
            }
            #[doc = "Counter value"]
            #[inline(always)]
            pub const fn set_counter(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
            }
        }
        impl Default for Counter {
            #[inline(always)]
            fn default() -> Counter {
                Counter(0)
            }
        }
        impl core::fmt::Debug for Counter {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Counter")
                    .field("counter", &self.counter())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Counter {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Counter {{ counter: {=u32:?} }}", self.counter())
            }
        }
        #[doc = "Enable or disable event routing"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Evt(pub u32);
        impl Evt {
            #[doc = "Enable or disable event routing for event TICK"]
            #[must_use]
            #[inline(always)]
            pub const fn tick(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable event routing for event TICK"]
            #[inline(always)]
            pub const fn set_tick(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable event routing for event OVRFLW"]
            #[must_use]
            #[inline(always)]
            pub const fn ovrflw(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable event routing for event OVRFLW"]
            #[inline(always)]
            pub const fn set_ovrflw(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable event routing for event COMPARE\\[0\\]"]
            #[must_use]
            #[inline(always)]
            pub const fn compare(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 16usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable event routing for event COMPARE\\[0\\]"]
            #[inline(always)]
            pub const fn set_compare(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 16usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Evt {
            #[inline(always)]
            fn default() -> Evt {
                Evt(0)
            }
        }
        impl core::fmt::Debug for Evt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Evt")
                    .field("tick", &self.tick())
                    .field("ovrflw", &self.ovrflw())
                    .field("compare[0]", &self.compare(0usize))
                    .field("compare[1]", &self.compare(1usize))
                    .field("compare[2]", &self.compare(2usize))
                    .field("compare[3]", &self.compare(3usize))
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Evt {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Evt {{ tick: {=bool:?}, ovrflw: {=bool:?}, compare[0]: {=bool:?}, compare[1]: {=bool:?}, compare[2]: {=bool:?}, compare[3]: {=bool:?} }}" , self . tick () , self . ovrflw () , self . compare (0usize) , self . compare (1usize) , self . compare (2usize) , self . compare (3usize))
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event TICK"]
            #[must_use]
            #[inline(always)]
            pub const fn tick(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event TICK"]
            #[inline(always)]
            pub const fn set_tick(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to disable interrupt for event OVRFLW"]
            #[must_use]
            #[inline(always)]
            pub const fn ovrflw(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event OVRFLW"]
            #[inline(always)]
            pub const fn set_ovrflw(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to disable interrupt for event COMPARE\\[0\\]"]
            #[must_use]
            #[inline(always)]
            pub const fn compare(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 16usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event COMPARE\\[0\\]"]
            #[inline(always)]
            pub const fn set_compare(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 16usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("tick", &self.tick())
                    .field("ovrflw", &self.ovrflw())
                    .field("compare[0]", &self.compare(0usize))
                    .field("compare[1]", &self.compare(1usize))
                    .field("compare[2]", &self.compare(2usize))
                    .field("compare[3]", &self.compare(3usize))
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Int {{ tick: {=bool:?}, ovrflw: {=bool:?}, compare[0]: {=bool:?}, compare[1]: {=bool:?}, compare[2]: {=bool:?}, compare[3]: {=bool:?} }}" , self . tick () , self . ovrflw () , self . compare (0usize) , self . compare (1usize) , self . compare (2usize) , self . compare (3usize))
            }
        }
        #[doc = "12-bit prescaler for counter frequency (32768 / (PRESCALER + 1)). Must be written when RTC is stopped."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Prescaler(pub u32);
        impl Prescaler {
            #[doc = "Prescaler value"]
            #[must_use]
            #[inline(always)]
            pub const fn prescaler(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "Prescaler value"]
            #[inline(always)]
            pub const fn set_prescaler(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
        }
        impl Default for Prescaler {
            #[inline(always)]
            fn default() -> Prescaler {
                Prescaler(0)
            }
        }
        impl core::fmt::Debug for Prescaler {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Prescaler")
                    .field("prescaler", &self.prescaler())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Prescaler {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Prescaler {{ prescaler: {=u16:?} }}", self.prescaler())
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event COMPARE\\[0\\] and task CLEAR"]
            #[must_use]
            #[inline(always)]
            pub const fn comparen_clear(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event COMPARE\\[0\\] and task CLEAR"]
            #[inline(always)]
            pub const fn set_comparen_clear(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        impl core::fmt::Debug for Shorts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Shorts")
                    .field("comparen_clear[0]", &self.comparen_clear(0usize))
                    .field("comparen_clear[1]", &self.comparen_clear(1usize))
                    .field("comparen_clear[2]", &self.comparen_clear(2usize))
                    .field("comparen_clear[3]", &self.comparen_clear(3usize))
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Shorts {{ comparen_clear[0]: {=bool:?}, comparen_clear[1]: {=bool:?}, comparen_clear[2]: {=bool:?}, comparen_clear[3]: {=bool:?} }}" , self . comparen_clear (0usize) , self . comparen_clear (1usize) , self . comparen_clear (2usize) , self . comparen_clear (3usize))
            }
        }
    }
}
pub mod shared {
    pub mod regs {
        #[doc = "Description collection: Pin select for DFE pin n"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Psel(pub u32);
        impl Psel {
            #[doc = "Pin number"]
            #[must_use]
            #[inline(always)]
            pub const fn pin(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x1f;
                val as u8
            }
            #[doc = "Pin number"]
            #[inline(always)]
            pub const fn set_pin(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
            }
            #[doc = "Port number"]
            #[must_use]
            #[inline(always)]
            pub const fn port(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Port number"]
            #[inline(always)]
            pub const fn set_port(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Connection"]
            #[must_use]
            #[inline(always)]
            pub const fn connect(&self) -> super::vals::Connect {
                let val = (self.0 >> 31usize) & 0x01;
                super::vals::Connect::from_bits(val as u8)
            }
            #[doc = "Connection"]
            #[inline(always)]
            pub const fn set_connect(&mut self, val: super::vals::Connect) {
                self.0 =
                    (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
            }
        }
        impl Default for Psel {
            #[inline(always)]
            fn default() -> Psel {
                Psel(0)
            }
        }
        impl core::fmt::Debug for Psel {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Psel")
                    .field("pin", &self.pin())
                    .field("port", &self.port())
                    .field("connect", &self.connect())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Psel {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Psel {{ pin: {=u8:?}, port: {=bool:?}, connect: {:?} }}",
                    self.pin(),
                    self.port(),
                    self.connect()
                )
            }
        }
        #[doc = "Publish configuration for event END"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Publish(pub u32);
        impl Publish {
            #[doc = "DPPI channel that event END will publish to."]
            #[must_use]
            #[inline(always)]
            pub const fn chidx(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "DPPI channel that event END will publish to."]
            #[inline(always)]
            pub const fn set_chidx(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn en(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
            }
        }
        impl Default for Publish {
            #[inline(always)]
            fn default() -> Publish {
                Publish(0)
            }
        }
        impl core::fmt::Debug for Publish {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Publish")
                    .field("chidx", &self.chidx())
                    .field("en", &self.en())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Publish {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Publish {{ chidx: {=u8:?}, en: {=bool:?} }}",
                    self.chidx(),
                    self.en()
                )
            }
        }
        #[doc = "Subscribe configuration for task START"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Subscribe(pub u32);
        impl Subscribe {
            #[doc = "DPPI channel that task START will subscribe to"]
            #[must_use]
            #[inline(always)]
            pub const fn chidx(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "DPPI channel that task START will subscribe to"]
            #[inline(always)]
            pub const fn set_chidx(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[must_use]
            #[inline(always)]
            pub const fn en(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub const fn set_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
            }
        }
        impl Default for Subscribe {
            #[inline(always)]
            fn default() -> Subscribe {
                Subscribe(0)
            }
        }
        impl core::fmt::Debug for Subscribe {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Subscribe")
                    .field("chidx", &self.chidx())
                    .field("en", &self.en())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Subscribe {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Subscribe {{ chidx: {=u8:?}, en: {=bool:?} }}",
                    self.chidx(),
                    self.en()
                )
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Connect {
            #[doc = "Connect"]
            CONNECTED = 0x0,
            #[doc = "Disconnect"]
            DISCONNECTED = 0x01,
        }
        impl Connect {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Connect {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Connect {
            #[inline(always)]
            fn from(val: u8) -> Connect {
                Connect::from_bits(val)
            }
        }
        impl From<Connect> for u8 {
            #[inline(always)]
            fn from(val: Connect) -> u8 {
                Connect::to_bits(val)
            }
        }
    }
}
pub mod spim {
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iftiming {
        ptr: *mut u8,
    }
    unsafe impl Send for Iftiming {}
    unsafe impl Sync for Iftiming {}
    impl Iftiming {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Sample delay for input serial data on MISO"]
        #[inline(always)]
        pub const fn rxdelay(self) -> crate::common::Reg<regs::Rxdelay, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Minimum duration between edge of CSN and edge of SCK. When SHORTS.END_START is used, this is also the minimum duration CSN must stay high between transactions."]
        #[inline(always)]
        pub const fn csndur(self) -> crate::common::Reg<regs::Csndur, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psel {
        ptr: *mut u8,
    }
    unsafe impl Send for Psel {}
    unsafe impl Sync for Psel {}
    impl Psel {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Pin select for SCK"]
        #[inline(always)]
        pub const fn sck(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Pin select for MOSI signal"]
        #[inline(always)]
        pub const fn mosi(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Pin select for MISO signal"]
        #[inline(always)]
        pub const fn miso(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "Pin select for CSN"]
        #[inline(always)]
        pub const fn csn(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
    }
    #[doc = "RXD EasyDMA channel"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxd {
        ptr: *mut u8,
    }
    unsafe impl Send for Rxd {}
    unsafe impl Sync for Rxd {}
    impl Rxd {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Data pointer"]
        #[inline(always)]
        pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in receive buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::RxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::RxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "EasyDMA list type"]
        #[inline(always)]
        pub const fn list(self) -> crate::common::Reg<regs::RxdList, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
    }
    #[doc = "Serial Peripheral Interface Master with EasyDMA"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Spim {
        ptr: *mut u8,
    }
    unsafe impl Send for Spim {}
    unsafe impl Sync for Spim {}
    impl Spim {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start SPI transaction"]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
        }
        #[doc = "Stop SPI transaction"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
        }
        #[doc = "Suspend SPI transaction"]
        #[inline(always)]
        pub const fn tasks_suspend(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
        }
        #[doc = "Resume SPI transaction"]
        #[inline(always)]
        pub const fn tasks_resume(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
        }
        #[doc = "Subscribe configuration for task START"]
        #[inline(always)]
        pub const fn subscribe_start(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
        }
        #[doc = "Subscribe configuration for task STOP"]
        #[inline(always)]
        pub const fn subscribe_stop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
        }
        #[doc = "Subscribe configuration for task SUSPEND"]
        #[inline(always)]
        pub const fn subscribe_suspend(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
        }
        #[doc = "Subscribe configuration for task RESUME"]
        #[inline(always)]
        pub const fn subscribe_resume(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
        }
        #[doc = "SPI transaction has stopped"]
        #[inline(always)]
        pub const fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
        }
        #[doc = "End of RXD buffer reached"]
        #[inline(always)]
        pub const fn events_endrx(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
        }
        #[doc = "End of RXD buffer and TXD buffer reached"]
        #[inline(always)]
        pub const fn events_end(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
        }
        #[doc = "End of TXD buffer reached"]
        #[inline(always)]
        pub const fn events_endtx(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
        }
        #[doc = "Transaction started"]
        #[inline(always)]
        pub const fn events_started(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
        }
        #[doc = "Publish configuration for event STOPPED"]
        #[inline(always)]
        pub const fn publish_stopped(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
        }
        #[doc = "Publish configuration for event ENDRX"]
        #[inline(always)]
        pub const fn publish_endrx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
        }
        #[doc = "Publish configuration for event END"]
        #[inline(always)]
        pub const fn publish_end(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
        }
        #[doc = "Publish configuration for event ENDTX"]
        #[inline(always)]
        pub const fn publish_endtx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
        }
        #[doc = "Publish configuration for event STARTED"]
        #[inline(always)]
        pub const fn publish_started(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
        }
        #[doc = "Stall status for EasyDMA RAM accesses. The fields in this register are set to STALL by hardware whenever a stall occurres and can be cleared (set to NOSTALL) by the CPU."]
        #[inline(always)]
        pub const fn stallstat(self) -> crate::common::Reg<regs::Stallstat, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
        }
        #[doc = "Enable SPIM"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn psel(self) -> Psel {
            unsafe { Psel::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
        }
        #[doc = "SPI frequency. Accuracy depends on the HFCLK source selected."]
        #[inline(always)]
        pub const fn frequency(self) -> crate::common::Reg<regs::Frequency, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0524usize) as _) }
        }
        #[doc = "RXD EasyDMA channel"]
        #[inline(always)]
        pub const fn rxd(self) -> Rxd {
            unsafe { Rxd::from_ptr(self.ptr.wrapping_add(0x0534usize) as _) }
        }
        #[doc = "TXD EasyDMA channel"]
        #[inline(always)]
        pub const fn txd(self) -> Txd {
            unsafe { Txd::from_ptr(self.ptr.wrapping_add(0x0544usize) as _) }
        }
        #[doc = "Configuration register"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0554usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn iftiming(self) -> Iftiming {
            unsafe { Iftiming::from_ptr(self.ptr.wrapping_add(0x0560usize) as _) }
        }
        #[doc = "Polarity of CSN output"]
        #[inline(always)]
        pub const fn csnpol(self) -> crate::common::Reg<regs::Csnpol, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0568usize) as _) }
        }
        #[doc = "Pin select for DCX signal"]
        #[inline(always)]
        pub const fn pseldcx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x056cusize) as _) }
        }
        #[doc = "DCX configuration"]
        #[inline(always)]
        pub const fn dcxcnt(self) -> crate::common::Reg<regs::Dcxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0570usize) as _) }
        }
        #[doc = "Byte transmitted after TXD.MAXCNT bytes have been transmitted in the case when RXD.MAXCNT is greater than TXD.MAXCNT"]
        #[inline(always)]
        pub const fn orc(self) -> crate::common::Reg<regs::Orc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
        }
    }
    #[doc = "TXD EasyDMA channel"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txd {
        ptr: *mut u8,
    }
    unsafe impl Send for Txd {}
    unsafe impl Sync for Txd {}
    impl Txd {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Data pointer"]
        #[inline(always)]
        pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Number of bytes in transmit buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::TxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::TxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "EasyDMA list type"]
        #[inline(always)]
        pub const fn list(self) -> crate::common::Reg<regs::TxdList, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Bit order"]
            #[must_use]
            #[inline(always)]
            pub const fn order(&self) -> super::vals::Order {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Order::from_bits(val as u8)
            }
            #[doc = "Bit order"]
            #[inline(always)]
            pub const fn set_order(&mut self, val: super::vals::Order) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Serial clock (SCK) phase"]
            #[must_use]
            #[inline(always)]
            pub const fn cpha(&self) -> super::vals::Cpha {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Cpha::from_bits(val as u8)
            }
            #[doc = "Serial clock (SCK) phase"]
            #[inline(always)]
            pub const fn set_cpha(&mut self, val: super::vals::Cpha) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Serial clock (SCK) polarity"]
            #[must_use]
            #[inline(always)]
            pub const fn cpol(&self) -> super::vals::Cpol {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Cpol::from_bits(val as u8)
            }
            #[doc = "Serial clock (SCK) polarity"]
            #[inline(always)]
            pub const fn set_cpol(&mut self, val: super::vals::Cpol) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        impl core::fmt::Debug for Config {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Config")
                    .field("order", &self.order())
                    .field("cpha", &self.cpha())
                    .field("cpol", &self.cpol())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Config {{ order: {:?}, cpha: {:?}, cpol: {:?} }}",
                    self.order(),
                    self.cpha(),
                    self.cpol()
                )
            }
        }
        #[doc = "Minimum duration between edge of CSN and edge of SCK. When SHORTS.END_START is used, this is also the minimum duration CSN must stay high between transactions."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Csndur(pub u32);
        impl Csndur {
            #[doc = "Minimum duration between edge of CSN and edge of SCK. When SHORTS.END_START is used, this is the minimum duration CSN must stay high between transactions. The value is specified in number of 64 MHz clock cycles (15.625 ns)."]
            #[must_use]
            #[inline(always)]
            pub const fn csndur(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Minimum duration between edge of CSN and edge of SCK. When SHORTS.END_START is used, this is the minimum duration CSN must stay high between transactions. The value is specified in number of 64 MHz clock cycles (15.625 ns)."]
            #[inline(always)]
            pub const fn set_csndur(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Csndur {
            #[inline(always)]
            fn default() -> Csndur {
                Csndur(0)
            }
        }
        impl core::fmt::Debug for Csndur {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Csndur")
                    .field("csndur", &self.csndur())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Csndur {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Csndur {{ csndur: {=u8:?} }}", self.csndur())
            }
        }
        #[doc = "Polarity of CSN output"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Csnpol(pub u32);
        impl Csnpol {
            #[doc = "Polarity of CSN output"]
            #[must_use]
            #[inline(always)]
            pub const fn csnpol(&self) -> super::vals::Csnpol {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Csnpol::from_bits(val as u8)
            }
            #[doc = "Polarity of CSN output"]
            #[inline(always)]
            pub const fn set_csnpol(&mut self, val: super::vals::Csnpol) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Csnpol {
            #[inline(always)]
            fn default() -> Csnpol {
                Csnpol(0)
            }
        }
        impl core::fmt::Debug for Csnpol {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Csnpol")
                    .field("csnpol", &self.csnpol())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Csnpol {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Csnpol {{ csnpol: {:?} }}", self.csnpol())
            }
        }
        #[doc = "DCX configuration"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dcxcnt(pub u32);
        impl Dcxcnt {
            #[doc = "This register specifies the number of command bytes preceding the data bytes. The PSEL.DCX line will be low during transmission of command bytes and high during transmission of data bytes. Value 0xF indicates that all bytes are command bytes."]
            #[must_use]
            #[inline(always)]
            pub const fn dcxcnt(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "This register specifies the number of command bytes preceding the data bytes. The PSEL.DCX line will be low during transmission of command bytes and high during transmission of data bytes. Value 0xF indicates that all bytes are command bytes."]
            #[inline(always)]
            pub const fn set_dcxcnt(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Dcxcnt {
            #[inline(always)]
            fn default() -> Dcxcnt {
                Dcxcnt(0)
            }
        }
        impl core::fmt::Debug for Dcxcnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dcxcnt")
                    .field("dcxcnt", &self.dcxcnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dcxcnt {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Dcxcnt {{ dcxcnt: {=u8:?} }}", self.dcxcnt())
            }
        }
        #[doc = "Enable SPIM"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable SPIM"]
            #[must_use]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable SPIM"]
            #[inline(always)]
            pub const fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        impl core::fmt::Debug for Enable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Enable")
                    .field("enable", &self.enable())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Enable {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Enable {{ enable: {:?} }}", self.enable())
            }
        }
        #[doc = "SPI frequency. Accuracy depends on the HFCLK source selected."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Frequency(pub u32);
        impl Frequency {
            #[doc = "SPI master data rate"]
            #[must_use]
            #[inline(always)]
            pub const fn frequency(&self) -> super::vals::Frequency {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Frequency::from_bits(val as u32)
            }
            #[doc = "SPI master data rate"]
            #[inline(always)]
            pub const fn set_frequency(&mut self, val: super::vals::Frequency) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Frequency {
            #[inline(always)]
            fn default() -> Frequency {
                Frequency(0)
            }
        }
        impl core::fmt::Debug for Frequency {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Frequency")
                    .field("frequency", &self.frequency())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Frequency {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Frequency {{ frequency: {:?} }}", self.frequency())
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event STOPPED"]
            #[must_use]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event STOPPED"]
            #[inline(always)]
            pub const fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to disable interrupt for event ENDRX"]
            #[must_use]
            #[inline(always)]
            pub const fn endrx(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ENDRX"]
            #[inline(always)]
            pub const fn set_endrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Write '1' to disable interrupt for event END"]
            #[must_use]
            #[inline(always)]
            pub const fn end(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event END"]
            #[inline(always)]
            pub const fn set_end(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Write '1' to disable interrupt for event ENDTX"]
            #[must_use]
            #[inline(always)]
            pub const fn endtx(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ENDTX"]
            #[inline(always)]
            pub const fn set_endtx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Write '1' to disable interrupt for event STARTED"]
            #[must_use]
            #[inline(always)]
            pub const fn started(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event STARTED"]
            #[inline(always)]
            pub const fn set_started(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("stopped", &self.stopped())
                    .field("endrx", &self.endrx())
                    .field("end", &self.end())
                    .field("endtx", &self.endtx())
                    .field("started", &self.started())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Int {{ stopped: {=bool:?}, endrx: {=bool:?}, end: {=bool:?}, endtx: {=bool:?}, started: {=bool:?} }}" , self . stopped () , self . endrx () , self . end () , self . endtx () , self . started ())
            }
        }
        #[doc = "Byte transmitted after TXD.MAXCNT bytes have been transmitted in the case when RXD.MAXCNT is greater than TXD.MAXCNT"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Orc(pub u32);
        impl Orc {
            #[doc = "Byte transmitted after TXD.MAXCNT bytes have been transmitted in the case when RXD.MAXCNT is greater than TXD.MAXCNT."]
            #[must_use]
            #[inline(always)]
            pub const fn orc(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Byte transmitted after TXD.MAXCNT bytes have been transmitted in the case when RXD.MAXCNT is greater than TXD.MAXCNT."]
            #[inline(always)]
            pub const fn set_orc(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Orc {
            #[inline(always)]
            fn default() -> Orc {
                Orc(0)
            }
        }
        impl core::fmt::Debug for Orc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Orc").field("orc", &self.orc()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Orc {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Orc {{ orc: {=u8:?} }}", self.orc())
            }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdAmount(pub u32);
        impl RxdAmount {
            #[doc = "Number of bytes transferred in the last transaction"]
            #[must_use]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last transaction"]
            #[inline(always)]
            pub const fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for RxdAmount {
            #[inline(always)]
            fn default() -> RxdAmount {
                RxdAmount(0)
            }
        }
        impl core::fmt::Debug for RxdAmount {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxdAmount")
                    .field("amount", &self.amount())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxdAmount {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "RxdAmount {{ amount: {=u16:?} }}", self.amount())
            }
        }
        #[doc = "EasyDMA list type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdList(pub u32);
        impl RxdList {
            #[doc = "List type"]
            #[must_use]
            #[inline(always)]
            pub const fn list(&self) -> super::vals::RxdListList {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::RxdListList::from_bits(val as u8)
            }
            #[doc = "List type"]
            #[inline(always)]
            pub const fn set_list(&mut self, val: super::vals::RxdListList) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for RxdList {
            #[inline(always)]
            fn default() -> RxdList {
                RxdList(0)
            }
        }
        impl core::fmt::Debug for RxdList {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxdList")
                    .field("list", &self.list())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxdList {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "RxdList {{ list: {:?} }}", self.list())
            }
        }
        #[doc = "Maximum number of bytes in receive buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdMaxcnt(pub u32);
        impl RxdMaxcnt {
            #[doc = "Maximum number of bytes in receive buffer"]
            #[must_use]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Maximum number of bytes in receive buffer"]
            #[inline(always)]
            pub const fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for RxdMaxcnt {
            #[inline(always)]
            fn default() -> RxdMaxcnt {
                RxdMaxcnt(0)
            }
        }
        impl core::fmt::Debug for RxdMaxcnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxdMaxcnt")
                    .field("maxcnt", &self.maxcnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxdMaxcnt {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "RxdMaxcnt {{ maxcnt: {=u16:?} }}", self.maxcnt())
            }
        }
        #[doc = "Sample delay for input serial data on MISO"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxdelay(pub u32);
        impl Rxdelay {
            #[doc = "Sample delay for input serial data on MISO. The value specifies the number of 64 MHz clock cycles (15.625 ns) delay from the the sampling edge of SCK (leading edge for CONFIG.CPHA = 0, trailing edge for CONFIG.CPHA = 1) until the input serial data is sampled. As en example, if RXDELAY = 0 and CONFIG.CPHA = 0, the input serial data is sampled on the rising edge of SCK."]
            #[must_use]
            #[inline(always)]
            pub const fn rxdelay(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x07;
                val as u8
            }
            #[doc = "Sample delay for input serial data on MISO. The value specifies the number of 64 MHz clock cycles (15.625 ns) delay from the the sampling edge of SCK (leading edge for CONFIG.CPHA = 0, trailing edge for CONFIG.CPHA = 1) until the input serial data is sampled. As en example, if RXDELAY = 0 and CONFIG.CPHA = 0, the input serial data is sampled on the rising edge of SCK."]
            #[inline(always)]
            pub const fn set_rxdelay(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
            }
        }
        impl Default for Rxdelay {
            #[inline(always)]
            fn default() -> Rxdelay {
                Rxdelay(0)
            }
        }
        impl core::fmt::Debug for Rxdelay {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rxdelay")
                    .field("rxdelay", &self.rxdelay())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rxdelay {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Rxdelay {{ rxdelay: {=u8:?} }}", self.rxdelay())
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event END and task START"]
            #[must_use]
            #[inline(always)]
            pub const fn end_start(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event END and task START"]
            #[inline(always)]
            pub const fn set_end_start(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        impl core::fmt::Debug for Shorts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Shorts")
                    .field("end_start", &self.end_start())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Shorts {{ end_start: {=bool:?} }}", self.end_start())
            }
        }
        #[doc = "Stall status for EasyDMA RAM accesses. The fields in this register are set to STALL by hardware whenever a stall occurres and can be cleared (set to NOSTALL) by the CPU."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Stallstat(pub u32);
        impl Stallstat {
            #[doc = "Stall status for EasyDMA RAM reads"]
            #[must_use]
            #[inline(always)]
            pub const fn tx(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Stall status for EasyDMA RAM reads"]
            #[inline(always)]
            pub const fn set_tx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Stall status for EasyDMA RAM writes"]
            #[must_use]
            #[inline(always)]
            pub const fn rx(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Stall status for EasyDMA RAM writes"]
            #[inline(always)]
            pub const fn set_rx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
        }
        impl Default for Stallstat {
            #[inline(always)]
            fn default() -> Stallstat {
                Stallstat(0)
            }
        }
        impl core::fmt::Debug for Stallstat {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Stallstat")
                    .field("tx", &self.tx())
                    .field("rx", &self.rx())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Stallstat {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Stallstat {{ tx: {=bool:?}, rx: {=bool:?} }}",
                    self.tx(),
                    self.rx()
                )
            }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdAmount(pub u32);
        impl TxdAmount {
            #[doc = "Number of bytes transferred in the last transaction"]
            #[must_use]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last transaction"]
            #[inline(always)]
            pub const fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for TxdAmount {
            #[inline(always)]
            fn default() -> TxdAmount {
                TxdAmount(0)
            }
        }
        impl core::fmt::Debug for TxdAmount {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxdAmount")
                    .field("amount", &self.amount())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxdAmount {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "TxdAmount {{ amount: {=u16:?} }}", self.amount())
            }
        }
        #[doc = "EasyDMA list type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdList(pub u32);
        impl TxdList {
            #[doc = "List type"]
            #[must_use]
            #[inline(always)]
            pub const fn list(&self) -> super::vals::TxdListList {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::TxdListList::from_bits(val as u8)
            }
            #[doc = "List type"]
            #[inline(always)]
            pub const fn set_list(&mut self, val: super::vals::TxdListList) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for TxdList {
            #[inline(always)]
            fn default() -> TxdList {
                TxdList(0)
            }
        }
        impl core::fmt::Debug for TxdList {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxdList")
                    .field("list", &self.list())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxdList {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "TxdList {{ list: {:?} }}", self.list())
            }
        }
        #[doc = "Number of bytes in transmit buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdMaxcnt(pub u32);
        impl TxdMaxcnt {
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[must_use]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[inline(always)]
            pub const fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for TxdMaxcnt {
            #[inline(always)]
            fn default() -> TxdMaxcnt {
                TxdMaxcnt(0)
            }
        }
        impl core::fmt::Debug for TxdMaxcnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxdMaxcnt")
                    .field("maxcnt", &self.maxcnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxdMaxcnt {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "TxdMaxcnt {{ maxcnt: {=u16:?} }}", self.maxcnt())
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Cpha {
            #[doc = "Sample on leading edge of clock, shift serial data on trailing edge"]
            LEADING = 0x0,
            #[doc = "Sample on trailing edge of clock, shift serial data on leading edge"]
            TRAILING = 0x01,
        }
        impl Cpha {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Cpha {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Cpha {
            #[inline(always)]
            fn from(val: u8) -> Cpha {
                Cpha::from_bits(val)
            }
        }
        impl From<Cpha> for u8 {
            #[inline(always)]
            fn from(val: Cpha) -> u8 {
                Cpha::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Cpol {
            #[doc = "Active high"]
            ACTIVE_HIGH = 0x0,
            #[doc = "Active low"]
            ACTIVE_LOW = 0x01,
        }
        impl Cpol {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Cpol {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Cpol {
            #[inline(always)]
            fn from(val: u8) -> Cpol {
                Cpol::from_bits(val)
            }
        }
        impl From<Cpol> for u8 {
            #[inline(always)]
            fn from(val: Cpol) -> u8 {
                Cpol::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Csnpol {
            #[doc = "Active low (idle state high)"]
            LOW = 0x0,
            #[doc = "Active high (idle state low)"]
            HIGH = 0x01,
        }
        impl Csnpol {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Csnpol {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Csnpol {
            #[inline(always)]
            fn from(val: u8) -> Csnpol {
                Csnpol::from_bits(val)
            }
        }
        impl From<Csnpol> for u8 {
            #[inline(always)]
            fn from(val: Csnpol) -> u8 {
                Csnpol::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Enable {
            #[doc = "Disable SPIM"]
            DISABLED = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            #[doc = "Enable SPIM"]
            ENABLED = 0x07,
            _RESERVED_8 = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Enable {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Enable {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Enable {
            #[inline(always)]
            fn from(val: u8) -> Enable {
                Enable::from_bits(val)
            }
        }
        impl From<Enable> for u8 {
            #[inline(always)]
            fn from(val: Enable) -> u8 {
                Enable::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Frequency(u32);
        impl Frequency {
            #[doc = "125 kbps"]
            pub const K125: Self = Self(0x0200_0000);
            #[doc = "250 kbps"]
            pub const K250: Self = Self(0x0400_0000);
            #[doc = "500 kbps"]
            pub const K500: Self = Self(0x0800_0000);
            #[doc = "16 Mbps"]
            pub const M16: Self = Self(0x0a00_0000);
            #[doc = "1 Mbps"]
            pub const M1: Self = Self(0x1000_0000);
            #[doc = "32 Mbps"]
            pub const M32: Self = Self(0x1400_0000);
            #[doc = "2 Mbps"]
            pub const M2: Self = Self(0x2000_0000);
            #[doc = "4 Mbps"]
            pub const M4: Self = Self(0x4000_0000);
            #[doc = "8 Mbps"]
            pub const M8: Self = Self(0x8000_0000);
        }
        impl Frequency {
            pub const fn from_bits(val: u32) -> Frequency {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Frequency {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0200_0000 => f.write_str("K125"),
                    0x0400_0000 => f.write_str("K250"),
                    0x0800_0000 => f.write_str("K500"),
                    0x0a00_0000 => f.write_str("M16"),
                    0x1000_0000 => f.write_str("M1"),
                    0x1400_0000 => f.write_str("M32"),
                    0x2000_0000 => f.write_str("M2"),
                    0x4000_0000 => f.write_str("M4"),
                    0x8000_0000 => f.write_str("M8"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Frequency {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0200_0000 => defmt::write!(f, "K125"),
                    0x0400_0000 => defmt::write!(f, "K250"),
                    0x0800_0000 => defmt::write!(f, "K500"),
                    0x0a00_0000 => defmt::write!(f, "M16"),
                    0x1000_0000 => defmt::write!(f, "M1"),
                    0x1400_0000 => defmt::write!(f, "M32"),
                    0x2000_0000 => defmt::write!(f, "M2"),
                    0x4000_0000 => defmt::write!(f, "M4"),
                    0x8000_0000 => defmt::write!(f, "M8"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for Frequency {
            #[inline(always)]
            fn from(val: u32) -> Frequency {
                Frequency::from_bits(val)
            }
        }
        impl From<Frequency> for u32 {
            #[inline(always)]
            fn from(val: Frequency) -> u32 {
                Frequency::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Order {
            #[doc = "Most significant bit shifted out first"]
            MSB_FIRST = 0x0,
            #[doc = "Least significant bit shifted out first"]
            LSB_FIRST = 0x01,
        }
        impl Order {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Order {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Order {
            #[inline(always)]
            fn from(val: u8) -> Order {
                Order::from_bits(val)
            }
        }
        impl From<Order> for u8 {
            #[inline(always)]
            fn from(val: Order) -> u8 {
                Order::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum RxdListList {
            #[doc = "Disable EasyDMA list"]
            DISABLED = 0x0,
            #[doc = "Use array list"]
            ARRAY_LIST = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl RxdListList {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> RxdListList {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for RxdListList {
            #[inline(always)]
            fn from(val: u8) -> RxdListList {
                RxdListList::from_bits(val)
            }
        }
        impl From<RxdListList> for u8 {
            #[inline(always)]
            fn from(val: RxdListList) -> u8 {
                RxdListList::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum TxdListList {
            #[doc = "Disable EasyDMA list"]
            DISABLED = 0x0,
            #[doc = "Use array list"]
            ARRAY_LIST = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl TxdListList {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> TxdListList {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for TxdListList {
            #[inline(always)]
            fn from(val: u8) -> TxdListList {
                TxdListList::from_bits(val)
            }
        }
        impl From<TxdListList> for u8 {
            #[inline(always)]
            fn from(val: TxdListList) -> u8 {
                TxdListList::to_bits(val)
            }
        }
    }
}
pub mod spis {
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psel {
        ptr: *mut u8,
    }
    unsafe impl Send for Psel {}
    unsafe impl Sync for Psel {}
    impl Psel {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Pin select for SCK"]
        #[inline(always)]
        pub const fn sck(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Pin select for MISO signal"]
        #[inline(always)]
        pub const fn miso(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Pin select for MOSI signal"]
        #[inline(always)]
        pub const fn mosi(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "Pin select for CSN signal"]
        #[inline(always)]
        pub const fn csn(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxd {
        ptr: *mut u8,
    }
    unsafe impl Send for Rxd {}
    unsafe impl Sync for Rxd {}
    impl Rxd {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "RXD data pointer"]
        #[inline(always)]
        pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in receive buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::RxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Number of bytes received in last granted transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::RxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "EasyDMA list type"]
        #[inline(always)]
        pub const fn list(self) -> crate::common::Reg<regs::RxdList, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
    }
    #[doc = "SPI Slave"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Spis {
        ptr: *mut u8,
    }
    unsafe impl Send for Spis {}
    unsafe impl Sync for Spis {}
    impl Spis {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Acquire SPI semaphore"]
        #[inline(always)]
        pub const fn tasks_acquire(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
        }
        #[doc = "Release SPI semaphore, enabling the SPI slave to acquire it"]
        #[inline(always)]
        pub const fn tasks_release(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
        }
        #[doc = "Subscribe configuration for task ACQUIRE"]
        #[inline(always)]
        pub const fn subscribe_acquire(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
        }
        #[doc = "Subscribe configuration for task RELEASE"]
        #[inline(always)]
        pub const fn subscribe_release(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
        }
        #[doc = "Granted transaction completed"]
        #[inline(always)]
        pub const fn events_end(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
        }
        #[doc = "End of RXD buffer reached"]
        #[inline(always)]
        pub const fn events_endrx(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
        }
        #[doc = "Semaphore acquired"]
        #[inline(always)]
        pub const fn events_acquired(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
        }
        #[doc = "Publish configuration for event END"]
        #[inline(always)]
        pub const fn publish_end(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
        }
        #[doc = "Publish configuration for event ENDRX"]
        #[inline(always)]
        pub const fn publish_endrx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
        }
        #[doc = "Publish configuration for event ACQUIRED"]
        #[inline(always)]
        pub const fn publish_acquired(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
        }
        #[doc = "Semaphore status register"]
        #[inline(always)]
        pub const fn semstat(self) -> crate::common::Reg<regs::Semstat, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
        }
        #[doc = "Status from last transaction"]
        #[inline(always)]
        pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize) as _) }
        }
        #[doc = "Enable SPI slave"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn psel(self) -> Psel {
            unsafe { Psel::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn rxd(self) -> Rxd {
            unsafe { Rxd::from_ptr(self.ptr.wrapping_add(0x0534usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn txd(self) -> Txd {
            unsafe { Txd::from_ptr(self.ptr.wrapping_add(0x0544usize) as _) }
        }
        #[doc = "Configuration register"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0554usize) as _) }
        }
        #[doc = "Default character. Character clocked out in case of an ignored transaction."]
        #[inline(always)]
        pub const fn def(self) -> crate::common::Reg<regs::Def, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x055cusize) as _) }
        }
        #[doc = "Over-read character"]
        #[inline(always)]
        pub const fn orc(self) -> crate::common::Reg<regs::Orc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txd {
        ptr: *mut u8,
    }
    unsafe impl Send for Txd {}
    unsafe impl Sync for Txd {}
    impl Txd {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "TXD data pointer"]
        #[inline(always)]
        pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in transmit buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::TxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Number of bytes transmitted in last granted transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::TxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "EasyDMA list type"]
        #[inline(always)]
        pub const fn list(self) -> crate::common::Reg<regs::TxdList, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Bit order"]
            #[must_use]
            #[inline(always)]
            pub const fn order(&self) -> super::vals::Order {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Order::from_bits(val as u8)
            }
            #[doc = "Bit order"]
            #[inline(always)]
            pub const fn set_order(&mut self, val: super::vals::Order) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Serial clock (SCK) phase"]
            #[must_use]
            #[inline(always)]
            pub const fn cpha(&self) -> super::vals::Cpha {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Cpha::from_bits(val as u8)
            }
            #[doc = "Serial clock (SCK) phase"]
            #[inline(always)]
            pub const fn set_cpha(&mut self, val: super::vals::Cpha) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Serial clock (SCK) polarity"]
            #[must_use]
            #[inline(always)]
            pub const fn cpol(&self) -> super::vals::Cpol {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Cpol::from_bits(val as u8)
            }
            #[doc = "Serial clock (SCK) polarity"]
            #[inline(always)]
            pub const fn set_cpol(&mut self, val: super::vals::Cpol) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        impl core::fmt::Debug for Config {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Config")
                    .field("order", &self.order())
                    .field("cpha", &self.cpha())
                    .field("cpol", &self.cpol())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Config {{ order: {:?}, cpha: {:?}, cpol: {:?} }}",
                    self.order(),
                    self.cpha(),
                    self.cpol()
                )
            }
        }
        #[doc = "Default character. Character clocked out in case of an ignored transaction."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Def(pub u32);
        impl Def {
            #[doc = "Default character. Character clocked out in case of an ignored transaction."]
            #[must_use]
            #[inline(always)]
            pub const fn def(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Default character. Character clocked out in case of an ignored transaction."]
            #[inline(always)]
            pub const fn set_def(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Def {
            #[inline(always)]
            fn default() -> Def {
                Def(0)
            }
        }
        impl core::fmt::Debug for Def {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Def").field("def", &self.def()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Def {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Def {{ def: {=u8:?} }}", self.def())
            }
        }
        #[doc = "Enable SPI slave"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable SPI slave"]
            #[must_use]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable SPI slave"]
            #[inline(always)]
            pub const fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        impl core::fmt::Debug for Enable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Enable")
                    .field("enable", &self.enable())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Enable {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Enable {{ enable: {:?} }}", self.enable())
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event END"]
            #[must_use]
            #[inline(always)]
            pub const fn end(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event END"]
            #[inline(always)]
            pub const fn set_end(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to disable interrupt for event ENDRX"]
            #[must_use]
            #[inline(always)]
            pub const fn endrx(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ENDRX"]
            #[inline(always)]
            pub const fn set_endrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Write '1' to disable interrupt for event ACQUIRED"]
            #[must_use]
            #[inline(always)]
            pub const fn acquired(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ACQUIRED"]
            #[inline(always)]
            pub const fn set_acquired(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("end", &self.end())
                    .field("endrx", &self.endrx())
                    .field("acquired", &self.acquired())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Int {{ end: {=bool:?}, endrx: {=bool:?}, acquired: {=bool:?} }}",
                    self.end(),
                    self.endrx(),
                    self.acquired()
                )
            }
        }
        #[doc = "Over-read character"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Orc(pub u32);
        impl Orc {
            #[doc = "Over-read character. Character clocked out after an over-read of the transmit buffer."]
            #[must_use]
            #[inline(always)]
            pub const fn orc(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Over-read character. Character clocked out after an over-read of the transmit buffer."]
            #[inline(always)]
            pub const fn set_orc(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Orc {
            #[inline(always)]
            fn default() -> Orc {
                Orc(0)
            }
        }
        impl core::fmt::Debug for Orc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Orc").field("orc", &self.orc()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Orc {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Orc {{ orc: {=u8:?} }}", self.orc())
            }
        }
        #[doc = "Number of bytes received in last granted transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdAmount(pub u32);
        impl RxdAmount {
            #[doc = "Number of bytes received in the last granted transaction"]
            #[must_use]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Number of bytes received in the last granted transaction"]
            #[inline(always)]
            pub const fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for RxdAmount {
            #[inline(always)]
            fn default() -> RxdAmount {
                RxdAmount(0)
            }
        }
        impl core::fmt::Debug for RxdAmount {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxdAmount")
                    .field("amount", &self.amount())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxdAmount {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "RxdAmount {{ amount: {=u16:?} }}", self.amount())
            }
        }
        #[doc = "EasyDMA list type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdList(pub u32);
        impl RxdList {
            #[doc = "List type"]
            #[must_use]
            #[inline(always)]
            pub const fn list(&self) -> super::vals::RxdListList {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::RxdListList::from_bits(val as u8)
            }
            #[doc = "List type"]
            #[inline(always)]
            pub const fn set_list(&mut self, val: super::vals::RxdListList) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for RxdList {
            #[inline(always)]
            fn default() -> RxdList {
                RxdList(0)
            }
        }
        impl core::fmt::Debug for RxdList {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxdList")
                    .field("list", &self.list())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxdList {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "RxdList {{ list: {:?} }}", self.list())
            }
        }
        #[doc = "Maximum number of bytes in receive buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdMaxcnt(pub u32);
        impl RxdMaxcnt {
            #[doc = "Maximum number of bytes in receive buffer"]
            #[must_use]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Maximum number of bytes in receive buffer"]
            #[inline(always)]
            pub const fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for RxdMaxcnt {
            #[inline(always)]
            fn default() -> RxdMaxcnt {
                RxdMaxcnt(0)
            }
        }
        impl core::fmt::Debug for RxdMaxcnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxdMaxcnt")
                    .field("maxcnt", &self.maxcnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxdMaxcnt {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "RxdMaxcnt {{ maxcnt: {=u16:?} }}", self.maxcnt())
            }
        }
        #[doc = "Semaphore status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Semstat(pub u32);
        impl Semstat {
            #[doc = "Semaphore status"]
            #[must_use]
            #[inline(always)]
            pub const fn semstat(&self) -> super::vals::Semstat {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Semstat::from_bits(val as u8)
            }
            #[doc = "Semaphore status"]
            #[inline(always)]
            pub const fn set_semstat(&mut self, val: super::vals::Semstat) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Semstat {
            #[inline(always)]
            fn default() -> Semstat {
                Semstat(0)
            }
        }
        impl core::fmt::Debug for Semstat {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Semstat")
                    .field("semstat", &self.semstat())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Semstat {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Semstat {{ semstat: {:?} }}", self.semstat())
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event END and task ACQUIRE"]
            #[must_use]
            #[inline(always)]
            pub const fn end_acquire(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event END and task ACQUIRE"]
            #[inline(always)]
            pub const fn set_end_acquire(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        impl core::fmt::Debug for Shorts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Shorts")
                    .field("end_acquire", &self.end_acquire())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Shorts {{ end_acquire: {=bool:?} }}", self.end_acquire())
            }
        }
        #[doc = "Status from last transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Status(pub u32);
        impl Status {
            #[doc = "TX buffer over-read detected, and prevented"]
            #[must_use]
            #[inline(always)]
            pub const fn overread(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "TX buffer over-read detected, and prevented"]
            #[inline(always)]
            pub const fn set_overread(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "RX buffer overflow detected, and prevented"]
            #[must_use]
            #[inline(always)]
            pub const fn overflow(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "RX buffer overflow detected, and prevented"]
            #[inline(always)]
            pub const fn set_overflow(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
        }
        impl Default for Status {
            #[inline(always)]
            fn default() -> Status {
                Status(0)
            }
        }
        impl core::fmt::Debug for Status {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Status")
                    .field("overread", &self.overread())
                    .field("overflow", &self.overflow())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Status {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Status {{ overread: {=bool:?}, overflow: {=bool:?} }}",
                    self.overread(),
                    self.overflow()
                )
            }
        }
        #[doc = "Number of bytes transmitted in last granted transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdAmount(pub u32);
        impl TxdAmount {
            #[doc = "Number of bytes transmitted in last granted transaction"]
            #[must_use]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Number of bytes transmitted in last granted transaction"]
            #[inline(always)]
            pub const fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for TxdAmount {
            #[inline(always)]
            fn default() -> TxdAmount {
                TxdAmount(0)
            }
        }
        impl core::fmt::Debug for TxdAmount {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxdAmount")
                    .field("amount", &self.amount())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxdAmount {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "TxdAmount {{ amount: {=u16:?} }}", self.amount())
            }
        }
        #[doc = "EasyDMA list type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdList(pub u32);
        impl TxdList {
            #[doc = "List type"]
            #[must_use]
            #[inline(always)]
            pub const fn list(&self) -> super::vals::TxdListList {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::TxdListList::from_bits(val as u8)
            }
            #[doc = "List type"]
            #[inline(always)]
            pub const fn set_list(&mut self, val: super::vals::TxdListList) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for TxdList {
            #[inline(always)]
            fn default() -> TxdList {
                TxdList(0)
            }
        }
        impl core::fmt::Debug for TxdList {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxdList")
                    .field("list", &self.list())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxdList {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "TxdList {{ list: {:?} }}", self.list())
            }
        }
        #[doc = "Maximum number of bytes in transmit buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdMaxcnt(pub u32);
        impl TxdMaxcnt {
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[must_use]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[inline(always)]
            pub const fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for TxdMaxcnt {
            #[inline(always)]
            fn default() -> TxdMaxcnt {
                TxdMaxcnt(0)
            }
        }
        impl core::fmt::Debug for TxdMaxcnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxdMaxcnt")
                    .field("maxcnt", &self.maxcnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxdMaxcnt {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "TxdMaxcnt {{ maxcnt: {=u16:?} }}", self.maxcnt())
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Cpha {
            #[doc = "Sample on leading edge of clock, shift serial data on trailing edge"]
            LEADING = 0x0,
            #[doc = "Sample on trailing edge of clock, shift serial data on leading edge"]
            TRAILING = 0x01,
        }
        impl Cpha {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Cpha {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Cpha {
            #[inline(always)]
            fn from(val: u8) -> Cpha {
                Cpha::from_bits(val)
            }
        }
        impl From<Cpha> for u8 {
            #[inline(always)]
            fn from(val: Cpha) -> u8 {
                Cpha::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Cpol {
            #[doc = "Active high"]
            ACTIVE_HIGH = 0x0,
            #[doc = "Active low"]
            ACTIVE_LOW = 0x01,
        }
        impl Cpol {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Cpol {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Cpol {
            #[inline(always)]
            fn from(val: u8) -> Cpol {
                Cpol::from_bits(val)
            }
        }
        impl From<Cpol> for u8 {
            #[inline(always)]
            fn from(val: Cpol) -> u8 {
                Cpol::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Enable {
            #[doc = "Disable SPI slave"]
            DISABLED = 0x0,
            _RESERVED_1 = 0x01,
            #[doc = "Enable SPI slave"]
            ENABLED = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
            _RESERVED_8 = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Enable {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Enable {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Enable {
            #[inline(always)]
            fn from(val: u8) -> Enable {
                Enable::from_bits(val)
            }
        }
        impl From<Enable> for u8 {
            #[inline(always)]
            fn from(val: Enable) -> u8 {
                Enable::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Order {
            #[doc = "Most significant bit shifted out first"]
            MSB_FIRST = 0x0,
            #[doc = "Least significant bit shifted out first"]
            LSB_FIRST = 0x01,
        }
        impl Order {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Order {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Order {
            #[inline(always)]
            fn from(val: u8) -> Order {
                Order::from_bits(val)
            }
        }
        impl From<Order> for u8 {
            #[inline(always)]
            fn from(val: Order) -> u8 {
                Order::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum RxdListList {
            #[doc = "Disable EasyDMA list"]
            DISABLED = 0x0,
            #[doc = "Use array list"]
            ARRAY_LIST = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl RxdListList {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> RxdListList {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for RxdListList {
            #[inline(always)]
            fn from(val: u8) -> RxdListList {
                RxdListList::from_bits(val)
            }
        }
        impl From<RxdListList> for u8 {
            #[inline(always)]
            fn from(val: RxdListList) -> u8 {
                RxdListList::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Semstat {
            #[doc = "Semaphore is free"]
            FREE = 0x0,
            #[doc = "Semaphore is assigned to CPU"]
            CPU = 0x01,
            #[doc = "Semaphore is assigned to SPI slave"]
            SPIS = 0x02,
            #[doc = "Semaphore is assigned to SPI but a handover to the CPU is pending"]
            CPUPENDING = 0x03,
        }
        impl Semstat {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Semstat {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Semstat {
            #[inline(always)]
            fn from(val: u8) -> Semstat {
                Semstat::from_bits(val)
            }
        }
        impl From<Semstat> for u8 {
            #[inline(always)]
            fn from(val: Semstat) -> u8 {
                Semstat::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum TxdListList {
            #[doc = "Disable EasyDMA list"]
            DISABLED = 0x0,
            #[doc = "Use array list"]
            ARRAY_LIST = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl TxdListList {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> TxdListList {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for TxdListList {
            #[inline(always)]
            fn from(val: u8) -> TxdListList {
                TxdListList::from_bits(val)
            }
        }
        impl From<TxdListList> for u8 {
            #[inline(always)]
            fn from(val: TxdListList) -> u8 {
                TxdListList::to_bits(val)
            }
        }
    }
}
pub mod swi {
    #[doc = "Software interrupt 0"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swi {
        ptr: *mut u8,
    }
    unsafe impl Send for Swi {}
    unsafe impl Sync for Swi {}
    impl Swi {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Unused."]
        #[inline(always)]
        pub const fn unused(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
    }
}
pub mod temp {
    #[doc = "Temperature Sensor"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Temp {
        ptr: *mut u8,
    }
    unsafe impl Send for Temp {}
    unsafe impl Sync for Temp {}
    impl Temp {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start temperature measurement"]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Stop temperature measurement"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Subscribe configuration for task START"]
        #[inline(always)]
        pub const fn subscribe_start(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
        }
        #[doc = "Subscribe configuration for task STOP"]
        #[inline(always)]
        pub const fn subscribe_stop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
        }
        #[doc = "Temperature measurement complete, data ready"]
        #[inline(always)]
        pub const fn events_datardy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
        }
        #[doc = "Publish configuration for event DATARDY"]
        #[inline(always)]
        pub const fn publish_datardy(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
        }
        #[doc = "Temperature in degC (0.25deg steps)"]
        #[inline(always)]
        pub const fn temp(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
        }
        #[doc = "Slope of first piecewise linear function"]
        #[inline(always)]
        pub const fn a(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 6usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0520usize + n * 4usize) as _)
            }
        }
        #[doc = "y-intercept of first piecewise linear function"]
        #[inline(always)]
        pub const fn b(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 6usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0540usize + n * 4usize) as _)
            }
        }
        #[doc = "Endpoint of first piecewise linear function"]
        #[inline(always)]
        pub const fn t(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 5usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0560usize + n * 4usize) as _)
            }
        }
    }
    pub mod regs {
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event DATARDY"]
            #[must_use]
            #[inline(always)]
            pub const fn datardy(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event DATARDY"]
            #[inline(always)]
            pub const fn set_datardy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("datardy", &self.datardy())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Int {{ datardy: {=bool:?} }}", self.datardy())
            }
        }
    }
}
pub mod timer {
    #[doc = "Timer/Counter 0"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer {
        ptr: *mut u8,
    }
    unsafe impl Send for Timer {}
    unsafe impl Sync for Timer {}
    impl Timer {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start Timer"]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Stop Timer"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Increment Timer (Counter mode only)"]
        #[inline(always)]
        pub const fn tasks_count(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "Clear time"]
        #[inline(always)]
        pub const fn tasks_clear(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
        #[doc = "Deprecated register - Shut down timer"]
        #[inline(always)]
        pub const fn tasks_shutdown(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
        }
        #[doc = "Description collection: Capture Timer value to CC\\[n\\] register"]
        #[inline(always)]
        pub const fn tasks_capture(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
            assert!(n < 8usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _)
            }
        }
        #[doc = "Subscribe configuration for task START"]
        #[inline(always)]
        pub const fn subscribe_start(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
        }
        #[doc = "Subscribe configuration for task STOP"]
        #[inline(always)]
        pub const fn subscribe_stop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
        }
        #[doc = "Subscribe configuration for task COUNT"]
        #[inline(always)]
        pub const fn subscribe_count(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
        }
        #[doc = "Subscribe configuration for task CLEAR"]
        #[inline(always)]
        pub const fn subscribe_clear(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
        }
        #[doc = "Deprecated register - Subscribe configuration for task SHUTDOWN"]
        #[inline(always)]
        pub const fn subscribe_shutdown(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
        }
        #[doc = "Description collection: Subscribe configuration for task CAPTURE\\[n\\]"]
        #[inline(always)]
        pub const fn subscribe_capture(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            assert!(n < 8usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize + n * 4usize) as _)
            }
        }
        #[doc = "Description collection: Compare event on CC\\[n\\] match"]
        #[inline(always)]
        pub const fn events_compare(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 8usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize + n * 4usize) as _)
            }
        }
        #[doc = "Description collection: Publish configuration for event COMPARE\\[n\\]"]
        #[inline(always)]
        pub const fn publish_compare(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            assert!(n < 8usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize + n * 4usize) as _)
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
        }
        #[doc = "Timer mode selection"]
        #[inline(always)]
        pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
        }
        #[doc = "Configure the number of bits used by the TIMER"]
        #[inline(always)]
        pub const fn bitmode(self) -> crate::common::Reg<regs::Bitmode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
        }
        #[doc = "Timer prescaler register"]
        #[inline(always)]
        pub const fn prescaler(self) -> crate::common::Reg<regs::Prescaler, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0510usize) as _) }
        }
        #[doc = "Description collection: Capture/Compare register n"]
        #[inline(always)]
        pub const fn cc(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 8usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0540usize + n * 4usize) as _)
            }
        }
        #[doc = "Description collection: Enable one-shot operation for Capture/Compare channel n"]
        #[inline(always)]
        pub const fn oneshoten(
            self,
            n: usize,
        ) -> crate::common::Reg<regs::Oneshoten, crate::common::RW> {
            assert!(n < 8usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0580usize + n * 4usize) as _)
            }
        }
    }
    pub mod regs {
        #[doc = "Configure the number of bits used by the TIMER"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Bitmode(pub u32);
        impl Bitmode {
            #[doc = "Timer bit width"]
            #[must_use]
            #[inline(always)]
            pub const fn bitmode(&self) -> super::vals::Bitmode {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Bitmode::from_bits(val as u8)
            }
            #[doc = "Timer bit width"]
            #[inline(always)]
            pub const fn set_bitmode(&mut self, val: super::vals::Bitmode) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Bitmode {
            #[inline(always)]
            fn default() -> Bitmode {
                Bitmode(0)
            }
        }
        impl core::fmt::Debug for Bitmode {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Bitmode")
                    .field("bitmode", &self.bitmode())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Bitmode {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Bitmode {{ bitmode: {:?} }}", self.bitmode())
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Enable or disable interrupt for event COMPARE\\[0\\]"]
            #[must_use]
            #[inline(always)]
            pub const fn compare(&self, n: usize) -> bool {
                assert!(n < 8usize);
                let offs = 16usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event COMPARE\\[0\\]"]
            #[inline(always)]
            pub const fn set_compare(&mut self, n: usize, val: bool) {
                assert!(n < 8usize);
                let offs = 16usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("compare[0]", &self.compare(0usize))
                    .field("compare[1]", &self.compare(1usize))
                    .field("compare[2]", &self.compare(2usize))
                    .field("compare[3]", &self.compare(3usize))
                    .field("compare[4]", &self.compare(4usize))
                    .field("compare[5]", &self.compare(5usize))
                    .field("compare[6]", &self.compare(6usize))
                    .field("compare[7]", &self.compare(7usize))
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Int {{ compare[0]: {=bool:?}, compare[1]: {=bool:?}, compare[2]: {=bool:?}, compare[3]: {=bool:?}, compare[4]: {=bool:?}, compare[5]: {=bool:?}, compare[6]: {=bool:?}, compare[7]: {=bool:?} }}" , self . compare (0usize) , self . compare (1usize) , self . compare (2usize) , self . compare (3usize) , self . compare (4usize) , self . compare (5usize) , self . compare (6usize) , self . compare (7usize))
            }
        }
        #[doc = "Timer mode selection"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Mode(pub u32);
        impl Mode {
            #[doc = "Timer mode"]
            #[must_use]
            #[inline(always)]
            pub const fn mode(&self) -> super::vals::Mode {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Mode::from_bits(val as u8)
            }
            #[doc = "Timer mode"]
            #[inline(always)]
            pub const fn set_mode(&mut self, val: super::vals::Mode) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Mode {
            #[inline(always)]
            fn default() -> Mode {
                Mode(0)
            }
        }
        impl core::fmt::Debug for Mode {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Mode").field("mode", &self.mode()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Mode {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Mode {{ mode: {:?} }}", self.mode())
            }
        }
        #[doc = "Description collection: Enable one-shot operation for Capture/Compare channel n"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Oneshoten(pub u32);
        impl Oneshoten {
            #[doc = "Enable one-shot operation"]
            #[must_use]
            #[inline(always)]
            pub const fn oneshoten(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable one-shot operation"]
            #[inline(always)]
            pub const fn set_oneshoten(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Oneshoten {
            #[inline(always)]
            fn default() -> Oneshoten {
                Oneshoten(0)
            }
        }
        impl core::fmt::Debug for Oneshoten {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Oneshoten")
                    .field("oneshoten", &self.oneshoten())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Oneshoten {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Oneshoten {{ oneshoten: {=bool:?} }}", self.oneshoten())
            }
        }
        #[doc = "Timer prescaler register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Prescaler(pub u32);
        impl Prescaler {
            #[doc = "Prescaler value"]
            #[must_use]
            #[inline(always)]
            pub const fn prescaler(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "Prescaler value"]
            #[inline(always)]
            pub const fn set_prescaler(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Prescaler {
            #[inline(always)]
            fn default() -> Prescaler {
                Prescaler(0)
            }
        }
        impl core::fmt::Debug for Prescaler {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Prescaler")
                    .field("prescaler", &self.prescaler())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Prescaler {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Prescaler {{ prescaler: {=u8:?} }}", self.prescaler())
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event COMPARE\\[0\\] and task CLEAR"]
            #[must_use]
            #[inline(always)]
            pub const fn compare_clear(&self, n: usize) -> bool {
                assert!(n < 8usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event COMPARE\\[0\\] and task CLEAR"]
            #[inline(always)]
            pub const fn set_compare_clear(&mut self, n: usize, val: bool) {
                assert!(n < 8usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Shortcut between event COMPARE\\[0\\] and task STOP"]
            #[must_use]
            #[inline(always)]
            pub const fn compare_stop(&self, n: usize) -> bool {
                assert!(n < 8usize);
                let offs = 16usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event COMPARE\\[0\\] and task STOP"]
            #[inline(always)]
            pub const fn set_compare_stop(&mut self, n: usize, val: bool) {
                assert!(n < 8usize);
                let offs = 16usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        impl core::fmt::Debug for Shorts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Shorts")
                    .field("compare_clear[0]", &self.compare_clear(0usize))
                    .field("compare_clear[1]", &self.compare_clear(1usize))
                    .field("compare_clear[2]", &self.compare_clear(2usize))
                    .field("compare_clear[3]", &self.compare_clear(3usize))
                    .field("compare_clear[4]", &self.compare_clear(4usize))
                    .field("compare_clear[5]", &self.compare_clear(5usize))
                    .field("compare_clear[6]", &self.compare_clear(6usize))
                    .field("compare_clear[7]", &self.compare_clear(7usize))
                    .field("compare_stop[0]", &self.compare_stop(0usize))
                    .field("compare_stop[1]", &self.compare_stop(1usize))
                    .field("compare_stop[2]", &self.compare_stop(2usize))
                    .field("compare_stop[3]", &self.compare_stop(3usize))
                    .field("compare_stop[4]", &self.compare_stop(4usize))
                    .field("compare_stop[5]", &self.compare_stop(5usize))
                    .field("compare_stop[6]", &self.compare_stop(6usize))
                    .field("compare_stop[7]", &self.compare_stop(7usize))
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Shorts {{ compare_clear[0]: {=bool:?}, compare_clear[1]: {=bool:?}, compare_clear[2]: {=bool:?}, compare_clear[3]: {=bool:?}, compare_clear[4]: {=bool:?}, compare_clear[5]: {=bool:?}, compare_clear[6]: {=bool:?}, compare_clear[7]: {=bool:?}, compare_stop[0]: {=bool:?}, compare_stop[1]: {=bool:?}, compare_stop[2]: {=bool:?}, compare_stop[3]: {=bool:?}, compare_stop[4]: {=bool:?}, compare_stop[5]: {=bool:?}, compare_stop[6]: {=bool:?}, compare_stop[7]: {=bool:?} }}" , self . compare_clear (0usize) , self . compare_clear (1usize) , self . compare_clear (2usize) , self . compare_clear (3usize) , self . compare_clear (4usize) , self . compare_clear (5usize) , self . compare_clear (6usize) , self . compare_clear (7usize) , self . compare_stop (0usize) , self . compare_stop (1usize) , self . compare_stop (2usize) , self . compare_stop (3usize) , self . compare_stop (4usize) , self . compare_stop (5usize) , self . compare_stop (6usize) , self . compare_stop (7usize))
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Bitmode {
            #[doc = "16 bit timer bit width"]
            _16BIT = 0x0,
            #[doc = "8 bit timer bit width"]
            _08BIT = 0x01,
            #[doc = "24 bit timer bit width"]
            _24BIT = 0x02,
            #[doc = "32 bit timer bit width"]
            _32BIT = 0x03,
        }
        impl Bitmode {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Bitmode {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Bitmode {
            #[inline(always)]
            fn from(val: u8) -> Bitmode {
                Bitmode::from_bits(val)
            }
        }
        impl From<Bitmode> for u8 {
            #[inline(always)]
            fn from(val: Bitmode) -> u8 {
                Bitmode::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Mode {
            #[doc = "Select Timer mode"]
            TIMER = 0x0,
            #[doc = "Deprecated enumerator - Select Counter mode"]
            COUNTER = 0x01,
            #[doc = "Select Low Power Counter mode"]
            LOW_POWER_COUNTER = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Mode {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Mode {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Mode {
            #[inline(always)]
            fn from(val: u8) -> Mode {
                Mode::from_bits(val)
            }
        }
        impl From<Mode> for u8 {
            #[inline(always)]
            fn from(val: Mode) -> u8 {
                Mode::to_bits(val)
            }
        }
    }
}
pub mod twim {
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psel {
        ptr: *mut u8,
    }
    unsafe impl Send for Psel {}
    unsafe impl Sync for Psel {}
    impl Psel {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Pin select for SCL signal"]
        #[inline(always)]
        pub const fn scl(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Pin select for SDA signal"]
        #[inline(always)]
        pub const fn sda(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
    }
    #[doc = "RXD EasyDMA channel"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxd {
        ptr: *mut u8,
    }
    unsafe impl Send for Rxd {}
    unsafe impl Sync for Rxd {}
    impl Rxd {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Data pointer"]
        #[inline(always)]
        pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in receive buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::RxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::RxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "EasyDMA list type"]
        #[inline(always)]
        pub const fn list(self) -> crate::common::Reg<regs::RxdList, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
    }
    #[doc = "I2C compatible Two-Wire Master Interface with EasyDMA"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Twim {
        ptr: *mut u8,
    }
    unsafe impl Send for Twim {}
    unsafe impl Sync for Twim {}
    impl Twim {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start TWI receive sequence"]
        #[inline(always)]
        pub const fn tasks_startrx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Start TWI transmit sequence"]
        #[inline(always)]
        pub const fn tasks_starttx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "Stop TWI transaction. Must be issued while the TWI master is not suspended."]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
        }
        #[doc = "Suspend TWI transaction"]
        #[inline(always)]
        pub const fn tasks_suspend(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
        }
        #[doc = "Resume TWI transaction"]
        #[inline(always)]
        pub const fn tasks_resume(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
        }
        #[doc = "Subscribe configuration for task STARTRX"]
        #[inline(always)]
        pub const fn subscribe_startrx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
        }
        #[doc = "Subscribe configuration for task STARTTX"]
        #[inline(always)]
        pub const fn subscribe_starttx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
        }
        #[doc = "Subscribe configuration for task STOP"]
        #[inline(always)]
        pub const fn subscribe_stop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
        }
        #[doc = "Subscribe configuration for task SUSPEND"]
        #[inline(always)]
        pub const fn subscribe_suspend(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
        }
        #[doc = "Subscribe configuration for task RESUME"]
        #[inline(always)]
        pub const fn subscribe_resume(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
        }
        #[doc = "TWI stopped"]
        #[inline(always)]
        pub const fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
        }
        #[doc = "TWI error"]
        #[inline(always)]
        pub const fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
        }
        #[doc = "SUSPEND task has been issued, TWI traffic is now suspended."]
        #[inline(always)]
        pub const fn events_suspended(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
        }
        #[doc = "Receive sequence started"]
        #[inline(always)]
        pub const fn events_rxstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
        }
        #[doc = "Transmit sequence started"]
        #[inline(always)]
        pub const fn events_txstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
        }
        #[doc = "Byte boundary, starting to receive the last byte"]
        #[inline(always)]
        pub const fn events_lastrx(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
        }
        #[doc = "Byte boundary, starting to transmit the last byte"]
        #[inline(always)]
        pub const fn events_lasttx(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
        }
        #[doc = "Publish configuration for event STOPPED"]
        #[inline(always)]
        pub const fn publish_stopped(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
        }
        #[doc = "Publish configuration for event ERROR"]
        #[inline(always)]
        pub const fn publish_error(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
        }
        #[doc = "Publish configuration for event SUSPENDED"]
        #[inline(always)]
        pub const fn publish_suspended(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
        }
        #[doc = "Publish configuration for event RXSTARTED"]
        #[inline(always)]
        pub const fn publish_rxstarted(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
        }
        #[doc = "Publish configuration for event TXSTARTED"]
        #[inline(always)]
        pub const fn publish_txstarted(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
        }
        #[doc = "Publish configuration for event LASTRX"]
        #[inline(always)]
        pub const fn publish_lastrx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01dcusize) as _) }
        }
        #[doc = "Publish configuration for event LASTTX"]
        #[inline(always)]
        pub const fn publish_lasttx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
        }
        #[doc = "Error source"]
        #[inline(always)]
        pub const fn errorsrc(self) -> crate::common::Reg<regs::Errorsrc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c4usize) as _) }
        }
        #[doc = "Enable TWIM"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn psel(self) -> Psel {
            unsafe { Psel::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
        }
        #[doc = "TWI frequency. Accuracy depends on the HFCLK source selected."]
        #[inline(always)]
        pub const fn frequency(self) -> crate::common::Reg<regs::Frequency, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0524usize) as _) }
        }
        #[doc = "RXD EasyDMA channel"]
        #[inline(always)]
        pub const fn rxd(self) -> Rxd {
            unsafe { Rxd::from_ptr(self.ptr.wrapping_add(0x0534usize) as _) }
        }
        #[doc = "TXD EasyDMA channel"]
        #[inline(always)]
        pub const fn txd(self) -> Txd {
            unsafe { Txd::from_ptr(self.ptr.wrapping_add(0x0544usize) as _) }
        }
        #[doc = "Address used in the TWI transfer"]
        #[inline(always)]
        pub const fn address(self) -> crate::common::Reg<regs::Address, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0588usize) as _) }
        }
    }
    #[doc = "TXD EasyDMA channel"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txd {
        ptr: *mut u8,
    }
    unsafe impl Send for Txd {}
    unsafe impl Sync for Txd {}
    impl Txd {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Data pointer"]
        #[inline(always)]
        pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in transmit buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::TxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::TxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "EasyDMA list type"]
        #[inline(always)]
        pub const fn list(self) -> crate::common::Reg<regs::TxdList, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Address used in the TWI transfer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Address(pub u32);
        impl Address {
            #[doc = "Address used in the TWI transfer"]
            #[must_use]
            #[inline(always)]
            pub const fn address(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "Address used in the TWI transfer"]
            #[inline(always)]
            pub const fn set_address(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
            }
        }
        impl Default for Address {
            #[inline(always)]
            fn default() -> Address {
                Address(0)
            }
        }
        impl core::fmt::Debug for Address {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Address")
                    .field("address", &self.address())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Address {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Address {{ address: {=u8:?} }}", self.address())
            }
        }
        #[doc = "Enable TWIM"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable TWIM"]
            #[must_use]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable TWIM"]
            #[inline(always)]
            pub const fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        impl core::fmt::Debug for Enable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Enable")
                    .field("enable", &self.enable())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Enable {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Enable {{ enable: {:?} }}", self.enable())
            }
        }
        #[doc = "Error source"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Errorsrc(pub u32);
        impl Errorsrc {
            #[doc = "Overrun error"]
            #[must_use]
            #[inline(always)]
            pub const fn overrun(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun error"]
            #[inline(always)]
            pub const fn set_overrun(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "NACK received after sending the address (write '1' to clear)"]
            #[must_use]
            #[inline(always)]
            pub const fn anack(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "NACK received after sending the address (write '1' to clear)"]
            #[inline(always)]
            pub const fn set_anack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "NACK received after sending a data byte (write '1' to clear)"]
            #[must_use]
            #[inline(always)]
            pub const fn dnack(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "NACK received after sending a data byte (write '1' to clear)"]
            #[inline(always)]
            pub const fn set_dnack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Errorsrc {
            #[inline(always)]
            fn default() -> Errorsrc {
                Errorsrc(0)
            }
        }
        impl core::fmt::Debug for Errorsrc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Errorsrc")
                    .field("overrun", &self.overrun())
                    .field("anack", &self.anack())
                    .field("dnack", &self.dnack())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Errorsrc {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Errorsrc {{ overrun: {=bool:?}, anack: {=bool:?}, dnack: {=bool:?} }}",
                    self.overrun(),
                    self.anack(),
                    self.dnack()
                )
            }
        }
        #[doc = "TWI frequency. Accuracy depends on the HFCLK source selected."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Frequency(pub u32);
        impl Frequency {
            #[doc = "TWI master clock frequency"]
            #[must_use]
            #[inline(always)]
            pub const fn frequency(&self) -> super::vals::Frequency {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Frequency::from_bits(val as u32)
            }
            #[doc = "TWI master clock frequency"]
            #[inline(always)]
            pub const fn set_frequency(&mut self, val: super::vals::Frequency) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Frequency {
            #[inline(always)]
            fn default() -> Frequency {
                Frequency(0)
            }
        }
        impl core::fmt::Debug for Frequency {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Frequency")
                    .field("frequency", &self.frequency())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Frequency {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Frequency {{ frequency: {:?} }}", self.frequency())
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Enable or disable interrupt for event STOPPED"]
            #[must_use]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event STOPPED"]
            #[inline(always)]
            pub const fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for event ERROR"]
            #[must_use]
            #[inline(always)]
            pub const fn error(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event ERROR"]
            #[inline(always)]
            pub const fn set_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Enable or disable interrupt for event SUSPENDED"]
            #[must_use]
            #[inline(always)]
            pub const fn suspended(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event SUSPENDED"]
            #[inline(always)]
            pub const fn set_suspended(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Enable or disable interrupt for event RXSTARTED"]
            #[must_use]
            #[inline(always)]
            pub const fn rxstarted(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RXSTARTED"]
            #[inline(always)]
            pub const fn set_rxstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "Enable or disable interrupt for event TXSTARTED"]
            #[must_use]
            #[inline(always)]
            pub const fn txstarted(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event TXSTARTED"]
            #[inline(always)]
            pub const fn set_txstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "Enable or disable interrupt for event LASTRX"]
            #[must_use]
            #[inline(always)]
            pub const fn lastrx(&self) -> bool {
                let val = (self.0 >> 23usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event LASTRX"]
            #[inline(always)]
            pub const fn set_lastrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
            }
            #[doc = "Enable or disable interrupt for event LASTTX"]
            #[must_use]
            #[inline(always)]
            pub const fn lasttx(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event LASTTX"]
            #[inline(always)]
            pub const fn set_lasttx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("stopped", &self.stopped())
                    .field("error", &self.error())
                    .field("suspended", &self.suspended())
                    .field("rxstarted", &self.rxstarted())
                    .field("txstarted", &self.txstarted())
                    .field("lastrx", &self.lastrx())
                    .field("lasttx", &self.lasttx())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Int {{ stopped: {=bool:?}, error: {=bool:?}, suspended: {=bool:?}, rxstarted: {=bool:?}, txstarted: {=bool:?}, lastrx: {=bool:?}, lasttx: {=bool:?} }}" , self . stopped () , self . error () , self . suspended () , self . rxstarted () , self . txstarted () , self . lastrx () , self . lasttx ())
            }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdAmount(pub u32);
        impl RxdAmount {
            #[doc = "Number of bytes transferred in the last transaction. In case of NACK error, includes the NACK'ed byte."]
            #[must_use]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last transaction. In case of NACK error, includes the NACK'ed byte."]
            #[inline(always)]
            pub const fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for RxdAmount {
            #[inline(always)]
            fn default() -> RxdAmount {
                RxdAmount(0)
            }
        }
        impl core::fmt::Debug for RxdAmount {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxdAmount")
                    .field("amount", &self.amount())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxdAmount {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "RxdAmount {{ amount: {=u16:?} }}", self.amount())
            }
        }
        #[doc = "EasyDMA list type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdList(pub u32);
        impl RxdList {
            #[doc = "List type"]
            #[must_use]
            #[inline(always)]
            pub const fn list(&self) -> super::vals::RxdListList {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::RxdListList::from_bits(val as u8)
            }
            #[doc = "List type"]
            #[inline(always)]
            pub const fn set_list(&mut self, val: super::vals::RxdListList) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
            }
        }
        impl Default for RxdList {
            #[inline(always)]
            fn default() -> RxdList {
                RxdList(0)
            }
        }
        impl core::fmt::Debug for RxdList {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxdList")
                    .field("list", &self.list())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxdList {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "RxdList {{ list: {:?} }}", self.list())
            }
        }
        #[doc = "Maximum number of bytes in receive buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdMaxcnt(pub u32);
        impl RxdMaxcnt {
            #[doc = "Maximum number of bytes in receive buffer"]
            #[must_use]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Maximum number of bytes in receive buffer"]
            #[inline(always)]
            pub const fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for RxdMaxcnt {
            #[inline(always)]
            fn default() -> RxdMaxcnt {
                RxdMaxcnt(0)
            }
        }
        impl core::fmt::Debug for RxdMaxcnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxdMaxcnt")
                    .field("maxcnt", &self.maxcnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxdMaxcnt {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "RxdMaxcnt {{ maxcnt: {=u16:?} }}", self.maxcnt())
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event LASTTX and task STARTRX"]
            #[must_use]
            #[inline(always)]
            pub const fn lasttx_startrx(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event LASTTX and task STARTRX"]
            #[inline(always)]
            pub const fn set_lasttx_startrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Shortcut between event LASTTX and task SUSPEND"]
            #[must_use]
            #[inline(always)]
            pub const fn lasttx_suspend(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event LASTTX and task SUSPEND"]
            #[inline(always)]
            pub const fn set_lasttx_suspend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Shortcut between event LASTTX and task STOP"]
            #[must_use]
            #[inline(always)]
            pub const fn lasttx_stop(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event LASTTX and task STOP"]
            #[inline(always)]
            pub const fn set_lasttx_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Shortcut between event LASTRX and task STARTTX"]
            #[must_use]
            #[inline(always)]
            pub const fn lastrx_starttx(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event LASTRX and task STARTTX"]
            #[inline(always)]
            pub const fn set_lastrx_starttx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Shortcut between event LASTRX and task STOP"]
            #[must_use]
            #[inline(always)]
            pub const fn lastrx_stop(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event LASTRX and task STOP"]
            #[inline(always)]
            pub const fn set_lastrx_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        impl core::fmt::Debug for Shorts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Shorts")
                    .field("lasttx_startrx", &self.lasttx_startrx())
                    .field("lasttx_suspend", &self.lasttx_suspend())
                    .field("lasttx_stop", &self.lasttx_stop())
                    .field("lastrx_starttx", &self.lastrx_starttx())
                    .field("lastrx_stop", &self.lastrx_stop())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Shorts {{ lasttx_startrx: {=bool:?}, lasttx_suspend: {=bool:?}, lasttx_stop: {=bool:?}, lastrx_starttx: {=bool:?}, lastrx_stop: {=bool:?} }}" , self . lasttx_startrx () , self . lasttx_suspend () , self . lasttx_stop () , self . lastrx_starttx () , self . lastrx_stop ())
            }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdAmount(pub u32);
        impl TxdAmount {
            #[doc = "Number of bytes transferred in the last transaction. In case of NACK error, includes the NACK'ed byte."]
            #[must_use]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last transaction. In case of NACK error, includes the NACK'ed byte."]
            #[inline(always)]
            pub const fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for TxdAmount {
            #[inline(always)]
            fn default() -> TxdAmount {
                TxdAmount(0)
            }
        }
        impl core::fmt::Debug for TxdAmount {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxdAmount")
                    .field("amount", &self.amount())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxdAmount {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "TxdAmount {{ amount: {=u16:?} }}", self.amount())
            }
        }
        #[doc = "EasyDMA list type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdList(pub u32);
        impl TxdList {
            #[doc = "List type"]
            #[must_use]
            #[inline(always)]
            pub const fn list(&self) -> super::vals::TxdListList {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::TxdListList::from_bits(val as u8)
            }
            #[doc = "List type"]
            #[inline(always)]
            pub const fn set_list(&mut self, val: super::vals::TxdListList) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
            }
        }
        impl Default for TxdList {
            #[inline(always)]
            fn default() -> TxdList {
                TxdList(0)
            }
        }
        impl core::fmt::Debug for TxdList {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxdList")
                    .field("list", &self.list())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxdList {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "TxdList {{ list: {:?} }}", self.list())
            }
        }
        #[doc = "Maximum number of bytes in transmit buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdMaxcnt(pub u32);
        impl TxdMaxcnt {
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[must_use]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[inline(always)]
            pub const fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for TxdMaxcnt {
            #[inline(always)]
            fn default() -> TxdMaxcnt {
                TxdMaxcnt(0)
            }
        }
        impl core::fmt::Debug for TxdMaxcnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxdMaxcnt")
                    .field("maxcnt", &self.maxcnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxdMaxcnt {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "TxdMaxcnt {{ maxcnt: {=u16:?} }}", self.maxcnt())
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Enable {
            #[doc = "Disable TWIM"]
            DISABLED = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            #[doc = "Enable TWIM"]
            ENABLED = 0x06,
            _RESERVED_7 = 0x07,
            _RESERVED_8 = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Enable {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Enable {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Enable {
            #[inline(always)]
            fn from(val: u8) -> Enable {
                Enable::from_bits(val)
            }
        }
        impl From<Enable> for u8 {
            #[inline(always)]
            fn from(val: Enable) -> u8 {
                Enable::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Frequency(u32);
        impl Frequency {
            #[doc = "100 kbps"]
            pub const K100: Self = Self(0x0198_0000);
            #[doc = "250 kbps"]
            pub const K250: Self = Self(0x0400_0000);
            #[doc = "400 kbps"]
            pub const K400: Self = Self(0x0640_0000);
            #[doc = "1000 kbps"]
            pub const K1000: Self = Self(0x0ff0_0000);
        }
        impl Frequency {
            pub const fn from_bits(val: u32) -> Frequency {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Frequency {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0198_0000 => f.write_str("K100"),
                    0x0400_0000 => f.write_str("K250"),
                    0x0640_0000 => f.write_str("K400"),
                    0x0ff0_0000 => f.write_str("K1000"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Frequency {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0198_0000 => defmt::write!(f, "K100"),
                    0x0400_0000 => defmt::write!(f, "K250"),
                    0x0640_0000 => defmt::write!(f, "K400"),
                    0x0ff0_0000 => defmt::write!(f, "K1000"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for Frequency {
            #[inline(always)]
            fn from(val: u32) -> Frequency {
                Frequency::from_bits(val)
            }
        }
        impl From<Frequency> for u32 {
            #[inline(always)]
            fn from(val: Frequency) -> u32 {
                Frequency::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum RxdListList {
            #[doc = "Disable EasyDMA list"]
            DISABLED = 0x0,
            #[doc = "Use array list"]
            ARRAY_LIST = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl RxdListList {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> RxdListList {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for RxdListList {
            #[inline(always)]
            fn from(val: u8) -> RxdListList {
                RxdListList::from_bits(val)
            }
        }
        impl From<RxdListList> for u8 {
            #[inline(always)]
            fn from(val: RxdListList) -> u8 {
                RxdListList::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum TxdListList {
            #[doc = "Disable EasyDMA list"]
            DISABLED = 0x0,
            #[doc = "Use array list"]
            ARRAY_LIST = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl TxdListList {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> TxdListList {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for TxdListList {
            #[inline(always)]
            fn from(val: u8) -> TxdListList {
                TxdListList::from_bits(val)
            }
        }
        impl From<TxdListList> for u8 {
            #[inline(always)]
            fn from(val: TxdListList) -> u8 {
                TxdListList::to_bits(val)
            }
        }
    }
}
pub mod twis {
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psel {
        ptr: *mut u8,
    }
    unsafe impl Send for Psel {}
    unsafe impl Sync for Psel {}
    impl Psel {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Pin select for SCL signal"]
        #[inline(always)]
        pub const fn scl(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Pin select for SDA signal"]
        #[inline(always)]
        pub const fn sda(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
    }
    #[doc = "RXD EasyDMA channel"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxd {
        ptr: *mut u8,
    }
    unsafe impl Send for Rxd {}
    unsafe impl Sync for Rxd {}
    impl Rxd {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "RXD Data pointer"]
        #[inline(always)]
        pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in RXD buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::RxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Number of bytes transferred in the last RXD transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::RxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "EasyDMA list type"]
        #[inline(always)]
        pub const fn list(self) -> crate::common::Reg<regs::RxdList, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
    }
    #[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Twis {
        ptr: *mut u8,
    }
    unsafe impl Send for Twis {}
    unsafe impl Sync for Twis {}
    impl Twis {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Stop TWI transaction"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
        }
        #[doc = "Suspend TWI transaction"]
        #[inline(always)]
        pub const fn tasks_suspend(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
        }
        #[doc = "Resume TWI transaction"]
        #[inline(always)]
        pub const fn tasks_resume(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
        }
        #[doc = "Prepare the TWI slave to respond to a write command"]
        #[inline(always)]
        pub const fn tasks_preparerx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
        }
        #[doc = "Prepare the TWI slave to respond to a read command"]
        #[inline(always)]
        pub const fn tasks_preparetx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
        }
        #[doc = "Subscribe configuration for task STOP"]
        #[inline(always)]
        pub const fn subscribe_stop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
        }
        #[doc = "Subscribe configuration for task SUSPEND"]
        #[inline(always)]
        pub const fn subscribe_suspend(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
        }
        #[doc = "Subscribe configuration for task RESUME"]
        #[inline(always)]
        pub const fn subscribe_resume(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
        }
        #[doc = "Subscribe configuration for task PREPARERX"]
        #[inline(always)]
        pub const fn subscribe_preparerx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
        }
        #[doc = "Subscribe configuration for task PREPARETX"]
        #[inline(always)]
        pub const fn subscribe_preparetx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
        }
        #[doc = "TWI stopped"]
        #[inline(always)]
        pub const fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
        }
        #[doc = "TWI error"]
        #[inline(always)]
        pub const fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
        }
        #[doc = "Receive sequence started"]
        #[inline(always)]
        pub const fn events_rxstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
        }
        #[doc = "Transmit sequence started"]
        #[inline(always)]
        pub const fn events_txstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
        }
        #[doc = "Write command received"]
        #[inline(always)]
        pub const fn events_write(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
        }
        #[doc = "Read command received"]
        #[inline(always)]
        pub const fn events_read(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
        }
        #[doc = "Publish configuration for event STOPPED"]
        #[inline(always)]
        pub const fn publish_stopped(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
        }
        #[doc = "Publish configuration for event ERROR"]
        #[inline(always)]
        pub const fn publish_error(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
        }
        #[doc = "Publish configuration for event RXSTARTED"]
        #[inline(always)]
        pub const fn publish_rxstarted(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
        }
        #[doc = "Publish configuration for event TXSTARTED"]
        #[inline(always)]
        pub const fn publish_txstarted(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
        }
        #[doc = "Publish configuration for event WRITE"]
        #[inline(always)]
        pub const fn publish_write(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
        }
        #[doc = "Publish configuration for event READ"]
        #[inline(always)]
        pub const fn publish_read(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
        }
        #[doc = "Error source"]
        #[inline(always)]
        pub const fn errorsrc(self) -> crate::common::Reg<regs::Errorsrc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d0usize) as _) }
        }
        #[doc = "Status register indicating which address had a match"]
        #[inline(always)]
        pub const fn match_(self) -> crate::common::Reg<regs::Match, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d4usize) as _) }
        }
        #[doc = "Enable TWIS"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn psel(self) -> Psel {
            unsafe { Psel::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
        }
        #[doc = "RXD EasyDMA channel"]
        #[inline(always)]
        pub const fn rxd(self) -> Rxd {
            unsafe { Rxd::from_ptr(self.ptr.wrapping_add(0x0534usize) as _) }
        }
        #[doc = "TXD EasyDMA channel"]
        #[inline(always)]
        pub const fn txd(self) -> Txd {
            unsafe { Txd::from_ptr(self.ptr.wrapping_add(0x0544usize) as _) }
        }
        #[doc = "Description collection: TWI slave address n"]
        #[inline(always)]
        pub const fn address(
            self,
            n: usize,
        ) -> crate::common::Reg<regs::Address, crate::common::RW> {
            assert!(n < 2usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0588usize + n * 4usize) as _)
            }
        }
        #[doc = "Configuration register for the address match mechanism"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0594usize) as _) }
        }
        #[doc = "Over-read character. Character sent out in case of an over-read of the transmit buffer."]
        #[inline(always)]
        pub const fn orc(self) -> crate::common::Reg<regs::Orc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
        }
    }
    #[doc = "TXD EasyDMA channel"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txd {
        ptr: *mut u8,
    }
    unsafe impl Send for Txd {}
    unsafe impl Sync for Txd {}
    impl Txd {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "TXD Data pointer"]
        #[inline(always)]
        pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in TXD buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::TxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Number of bytes transferred in the last TXD transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::TxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "EasyDMA list type"]
        #[inline(always)]
        pub const fn list(self) -> crate::common::Reg<regs::TxdList, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Description collection: TWI slave address n"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Address(pub u32);
        impl Address {
            #[doc = "TWI slave address"]
            #[must_use]
            #[inline(always)]
            pub const fn address(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "TWI slave address"]
            #[inline(always)]
            pub const fn set_address(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
            }
        }
        impl Default for Address {
            #[inline(always)]
            fn default() -> Address {
                Address(0)
            }
        }
        impl core::fmt::Debug for Address {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Address")
                    .field("address", &self.address())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Address {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Address {{ address: {=u8:?} }}", self.address())
            }
        }
        #[doc = "Configuration register for the address match mechanism"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Enable or disable address matching on ADDRESS\\[0\\]"]
            #[must_use]
            #[inline(always)]
            pub const fn address0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable address matching on ADDRESS\\[0\\]"]
            #[inline(always)]
            pub const fn set_address0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable address matching on ADDRESS\\[1\\]"]
            #[must_use]
            #[inline(always)]
            pub const fn address1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable address matching on ADDRESS\\[1\\]"]
            #[inline(always)]
            pub const fn set_address1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        impl core::fmt::Debug for Config {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Config")
                    .field("address0", &self.address0())
                    .field("address1", &self.address1())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Config {{ address0: {=bool:?}, address1: {=bool:?} }}",
                    self.address0(),
                    self.address1()
                )
            }
        }
        #[doc = "Enable TWIS"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable TWIS"]
            #[must_use]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable TWIS"]
            #[inline(always)]
            pub const fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        impl core::fmt::Debug for Enable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Enable")
                    .field("enable", &self.enable())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Enable {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Enable {{ enable: {:?} }}", self.enable())
            }
        }
        #[doc = "Error source"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Errorsrc(pub u32);
        impl Errorsrc {
            #[doc = "RX buffer overflow detected, and prevented"]
            #[must_use]
            #[inline(always)]
            pub const fn overflow(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "RX buffer overflow detected, and prevented"]
            #[inline(always)]
            pub const fn set_overflow(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "NACK sent after receiving a data byte"]
            #[must_use]
            #[inline(always)]
            pub const fn dnack(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "NACK sent after receiving a data byte"]
            #[inline(always)]
            pub const fn set_dnack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "TX buffer over-read detected, and prevented"]
            #[must_use]
            #[inline(always)]
            pub const fn overread(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "TX buffer over-read detected, and prevented"]
            #[inline(always)]
            pub const fn set_overread(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Errorsrc {
            #[inline(always)]
            fn default() -> Errorsrc {
                Errorsrc(0)
            }
        }
        impl core::fmt::Debug for Errorsrc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Errorsrc")
                    .field("overflow", &self.overflow())
                    .field("dnack", &self.dnack())
                    .field("overread", &self.overread())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Errorsrc {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Errorsrc {{ overflow: {=bool:?}, dnack: {=bool:?}, overread: {=bool:?} }}",
                    self.overflow(),
                    self.dnack(),
                    self.overread()
                )
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Enable or disable interrupt for event STOPPED"]
            #[must_use]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event STOPPED"]
            #[inline(always)]
            pub const fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for event ERROR"]
            #[must_use]
            #[inline(always)]
            pub const fn error(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event ERROR"]
            #[inline(always)]
            pub const fn set_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Enable or disable interrupt for event RXSTARTED"]
            #[must_use]
            #[inline(always)]
            pub const fn rxstarted(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RXSTARTED"]
            #[inline(always)]
            pub const fn set_rxstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "Enable or disable interrupt for event TXSTARTED"]
            #[must_use]
            #[inline(always)]
            pub const fn txstarted(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event TXSTARTED"]
            #[inline(always)]
            pub const fn set_txstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "Enable or disable interrupt for event WRITE"]
            #[must_use]
            #[inline(always)]
            pub const fn write(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event WRITE"]
            #[inline(always)]
            pub const fn set_write(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
            #[doc = "Enable or disable interrupt for event READ"]
            #[must_use]
            #[inline(always)]
            pub const fn read(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event READ"]
            #[inline(always)]
            pub const fn set_read(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("stopped", &self.stopped())
                    .field("error", &self.error())
                    .field("rxstarted", &self.rxstarted())
                    .field("txstarted", &self.txstarted())
                    .field("write", &self.write())
                    .field("read", &self.read())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Int {{ stopped: {=bool:?}, error: {=bool:?}, rxstarted: {=bool:?}, txstarted: {=bool:?}, write: {=bool:?}, read: {=bool:?} }}" , self . stopped () , self . error () , self . rxstarted () , self . txstarted () , self . write () , self . read ())
            }
        }
        #[doc = "Status register indicating which address had a match"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Match(pub u32);
        impl Match {
            #[doc = "Indication of which address in {ADDRESS} that matched the incoming address"]
            #[must_use]
            #[inline(always)]
            pub const fn match_(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Indication of which address in {ADDRESS} that matched the incoming address"]
            #[inline(always)]
            pub const fn set_match_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Match {
            #[inline(always)]
            fn default() -> Match {
                Match(0)
            }
        }
        impl core::fmt::Debug for Match {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Match")
                    .field("match_", &self.match_())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Match {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Match {{ match_: {=bool:?} }}", self.match_())
            }
        }
        #[doc = "Over-read character. Character sent out in case of an over-read of the transmit buffer."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Orc(pub u32);
        impl Orc {
            #[doc = "Over-read character. Character sent out in case of an over-read of the transmit buffer."]
            #[must_use]
            #[inline(always)]
            pub const fn orc(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Over-read character. Character sent out in case of an over-read of the transmit buffer."]
            #[inline(always)]
            pub const fn set_orc(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Orc {
            #[inline(always)]
            fn default() -> Orc {
                Orc(0)
            }
        }
        impl core::fmt::Debug for Orc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Orc").field("orc", &self.orc()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Orc {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Orc {{ orc: {=u8:?} }}", self.orc())
            }
        }
        #[doc = "Number of bytes transferred in the last RXD transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdAmount(pub u32);
        impl RxdAmount {
            #[doc = "Number of bytes transferred in the last RXD transaction"]
            #[must_use]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last RXD transaction"]
            #[inline(always)]
            pub const fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for RxdAmount {
            #[inline(always)]
            fn default() -> RxdAmount {
                RxdAmount(0)
            }
        }
        impl core::fmt::Debug for RxdAmount {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxdAmount")
                    .field("amount", &self.amount())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxdAmount {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "RxdAmount {{ amount: {=u16:?} }}", self.amount())
            }
        }
        #[doc = "EasyDMA list type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdList(pub u32);
        impl RxdList {
            #[doc = "List type"]
            #[must_use]
            #[inline(always)]
            pub const fn list(&self) -> super::vals::RxdListList {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::RxdListList::from_bits(val as u8)
            }
            #[doc = "List type"]
            #[inline(always)]
            pub const fn set_list(&mut self, val: super::vals::RxdListList) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for RxdList {
            #[inline(always)]
            fn default() -> RxdList {
                RxdList(0)
            }
        }
        impl core::fmt::Debug for RxdList {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxdList")
                    .field("list", &self.list())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxdList {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "RxdList {{ list: {:?} }}", self.list())
            }
        }
        #[doc = "Maximum number of bytes in RXD buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdMaxcnt(pub u32);
        impl RxdMaxcnt {
            #[doc = "Maximum number of bytes in RXD buffer"]
            #[must_use]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Maximum number of bytes in RXD buffer"]
            #[inline(always)]
            pub const fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for RxdMaxcnt {
            #[inline(always)]
            fn default() -> RxdMaxcnt {
                RxdMaxcnt(0)
            }
        }
        impl core::fmt::Debug for RxdMaxcnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxdMaxcnt")
                    .field("maxcnt", &self.maxcnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxdMaxcnt {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "RxdMaxcnt {{ maxcnt: {=u16:?} }}", self.maxcnt())
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event WRITE and task SUSPEND"]
            #[must_use]
            #[inline(always)]
            pub const fn write_suspend(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event WRITE and task SUSPEND"]
            #[inline(always)]
            pub const fn set_write_suspend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Shortcut between event READ and task SUSPEND"]
            #[must_use]
            #[inline(always)]
            pub const fn read_suspend(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event READ and task SUSPEND"]
            #[inline(always)]
            pub const fn set_read_suspend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        impl core::fmt::Debug for Shorts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Shorts")
                    .field("write_suspend", &self.write_suspend())
                    .field("read_suspend", &self.read_suspend())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Shorts {{ write_suspend: {=bool:?}, read_suspend: {=bool:?} }}",
                    self.write_suspend(),
                    self.read_suspend()
                )
            }
        }
        #[doc = "Number of bytes transferred in the last TXD transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdAmount(pub u32);
        impl TxdAmount {
            #[doc = "Number of bytes transferred in the last TXD transaction"]
            #[must_use]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last TXD transaction"]
            #[inline(always)]
            pub const fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for TxdAmount {
            #[inline(always)]
            fn default() -> TxdAmount {
                TxdAmount(0)
            }
        }
        impl core::fmt::Debug for TxdAmount {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxdAmount")
                    .field("amount", &self.amount())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxdAmount {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "TxdAmount {{ amount: {=u16:?} }}", self.amount())
            }
        }
        #[doc = "EasyDMA list type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdList(pub u32);
        impl TxdList {
            #[doc = "List type"]
            #[must_use]
            #[inline(always)]
            pub const fn list(&self) -> super::vals::TxdListList {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::TxdListList::from_bits(val as u8)
            }
            #[doc = "List type"]
            #[inline(always)]
            pub const fn set_list(&mut self, val: super::vals::TxdListList) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for TxdList {
            #[inline(always)]
            fn default() -> TxdList {
                TxdList(0)
            }
        }
        impl core::fmt::Debug for TxdList {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxdList")
                    .field("list", &self.list())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxdList {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "TxdList {{ list: {:?} }}", self.list())
            }
        }
        #[doc = "Maximum number of bytes in TXD buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdMaxcnt(pub u32);
        impl TxdMaxcnt {
            #[doc = "Maximum number of bytes in TXD buffer"]
            #[must_use]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Maximum number of bytes in TXD buffer"]
            #[inline(always)]
            pub const fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for TxdMaxcnt {
            #[inline(always)]
            fn default() -> TxdMaxcnt {
                TxdMaxcnt(0)
            }
        }
        impl core::fmt::Debug for TxdMaxcnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxdMaxcnt")
                    .field("maxcnt", &self.maxcnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxdMaxcnt {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "TxdMaxcnt {{ maxcnt: {=u16:?} }}", self.maxcnt())
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Enable {
            #[doc = "Disable TWIS"]
            DISABLED = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
            _RESERVED_8 = 0x08,
            #[doc = "Enable TWIS"]
            ENABLED = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Enable {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Enable {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Enable {
            #[inline(always)]
            fn from(val: u8) -> Enable {
                Enable::from_bits(val)
            }
        }
        impl From<Enable> for u8 {
            #[inline(always)]
            fn from(val: Enable) -> u8 {
                Enable::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum RxdListList {
            #[doc = "Disable EasyDMA list"]
            DISABLED = 0x0,
            #[doc = "Use array list"]
            ARRAY_LIST = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl RxdListList {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> RxdListList {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for RxdListList {
            #[inline(always)]
            fn from(val: u8) -> RxdListList {
                RxdListList::from_bits(val)
            }
        }
        impl From<RxdListList> for u8 {
            #[inline(always)]
            fn from(val: RxdListList) -> u8 {
                RxdListList::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum TxdListList {
            #[doc = "Disable EasyDMA list"]
            DISABLED = 0x0,
            #[doc = "Use array list"]
            ARRAY_LIST = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl TxdListList {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> TxdListList {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for TxdListList {
            #[inline(always)]
            fn from(val: u8) -> TxdListList {
                TxdListList::from_bits(val)
            }
        }
        impl From<TxdListList> for u8 {
            #[inline(always)]
            fn from(val: TxdListList) -> u8 {
                TxdListList::to_bits(val)
            }
        }
    }
}
pub mod uarte {
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psel {
        ptr: *mut u8,
    }
    unsafe impl Send for Psel {}
    unsafe impl Sync for Psel {}
    impl Psel {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Pin select for RTS signal"]
        #[inline(always)]
        pub const fn rts(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Pin select for TXD signal"]
        #[inline(always)]
        pub const fn txd(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Pin select for CTS signal"]
        #[inline(always)]
        pub const fn cts(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "Pin select for RXD signal"]
        #[inline(always)]
        pub const fn rxd(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
    }
    #[doc = "RXD EasyDMA channel"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxd {
        ptr: *mut u8,
    }
    unsafe impl Send for Rxd {}
    unsafe impl Sync for Rxd {}
    impl Rxd {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Data pointer"]
        #[inline(always)]
        pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in receive buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::RxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::RxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
    }
    #[doc = "TXD EasyDMA channel"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txd {
        ptr: *mut u8,
    }
    unsafe impl Send for Txd {}
    unsafe impl Sync for Txd {}
    impl Txd {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Data pointer"]
        #[inline(always)]
        pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in transmit buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::TxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::TxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
    }
    #[doc = "UART with EasyDMA"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uarte {
        ptr: *mut u8,
    }
    unsafe impl Send for Uarte {}
    unsafe impl Sync for Uarte {}
    impl Uarte {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start UART receiver"]
        #[inline(always)]
        pub const fn tasks_startrx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Stop UART receiver"]
        #[inline(always)]
        pub const fn tasks_stoprx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Start UART transmitter"]
        #[inline(always)]
        pub const fn tasks_starttx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "Stop UART transmitter"]
        #[inline(always)]
        pub const fn tasks_stoptx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
        #[doc = "Flush RX FIFO into RX buffer"]
        #[inline(always)]
        pub const fn tasks_flushrx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
        }
        #[doc = "Subscribe configuration for task STARTRX"]
        #[inline(always)]
        pub const fn subscribe_startrx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
        }
        #[doc = "Subscribe configuration for task STOPRX"]
        #[inline(always)]
        pub const fn subscribe_stoprx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
        }
        #[doc = "Subscribe configuration for task STARTTX"]
        #[inline(always)]
        pub const fn subscribe_starttx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
        }
        #[doc = "Subscribe configuration for task STOPTX"]
        #[inline(always)]
        pub const fn subscribe_stoptx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
        }
        #[doc = "Subscribe configuration for task FLUSHRX"]
        #[inline(always)]
        pub const fn subscribe_flushrx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
        }
        #[doc = "CTS is activated (set low). Clear To Send."]
        #[inline(always)]
        pub const fn events_cts(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
        }
        #[doc = "CTS is deactivated (set high). Not Clear To Send."]
        #[inline(always)]
        pub const fn events_ncts(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
        }
        #[doc = "Data received in RXD (but potentially not yet transferred to Data RAM)"]
        #[inline(always)]
        pub const fn events_rxdrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
        }
        #[doc = "Receive buffer is filled up"]
        #[inline(always)]
        pub const fn events_endrx(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
        }
        #[doc = "Data sent from TXD"]
        #[inline(always)]
        pub const fn events_txdrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
        }
        #[doc = "Last TX byte transmitted"]
        #[inline(always)]
        pub const fn events_endtx(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
        }
        #[doc = "Error detected"]
        #[inline(always)]
        pub const fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
        }
        #[doc = "Receiver timeout"]
        #[inline(always)]
        pub const fn events_rxto(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
        }
        #[doc = "UART receiver has started"]
        #[inline(always)]
        pub const fn events_rxstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
        }
        #[doc = "UART transmitter has started"]
        #[inline(always)]
        pub const fn events_txstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
        }
        #[doc = "Transmitter stopped"]
        #[inline(always)]
        pub const fn events_txstopped(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
        }
        #[doc = "Publish configuration for event CTS"]
        #[inline(always)]
        pub const fn publish_cts(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
        }
        #[doc = "Publish configuration for event NCTS"]
        #[inline(always)]
        pub const fn publish_ncts(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
        }
        #[doc = "Publish configuration for event RXDRDY"]
        #[inline(always)]
        pub const fn publish_rxdrdy(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
        }
        #[doc = "Publish configuration for event ENDRX"]
        #[inline(always)]
        pub const fn publish_endrx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
        }
        #[doc = "Publish configuration for event TXDRDY"]
        #[inline(always)]
        pub const fn publish_txdrdy(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
        }
        #[doc = "Publish configuration for event ENDTX"]
        #[inline(always)]
        pub const fn publish_endtx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
        }
        #[doc = "Publish configuration for event ERROR"]
        #[inline(always)]
        pub const fn publish_error(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
        }
        #[doc = "Publish configuration for event RXTO"]
        #[inline(always)]
        pub const fn publish_rxto(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
        }
        #[doc = "Publish configuration for event RXSTARTED"]
        #[inline(always)]
        pub const fn publish_rxstarted(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
        }
        #[doc = "Publish configuration for event TXSTARTED"]
        #[inline(always)]
        pub const fn publish_txstarted(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
        }
        #[doc = "Publish configuration for event TXSTOPPED"]
        #[inline(always)]
        pub const fn publish_txstopped(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
        }
        #[doc = "Error source"]
        #[inline(always)]
        pub const fn errorsrc(self) -> crate::common::Reg<regs::Errorsrc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0480usize) as _) }
        }
        #[doc = "Enable UART"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn psel(self) -> Psel {
            unsafe { Psel::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
        }
        #[doc = "Baud rate. Accuracy depends on the HFCLK source selected."]
        #[inline(always)]
        pub const fn baudrate(self) -> crate::common::Reg<regs::Baudrate, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0524usize) as _) }
        }
        #[doc = "RXD EasyDMA channel"]
        #[inline(always)]
        pub const fn rxd(self) -> Rxd {
            unsafe { Rxd::from_ptr(self.ptr.wrapping_add(0x0534usize) as _) }
        }
        #[doc = "TXD EasyDMA channel"]
        #[inline(always)]
        pub const fn txd(self) -> Txd {
            unsafe { Txd::from_ptr(self.ptr.wrapping_add(0x0544usize) as _) }
        }
        #[doc = "Configuration of parity and hardware flow control"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x056cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Baud rate. Accuracy depends on the HFCLK source selected."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Baudrate(pub u32);
        impl Baudrate {
            #[doc = "Baud rate"]
            #[must_use]
            #[inline(always)]
            pub const fn baudrate(&self) -> super::vals::Baudrate {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Baudrate::from_bits(val as u32)
            }
            #[doc = "Baud rate"]
            #[inline(always)]
            pub const fn set_baudrate(&mut self, val: super::vals::Baudrate) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Baudrate {
            #[inline(always)]
            fn default() -> Baudrate {
                Baudrate(0)
            }
        }
        impl core::fmt::Debug for Baudrate {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Baudrate")
                    .field("baudrate", &self.baudrate())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Baudrate {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Baudrate {{ baudrate: {:?} }}", self.baudrate())
            }
        }
        #[doc = "Configuration of parity and hardware flow control"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Hardware flow control"]
            #[must_use]
            #[inline(always)]
            pub const fn hwfc(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Hardware flow control"]
            #[inline(always)]
            pub const fn set_hwfc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Parity"]
            #[must_use]
            #[inline(always)]
            pub const fn parity(&self) -> super::vals::ConfigParity {
                let val = (self.0 >> 1usize) & 0x07;
                super::vals::ConfigParity::from_bits(val as u8)
            }
            #[doc = "Parity"]
            #[inline(always)]
            pub const fn set_parity(&mut self, val: super::vals::ConfigParity) {
                self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
            }
            #[doc = "Stop bits"]
            #[must_use]
            #[inline(always)]
            pub const fn stop(&self) -> super::vals::Stop {
                let val = (self.0 >> 4usize) & 0x01;
                super::vals::Stop::from_bits(val as u8)
            }
            #[doc = "Stop bits"]
            #[inline(always)]
            pub const fn set_stop(&mut self, val: super::vals::Stop) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
            }
            #[doc = "Even or odd parity type"]
            #[must_use]
            #[inline(always)]
            pub const fn paritytype(&self) -> super::vals::Paritytype {
                let val = (self.0 >> 8usize) & 0x01;
                super::vals::Paritytype::from_bits(val as u8)
            }
            #[doc = "Even or odd parity type"]
            #[inline(always)]
            pub const fn set_paritytype(&mut self, val: super::vals::Paritytype) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        impl core::fmt::Debug for Config {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Config")
                    .field("hwfc", &self.hwfc())
                    .field("parity", &self.parity())
                    .field("stop", &self.stop())
                    .field("paritytype", &self.paritytype())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Config {{ hwfc: {=bool:?}, parity: {:?}, stop: {:?}, paritytype: {:?} }}",
                    self.hwfc(),
                    self.parity(),
                    self.stop(),
                    self.paritytype()
                )
            }
        }
        #[doc = "Enable UART"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable UARTE"]
            #[must_use]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable UARTE"]
            #[inline(always)]
            pub const fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        impl core::fmt::Debug for Enable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Enable")
                    .field("enable", &self.enable())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Enable {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Enable {{ enable: {:?} }}", self.enable())
            }
        }
        #[doc = "Error source"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Errorsrc(pub u32);
        impl Errorsrc {
            #[doc = "Overrun error"]
            #[must_use]
            #[inline(always)]
            pub const fn overrun(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun error"]
            #[inline(always)]
            pub const fn set_overrun(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Parity error"]
            #[must_use]
            #[inline(always)]
            pub const fn parity(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Parity error"]
            #[inline(always)]
            pub const fn set_parity(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Framing error occurred"]
            #[must_use]
            #[inline(always)]
            pub const fn framing(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Framing error occurred"]
            #[inline(always)]
            pub const fn set_framing(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Break condition"]
            #[must_use]
            #[inline(always)]
            pub const fn break_(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Break condition"]
            #[inline(always)]
            pub const fn set_break_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Errorsrc {
            #[inline(always)]
            fn default() -> Errorsrc {
                Errorsrc(0)
            }
        }
        impl core::fmt::Debug for Errorsrc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Errorsrc")
                    .field("overrun", &self.overrun())
                    .field("parity", &self.parity())
                    .field("framing", &self.framing())
                    .field("break_", &self.break_())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Errorsrc {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Errorsrc {{ overrun: {=bool:?}, parity: {=bool:?}, framing: {=bool:?}, break_: {=bool:?} }}" , self . overrun () , self . parity () , self . framing () , self . break_ ())
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Enable or disable interrupt for event CTS"]
            #[must_use]
            #[inline(always)]
            pub const fn cts(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event CTS"]
            #[inline(always)]
            pub const fn set_cts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable interrupt for event NCTS"]
            #[must_use]
            #[inline(always)]
            pub const fn ncts(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event NCTS"]
            #[inline(always)]
            pub const fn set_ncts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for event RXDRDY"]
            #[must_use]
            #[inline(always)]
            pub const fn rxdrdy(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RXDRDY"]
            #[inline(always)]
            pub const fn set_rxdrdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable or disable interrupt for event ENDRX"]
            #[must_use]
            #[inline(always)]
            pub const fn endrx(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event ENDRX"]
            #[inline(always)]
            pub const fn set_endrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable or disable interrupt for event TXDRDY"]
            #[must_use]
            #[inline(always)]
            pub const fn txdrdy(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event TXDRDY"]
            #[inline(always)]
            pub const fn set_txdrdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Enable or disable interrupt for event ENDTX"]
            #[must_use]
            #[inline(always)]
            pub const fn endtx(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event ENDTX"]
            #[inline(always)]
            pub const fn set_endtx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Enable or disable interrupt for event ERROR"]
            #[must_use]
            #[inline(always)]
            pub const fn error(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event ERROR"]
            #[inline(always)]
            pub const fn set_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Enable or disable interrupt for event RXTO"]
            #[must_use]
            #[inline(always)]
            pub const fn rxto(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RXTO"]
            #[inline(always)]
            pub const fn set_rxto(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Enable or disable interrupt for event RXSTARTED"]
            #[must_use]
            #[inline(always)]
            pub const fn rxstarted(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RXSTARTED"]
            #[inline(always)]
            pub const fn set_rxstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "Enable or disable interrupt for event TXSTARTED"]
            #[must_use]
            #[inline(always)]
            pub const fn txstarted(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event TXSTARTED"]
            #[inline(always)]
            pub const fn set_txstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "Enable or disable interrupt for event TXSTOPPED"]
            #[must_use]
            #[inline(always)]
            pub const fn txstopped(&self) -> bool {
                let val = (self.0 >> 22usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event TXSTOPPED"]
            #[inline(always)]
            pub const fn set_txstopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("cts", &self.cts())
                    .field("ncts", &self.ncts())
                    .field("rxdrdy", &self.rxdrdy())
                    .field("endrx", &self.endrx())
                    .field("txdrdy", &self.txdrdy())
                    .field("endtx", &self.endtx())
                    .field("error", &self.error())
                    .field("rxto", &self.rxto())
                    .field("rxstarted", &self.rxstarted())
                    .field("txstarted", &self.txstarted())
                    .field("txstopped", &self.txstopped())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Int {{ cts: {=bool:?}, ncts: {=bool:?}, rxdrdy: {=bool:?}, endrx: {=bool:?}, txdrdy: {=bool:?}, endtx: {=bool:?}, error: {=bool:?}, rxto: {=bool:?}, rxstarted: {=bool:?}, txstarted: {=bool:?}, txstopped: {=bool:?} }}" , self . cts () , self . ncts () , self . rxdrdy () , self . endrx () , self . txdrdy () , self . endtx () , self . error () , self . rxto () , self . rxstarted () , self . txstarted () , self . txstopped ())
            }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdAmount(pub u32);
        impl RxdAmount {
            #[doc = "Number of bytes transferred in the last transaction"]
            #[must_use]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last transaction"]
            #[inline(always)]
            pub const fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for RxdAmount {
            #[inline(always)]
            fn default() -> RxdAmount {
                RxdAmount(0)
            }
        }
        impl core::fmt::Debug for RxdAmount {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxdAmount")
                    .field("amount", &self.amount())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxdAmount {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "RxdAmount {{ amount: {=u16:?} }}", self.amount())
            }
        }
        #[doc = "Maximum number of bytes in receive buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdMaxcnt(pub u32);
        impl RxdMaxcnt {
            #[doc = "Maximum number of bytes in receive buffer"]
            #[must_use]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Maximum number of bytes in receive buffer"]
            #[inline(always)]
            pub const fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for RxdMaxcnt {
            #[inline(always)]
            fn default() -> RxdMaxcnt {
                RxdMaxcnt(0)
            }
        }
        impl core::fmt::Debug for RxdMaxcnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxdMaxcnt")
                    .field("maxcnt", &self.maxcnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxdMaxcnt {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "RxdMaxcnt {{ maxcnt: {=u16:?} }}", self.maxcnt())
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event ENDRX and task STARTRX"]
            #[must_use]
            #[inline(always)]
            pub const fn endrx_startrx(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event ENDRX and task STARTRX"]
            #[inline(always)]
            pub const fn set_endrx_startrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Shortcut between event ENDRX and task STOPRX"]
            #[must_use]
            #[inline(always)]
            pub const fn endrx_stoprx(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event ENDRX and task STOPRX"]
            #[inline(always)]
            pub const fn set_endrx_stoprx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        impl core::fmt::Debug for Shorts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Shorts")
                    .field("endrx_startrx", &self.endrx_startrx())
                    .field("endrx_stoprx", &self.endrx_stoprx())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Shorts {{ endrx_startrx: {=bool:?}, endrx_stoprx: {=bool:?} }}",
                    self.endrx_startrx(),
                    self.endrx_stoprx()
                )
            }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdAmount(pub u32);
        impl TxdAmount {
            #[doc = "Number of bytes transferred in the last transaction"]
            #[must_use]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last transaction"]
            #[inline(always)]
            pub const fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for TxdAmount {
            #[inline(always)]
            fn default() -> TxdAmount {
                TxdAmount(0)
            }
        }
        impl core::fmt::Debug for TxdAmount {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxdAmount")
                    .field("amount", &self.amount())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxdAmount {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "TxdAmount {{ amount: {=u16:?} }}", self.amount())
            }
        }
        #[doc = "Maximum number of bytes in transmit buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdMaxcnt(pub u32);
        impl TxdMaxcnt {
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[must_use]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[inline(always)]
            pub const fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for TxdMaxcnt {
            #[inline(always)]
            fn default() -> TxdMaxcnt {
                TxdMaxcnt(0)
            }
        }
        impl core::fmt::Debug for TxdMaxcnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxdMaxcnt")
                    .field("maxcnt", &self.maxcnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxdMaxcnt {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "TxdMaxcnt {{ maxcnt: {=u16:?} }}", self.maxcnt())
            }
        }
    }
    pub mod vals {
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Baudrate(u32);
        impl Baudrate {
            #[doc = "1200 baud (actual rate: 1205)"]
            pub const BAUD1200: Self = Self(0x0004_f000);
            #[doc = "2400 baud (actual rate: 2396)"]
            pub const BAUD2400: Self = Self(0x0009_d000);
            #[doc = "4800 baud (actual rate: 4808)"]
            pub const BAUD4800: Self = Self(0x0013_b000);
            #[doc = "9600 baud (actual rate: 9598)"]
            pub const BAUD9600: Self = Self(0x0027_5000);
            #[doc = "14400 baud (actual rate: 14401)"]
            pub const BAUD14400: Self = Self(0x003a_f000);
            #[doc = "19200 baud (actual rate: 19208)"]
            pub const BAUD19200: Self = Self(0x004e_a000);
            #[doc = "28800 baud (actual rate: 28777)"]
            pub const BAUD28800: Self = Self(0x0075_c000);
            #[doc = "31250 baud"]
            pub const BAUD31250: Self = Self(0x0080_0000);
            #[doc = "38400 baud (actual rate: 38369)"]
            pub const BAUD38400: Self = Self(0x009d_0000);
            #[doc = "56000 baud (actual rate: 55944)"]
            pub const BAUD56000: Self = Self(0x00e5_0000);
            #[doc = "57600 baud (actual rate: 57554)"]
            pub const BAUD57600: Self = Self(0x00eb_0000);
            #[doc = "76800 baud (actual rate: 76923)"]
            pub const BAUD76800: Self = Self(0x013a_9000);
            #[doc = "115200 baud (actual rate: 115108)"]
            pub const BAUD115200: Self = Self(0x01d6_0000);
            #[doc = "230400 baud (actual rate: 231884)"]
            pub const BAUD230400: Self = Self(0x03b0_0000);
            #[doc = "250000 baud"]
            pub const BAUD250000: Self = Self(0x0400_0000);
            #[doc = "460800 baud (actual rate: 457143)"]
            pub const BAUD460800: Self = Self(0x0740_0000);
            #[doc = "921600 baud (actual rate: 941176)"]
            pub const BAUD921600: Self = Self(0x0f00_0000);
            #[doc = "1 megabaud"]
            pub const BAUD1M: Self = Self(0x1000_0000);
        }
        impl Baudrate {
            pub const fn from_bits(val: u32) -> Baudrate {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Baudrate {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0004_f000 => f.write_str("BAUD1200"),
                    0x0009_d000 => f.write_str("BAUD2400"),
                    0x0013_b000 => f.write_str("BAUD4800"),
                    0x0027_5000 => f.write_str("BAUD9600"),
                    0x003a_f000 => f.write_str("BAUD14400"),
                    0x004e_a000 => f.write_str("BAUD19200"),
                    0x0075_c000 => f.write_str("BAUD28800"),
                    0x0080_0000 => f.write_str("BAUD31250"),
                    0x009d_0000 => f.write_str("BAUD38400"),
                    0x00e5_0000 => f.write_str("BAUD56000"),
                    0x00eb_0000 => f.write_str("BAUD57600"),
                    0x013a_9000 => f.write_str("BAUD76800"),
                    0x01d6_0000 => f.write_str("BAUD115200"),
                    0x03b0_0000 => f.write_str("BAUD230400"),
                    0x0400_0000 => f.write_str("BAUD250000"),
                    0x0740_0000 => f.write_str("BAUD460800"),
                    0x0f00_0000 => f.write_str("BAUD921600"),
                    0x1000_0000 => f.write_str("BAUD1M"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Baudrate {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0004_f000 => defmt::write!(f, "BAUD1200"),
                    0x0009_d000 => defmt::write!(f, "BAUD2400"),
                    0x0013_b000 => defmt::write!(f, "BAUD4800"),
                    0x0027_5000 => defmt::write!(f, "BAUD9600"),
                    0x003a_f000 => defmt::write!(f, "BAUD14400"),
                    0x004e_a000 => defmt::write!(f, "BAUD19200"),
                    0x0075_c000 => defmt::write!(f, "BAUD28800"),
                    0x0080_0000 => defmt::write!(f, "BAUD31250"),
                    0x009d_0000 => defmt::write!(f, "BAUD38400"),
                    0x00e5_0000 => defmt::write!(f, "BAUD56000"),
                    0x00eb_0000 => defmt::write!(f, "BAUD57600"),
                    0x013a_9000 => defmt::write!(f, "BAUD76800"),
                    0x01d6_0000 => defmt::write!(f, "BAUD115200"),
                    0x03b0_0000 => defmt::write!(f, "BAUD230400"),
                    0x0400_0000 => defmt::write!(f, "BAUD250000"),
                    0x0740_0000 => defmt::write!(f, "BAUD460800"),
                    0x0f00_0000 => defmt::write!(f, "BAUD921600"),
                    0x1000_0000 => defmt::write!(f, "BAUD1M"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for Baudrate {
            #[inline(always)]
            fn from(val: u32) -> Baudrate {
                Baudrate::from_bits(val)
            }
        }
        impl From<Baudrate> for u32 {
            #[inline(always)]
            fn from(val: Baudrate) -> u32 {
                Baudrate::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum ConfigParity {
            #[doc = "Exclude parity bit"]
            EXCLUDED = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            #[doc = "Include even parity bit"]
            INCLUDED = 0x07,
        }
        impl ConfigParity {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> ConfigParity {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for ConfigParity {
            #[inline(always)]
            fn from(val: u8) -> ConfigParity {
                ConfigParity::from_bits(val)
            }
        }
        impl From<ConfigParity> for u8 {
            #[inline(always)]
            fn from(val: ConfigParity) -> u8 {
                ConfigParity::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Enable {
            #[doc = "Disable UARTE"]
            DISABLED = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
            #[doc = "Enable UARTE"]
            ENABLED = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Enable {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Enable {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Enable {
            #[inline(always)]
            fn from(val: u8) -> Enable {
                Enable::from_bits(val)
            }
        }
        impl From<Enable> for u8 {
            #[inline(always)]
            fn from(val: Enable) -> u8 {
                Enable::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Paritytype {
            #[doc = "Even parity"]
            EVEN = 0x0,
            #[doc = "Odd parity"]
            ODD = 0x01,
        }
        impl Paritytype {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Paritytype {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Paritytype {
            #[inline(always)]
            fn from(val: u8) -> Paritytype {
                Paritytype::from_bits(val)
            }
        }
        impl From<Paritytype> for u8 {
            #[inline(always)]
            fn from(val: Paritytype) -> u8 {
                Paritytype::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Stop {
            #[doc = "One stop bit"]
            ONE = 0x0,
            #[doc = "Two stop bits"]
            TWO = 0x01,
        }
        impl Stop {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Stop {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Stop {
            #[inline(always)]
            fn from(val: u8) -> Stop {
                Stop::from_bits(val)
            }
        }
        impl From<Stop> for u8 {
            #[inline(always)]
            fn from(val: Stop) -> u8 {
                Stop::to_bits(val)
            }
        }
    }
}
pub mod uicr {
    #[doc = "User Information Configuration Registers"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uicr {
        ptr: *mut u8,
    }
    unsafe impl Send for Uicr {}
    unsafe impl Sync for Uicr {}
    impl Uicr {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Access port protection"]
        #[inline(always)]
        pub const fn approtect(self) -> crate::common::Reg<regs::Approtect, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Erase protection"]
        #[inline(always)]
        pub const fn eraseprotect(
            self,
        ) -> crate::common::Reg<regs::Eraseprotect, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Description collection: Reserved for Nordic firmware design"]
        #[inline(always)]
        pub const fn nrffw(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 32usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
            }
        }
        #[doc = "Description collection: Reserved for customer"]
        #[inline(always)]
        pub const fn customer(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 32usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize + n * 4usize) as _)
            }
        }
    }
    pub mod regs {
        #[doc = "Access port protection"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Approtect(pub u32);
        impl Approtect {
            #[doc = "Blocks debugger read/write access to all CPU registers and memory mapped addresses."]
            #[must_use]
            #[inline(always)]
            pub const fn pall(&self) -> super::vals::ApprotectPall {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::ApprotectPall::from_bits(val as u32)
            }
            #[doc = "Blocks debugger read/write access to all CPU registers and memory mapped addresses."]
            #[inline(always)]
            pub const fn set_pall(&mut self, val: super::vals::ApprotectPall) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Approtect {
            #[inline(always)]
            fn default() -> Approtect {
                Approtect(0)
            }
        }
        impl core::fmt::Debug for Approtect {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Approtect")
                    .field("pall", &self.pall())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Approtect {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Approtect {{ pall: {:?} }}", self.pall())
            }
        }
        #[doc = "Erase protection"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Eraseprotect(pub u32);
        impl Eraseprotect {
            #[doc = "Blocks NVMC ERASEALL and CTRLAP ERASEALL functionality. Using any value except Unprotected will lead to the protection being enabled."]
            #[must_use]
            #[inline(always)]
            pub const fn pall(&self) -> super::vals::EraseprotectPall {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::EraseprotectPall::from_bits(val as u32)
            }
            #[doc = "Blocks NVMC ERASEALL and CTRLAP ERASEALL functionality. Using any value except Unprotected will lead to the protection being enabled."]
            #[inline(always)]
            pub const fn set_pall(&mut self, val: super::vals::EraseprotectPall) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Eraseprotect {
            #[inline(always)]
            fn default() -> Eraseprotect {
                Eraseprotect(0)
            }
        }
        impl core::fmt::Debug for Eraseprotect {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Eraseprotect")
                    .field("pall", &self.pall())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Eraseprotect {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Eraseprotect {{ pall: {:?} }}", self.pall())
            }
        }
    }
    pub mod vals {
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct ApprotectPall(u32);
        impl ApprotectPall {
            #[doc = "Protected"]
            pub const PROTECTED: Self = Self(0x0);
            #[doc = "Unprotected"]
            pub const UNPROTECTED: Self = Self(0x50fa_50fa);
        }
        impl ApprotectPall {
            pub const fn from_bits(val: u32) -> ApprotectPall {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for ApprotectPall {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0 => f.write_str("PROTECTED"),
                    0x50fa_50fa => f.write_str("UNPROTECTED"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for ApprotectPall {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0 => defmt::write!(f, "PROTECTED"),
                    0x50fa_50fa => defmt::write!(f, "UNPROTECTED"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for ApprotectPall {
            #[inline(always)]
            fn from(val: u32) -> ApprotectPall {
                ApprotectPall::from_bits(val)
            }
        }
        impl From<ApprotectPall> for u32 {
            #[inline(always)]
            fn from(val: ApprotectPall) -> u32 {
                ApprotectPall::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct EraseprotectPall(u32);
        impl EraseprotectPall {
            #[doc = "Protected"]
            pub const PROTECTED: Self = Self(0x0);
            #[doc = "Unprotected"]
            pub const UNPROTECTED: Self = Self(0xffff_ffff);
        }
        impl EraseprotectPall {
            pub const fn from_bits(val: u32) -> EraseprotectPall {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for EraseprotectPall {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0 => f.write_str("PROTECTED"),
                    0xffff_ffff => f.write_str("UNPROTECTED"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for EraseprotectPall {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0 => defmt::write!(f, "PROTECTED"),
                    0xffff_ffff => defmt::write!(f, "UNPROTECTED"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for EraseprotectPall {
            #[inline(always)]
            fn from(val: u32) -> EraseprotectPall {
                EraseprotectPall::from_bits(val)
            }
        }
        impl From<EraseprotectPall> for u32 {
            #[inline(always)]
            fn from(val: EraseprotectPall) -> u32 {
                EraseprotectPall::to_bits(val)
            }
        }
    }
}
pub mod vmc {
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram {
        ptr: *mut u8,
    }
    unsafe impl Send for Ram {}
    unsafe impl Sync for Ram {}
    impl Ram {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster: RAM\\[n\\] power control register"]
        #[inline(always)]
        pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Description cluster: RAM\\[n\\] power control set register"]
        #[inline(always)]
        pub const fn powerset(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Description cluster: RAM\\[n\\] power control clear register"]
        #[inline(always)]
        pub const fn powerclr(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
    }
    #[doc = "Volatile Memory controller"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vmc {
        ptr: *mut u8,
    }
    unsafe impl Send for Vmc {}
    unsafe impl Sync for Vmc {}
    impl Vmc {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn ram(self, n: usize) -> Ram {
            assert!(n < 4usize);
            unsafe { Ram::from_ptr(self.ptr.wrapping_add(0x0600usize + n * 16usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Description cluster: RAM\\[n\\] power control register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Power(pub u32);
        impl Power {
            #[doc = "Keep RAM section S0 of RAM\\[n\\] on or off in System ON mode"]
            #[must_use]
            #[inline(always)]
            pub const fn spower(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Keep RAM section S0 of RAM\\[n\\] on or off in System ON mode"]
            #[inline(always)]
            pub const fn set_spower(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Keep retention on RAM section S0 of RAM\\[n\\] when RAM section is switched off"]
            #[must_use]
            #[inline(always)]
            pub const fn sretention(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 16usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Keep retention on RAM section S0 of RAM\\[n\\] when RAM section is switched off"]
            #[inline(always)]
            pub const fn set_sretention(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 16usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Power {
            #[inline(always)]
            fn default() -> Power {
                Power(0)
            }
        }
        impl core::fmt::Debug for Power {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Power")
                    .field("spower[0]", &self.spower(0usize))
                    .field("spower[1]", &self.spower(1usize))
                    .field("spower[2]", &self.spower(2usize))
                    .field("spower[3]", &self.spower(3usize))
                    .field("sretention[0]", &self.sretention(0usize))
                    .field("sretention[1]", &self.sretention(1usize))
                    .field("sretention[2]", &self.sretention(2usize))
                    .field("sretention[3]", &self.sretention(3usize))
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Power {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Power {{ spower[0]: {=bool:?}, spower[1]: {=bool:?}, spower[2]: {=bool:?}, spower[3]: {=bool:?}, sretention[0]: {=bool:?}, sretention[1]: {=bool:?}, sretention[2]: {=bool:?}, sretention[3]: {=bool:?} }}" , self . spower (0usize) , self . spower (1usize) , self . spower (2usize) , self . spower (3usize) , self . sretention (0usize) , self . sretention (1usize) , self . sretention (2usize) , self . sretention (3usize))
            }
        }
    }
}
pub mod vreqctrl {
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vregradio {
        ptr: *mut u8,
    }
    unsafe impl Send for Vregradio {}
    unsafe impl Sync for Vregradio {}
    impl Vregradio {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Request high voltage on RADIO After requesting high voltage, the user must wait until VREQHREADY is set to Ready"]
        #[inline(always)]
        pub const fn vreqh(self) -> crate::common::Reg<regs::Vreqh, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "High voltage on RADIO is ready"]
        #[inline(always)]
        pub const fn vreqhready(self) -> crate::common::Reg<regs::Vreqhready, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
    }
    #[doc = "Voltage request control"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vreqctrl {
        ptr: *mut u8,
    }
    unsafe impl Send for Vreqctrl {}
    unsafe impl Sync for Vreqctrl {}
    impl Vreqctrl {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn vregradio(self) -> Vregradio {
            unsafe { Vregradio::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Request high voltage on RADIO After requesting high voltage, the user must wait until VREQHREADY is set to Ready"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Vreqh(pub u32);
        impl Vreqh {
            #[doc = "Request high voltage"]
            #[must_use]
            #[inline(always)]
            pub const fn vreqh(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Request high voltage"]
            #[inline(always)]
            pub const fn set_vreqh(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Vreqh {
            #[inline(always)]
            fn default() -> Vreqh {
                Vreqh(0)
            }
        }
        impl core::fmt::Debug for Vreqh {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Vreqh")
                    .field("vreqh", &self.vreqh())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Vreqh {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Vreqh {{ vreqh: {=bool:?} }}", self.vreqh())
            }
        }
        #[doc = "High voltage on RADIO is ready"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Vreqhready(pub u32);
        impl Vreqhready {
            #[doc = "RADIO is ready to operate on high voltage"]
            #[must_use]
            #[inline(always)]
            pub const fn ready(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "RADIO is ready to operate on high voltage"]
            #[inline(always)]
            pub const fn set_ready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Vreqhready {
            #[inline(always)]
            fn default() -> Vreqhready {
                Vreqhready(0)
            }
        }
        impl core::fmt::Debug for Vreqhready {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Vreqhready")
                    .field("ready", &self.ready())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Vreqhready {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Vreqhready {{ ready: {=bool:?} }}", self.ready())
            }
        }
    }
}
pub mod wdt {
    #[doc = "Watchdog Timer"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wdt {
        ptr: *mut u8,
    }
    unsafe impl Send for Wdt {}
    unsafe impl Sync for Wdt {}
    impl Wdt {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start WDT"]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Stop WDT"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Subscribe configuration for task START"]
        #[inline(always)]
        pub const fn subscribe_start(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
        }
        #[doc = "Subscribe configuration for task STOP"]
        #[inline(always)]
        pub const fn subscribe_stop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
        }
        #[doc = "Watchdog timeout"]
        #[inline(always)]
        pub const fn events_timeout(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
        }
        #[doc = "Watchdog stopped"]
        #[inline(always)]
        pub const fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
        }
        #[doc = "Publish configuration for event TIMEOUT"]
        #[inline(always)]
        pub const fn publish_timeout(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
        }
        #[doc = "Publish configuration for event STOPPED"]
        #[inline(always)]
        pub const fn publish_stopped(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn nmienset(self) -> crate::common::Reg<regs::Nmi, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0324usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn nmienclr(self) -> crate::common::Reg<regs::Nmi, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0328usize) as _) }
        }
        #[doc = "Run status"]
        #[inline(always)]
        pub const fn runstatus(self) -> crate::common::Reg<regs::Runstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
        }
        #[doc = "Request status"]
        #[inline(always)]
        pub const fn reqstatus(self) -> crate::common::Reg<regs::Reqstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
        }
        #[doc = "Counter reload value"]
        #[inline(always)]
        pub const fn crv(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
        }
        #[doc = "Enable register for reload request registers"]
        #[inline(always)]
        pub const fn rren(self) -> crate::common::Reg<regs::Rren, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
        }
        #[doc = "Configuration register"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x050cusize) as _) }
        }
        #[doc = "Task stop enable"]
        #[inline(always)]
        pub const fn tsen(self) -> crate::common::Reg<regs::Tsen, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0520usize) as _) }
        }
        #[doc = "Description collection: Reload request n"]
        #[inline(always)]
        pub const fn rr(self, n: usize) -> crate::common::Reg<regs::Rr, crate::common::W> {
            assert!(n < 8usize);
            unsafe {
                crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize + n * 4usize) as _)
            }
        }
    }
    pub mod regs {
        #[doc = "Configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Configure WDT to either be paused, or kept running, while the CPU is sleeping"]
            #[must_use]
            #[inline(always)]
            pub const fn sleep(&self) -> super::vals::Sleep {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Sleep::from_bits(val as u8)
            }
            #[doc = "Configure WDT to either be paused, or kept running, while the CPU is sleeping"]
            #[inline(always)]
            pub const fn set_sleep(&mut self, val: super::vals::Sleep) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Configure WDT to either be paused, or kept running, while the CPU is halted by the debugger"]
            #[must_use]
            #[inline(always)]
            pub const fn halt(&self) -> super::vals::Halt {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Halt::from_bits(val as u8)
            }
            #[doc = "Configure WDT to either be paused, or kept running, while the CPU is halted by the debugger"]
            #[inline(always)]
            pub const fn set_halt(&mut self, val: super::vals::Halt) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
            }
            #[doc = "Allow stopping WDT"]
            #[must_use]
            #[inline(always)]
            pub const fn stopen(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Allow stopping WDT"]
            #[inline(always)]
            pub const fn set_stopen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        impl core::fmt::Debug for Config {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Config")
                    .field("sleep", &self.sleep())
                    .field("halt", &self.halt())
                    .field("stopen", &self.stopen())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Config {{ sleep: {:?}, halt: {:?}, stopen: {=bool:?} }}",
                    self.sleep(),
                    self.halt(),
                    self.stopen()
                )
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event TIMEOUT"]
            #[must_use]
            #[inline(always)]
            pub const fn timeout(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event TIMEOUT"]
            #[inline(always)]
            pub const fn set_timeout(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to disable interrupt for event STOPPED"]
            #[must_use]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event STOPPED"]
            #[inline(always)]
            pub const fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("timeout", &self.timeout())
                    .field("stopped", &self.stopped())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Int {{ timeout: {=bool:?}, stopped: {=bool:?} }}",
                    self.timeout(),
                    self.stopped()
                )
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Nmi(pub u32);
        impl Nmi {
            #[doc = "Write '1' to disable interrupt for event TIMEOUT"]
            #[must_use]
            #[inline(always)]
            pub const fn timeout(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event TIMEOUT"]
            #[inline(always)]
            pub const fn set_timeout(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to disable interrupt for event STOPPED"]
            #[must_use]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event STOPPED"]
            #[inline(always)]
            pub const fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
        }
        impl Default for Nmi {
            #[inline(always)]
            fn default() -> Nmi {
                Nmi(0)
            }
        }
        impl core::fmt::Debug for Nmi {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Nmi")
                    .field("timeout", &self.timeout())
                    .field("stopped", &self.stopped())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Nmi {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Nmi {{ timeout: {=bool:?}, stopped: {=bool:?} }}",
                    self.timeout(),
                    self.stopped()
                )
            }
        }
        #[doc = "Request status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Reqstatus(pub u32);
        impl Reqstatus {
            #[doc = "Request status for RR\\[0\\] register"]
            #[must_use]
            #[inline(always)]
            pub const fn rr(&self, n: usize) -> bool {
                assert!(n < 8usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Request status for RR\\[0\\] register"]
            #[inline(always)]
            pub const fn set_rr(&mut self, n: usize, val: bool) {
                assert!(n < 8usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Reqstatus {
            #[inline(always)]
            fn default() -> Reqstatus {
                Reqstatus(0)
            }
        }
        impl core::fmt::Debug for Reqstatus {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Reqstatus")
                    .field("rr[0]", &self.rr(0usize))
                    .field("rr[1]", &self.rr(1usize))
                    .field("rr[2]", &self.rr(2usize))
                    .field("rr[3]", &self.rr(3usize))
                    .field("rr[4]", &self.rr(4usize))
                    .field("rr[5]", &self.rr(5usize))
                    .field("rr[6]", &self.rr(6usize))
                    .field("rr[7]", &self.rr(7usize))
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Reqstatus {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Reqstatus {{ rr[0]: {=bool:?}, rr[1]: {=bool:?}, rr[2]: {=bool:?}, rr[3]: {=bool:?}, rr[4]: {=bool:?}, rr[5]: {=bool:?}, rr[6]: {=bool:?}, rr[7]: {=bool:?} }}" , self . rr (0usize) , self . rr (1usize) , self . rr (2usize) , self . rr (3usize) , self . rr (4usize) , self . rr (5usize) , self . rr (6usize) , self . rr (7usize))
            }
        }
        #[doc = "Description collection: Reload request n"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rr(pub u32);
        impl Rr {
            #[doc = "Reload request register"]
            #[must_use]
            #[inline(always)]
            pub const fn rr(&self) -> super::vals::Rr {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Rr::from_bits(val as u32)
            }
            #[doc = "Reload request register"]
            #[inline(always)]
            pub const fn set_rr(&mut self, val: super::vals::Rr) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Rr {
            #[inline(always)]
            fn default() -> Rr {
                Rr(0)
            }
        }
        impl core::fmt::Debug for Rr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rr").field("rr", &self.rr()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Rr {{ rr: {:?} }}", self.rr())
            }
        }
        #[doc = "Enable register for reload request registers"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rren(pub u32);
        impl Rren {
            #[doc = "Enable or disable RR\\[0\\] register"]
            #[must_use]
            #[inline(always)]
            pub const fn rr(&self, n: usize) -> bool {
                assert!(n < 8usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable RR\\[0\\] register"]
            #[inline(always)]
            pub const fn set_rr(&mut self, n: usize, val: bool) {
                assert!(n < 8usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Rren {
            #[inline(always)]
            fn default() -> Rren {
                Rren(0)
            }
        }
        impl core::fmt::Debug for Rren {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rren")
                    .field("rr[0]", &self.rr(0usize))
                    .field("rr[1]", &self.rr(1usize))
                    .field("rr[2]", &self.rr(2usize))
                    .field("rr[3]", &self.rr(3usize))
                    .field("rr[4]", &self.rr(4usize))
                    .field("rr[5]", &self.rr(5usize))
                    .field("rr[6]", &self.rr(6usize))
                    .field("rr[7]", &self.rr(7usize))
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rren {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Rren {{ rr[0]: {=bool:?}, rr[1]: {=bool:?}, rr[2]: {=bool:?}, rr[3]: {=bool:?}, rr[4]: {=bool:?}, rr[5]: {=bool:?}, rr[6]: {=bool:?}, rr[7]: {=bool:?} }}" , self . rr (0usize) , self . rr (1usize) , self . rr (2usize) , self . rr (3usize) , self . rr (4usize) , self . rr (5usize) , self . rr (6usize) , self . rr (7usize))
            }
        }
        #[doc = "Run status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Runstatus(pub u32);
        impl Runstatus {
            #[doc = "Indicates whether or not WDT is running"]
            #[must_use]
            #[inline(always)]
            pub const fn runstatuswdt(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Indicates whether or not WDT is running"]
            #[inline(always)]
            pub const fn set_runstatuswdt(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Runstatus {
            #[inline(always)]
            fn default() -> Runstatus {
                Runstatus(0)
            }
        }
        impl core::fmt::Debug for Runstatus {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Runstatus")
                    .field("runstatuswdt", &self.runstatuswdt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Runstatus {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Runstatus {{ runstatuswdt: {=bool:?} }}",
                    self.runstatuswdt()
                )
            }
        }
        #[doc = "Task stop enable"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Tsen(pub u32);
        impl Tsen {
            #[doc = "Allow stopping WDT"]
            #[must_use]
            #[inline(always)]
            pub const fn tsen(&self) -> super::vals::Tsen {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Tsen::from_bits(val as u32)
            }
            #[doc = "Allow stopping WDT"]
            #[inline(always)]
            pub const fn set_tsen(&mut self, val: super::vals::Tsen) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Tsen {
            #[inline(always)]
            fn default() -> Tsen {
                Tsen(0)
            }
        }
        impl core::fmt::Debug for Tsen {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Tsen").field("tsen", &self.tsen()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Tsen {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Tsen {{ tsen: {:?} }}", self.tsen())
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Halt {
            #[doc = "Pause WDT while the CPU is halted by the debugger"]
            PAUSE = 0x0,
            #[doc = "Keep WDT running while the CPU is halted by the debugger"]
            RUN = 0x01,
        }
        impl Halt {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Halt {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Halt {
            #[inline(always)]
            fn from(val: u8) -> Halt {
                Halt::from_bits(val)
            }
        }
        impl From<Halt> for u8 {
            #[inline(always)]
            fn from(val: Halt) -> u8 {
                Halt::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Rr(u32);
        impl Rr {
            #[doc = "Value to request a reload of the watchdog timer"]
            pub const RELOAD: Self = Self(0x6e52_4635);
        }
        impl Rr {
            pub const fn from_bits(val: u32) -> Rr {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Rr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x6e52_4635 => f.write_str("RELOAD"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rr {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x6e52_4635 => defmt::write!(f, "RELOAD"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for Rr {
            #[inline(always)]
            fn from(val: u32) -> Rr {
                Rr::from_bits(val)
            }
        }
        impl From<Rr> for u32 {
            #[inline(always)]
            fn from(val: Rr) -> u32 {
                Rr::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Sleep {
            #[doc = "Pause WDT while the CPU is sleeping"]
            PAUSE = 0x0,
            #[doc = "Keep WDT running while the CPU is sleeping"]
            RUN = 0x01,
        }
        impl Sleep {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Sleep {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Sleep {
            #[inline(always)]
            fn from(val: u8) -> Sleep {
                Sleep::from_bits(val)
            }
        }
        impl From<Sleep> for u8 {
            #[inline(always)]
            fn from(val: Sleep) -> u8 {
                Sleep::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Tsen(u32);
        impl Tsen {
            #[doc = "Value to allow stopping WDT"]
            pub const ENABLE: Self = Self(0x6e52_4635);
        }
        impl Tsen {
            pub const fn from_bits(val: u32) -> Tsen {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Tsen {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x6e52_4635 => f.write_str("ENABLE"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Tsen {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x6e52_4635 => defmt::write!(f, "ENABLE"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for Tsen {
            #[inline(always)]
            fn from(val: u32) -> Tsen {
                Tsen::from_bits(val)
            }
        }
        impl From<Tsen> for u32 {
            #[inline(always)]
            fn from(val: Tsen) -> u32 {
                Tsen::to_bits(val)
            }
        }
    }
}
