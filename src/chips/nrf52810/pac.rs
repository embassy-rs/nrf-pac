#![doc = "Peripheral access API (generated using chiptool v0.1.0 (4df74f6 2024-11-11))"]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - POWER_CLOCK"]
    POWER_CLOCK = 0,
    #[doc = "1 - RADIO"]
    RADIO = 1,
    #[doc = "2 - UARTE0_UART0"]
    UARTE0_UART0 = 2,
    #[doc = "3 - TWIM0_TWIS0_TWI0"]
    TWIM0_TWIS0_TWI0 = 3,
    #[doc = "4 - SPIM0_SPIS0_SPI0"]
    SPIM0_SPIS0_SPI0 = 4,
    #[doc = "6 - GPIOTE"]
    GPIOTE = 6,
    #[doc = "7 - SAADC"]
    SAADC = 7,
    #[doc = "8 - TIMER0"]
    TIMER0 = 8,
    #[doc = "9 - TIMER1"]
    TIMER1 = 9,
    #[doc = "10 - TIMER2"]
    TIMER2 = 10,
    #[doc = "11 - RTC0"]
    RTC0 = 11,
    #[doc = "12 - TEMP"]
    TEMP = 12,
    #[doc = "13 - RNG"]
    RNG = 13,
    #[doc = "14 - ECB"]
    ECB = 14,
    #[doc = "15 - CCM_AAR"]
    CCM_AAR = 15,
    #[doc = "16 - WDT"]
    WDT = 16,
    #[doc = "17 - RTC1"]
    RTC1 = 17,
    #[doc = "18 - QDEC"]
    QDEC = 18,
    #[doc = "19 - COMP"]
    COMP = 19,
    #[doc = "20 - SWI0_EGU0"]
    SWI0_EGU0 = 20,
    #[doc = "21 - SWI1_EGU1"]
    SWI1_EGU1 = 21,
    #[doc = "22 - SWI2"]
    SWI2 = 22,
    #[doc = "23 - SWI3"]
    SWI3 = 23,
    #[doc = "24 - SWI4"]
    SWI4 = 24,
    #[doc = "25 - SWI5"]
    SWI5 = 25,
    #[doc = "28 - PWM0"]
    PWM0 = 28,
    #[doc = "29 - PDM"]
    PDM = 29,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors {
    extern "C" {
        fn POWER_CLOCK();
        fn RADIO();
        fn UARTE0_UART0();
        fn TWIM0_TWIS0_TWI0();
        fn SPIM0_SPIS0_SPI0();
        fn GPIOTE();
        fn SAADC();
        fn TIMER0();
        fn TIMER1();
        fn TIMER2();
        fn RTC0();
        fn TEMP();
        fn RNG();
        fn ECB();
        fn CCM_AAR();
        fn WDT();
        fn RTC1();
        fn QDEC();
        fn COMP();
        fn SWI0_EGU0();
        fn SWI1_EGU1();
        fn SWI2();
        fn SWI3();
        fn SWI4();
        fn SWI5();
        fn PWM0();
        fn PDM();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[link_section = ".vector_table.interrupts"]
    #[no_mangle]
    pub static __INTERRUPTS: [Vector; 30] = [
        Vector {
            _handler: POWER_CLOCK,
        },
        Vector { _handler: RADIO },
        Vector {
            _handler: UARTE0_UART0,
        },
        Vector {
            _handler: TWIM0_TWIS0_TWI0,
        },
        Vector {
            _handler: SPIM0_SPIS0_SPI0,
        },
        Vector { _reserved: 0 },
        Vector { _handler: GPIOTE },
        Vector { _handler: SAADC },
        Vector { _handler: TIMER0 },
        Vector { _handler: TIMER1 },
        Vector { _handler: TIMER2 },
        Vector { _handler: RTC0 },
        Vector { _handler: TEMP },
        Vector { _handler: RNG },
        Vector { _handler: ECB },
        Vector { _handler: CCM_AAR },
        Vector { _handler: WDT },
        Vector { _handler: RTC1 },
        Vector { _handler: QDEC },
        Vector { _handler: COMP },
        Vector {
            _handler: SWI0_EGU0,
        },
        Vector {
            _handler: SWI1_EGU1,
        },
        Vector { _handler: SWI2 },
        Vector { _handler: SWI3 },
        Vector { _handler: SWI4 },
        Vector { _handler: SWI5 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: PWM0 },
        Vector { _handler: PDM },
    ];
}
#[doc = "Factory information configuration registers"]
pub const FICR: ficr::Ficr = unsafe { ficr::Ficr::from_ptr(0x1000_0000usize as _) };
#[doc = "User information configuration registers"]
pub const UICR: uicr::Uicr = unsafe { uicr::Uicr::from_ptr(0x1000_1000usize as _) };
#[doc = "Block Protect"]
pub const BPROT: bprot::Bprot = unsafe { bprot::Bprot::from_ptr(0x4000_0000usize as _) };
#[doc = "Only for emulation on devices that support hardened AP-PROTECT."]
pub const APPROTECT: approtect::Approtect =
    unsafe { approtect::Approtect::from_ptr(0x4000_0000usize as _) };
#[doc = "Clock control"]
pub const CLOCK: clock::Clock = unsafe { clock::Clock::from_ptr(0x4000_0000usize as _) };
#[doc = "Power control"]
pub const POWER: power::Power = unsafe { power::Power::from_ptr(0x4000_0000usize as _) };
#[doc = "2.4 GHz Radio"]
pub const RADIO: radio::Radio = unsafe { radio::Radio::from_ptr(0x4000_1000usize as _) };
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub const UART0: uart::Uart = unsafe { uart::Uart::from_ptr(0x4000_2000usize as _) };
#[doc = "UART with EasyDMA"]
pub const UARTE0: uarte::Uarte = unsafe { uarte::Uarte::from_ptr(0x4000_2000usize as _) };
#[doc = "I2C compatible Two-Wire Interface"]
pub const TWI0: twi::Twi = unsafe { twi::Twi::from_ptr(0x4000_3000usize as _) };
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA"]
pub const TWIM0: twim::Twim = unsafe { twim::Twim::from_ptr(0x4000_3000usize as _) };
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA"]
pub const TWIS0: twis::Twis = unsafe { twis::Twis::from_ptr(0x4000_3000usize as _) };
#[doc = "Serial Peripheral Interface"]
pub const SPI0: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_4000usize as _) };
#[doc = "Serial Peripheral Interface Master with EasyDMA"]
pub const SPIM0: spim::Spim = unsafe { spim::Spim::from_ptr(0x4000_4000usize as _) };
#[doc = "SPI Slave"]
pub const SPIS0: spis::Spis = unsafe { spis::Spis::from_ptr(0x4000_4000usize as _) };
#[doc = "GPIO Tasks and Events"]
pub const GPIOTE: gpiote::Gpiote = unsafe { gpiote::Gpiote::from_ptr(0x4000_6000usize as _) };
#[doc = "Analog to Digital Converter"]
pub const SAADC: saadc::Saadc = unsafe { saadc::Saadc::from_ptr(0x4000_7000usize as _) };
#[doc = "Timer/Counter 0"]
pub const TIMER0: timer::Timer = unsafe { timer::Timer::from_ptr(0x4000_8000usize as _) };
#[doc = "Timer/Counter 1"]
pub const TIMER1: timer::Timer = unsafe { timer::Timer::from_ptr(0x4000_9000usize as _) };
#[doc = "Timer/Counter 2"]
pub const TIMER2: timer::Timer = unsafe { timer::Timer::from_ptr(0x4000_a000usize as _) };
#[doc = "Real time counter 0"]
pub const RTC0: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4000_b000usize as _) };
#[doc = "Temperature Sensor"]
pub const TEMP: temp::Temp = unsafe { temp::Temp::from_ptr(0x4000_c000usize as _) };
#[doc = "Random Number Generator"]
pub const RNG: rng::Rng = unsafe { rng::Rng::from_ptr(0x4000_d000usize as _) };
#[doc = "AES ECB Mode Encryption"]
pub const ECB: ecb::Ecb = unsafe { ecb::Ecb::from_ptr(0x4000_e000usize as _) };
#[doc = "Accelerated Address Resolver"]
pub const AAR: aar::Aar = unsafe { aar::Aar::from_ptr(0x4000_f000usize as _) };
#[doc = "AES CCM Mode Encryption"]
pub const CCM: ccm::Ccm = unsafe { ccm::Ccm::from_ptr(0x4000_f000usize as _) };
#[doc = "Watchdog Timer"]
pub const WDT: wdt::Wdt = unsafe { wdt::Wdt::from_ptr(0x4001_0000usize as _) };
#[doc = "Real time counter 1"]
pub const RTC1: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4001_1000usize as _) };
#[doc = "Quadrature Decoder"]
pub const QDEC: qdec::Qdec = unsafe { qdec::Qdec::from_ptr(0x4001_2000usize as _) };
#[doc = "Comparator"]
pub const COMP: comp::Comp = unsafe { comp::Comp::from_ptr(0x4001_3000usize as _) };
#[doc = "Event Generator Unit 0"]
pub const EGU0: egu::Egu = unsafe { egu::Egu::from_ptr(0x4001_4000usize as _) };
#[doc = "Software interrupt 0"]
pub const SWI0: swi::Swi = unsafe { swi::Swi::from_ptr(0x4001_4000usize as _) };
#[doc = "Event Generator Unit 1"]
pub const EGU1: egu::Egu = unsafe { egu::Egu::from_ptr(0x4001_5000usize as _) };
#[doc = "Software interrupt 1"]
pub const SWI1: swi::Swi = unsafe { swi::Swi::from_ptr(0x4001_5000usize as _) };
#[doc = "Software interrupt 2"]
pub const SWI2: swi::Swi = unsafe { swi::Swi::from_ptr(0x4001_6000usize as _) };
#[doc = "Software interrupt 3"]
pub const SWI3: swi::Swi = unsafe { swi::Swi::from_ptr(0x4001_7000usize as _) };
#[doc = "Software interrupt 4"]
pub const SWI4: swi::Swi = unsafe { swi::Swi::from_ptr(0x4001_8000usize as _) };
#[doc = "Software interrupt 5"]
pub const SWI5: swi::Swi = unsafe { swi::Swi::from_ptr(0x4001_9000usize as _) };
#[doc = "Pulse width modulation unit"]
pub const PWM0: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x4001_c000usize as _) };
#[doc = "Pulse Density Modulation (Digital Microphone) Interface"]
pub const PDM: pdm::Pdm = unsafe { pdm::Pdm::from_ptr(0x4001_d000usize as _) };
#[doc = "Non-volatile memory controller"]
pub const NVMC: nvmc::Nvmc = unsafe { nvmc::Nvmc::from_ptr(0x4001_e000usize as _) };
#[doc = "Programmable Peripheral Interconnect"]
pub const PPI: ppi::Ppi = unsafe { ppi::Ppi::from_ptr(0x4001_f000usize as _) };
#[doc = "GPIO Port"]
pub const P0: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5000_0000usize as _) };
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop resolving addresses"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Address resolution procedure complete"]
        #[inline(always)]
        pub const fn events_end(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Address resolved"]
        #[inline(always)]
        pub const fn events_resolved(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Address not resolved"]
        #[inline(always)]
        pub const fn events_notresolved(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Resolution status"]
        #[inline(always)]
        pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "Enable AAR"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Number of IRKs"]
        #[inline(always)]
        pub const fn nirk(self) -> crate::common::Reg<regs::Nirk, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Pointer to IRK data structure"]
        #[inline(always)]
        pub const fn irkptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Pointer to the resolvable address"]
        #[inline(always)]
        pub const fn addrptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "Pointer to data area used for temporary storage"]
        #[inline(always)]
        pub const fn scratchptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Enable AAR"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable AAR"]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable AAR"]
            #[inline(always)]
            pub fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Write '1' to disable interrupt for event END"]
            #[inline(always)]
            pub const fn end(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event END"]
            #[inline(always)]
            pub fn set_end(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to disable interrupt for event RESOLVED"]
            #[inline(always)]
            pub const fn resolved(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event RESOLVED"]
            #[inline(always)]
            pub fn set_resolved(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to disable interrupt for event NOTRESOLVED"]
            #[inline(always)]
            pub const fn notresolved(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event NOTRESOLVED"]
            #[inline(always)]
            pub fn set_notresolved(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
        #[doc = "Number of IRKs"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Nirk(pub u32);
        impl Nirk {
            #[doc = "Number of Identity root keys available in the IRK data structure"]
            #[inline(always)]
            pub const fn nirk(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x1f;
                val as u8
            }
            #[doc = "Number of Identity root keys available in the IRK data structure"]
            #[inline(always)]
            pub fn set_nirk(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
            }
        }
        impl Default for Nirk {
            #[inline(always)]
            fn default() -> Nirk {
                Nirk(0)
            }
        }
        #[doc = "Resolution status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Status(pub u32);
        impl Status {
            #[doc = "The IRK that was used last time an address was resolved"]
            #[inline(always)]
            pub const fn status(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "The IRK that was used last time an address was resolved"]
            #[inline(always)]
            pub fn set_status(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Status {
            #[inline(always)]
            fn default() -> Status {
                Status(0)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
pub mod approtect {
    #[doc = "Only for emulation on devices that support hardened AP-PROTECT."]
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
        #[doc = "Software force enable APPROTECT mechanism until next reset."]
        #[inline(always)]
        pub const fn forceprotect(
            self,
        ) -> crate::common::Reg<regs::Forceprotect, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0550usize) as _) }
        }
        #[doc = "Software disable APPROTECT mechanism"]
        #[inline(always)]
        pub const fn disable(self) -> crate::common::Reg<regs::Disable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0558usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Software disable APPROTECT mechanism"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Disable(pub u32);
        impl Disable {
            #[doc = "Software disable APPROTECT mechanism"]
            #[inline(always)]
            pub const fn disable(&self) -> super::vals::Disable {
                let val = (self.0 >> 0usize) & 0xff;
                super::vals::Disable::from_bits(val as u8)
            }
            #[doc = "Software disable APPROTECT mechanism"]
            #[inline(always)]
            pub fn set_disable(&mut self, val: super::vals::Disable) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Disable {
            #[inline(always)]
            fn default() -> Disable {
                Disable(0)
            }
        }
        #[doc = "Software force enable APPROTECT mechanism until next reset."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Forceprotect(pub u32);
        impl Forceprotect {
            #[doc = "Write 0x0 to force enable APPROTECT mechanism"]
            #[inline(always)]
            pub const fn forceprotect(&self) -> super::vals::Forceprotect {
                let val = (self.0 >> 0usize) & 0xff;
                super::vals::Forceprotect::from_bits(val as u8)
            }
            #[doc = "Write 0x0 to force enable APPROTECT mechanism"]
            #[inline(always)]
            pub fn set_forceprotect(&mut self, val: super::vals::Forceprotect) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Forceprotect {
            #[inline(always)]
            fn default() -> Forceprotect {
                Forceprotect(0)
            }
        }
    }
    pub mod vals {
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Disable(pub u8);
        impl Disable {
            #[doc = "Software disable APPROTECT mechanism"]
            pub const SW_DISABLE: Self = Self(0x5a);
        }
        impl Disable {
            pub const fn from_bits(val: u8) -> Disable {
                Self(val & 0xff)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl From<u8> for Disable {
            #[inline(always)]
            fn from(val: u8) -> Disable {
                Disable::from_bits(val)
            }
        }
        impl From<Disable> for u8 {
            #[inline(always)]
            fn from(val: Disable) -> u8 {
                Disable::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Forceprotect(pub u8);
        impl Forceprotect {
            #[doc = "Software force enable APPROTECT mechanism"]
            pub const FORCE: Self = Self(0x0);
        }
        impl Forceprotect {
            pub const fn from_bits(val: u8) -> Forceprotect {
                Self(val & 0xff)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl From<u8> for Forceprotect {
            #[inline(always)]
            fn from(val: u8) -> Forceprotect {
                Forceprotect::from_bits(val)
            }
        }
        impl From<Forceprotect> for u8 {
            #[inline(always)]
            fn from(val: Forceprotect) -> u8 {
                Forceprotect::to_bits(val)
            }
        }
    }
}
pub mod bprot {
    #[doc = "Block Protect"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bprot {
        ptr: *mut u8,
    }
    unsafe impl Send for Bprot {}
    unsafe impl Sync for Bprot {}
    impl Bprot {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Block protect configuration register 0"]
        #[inline(always)]
        pub const fn config0(self) -> crate::common::Reg<regs::Config0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize) as _) }
        }
        #[doc = "Block protect configuration register 1"]
        #[inline(always)]
        pub const fn config1(self) -> crate::common::Reg<regs::Config1, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0604usize) as _) }
        }
        #[doc = "Disable protection mechanism in debug mode"]
        #[inline(always)]
        pub const fn disableindebug(
            self,
        ) -> crate::common::Reg<regs::Disableindebug, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0608usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Block protect configuration register 0"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config0(pub u32);
        impl Config0 {
            #[doc = "Enable protection for region 0. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 0. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable protection for region 1. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 1. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable protection for region 2. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 2. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable protection for region 3. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 3. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable protection for region 4. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region4(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 4. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable protection for region 5. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region5(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 5. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable protection for region 6. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region6(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 6. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region6(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Enable protection for region 7. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region7(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 7. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region7(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Enable protection for region 8. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region8(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 8. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region8(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Enable protection for region 9. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region9(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 9. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region9(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Enable protection for region 10. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region10(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 10. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region10(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Enable protection for region 11. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region11(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 11. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region11(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "Enable protection for region 12. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region12(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 12. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region12(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "Enable protection for region 13. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region13(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 13. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region13(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Enable protection for region 14. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region14(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 14. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region14(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Enable protection for region 15. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region15(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 15. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region15(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
            #[doc = "Enable protection for region 16. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region16(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 16. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region16(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Enable protection for region 17. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region17(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 17. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region17(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Enable protection for region 18. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region18(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 18. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region18(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Enable protection for region 19. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region19(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 19. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region19(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "Enable protection for region 20. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region20(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 20. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region20(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "Enable protection for region 21. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region21(&self) -> bool {
                let val = (self.0 >> 21usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 21. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region21(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
            }
            #[doc = "Enable protection for region 22. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region22(&self) -> bool {
                let val = (self.0 >> 22usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 22. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region22(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
            }
            #[doc = "Enable protection for region 23. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region23(&self) -> bool {
                let val = (self.0 >> 23usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 23. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region23(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
            }
            #[doc = "Enable protection for region 24. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region24(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 24. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region24(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
            #[doc = "Enable protection for region 25. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region25(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 25. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region25(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
            #[doc = "Enable protection for region 26. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region26(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 26. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region26(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
            #[doc = "Enable protection for region 27. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region27(&self) -> bool {
                let val = (self.0 >> 27usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 27. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region27(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
            }
            #[doc = "Enable protection for region 28. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region28(&self) -> bool {
                let val = (self.0 >> 28usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 28. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region28(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
            }
            #[doc = "Enable protection for region 29. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region29(&self) -> bool {
                let val = (self.0 >> 29usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 29. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region29(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
            }
            #[doc = "Enable protection for region 30. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region30(&self) -> bool {
                let val = (self.0 >> 30usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 30. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region30(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
            }
            #[doc = "Enable protection for region 31. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region31(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 31. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region31(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
            }
        }
        impl Default for Config0 {
            #[inline(always)]
            fn default() -> Config0 {
                Config0(0)
            }
        }
        #[doc = "Block protect configuration register 1"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config1(pub u32);
        impl Config1 {
            #[doc = "Enable protection for region 32. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region32(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 32. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region32(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable protection for region 33. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region33(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 33. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region33(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable protection for region 34. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region34(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 34. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region34(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable protection for region 35. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region35(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 35. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region35(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable protection for region 36. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region36(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 36. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region36(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable protection for region 37. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region37(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 37. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region37(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable protection for region 38. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region38(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 38. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region38(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Enable protection for region 39. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region39(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 39. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region39(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Enable protection for region 40. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region40(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 40. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region40(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Enable protection for region 41. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region41(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 41. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region41(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Enable protection for region 42. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region42(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 42. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region42(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Enable protection for region 43. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region43(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 43. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region43(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "Enable protection for region 44. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region44(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 44. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region44(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "Enable protection for region 45. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region45(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 45. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region45(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Enable protection for region 46. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region46(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 46. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region46(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Enable protection for region 47. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region47(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 47. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region47(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
        }
        impl Default for Config1 {
            #[inline(always)]
            fn default() -> Config1 {
                Config1(0)
            }
        }
        #[doc = "Disable protection mechanism in debug mode"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Disableindebug(pub u32);
        impl Disableindebug {
            #[doc = "Disable the protection mechanism for NVM regions while in debug mode. This register will only disable the protection mechanism if the device is in debug mode."]
            #[inline(always)]
            pub const fn disableindebug(&self) -> super::vals::Disableindebug {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Disableindebug::from_bits(val as u8)
            }
            #[doc = "Disable the protection mechanism for NVM regions while in debug mode. This register will only disable the protection mechanism if the device is in debug mode."]
            #[inline(always)]
            pub fn set_disableindebug(&mut self, val: super::vals::Disableindebug) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Disableindebug {
            #[inline(always)]
            fn default() -> Disableindebug {
                Disableindebug(0)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Disableindebug {
            #[doc = "Enabled in debug"]
            ENABLED = 0x0,
            #[doc = "Disabled in debug"]
            DISABLED = 0x01,
        }
        impl Disableindebug {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Disableindebug {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Disableindebug {
            #[inline(always)]
            fn from(val: u8) -> Disableindebug {
                Disableindebug::from_bits(val)
            }
        }
        impl From<Disableindebug> for u8 {
            #[inline(always)]
            fn from(val: Disableindebug) -> u8 {
                Disableindebug::to_bits(val)
            }
        }
    }
}
pub mod ccm {
    #[doc = "AES CCM Mode Encryption"]
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
        #[doc = "Start generation of key-stream. This operation will stop by itself when completed."]
        #[inline(always)]
        pub const fn tasks_ksgen(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Start encryption/decryption. This operation will stop by itself when completed."]
        #[inline(always)]
        pub const fn tasks_crypt(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Stop encryption/decryption"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Override DATARATE setting in MODE register with the contents of the RATEOVERRIDE register for any ongoing encryption/decryption"]
        #[inline(always)]
        pub const fn tasks_rateoverride(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Key-stream generation complete"]
        #[inline(always)]
        pub const fn events_endksgen(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Encrypt/decrypt complete"]
        #[inline(always)]
        pub const fn events_endcrypt(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Deprecated register - CCM error event"]
        #[inline(always)]
        pub const fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "MIC check result"]
        #[inline(always)]
        pub const fn micstatus(self) -> crate::common::Reg<regs::Micstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "Enable"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Operation mode"]
        #[inline(always)]
        pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Pointer to data structure holding AES key and NONCE vector"]
        #[inline(always)]
        pub const fn cnfptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Input pointer"]
        #[inline(always)]
        pub const fn inptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Output pointer"]
        #[inline(always)]
        pub const fn outptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "Pointer to data area used for temporary storage"]
        #[inline(always)]
        pub const fn scratchptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
        }
        #[doc = "Length of key-stream generated when MODE.LENGTH = Extended."]
        #[inline(always)]
        pub const fn maxpacketsize(
            self,
        ) -> crate::common::Reg<regs::Maxpacketsize, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
        #[doc = "Data rate override setting."]
        #[inline(always)]
        pub const fn rateoverride(
            self,
        ) -> crate::common::Reg<regs::Rateoverride, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Enable"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable CCM"]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable CCM"]
            #[inline(always)]
            pub fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Write '1' to disable interrupt for event ENDKSGEN"]
            #[inline(always)]
            pub const fn endksgen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ENDKSGEN"]
            #[inline(always)]
            pub fn set_endksgen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to disable interrupt for event ENDCRYPT"]
            #[inline(always)]
            pub const fn endcrypt(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ENDCRYPT"]
            #[inline(always)]
            pub fn set_endcrypt(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Deprecated intclrfield - Write '1' to disable interrupt for event ERROR"]
            #[inline(always)]
            pub const fn error(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Deprecated intclrfield - Write '1' to disable interrupt for event ERROR"]
            #[inline(always)]
            pub fn set_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
        #[doc = "Length of key-stream generated when MODE.LENGTH = Extended."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Maxpacketsize(pub u32);
        impl Maxpacketsize {
            #[doc = "Length of key-stream generated when MODE.LENGTH = Extended. This value must be greater or equal to the subsequent packet payload to be encrypted/decrypted."]
            #[inline(always)]
            pub const fn maxpacketsize(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Length of key-stream generated when MODE.LENGTH = Extended. This value must be greater or equal to the subsequent packet payload to be encrypted/decrypted."]
            #[inline(always)]
            pub fn set_maxpacketsize(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Maxpacketsize {
            #[inline(always)]
            fn default() -> Maxpacketsize {
                Maxpacketsize(0)
            }
        }
        #[doc = "MIC check result"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Micstatus(pub u32);
        impl Micstatus {
            #[doc = "The result of the MIC check performed during the previous decryption operation"]
            #[inline(always)]
            pub const fn micstatus(&self) -> super::vals::Micstatus {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Micstatus::from_bits(val as u8)
            }
            #[doc = "The result of the MIC check performed during the previous decryption operation"]
            #[inline(always)]
            pub fn set_micstatus(&mut self, val: super::vals::Micstatus) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Micstatus {
            #[inline(always)]
            fn default() -> Micstatus {
                Micstatus(0)
            }
        }
        #[doc = "Operation mode"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Mode(pub u32);
        impl Mode {
            #[doc = "The mode of operation to be used. The settings in this register apply whenever either the KSGEN or CRYPT tasks are triggered."]
            #[inline(always)]
            pub const fn mode(&self) -> super::vals::Mode {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Mode::from_bits(val as u8)
            }
            #[doc = "The mode of operation to be used. The settings in this register apply whenever either the KSGEN or CRYPT tasks are triggered."]
            #[inline(always)]
            pub fn set_mode(&mut self, val: super::vals::Mode) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Radio data rate that the CCM shall run synchronous with"]
            #[inline(always)]
            pub const fn datarate(&self) -> super::vals::Datarate {
                let val = (self.0 >> 16usize) & 0x03;
                super::vals::Datarate::from_bits(val as u8)
            }
            #[doc = "Radio data rate that the CCM shall run synchronous with"]
            #[inline(always)]
            pub fn set_datarate(&mut self, val: super::vals::Datarate) {
                self.0 =
                    (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
            }
            #[doc = "Packet length configuration"]
            #[inline(always)]
            pub const fn length(&self) -> super::vals::Length {
                let val = (self.0 >> 24usize) & 0x01;
                super::vals::Length::from_bits(val as u8)
            }
            #[doc = "Packet length configuration"]
            #[inline(always)]
            pub fn set_length(&mut self, val: super::vals::Length) {
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
        #[doc = "Data rate override setting."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rateoverride(pub u32);
        impl Rateoverride {
            #[doc = "Data rate override setting."]
            #[inline(always)]
            pub const fn rateoverride(&self) -> super::vals::Rateoverride {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Rateoverride::from_bits(val as u8)
            }
            #[doc = "Data rate override setting."]
            #[inline(always)]
            pub fn set_rateoverride(&mut self, val: super::vals::Rateoverride) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Rateoverride {
            #[inline(always)]
            fn default() -> Rateoverride {
                Rateoverride(0)
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event ENDKSGEN and task CRYPT"]
            #[inline(always)]
            pub const fn endksgen_crypt(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event ENDKSGEN and task CRYPT"]
            #[inline(always)]
            pub fn set_endksgen_crypt(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Datarate {
            #[doc = "1 Mbps"]
            _1MBIT = 0x0,
            #[doc = "2 Mbps"]
            _2MBIT = 0x01,
            #[doc = "125 Kbps"]
            _125KBPS = 0x02,
            #[doc = "500 Kbps"]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Length {
            #[doc = "Default length. Effective length of LENGTH field in encrypted/decrypted packet is 5 bits. A key-stream for packet payloads up to 27 bytes will be generated."]
            DEFAULT = 0x0,
            #[doc = "Extended length. Effective length of LENGTH field in encrypted/decrypted packet is 8 bits. A key-stream for packet payloads up to MAXPACKETSIZE bytes will be generated."]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Rateoverride {
            #[doc = "1 Mbps"]
            _1MBIT = 0x0,
            #[doc = "2 Mbps"]
            _2MBIT = 0x01,
            #[doc = "125 Kbps"]
            _125KBPS = 0x02,
            #[doc = "500 Kbps"]
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
    #[doc = "Clock control"]
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
        #[doc = "Start HFCLK crystal oscillator"]
        #[inline(always)]
        pub const fn tasks_hfclkstart(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop HFCLK crystal oscillator"]
        #[inline(always)]
        pub const fn tasks_hfclkstop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Start LFCLK source"]
        #[inline(always)]
        pub const fn tasks_lfclkstart(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Stop LFCLK source"]
        #[inline(always)]
        pub const fn tasks_lfclkstop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Start calibration of LFRC oscillator"]
        #[inline(always)]
        pub const fn tasks_cal(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Start calibration timer"]
        #[inline(always)]
        pub const fn tasks_ctstart(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Stop calibration timer"]
        #[inline(always)]
        pub const fn tasks_ctstop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }
        #[doc = "HFCLK oscillator started"]
        #[inline(always)]
        pub const fn events_hfclkstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "LFCLK started"]
        #[inline(always)]
        pub const fn events_lfclkstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Calibration of LFCLK RC oscillator complete event"]
        #[inline(always)]
        pub const fn events_done(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
        }
        #[doc = "Calibration timer timeout"]
        #[inline(always)]
        pub const fn events_ctto(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Status indicating that HFCLKSTART task has been triggered"]
        #[inline(always)]
        pub const fn hfclkrun(self) -> crate::common::Reg<regs::Hfclkrun, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0408usize) as _) }
        }
        #[doc = "HFCLK status"]
        #[inline(always)]
        pub const fn hfclkstat(self) -> crate::common::Reg<regs::Hfclkstat, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x040cusize) as _) }
        }
        #[doc = "Status indicating that LFCLKSTART task has been triggered"]
        #[inline(always)]
        pub const fn lfclkrun(self) -> crate::common::Reg<regs::Lfclkrun, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0414usize) as _) }
        }
        #[doc = "LFCLK status"]
        #[inline(always)]
        pub const fn lfclkstat(self) -> crate::common::Reg<regs::Lfclkstat, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0418usize) as _) }
        }
        #[doc = "Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
        #[inline(always)]
        pub const fn lfclksrccopy(
            self,
        ) -> crate::common::Reg<regs::Lfclksrccopy, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x041cusize) as _) }
        }
        #[doc = "Clock source for the LFCLK"]
        #[inline(always)]
        pub const fn lfclksrc(self) -> crate::common::Reg<regs::Lfclksrc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
        #[doc = "Calibration timer interval"]
        #[inline(always)]
        pub const fn ctiv(self) -> crate::common::Reg<regs::Ctiv, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0538usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Calibration timer interval"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctiv(pub u32);
        impl Ctiv {
            #[doc = "Calibration timer interval in multiple of 0.25 seconds. Range: 0.25 seconds to 31.75 seconds."]
            #[inline(always)]
            pub const fn ctiv(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "Calibration timer interval in multiple of 0.25 seconds. Range: 0.25 seconds to 31.75 seconds."]
            #[inline(always)]
            pub fn set_ctiv(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
            }
        }
        impl Default for Ctiv {
            #[inline(always)]
            fn default() -> Ctiv {
                Ctiv(0)
            }
        }
        #[doc = "Status indicating that HFCLKSTART task has been triggered"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Hfclkrun(pub u32);
        impl Hfclkrun {
            #[doc = "HFCLKSTART task triggered or not"]
            #[inline(always)]
            pub const fn status(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "HFCLKSTART task triggered or not"]
            #[inline(always)]
            pub fn set_status(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Hfclkrun {
            #[inline(always)]
            fn default() -> Hfclkrun {
                Hfclkrun(0)
            }
        }
        #[doc = "HFCLK status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Hfclkstat(pub u32);
        impl Hfclkstat {
            #[doc = "Source of HFCLK"]
            #[inline(always)]
            pub const fn src(&self) -> super::vals::HfclkstatSrc {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::HfclkstatSrc::from_bits(val as u8)
            }
            #[doc = "Source of HFCLK"]
            #[inline(always)]
            pub fn set_src(&mut self, val: super::vals::HfclkstatSrc) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "HFCLK state"]
            #[inline(always)]
            pub const fn state(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "HFCLK state"]
            #[inline(always)]
            pub fn set_state(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
        }
        impl Default for Hfclkstat {
            #[inline(always)]
            fn default() -> Hfclkstat {
                Hfclkstat(0)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Write '1' to disable interrupt for event HFCLKSTARTED"]
            #[inline(always)]
            pub const fn hfclkstarted(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event HFCLKSTARTED"]
            #[inline(always)]
            pub fn set_hfclkstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to disable interrupt for event LFCLKSTARTED"]
            #[inline(always)]
            pub const fn lfclkstarted(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event LFCLKSTARTED"]
            #[inline(always)]
            pub fn set_lfclkstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to disable interrupt for event DONE"]
            #[inline(always)]
            pub const fn done(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event DONE"]
            #[inline(always)]
            pub fn set_done(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Write '1' to disable interrupt for event CTTO"]
            #[inline(always)]
            pub const fn ctto(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event CTTO"]
            #[inline(always)]
            pub fn set_ctto(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
        #[doc = "Status indicating that LFCLKSTART task has been triggered"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Lfclkrun(pub u32);
        impl Lfclkrun {
            #[doc = "LFCLKSTART task triggered or not"]
            #[inline(always)]
            pub const fn status(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "LFCLKSTART task triggered or not"]
            #[inline(always)]
            pub fn set_status(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Lfclkrun {
            #[inline(always)]
            fn default() -> Lfclkrun {
                Lfclkrun(0)
            }
        }
        #[doc = "Clock source for the LFCLK"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Lfclksrc(pub u32);
        impl Lfclksrc {
            #[doc = "Clock source"]
            #[inline(always)]
            pub const fn src(&self) -> super::vals::Lfclksrc {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Lfclksrc::from_bits(val as u8)
            }
            #[doc = "Clock source"]
            #[inline(always)]
            pub fn set_src(&mut self, val: super::vals::Lfclksrc) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
            #[doc = "Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
            #[inline(always)]
            pub const fn bypass(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
            #[inline(always)]
            pub fn set_bypass(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Enable or disable external source for LFCLK"]
            #[inline(always)]
            pub const fn external(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable external source for LFCLK"]
            #[inline(always)]
            pub fn set_external(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
        }
        impl Default for Lfclksrc {
            #[inline(always)]
            fn default() -> Lfclksrc {
                Lfclksrc(0)
            }
        }
        #[doc = "Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Lfclksrccopy(pub u32);
        impl Lfclksrccopy {
            #[doc = "Clock source"]
            #[inline(always)]
            pub const fn src(&self) -> super::vals::Lfclksrc {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Lfclksrc::from_bits(val as u8)
            }
            #[doc = "Clock source"]
            #[inline(always)]
            pub fn set_src(&mut self, val: super::vals::Lfclksrc) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Lfclksrccopy {
            #[inline(always)]
            fn default() -> Lfclksrccopy {
                Lfclksrccopy(0)
            }
        }
        #[doc = "LFCLK status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Lfclkstat(pub u32);
        impl Lfclkstat {
            #[doc = "Source of LFCLK"]
            #[inline(always)]
            pub const fn src(&self) -> super::vals::Lfclksrc {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Lfclksrc::from_bits(val as u8)
            }
            #[doc = "Source of LFCLK"]
            #[inline(always)]
            pub fn set_src(&mut self, val: super::vals::Lfclksrc) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
            #[doc = "LFCLK state"]
            #[inline(always)]
            pub const fn state(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "LFCLK state"]
            #[inline(always)]
            pub fn set_state(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
        }
        impl Default for Lfclkstat {
            #[inline(always)]
            fn default() -> Lfclkstat {
                Lfclkstat(0)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum HfclkstatSrc {
            #[doc = "64 MHz internal oscillator (HFINT)"]
            RC = 0x0,
            #[doc = "64 MHz crystal oscillator (HFXO)"]
            XTAL = 0x01,
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Lfclksrc {
            #[doc = "32.768 kHz RC oscillator"]
            RC = 0x0,
            #[doc = "32.768 kHz crystal oscillator"]
            XTAL = 0x01,
            #[doc = "32.768 kHz synthesized from HFCLK"]
            SYNTH = 0x02,
            _RESERVED_3 = 0x03,
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
        pub fn write<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            let mut val = Default::default();
            let res = f(&mut val);
            self.write_value(val);
            res
        }
    }
    impl<T: Copy, A: Read + Write> Reg<T, A> {
        #[inline(always)]
        pub fn modify<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            let mut val = self.read();
            let res = f(&mut val);
            self.write_value(val);
            res
        }
    }
}
pub mod comp {
    #[doc = "Comparator"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Comp {
        ptr: *mut u8,
    }
    unsafe impl Send for Comp {}
    unsafe impl Sync for Comp {}
    impl Comp {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start comparator"]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop comparator"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Sample comparator value"]
        #[inline(always)]
        pub const fn tasks_sample(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "COMP is ready and output is valid"]
        #[inline(always)]
        pub const fn events_ready(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Downward crossing"]
        #[inline(always)]
        pub const fn events_down(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Upward crossing"]
        #[inline(always)]
        pub const fn events_up(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "Downward or upward crossing"]
        #[inline(always)]
        pub const fn events_cross(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Compare result"]
        #[inline(always)]
        pub const fn result(self) -> crate::common::Reg<regs::Result, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "COMP enable"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Pin select"]
        #[inline(always)]
        pub const fn psel(self) -> crate::common::Reg<regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Reference source select for single-ended mode"]
        #[inline(always)]
        pub const fn refsel(self) -> crate::common::Reg<regs::Refsel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "External reference select"]
        #[inline(always)]
        pub const fn extrefsel(self) -> crate::common::Reg<regs::Extrefsel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Threshold configuration for hysteresis unit"]
        #[inline(always)]
        pub const fn th(self) -> crate::common::Reg<regs::Th, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0530usize) as _) }
        }
        #[doc = "Mode configuration"]
        #[inline(always)]
        pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0534usize) as _) }
        }
        #[doc = "Comparator hysteresis enable"]
        #[inline(always)]
        pub const fn hyst(self) -> crate::common::Reg<regs::Hyst, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0538usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "COMP enable"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable COMP"]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable COMP"]
            #[inline(always)]
            pub fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        #[doc = "External reference select"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Extrefsel(pub u32);
        impl Extrefsel {
            #[doc = "External analog reference select"]
            #[inline(always)]
            pub const fn extrefsel(&self) -> super::vals::Extrefsel {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::Extrefsel::from_bits(val as u8)
            }
            #[doc = "External analog reference select"]
            #[inline(always)]
            pub fn set_extrefsel(&mut self, val: super::vals::Extrefsel) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
            }
        }
        impl Default for Extrefsel {
            #[inline(always)]
            fn default() -> Extrefsel {
                Extrefsel(0)
            }
        }
        #[doc = "Comparator hysteresis enable"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Hyst(pub u32);
        impl Hyst {
            #[doc = "Comparator hysteresis"]
            #[inline(always)]
            pub const fn hyst(&self) -> super::vals::Hyst {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Hyst::from_bits(val as u8)
            }
            #[doc = "Comparator hysteresis"]
            #[inline(always)]
            pub fn set_hyst(&mut self, val: super::vals::Hyst) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Hyst {
            #[inline(always)]
            fn default() -> Hyst {
                Hyst(0)
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Enable or disable interrupt for event READY"]
            #[inline(always)]
            pub const fn ready(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event READY"]
            #[inline(always)]
            pub fn set_ready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable interrupt for event DOWN"]
            #[inline(always)]
            pub const fn down(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event DOWN"]
            #[inline(always)]
            pub fn set_down(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for event UP"]
            #[inline(always)]
            pub const fn up(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event UP"]
            #[inline(always)]
            pub fn set_up(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable or disable interrupt for event CROSS"]
            #[inline(always)]
            pub const fn cross(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event CROSS"]
            #[inline(always)]
            pub fn set_cross(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
        #[doc = "Mode configuration"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Mode(pub u32);
        impl Mode {
            #[doc = "Speed and power modes"]
            #[inline(always)]
            pub const fn sp(&self) -> super::vals::Sp {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Sp::from_bits(val as u8)
            }
            #[doc = "Speed and power modes"]
            #[inline(always)]
            pub fn set_sp(&mut self, val: super::vals::Sp) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
            #[doc = "Main operation modes"]
            #[inline(always)]
            pub const fn main(&self) -> super::vals::Main {
                let val = (self.0 >> 8usize) & 0x01;
                super::vals::Main::from_bits(val as u8)
            }
            #[doc = "Main operation modes"]
            #[inline(always)]
            pub fn set_main(&mut self, val: super::vals::Main) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
            }
        }
        impl Default for Mode {
            #[inline(always)]
            fn default() -> Mode {
                Mode(0)
            }
        }
        #[doc = "Pin select"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Psel(pub u32);
        impl Psel {
            #[doc = "Analog pin select"]
            #[inline(always)]
            pub const fn psel(&self) -> super::vals::PselPsel {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::PselPsel::from_bits(val as u8)
            }
            #[doc = "Analog pin select"]
            #[inline(always)]
            pub fn set_psel(&mut self, val: super::vals::PselPsel) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
            }
        }
        impl Default for Psel {
            #[inline(always)]
            fn default() -> Psel {
                Psel(0)
            }
        }
        #[doc = "Reference source select for single-ended mode"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Refsel(pub u32);
        impl Refsel {
            #[doc = "Reference select"]
            #[inline(always)]
            pub const fn refsel(&self) -> super::vals::Refsel {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::Refsel::from_bits(val as u8)
            }
            #[doc = "Reference select"]
            #[inline(always)]
            pub fn set_refsel(&mut self, val: super::vals::Refsel) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
            }
        }
        impl Default for Refsel {
            #[inline(always)]
            fn default() -> Refsel {
                Refsel(0)
            }
        }
        #[doc = "Compare result"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Result(pub u32);
        impl Result {
            #[doc = "Result of last compare. Decision point SAMPLE task."]
            #[inline(always)]
            pub const fn result(&self) -> super::vals::Result {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Result::from_bits(val as u8)
            }
            #[doc = "Result of last compare. Decision point SAMPLE task."]
            #[inline(always)]
            pub fn set_result(&mut self, val: super::vals::Result) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Result {
            #[inline(always)]
            fn default() -> Result {
                Result(0)
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event READY and task SAMPLE"]
            #[inline(always)]
            pub const fn ready_sample(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event READY and task SAMPLE"]
            #[inline(always)]
            pub fn set_ready_sample(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Shortcut between event READY and task STOP"]
            #[inline(always)]
            pub const fn ready_stop(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event READY and task STOP"]
            #[inline(always)]
            pub fn set_ready_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Shortcut between event DOWN and task STOP"]
            #[inline(always)]
            pub const fn down_stop(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event DOWN and task STOP"]
            #[inline(always)]
            pub fn set_down_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Shortcut between event UP and task STOP"]
            #[inline(always)]
            pub const fn up_stop(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event UP and task STOP"]
            #[inline(always)]
            pub fn set_up_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Shortcut between event CROSS and task STOP"]
            #[inline(always)]
            pub const fn cross_stop(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event CROSS and task STOP"]
            #[inline(always)]
            pub fn set_cross_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        #[doc = "Threshold configuration for hysteresis unit"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Th(pub u32);
        impl Th {
            #[doc = "VDOWN = (THDOWN+1)/64*VREF"]
            #[inline(always)]
            pub const fn thdown(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x3f;
                val as u8
            }
            #[doc = "VDOWN = (THDOWN+1)/64*VREF"]
            #[inline(always)]
            pub fn set_thdown(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
            }
            #[doc = "VUP = (THUP+1)/64*VREF"]
            #[inline(always)]
            pub const fn thup(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0x3f;
                val as u8
            }
            #[doc = "VUP = (THUP+1)/64*VREF"]
            #[inline(always)]
            pub fn set_thup(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
            }
        }
        impl Default for Th {
            #[inline(always)]
            fn default() -> Th {
                Th(0)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Extrefsel {
            #[doc = "Use AIN0 as external analog reference"]
            ANALOG_REFERENCE0 = 0x0,
            #[doc = "Use AIN1 as external analog reference"]
            ANALOG_REFERENCE1 = 0x01,
            #[doc = "Use AIN2 as external analog reference"]
            ANALOG_REFERENCE2 = 0x02,
            #[doc = "Use AIN3 as external analog reference"]
            ANALOG_REFERENCE3 = 0x03,
            #[doc = "Use AIN4 as external analog reference"]
            ANALOG_REFERENCE4 = 0x04,
            #[doc = "Use AIN5 as external analog reference"]
            ANALOG_REFERENCE5 = 0x05,
            #[doc = "Use AIN6 as external analog reference"]
            ANALOG_REFERENCE6 = 0x06,
            #[doc = "Use AIN7 as external analog reference"]
            ANALOG_REFERENCE7 = 0x07,
        }
        impl Extrefsel {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Extrefsel {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Extrefsel {
            #[inline(always)]
            fn from(val: u8) -> Extrefsel {
                Extrefsel::from_bits(val)
            }
        }
        impl From<Extrefsel> for u8 {
            #[inline(always)]
            fn from(val: Extrefsel) -> u8 {
                Extrefsel::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Hyst {
            #[doc = "Comparator hysteresis disabled"]
            NO_HYST = 0x0,
            #[doc = "Comparator hysteresis enabled"]
            HYST50M_V = 0x01,
        }
        impl Hyst {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Hyst {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Hyst {
            #[inline(always)]
            fn from(val: u8) -> Hyst {
                Hyst::from_bits(val)
            }
        }
        impl From<Hyst> for u8 {
            #[inline(always)]
            fn from(val: Hyst) -> u8 {
                Hyst::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Main {
            #[doc = "Single-ended mode"]
            SE = 0x0,
            #[doc = "Differential mode"]
            DIFF = 0x01,
        }
        impl Main {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Main {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Main {
            #[inline(always)]
            fn from(val: u8) -> Main {
                Main::from_bits(val)
            }
        }
        impl From<Main> for u8 {
            #[inline(always)]
            fn from(val: Main) -> u8 {
                Main::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum PselPsel {
            #[doc = "AIN0 selected as analog input"]
            ANALOG_INPUT0 = 0x0,
            #[doc = "AIN1 selected as analog input"]
            ANALOG_INPUT1 = 0x01,
            #[doc = "AIN2 selected as analog input"]
            ANALOG_INPUT2 = 0x02,
            #[doc = "AIN3 selected as analog input"]
            ANALOG_INPUT3 = 0x03,
            #[doc = "AIN4 selected as analog input"]
            ANALOG_INPUT4 = 0x04,
            #[doc = "AIN5 selected as analog input"]
            ANALOG_INPUT5 = 0x05,
            #[doc = "AIN6 selected as analog input"]
            ANALOG_INPUT6 = 0x06,
            #[doc = "VDD/2 selected as analog input"]
            VDD_DIV2 = 0x07,
        }
        impl PselPsel {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> PselPsel {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for PselPsel {
            #[inline(always)]
            fn from(val: u8) -> PselPsel {
                PselPsel::from_bits(val)
            }
        }
        impl From<PselPsel> for u8 {
            #[inline(always)]
            fn from(val: PselPsel) -> u8 {
                PselPsel::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Refsel {
            #[doc = "VREF = internal 1.2 V reference (VDD &gt;= 1.7 V)"]
            INT1V2 = 0x0,
            #[doc = "VREF = internal 1.8 V reference (VDD &gt;= VREF + 0.2 V)"]
            INT1V8 = 0x01,
            #[doc = "VREF = internal 2.4 V reference (VDD &gt;= VREF + 0.2 V)"]
            INT2V4 = 0x02,
            _RESERVED_3 = 0x03,
            #[doc = "VREF = VDD"]
            VDD = 0x04,
            #[doc = "VREF = AREF (VDD &gt;= VREF &gt;= AREFMIN)"]
            AREF = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl Refsel {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Refsel {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Refsel {
            #[inline(always)]
            fn from(val: u8) -> Refsel {
                Refsel::from_bits(val)
            }
        }
        impl From<Refsel> for u8 {
            #[inline(always)]
            fn from(val: Refsel) -> u8 {
                Refsel::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Result {
            #[doc = "Input voltage is below the threshold (VIN+ &lt; VIN-)"]
            BELOW = 0x0,
            #[doc = "Input voltage is above the threshold (VIN+ &gt; VIN-)"]
            ABOVE = 0x01,
        }
        impl Result {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Result {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Result {
            #[inline(always)]
            fn from(val: u8) -> Result {
                Result::from_bits(val)
            }
        }
        impl From<Result> for u8 {
            #[inline(always)]
            fn from(val: Result) -> u8 {
                Result::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Sp {
            #[doc = "Low-power mode"]
            LOW = 0x0,
            #[doc = "Normal mode"]
            NORMAL = 0x01,
            #[doc = "High-speed mode"]
            HIGH = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Sp {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Sp {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Sp {
            #[inline(always)]
            fn from(val: u8) -> Sp {
                Sp::from_bits(val)
            }
        }
        impl From<Sp> for u8 {
            #[inline(always)]
            fn from(val: Sp) -> u8 {
                Sp::to_bits(val)
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Abort a possible executing ECB operation"]
        #[inline(always)]
        pub const fn tasks_stopecb(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "ECB block encrypt complete"]
        #[inline(always)]
        pub const fn events_endecb(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "ECB block encrypt aborted because of a STOPECB task or due to an error"]
        #[inline(always)]
        pub const fn events_errorecb(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "ECB block encrypt memory pointers"]
        #[inline(always)]
        pub const fn ecbdataptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Write '1' to disable interrupt for event ENDECB"]
            #[inline(always)]
            pub const fn endecb(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ENDECB"]
            #[inline(always)]
            pub fn set_endecb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to disable interrupt for event ERRORECB"]
            #[inline(always)]
            pub const fn errorecb(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ERRORECB"]
            #[inline(always)]
            pub fn set_errorecb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
    }
}
pub mod egu {
    #[doc = "Event Generator Unit 0"]
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Event number n generated by triggering the corresponding TRIGGER\\[n\\] task"]
        #[inline(always)]
        pub const fn events_triggered(
            self,
            n: usize,
        ) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 16usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Enable or disable interrupt for event TRIGGERED\\[0\\]"]
            #[inline(always)]
            pub const fn triggered(&self, n: usize) -> bool {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event TRIGGERED\\[0\\]"]
            #[inline(always)]
            pub fn set_triggered(&mut self, n: usize, val: bool) {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
    }
}
pub mod ficr {
    #[doc = "Factory information configuration registers"]
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
        #[doc = "Code memory page size"]
        #[inline(always)]
        pub const fn codepagesize(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Code memory size"]
        #[inline(always)]
        pub const fn codesize(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Description collection: Device identifier"]
        #[inline(always)]
        pub const fn deviceid(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
            assert!(n < 2usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Encryption root, word n"]
        #[inline(always)]
        pub const fn er(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Identity root, word n"]
        #[inline(always)]
        pub const fn ir(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize + n * 4usize) as _) }
        }
        #[doc = "Device address type"]
        #[inline(always)]
        pub const fn deviceaddrtype(
            self,
        ) -> crate::common::Reg<regs::Deviceaddrtype, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
        }
        #[doc = "Description collection: Device address n"]
        #[inline(always)]
        pub const fn deviceaddr(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
            assert!(n < 2usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize + n * 4usize) as _) }
        }
        #[doc = "Device info"]
        #[inline(always)]
        pub const fn info(self) -> Info {
            unsafe { Info::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Registers storing factory TEMP module linearization coefficients"]
        #[inline(always)]
        pub const fn temp(self) -> Temp {
            unsafe { Temp::from_ptr(self.ptr.add(0x0404usize) as _) }
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
        #[doc = "Part code"]
        #[inline(always)]
        pub const fn part(self) -> crate::common::Reg<regs::Part, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Part variant, hardware version and production configuration"]
        #[inline(always)]
        pub const fn variant(self) -> crate::common::Reg<regs::Variant, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Package option"]
        #[inline(always)]
        pub const fn package(self) -> crate::common::Reg<regs::Package, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "RAM variant"]
        #[inline(always)]
        pub const fn ram(self) -> crate::common::Reg<regs::Ram, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Flash variant"]
        #[inline(always)]
        pub const fn flash(self) -> crate::common::Reg<regs::Flash, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
    }
    #[doc = "Registers storing factory TEMP module linearization coefficients"]
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
        #[doc = "Slope definition A0"]
        #[inline(always)]
        pub const fn a0(self) -> crate::common::Reg<regs::A0, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Slope definition A1"]
        #[inline(always)]
        pub const fn a1(self) -> crate::common::Reg<regs::A1, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Slope definition A2"]
        #[inline(always)]
        pub const fn a2(self) -> crate::common::Reg<regs::A2, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Slope definition A3"]
        #[inline(always)]
        pub const fn a3(self) -> crate::common::Reg<regs::A3, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Slope definition A4"]
        #[inline(always)]
        pub const fn a4(self) -> crate::common::Reg<regs::A4, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Slope definition A5"]
        #[inline(always)]
        pub const fn a5(self) -> crate::common::Reg<regs::A5, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Y-intercept B0"]
        #[inline(always)]
        pub const fn b0(self) -> crate::common::Reg<regs::B0, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }
        #[doc = "Y-intercept B1"]
        #[inline(always)]
        pub const fn b1(self) -> crate::common::Reg<regs::B1, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "Y-intercept B2"]
        #[inline(always)]
        pub const fn b2(self) -> crate::common::Reg<regs::B2, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "Y-intercept B3"]
        #[inline(always)]
        pub const fn b3(self) -> crate::common::Reg<regs::B3, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
        }
        #[doc = "Y-intercept B4"]
        #[inline(always)]
        pub const fn b4(self) -> crate::common::Reg<regs::B4, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
        }
        #[doc = "Y-intercept B5"]
        #[inline(always)]
        pub const fn b5(self) -> crate::common::Reg<regs::B5, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
        }
        #[doc = "Segment end T0"]
        #[inline(always)]
        pub const fn t0(self) -> crate::common::Reg<regs::T0, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
        }
        #[doc = "Segment end T1"]
        #[inline(always)]
        pub const fn t1(self) -> crate::common::Reg<regs::T1, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
        }
        #[doc = "Segment end T2"]
        #[inline(always)]
        pub const fn t2(self) -> crate::common::Reg<regs::T2, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
        }
        #[doc = "Segment end T3"]
        #[inline(always)]
        pub const fn t3(self) -> crate::common::Reg<regs::T3, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
        }
        #[doc = "Segment end T4"]
        #[inline(always)]
        pub const fn t4(self) -> crate::common::Reg<regs::T4, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Slope definition A0"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct A0(pub u32);
        impl A0 {
            #[doc = "A (slope definition) register"]
            #[inline(always)]
            pub const fn a(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "A (slope definition) register"]
            #[inline(always)]
            pub fn set_a(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
        }
        impl Default for A0 {
            #[inline(always)]
            fn default() -> A0 {
                A0(0)
            }
        }
        #[doc = "Slope definition A1"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct A1(pub u32);
        impl A1 {
            #[doc = "A (slope definition) register"]
            #[inline(always)]
            pub const fn a(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "A (slope definition) register"]
            #[inline(always)]
            pub fn set_a(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
        }
        impl Default for A1 {
            #[inline(always)]
            fn default() -> A1 {
                A1(0)
            }
        }
        #[doc = "Slope definition A2"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct A2(pub u32);
        impl A2 {
            #[doc = "A (slope definition) register"]
            #[inline(always)]
            pub const fn a(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "A (slope definition) register"]
            #[inline(always)]
            pub fn set_a(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
        }
        impl Default for A2 {
            #[inline(always)]
            fn default() -> A2 {
                A2(0)
            }
        }
        #[doc = "Slope definition A3"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct A3(pub u32);
        impl A3 {
            #[doc = "A (slope definition) register"]
            #[inline(always)]
            pub const fn a(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "A (slope definition) register"]
            #[inline(always)]
            pub fn set_a(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
        }
        impl Default for A3 {
            #[inline(always)]
            fn default() -> A3 {
                A3(0)
            }
        }
        #[doc = "Slope definition A4"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct A4(pub u32);
        impl A4 {
            #[doc = "A (slope definition) register"]
            #[inline(always)]
            pub const fn a(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "A (slope definition) register"]
            #[inline(always)]
            pub fn set_a(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
        }
        impl Default for A4 {
            #[inline(always)]
            fn default() -> A4 {
                A4(0)
            }
        }
        #[doc = "Slope definition A5"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct A5(pub u32);
        impl A5 {
            #[doc = "A (slope definition) register"]
            #[inline(always)]
            pub const fn a(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "A (slope definition) register"]
            #[inline(always)]
            pub fn set_a(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
        }
        impl Default for A5 {
            #[inline(always)]
            fn default() -> A5 {
                A5(0)
            }
        }
        #[doc = "Y-intercept B0"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct B0(pub u32);
        impl B0 {
            #[doc = "B (y-intercept)"]
            #[inline(always)]
            pub const fn b(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "B (y-intercept)"]
            #[inline(always)]
            pub fn set_b(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for B0 {
            #[inline(always)]
            fn default() -> B0 {
                B0(0)
            }
        }
        #[doc = "Y-intercept B1"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct B1(pub u32);
        impl B1 {
            #[doc = "B (y-intercept)"]
            #[inline(always)]
            pub const fn b(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "B (y-intercept)"]
            #[inline(always)]
            pub fn set_b(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for B1 {
            #[inline(always)]
            fn default() -> B1 {
                B1(0)
            }
        }
        #[doc = "Y-intercept B2"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct B2(pub u32);
        impl B2 {
            #[doc = "B (y-intercept)"]
            #[inline(always)]
            pub const fn b(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "B (y-intercept)"]
            #[inline(always)]
            pub fn set_b(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for B2 {
            #[inline(always)]
            fn default() -> B2 {
                B2(0)
            }
        }
        #[doc = "Y-intercept B3"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct B3(pub u32);
        impl B3 {
            #[doc = "B (y-intercept)"]
            #[inline(always)]
            pub const fn b(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "B (y-intercept)"]
            #[inline(always)]
            pub fn set_b(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for B3 {
            #[inline(always)]
            fn default() -> B3 {
                B3(0)
            }
        }
        #[doc = "Y-intercept B4"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct B4(pub u32);
        impl B4 {
            #[doc = "B (y-intercept)"]
            #[inline(always)]
            pub const fn b(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "B (y-intercept)"]
            #[inline(always)]
            pub fn set_b(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for B4 {
            #[inline(always)]
            fn default() -> B4 {
                B4(0)
            }
        }
        #[doc = "Y-intercept B5"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct B5(pub u32);
        impl B5 {
            #[doc = "B (y-intercept)"]
            #[inline(always)]
            pub const fn b(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "B (y-intercept)"]
            #[inline(always)]
            pub fn set_b(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for B5 {
            #[inline(always)]
            fn default() -> B5 {
                B5(0)
            }
        }
        #[doc = "Device address type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Deviceaddrtype(pub u32);
        impl Deviceaddrtype {
            #[doc = "Device address type"]
            #[inline(always)]
            pub const fn deviceaddrtype(&self) -> super::vals::Deviceaddrtype {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Deviceaddrtype::from_bits(val as u8)
            }
            #[doc = "Device address type"]
            #[inline(always)]
            pub fn set_deviceaddrtype(&mut self, val: super::vals::Deviceaddrtype) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Deviceaddrtype {
            #[inline(always)]
            fn default() -> Deviceaddrtype {
                Deviceaddrtype(0)
            }
        }
        #[doc = "Flash variant"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Flash(pub u32);
        impl Flash {
            #[doc = "Flash variant"]
            #[inline(always)]
            pub const fn flash(&self) -> super::vals::Flash {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Flash::from_bits(val as u32)
            }
            #[doc = "Flash variant"]
            #[inline(always)]
            pub fn set_flash(&mut self, val: super::vals::Flash) {
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
        #[doc = "Package option"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Package(pub u32);
        impl Package {
            #[doc = "Package option"]
            #[inline(always)]
            pub const fn package(&self) -> super::vals::Package {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Package::from_bits(val as u32)
            }
            #[doc = "Package option"]
            #[inline(always)]
            pub fn set_package(&mut self, val: super::vals::Package) {
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
        #[doc = "Part code"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Part(pub u32);
        impl Part {
            #[doc = "Part code"]
            #[inline(always)]
            pub const fn part(&self) -> super::vals::Part {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Part::from_bits(val as u32)
            }
            #[doc = "Part code"]
            #[inline(always)]
            pub fn set_part(&mut self, val: super::vals::Part) {
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
        #[doc = "RAM variant"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ram(pub u32);
        impl Ram {
            #[doc = "RAM variant"]
            #[inline(always)]
            pub const fn ram(&self) -> super::vals::Ram {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Ram::from_bits(val as u32)
            }
            #[doc = "RAM variant"]
            #[inline(always)]
            pub fn set_ram(&mut self, val: super::vals::Ram) {
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
        #[doc = "Segment end T0"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct T0(pub u32);
        impl T0 {
            #[doc = "T (segment end) register"]
            #[inline(always)]
            pub const fn t(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "T (segment end) register"]
            #[inline(always)]
            pub fn set_t(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for T0 {
            #[inline(always)]
            fn default() -> T0 {
                T0(0)
            }
        }
        #[doc = "Segment end T1"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct T1(pub u32);
        impl T1 {
            #[doc = "T (segment end) register"]
            #[inline(always)]
            pub const fn t(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "T (segment end) register"]
            #[inline(always)]
            pub fn set_t(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for T1 {
            #[inline(always)]
            fn default() -> T1 {
                T1(0)
            }
        }
        #[doc = "Segment end T2"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct T2(pub u32);
        impl T2 {
            #[doc = "T (segment end) register"]
            #[inline(always)]
            pub const fn t(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "T (segment end) register"]
            #[inline(always)]
            pub fn set_t(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for T2 {
            #[inline(always)]
            fn default() -> T2 {
                T2(0)
            }
        }
        #[doc = "Segment end T3"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct T3(pub u32);
        impl T3 {
            #[doc = "T (segment end) register"]
            #[inline(always)]
            pub const fn t(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "T (segment end) register"]
            #[inline(always)]
            pub fn set_t(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for T3 {
            #[inline(always)]
            fn default() -> T3 {
                T3(0)
            }
        }
        #[doc = "Segment end T4"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct T4(pub u32);
        impl T4 {
            #[doc = "T (segment end) register"]
            #[inline(always)]
            pub const fn t(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "T (segment end) register"]
            #[inline(always)]
            pub fn set_t(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for T4 {
            #[inline(always)]
            fn default() -> T4 {
                T4(0)
            }
        }
        #[doc = "Part variant, hardware version and production configuration"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Variant(pub u32);
        impl Variant {
            #[doc = "Part variant, hardware version and production configuration, encoded as ASCII"]
            #[inline(always)]
            pub const fn variant(&self) -> super::vals::Variant {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Variant::from_bits(val as u32)
            }
            #[doc = "Part variant, hardware version and production configuration, encoded as ASCII"]
            #[inline(always)]
            pub fn set_variant(&mut self, val: super::vals::Variant) {
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
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        pub struct Flash(pub u32);
        impl Flash {
            #[doc = "192 kByte flash"]
            pub const K192: Self = Self(0xc0);
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
        pub struct Package(pub u32);
        impl Package {
            #[doc = "QFxx - 48-pin QFN"]
            pub const QF: Self = Self(0x2000);
            #[doc = "QCxx - 32-pin QFN"]
            pub const QC: Self = Self(0x2003);
            #[doc = "CAxx - WLCSP"]
            pub const CA: Self = Self(0x2004);
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
        pub struct Part(pub u32);
        impl Part {
            #[doc = "nRF52810"]
            pub const N52810: Self = Self(0x0005_2810);
            #[doc = "nRF52811"]
            pub const N52811: Self = Self(0x0005_2811);
            #[doc = "nRF52832"]
            pub const N52832: Self = Self(0x0005_2832);
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
        pub struct Ram(pub u32);
        impl Ram {
            #[doc = "24 kByte RAM"]
            pub const K24: Self = Self(0x18);
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
        pub struct Variant(pub u32);
        impl Variant {
            #[doc = "AAA0"]
            pub const AAA0: Self = Self(0x4141_4130);
            #[doc = "AAAA"]
            pub const AAAA: Self = Self(0x4141_4141);
            #[doc = "AAB0"]
            pub const AAB0: Self = Self(0x4141_4230);
            #[doc = "AABA"]
            pub const AABA: Self = Self(0x4141_4241);
            #[doc = "AABB"]
            pub const AABB: Self = Self(0x4141_4242);
            #[doc = "AAC0"]
            pub const AAC0: Self = Self(0x4141_4330);
            #[doc = "AACA"]
            pub const AACA: Self = Self(0x4141_4341);
            #[doc = "AACB"]
            pub const AACB: Self = Self(0x4141_4342);
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
    #[doc = "GPIO Port"]
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Set individual bits in GPIO port"]
        #[inline(always)]
        pub const fn outset(self) -> crate::common::Reg<regs::Outset, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Clear individual bits in GPIO port"]
        #[inline(always)]
        pub const fn outclr(self) -> crate::common::Reg<regs::Outclr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Read GPIO port"]
        #[inline(always)]
        pub const fn in_(self) -> crate::common::Reg<regs::In, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "Direction of GPIO pins"]
        #[inline(always)]
        pub const fn dir(self) -> crate::common::Reg<regs::Dir, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
        }
        #[doc = "DIR set register"]
        #[inline(always)]
        pub const fn dirset(self) -> crate::common::Reg<regs::Dirset, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
        #[doc = "DIR clear register"]
        #[inline(always)]
        pub const fn dirclr(self) -> crate::common::Reg<regs::Dirclr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
        }
        #[doc = "Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF\\[n\\].SENSE registers"]
        #[inline(always)]
        pub const fn latch(self) -> crate::common::Reg<regs::Latch, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize) as _) }
        }
        #[doc = "Select between default DETECT signal behaviour and LDETECT mode"]
        #[inline(always)]
        pub const fn detectmode(self) -> crate::common::Reg<regs::Detectmode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
        }
        #[doc = "Description collection: Configuration of GPIO pins"]
        #[inline(always)]
        pub const fn pin_cnf(
            self,
            n: usize,
        ) -> crate::common::Reg<regs::PinCnf, crate::common::RW> {
            assert!(n < 32usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0700usize + n * 4usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Select between default DETECT signal behaviour and LDETECT mode"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Detectmode(pub u32);
        impl Detectmode {
            #[doc = "Select between default DETECT signal behaviour and LDETECT mode"]
            #[inline(always)]
            pub const fn detectmode(&self) -> super::vals::Detectmode {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Detectmode::from_bits(val as u8)
            }
            #[doc = "Select between default DETECT signal behaviour and LDETECT mode"]
            #[inline(always)]
            pub fn set_detectmode(&mut self, val: super::vals::Detectmode) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Detectmode {
            #[inline(always)]
            fn default() -> Detectmode {
                Detectmode(0)
            }
        }
        #[doc = "Direction of GPIO pins"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dir(pub u32);
        impl Dir {
            #[doc = "Pin 0"]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> super::vals::Dir {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Dir::from_bits(val as u8)
            }
            #[doc = "Pin 0"]
            #[inline(always)]
            pub fn set_pin(&mut self, n: usize, val: super::vals::Dir) {
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
        #[doc = "DIR clear register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dirclr(pub u32);
        impl Dirclr {
            #[doc = "Set as input pin 0"]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Set as input pin 0"]
            #[inline(always)]
            pub fn set_pin(&mut self, n: usize, val: bool) {
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
        #[doc = "DIR set register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dirset(pub u32);
        impl Dirset {
            #[doc = "Set as output pin 0"]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Set as output pin 0"]
            #[inline(always)]
            pub fn set_pin(&mut self, n: usize, val: bool) {
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
        #[doc = "Read GPIO port"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct In(pub u32);
        impl In {
            #[doc = "Pin 0"]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Pin 0"]
            #[inline(always)]
            pub fn set_pin(&mut self, n: usize, val: bool) {
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
        #[doc = "Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF\\[n\\].SENSE registers"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Latch(pub u32);
        impl Latch {
            #[doc = "Status on whether PIN0 has met criteria set in PIN_CNF0.SENSE register. Write '1' to clear."]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Status on whether PIN0 has met criteria set in PIN_CNF0.SENSE register. Write '1' to clear."]
            #[inline(always)]
            pub fn set_pin(&mut self, n: usize, val: bool) {
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
        #[doc = "Write GPIO port"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Out(pub u32);
        impl Out {
            #[doc = "Pin 0"]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Pin 0"]
            #[inline(always)]
            pub fn set_pin(&mut self, n: usize, val: bool) {
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
        #[doc = "Clear individual bits in GPIO port"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Outclr(pub u32);
        impl Outclr {
            #[doc = "Pin 0"]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Pin 0"]
            #[inline(always)]
            pub fn set_pin(&mut self, n: usize, val: bool) {
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
        #[doc = "Set individual bits in GPIO port"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Outset(pub u32);
        impl Outset {
            #[doc = "Pin 0"]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Pin 0"]
            #[inline(always)]
            pub fn set_pin(&mut self, n: usize, val: bool) {
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
        #[doc = "Description collection: Configuration of GPIO pins"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct PinCnf(pub u32);
        impl PinCnf {
            #[doc = "Pin direction. Same physical register as DIR register"]
            #[inline(always)]
            pub const fn dir(&self) -> super::vals::Dir {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Dir::from_bits(val as u8)
            }
            #[doc = "Pin direction. Same physical register as DIR register"]
            #[inline(always)]
            pub fn set_dir(&mut self, val: super::vals::Dir) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Connect or disconnect input buffer"]
            #[inline(always)]
            pub const fn input(&self) -> super::vals::Input {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Input::from_bits(val as u8)
            }
            #[doc = "Connect or disconnect input buffer"]
            #[inline(always)]
            pub fn set_input(&mut self, val: super::vals::Input) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Pull configuration"]
            #[inline(always)]
            pub const fn pull(&self) -> super::vals::Pull {
                let val = (self.0 >> 2usize) & 0x03;
                super::vals::Pull::from_bits(val as u8)
            }
            #[doc = "Pull configuration"]
            #[inline(always)]
            pub fn set_pull(&mut self, val: super::vals::Pull) {
                self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
            }
            #[doc = "Drive configuration"]
            #[inline(always)]
            pub const fn drive(&self) -> super::vals::Drive {
                let val = (self.0 >> 8usize) & 0x07;
                super::vals::Drive::from_bits(val as u8)
            }
            #[doc = "Drive configuration"]
            #[inline(always)]
            pub fn set_drive(&mut self, val: super::vals::Drive) {
                self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
            }
            #[doc = "Pin sensing mechanism"]
            #[inline(always)]
            pub const fn sense(&self) -> super::vals::Sense {
                let val = (self.0 >> 16usize) & 0x03;
                super::vals::Sense::from_bits(val as u8)
            }
            #[doc = "Pin sensing mechanism"]
            #[inline(always)]
            pub fn set_sense(&mut self, val: super::vals::Sense) {
                self.0 =
                    (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
            }
        }
        impl Default for PinCnf {
            #[inline(always)]
            fn default() -> PinCnf {
                PinCnf(0)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Detectmode {
            #[doc = "DETECT directly connected to PIN DETECT signals"]
            DEFAULT = 0x0,
            #[doc = "Use the latched LDETECT behaviour"]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Drive {
            #[doc = "Standard '0', standard '1'"]
            S0S1 = 0x0,
            #[doc = "High drive '0', standard '1'"]
            H0S1 = 0x01,
            #[doc = "Standard '0', high drive '1'"]
            S0H1 = 0x02,
            #[doc = "High drive '0', high 'drive '1''"]
            H0H1 = 0x03,
            #[doc = "Disconnect '0' standard '1' (normally used for wired-or connections)"]
            D0S1 = 0x04,
            #[doc = "Disconnect '0', high drive '1' (normally used for wired-or connections)"]
            D0H1 = 0x05,
            #[doc = "Standard '0'. disconnect '1' (normally used for wired-and connections)"]
            S0D1 = 0x06,
            #[doc = "High drive '0', disconnect '1' (normally used for wired-and connections)"]
            H0D1 = 0x07,
        }
        impl Drive {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Drive {
                unsafe { core::mem::transmute(val & 0x07) }
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it high."]
        #[inline(always)]
        pub const fn tasks_set(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it low."]
        #[inline(always)]
        pub const fn tasks_clr(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Event generated from pin specified in CONFIG\\[n\\].PSEL"]
        #[inline(always)]
        pub const fn events_in(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
        }
        #[doc = "Event generated from multiple input GPIO pins with SENSE mechanism enabled"]
        #[inline(always)]
        pub const fn events_port(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x017cusize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Description collection: Configuration for OUT\\[n\\], SET\\[n\\] and CLR\\[n\\] tasks and IN\\[n\\] event"]
        #[inline(always)]
        pub const fn config(self, n: usize) -> crate::common::Reg<regs::Config, crate::common::RW> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize + n * 4usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Description collection: Configuration for OUT\\[n\\], SET\\[n\\] and CLR\\[n\\] tasks and IN\\[n\\] event"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Mode"]
            #[inline(always)]
            pub const fn mode(&self) -> super::vals::Mode {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Mode::from_bits(val as u8)
            }
            #[doc = "Mode"]
            #[inline(always)]
            pub fn set_mode(&mut self, val: super::vals::Mode) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
            #[doc = "GPIO number associated with SET\\[n\\], CLR\\[n\\] and OUT\\[n\\] tasks and IN\\[n\\] event"]
            #[inline(always)]
            pub const fn psel(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0x1f;
                val as u8
            }
            #[doc = "GPIO number associated with SET\\[n\\], CLR\\[n\\] and OUT\\[n\\] tasks and IN\\[n\\] event"]
            #[inline(always)]
            pub fn set_psel(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
            }
            #[doc = "When In task mode: Operation to be performed on output when OUT\\[n\\] task is triggered. When In event mode: Operation on input that shall trigger IN\\[n\\] event."]
            #[inline(always)]
            pub const fn polarity(&self) -> super::vals::Polarity {
                let val = (self.0 >> 16usize) & 0x03;
                super::vals::Polarity::from_bits(val as u8)
            }
            #[doc = "When In task mode: Operation to be performed on output when OUT\\[n\\] task is triggered. When In event mode: Operation on input that shall trigger IN\\[n\\] event."]
            #[inline(always)]
            pub fn set_polarity(&mut self, val: super::vals::Polarity) {
                self.0 =
                    (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
            }
            #[doc = "When in task mode: Initial value of the output when the GPIOTE channel is configured. When in event mode: No effect."]
            #[inline(always)]
            pub const fn outinit(&self) -> super::vals::Outinit {
                let val = (self.0 >> 20usize) & 0x01;
                super::vals::Outinit::from_bits(val as u8)
            }
            #[doc = "When in task mode: Initial value of the output when the GPIOTE channel is configured. When in event mode: No effect."]
            #[inline(always)]
            pub fn set_outinit(&mut self, val: super::vals::Outinit) {
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
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Write '1' to disable interrupt for event IN\\[0\\]"]
            #[inline(always)]
            pub const fn in0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event IN\\[0\\]"]
            #[inline(always)]
            pub fn set_in0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to disable interrupt for event IN\\[1\\]"]
            #[inline(always)]
            pub const fn in1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event IN\\[1\\]"]
            #[inline(always)]
            pub fn set_in1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to disable interrupt for event IN\\[2\\]"]
            #[inline(always)]
            pub const fn in2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event IN\\[2\\]"]
            #[inline(always)]
            pub fn set_in2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Write '1' to disable interrupt for event IN\\[3\\]"]
            #[inline(always)]
            pub const fn in3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event IN\\[3\\]"]
            #[inline(always)]
            pub fn set_in3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Write '1' to disable interrupt for event IN\\[4\\]"]
            #[inline(always)]
            pub const fn in4(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event IN\\[4\\]"]
            #[inline(always)]
            pub fn set_in4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Write '1' to disable interrupt for event IN\\[5\\]"]
            #[inline(always)]
            pub const fn in5(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event IN\\[5\\]"]
            #[inline(always)]
            pub fn set_in5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Write '1' to disable interrupt for event IN\\[6\\]"]
            #[inline(always)]
            pub const fn in6(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event IN\\[6\\]"]
            #[inline(always)]
            pub fn set_in6(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Write '1' to disable interrupt for event IN\\[7\\]"]
            #[inline(always)]
            pub const fn in7(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event IN\\[7\\]"]
            #[inline(always)]
            pub fn set_in7(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Write '1' to disable interrupt for event PORT"]
            #[inline(always)]
            pub const fn port(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event PORT"]
            #[inline(always)]
            pub fn set_port(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "Configuration register"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Register for erasing a page in code area"]
        #[inline(always)]
        pub const fn erasepage(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Deprecated register - Register for erasing a page in code area. Equivalent to ERASEPAGE."]
        #[inline(always)]
        pub const fn erasepcr1(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Register for erasing all non-volatile user memory"]
        #[inline(always)]
        pub const fn eraseall(self) -> crate::common::Reg<regs::Eraseall, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Deprecated register - Register for erasing a page in code area. Equivalent to ERASEPAGE."]
        #[inline(always)]
        pub const fn erasepcr0(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "Register for erasing user information configuration registers"]
        #[inline(always)]
        pub const fn eraseuicr(self) -> crate::common::Reg<regs::Eraseuicr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
        }
        #[doc = "Register for partial erase of a page in code area"]
        #[inline(always)]
        pub const fn erasepagepartial(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
        #[doc = "Register for partial erase configuration"]
        #[inline(always)]
        pub const fn erasepagepartialcfg(
            self,
        ) -> crate::common::Reg<regs::Erasepagepartialcfg, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Program memory access mode. It is strongly recommended to activate erase and write modes only when they are actively used."]
            #[inline(always)]
            pub const fn wen(&self) -> super::vals::Wen {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Wen::from_bits(val as u8)
            }
            #[doc = "Program memory access mode. It is strongly recommended to activate erase and write modes only when they are actively used."]
            #[inline(always)]
            pub fn set_wen(&mut self, val: super::vals::Wen) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        #[doc = "Register for erasing all non-volatile user memory"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Eraseall(pub u32);
        impl Eraseall {
            #[doc = "Erase all non-volatile memory including UICR registers. Note that the erase must be enabled using CONFIG.WEN before the non-volatile memory can be erased."]
            #[inline(always)]
            pub const fn eraseall(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Erase all non-volatile memory including UICR registers. Note that the erase must be enabled using CONFIG.WEN before the non-volatile memory can be erased."]
            #[inline(always)]
            pub fn set_eraseall(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Eraseall {
            #[inline(always)]
            fn default() -> Eraseall {
                Eraseall(0)
            }
        }
        #[doc = "Register for partial erase configuration"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Erasepagepartialcfg(pub u32);
        impl Erasepagepartialcfg {
            #[doc = "Duration of the partial erase in milliseconds"]
            #[inline(always)]
            pub const fn duration(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "Duration of the partial erase in milliseconds"]
            #[inline(always)]
            pub fn set_duration(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
            }
        }
        impl Default for Erasepagepartialcfg {
            #[inline(always)]
            fn default() -> Erasepagepartialcfg {
                Erasepagepartialcfg(0)
            }
        }
        #[doc = "Register for erasing user information configuration registers"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Eraseuicr(pub u32);
        impl Eraseuicr {
            #[doc = "Register starting erase of all user information configuration registers. Note that the erase must be enabled using CONFIG.WEN before the UICR can be erased."]
            #[inline(always)]
            pub const fn eraseuicr(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Register starting erase of all user information configuration registers. Note that the erase must be enabled using CONFIG.WEN before the UICR can be erased."]
            #[inline(always)]
            pub fn set_eraseuicr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Eraseuicr {
            #[inline(always)]
            fn default() -> Eraseuicr {
                Eraseuicr(0)
            }
        }
        #[doc = "Ready flag"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ready(pub u32);
        impl Ready {
            #[doc = "NVMC is ready or busy"]
            #[inline(always)]
            pub const fn ready(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "NVMC is ready or busy"]
            #[inline(always)]
            pub fn set_ready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Ready {
            #[inline(always)]
            fn default() -> Ready {
                Ready(0)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Wen {
            #[doc = "Read only access"]
            REN = 0x0,
            #[doc = "Write enabled"]
            WEN = 0x01,
            #[doc = "Erase enabled"]
            EEN = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Wen {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Wen {
                unsafe { core::mem::transmute(val & 0x03) }
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
pub mod pdm {
    #[doc = "Pulse Density Modulation (Digital Microphone) Interface"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdm {
        ptr: *mut u8,
    }
    unsafe impl Send for Pdm {}
    unsafe impl Sync for Pdm {}
    impl Pdm {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Starts continuous PDM transfer"]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stops PDM transfer"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "PDM transfer has started"]
        #[inline(always)]
        pub const fn events_started(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "PDM transfer has finished"]
        #[inline(always)]
        pub const fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "The PDM has written the last sample specified by SAMPLE.MAXCNT (or the last sample after a STOP task has been received) to Data RAM"]
        #[inline(always)]
        pub const fn events_end(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "PDM module enable register"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "PDM clock generator control"]
        #[inline(always)]
        pub const fn pdmclkctrl(self) -> crate::common::Reg<regs::Pdmclkctrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Defines the routing of the connected PDM microphones' signals"]
        #[inline(always)]
        pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Left output gain adjustment"]
        #[inline(always)]
        pub const fn gainl(self) -> crate::common::Reg<regs::Gainl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
        #[doc = "Right output gain adjustment"]
        #[inline(always)]
        pub const fn gainr(self) -> crate::common::Reg<regs::Gainr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn psel(self) -> Psel {
            unsafe { Psel::from_ptr(self.ptr.add(0x0540usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn sample(self) -> Sample {
            unsafe { Sample::from_ptr(self.ptr.add(0x0560usize) as _) }
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
        #[doc = "Pin number configuration for PDM CLK signal"]
        #[inline(always)]
        pub const fn clk(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Pin number configuration for PDM DIN signal"]
        #[inline(always)]
        pub const fn din(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sample {
        ptr: *mut u8,
    }
    unsafe impl Send for Sample {}
    unsafe impl Sync for Sample {}
    impl Sample {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "RAM address pointer to write samples to with EasyDMA"]
        #[inline(always)]
        pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Number of samples to allocate memory for in EasyDMA mode"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::Maxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "PDM module enable register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable PDM module"]
            #[inline(always)]
            pub const fn enable(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable PDM module"]
            #[inline(always)]
            pub fn set_enable(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        #[doc = "Left output gain adjustment"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Gainl(pub u32);
        impl Gainl {
            #[doc = "Left output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters) 0x00 -20 dB gain adjust 0x01 -19.5 dB gain adjust (...) 0x27 -0.5 dB gain adjust 0x28 0 dB gain adjust 0x29 +0.5 dB gain adjust (...) 0x4F +19.5 dB gain adjust 0x50 +20 dB gain adjust"]
            #[inline(always)]
            pub const fn gainl(&self) -> super::vals::Gain {
                let val = (self.0 >> 0usize) & 0x7f;
                super::vals::Gain::from_bits(val as u8)
            }
            #[doc = "Left output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters) 0x00 -20 dB gain adjust 0x01 -19.5 dB gain adjust (...) 0x27 -0.5 dB gain adjust 0x28 0 dB gain adjust 0x29 +0.5 dB gain adjust (...) 0x4F +19.5 dB gain adjust 0x50 +20 dB gain adjust"]
            #[inline(always)]
            pub fn set_gainl(&mut self, val: super::vals::Gain) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
            }
        }
        impl Default for Gainl {
            #[inline(always)]
            fn default() -> Gainl {
                Gainl(0)
            }
        }
        #[doc = "Right output gain adjustment"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Gainr(pub u32);
        impl Gainr {
            #[doc = "Right output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters)"]
            #[inline(always)]
            pub const fn gainr(&self) -> super::vals::Gain {
                let val = (self.0 >> 0usize) & 0x7f;
                super::vals::Gain::from_bits(val as u8)
            }
            #[doc = "Right output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters)"]
            #[inline(always)]
            pub fn set_gainr(&mut self, val: super::vals::Gain) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
            }
        }
        impl Default for Gainr {
            #[inline(always)]
            fn default() -> Gainr {
                Gainr(0)
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Enable or disable interrupt for event STARTED"]
            #[inline(always)]
            pub const fn started(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event STARTED"]
            #[inline(always)]
            pub fn set_started(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable interrupt for event STOPPED"]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event STOPPED"]
            #[inline(always)]
            pub fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for event END"]
            #[inline(always)]
            pub const fn end(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event END"]
            #[inline(always)]
            pub fn set_end(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
        #[doc = "Number of samples to allocate memory for in EasyDMA mode"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Maxcnt(pub u32);
        impl Maxcnt {
            #[doc = "Length of DMA RAM allocation in number of samples"]
            #[inline(always)]
            pub const fn buffsize(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x7fff;
                val as u16
            }
            #[doc = "Length of DMA RAM allocation in number of samples"]
            #[inline(always)]
            pub fn set_buffsize(&mut self, val: u16) {
                self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
            }
        }
        impl Default for Maxcnt {
            #[inline(always)]
            fn default() -> Maxcnt {
                Maxcnt(0)
            }
        }
        #[doc = "Defines the routing of the connected PDM microphones' signals"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Mode(pub u32);
        impl Mode {
            #[doc = "Mono or stereo operation"]
            #[inline(always)]
            pub const fn operation(&self) -> super::vals::Operation {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Operation::from_bits(val as u8)
            }
            #[doc = "Mono or stereo operation"]
            #[inline(always)]
            pub fn set_operation(&mut self, val: super::vals::Operation) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Defines on which PDM_CLK edge Left (or mono) is sampled"]
            #[inline(always)]
            pub const fn edge(&self) -> super::vals::Edge {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Edge::from_bits(val as u8)
            }
            #[doc = "Defines on which PDM_CLK edge Left (or mono) is sampled"]
            #[inline(always)]
            pub fn set_edge(&mut self, val: super::vals::Edge) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
        }
        impl Default for Mode {
            #[inline(always)]
            fn default() -> Mode {
                Mode(0)
            }
        }
        #[doc = "PDM clock generator control"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pdmclkctrl(pub u32);
        impl Pdmclkctrl {
            #[doc = "PDM_CLK frequency"]
            #[inline(always)]
            pub const fn freq(&self) -> super::vals::Freq {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Freq::from_bits(val as u32)
            }
            #[doc = "PDM_CLK frequency"]
            #[inline(always)]
            pub fn set_freq(&mut self, val: super::vals::Freq) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Pdmclkctrl {
            #[inline(always)]
            fn default() -> Pdmclkctrl {
                Pdmclkctrl(0)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Edge {
            #[doc = "Left (or mono) is sampled on falling edge of PDM_CLK"]
            LEFT_FALLING = 0x0,
            #[doc = "Left (or mono) is sampled on rising edge of PDM_CLK"]
            LEFT_RISING = 0x01,
        }
        impl Edge {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Edge {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Edge {
            #[inline(always)]
            fn from(val: u8) -> Edge {
                Edge::from_bits(val)
            }
        }
        impl From<Edge> for u8 {
            #[inline(always)]
            fn from(val: Edge) -> u8 {
                Edge::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Freq(pub u32);
        impl Freq {
            #[doc = "PDM_CLK = 32 MHz / 32 = 1.000 MHz"]
            pub const _1000K: Self = Self(0x0800_0000);
            #[doc = "PDM_CLK = 32 MHz / 31 = 1.032 MHz"]
            pub const DEFAULT: Self = Self(0x0840_0000);
            #[doc = "PDM_CLK = 32 MHz / 30 = 1.067 MHz"]
            pub const _1067K: Self = Self(0x0880_0000);
        }
        impl Freq {
            pub const fn from_bits(val: u32) -> Freq {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl From<u32> for Freq {
            #[inline(always)]
            fn from(val: u32) -> Freq {
                Freq::from_bits(val)
            }
        }
        impl From<Freq> for u32 {
            #[inline(always)]
            fn from(val: Freq) -> u32 {
                Freq::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Gain(pub u8);
        impl Gain {
            #[doc = "-20dB gain adjustment (minimum)"]
            pub const MIN_GAIN: Self = Self(0x0);
            #[doc = "0dB gain adjustment ('2500 RMS' requirement)"]
            pub const DEFAULT_GAIN: Self = Self(0x28);
            #[doc = "+20dB gain adjustment (maximum)"]
            pub const MAX_GAIN: Self = Self(0x50);
        }
        impl Gain {
            pub const fn from_bits(val: u8) -> Gain {
                Self(val & 0x7f)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl From<u8> for Gain {
            #[inline(always)]
            fn from(val: u8) -> Gain {
                Gain::from_bits(val)
            }
        }
        impl From<Gain> for u8 {
            #[inline(always)]
            fn from(val: Gain) -> u8 {
                Gain::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Operation {
            #[doc = "Sample and store one pair (Left + Right) of 16bit samples per RAM word R=\\[31:16\\]; L=\\[15:0\\]"]
            STEREO = 0x0,
            #[doc = "Sample and store two successive Left samples (16 bit each) per RAM word L1=\\[31:16\\]; L0=\\[15:0\\]"]
            MONO = 0x01,
        }
        impl Operation {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Operation {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Operation {
            #[inline(always)]
            fn from(val: u8) -> Operation {
                Operation::from_bits(val)
            }
        }
        impl From<Operation> for u8 {
            #[inline(always)]
            fn from(val: Operation) -> u8 {
                Operation::to_bits(val)
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
        }
        #[doc = "Enable Low-power mode (variable latency)"]
        #[inline(always)]
        pub const fn tasks_lowpwr(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
        }
        #[doc = "Power failure warning"]
        #[inline(always)]
        pub const fn events_pofwarn(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "CPU entered WFI/WFE sleep"]
        #[inline(always)]
        pub const fn events_sleepenter(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
        }
        #[doc = "CPU exited WFI/WFE sleep"]
        #[inline(always)]
        pub const fn events_sleepexit(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Reset reason"]
        #[inline(always)]
        pub const fn resetreas(self) -> crate::common::Reg<regs::Resetreas, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "System OFF register"]
        #[inline(always)]
        pub const fn systemoff(self) -> crate::common::Reg<regs::Systemoff, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Power failure comparator configuration"]
        #[inline(always)]
        pub const fn pofcon(self) -> crate::common::Reg<regs::Pofcon, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "General purpose retention register"]
        #[inline(always)]
        pub const fn gpregret(self) -> crate::common::Reg<regs::Gpregret, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
        }
        #[doc = "General purpose retention register"]
        #[inline(always)]
        pub const fn gpregret2(self) -> crate::common::Reg<regs::Gpregret2, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize) as _) }
        }
        #[doc = "DC/DC enable register"]
        #[inline(always)]
        pub const fn dcdcen(self) -> crate::common::Reg<regs::Dcdcen, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0578usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn ram(self, n: usize) -> Ram {
            assert!(n < 8usize);
            unsafe { Ram::from_ptr(self.ptr.add(0x0900usize + n * 16usize) as _) }
        }
    }
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
        #[doc = "Description cluster: RAMn power control register. The RAM size will vary depending on product variant, and the RAMn register will only be present if the corresponding RAM AHB slave is present on the device."]
        #[inline(always)]
        pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster: RAMn power control set register"]
        #[inline(always)]
        pub const fn powerset(self) -> crate::common::Reg<regs::Power, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Description cluster: RAMn power control clear register"]
        #[inline(always)]
        pub const fn powerclr(self) -> crate::common::Reg<regs::Power, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "DC/DC enable register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dcdcen(pub u32);
        impl Dcdcen {
            #[doc = "Enable or disable DC/DC converter"]
            #[inline(always)]
            pub const fn dcdcen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable DC/DC converter"]
            #[inline(always)]
            pub fn set_dcdcen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Dcdcen {
            #[inline(always)]
            fn default() -> Dcdcen {
                Dcdcen(0)
            }
        }
        #[doc = "General purpose retention register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Gpregret(pub u32);
        impl Gpregret {
            #[doc = "General purpose retention register"]
            #[inline(always)]
            pub const fn gpregret(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "General purpose retention register"]
            #[inline(always)]
            pub fn set_gpregret(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Gpregret {
            #[inline(always)]
            fn default() -> Gpregret {
                Gpregret(0)
            }
        }
        #[doc = "General purpose retention register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Gpregret2(pub u32);
        impl Gpregret2 {
            #[doc = "General purpose retention register"]
            #[inline(always)]
            pub const fn gpregret(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "General purpose retention register"]
            #[inline(always)]
            pub fn set_gpregret(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Gpregret2 {
            #[inline(always)]
            fn default() -> Gpregret2 {
                Gpregret2(0)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Write '1' to disable interrupt for event POFWARN"]
            #[inline(always)]
            pub const fn pofwarn(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event POFWARN"]
            #[inline(always)]
            pub fn set_pofwarn(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Write '1' to disable interrupt for event SLEEPENTER"]
            #[inline(always)]
            pub const fn sleepenter(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event SLEEPENTER"]
            #[inline(always)]
            pub fn set_sleepenter(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Write '1' to disable interrupt for event SLEEPEXIT"]
            #[inline(always)]
            pub const fn sleepexit(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event SLEEPEXIT"]
            #[inline(always)]
            pub fn set_sleepexit(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
        #[doc = "Power failure comparator configuration"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pofcon(pub u32);
        impl Pofcon {
            #[doc = "Enable or disable power failure comparator"]
            #[inline(always)]
            pub const fn pof(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable power failure comparator"]
            #[inline(always)]
            pub fn set_pof(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Power failure comparator threshold setting"]
            #[inline(always)]
            pub const fn threshold(&self) -> super::vals::Threshold {
                let val = (self.0 >> 1usize) & 0x0f;
                super::vals::Threshold::from_bits(val as u8)
            }
            #[doc = "Power failure comparator threshold setting"]
            #[inline(always)]
            pub fn set_threshold(&mut self, val: super::vals::Threshold) {
                self.0 = (self.0 & !(0x0f << 1usize)) | (((val.to_bits() as u32) & 0x0f) << 1usize);
            }
        }
        impl Default for Pofcon {
            #[inline(always)]
            fn default() -> Pofcon {
                Pofcon(0)
            }
        }
        #[doc = "Description cluster: RAMn power control register. The RAM size will vary depending on product variant, and the RAMn register will only be present if the corresponding RAM AHB slave is present on the device."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Power(pub u32);
        impl Power {
            #[doc = "Keep RAM section S0 ON or OFF in System ON mode."]
            #[inline(always)]
            pub const fn s_power(&self, n: usize) -> bool {
                assert!(n < 2usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Keep RAM section S0 ON or OFF in System ON mode."]
            #[inline(always)]
            pub fn set_s_power(&mut self, n: usize, val: bool) {
                assert!(n < 2usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Keep retention on RAM section S0 when RAM section is in OFF"]
            #[inline(always)]
            pub const fn s_retention(&self, n: usize) -> bool {
                assert!(n < 2usize);
                let offs = 16usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Keep retention on RAM section S0 when RAM section is in OFF"]
            #[inline(always)]
            pub fn set_s_retention(&mut self, n: usize, val: bool) {
                assert!(n < 2usize);
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
        #[doc = "Reset reason"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Resetreas(pub u32);
        impl Resetreas {
            #[doc = "Reset from pin-reset detected"]
            #[inline(always)]
            pub const fn resetpin(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from pin-reset detected"]
            #[inline(always)]
            pub fn set_resetpin(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Reset from watchdog detected"]
            #[inline(always)]
            pub const fn dog(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from watchdog detected"]
            #[inline(always)]
            pub fn set_dog(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Reset from soft reset detected"]
            #[inline(always)]
            pub const fn sreq(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from soft reset detected"]
            #[inline(always)]
            pub fn set_sreq(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Reset from CPU lock-up detected"]
            #[inline(always)]
            pub const fn lockup(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from CPU lock-up detected"]
            #[inline(always)]
            pub fn set_lockup(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Reset due to wake up from System OFF mode when wakeup is triggered from DETECT signal from GPIO"]
            #[inline(always)]
            pub const fn off(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Reset due to wake up from System OFF mode when wakeup is triggered from DETECT signal from GPIO"]
            #[inline(always)]
            pub fn set_off(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Reset due to wake up from System OFF mode when wakeup is triggered from entering into debug interface mode"]
            #[inline(always)]
            pub const fn dif(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Reset due to wake up from System OFF mode when wakeup is triggered from entering into debug interface mode"]
            #[inline(always)]
            pub fn set_dif(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
        }
        impl Default for Resetreas {
            #[inline(always)]
            fn default() -> Resetreas {
                Resetreas(0)
            }
        }
        #[doc = "System OFF register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Systemoff(pub u32);
        impl Systemoff {
            #[doc = "Enable System OFF mode"]
            #[inline(always)]
            pub const fn systemoff(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable System OFF mode"]
            #[inline(always)]
            pub fn set_systemoff(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Systemoff {
            #[inline(always)]
            fn default() -> Systemoff {
                Systemoff(0)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Threshold {
            _RESERVED_0 = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            #[doc = "Set threshold to 1.7 V"]
            V17 = 0x04,
            #[doc = "Set threshold to 1.8 V"]
            V18 = 0x05,
            #[doc = "Set threshold to 1.9 V"]
            V19 = 0x06,
            #[doc = "Set threshold to 2.0 V"]
            V20 = 0x07,
            #[doc = "Set threshold to 2.1 V"]
            V21 = 0x08,
            #[doc = "Set threshold to 2.2 V"]
            V22 = 0x09,
            #[doc = "Set threshold to 2.3 V"]
            V23 = 0x0a,
            #[doc = "Set threshold to 2.4 V"]
            V24 = 0x0b,
            #[doc = "Set threshold to 2.5 V"]
            V25 = 0x0c,
            #[doc = "Set threshold to 2.6 V"]
            V26 = 0x0d,
            #[doc = "Set threshold to 2.7 V"]
            V27 = 0x0e,
            #[doc = "Set threshold to 2.8 V"]
            V28 = 0x0f,
        }
        impl Threshold {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Threshold {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Threshold {
            #[inline(always)]
            fn from(val: u8) -> Threshold {
                Threshold::from_bits(val)
            }
        }
        impl From<Threshold> for u8 {
            #[inline(always)]
            fn from(val: Threshold) -> u8 {
                Threshold::to_bits(val)
            }
        }
    }
}
pub mod ppi {
    #[doc = "PPI Channel"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch {
        ptr: *mut u8,
    }
    unsafe impl Send for Ch {}
    unsafe impl Sync for Ch {}
    impl Ch {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster: Channel n event end-point"]
        #[inline(always)]
        pub const fn eep(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster: Channel n task end-point"]
        #[inline(always)]
        pub const fn tep(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    #[doc = "Fork"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fork {
        ptr: *mut u8,
    }
    unsafe impl Send for Fork {}
    unsafe impl Sync for Fork {}
    impl Fork {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster: Channel n task end-point"]
        #[inline(always)]
        pub const fn tep(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
    }
    #[doc = "Programmable Peripheral Interconnect"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ppi {
        ptr: *mut u8,
    }
    unsafe impl Send for Ppi {}
    unsafe impl Sync for Ppi {}
    impl Ppi {
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
            unsafe { TasksChg::from_ptr(self.ptr.add(0x0usize + n * 8usize) as _) }
        }
        #[doc = "Channel enable register"]
        #[inline(always)]
        pub const fn chen(self) -> crate::common::Reg<regs::Chen, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Channel enable set register"]
        #[inline(always)]
        pub const fn chenset(self) -> crate::common::Reg<regs::Chen, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Channel enable clear register"]
        #[inline(always)]
        pub const fn chenclr(self) -> crate::common::Reg<regs::Chen, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "PPI Channel"]
        #[inline(always)]
        pub const fn ch(self, n: usize) -> Ch {
            assert!(n < 20usize);
            unsafe { Ch::from_ptr(self.ptr.add(0x0510usize + n * 8usize) as _) }
        }
        #[doc = "Description collection: Channel group n"]
        #[inline(always)]
        pub const fn chg(self, n: usize) -> crate::common::Reg<regs::Chg, crate::common::RW> {
            assert!(n < 6usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize + n * 4usize) as _) }
        }
        #[doc = "Fork"]
        #[inline(always)]
        pub const fn fork(self, n: usize) -> Fork {
            assert!(n < 32usize);
            unsafe { Fork::from_ptr(self.ptr.add(0x0910usize + n * 4usize) as _) }
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster: Disable channel group n"]
        #[inline(always)]
        pub const fn dis(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Channel enable register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Chen(pub u32);
        impl Chen {
            #[doc = "Enable or disable channel 0"]
            #[inline(always)]
            pub const fn ch(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable channel 0"]
            #[inline(always)]
            pub fn set_ch(&mut self, n: usize, val: bool) {
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
        #[doc = "Description collection: Channel group n"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Chg(pub u32);
        impl Chg {
            #[doc = "Include or exclude channel 0"]
            #[inline(always)]
            pub const fn ch(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Include or exclude channel 0"]
            #[inline(always)]
            pub fn set_ch(&mut self, n: usize, val: bool) {
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
    }
}
pub mod pwm {
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
        #[doc = "Description collection: Output pin select for PWM channel n"]
        #[inline(always)]
        pub const fn out(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
        }
    }
    #[doc = "Pulse width modulation unit"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwm {
        ptr: *mut u8,
    }
    unsafe impl Send for Pwm {}
    unsafe impl Sync for Pwm {}
    impl Pwm {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Stops PWM pulse generation on all channels at the end of current PWM period, and stops sequence playback"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Description collection: Loads the first PWM value on all enabled channels from sequence n, and starts playing that sequence at the rate defined in SEQ\\[n\\]REFRESH and/or DECODER.MODE. Causes PWM generation to start if not running."]
        #[inline(always)]
        pub const fn tasks_seqstart(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
            assert!(n < 2usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 4usize) as _) }
        }
        #[doc = "Steps by one value in the current sequence on all enabled channels if DECODER.MODE=NextStep. Does not cause PWM generation to start if not running."]
        #[inline(always)]
        pub const fn tasks_nextstep(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Response to STOP task, emitted when PWM pulses are no longer generated"]
        #[inline(always)]
        pub const fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Description collection: First PWM period started on sequence n"]
        #[inline(always)]
        pub const fn events_seqstarted(
            self,
            n: usize,
        ) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 2usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Emitted at end of every sequence n, when last value from RAM has been applied to wave counter"]
        #[inline(always)]
        pub const fn events_seqend(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 2usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize + n * 4usize) as _) }
        }
        #[doc = "Emitted at the end of each PWM period"]
        #[inline(always)]
        pub const fn events_pwmperiodend(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
        }
        #[doc = "Concatenated sequences have been played the amount of times defined in LOOP.CNT"]
        #[inline(always)]
        pub const fn events_loopsdone(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "PWM module enable register"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Selects operating mode of the wave counter"]
        #[inline(always)]
        pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Value up to which the pulse generator counter counts"]
        #[inline(always)]
        pub const fn countertop(self) -> crate::common::Reg<regs::Countertop, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Configuration for PWM_CLK"]
        #[inline(always)]
        pub const fn prescaler(self) -> crate::common::Reg<regs::Prescaler, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Configuration of the decoder"]
        #[inline(always)]
        pub const fn decoder(self) -> crate::common::Reg<regs::Decoder, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "Number of playbacks of a loop"]
        #[inline(always)]
        pub const fn loop_(self) -> crate::common::Reg<regs::Loop, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn seq(self, n: usize) -> Seq {
            assert!(n < 2usize);
            unsafe { Seq::from_ptr(self.ptr.add(0x0520usize + n * 32usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn psel(self) -> Psel {
            unsafe { Psel::from_ptr(self.ptr.add(0x0560usize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seq {
        ptr: *mut u8,
    }
    unsafe impl Send for Seq {}
    unsafe impl Sync for Seq {}
    impl Seq {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster: Beginning address in RAM of this sequence"]
        #[inline(always)]
        pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster: Number of values (duty cycles) in this sequence"]
        #[inline(always)]
        pub const fn cnt(self) -> crate::common::Reg<regs::Cnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Description cluster: Number of additional PWM periods between samples loaded into compare register"]
        #[inline(always)]
        pub const fn refresh(self) -> crate::common::Reg<regs::Refresh, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Description cluster: Time added after the sequence"]
        #[inline(always)]
        pub const fn enddelay(self) -> crate::common::Reg<regs::Enddelay, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Description cluster: Number of values (duty cycles) in this sequence"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cnt(pub u32);
        impl Cnt {
            #[doc = "Number of values (duty cycles) in this sequence"]
            #[inline(always)]
            pub const fn cnt(&self) -> super::vals::CntCnt {
                let val = (self.0 >> 0usize) & 0x7fff;
                super::vals::CntCnt::from_bits(val as u16)
            }
            #[doc = "Number of values (duty cycles) in this sequence"]
            #[inline(always)]
            pub fn set_cnt(&mut self, val: super::vals::CntCnt) {
                self.0 =
                    (self.0 & !(0x7fff << 0usize)) | (((val.to_bits() as u32) & 0x7fff) << 0usize);
            }
        }
        impl Default for Cnt {
            #[inline(always)]
            fn default() -> Cnt {
                Cnt(0)
            }
        }
        #[doc = "Value up to which the pulse generator counter counts"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Countertop(pub u32);
        impl Countertop {
            #[doc = "Value up to which the pulse generator counter counts. This register is ignored when DECODER.MODE=WaveForm and only values from RAM are used."]
            #[inline(always)]
            pub const fn countertop(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x7fff;
                val as u16
            }
            #[doc = "Value up to which the pulse generator counter counts. This register is ignored when DECODER.MODE=WaveForm and only values from RAM are used."]
            #[inline(always)]
            pub fn set_countertop(&mut self, val: u16) {
                self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
            }
        }
        impl Default for Countertop {
            #[inline(always)]
            fn default() -> Countertop {
                Countertop(0)
            }
        }
        #[doc = "Configuration of the decoder"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Decoder(pub u32);
        impl Decoder {
            #[doc = "How a sequence is read from RAM and spread to the compare register"]
            #[inline(always)]
            pub const fn load(&self) -> super::vals::Load {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Load::from_bits(val as u8)
            }
            #[doc = "How a sequence is read from RAM and spread to the compare register"]
            #[inline(always)]
            pub fn set_load(&mut self, val: super::vals::Load) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
            #[doc = "Selects source for advancing the active sequence"]
            #[inline(always)]
            pub const fn mode(&self) -> super::vals::Mode {
                let val = (self.0 >> 8usize) & 0x01;
                super::vals::Mode::from_bits(val as u8)
            }
            #[doc = "Selects source for advancing the active sequence"]
            #[inline(always)]
            pub fn set_mode(&mut self, val: super::vals::Mode) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
            }
        }
        impl Default for Decoder {
            #[inline(always)]
            fn default() -> Decoder {
                Decoder(0)
            }
        }
        #[doc = "PWM module enable register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable PWM module"]
            #[inline(always)]
            pub const fn enable(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable PWM module"]
            #[inline(always)]
            pub fn set_enable(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        #[doc = "Description cluster: Time added after the sequence"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enddelay(pub u32);
        impl Enddelay {
            #[doc = "Time added after the sequence in PWM periods"]
            #[inline(always)]
            pub const fn cnt(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x00ff_ffff;
                val as u32
            }
            #[doc = "Time added after the sequence in PWM periods"]
            #[inline(always)]
            pub fn set_cnt(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
            }
        }
        impl Default for Enddelay {
            #[inline(always)]
            fn default() -> Enddelay {
                Enddelay(0)
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Enable or disable interrupt for event STOPPED"]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event STOPPED"]
            #[inline(always)]
            pub fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for event SEQSTARTED\\[0\\]"]
            #[inline(always)]
            pub const fn seqstarted0(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event SEQSTARTED\\[0\\]"]
            #[inline(always)]
            pub fn set_seqstarted0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable or disable interrupt for event SEQSTARTED\\[1\\]"]
            #[inline(always)]
            pub const fn seqstarted1(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event SEQSTARTED\\[1\\]"]
            #[inline(always)]
            pub fn set_seqstarted1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable or disable interrupt for event SEQEND\\[0\\]"]
            #[inline(always)]
            pub const fn seqend0(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event SEQEND\\[0\\]"]
            #[inline(always)]
            pub fn set_seqend0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable or disable interrupt for event SEQEND\\[1\\]"]
            #[inline(always)]
            pub const fn seqend1(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event SEQEND\\[1\\]"]
            #[inline(always)]
            pub fn set_seqend1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable or disable interrupt for event PWMPERIODEND"]
            #[inline(always)]
            pub const fn pwmperiodend(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event PWMPERIODEND"]
            #[inline(always)]
            pub fn set_pwmperiodend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Enable or disable interrupt for event LOOPSDONE"]
            #[inline(always)]
            pub const fn loopsdone(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event LOOPSDONE"]
            #[inline(always)]
            pub fn set_loopsdone(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
        #[doc = "Number of playbacks of a loop"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Loop(pub u32);
        impl Loop {
            #[doc = "Number of playbacks of pattern cycles"]
            #[inline(always)]
            pub const fn cnt(&self) -> super::vals::LoopCnt {
                let val = (self.0 >> 0usize) & 0xffff;
                super::vals::LoopCnt::from_bits(val as u16)
            }
            #[doc = "Number of playbacks of pattern cycles"]
            #[inline(always)]
            pub fn set_cnt(&mut self, val: super::vals::LoopCnt) {
                self.0 =
                    (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Loop {
            #[inline(always)]
            fn default() -> Loop {
                Loop(0)
            }
        }
        #[doc = "Selects operating mode of the wave counter"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Mode(pub u32);
        impl Mode {
            #[doc = "Selects up mode or up-and-down mode for the counter"]
            #[inline(always)]
            pub const fn updown(&self) -> super::vals::Updown {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Updown::from_bits(val as u8)
            }
            #[doc = "Selects up mode or up-and-down mode for the counter"]
            #[inline(always)]
            pub fn set_updown(&mut self, val: super::vals::Updown) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Mode {
            #[inline(always)]
            fn default() -> Mode {
                Mode(0)
            }
        }
        #[doc = "Configuration for PWM_CLK"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Prescaler(pub u32);
        impl Prescaler {
            #[doc = "Prescaler of PWM_CLK"]
            #[inline(always)]
            pub const fn prescaler(&self) -> super::vals::Prescaler {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::Prescaler::from_bits(val as u8)
            }
            #[doc = "Prescaler of PWM_CLK"]
            #[inline(always)]
            pub fn set_prescaler(&mut self, val: super::vals::Prescaler) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
            }
        }
        impl Default for Prescaler {
            #[inline(always)]
            fn default() -> Prescaler {
                Prescaler(0)
            }
        }
        #[doc = "Description cluster: Number of additional PWM periods between samples loaded into compare register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Refresh(pub u32);
        impl Refresh {
            #[doc = "Number of additional PWM periods between samples loaded into compare register (load every REFRESH.CNT+1 PWM periods)"]
            #[inline(always)]
            pub const fn cnt(&self) -> super::vals::RefreshCnt {
                let val = (self.0 >> 0usize) & 0x00ff_ffff;
                super::vals::RefreshCnt::from_bits(val as u32)
            }
            #[doc = "Number of additional PWM periods between samples loaded into compare register (load every REFRESH.CNT+1 PWM periods)"]
            #[inline(always)]
            pub fn set_cnt(&mut self, val: super::vals::RefreshCnt) {
                self.0 = (self.0 & !(0x00ff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0x00ff_ffff) << 0usize);
            }
        }
        impl Default for Refresh {
            #[inline(always)]
            fn default() -> Refresh {
                Refresh(0)
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event SEQEND\\[0\\] and task STOP"]
            #[inline(always)]
            pub const fn seqend0_stop(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event SEQEND\\[0\\] and task STOP"]
            #[inline(always)]
            pub fn set_seqend0_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Shortcut between event SEQEND\\[1\\] and task STOP"]
            #[inline(always)]
            pub const fn seqend1_stop(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event SEQEND\\[1\\] and task STOP"]
            #[inline(always)]
            pub fn set_seqend1_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Shortcut between event LOOPSDONE and task SEQSTART\\[0\\]"]
            #[inline(always)]
            pub const fn loopsdone_seqstart0(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event LOOPSDONE and task SEQSTART\\[0\\]"]
            #[inline(always)]
            pub fn set_loopsdone_seqstart0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Shortcut between event LOOPSDONE and task SEQSTART\\[1\\]"]
            #[inline(always)]
            pub const fn loopsdone_seqstart1(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event LOOPSDONE and task SEQSTART\\[1\\]"]
            #[inline(always)]
            pub fn set_loopsdone_seqstart1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Shortcut between event LOOPSDONE and task STOP"]
            #[inline(always)]
            pub const fn loopsdone_stop(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event LOOPSDONE and task STOP"]
            #[inline(always)]
            pub fn set_loopsdone_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
    }
    pub mod vals {
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct CntCnt(pub u16);
        impl CntCnt {
            #[doc = "Sequence is disabled, and shall not be started as it is empty"]
            pub const DISABLED: Self = Self(0x0);
        }
        impl CntCnt {
            pub const fn from_bits(val: u16) -> CntCnt {
                Self(val & 0x7fff)
            }
            pub const fn to_bits(self) -> u16 {
                self.0
            }
        }
        impl From<u16> for CntCnt {
            #[inline(always)]
            fn from(val: u16) -> CntCnt {
                CntCnt::from_bits(val)
            }
        }
        impl From<CntCnt> for u16 {
            #[inline(always)]
            fn from(val: CntCnt) -> u16 {
                CntCnt::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Load {
            #[doc = "1st half word (16-bit) used in all PWM channels 0..3"]
            COMMON = 0x0,
            #[doc = "1st half word (16-bit) used in channel 0..1; 2nd word in channel 2..3"]
            GROUPED = 0x01,
            #[doc = "1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in ch.3"]
            INDIVIDUAL = 0x02,
            #[doc = "1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in COUNTERTOP"]
            WAVE_FORM = 0x03,
        }
        impl Load {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Load {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Load {
            #[inline(always)]
            fn from(val: u8) -> Load {
                Load::from_bits(val)
            }
        }
        impl From<Load> for u8 {
            #[inline(always)]
            fn from(val: Load) -> u8 {
                Load::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct LoopCnt(pub u16);
        impl LoopCnt {
            #[doc = "Looping disabled (stop at the end of the sequence)"]
            pub const DISABLED: Self = Self(0x0);
        }
        impl LoopCnt {
            pub const fn from_bits(val: u16) -> LoopCnt {
                Self(val & 0xffff)
            }
            pub const fn to_bits(self) -> u16 {
                self.0
            }
        }
        impl From<u16> for LoopCnt {
            #[inline(always)]
            fn from(val: u16) -> LoopCnt {
                LoopCnt::from_bits(val)
            }
        }
        impl From<LoopCnt> for u16 {
            #[inline(always)]
            fn from(val: LoopCnt) -> u16 {
                LoopCnt::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Mode {
            #[doc = "SEQ\\[n\\].REFRESH is used to determine loading internal compare registers"]
            REFRESH_COUNT = 0x0,
            #[doc = "NEXTSTEP task causes a new value to be loaded to internal compare registers"]
            NEXT_STEP = 0x01,
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Prescaler {
            #[doc = "Divide by 1 (16 MHz)"]
            DIV_1 = 0x0,
            #[doc = "Divide by 2 (8 MHz)"]
            DIV_2 = 0x01,
            #[doc = "Divide by 4 (4 MHz)"]
            DIV_4 = 0x02,
            #[doc = "Divide by 8 (2 MHz)"]
            DIV_8 = 0x03,
            #[doc = "Divide by 16 (1 MHz)"]
            DIV_16 = 0x04,
            #[doc = "Divide by 32 (500 kHz)"]
            DIV_32 = 0x05,
            #[doc = "Divide by 64 (250 kHz)"]
            DIV_64 = 0x06,
            #[doc = "Divide by 128 (125 kHz)"]
            DIV_128 = 0x07,
        }
        impl Prescaler {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Prescaler {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Prescaler {
            #[inline(always)]
            fn from(val: u8) -> Prescaler {
                Prescaler::from_bits(val)
            }
        }
        impl From<Prescaler> for u8 {
            #[inline(always)]
            fn from(val: Prescaler) -> u8 {
                Prescaler::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct RefreshCnt(pub u32);
        impl RefreshCnt {
            #[doc = "Update every PWM period"]
            pub const CONTINUOUS: Self = Self(0x0);
        }
        impl RefreshCnt {
            pub const fn from_bits(val: u32) -> RefreshCnt {
                Self(val & 0x00ff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl From<u32> for RefreshCnt {
            #[inline(always)]
            fn from(val: u32) -> RefreshCnt {
                RefreshCnt::from_bits(val)
            }
        }
        impl From<RefreshCnt> for u32 {
            #[inline(always)]
            fn from(val: RefreshCnt) -> u32 {
                RefreshCnt::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Updown {
            #[doc = "Up counter, edge-aligned PWM duty cycle"]
            UP = 0x0,
            #[doc = "Up and down counter, center-aligned PWM duty cycle"]
            UP_AND_DOWN = 0x01,
        }
        impl Updown {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Updown {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Updown {
            #[inline(always)]
            fn from(val: u8) -> Updown {
                Updown::from_bits(val)
            }
        }
        impl From<Updown> for u8 {
            #[inline(always)]
            fn from(val: Updown) -> u8 {
                Updown::to_bits(val)
            }
        }
    }
}
pub mod qdec {
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
        #[doc = "Pin select for LED signal"]
        #[inline(always)]
        pub const fn led(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Pin select for A signal"]
        #[inline(always)]
        pub const fn a(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Pin select for B signal"]
        #[inline(always)]
        pub const fn b(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
    }
    #[doc = "Quadrature Decoder"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qdec {
        ptr: *mut u8,
    }
    unsafe impl Send for Qdec {}
    unsafe impl Sync for Qdec {}
    impl Qdec {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Task starting the quadrature decoder"]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Task stopping the quadrature decoder"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Read and clear ACC and ACCDBL"]
        #[inline(always)]
        pub const fn tasks_readclracc(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Read and clear ACC"]
        #[inline(always)]
        pub const fn tasks_rdclracc(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Read and clear ACCDBL"]
        #[inline(always)]
        pub const fn tasks_rdclrdbl(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Event being generated for every new sample value written to the SAMPLE register"]
        #[inline(always)]
        pub const fn events_samplerdy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Non-null report ready"]
        #[inline(always)]
        pub const fn events_reportrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "ACC or ACCDBL register overflow"]
        #[inline(always)]
        pub const fn events_accof(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "Double displacement(s) detected"]
        #[inline(always)]
        pub const fn events_dblrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
        }
        #[doc = "QDEC has been stopped"]
        #[inline(always)]
        pub const fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Enable the quadrature decoder"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "LED output pin polarity"]
        #[inline(always)]
        pub const fn ledpol(self) -> crate::common::Reg<regs::Ledpol, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Sample period"]
        #[inline(always)]
        pub const fn sampleper(self) -> crate::common::Reg<regs::Sampleper, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Motion sample value"]
        #[inline(always)]
        pub const fn sample(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Number of samples to be taken before REPORTRDY and DBLRDY events can be generated"]
        #[inline(always)]
        pub const fn reportper(self) -> crate::common::Reg<regs::Reportper, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "Register accumulating the valid transitions"]
        #[inline(always)]
        pub const fn acc(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
        }
        #[doc = "Snapshot of the ACC register, updated by the READCLRACC or RDCLRACC task"]
        #[inline(always)]
        pub const fn accread(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn psel(self) -> Psel {
            unsafe { Psel::from_ptr(self.ptr.add(0x051cusize) as _) }
        }
        #[doc = "Enable input debounce filters"]
        #[inline(always)]
        pub const fn dbfen(self) -> crate::common::Reg<regs::Dbfen, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0528usize) as _) }
        }
        #[doc = "Time period the LED is switched ON prior to sampling"]
        #[inline(always)]
        pub const fn ledpre(self) -> crate::common::Reg<regs::Ledpre, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize) as _) }
        }
        #[doc = "Register accumulating the number of detected double transitions"]
        #[inline(always)]
        pub const fn accdbl(self) -> crate::common::Reg<regs::Accdbl, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0544usize) as _) }
        }
        #[doc = "Snapshot of the ACCDBL, updated by the READCLRACC or RDCLRDBL task"]
        #[inline(always)]
        pub const fn accdblread(self) -> crate::common::Reg<regs::Accdblread, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0548usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Register accumulating the number of detected double transitions"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Accdbl(pub u32);
        impl Accdbl {
            #[doc = "Register accumulating the number of detected double or illegal transitions. ( SAMPLE = 2 )."]
            #[inline(always)]
            pub const fn accdbl(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "Register accumulating the number of detected double or illegal transitions. ( SAMPLE = 2 )."]
            #[inline(always)]
            pub fn set_accdbl(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Accdbl {
            #[inline(always)]
            fn default() -> Accdbl {
                Accdbl(0)
            }
        }
        #[doc = "Snapshot of the ACCDBL, updated by the READCLRACC or RDCLRDBL task"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Accdblread(pub u32);
        impl Accdblread {
            #[doc = "Snapshot of the ACCDBL register. This field is updated when the READCLRACC or RDCLRDBL task is triggered."]
            #[inline(always)]
            pub const fn accdblread(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "Snapshot of the ACCDBL register. This field is updated when the READCLRACC or RDCLRDBL task is triggered."]
            #[inline(always)]
            pub fn set_accdblread(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Accdblread {
            #[inline(always)]
            fn default() -> Accdblread {
                Accdblread(0)
            }
        }
        #[doc = "Enable input debounce filters"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dbfen(pub u32);
        impl Dbfen {
            #[doc = "Enable input debounce filters"]
            #[inline(always)]
            pub const fn dbfen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable input debounce filters"]
            #[inline(always)]
            pub fn set_dbfen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Dbfen {
            #[inline(always)]
            fn default() -> Dbfen {
                Dbfen(0)
            }
        }
        #[doc = "Enable the quadrature decoder"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable the quadrature decoder"]
            #[inline(always)]
            pub const fn enable(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable the quadrature decoder"]
            #[inline(always)]
            pub fn set_enable(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Write '1' to disable interrupt for event SAMPLERDY"]
            #[inline(always)]
            pub const fn samplerdy(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event SAMPLERDY"]
            #[inline(always)]
            pub fn set_samplerdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to disable interrupt for event REPORTRDY"]
            #[inline(always)]
            pub const fn reportrdy(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event REPORTRDY"]
            #[inline(always)]
            pub fn set_reportrdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to disable interrupt for event ACCOF"]
            #[inline(always)]
            pub const fn accof(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ACCOF"]
            #[inline(always)]
            pub fn set_accof(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Write '1' to disable interrupt for event DBLRDY"]
            #[inline(always)]
            pub const fn dblrdy(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event DBLRDY"]
            #[inline(always)]
            pub fn set_dblrdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Write '1' to disable interrupt for event STOPPED"]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event STOPPED"]
            #[inline(always)]
            pub fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
        #[doc = "LED output pin polarity"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ledpol(pub u32);
        impl Ledpol {
            #[doc = "LED output pin polarity"]
            #[inline(always)]
            pub const fn ledpol(&self) -> super::vals::Ledpol {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Ledpol::from_bits(val as u8)
            }
            #[doc = "LED output pin polarity"]
            #[inline(always)]
            pub fn set_ledpol(&mut self, val: super::vals::Ledpol) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Ledpol {
            #[inline(always)]
            fn default() -> Ledpol {
                Ledpol(0)
            }
        }
        #[doc = "Time period the LED is switched ON prior to sampling"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ledpre(pub u32);
        impl Ledpre {
            #[doc = "Period in us the LED is switched on prior to sampling"]
            #[inline(always)]
            pub const fn ledpre(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x01ff;
                val as u16
            }
            #[doc = "Period in us the LED is switched on prior to sampling"]
            #[inline(always)]
            pub fn set_ledpre(&mut self, val: u16) {
                self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
            }
        }
        impl Default for Ledpre {
            #[inline(always)]
            fn default() -> Ledpre {
                Ledpre(0)
            }
        }
        #[doc = "Number of samples to be taken before REPORTRDY and DBLRDY events can be generated"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Reportper(pub u32);
        impl Reportper {
            #[doc = "Specifies the number of samples to be accumulated in the ACC register before the REPORTRDY and DBLRDY events can be generated"]
            #[inline(always)]
            pub const fn reportper(&self) -> super::vals::Reportper {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Reportper::from_bits(val as u8)
            }
            #[doc = "Specifies the number of samples to be accumulated in the ACC register before the REPORTRDY and DBLRDY events can be generated"]
            #[inline(always)]
            pub fn set_reportper(&mut self, val: super::vals::Reportper) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Reportper {
            #[inline(always)]
            fn default() -> Reportper {
                Reportper(0)
            }
        }
        #[doc = "Sample period"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sampleper(pub u32);
        impl Sampleper {
            #[doc = "Sample period. The SAMPLE register will be updated for every new sample"]
            #[inline(always)]
            pub const fn sampleper(&self) -> super::vals::Sampleper {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Sampleper::from_bits(val as u8)
            }
            #[doc = "Sample period. The SAMPLE register will be updated for every new sample"]
            #[inline(always)]
            pub fn set_sampleper(&mut self, val: super::vals::Sampleper) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Sampleper {
            #[inline(always)]
            fn default() -> Sampleper {
                Sampleper(0)
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event REPORTRDY and task READCLRACC"]
            #[inline(always)]
            pub const fn reportrdy_readclracc(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event REPORTRDY and task READCLRACC"]
            #[inline(always)]
            pub fn set_reportrdy_readclracc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Shortcut between event SAMPLERDY and task STOP"]
            #[inline(always)]
            pub const fn samplerdy_stop(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event SAMPLERDY and task STOP"]
            #[inline(always)]
            pub fn set_samplerdy_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Shortcut between event REPORTRDY and task RDCLRACC"]
            #[inline(always)]
            pub const fn reportrdy_rdclracc(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event REPORTRDY and task RDCLRACC"]
            #[inline(always)]
            pub fn set_reportrdy_rdclracc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Shortcut between event REPORTRDY and task STOP"]
            #[inline(always)]
            pub const fn reportrdy_stop(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event REPORTRDY and task STOP"]
            #[inline(always)]
            pub fn set_reportrdy_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Shortcut between event DBLRDY and task RDCLRDBL"]
            #[inline(always)]
            pub const fn dblrdy_rdclrdbl(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event DBLRDY and task RDCLRDBL"]
            #[inline(always)]
            pub fn set_dblrdy_rdclrdbl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Shortcut between event DBLRDY and task STOP"]
            #[inline(always)]
            pub const fn dblrdy_stop(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event DBLRDY and task STOP"]
            #[inline(always)]
            pub fn set_dblrdy_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Shortcut between event SAMPLERDY and task READCLRACC"]
            #[inline(always)]
            pub const fn samplerdy_readclracc(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event SAMPLERDY and task READCLRACC"]
            #[inline(always)]
            pub fn set_samplerdy_readclracc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Ledpol {
            #[doc = "Led active on output pin low"]
            ACTIVE_LOW = 0x0,
            #[doc = "Led active on output pin high"]
            ACTIVE_HIGH = 0x01,
        }
        impl Ledpol {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Ledpol {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Ledpol {
            #[inline(always)]
            fn from(val: u8) -> Ledpol {
                Ledpol::from_bits(val)
            }
        }
        impl From<Ledpol> for u8 {
            #[inline(always)]
            fn from(val: Ledpol) -> u8 {
                Ledpol::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Reportper {
            #[doc = "10 samples / report"]
            _10SMPL = 0x0,
            #[doc = "40 samples / report"]
            _40SMPL = 0x01,
            #[doc = "80 samples / report"]
            _80SMPL = 0x02,
            #[doc = "120 samples / report"]
            _120SMPL = 0x03,
            #[doc = "160 samples / report"]
            _160SMPL = 0x04,
            #[doc = "200 samples / report"]
            _200SMPL = 0x05,
            #[doc = "240 samples / report"]
            _240SMPL = 0x06,
            #[doc = "280 samples / report"]
            _280SMPL = 0x07,
            #[doc = "1 sample / report"]
            _1SMPL = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Reportper {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Reportper {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Reportper {
            #[inline(always)]
            fn from(val: u8) -> Reportper {
                Reportper::from_bits(val)
            }
        }
        impl From<Reportper> for u8 {
            #[inline(always)]
            fn from(val: Reportper) -> u8 {
                Reportper::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Sampleper {
            #[doc = "128 us"]
            _128US = 0x0,
            #[doc = "256 us"]
            _256US = 0x01,
            #[doc = "512 us"]
            _512US = 0x02,
            #[doc = "1024 us"]
            _1024US = 0x03,
            #[doc = "2048 us"]
            _2048US = 0x04,
            #[doc = "4096 us"]
            _4096US = 0x05,
            #[doc = "8192 us"]
            _8192US = 0x06,
            #[doc = "16384 us"]
            _16384US = 0x07,
            #[doc = "32768 us"]
            _32MS = 0x08,
            #[doc = "65536 us"]
            _65MS = 0x09,
            #[doc = "131072 us"]
            _131MS = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Sampleper {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Sampleper {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Sampleper {
            #[inline(always)]
            fn from(val: u8) -> Sampleper {
                Sampleper::from_bits(val)
            }
        }
        impl From<Sampleper> for u8 {
            #[inline(always)]
            fn from(val: Sampleper) -> u8 {
                Sampleper::to_bits(val)
            }
        }
    }
}
pub mod radio {
    #[doc = "2.4 GHz Radio"]
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Enable RADIO in RX mode"]
        #[inline(always)]
        pub const fn tasks_rxen(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Start RADIO"]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Stop RADIO"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Disable RADIO"]
        #[inline(always)]
        pub const fn tasks_disable(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Start the RSSI and take one single sample of the receive signal strength."]
        #[inline(always)]
        pub const fn tasks_rssistart(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Stop the RSSI measurement"]
        #[inline(always)]
        pub const fn tasks_rssistop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }
        #[doc = "Start the bit counter"]
        #[inline(always)]
        pub const fn tasks_bcstart(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "Stop the bit counter"]
        #[inline(always)]
        pub const fn tasks_bcstop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "RADIO has ramped up and is ready to be started"]
        #[inline(always)]
        pub const fn events_ready(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Address sent or received"]
        #[inline(always)]
        pub const fn events_address(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Packet payload sent or received"]
        #[inline(always)]
        pub const fn events_payload(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "Packet sent or received"]
        #[inline(always)]
        pub const fn events_end(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
        }
        #[doc = "RADIO has been disabled"]
        #[inline(always)]
        pub const fn events_disabled(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
        }
        #[doc = "A device address match occurred on the last received packet"]
        #[inline(always)]
        pub const fn events_devmatch(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
        }
        #[doc = "No device address match occurred on the last received packet"]
        #[inline(always)]
        pub const fn events_devmiss(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
        }
        #[doc = "Sampling of receive signal strength complete."]
        #[inline(always)]
        pub const fn events_rssiend(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
        }
        #[doc = "Bit counter reached bit count value."]
        #[inline(always)]
        pub const fn events_bcmatch(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
        }
        #[doc = "Packet received with CRC ok"]
        #[inline(always)]
        pub const fn events_crcok(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
        }
        #[doc = "Packet received with CRC error"]
        #[inline(always)]
        pub const fn events_crcerror(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "CRC status"]
        #[inline(always)]
        pub const fn crcstatus(self) -> crate::common::Reg<regs::Crcstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "Received address"]
        #[inline(always)]
        pub const fn rxmatch(self) -> crate::common::Reg<regs::Rxmatch, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0408usize) as _) }
        }
        #[doc = "CRC field of previously received packet"]
        #[inline(always)]
        pub const fn rxcrc(self) -> crate::common::Reg<regs::Rxcrc, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x040cusize) as _) }
        }
        #[doc = "Device address match index"]
        #[inline(always)]
        pub const fn dai(self) -> crate::common::Reg<regs::Dai, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0410usize) as _) }
        }
        #[doc = "Packet pointer"]
        #[inline(always)]
        pub const fn packetptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Frequency"]
        #[inline(always)]
        pub const fn frequency(self) -> crate::common::Reg<regs::Frequency, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Output power"]
        #[inline(always)]
        pub const fn txpower(self) -> crate::common::Reg<regs::Txpower, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Data rate and modulation"]
        #[inline(always)]
        pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "Packet configuration register 0"]
        #[inline(always)]
        pub const fn pcnf0(self) -> crate::common::Reg<regs::Pcnf0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
        }
        #[doc = "Packet configuration register 1"]
        #[inline(always)]
        pub const fn pcnf1(self) -> crate::common::Reg<regs::Pcnf1, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
        #[doc = "Base address 0"]
        #[inline(always)]
        pub const fn base0(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
        }
        #[doc = "Base address 1"]
        #[inline(always)]
        pub const fn base1(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize) as _) }
        }
        #[doc = "Prefixes bytes for logical addresses 0-3"]
        #[inline(always)]
        pub const fn prefix0(self) -> crate::common::Reg<regs::Prefix0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
        }
        #[doc = "Prefixes bytes for logical addresses 4-7"]
        #[inline(always)]
        pub const fn prefix1(self) -> crate::common::Reg<regs::Prefix1, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0528usize) as _) }
        }
        #[doc = "Transmit address select"]
        #[inline(always)]
        pub const fn txaddress(self) -> crate::common::Reg<regs::Txaddress, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x052cusize) as _) }
        }
        #[doc = "Receive address select"]
        #[inline(always)]
        pub const fn rxaddresses(self) -> crate::common::Reg<regs::Rxaddresses, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0530usize) as _) }
        }
        #[doc = "CRC configuration"]
        #[inline(always)]
        pub const fn crccnf(self) -> crate::common::Reg<regs::Crccnf, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0534usize) as _) }
        }
        #[doc = "CRC polynomial"]
        #[inline(always)]
        pub const fn crcpoly(self) -> crate::common::Reg<regs::Crcpoly, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0538usize) as _) }
        }
        #[doc = "CRC initial value"]
        #[inline(always)]
        pub const fn crcinit(self) -> crate::common::Reg<regs::Crcinit, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x053cusize) as _) }
        }
        #[doc = "Inter Frame Spacing in us"]
        #[inline(always)]
        pub const fn tifs(self) -> crate::common::Reg<regs::Tifs, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0544usize) as _) }
        }
        #[doc = "RSSI sample"]
        #[inline(always)]
        pub const fn rssisample(self) -> crate::common::Reg<regs::Rssisample, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0548usize) as _) }
        }
        #[doc = "Current radio state"]
        #[inline(always)]
        pub const fn state(self) -> crate::common::Reg<regs::State, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0550usize) as _) }
        }
        #[doc = "Data whitening initial value"]
        #[inline(always)]
        pub const fn datawhiteiv(self) -> crate::common::Reg<regs::Datawhiteiv, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0554usize) as _) }
        }
        #[doc = "Bit counter compare"]
        #[inline(always)]
        pub const fn bcc(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0560usize) as _) }
        }
        #[doc = "Description collection: Device address base segment n"]
        #[inline(always)]
        pub const fn dab(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Device address prefix n"]
        #[inline(always)]
        pub const fn dap(self, n: usize) -> crate::common::Reg<regs::Dap, crate::common::RW> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0620usize + n * 4usize) as _) }
        }
        #[doc = "Device address match configuration"]
        #[inline(always)]
        pub const fn dacnf(self) -> crate::common::Reg<regs::Dacnf, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0640usize) as _) }
        }
        #[doc = "Radio mode configuration register 0"]
        #[inline(always)]
        pub const fn modecnf0(self) -> crate::common::Reg<regs::Modecnf0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0650usize) as _) }
        }
        #[doc = "Peripheral power control"]
        #[inline(always)]
        pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "CRC configuration"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Crccnf(pub u32);
        impl Crccnf {
            #[doc = "CRC length in number of bytes."]
            #[inline(always)]
            pub const fn len(&self) -> super::vals::Len {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Len::from_bits(val as u8)
            }
            #[doc = "CRC length in number of bytes."]
            #[inline(always)]
            pub fn set_len(&mut self, val: super::vals::Len) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
            #[doc = "Include or exclude packet address field out of CRC calculation."]
            #[inline(always)]
            pub const fn skipaddr(&self) -> super::vals::Skipaddr {
                let val = (self.0 >> 8usize) & 0x01;
                super::vals::Skipaddr::from_bits(val as u8)
            }
            #[doc = "Include or exclude packet address field out of CRC calculation."]
            #[inline(always)]
            pub fn set_skipaddr(&mut self, val: super::vals::Skipaddr) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
            }
        }
        impl Default for Crccnf {
            #[inline(always)]
            fn default() -> Crccnf {
                Crccnf(0)
            }
        }
        #[doc = "CRC initial value"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Crcinit(pub u32);
        impl Crcinit {
            #[doc = "CRC initial value"]
            #[inline(always)]
            pub const fn crcinit(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x00ff_ffff;
                val as u32
            }
            #[doc = "CRC initial value"]
            #[inline(always)]
            pub fn set_crcinit(&mut self, val: u32) {
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
        #[doc = "CRC polynomial"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Crcpoly(pub u32);
        impl Crcpoly {
            #[doc = "CRC polynomial"]
            #[inline(always)]
            pub const fn crcpoly(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x00ff_ffff;
                val as u32
            }
            #[doc = "CRC polynomial"]
            #[inline(always)]
            pub fn set_crcpoly(&mut self, val: u32) {
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
        #[doc = "CRC status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Crcstatus(pub u32);
        impl Crcstatus {
            #[doc = "CRC status of packet received"]
            #[inline(always)]
            pub const fn crcstatus(&self) -> super::vals::Crcstatus {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Crcstatus::from_bits(val as u8)
            }
            #[doc = "CRC status of packet received"]
            #[inline(always)]
            pub fn set_crcstatus(&mut self, val: super::vals::Crcstatus) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Crcstatus {
            #[inline(always)]
            fn default() -> Crcstatus {
                Crcstatus(0)
            }
        }
        #[doc = "Device address match configuration"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dacnf(pub u32);
        impl Dacnf {
            #[doc = "Enable or disable device address matching using device address 0"]
            #[inline(always)]
            pub const fn ena0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 0"]
            #[inline(always)]
            pub fn set_ena0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable device address matching using device address 1"]
            #[inline(always)]
            pub const fn ena1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 1"]
            #[inline(always)]
            pub fn set_ena1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable device address matching using device address 2"]
            #[inline(always)]
            pub const fn ena2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 2"]
            #[inline(always)]
            pub fn set_ena2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable or disable device address matching using device address 3"]
            #[inline(always)]
            pub const fn ena3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 3"]
            #[inline(always)]
            pub fn set_ena3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable or disable device address matching using device address 4"]
            #[inline(always)]
            pub const fn ena4(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 4"]
            #[inline(always)]
            pub fn set_ena4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable or disable device address matching using device address 5"]
            #[inline(always)]
            pub const fn ena5(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 5"]
            #[inline(always)]
            pub fn set_ena5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable or disable device address matching using device address 6"]
            #[inline(always)]
            pub const fn ena6(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 6"]
            #[inline(always)]
            pub fn set_ena6(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Enable or disable device address matching using device address 7"]
            #[inline(always)]
            pub const fn ena7(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 7"]
            #[inline(always)]
            pub fn set_ena7(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "TxAdd for device address 0"]
            #[inline(always)]
            pub const fn txadd0(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 0"]
            #[inline(always)]
            pub fn set_txadd0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "TxAdd for device address 1"]
            #[inline(always)]
            pub const fn txadd1(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 1"]
            #[inline(always)]
            pub fn set_txadd1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "TxAdd for device address 2"]
            #[inline(always)]
            pub const fn txadd2(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 2"]
            #[inline(always)]
            pub fn set_txadd2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "TxAdd for device address 3"]
            #[inline(always)]
            pub const fn txadd3(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 3"]
            #[inline(always)]
            pub fn set_txadd3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "TxAdd for device address 4"]
            #[inline(always)]
            pub const fn txadd4(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 4"]
            #[inline(always)]
            pub fn set_txadd4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "TxAdd for device address 5"]
            #[inline(always)]
            pub const fn txadd5(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 5"]
            #[inline(always)]
            pub fn set_txadd5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "TxAdd for device address 6"]
            #[inline(always)]
            pub const fn txadd6(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 6"]
            #[inline(always)]
            pub fn set_txadd6(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "TxAdd for device address 7"]
            #[inline(always)]
            pub const fn txadd7(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 7"]
            #[inline(always)]
            pub fn set_txadd7(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
        }
        impl Default for Dacnf {
            #[inline(always)]
            fn default() -> Dacnf {
                Dacnf(0)
            }
        }
        #[doc = "Device address match index"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dai(pub u32);
        impl Dai {
            #[doc = "Device address match index"]
            #[inline(always)]
            pub const fn dai(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x07;
                val as u8
            }
            #[doc = "Device address match index"]
            #[inline(always)]
            pub fn set_dai(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
            }
        }
        impl Default for Dai {
            #[inline(always)]
            fn default() -> Dai {
                Dai(0)
            }
        }
        #[doc = "Description collection: Device address prefix n"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dap(pub u32);
        impl Dap {
            #[doc = "Device address prefix n"]
            #[inline(always)]
            pub const fn dap(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Device address prefix n"]
            #[inline(always)]
            pub fn set_dap(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Dap {
            #[inline(always)]
            fn default() -> Dap {
                Dap(0)
            }
        }
        #[doc = "Data whitening initial value"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Datawhiteiv(pub u32);
        impl Datawhiteiv {
            #[doc = "Data whitening initial value. Bit 6 is hard-wired to '1', writing '0' to it has no effect, and it will always be read back and used by the device as '1'."]
            #[inline(always)]
            pub const fn datawhiteiv(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "Data whitening initial value. Bit 6 is hard-wired to '1', writing '0' to it has no effect, and it will always be read back and used by the device as '1'."]
            #[inline(always)]
            pub fn set_datawhiteiv(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
            }
        }
        impl Default for Datawhiteiv {
            #[inline(always)]
            fn default() -> Datawhiteiv {
                Datawhiteiv(0)
            }
        }
        #[doc = "Frequency"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Frequency(pub u32);
        impl Frequency {
            #[doc = "Radio channel frequency"]
            #[inline(always)]
            pub const fn frequency(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "Radio channel frequency"]
            #[inline(always)]
            pub fn set_frequency(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
            }
            #[doc = "Channel map selection."]
            #[inline(always)]
            pub const fn map(&self) -> super::vals::Map {
                let val = (self.0 >> 8usize) & 0x01;
                super::vals::Map::from_bits(val as u8)
            }
            #[doc = "Channel map selection."]
            #[inline(always)]
            pub fn set_map(&mut self, val: super::vals::Map) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
            }
        }
        impl Default for Frequency {
            #[inline(always)]
            fn default() -> Frequency {
                Frequency(0)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Write '1' to disable interrupt for event READY"]
            #[inline(always)]
            pub const fn ready(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event READY"]
            #[inline(always)]
            pub fn set_ready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to disable interrupt for event ADDRESS"]
            #[inline(always)]
            pub const fn address(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ADDRESS"]
            #[inline(always)]
            pub fn set_address(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to disable interrupt for event PAYLOAD"]
            #[inline(always)]
            pub const fn payload(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event PAYLOAD"]
            #[inline(always)]
            pub fn set_payload(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Write '1' to disable interrupt for event END"]
            #[inline(always)]
            pub const fn end(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event END"]
            #[inline(always)]
            pub fn set_end(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Write '1' to disable interrupt for event DISABLED"]
            #[inline(always)]
            pub const fn disabled(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event DISABLED"]
            #[inline(always)]
            pub fn set_disabled(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Write '1' to disable interrupt for event DEVMATCH"]
            #[inline(always)]
            pub const fn devmatch(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event DEVMATCH"]
            #[inline(always)]
            pub fn set_devmatch(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Write '1' to disable interrupt for event DEVMISS"]
            #[inline(always)]
            pub const fn devmiss(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event DEVMISS"]
            #[inline(always)]
            pub fn set_devmiss(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Write '1' to disable interrupt for event RSSIEND"]
            #[inline(always)]
            pub const fn rssiend(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event RSSIEND"]
            #[inline(always)]
            pub fn set_rssiend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Write '1' to disable interrupt for event BCMATCH"]
            #[inline(always)]
            pub const fn bcmatch(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event BCMATCH"]
            #[inline(always)]
            pub fn set_bcmatch(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Write '1' to disable interrupt for event CRCOK"]
            #[inline(always)]
            pub const fn crcok(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event CRCOK"]
            #[inline(always)]
            pub fn set_crcok(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "Write '1' to disable interrupt for event CRCERROR"]
            #[inline(always)]
            pub const fn crcerror(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event CRCERROR"]
            #[inline(always)]
            pub fn set_crcerror(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
        #[doc = "Data rate and modulation"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Mode(pub u32);
        impl Mode {
            #[doc = "Radio data rate and modulation setting. The radio supports Frequency-shift Keying (FSK) modulation."]
            #[inline(always)]
            pub const fn mode(&self) -> super::vals::Mode {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Mode::from_bits(val as u8)
            }
            #[doc = "Radio data rate and modulation setting. The radio supports Frequency-shift Keying (FSK) modulation."]
            #[inline(always)]
            pub fn set_mode(&mut self, val: super::vals::Mode) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Mode {
            #[inline(always)]
            fn default() -> Mode {
                Mode(0)
            }
        }
        #[doc = "Radio mode configuration register 0"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Modecnf0(pub u32);
        impl Modecnf0 {
            #[doc = "Radio ramp-up time"]
            #[inline(always)]
            pub const fn ru(&self) -> super::vals::Ru {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Ru::from_bits(val as u8)
            }
            #[doc = "Radio ramp-up time"]
            #[inline(always)]
            pub fn set_ru(&mut self, val: super::vals::Ru) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Default TX value"]
            #[inline(always)]
            pub const fn dtx(&self) -> super::vals::Dtx {
                let val = (self.0 >> 8usize) & 0x03;
                super::vals::Dtx::from_bits(val as u8)
            }
            #[doc = "Default TX value"]
            #[inline(always)]
            pub fn set_dtx(&mut self, val: super::vals::Dtx) {
                self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
            }
        }
        impl Default for Modecnf0 {
            #[inline(always)]
            fn default() -> Modecnf0 {
                Modecnf0(0)
            }
        }
        #[doc = "Packet configuration register 0"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pcnf0(pub u32);
        impl Pcnf0 {
            #[doc = "Length on air of LENGTH field in number of bits."]
            #[inline(always)]
            pub const fn lflen(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "Length on air of LENGTH field in number of bits."]
            #[inline(always)]
            pub fn set_lflen(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
            #[doc = "Length on air of S0 field in number of bytes."]
            #[inline(always)]
            pub const fn s0len(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Length on air of S0 field in number of bytes."]
            #[inline(always)]
            pub fn set_s0len(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Length on air of S1 field in number of bits."]
            #[inline(always)]
            pub const fn s1len(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0x0f;
                val as u8
            }
            #[doc = "Length on air of S1 field in number of bits."]
            #[inline(always)]
            pub fn set_s1len(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
            }
            #[doc = "Include or exclude S1 field in RAM"]
            #[inline(always)]
            pub const fn s1incl(&self) -> super::vals::S1incl {
                let val = (self.0 >> 20usize) & 0x01;
                super::vals::S1incl::from_bits(val as u8)
            }
            #[doc = "Include or exclude S1 field in RAM"]
            #[inline(always)]
            pub fn set_s1incl(&mut self, val: super::vals::S1incl) {
                self.0 =
                    (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
            }
            #[doc = "Length of preamble on air. Decision point: TASKS_START task"]
            #[inline(always)]
            pub const fn plen(&self) -> super::vals::Plen {
                let val = (self.0 >> 24usize) & 0x01;
                super::vals::Plen::from_bits(val as u8)
            }
            #[doc = "Length of preamble on air. Decision point: TASKS_START task"]
            #[inline(always)]
            pub fn set_plen(&mut self, val: super::vals::Plen) {
                self.0 =
                    (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
            }
        }
        impl Default for Pcnf0 {
            #[inline(always)]
            fn default() -> Pcnf0 {
                Pcnf0(0)
            }
        }
        #[doc = "Packet configuration register 1"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pcnf1(pub u32);
        impl Pcnf1 {
            #[doc = "Maximum length of packet payload. If the packet payload is larger than MAXLEN, the radio will truncate the payload to MAXLEN."]
            #[inline(always)]
            pub const fn maxlen(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Maximum length of packet payload. If the packet payload is larger than MAXLEN, the radio will truncate the payload to MAXLEN."]
            #[inline(always)]
            pub fn set_maxlen(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "Static length in number of bytes"]
            #[inline(always)]
            pub const fn statlen(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "Static length in number of bytes"]
            #[inline(always)]
            pub fn set_statlen(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[doc = "Base address length in number of bytes"]
            #[inline(always)]
            pub const fn balen(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0x07;
                val as u8
            }
            #[doc = "Base address length in number of bytes"]
            #[inline(always)]
            pub fn set_balen(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
            }
            #[doc = "On air endianness of packet, this applies to the S0, LENGTH, S1 and the PAYLOAD fields."]
            #[inline(always)]
            pub const fn endian(&self) -> super::vals::Endian {
                let val = (self.0 >> 24usize) & 0x01;
                super::vals::Endian::from_bits(val as u8)
            }
            #[doc = "On air endianness of packet, this applies to the S0, LENGTH, S1 and the PAYLOAD fields."]
            #[inline(always)]
            pub fn set_endian(&mut self, val: super::vals::Endian) {
                self.0 =
                    (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
            }
            #[doc = "Enable or disable packet whitening"]
            #[inline(always)]
            pub const fn whiteen(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable packet whitening"]
            #[inline(always)]
            pub fn set_whiteen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
        }
        impl Default for Pcnf1 {
            #[inline(always)]
            fn default() -> Pcnf1 {
                Pcnf1(0)
            }
        }
        #[doc = "Peripheral power control"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Power(pub u32);
        impl Power {
            #[doc = "Peripheral power control. The peripheral and its registers will be reset to its initial state by switching the peripheral off and then back on again."]
            #[inline(always)]
            pub const fn power(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Peripheral power control. The peripheral and its registers will be reset to its initial state by switching the peripheral off and then back on again."]
            #[inline(always)]
            pub fn set_power(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Power {
            #[inline(always)]
            fn default() -> Power {
                Power(0)
            }
        }
        #[doc = "Prefixes bytes for logical addresses 0-3"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Prefix0(pub u32);
        impl Prefix0 {
            #[doc = "Address prefix 0."]
            #[inline(always)]
            pub const fn ap0(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 0."]
            #[inline(always)]
            pub fn set_ap0(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "Address prefix 1."]
            #[inline(always)]
            pub const fn ap1(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 1."]
            #[inline(always)]
            pub fn set_ap1(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[doc = "Address prefix 2."]
            #[inline(always)]
            pub const fn ap2(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 2."]
            #[inline(always)]
            pub fn set_ap2(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
            }
            #[doc = "Address prefix 3."]
            #[inline(always)]
            pub const fn ap3(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 3."]
            #[inline(always)]
            pub fn set_ap3(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
            }
        }
        impl Default for Prefix0 {
            #[inline(always)]
            fn default() -> Prefix0 {
                Prefix0(0)
            }
        }
        #[doc = "Prefixes bytes for logical addresses 4-7"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Prefix1(pub u32);
        impl Prefix1 {
            #[doc = "Address prefix 4."]
            #[inline(always)]
            pub const fn ap4(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 4."]
            #[inline(always)]
            pub fn set_ap4(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "Address prefix 5."]
            #[inline(always)]
            pub const fn ap5(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 5."]
            #[inline(always)]
            pub fn set_ap5(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[doc = "Address prefix 6."]
            #[inline(always)]
            pub const fn ap6(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 6."]
            #[inline(always)]
            pub fn set_ap6(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
            }
            #[doc = "Address prefix 7."]
            #[inline(always)]
            pub const fn ap7(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 7."]
            #[inline(always)]
            pub fn set_ap7(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
            }
        }
        impl Default for Prefix1 {
            #[inline(always)]
            fn default() -> Prefix1 {
                Prefix1(0)
            }
        }
        #[doc = "RSSI sample"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rssisample(pub u32);
        impl Rssisample {
            #[doc = "RSSI sample"]
            #[inline(always)]
            pub const fn rssisample(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "RSSI sample"]
            #[inline(always)]
            pub fn set_rssisample(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
            }
        }
        impl Default for Rssisample {
            #[inline(always)]
            fn default() -> Rssisample {
                Rssisample(0)
            }
        }
        #[doc = "Receive address select"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxaddresses(pub u32);
        impl Rxaddresses {
            #[doc = "Enable or disable reception on logical address 0."]
            #[inline(always)]
            pub const fn addr0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable reception on logical address 0."]
            #[inline(always)]
            pub fn set_addr0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable reception on logical address 1."]
            #[inline(always)]
            pub const fn addr1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable reception on logical address 1."]
            #[inline(always)]
            pub fn set_addr1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable reception on logical address 2."]
            #[inline(always)]
            pub const fn addr2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable reception on logical address 2."]
            #[inline(always)]
            pub fn set_addr2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable or disable reception on logical address 3."]
            #[inline(always)]
            pub const fn addr3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable reception on logical address 3."]
            #[inline(always)]
            pub fn set_addr3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable or disable reception on logical address 4."]
            #[inline(always)]
            pub const fn addr4(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable reception on logical address 4."]
            #[inline(always)]
            pub fn set_addr4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable or disable reception on logical address 5."]
            #[inline(always)]
            pub const fn addr5(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable reception on logical address 5."]
            #[inline(always)]
            pub fn set_addr5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable or disable reception on logical address 6."]
            #[inline(always)]
            pub const fn addr6(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable reception on logical address 6."]
            #[inline(always)]
            pub fn set_addr6(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Enable or disable reception on logical address 7."]
            #[inline(always)]
            pub const fn addr7(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable reception on logical address 7."]
            #[inline(always)]
            pub fn set_addr7(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
        }
        impl Default for Rxaddresses {
            #[inline(always)]
            fn default() -> Rxaddresses {
                Rxaddresses(0)
            }
        }
        #[doc = "CRC field of previously received packet"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxcrc(pub u32);
        impl Rxcrc {
            #[doc = "CRC field of previously received packet"]
            #[inline(always)]
            pub const fn rxcrc(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x00ff_ffff;
                val as u32
            }
            #[doc = "CRC field of previously received packet"]
            #[inline(always)]
            pub fn set_rxcrc(&mut self, val: u32) {
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
        #[doc = "Received address"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxmatch(pub u32);
        impl Rxmatch {
            #[doc = "Received address"]
            #[inline(always)]
            pub const fn rxmatch(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x07;
                val as u8
            }
            #[doc = "Received address"]
            #[inline(always)]
            pub fn set_rxmatch(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
            }
        }
        impl Default for Rxmatch {
            #[inline(always)]
            fn default() -> Rxmatch {
                Rxmatch(0)
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event READY and task START"]
            #[inline(always)]
            pub const fn ready_start(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event READY and task START"]
            #[inline(always)]
            pub fn set_ready_start(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Shortcut between event END and task DISABLE"]
            #[inline(always)]
            pub const fn end_disable(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event END and task DISABLE"]
            #[inline(always)]
            pub fn set_end_disable(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Shortcut between event DISABLED and task TXEN"]
            #[inline(always)]
            pub const fn disabled_txen(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event DISABLED and task TXEN"]
            #[inline(always)]
            pub fn set_disabled_txen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Shortcut between event DISABLED and task RXEN"]
            #[inline(always)]
            pub const fn disabled_rxen(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event DISABLED and task RXEN"]
            #[inline(always)]
            pub fn set_disabled_rxen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Shortcut between event ADDRESS and task RSSISTART"]
            #[inline(always)]
            pub const fn address_rssistart(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event ADDRESS and task RSSISTART"]
            #[inline(always)]
            pub fn set_address_rssistart(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Shortcut between event END and task START"]
            #[inline(always)]
            pub const fn end_start(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event END and task START"]
            #[inline(always)]
            pub fn set_end_start(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Shortcut between event ADDRESS and task BCSTART"]
            #[inline(always)]
            pub const fn address_bcstart(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event ADDRESS and task BCSTART"]
            #[inline(always)]
            pub fn set_address_bcstart(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Shortcut between event DISABLED and task RSSISTOP"]
            #[inline(always)]
            pub const fn disabled_rssistop(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event DISABLED and task RSSISTOP"]
            #[inline(always)]
            pub fn set_disabled_rssistop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        #[doc = "Current radio state"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct State(pub u32);
        impl State {
            #[doc = "Current radio state"]
            #[inline(always)]
            pub const fn state(&self) -> super::vals::State {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::State::from_bits(val as u8)
            }
            #[doc = "Current radio state"]
            #[inline(always)]
            pub fn set_state(&mut self, val: super::vals::State) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for State {
            #[inline(always)]
            fn default() -> State {
                State(0)
            }
        }
        #[doc = "Inter Frame Spacing in us"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Tifs(pub u32);
        impl Tifs {
            #[doc = "Inter Frame Spacing in us"]
            #[inline(always)]
            pub const fn tifs(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Inter Frame Spacing in us"]
            #[inline(always)]
            pub fn set_tifs(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Tifs {
            #[inline(always)]
            fn default() -> Tifs {
                Tifs(0)
            }
        }
        #[doc = "Transmit address select"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Txaddress(pub u32);
        impl Txaddress {
            #[doc = "Transmit address select"]
            #[inline(always)]
            pub const fn txaddress(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x07;
                val as u8
            }
            #[doc = "Transmit address select"]
            #[inline(always)]
            pub fn set_txaddress(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
            }
        }
        impl Default for Txaddress {
            #[inline(always)]
            fn default() -> Txaddress {
                Txaddress(0)
            }
        }
        #[doc = "Output power"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Txpower(pub u32);
        impl Txpower {
            #[doc = "RADIO output power."]
            #[inline(always)]
            pub const fn txpower(&self) -> super::vals::Txpower {
                let val = (self.0 >> 0usize) & 0xff;
                super::vals::Txpower::from_bits(val as u8)
            }
            #[doc = "RADIO output power."]
            #[inline(always)]
            pub fn set_txpower(&mut self, val: super::vals::Txpower) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Txpower {
            #[inline(always)]
            fn default() -> Txpower {
                Txpower(0)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Endian {
            #[doc = "Least Significant bit on air first"]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Map {
            #[doc = "Channel map between 2400 MHZ .. 2500 MHz"]
            DEFAULT = 0x0,
            #[doc = "Channel map between 2360 MHZ .. 2460 MHz"]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Mode {
            #[doc = "1 Mbit/s Nordic proprietary radio mode"]
            NRF_1MBIT = 0x0,
            #[doc = "2 Mbit/s Nordic proprietary radio mode"]
            NRF_2MBIT = 0x01,
            _RESERVED_2 = 0x02,
            #[doc = "1 Mbit/s Bluetooth Low Energy"]
            BLE_1MBIT = 0x03,
            #[doc = "2 Mbit/s Bluetooth Low Energy"]
            BLE_2MBIT = 0x04,
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Plen {
            #[doc = "8-bit preamble"]
            _8BIT = 0x0,
            #[doc = "16-bit preamble"]
            _16BIT = 0x01,
        }
        impl Plen {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Plen {
                unsafe { core::mem::transmute(val & 0x01) }
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Ru {
            #[doc = "Default ramp-up time (tRXEN), compatible with firmware written for nRF51"]
            DEFAULT = 0x0,
            #[doc = "Fast ramp-up (tRXEN,FAST), see electrical specification for more information"]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Skipaddr {
            #[doc = "CRC calculation includes address field"]
            INCLUDE = 0x0,
            #[doc = "CRC calculation does not include address field. The CRC calculation will start at the first byte after the address."]
            SKIP = 0x01,
        }
        impl Skipaddr {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Skipaddr {
                unsafe { core::mem::transmute(val & 0x01) }
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Txpower {
            #[doc = "0 dBm"]
            _0_DBM = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            #[doc = "+3 dBm"]
            POS3_DBM = 0x03,
            #[doc = "+4 dBm"]
            POS4_DBM = 0x04,
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
            _RESERVED_10 = 0x10,
            _RESERVED_11 = 0x11,
            _RESERVED_12 = 0x12,
            _RESERVED_13 = 0x13,
            _RESERVED_14 = 0x14,
            _RESERVED_15 = 0x15,
            _RESERVED_16 = 0x16,
            _RESERVED_17 = 0x17,
            _RESERVED_18 = 0x18,
            _RESERVED_19 = 0x19,
            _RESERVED_1a = 0x1a,
            _RESERVED_1b = 0x1b,
            _RESERVED_1c = 0x1c,
            _RESERVED_1d = 0x1d,
            _RESERVED_1e = 0x1e,
            _RESERVED_1f = 0x1f,
            _RESERVED_20 = 0x20,
            _RESERVED_21 = 0x21,
            _RESERVED_22 = 0x22,
            _RESERVED_23 = 0x23,
            _RESERVED_24 = 0x24,
            _RESERVED_25 = 0x25,
            _RESERVED_26 = 0x26,
            _RESERVED_27 = 0x27,
            _RESERVED_28 = 0x28,
            _RESERVED_29 = 0x29,
            _RESERVED_2a = 0x2a,
            _RESERVED_2b = 0x2b,
            _RESERVED_2c = 0x2c,
            _RESERVED_2d = 0x2d,
            _RESERVED_2e = 0x2e,
            _RESERVED_2f = 0x2f,
            _RESERVED_30 = 0x30,
            _RESERVED_31 = 0x31,
            _RESERVED_32 = 0x32,
            _RESERVED_33 = 0x33,
            _RESERVED_34 = 0x34,
            _RESERVED_35 = 0x35,
            _RESERVED_36 = 0x36,
            _RESERVED_37 = 0x37,
            _RESERVED_38 = 0x38,
            _RESERVED_39 = 0x39,
            _RESERVED_3a = 0x3a,
            _RESERVED_3b = 0x3b,
            _RESERVED_3c = 0x3c,
            _RESERVED_3d = 0x3d,
            _RESERVED_3e = 0x3e,
            _RESERVED_3f = 0x3f,
            _RESERVED_40 = 0x40,
            _RESERVED_41 = 0x41,
            _RESERVED_42 = 0x42,
            _RESERVED_43 = 0x43,
            _RESERVED_44 = 0x44,
            _RESERVED_45 = 0x45,
            _RESERVED_46 = 0x46,
            _RESERVED_47 = 0x47,
            _RESERVED_48 = 0x48,
            _RESERVED_49 = 0x49,
            _RESERVED_4a = 0x4a,
            _RESERVED_4b = 0x4b,
            _RESERVED_4c = 0x4c,
            _RESERVED_4d = 0x4d,
            _RESERVED_4e = 0x4e,
            _RESERVED_4f = 0x4f,
            _RESERVED_50 = 0x50,
            _RESERVED_51 = 0x51,
            _RESERVED_52 = 0x52,
            _RESERVED_53 = 0x53,
            _RESERVED_54 = 0x54,
            _RESERVED_55 = 0x55,
            _RESERVED_56 = 0x56,
            _RESERVED_57 = 0x57,
            _RESERVED_58 = 0x58,
            _RESERVED_59 = 0x59,
            _RESERVED_5a = 0x5a,
            _RESERVED_5b = 0x5b,
            _RESERVED_5c = 0x5c,
            _RESERVED_5d = 0x5d,
            _RESERVED_5e = 0x5e,
            _RESERVED_5f = 0x5f,
            _RESERVED_60 = 0x60,
            _RESERVED_61 = 0x61,
            _RESERVED_62 = 0x62,
            _RESERVED_63 = 0x63,
            _RESERVED_64 = 0x64,
            _RESERVED_65 = 0x65,
            _RESERVED_66 = 0x66,
            _RESERVED_67 = 0x67,
            _RESERVED_68 = 0x68,
            _RESERVED_69 = 0x69,
            _RESERVED_6a = 0x6a,
            _RESERVED_6b = 0x6b,
            _RESERVED_6c = 0x6c,
            _RESERVED_6d = 0x6d,
            _RESERVED_6e = 0x6e,
            _RESERVED_6f = 0x6f,
            _RESERVED_70 = 0x70,
            _RESERVED_71 = 0x71,
            _RESERVED_72 = 0x72,
            _RESERVED_73 = 0x73,
            _RESERVED_74 = 0x74,
            _RESERVED_75 = 0x75,
            _RESERVED_76 = 0x76,
            _RESERVED_77 = 0x77,
            _RESERVED_78 = 0x78,
            _RESERVED_79 = 0x79,
            _RESERVED_7a = 0x7a,
            _RESERVED_7b = 0x7b,
            _RESERVED_7c = 0x7c,
            _RESERVED_7d = 0x7d,
            _RESERVED_7e = 0x7e,
            _RESERVED_7f = 0x7f,
            _RESERVED_80 = 0x80,
            _RESERVED_81 = 0x81,
            _RESERVED_82 = 0x82,
            _RESERVED_83 = 0x83,
            _RESERVED_84 = 0x84,
            _RESERVED_85 = 0x85,
            _RESERVED_86 = 0x86,
            _RESERVED_87 = 0x87,
            _RESERVED_88 = 0x88,
            _RESERVED_89 = 0x89,
            _RESERVED_8a = 0x8a,
            _RESERVED_8b = 0x8b,
            _RESERVED_8c = 0x8c,
            _RESERVED_8d = 0x8d,
            _RESERVED_8e = 0x8e,
            _RESERVED_8f = 0x8f,
            _RESERVED_90 = 0x90,
            _RESERVED_91 = 0x91,
            _RESERVED_92 = 0x92,
            _RESERVED_93 = 0x93,
            _RESERVED_94 = 0x94,
            _RESERVED_95 = 0x95,
            _RESERVED_96 = 0x96,
            _RESERVED_97 = 0x97,
            _RESERVED_98 = 0x98,
            _RESERVED_99 = 0x99,
            _RESERVED_9a = 0x9a,
            _RESERVED_9b = 0x9b,
            _RESERVED_9c = 0x9c,
            _RESERVED_9d = 0x9d,
            _RESERVED_9e = 0x9e,
            _RESERVED_9f = 0x9f,
            _RESERVED_a0 = 0xa0,
            _RESERVED_a1 = 0xa1,
            _RESERVED_a2 = 0xa2,
            _RESERVED_a3 = 0xa3,
            _RESERVED_a4 = 0xa4,
            _RESERVED_a5 = 0xa5,
            _RESERVED_a6 = 0xa6,
            _RESERVED_a7 = 0xa7,
            _RESERVED_a8 = 0xa8,
            _RESERVED_a9 = 0xa9,
            _RESERVED_aa = 0xaa,
            _RESERVED_ab = 0xab,
            _RESERVED_ac = 0xac,
            _RESERVED_ad = 0xad,
            _RESERVED_ae = 0xae,
            _RESERVED_af = 0xaf,
            _RESERVED_b0 = 0xb0,
            _RESERVED_b1 = 0xb1,
            _RESERVED_b2 = 0xb2,
            _RESERVED_b3 = 0xb3,
            _RESERVED_b4 = 0xb4,
            _RESERVED_b5 = 0xb5,
            _RESERVED_b6 = 0xb6,
            _RESERVED_b7 = 0xb7,
            _RESERVED_b8 = 0xb8,
            _RESERVED_b9 = 0xb9,
            _RESERVED_ba = 0xba,
            _RESERVED_bb = 0xbb,
            _RESERVED_bc = 0xbc,
            _RESERVED_bd = 0xbd,
            _RESERVED_be = 0xbe,
            _RESERVED_bf = 0xbf,
            _RESERVED_c0 = 0xc0,
            _RESERVED_c1 = 0xc1,
            _RESERVED_c2 = 0xc2,
            _RESERVED_c3 = 0xc3,
            _RESERVED_c4 = 0xc4,
            _RESERVED_c5 = 0xc5,
            _RESERVED_c6 = 0xc6,
            _RESERVED_c7 = 0xc7,
            _RESERVED_c8 = 0xc8,
            _RESERVED_c9 = 0xc9,
            _RESERVED_ca = 0xca,
            _RESERVED_cb = 0xcb,
            _RESERVED_cc = 0xcc,
            _RESERVED_cd = 0xcd,
            _RESERVED_ce = 0xce,
            _RESERVED_cf = 0xcf,
            _RESERVED_d0 = 0xd0,
            _RESERVED_d1 = 0xd1,
            _RESERVED_d2 = 0xd2,
            _RESERVED_d3 = 0xd3,
            _RESERVED_d4 = 0xd4,
            _RESERVED_d5 = 0xd5,
            _RESERVED_d6 = 0xd6,
            _RESERVED_d7 = 0xd7,
            #[doc = "-40 dBm"]
            NEG40_DBM = 0xd8,
            _RESERVED_d9 = 0xd9,
            _RESERVED_da = 0xda,
            _RESERVED_db = 0xdb,
            _RESERVED_dc = 0xdc,
            _RESERVED_dd = 0xdd,
            _RESERVED_de = 0xde,
            _RESERVED_df = 0xdf,
            _RESERVED_e0 = 0xe0,
            _RESERVED_e1 = 0xe1,
            #[doc = "Deprecated enumerator - -40 dBm"]
            NEG30_DBM = 0xe2,
            _RESERVED_e3 = 0xe3,
            _RESERVED_e4 = 0xe4,
            _RESERVED_e5 = 0xe5,
            _RESERVED_e6 = 0xe6,
            _RESERVED_e7 = 0xe7,
            _RESERVED_e8 = 0xe8,
            _RESERVED_e9 = 0xe9,
            _RESERVED_ea = 0xea,
            _RESERVED_eb = 0xeb,
            #[doc = "-20 dBm"]
            NEG20_DBM = 0xec,
            _RESERVED_ed = 0xed,
            _RESERVED_ee = 0xee,
            _RESERVED_ef = 0xef,
            #[doc = "-16 dBm"]
            NEG16_DBM = 0xf0,
            _RESERVED_f1 = 0xf1,
            _RESERVED_f2 = 0xf2,
            _RESERVED_f3 = 0xf3,
            #[doc = "-12 dBm"]
            NEG12_DBM = 0xf4,
            _RESERVED_f5 = 0xf5,
            _RESERVED_f6 = 0xf6,
            _RESERVED_f7 = 0xf7,
            #[doc = "-8 dBm"]
            NEG8_DBM = 0xf8,
            _RESERVED_f9 = 0xf9,
            _RESERVED_fa = 0xfa,
            _RESERVED_fb = 0xfb,
            #[doc = "-4 dBm"]
            NEG4_DBM = 0xfc,
            _RESERVED_fd = 0xfd,
            _RESERVED_fe = 0xfe,
            _RESERVED_ff = 0xff,
        }
        impl Txpower {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Txpower {
                unsafe { core::mem::transmute(val & 0xff) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Task stopping the random number generator"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Event being generated for every new random number written to the VALUE register"]
        #[inline(always)]
        pub const fn events_valrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Configuration register"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Output random number"]
        #[inline(always)]
        pub const fn value(self) -> crate::common::Reg<regs::Value, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Bias correction"]
            #[inline(always)]
            pub const fn dercen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Bias correction"]
            #[inline(always)]
            pub fn set_dercen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Write '1' to disable interrupt for event VALRDY"]
            #[inline(always)]
            pub const fn valrdy(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event VALRDY"]
            #[inline(always)]
            pub fn set_valrdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event VALRDY and task STOP"]
            #[inline(always)]
            pub const fn valrdy_stop(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event VALRDY and task STOP"]
            #[inline(always)]
            pub fn set_valrdy_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        #[doc = "Output random number"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Value(pub u32);
        impl Value {
            #[doc = "Generated random number"]
            #[inline(always)]
            pub const fn value(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Generated random number"]
            #[inline(always)]
            pub fn set_value(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Value {
            #[inline(always)]
            fn default() -> Value {
                Value(0)
            }
        }
    }
}
pub mod rtc {
    #[doc = "Real time counter 0"]
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
        #[doc = "Start RTC COUNTER"]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop RTC COUNTER"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Clear RTC COUNTER"]
        #[inline(always)]
        pub const fn tasks_clear(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Set COUNTER to 0xFFFFF0"]
        #[inline(always)]
        pub const fn tasks_trigovrflw(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Event on COUNTER increment"]
        #[inline(always)]
        pub const fn events_tick(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Event on COUNTER overflow"]
        #[inline(always)]
        pub const fn events_ovrflw(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Description collection: Compare event on CC\\[n\\] match"]
        #[inline(always)]
        pub const fn events_compare(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize + n * 4usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Enable or disable event routing"]
        #[inline(always)]
        pub const fn evten(self) -> crate::common::Reg<regs::Evten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0340usize) as _) }
        }
        #[doc = "Enable event routing"]
        #[inline(always)]
        pub const fn evtenset(self) -> crate::common::Reg<regs::Evtenset, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0344usize) as _) }
        }
        #[doc = "Disable event routing"]
        #[inline(always)]
        pub const fn evtenclr(self) -> crate::common::Reg<regs::Evtenclr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0348usize) as _) }
        }
        #[doc = "Current COUNTER value"]
        #[inline(always)]
        pub const fn counter(self) -> crate::common::Reg<regs::Counter, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "12 bit prescaler for COUNTER frequency (32768/(PRESCALER+1)).Must be written when RTC is stopped"]
        #[inline(always)]
        pub const fn prescaler(self) -> crate::common::Reg<regs::Prescaler, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Description collection: Compare register n"]
        #[inline(always)]
        pub const fn cc(self, n: usize) -> crate::common::Reg<regs::Cc, crate::common::RW> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize + n * 4usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Description collection: Compare register n"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cc(pub u32);
        impl Cc {
            #[doc = "Compare value"]
            #[inline(always)]
            pub const fn compare(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x00ff_ffff;
                val as u32
            }
            #[doc = "Compare value"]
            #[inline(always)]
            pub fn set_compare(&mut self, val: u32) {
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
        #[doc = "Current COUNTER value"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Counter(pub u32);
        impl Counter {
            #[doc = "Counter value"]
            #[inline(always)]
            pub const fn counter(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x00ff_ffff;
                val as u32
            }
            #[doc = "Counter value"]
            #[inline(always)]
            pub fn set_counter(&mut self, val: u32) {
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
        #[doc = "Enable or disable event routing"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Evten(pub u32);
        impl Evten {
            #[doc = "Enable or disable event routing for event TICK"]
            #[inline(always)]
            pub const fn tick(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable event routing for event TICK"]
            #[inline(always)]
            pub fn set_tick(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable event routing for event OVRFLW"]
            #[inline(always)]
            pub const fn ovrflw(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable event routing for event OVRFLW"]
            #[inline(always)]
            pub fn set_ovrflw(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable event routing for event COMPARE\\[0\\]"]
            #[inline(always)]
            pub const fn compare0(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable event routing for event COMPARE\\[0\\]"]
            #[inline(always)]
            pub fn set_compare0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Enable or disable event routing for event COMPARE\\[1\\]"]
            #[inline(always)]
            pub const fn compare1(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable event routing for event COMPARE\\[1\\]"]
            #[inline(always)]
            pub fn set_compare1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Enable or disable event routing for event COMPARE\\[2\\]"]
            #[inline(always)]
            pub const fn compare2(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable event routing for event COMPARE\\[2\\]"]
            #[inline(always)]
            pub fn set_compare2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Enable or disable event routing for event COMPARE\\[3\\]"]
            #[inline(always)]
            pub const fn compare3(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable event routing for event COMPARE\\[3\\]"]
            #[inline(always)]
            pub fn set_compare3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
        }
        impl Default for Evten {
            #[inline(always)]
            fn default() -> Evten {
                Evten(0)
            }
        }
        #[doc = "Disable event routing"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Evtenclr(pub u32);
        impl Evtenclr {
            #[doc = "Write '1' to disable event routing for event TICK"]
            #[inline(always)]
            pub const fn tick(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable event routing for event TICK"]
            #[inline(always)]
            pub fn set_tick(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to disable event routing for event OVRFLW"]
            #[inline(always)]
            pub const fn ovrflw(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable event routing for event OVRFLW"]
            #[inline(always)]
            pub fn set_ovrflw(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to disable event routing for event COMPARE\\[0\\]"]
            #[inline(always)]
            pub const fn compare0(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable event routing for event COMPARE\\[0\\]"]
            #[inline(always)]
            pub fn set_compare0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Write '1' to disable event routing for event COMPARE\\[1\\]"]
            #[inline(always)]
            pub const fn compare1(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable event routing for event COMPARE\\[1\\]"]
            #[inline(always)]
            pub fn set_compare1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Write '1' to disable event routing for event COMPARE\\[2\\]"]
            #[inline(always)]
            pub const fn compare2(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable event routing for event COMPARE\\[2\\]"]
            #[inline(always)]
            pub fn set_compare2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Write '1' to disable event routing for event COMPARE\\[3\\]"]
            #[inline(always)]
            pub const fn compare3(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable event routing for event COMPARE\\[3\\]"]
            #[inline(always)]
            pub fn set_compare3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
        }
        impl Default for Evtenclr {
            #[inline(always)]
            fn default() -> Evtenclr {
                Evtenclr(0)
            }
        }
        #[doc = "Enable event routing"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Evtenset(pub u32);
        impl Evtenset {
            #[doc = "Write '1' to enable event routing for event TICK"]
            #[inline(always)]
            pub const fn tick(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to enable event routing for event TICK"]
            #[inline(always)]
            pub fn set_tick(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to enable event routing for event OVRFLW"]
            #[inline(always)]
            pub const fn ovrflw(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to enable event routing for event OVRFLW"]
            #[inline(always)]
            pub fn set_ovrflw(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to enable event routing for event COMPARE\\[0\\]"]
            #[inline(always)]
            pub const fn compare0(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to enable event routing for event COMPARE\\[0\\]"]
            #[inline(always)]
            pub fn set_compare0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Write '1' to enable event routing for event COMPARE\\[1\\]"]
            #[inline(always)]
            pub const fn compare1(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to enable event routing for event COMPARE\\[1\\]"]
            #[inline(always)]
            pub fn set_compare1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Write '1' to enable event routing for event COMPARE\\[2\\]"]
            #[inline(always)]
            pub const fn compare2(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to enable event routing for event COMPARE\\[2\\]"]
            #[inline(always)]
            pub fn set_compare2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Write '1' to enable event routing for event COMPARE\\[3\\]"]
            #[inline(always)]
            pub const fn compare3(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to enable event routing for event COMPARE\\[3\\]"]
            #[inline(always)]
            pub fn set_compare3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
        }
        impl Default for Evtenset {
            #[inline(always)]
            fn default() -> Evtenset {
                Evtenset(0)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Write '1' to disable interrupt for event TICK"]
            #[inline(always)]
            pub const fn tick(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event TICK"]
            #[inline(always)]
            pub fn set_tick(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to disable interrupt for event OVRFLW"]
            #[inline(always)]
            pub const fn ovrflw(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event OVRFLW"]
            #[inline(always)]
            pub fn set_ovrflw(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to disable interrupt for event COMPARE\\[0\\]"]
            #[inline(always)]
            pub const fn compare0(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event COMPARE\\[0\\]"]
            #[inline(always)]
            pub fn set_compare0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Write '1' to disable interrupt for event COMPARE\\[1\\]"]
            #[inline(always)]
            pub const fn compare1(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event COMPARE\\[1\\]"]
            #[inline(always)]
            pub fn set_compare1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Write '1' to disable interrupt for event COMPARE\\[2\\]"]
            #[inline(always)]
            pub const fn compare2(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event COMPARE\\[2\\]"]
            #[inline(always)]
            pub fn set_compare2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Write '1' to disable interrupt for event COMPARE\\[3\\]"]
            #[inline(always)]
            pub const fn compare3(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event COMPARE\\[3\\]"]
            #[inline(always)]
            pub fn set_compare3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
        #[doc = "12 bit prescaler for COUNTER frequency (32768/(PRESCALER+1)).Must be written when RTC is stopped"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Prescaler(pub u32);
        impl Prescaler {
            #[doc = "Prescaler value"]
            #[inline(always)]
            pub const fn prescaler(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "Prescaler value"]
            #[inline(always)]
            pub fn set_prescaler(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
        }
        impl Default for Prescaler {
            #[inline(always)]
            fn default() -> Prescaler {
                Prescaler(0)
            }
        }
    }
}
pub mod saadc {
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch {
        ptr: *mut u8,
    }
    unsafe impl Send for Ch {}
    unsafe impl Sync for Ch {}
    impl Ch {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster: Input positive pin selection for CH\\[n\\]"]
        #[inline(always)]
        pub const fn pselp(self) -> crate::common::Reg<regs::Pselp, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster: Input negative pin selection for CH\\[n\\]"]
        #[inline(always)]
        pub const fn pseln(self) -> crate::common::Reg<regs::Pseln, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Description cluster: Input configuration for CH\\[n\\]"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Description cluster: High/low limits for event monitoring a channel"]
        #[inline(always)]
        pub const fn limit(self) -> crate::common::Reg<regs::Limit, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
    }
    #[doc = "Peripheral events."]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EventsCh {
        ptr: *mut u8,
    }
    unsafe impl Send for EventsCh {}
    unsafe impl Sync for EventsCh {}
    impl EventsCh {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster: Last results is equal or above CH\\[n\\].LIMIT.HIGH"]
        #[inline(always)]
        pub const fn limith(self) -> crate::common::Reg<regs::Limith, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster: Last results is equal or below CH\\[n\\].LIMIT.LOW"]
        #[inline(always)]
        pub const fn limitl(self) -> crate::common::Reg<regs::Limitl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    #[doc = "RESULT EasyDMA channel"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Result {
        ptr: *mut u8,
    }
    unsafe impl Send for Result {}
    unsafe impl Sync for Result {}
    impl Result {
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Maximum number of buffer words to transfer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::Maxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Number of buffer words transferred since last START"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::Amount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
    }
    #[doc = "Analog to Digital Converter"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Saadc {
        ptr: *mut u8,
    }
    unsafe impl Send for Saadc {}
    unsafe impl Sync for Saadc {}
    impl Saadc {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start the ADC and prepare the result buffer in RAM"]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Take one ADC sample, if scan is enabled all channels are sampled"]
        #[inline(always)]
        pub const fn tasks_sample(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Stop the ADC and terminate any on-going conversion"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Starts offset auto-calibration"]
        #[inline(always)]
        pub const fn tasks_calibrateoffset(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "The ADC has started"]
        #[inline(always)]
        pub const fn events_started(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "The ADC has filled up the Result buffer"]
        #[inline(always)]
        pub const fn events_end(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "A conversion task has been completed. Depending on the mode, multiple conversions might be needed for a result to be transferred to RAM."]
        #[inline(always)]
        pub const fn events_done(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "A result is ready to get transferred to RAM."]
        #[inline(always)]
        pub const fn events_resultdone(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
        }
        #[doc = "Calibration is complete"]
        #[inline(always)]
        pub const fn events_calibratedone(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
        }
        #[doc = "The ADC has stopped"]
        #[inline(always)]
        pub const fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
        }
        #[doc = "Peripheral events."]
        #[inline(always)]
        pub const fn events_ch(self, n: usize) -> EventsCh {
            assert!(n < 8usize);
            unsafe { EventsCh::from_ptr(self.ptr.add(0x0118usize + n * 8usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Status"]
        #[inline(always)]
        pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "Enable or disable ADC"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn ch(self, n: usize) -> Ch {
            assert!(n < 8usize);
            unsafe { Ch::from_ptr(self.ptr.add(0x0510usize + n * 16usize) as _) }
        }
        #[doc = "Resolution configuration"]
        #[inline(always)]
        pub const fn resolution(self) -> crate::common::Reg<regs::Resolution, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05f0usize) as _) }
        }
        #[doc = "Oversampling configuration. OVERSAMPLE should not be combined with SCAN. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used."]
        #[inline(always)]
        pub const fn oversample(self) -> crate::common::Reg<regs::Oversample, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05f4usize) as _) }
        }
        #[doc = "Controls normal or continuous sample rate"]
        #[inline(always)]
        pub const fn samplerate(self) -> crate::common::Reg<regs::Samplerate, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05f8usize) as _) }
        }
        #[doc = "RESULT EasyDMA channel"]
        #[inline(always)]
        pub const fn result(self) -> Result {
            unsafe { Result::from_ptr(self.ptr.add(0x062cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Number of buffer words transferred since last START"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Amount(pub u32);
        impl Amount {
            #[doc = "Number of buffer words transferred since last START. This register can be read after an END or STOPPED event."]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x7fff;
                val as u16
            }
            #[doc = "Number of buffer words transferred since last START. This register can be read after an END or STOPPED event."]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
            }
        }
        impl Default for Amount {
            #[inline(always)]
            fn default() -> Amount {
                Amount(0)
            }
        }
        #[doc = "Description cluster: Input configuration for CH\\[n\\]"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Positive channel resistor control"]
            #[inline(always)]
            pub const fn resp(&self) -> super::vals::Resp {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Resp::from_bits(val as u8)
            }
            #[doc = "Positive channel resistor control"]
            #[inline(always)]
            pub fn set_resp(&mut self, val: super::vals::Resp) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
            #[doc = "Negative channel resistor control"]
            #[inline(always)]
            pub const fn resn(&self) -> super::vals::Resn {
                let val = (self.0 >> 4usize) & 0x03;
                super::vals::Resn::from_bits(val as u8)
            }
            #[doc = "Negative channel resistor control"]
            #[inline(always)]
            pub fn set_resn(&mut self, val: super::vals::Resn) {
                self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
            }
            #[doc = "Gain control"]
            #[inline(always)]
            pub const fn gain(&self) -> super::vals::Gain {
                let val = (self.0 >> 8usize) & 0x07;
                super::vals::Gain::from_bits(val as u8)
            }
            #[doc = "Gain control"]
            #[inline(always)]
            pub fn set_gain(&mut self, val: super::vals::Gain) {
                self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
            }
            #[doc = "Reference control"]
            #[inline(always)]
            pub const fn refsel(&self) -> super::vals::Refsel {
                let val = (self.0 >> 12usize) & 0x01;
                super::vals::Refsel::from_bits(val as u8)
            }
            #[doc = "Reference control"]
            #[inline(always)]
            pub fn set_refsel(&mut self, val: super::vals::Refsel) {
                self.0 =
                    (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
            }
            #[doc = "Acquisition time, the time the ADC uses to sample the input voltage"]
            #[inline(always)]
            pub const fn tacq(&self) -> super::vals::Tacq {
                let val = (self.0 >> 16usize) & 0x07;
                super::vals::Tacq::from_bits(val as u8)
            }
            #[doc = "Acquisition time, the time the ADC uses to sample the input voltage"]
            #[inline(always)]
            pub fn set_tacq(&mut self, val: super::vals::Tacq) {
                self.0 =
                    (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
            }
            #[doc = "Enable differential mode"]
            #[inline(always)]
            pub const fn mode(&self) -> super::vals::ConfigMode {
                let val = (self.0 >> 20usize) & 0x01;
                super::vals::ConfigMode::from_bits(val as u8)
            }
            #[doc = "Enable differential mode"]
            #[inline(always)]
            pub fn set_mode(&mut self, val: super::vals::ConfigMode) {
                self.0 =
                    (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
            }
            #[doc = "Enable burst mode"]
            #[inline(always)]
            pub const fn burst(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "Enable burst mode"]
            #[inline(always)]
            pub fn set_burst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        #[doc = "Enable or disable ADC"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable ADC"]
            #[inline(always)]
            pub const fn enable(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable ADC"]
            #[inline(always)]
            pub fn set_enable(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Enable or disable interrupt for event STARTED"]
            #[inline(always)]
            pub const fn started(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event STARTED"]
            #[inline(always)]
            pub fn set_started(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable interrupt for event END"]
            #[inline(always)]
            pub const fn end(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event END"]
            #[inline(always)]
            pub fn set_end(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for event DONE"]
            #[inline(always)]
            pub const fn done(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event DONE"]
            #[inline(always)]
            pub fn set_done(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable or disable interrupt for event RESULTDONE"]
            #[inline(always)]
            pub const fn resultdone(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RESULTDONE"]
            #[inline(always)]
            pub fn set_resultdone(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable or disable interrupt for event CALIBRATEDONE"]
            #[inline(always)]
            pub const fn calibratedone(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event CALIBRATEDONE"]
            #[inline(always)]
            pub fn set_calibratedone(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable or disable interrupt for event STOPPED"]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event STOPPED"]
            #[inline(always)]
            pub fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable or disable interrupt for event CH0LIMITH"]
            #[inline(always)]
            pub const fn ch0limith(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event CH0LIMITH"]
            #[inline(always)]
            pub fn set_ch0limith(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Enable or disable interrupt for event CH0LIMITL"]
            #[inline(always)]
            pub const fn ch0limitl(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event CH0LIMITL"]
            #[inline(always)]
            pub fn set_ch0limitl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Enable or disable interrupt for event CH1LIMITH"]
            #[inline(always)]
            pub const fn ch1limith(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event CH1LIMITH"]
            #[inline(always)]
            pub fn set_ch1limith(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Enable or disable interrupt for event CH1LIMITL"]
            #[inline(always)]
            pub const fn ch1limitl(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event CH1LIMITL"]
            #[inline(always)]
            pub fn set_ch1limitl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Enable or disable interrupt for event CH2LIMITH"]
            #[inline(always)]
            pub const fn ch2limith(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event CH2LIMITH"]
            #[inline(always)]
            pub fn set_ch2limith(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Enable or disable interrupt for event CH2LIMITL"]
            #[inline(always)]
            pub const fn ch2limitl(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event CH2LIMITL"]
            #[inline(always)]
            pub fn set_ch2limitl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "Enable or disable interrupt for event CH3LIMITH"]
            #[inline(always)]
            pub const fn ch3limith(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event CH3LIMITH"]
            #[inline(always)]
            pub fn set_ch3limith(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "Enable or disable interrupt for event CH3LIMITL"]
            #[inline(always)]
            pub const fn ch3limitl(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event CH3LIMITL"]
            #[inline(always)]
            pub fn set_ch3limitl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Enable or disable interrupt for event CH4LIMITH"]
            #[inline(always)]
            pub const fn ch4limith(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event CH4LIMITH"]
            #[inline(always)]
            pub fn set_ch4limith(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Enable or disable interrupt for event CH4LIMITL"]
            #[inline(always)]
            pub const fn ch4limitl(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event CH4LIMITL"]
            #[inline(always)]
            pub fn set_ch4limitl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
            #[doc = "Enable or disable interrupt for event CH5LIMITH"]
            #[inline(always)]
            pub const fn ch5limith(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event CH5LIMITH"]
            #[inline(always)]
            pub fn set_ch5limith(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Enable or disable interrupt for event CH5LIMITL"]
            #[inline(always)]
            pub const fn ch5limitl(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event CH5LIMITL"]
            #[inline(always)]
            pub fn set_ch5limitl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Enable or disable interrupt for event CH6LIMITH"]
            #[inline(always)]
            pub const fn ch6limith(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event CH6LIMITH"]
            #[inline(always)]
            pub fn set_ch6limith(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Enable or disable interrupt for event CH6LIMITL"]
            #[inline(always)]
            pub const fn ch6limitl(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event CH6LIMITL"]
            #[inline(always)]
            pub fn set_ch6limitl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "Enable or disable interrupt for event CH7LIMITH"]
            #[inline(always)]
            pub const fn ch7limith(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event CH7LIMITH"]
            #[inline(always)]
            pub fn set_ch7limith(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "Enable or disable interrupt for event CH7LIMITL"]
            #[inline(always)]
            pub const fn ch7limitl(&self) -> bool {
                let val = (self.0 >> 21usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event CH7LIMITL"]
            #[inline(always)]
            pub fn set_ch7limitl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
        #[doc = "Description cluster: High/low limits for event monitoring a channel"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Limit(pub u32);
        impl Limit {
            #[doc = "Low level limit"]
            #[inline(always)]
            pub const fn low(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Low level limit"]
            #[inline(always)]
            pub fn set_low(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
            #[doc = "High level limit"]
            #[inline(always)]
            pub const fn high(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0xffff;
                val as u16
            }
            #[doc = "High level limit"]
            #[inline(always)]
            pub fn set_high(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
            }
        }
        impl Default for Limit {
            #[inline(always)]
            fn default() -> Limit {
                Limit(0)
            }
        }
        #[doc = "Description cluster: Last results is equal or above CH\\[n\\].LIMIT.HIGH"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Limith(pub u32);
        impl Limith {
            #[doc = "Last results is equal or above CH\\[n\\].LIMIT.HIGH"]
            #[inline(always)]
            pub const fn limith(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Last results is equal or above CH\\[n\\].LIMIT.HIGH"]
            #[inline(always)]
            pub fn set_limith(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Limith {
            #[inline(always)]
            fn default() -> Limith {
                Limith(0)
            }
        }
        #[doc = "Description cluster: Last results is equal or below CH\\[n\\].LIMIT.LOW"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Limitl(pub u32);
        impl Limitl {
            #[doc = "Last results is equal or below CH\\[n\\].LIMIT.LOW"]
            #[inline(always)]
            pub const fn limitl(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Last results is equal or below CH\\[n\\].LIMIT.LOW"]
            #[inline(always)]
            pub fn set_limitl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Limitl {
            #[inline(always)]
            fn default() -> Limitl {
                Limitl(0)
            }
        }
        #[doc = "Maximum number of buffer words to transfer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Maxcnt(pub u32);
        impl Maxcnt {
            #[doc = "Maximum number of buffer words to transfer"]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x7fff;
                val as u16
            }
            #[doc = "Maximum number of buffer words to transfer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
            }
        }
        impl Default for Maxcnt {
            #[inline(always)]
            fn default() -> Maxcnt {
                Maxcnt(0)
            }
        }
        #[doc = "Oversampling configuration. OVERSAMPLE should not be combined with SCAN. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Oversample(pub u32);
        impl Oversample {
            #[doc = "Oversample control"]
            #[inline(always)]
            pub const fn oversample(&self) -> super::vals::Oversample {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Oversample::from_bits(val as u8)
            }
            #[doc = "Oversample control"]
            #[inline(always)]
            pub fn set_oversample(&mut self, val: super::vals::Oversample) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Oversample {
            #[inline(always)]
            fn default() -> Oversample {
                Oversample(0)
            }
        }
        #[doc = "Description cluster: Input negative pin selection for CH\\[n\\]"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pseln(pub u32);
        impl Pseln {
            #[doc = "Analog negative input, enables differential channel"]
            #[inline(always)]
            pub const fn pseln(&self) -> super::vals::Psel {
                let val = (self.0 >> 0usize) & 0x1f;
                super::vals::Psel::from_bits(val as u8)
            }
            #[doc = "Analog negative input, enables differential channel"]
            #[inline(always)]
            pub fn set_pseln(&mut self, val: super::vals::Psel) {
                self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
            }
        }
        impl Default for Pseln {
            #[inline(always)]
            fn default() -> Pseln {
                Pseln(0)
            }
        }
        #[doc = "Description cluster: Input positive pin selection for CH\\[n\\]"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pselp(pub u32);
        impl Pselp {
            #[doc = "Analog positive input channel"]
            #[inline(always)]
            pub const fn pselp(&self) -> super::vals::Psel {
                let val = (self.0 >> 0usize) & 0x1f;
                super::vals::Psel::from_bits(val as u8)
            }
            #[doc = "Analog positive input channel"]
            #[inline(always)]
            pub fn set_pselp(&mut self, val: super::vals::Psel) {
                self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
            }
        }
        impl Default for Pselp {
            #[inline(always)]
            fn default() -> Pselp {
                Pselp(0)
            }
        }
        #[doc = "Resolution configuration"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Resolution(pub u32);
        impl Resolution {
            #[doc = "Set the resolution"]
            #[inline(always)]
            pub const fn val(&self) -> super::vals::Val {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::Val::from_bits(val as u8)
            }
            #[doc = "Set the resolution"]
            #[inline(always)]
            pub fn set_val(&mut self, val: super::vals::Val) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
            }
        }
        impl Default for Resolution {
            #[inline(always)]
            fn default() -> Resolution {
                Resolution(0)
            }
        }
        #[doc = "Controls normal or continuous sample rate"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Samplerate(pub u32);
        impl Samplerate {
            #[doc = "Capture and compare value. Sample rate is 16 MHz/CC"]
            #[inline(always)]
            pub const fn cc(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x07ff;
                val as u16
            }
            #[doc = "Capture and compare value. Sample rate is 16 MHz/CC"]
            #[inline(always)]
            pub fn set_cc(&mut self, val: u16) {
                self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
            }
            #[doc = "Select mode for sample rate control"]
            #[inline(always)]
            pub const fn mode(&self) -> super::vals::SamplerateMode {
                let val = (self.0 >> 12usize) & 0x01;
                super::vals::SamplerateMode::from_bits(val as u8)
            }
            #[doc = "Select mode for sample rate control"]
            #[inline(always)]
            pub fn set_mode(&mut self, val: super::vals::SamplerateMode) {
                self.0 =
                    (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
            }
        }
        impl Default for Samplerate {
            #[inline(always)]
            fn default() -> Samplerate {
                Samplerate(0)
            }
        }
        #[doc = "Status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Status(pub u32);
        impl Status {
            #[doc = "Status"]
            #[inline(always)]
            pub const fn status(&self) -> super::vals::Status {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Status::from_bits(val as u8)
            }
            #[doc = "Status"]
            #[inline(always)]
            pub fn set_status(&mut self, val: super::vals::Status) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Status {
            #[inline(always)]
            fn default() -> Status {
                Status(0)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum ConfigMode {
            #[doc = "Single ended, PSELN will be ignored, negative input to ADC shorted to GND"]
            SE = 0x0,
            #[doc = "Differential"]
            DIFF = 0x01,
        }
        impl ConfigMode {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> ConfigMode {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for ConfigMode {
            #[inline(always)]
            fn from(val: u8) -> ConfigMode {
                ConfigMode::from_bits(val)
            }
        }
        impl From<ConfigMode> for u8 {
            #[inline(always)]
            fn from(val: ConfigMode) -> u8 {
                ConfigMode::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Gain {
            #[doc = "1/6"]
            GAIN1_6 = 0x0,
            #[doc = "1/5"]
            GAIN1_5 = 0x01,
            #[doc = "1/4"]
            GAIN1_4 = 0x02,
            #[doc = "1/3"]
            GAIN1_3 = 0x03,
            #[doc = "1/2"]
            GAIN1_2 = 0x04,
            #[doc = "1"]
            GAIN1 = 0x05,
            #[doc = "2"]
            GAIN2 = 0x06,
            #[doc = "4"]
            GAIN4 = 0x07,
        }
        impl Gain {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Gain {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Gain {
            #[inline(always)]
            fn from(val: u8) -> Gain {
                Gain::from_bits(val)
            }
        }
        impl From<Gain> for u8 {
            #[inline(always)]
            fn from(val: Gain) -> u8 {
                Gain::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Oversample {
            #[doc = "Bypass oversampling"]
            BYPASS = 0x0,
            #[doc = "Oversample 2x"]
            OVER2X = 0x01,
            #[doc = "Oversample 4x"]
            OVER4X = 0x02,
            #[doc = "Oversample 8x"]
            OVER8X = 0x03,
            #[doc = "Oversample 16x"]
            OVER16X = 0x04,
            #[doc = "Oversample 32x"]
            OVER32X = 0x05,
            #[doc = "Oversample 64x"]
            OVER64X = 0x06,
            #[doc = "Oversample 128x"]
            OVER128X = 0x07,
            #[doc = "Oversample 256x"]
            OVER256X = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Oversample {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Oversample {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Oversample {
            #[inline(always)]
            fn from(val: u8) -> Oversample {
                Oversample::from_bits(val)
            }
        }
        impl From<Oversample> for u8 {
            #[inline(always)]
            fn from(val: Oversample) -> u8 {
                Oversample::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Psel {
            #[doc = "Not connected"]
            NC = 0x0,
            #[doc = "AIN0"]
            ANALOG_INPUT0 = 0x01,
            #[doc = "AIN1"]
            ANALOG_INPUT1 = 0x02,
            #[doc = "AIN2"]
            ANALOG_INPUT2 = 0x03,
            #[doc = "AIN3"]
            ANALOG_INPUT3 = 0x04,
            #[doc = "AIN4"]
            ANALOG_INPUT4 = 0x05,
            #[doc = "AIN5"]
            ANALOG_INPUT5 = 0x06,
            #[doc = "AIN6"]
            ANALOG_INPUT6 = 0x07,
            #[doc = "AIN7"]
            ANALOG_INPUT7 = 0x08,
            #[doc = "VDD"]
            VDD = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
            _RESERVED_10 = 0x10,
            _RESERVED_11 = 0x11,
            _RESERVED_12 = 0x12,
            _RESERVED_13 = 0x13,
            _RESERVED_14 = 0x14,
            _RESERVED_15 = 0x15,
            _RESERVED_16 = 0x16,
            _RESERVED_17 = 0x17,
            _RESERVED_18 = 0x18,
            _RESERVED_19 = 0x19,
            _RESERVED_1a = 0x1a,
            _RESERVED_1b = 0x1b,
            _RESERVED_1c = 0x1c,
            _RESERVED_1d = 0x1d,
            _RESERVED_1e = 0x1e,
            _RESERVED_1f = 0x1f,
        }
        impl Psel {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Psel {
                unsafe { core::mem::transmute(val & 0x1f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Psel {
            #[inline(always)]
            fn from(val: u8) -> Psel {
                Psel::from_bits(val)
            }
        }
        impl From<Psel> for u8 {
            #[inline(always)]
            fn from(val: Psel) -> u8 {
                Psel::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Refsel {
            #[doc = "Internal reference (0.6 V)"]
            INTERNAL = 0x0,
            #[doc = "VDD/4 as reference"]
            VDD1_4 = 0x01,
        }
        impl Refsel {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Refsel {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Refsel {
            #[inline(always)]
            fn from(val: u8) -> Refsel {
                Refsel::from_bits(val)
            }
        }
        impl From<Refsel> for u8 {
            #[inline(always)]
            fn from(val: Refsel) -> u8 {
                Refsel::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Resn {
            #[doc = "Bypass resistor ladder"]
            BYPASS = 0x0,
            #[doc = "Pull-down to GND"]
            PULLDOWN = 0x01,
            #[doc = "Pull-up to VDD"]
            PULLUP = 0x02,
            #[doc = "Set input at VDD/2"]
            VDD1_2 = 0x03,
        }
        impl Resn {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Resn {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Resn {
            #[inline(always)]
            fn from(val: u8) -> Resn {
                Resn::from_bits(val)
            }
        }
        impl From<Resn> for u8 {
            #[inline(always)]
            fn from(val: Resn) -> u8 {
                Resn::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Resp {
            #[doc = "Bypass resistor ladder"]
            BYPASS = 0x0,
            #[doc = "Pull-down to GND"]
            PULLDOWN = 0x01,
            #[doc = "Pull-up to VDD"]
            PULLUP = 0x02,
            #[doc = "Set input at VDD/2"]
            VDD1_2 = 0x03,
        }
        impl Resp {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Resp {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Resp {
            #[inline(always)]
            fn from(val: u8) -> Resp {
                Resp::from_bits(val)
            }
        }
        impl From<Resp> for u8 {
            #[inline(always)]
            fn from(val: Resp) -> u8 {
                Resp::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum SamplerateMode {
            #[doc = "Rate is controlled from SAMPLE task"]
            TASK = 0x0,
            #[doc = "Rate is controlled from local timer (use CC to control the rate)"]
            TIMERS = 0x01,
        }
        impl SamplerateMode {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> SamplerateMode {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for SamplerateMode {
            #[inline(always)]
            fn from(val: u8) -> SamplerateMode {
                SamplerateMode::from_bits(val)
            }
        }
        impl From<SamplerateMode> for u8 {
            #[inline(always)]
            fn from(val: SamplerateMode) -> u8 {
                SamplerateMode::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Status {
            #[doc = "ADC is ready. No on-going conversion."]
            READY = 0x0,
            #[doc = "ADC is busy. Conversion in progress."]
            BUSY = 0x01,
        }
        impl Status {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Status {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Status {
            #[inline(always)]
            fn from(val: u8) -> Status {
                Status::from_bits(val)
            }
        }
        impl From<Status> for u8 {
            #[inline(always)]
            fn from(val: Status) -> u8 {
                Status::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Tacq {
            #[doc = "3 us"]
            _3US = 0x0,
            #[doc = "5 us"]
            _5US = 0x01,
            #[doc = "10 us"]
            _10US = 0x02,
            #[doc = "15 us"]
            _15US = 0x03,
            #[doc = "20 us"]
            _20US = 0x04,
            #[doc = "40 us"]
            _40US = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl Tacq {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Tacq {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Tacq {
            #[inline(always)]
            fn from(val: u8) -> Tacq {
                Tacq::from_bits(val)
            }
        }
        impl From<Tacq> for u8 {
            #[inline(always)]
            fn from(val: Tacq) -> u8 {
                Tacq::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Val {
            #[doc = "8 bit"]
            _8BIT = 0x0,
            #[doc = "10 bit"]
            _10BIT = 0x01,
            #[doc = "12 bit"]
            _12BIT = 0x02,
            #[doc = "14 bit"]
            _14BIT = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl Val {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Val {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Val {
            #[inline(always)]
            fn from(val: u8) -> Val {
                Val::from_bits(val)
            }
        }
        impl From<Val> for u8 {
            #[inline(always)]
            fn from(val: Val) -> u8 {
                Val::to_bits(val)
            }
        }
    }
}
pub mod shared {
    pub mod regs {
        #[doc = "Pin number configuration for PDM CLK signal"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Psel(pub u32);
        impl Psel {
            #[doc = "Pin number"]
            #[inline(always)]
            pub const fn pin(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x1f;
                val as u8
            }
            #[doc = "Pin number"]
            #[inline(always)]
            pub fn set_pin(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
            }
            #[doc = "Connection"]
            #[inline(always)]
            pub const fn connect(&self) -> super::vals::Connect {
                let val = (self.0 >> 31usize) & 0x01;
                super::vals::Connect::from_bits(val as u8)
            }
            #[doc = "Connection"]
            #[inline(always)]
            pub fn set_connect(&mut self, val: super::vals::Connect) {
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
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
pub mod spi {
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Pin select for MOSI signal"]
        #[inline(always)]
        pub const fn mosi(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Pin select for MISO signal"]
        #[inline(always)]
        pub const fn miso(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
    }
    #[doc = "Serial Peripheral Interface"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Spi {
        ptr: *mut u8,
    }
    unsafe impl Send for Spi {}
    unsafe impl Sync for Spi {}
    impl Spi {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "TXD byte sent and RXD byte received"]
        #[inline(always)]
        pub const fn events_ready(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Enable SPI"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn psel(self) -> Psel {
            unsafe { Psel::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "RXD register"]
        #[inline(always)]
        pub const fn rxd(self) -> crate::common::Reg<regs::Rxd, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
        #[doc = "TXD register"]
        #[inline(always)]
        pub const fn txd(self) -> crate::common::Reg<regs::Txd, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
        }
        #[doc = "SPI frequency. Accuracy depends on the HFCLK source selected."]
        #[inline(always)]
        pub const fn frequency(self) -> crate::common::Reg<regs::Frequency, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
        }
        #[doc = "Configuration register"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0554usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Bit order"]
            #[inline(always)]
            pub const fn order(&self) -> super::vals::Order {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Order::from_bits(val as u8)
            }
            #[doc = "Bit order"]
            #[inline(always)]
            pub fn set_order(&mut self, val: super::vals::Order) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Serial clock (SCK) phase"]
            #[inline(always)]
            pub const fn cpha(&self) -> super::vals::Cpha {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Cpha::from_bits(val as u8)
            }
            #[doc = "Serial clock (SCK) phase"]
            #[inline(always)]
            pub fn set_cpha(&mut self, val: super::vals::Cpha) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Serial clock (SCK) polarity"]
            #[inline(always)]
            pub const fn cpol(&self) -> super::vals::Cpol {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Cpol::from_bits(val as u8)
            }
            #[doc = "Serial clock (SCK) polarity"]
            #[inline(always)]
            pub fn set_cpol(&mut self, val: super::vals::Cpol) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        #[doc = "Enable SPI"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable SPI"]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable SPI"]
            #[inline(always)]
            pub fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        #[doc = "SPI frequency. Accuracy depends on the HFCLK source selected."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Frequency(pub u32);
        impl Frequency {
            #[doc = "SPI master data rate"]
            #[inline(always)]
            pub const fn frequency(&self) -> super::vals::Frequency {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Frequency::from_bits(val as u32)
            }
            #[doc = "SPI master data rate"]
            #[inline(always)]
            pub fn set_frequency(&mut self, val: super::vals::Frequency) {
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
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Write '1' to disable interrupt for event READY"]
            #[inline(always)]
            pub const fn ready(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event READY"]
            #[inline(always)]
            pub fn set_ready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
        #[doc = "RXD register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxd(pub u32);
        impl Rxd {
            #[doc = "RX data received. Double buffered"]
            #[inline(always)]
            pub const fn rxd(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "RX data received. Double buffered"]
            #[inline(always)]
            pub fn set_rxd(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Rxd {
            #[inline(always)]
            fn default() -> Rxd {
                Rxd(0)
            }
        }
        #[doc = "TXD register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Txd(pub u32);
        impl Txd {
            #[doc = "TX data to send. Double buffered"]
            #[inline(always)]
            pub const fn txd(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "TX data to send. Double buffered"]
            #[inline(always)]
            pub fn set_txd(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Txd {
            #[inline(always)]
            fn default() -> Txd {
                Txd(0)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Enable {
            #[doc = "Disable SPI"]
            DISABLED = 0x0,
            #[doc = "Enable SPI"]
            ENABLED = 0x01,
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
        pub struct Frequency(pub u32);
        impl Frequency {
            #[doc = "125 kbps"]
            pub const K125: Self = Self(0x0200_0000);
            #[doc = "250 kbps"]
            pub const K250: Self = Self(0x0400_0000);
            #[doc = "500 kbps"]
            pub const K500: Self = Self(0x0800_0000);
            #[doc = "1 Mbps"]
            pub const M1: Self = Self(0x1000_0000);
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
    }
}
pub mod spim {
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Pin select for MOSI signal"]
        #[inline(always)]
        pub const fn mosi(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Pin select for MISO signal"]
        #[inline(always)]
        pub const fn miso(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in receive buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::RxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::RxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "EasyDMA list type"]
        #[inline(always)]
        pub const fn list(self) -> crate::common::Reg<regs::RxdList, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Stop SPI transaction"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Suspend SPI transaction"]
        #[inline(always)]
        pub const fn tasks_suspend(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "Resume SPI transaction"]
        #[inline(always)]
        pub const fn tasks_resume(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "SPI transaction has stopped"]
        #[inline(always)]
        pub const fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "End of RXD buffer reached"]
        #[inline(always)]
        pub const fn events_endrx(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
        }
        #[doc = "End of RXD buffer and TXD buffer reached"]
        #[inline(always)]
        pub const fn events_end(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
        }
        #[doc = "End of TXD buffer reached"]
        #[inline(always)]
        pub const fn events_endtx(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
        }
        #[doc = "Transaction started"]
        #[inline(always)]
        pub const fn events_started(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Enable SPIM"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn psel(self) -> Psel {
            unsafe { Psel::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "SPI frequency. Accuracy depends on the HFCLK source selected."]
        #[inline(always)]
        pub const fn frequency(self) -> crate::common::Reg<regs::Frequency, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
        }
        #[doc = "RXD EasyDMA channel"]
        #[inline(always)]
        pub const fn rxd(self) -> Rxd {
            unsafe { Rxd::from_ptr(self.ptr.add(0x0534usize) as _) }
        }
        #[doc = "TXD EasyDMA channel"]
        #[inline(always)]
        pub const fn txd(self) -> Txd {
            unsafe { Txd::from_ptr(self.ptr.add(0x0544usize) as _) }
        }
        #[doc = "Configuration register"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0554usize) as _) }
        }
        #[doc = "Over-read character. Character clocked out in case and over-read of the TXD buffer."]
        #[inline(always)]
        pub const fn orc(self) -> crate::common::Reg<regs::Orc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05c0usize) as _) }
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in transmit buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::TxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::TxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "EasyDMA list type"]
        #[inline(always)]
        pub const fn list(self) -> crate::common::Reg<regs::TxdList, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Bit order"]
            #[inline(always)]
            pub const fn order(&self) -> super::vals::Order {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Order::from_bits(val as u8)
            }
            #[doc = "Bit order"]
            #[inline(always)]
            pub fn set_order(&mut self, val: super::vals::Order) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Serial clock (SCK) phase"]
            #[inline(always)]
            pub const fn cpha(&self) -> super::vals::Cpha {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Cpha::from_bits(val as u8)
            }
            #[doc = "Serial clock (SCK) phase"]
            #[inline(always)]
            pub fn set_cpha(&mut self, val: super::vals::Cpha) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Serial clock (SCK) polarity"]
            #[inline(always)]
            pub const fn cpol(&self) -> super::vals::Cpol {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Cpol::from_bits(val as u8)
            }
            #[doc = "Serial clock (SCK) polarity"]
            #[inline(always)]
            pub fn set_cpol(&mut self, val: super::vals::Cpol) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        #[doc = "Enable SPIM"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable SPIM"]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable SPIM"]
            #[inline(always)]
            pub fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        #[doc = "SPI frequency. Accuracy depends on the HFCLK source selected."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Frequency(pub u32);
        impl Frequency {
            #[doc = "SPI master data rate"]
            #[inline(always)]
            pub const fn frequency(&self) -> super::vals::Frequency {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Frequency::from_bits(val as u32)
            }
            #[doc = "SPI master data rate"]
            #[inline(always)]
            pub fn set_frequency(&mut self, val: super::vals::Frequency) {
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
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Write '1' to disable interrupt for event STOPPED"]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event STOPPED"]
            #[inline(always)]
            pub fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to disable interrupt for event ENDRX"]
            #[inline(always)]
            pub const fn endrx(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ENDRX"]
            #[inline(always)]
            pub fn set_endrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Write '1' to disable interrupt for event END"]
            #[inline(always)]
            pub const fn end(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event END"]
            #[inline(always)]
            pub fn set_end(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Write '1' to disable interrupt for event ENDTX"]
            #[inline(always)]
            pub const fn endtx(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ENDTX"]
            #[inline(always)]
            pub fn set_endtx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Write '1' to disable interrupt for event STARTED"]
            #[inline(always)]
            pub const fn started(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event STARTED"]
            #[inline(always)]
            pub fn set_started(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
        #[doc = "Over-read character. Character clocked out in case and over-read of the TXD buffer."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Orc(pub u32);
        impl Orc {
            #[doc = "Over-read character. Character clocked out in case and over-read of the TXD buffer."]
            #[inline(always)]
            pub const fn orc(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Over-read character. Character clocked out in case and over-read of the TXD buffer."]
            #[inline(always)]
            pub fn set_orc(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Orc {
            #[inline(always)]
            fn default() -> Orc {
                Orc(0)
            }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdAmount(pub u32);
        impl RxdAmount {
            #[doc = "Number of bytes transferred in the last transaction"]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
        }
        impl Default for RxdAmount {
            #[inline(always)]
            fn default() -> RxdAmount {
                RxdAmount(0)
            }
        }
        #[doc = "EasyDMA list type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdList(pub u32);
        impl RxdList {
            #[doc = "List type"]
            #[inline(always)]
            pub const fn list(&self) -> super::vals::RxdListList {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::RxdListList::from_bits(val as u8)
            }
            #[doc = "List type"]
            #[inline(always)]
            pub fn set_list(&mut self, val: super::vals::RxdListList) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for RxdList {
            #[inline(always)]
            fn default() -> RxdList {
                RxdList(0)
            }
        }
        #[doc = "Maximum number of bytes in receive buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdMaxcnt(pub u32);
        impl RxdMaxcnt {
            #[doc = "Maximum number of bytes in receive buffer"]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Maximum number of bytes in receive buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
        }
        impl Default for RxdMaxcnt {
            #[inline(always)]
            fn default() -> RxdMaxcnt {
                RxdMaxcnt(0)
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event END and task START"]
            #[inline(always)]
            pub const fn end_start(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event END and task START"]
            #[inline(always)]
            pub fn set_end_start(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdAmount(pub u32);
        impl TxdAmount {
            #[doc = "Number of bytes transferred in the last transaction"]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
        }
        impl Default for TxdAmount {
            #[inline(always)]
            fn default() -> TxdAmount {
                TxdAmount(0)
            }
        }
        #[doc = "EasyDMA list type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdList(pub u32);
        impl TxdList {
            #[doc = "List type"]
            #[inline(always)]
            pub const fn list(&self) -> super::vals::TxdListList {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::TxdListList::from_bits(val as u8)
            }
            #[doc = "List type"]
            #[inline(always)]
            pub fn set_list(&mut self, val: super::vals::TxdListList) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for TxdList {
            #[inline(always)]
            fn default() -> TxdList {
                TxdList(0)
            }
        }
        #[doc = "Maximum number of bytes in transmit buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdMaxcnt(pub u32);
        impl TxdMaxcnt {
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
        }
        impl Default for TxdMaxcnt {
            #[inline(always)]
            fn default() -> TxdMaxcnt {
                TxdMaxcnt(0)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        pub struct Frequency(pub u32);
        impl Frequency {
            #[doc = "125 kbps"]
            pub const K125: Self = Self(0x0200_0000);
            #[doc = "250 kbps"]
            pub const K250: Self = Self(0x0400_0000);
            #[doc = "500 kbps"]
            pub const K500: Self = Self(0x0800_0000);
            #[doc = "1 Mbps"]
            pub const M1: Self = Self(0x1000_0000);
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Pin select for MISO signal"]
        #[inline(always)]
        pub const fn miso(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Pin select for MOSI signal"]
        #[inline(always)]
        pub const fn mosi(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Pin select for CSN signal"]
        #[inline(always)]
        pub const fn csn(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in receive buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::RxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Number of bytes received in last granted transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::RxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "EasyDMA list type"]
        #[inline(always)]
        pub const fn list(self) -> crate::common::Reg<regs::RxdList, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
        }
        #[doc = "Release SPI semaphore, enabling the SPI slave to acquire it"]
        #[inline(always)]
        pub const fn tasks_release(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
        }
        #[doc = "Granted transaction completed"]
        #[inline(always)]
        pub const fn events_end(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "End of RXD buffer reached"]
        #[inline(always)]
        pub const fn events_endrx(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
        }
        #[doc = "Semaphore acquired"]
        #[inline(always)]
        pub const fn events_acquired(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Semaphore status register"]
        #[inline(always)]
        pub const fn semstat(self) -> crate::common::Reg<regs::Semstat, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "Status from last transaction"]
        #[inline(always)]
        pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0440usize) as _) }
        }
        #[doc = "Enable SPI slave"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn psel(self) -> Psel {
            unsafe { Psel::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn rxd(self) -> Rxd {
            unsafe { Rxd::from_ptr(self.ptr.add(0x0534usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn txd(self) -> Txd {
            unsafe { Txd::from_ptr(self.ptr.add(0x0544usize) as _) }
        }
        #[doc = "Configuration register"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0554usize) as _) }
        }
        #[doc = "Default character. Character clocked out in case of an ignored transaction."]
        #[inline(always)]
        pub const fn def(self) -> crate::common::Reg<regs::Def, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x055cusize) as _) }
        }
        #[doc = "Over-read character"]
        #[inline(always)]
        pub const fn orc(self) -> crate::common::Reg<regs::Orc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05c0usize) as _) }
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in transmit buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::TxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Number of bytes transmitted in last granted transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::TxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "EasyDMA list type"]
        #[inline(always)]
        pub const fn list(self) -> crate::common::Reg<regs::TxdList, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Bit order"]
            #[inline(always)]
            pub const fn order(&self) -> super::vals::Order {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Order::from_bits(val as u8)
            }
            #[doc = "Bit order"]
            #[inline(always)]
            pub fn set_order(&mut self, val: super::vals::Order) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Serial clock (SCK) phase"]
            #[inline(always)]
            pub const fn cpha(&self) -> super::vals::Cpha {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Cpha::from_bits(val as u8)
            }
            #[doc = "Serial clock (SCK) phase"]
            #[inline(always)]
            pub fn set_cpha(&mut self, val: super::vals::Cpha) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Serial clock (SCK) polarity"]
            #[inline(always)]
            pub const fn cpol(&self) -> super::vals::Cpol {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Cpol::from_bits(val as u8)
            }
            #[doc = "Serial clock (SCK) polarity"]
            #[inline(always)]
            pub fn set_cpol(&mut self, val: super::vals::Cpol) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        #[doc = "Default character. Character clocked out in case of an ignored transaction."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Def(pub u32);
        impl Def {
            #[doc = "Default character. Character clocked out in case of an ignored transaction."]
            #[inline(always)]
            pub const fn def(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Default character. Character clocked out in case of an ignored transaction."]
            #[inline(always)]
            pub fn set_def(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Def {
            #[inline(always)]
            fn default() -> Def {
                Def(0)
            }
        }
        #[doc = "Enable SPI slave"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable SPI slave"]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable SPI slave"]
            #[inline(always)]
            pub fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Write '1' to disable interrupt for event END"]
            #[inline(always)]
            pub const fn end(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event END"]
            #[inline(always)]
            pub fn set_end(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to disable interrupt for event ENDRX"]
            #[inline(always)]
            pub const fn endrx(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ENDRX"]
            #[inline(always)]
            pub fn set_endrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Write '1' to disable interrupt for event ACQUIRED"]
            #[inline(always)]
            pub const fn acquired(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ACQUIRED"]
            #[inline(always)]
            pub fn set_acquired(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
        #[doc = "Over-read character"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Orc(pub u32);
        impl Orc {
            #[doc = "Over-read character. Character clocked out after an over-read of the transmit buffer."]
            #[inline(always)]
            pub const fn orc(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Over-read character. Character clocked out after an over-read of the transmit buffer."]
            #[inline(always)]
            pub fn set_orc(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Orc {
            #[inline(always)]
            fn default() -> Orc {
                Orc(0)
            }
        }
        #[doc = "Number of bytes received in last granted transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdAmount(pub u32);
        impl RxdAmount {
            #[doc = "Number of bytes received in the last granted transaction"]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Number of bytes received in the last granted transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
        }
        impl Default for RxdAmount {
            #[inline(always)]
            fn default() -> RxdAmount {
                RxdAmount(0)
            }
        }
        #[doc = "EasyDMA list type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdList(pub u32);
        impl RxdList {
            #[doc = "List type"]
            #[inline(always)]
            pub const fn list(&self) -> super::vals::RxdListList {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::RxdListList::from_bits(val as u8)
            }
            #[doc = "List type"]
            #[inline(always)]
            pub fn set_list(&mut self, val: super::vals::RxdListList) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for RxdList {
            #[inline(always)]
            fn default() -> RxdList {
                RxdList(0)
            }
        }
        #[doc = "Maximum number of bytes in receive buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdMaxcnt(pub u32);
        impl RxdMaxcnt {
            #[doc = "Maximum number of bytes in receive buffer"]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Maximum number of bytes in receive buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
        }
        impl Default for RxdMaxcnt {
            #[inline(always)]
            fn default() -> RxdMaxcnt {
                RxdMaxcnt(0)
            }
        }
        #[doc = "Semaphore status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Semstat(pub u32);
        impl Semstat {
            #[doc = "Semaphore status"]
            #[inline(always)]
            pub const fn semstat(&self) -> super::vals::Semstat {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Semstat::from_bits(val as u8)
            }
            #[doc = "Semaphore status"]
            #[inline(always)]
            pub fn set_semstat(&mut self, val: super::vals::Semstat) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Semstat {
            #[inline(always)]
            fn default() -> Semstat {
                Semstat(0)
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event END and task ACQUIRE"]
            #[inline(always)]
            pub const fn end_acquire(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event END and task ACQUIRE"]
            #[inline(always)]
            pub fn set_end_acquire(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        #[doc = "Status from last transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Status(pub u32);
        impl Status {
            #[doc = "TX buffer over-read detected, and prevented"]
            #[inline(always)]
            pub const fn overread(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "TX buffer over-read detected, and prevented"]
            #[inline(always)]
            pub fn set_overread(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "RX buffer overflow detected, and prevented"]
            #[inline(always)]
            pub const fn overflow(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "RX buffer overflow detected, and prevented"]
            #[inline(always)]
            pub fn set_overflow(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
        }
        impl Default for Status {
            #[inline(always)]
            fn default() -> Status {
                Status(0)
            }
        }
        #[doc = "Number of bytes transmitted in last granted transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdAmount(pub u32);
        impl TxdAmount {
            #[doc = "Number of bytes transmitted in last granted transaction"]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Number of bytes transmitted in last granted transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
        }
        impl Default for TxdAmount {
            #[inline(always)]
            fn default() -> TxdAmount {
                TxdAmount(0)
            }
        }
        #[doc = "EasyDMA list type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdList(pub u32);
        impl TxdList {
            #[doc = "List type"]
            #[inline(always)]
            pub const fn list(&self) -> super::vals::TxdListList {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::TxdListList::from_bits(val as u8)
            }
            #[doc = "List type"]
            #[inline(always)]
            pub fn set_list(&mut self, val: super::vals::TxdListList) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for TxdList {
            #[inline(always)]
            fn default() -> TxdList {
                TxdList(0)
            }
        }
        #[doc = "Maximum number of bytes in transmit buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdMaxcnt(pub u32);
        impl TxdMaxcnt {
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
        }
        impl Default for TxdMaxcnt {
            #[inline(always)]
            fn default() -> TxdMaxcnt {
                TxdMaxcnt(0)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop temperature measurement"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Temperature measurement complete, data ready"]
        #[inline(always)]
        pub const fn events_datardy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Temperature in degC (0.25deg steps)"]
        #[inline(always)]
        pub const fn temp(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Slope of 1st piece wise linear function"]
        #[inline(always)]
        pub const fn a0(self) -> crate::common::Reg<regs::A0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize) as _) }
        }
        #[doc = "Slope of 2nd piece wise linear function"]
        #[inline(always)]
        pub const fn a1(self) -> crate::common::Reg<regs::A1, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
        }
        #[doc = "Slope of 3rd piece wise linear function"]
        #[inline(always)]
        pub const fn a2(self) -> crate::common::Reg<regs::A2, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0528usize) as _) }
        }
        #[doc = "Slope of 4th piece wise linear function"]
        #[inline(always)]
        pub const fn a3(self) -> crate::common::Reg<regs::A3, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x052cusize) as _) }
        }
        #[doc = "Slope of 5th piece wise linear function"]
        #[inline(always)]
        pub const fn a4(self) -> crate::common::Reg<regs::A4, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0530usize) as _) }
        }
        #[doc = "Slope of 6th piece wise linear function"]
        #[inline(always)]
        pub const fn a5(self) -> crate::common::Reg<regs::A5, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0534usize) as _) }
        }
        #[doc = "y-intercept of 1st piece wise linear function"]
        #[inline(always)]
        pub const fn b0(self) -> crate::common::Reg<regs::B0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize) as _) }
        }
        #[doc = "y-intercept of 2nd piece wise linear function"]
        #[inline(always)]
        pub const fn b1(self) -> crate::common::Reg<regs::B1, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0544usize) as _) }
        }
        #[doc = "y-intercept of 3rd piece wise linear function"]
        #[inline(always)]
        pub const fn b2(self) -> crate::common::Reg<regs::B2, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0548usize) as _) }
        }
        #[doc = "y-intercept of 4th piece wise linear function"]
        #[inline(always)]
        pub const fn b3(self) -> crate::common::Reg<regs::B3, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x054cusize) as _) }
        }
        #[doc = "y-intercept of 5th piece wise linear function"]
        #[inline(always)]
        pub const fn b4(self) -> crate::common::Reg<regs::B4, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0550usize) as _) }
        }
        #[doc = "y-intercept of 6th piece wise linear function"]
        #[inline(always)]
        pub const fn b5(self) -> crate::common::Reg<regs::B5, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0554usize) as _) }
        }
        #[doc = "End point of 1st piece wise linear function"]
        #[inline(always)]
        pub const fn t0(self) -> crate::common::Reg<regs::T0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0560usize) as _) }
        }
        #[doc = "End point of 2nd piece wise linear function"]
        #[inline(always)]
        pub const fn t1(self) -> crate::common::Reg<regs::T1, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0564usize) as _) }
        }
        #[doc = "End point of 3rd piece wise linear function"]
        #[inline(always)]
        pub const fn t2(self) -> crate::common::Reg<regs::T2, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0568usize) as _) }
        }
        #[doc = "End point of 4th piece wise linear function"]
        #[inline(always)]
        pub const fn t3(self) -> crate::common::Reg<regs::T3, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x056cusize) as _) }
        }
        #[doc = "End point of 5th piece wise linear function"]
        #[inline(always)]
        pub const fn t4(self) -> crate::common::Reg<regs::T4, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0570usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Slope of 1st piece wise linear function"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct A0(pub u32);
        impl A0 {
            #[doc = "Slope of 1st piece wise linear function"]
            #[inline(always)]
            pub const fn a0(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "Slope of 1st piece wise linear function"]
            #[inline(always)]
            pub fn set_a0(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
        }
        impl Default for A0 {
            #[inline(always)]
            fn default() -> A0 {
                A0(0)
            }
        }
        #[doc = "Slope of 2nd piece wise linear function"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct A1(pub u32);
        impl A1 {
            #[doc = "Slope of 2nd piece wise linear function"]
            #[inline(always)]
            pub const fn a1(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "Slope of 2nd piece wise linear function"]
            #[inline(always)]
            pub fn set_a1(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
        }
        impl Default for A1 {
            #[inline(always)]
            fn default() -> A1 {
                A1(0)
            }
        }
        #[doc = "Slope of 3rd piece wise linear function"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct A2(pub u32);
        impl A2 {
            #[doc = "Slope of 3rd piece wise linear function"]
            #[inline(always)]
            pub const fn a2(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "Slope of 3rd piece wise linear function"]
            #[inline(always)]
            pub fn set_a2(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
        }
        impl Default for A2 {
            #[inline(always)]
            fn default() -> A2 {
                A2(0)
            }
        }
        #[doc = "Slope of 4th piece wise linear function"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct A3(pub u32);
        impl A3 {
            #[doc = "Slope of 4th piece wise linear function"]
            #[inline(always)]
            pub const fn a3(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "Slope of 4th piece wise linear function"]
            #[inline(always)]
            pub fn set_a3(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
        }
        impl Default for A3 {
            #[inline(always)]
            fn default() -> A3 {
                A3(0)
            }
        }
        #[doc = "Slope of 5th piece wise linear function"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct A4(pub u32);
        impl A4 {
            #[doc = "Slope of 5th piece wise linear function"]
            #[inline(always)]
            pub const fn a4(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "Slope of 5th piece wise linear function"]
            #[inline(always)]
            pub fn set_a4(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
        }
        impl Default for A4 {
            #[inline(always)]
            fn default() -> A4 {
                A4(0)
            }
        }
        #[doc = "Slope of 6th piece wise linear function"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct A5(pub u32);
        impl A5 {
            #[doc = "Slope of 6th piece wise linear function"]
            #[inline(always)]
            pub const fn a5(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "Slope of 6th piece wise linear function"]
            #[inline(always)]
            pub fn set_a5(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
        }
        impl Default for A5 {
            #[inline(always)]
            fn default() -> A5 {
                A5(0)
            }
        }
        #[doc = "y-intercept of 1st piece wise linear function"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct B0(pub u32);
        impl B0 {
            #[doc = "y-intercept of 1st piece wise linear function"]
            #[inline(always)]
            pub const fn b0(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "y-intercept of 1st piece wise linear function"]
            #[inline(always)]
            pub fn set_b0(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for B0 {
            #[inline(always)]
            fn default() -> B0 {
                B0(0)
            }
        }
        #[doc = "y-intercept of 2nd piece wise linear function"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct B1(pub u32);
        impl B1 {
            #[doc = "y-intercept of 2nd piece wise linear function"]
            #[inline(always)]
            pub const fn b1(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "y-intercept of 2nd piece wise linear function"]
            #[inline(always)]
            pub fn set_b1(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for B1 {
            #[inline(always)]
            fn default() -> B1 {
                B1(0)
            }
        }
        #[doc = "y-intercept of 3rd piece wise linear function"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct B2(pub u32);
        impl B2 {
            #[doc = "y-intercept of 3rd piece wise linear function"]
            #[inline(always)]
            pub const fn b2(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "y-intercept of 3rd piece wise linear function"]
            #[inline(always)]
            pub fn set_b2(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for B2 {
            #[inline(always)]
            fn default() -> B2 {
                B2(0)
            }
        }
        #[doc = "y-intercept of 4th piece wise linear function"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct B3(pub u32);
        impl B3 {
            #[doc = "y-intercept of 4th piece wise linear function"]
            #[inline(always)]
            pub const fn b3(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "y-intercept of 4th piece wise linear function"]
            #[inline(always)]
            pub fn set_b3(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for B3 {
            #[inline(always)]
            fn default() -> B3 {
                B3(0)
            }
        }
        #[doc = "y-intercept of 5th piece wise linear function"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct B4(pub u32);
        impl B4 {
            #[doc = "y-intercept of 5th piece wise linear function"]
            #[inline(always)]
            pub const fn b4(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "y-intercept of 5th piece wise linear function"]
            #[inline(always)]
            pub fn set_b4(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for B4 {
            #[inline(always)]
            fn default() -> B4 {
                B4(0)
            }
        }
        #[doc = "y-intercept of 6th piece wise linear function"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct B5(pub u32);
        impl B5 {
            #[doc = "y-intercept of 6th piece wise linear function"]
            #[inline(always)]
            pub const fn b5(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "y-intercept of 6th piece wise linear function"]
            #[inline(always)]
            pub fn set_b5(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for B5 {
            #[inline(always)]
            fn default() -> B5 {
                B5(0)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Write '1' to disable interrupt for event DATARDY"]
            #[inline(always)]
            pub const fn datardy(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event DATARDY"]
            #[inline(always)]
            pub fn set_datardy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
        #[doc = "End point of 1st piece wise linear function"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct T0(pub u32);
        impl T0 {
            #[doc = "End point of 1st piece wise linear function"]
            #[inline(always)]
            pub const fn t0(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "End point of 1st piece wise linear function"]
            #[inline(always)]
            pub fn set_t0(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for T0 {
            #[inline(always)]
            fn default() -> T0 {
                T0(0)
            }
        }
        #[doc = "End point of 2nd piece wise linear function"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct T1(pub u32);
        impl T1 {
            #[doc = "End point of 2nd piece wise linear function"]
            #[inline(always)]
            pub const fn t1(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "End point of 2nd piece wise linear function"]
            #[inline(always)]
            pub fn set_t1(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for T1 {
            #[inline(always)]
            fn default() -> T1 {
                T1(0)
            }
        }
        #[doc = "End point of 3rd piece wise linear function"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct T2(pub u32);
        impl T2 {
            #[doc = "End point of 3rd piece wise linear function"]
            #[inline(always)]
            pub const fn t2(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "End point of 3rd piece wise linear function"]
            #[inline(always)]
            pub fn set_t2(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for T2 {
            #[inline(always)]
            fn default() -> T2 {
                T2(0)
            }
        }
        #[doc = "End point of 4th piece wise linear function"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct T3(pub u32);
        impl T3 {
            #[doc = "End point of 4th piece wise linear function"]
            #[inline(always)]
            pub const fn t3(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "End point of 4th piece wise linear function"]
            #[inline(always)]
            pub fn set_t3(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for T3 {
            #[inline(always)]
            fn default() -> T3 {
                T3(0)
            }
        }
        #[doc = "End point of 5th piece wise linear function"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct T4(pub u32);
        impl T4 {
            #[doc = "End point of 5th piece wise linear function"]
            #[inline(always)]
            pub const fn t4(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "End point of 5th piece wise linear function"]
            #[inline(always)]
            pub fn set_t4(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for T4 {
            #[inline(always)]
            fn default() -> T4 {
                T4(0)
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop Timer"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Increment Timer (Counter mode only)"]
        #[inline(always)]
        pub const fn tasks_count(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Clear time"]
        #[inline(always)]
        pub const fn tasks_clear(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Deprecated register - Shut down timer"]
        #[inline(always)]
        pub const fn tasks_shutdown(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Description collection: Capture Timer value to CC\\[n\\] register"]
        #[inline(always)]
        pub const fn tasks_capture(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
            assert!(n < 6usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Compare event on CC\\[n\\] match"]
        #[inline(always)]
        pub const fn events_compare(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 6usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize + n * 4usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Timer mode selection"]
        #[inline(always)]
        pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Configure the number of bits used by the TIMER"]
        #[inline(always)]
        pub const fn bitmode(self) -> crate::common::Reg<regs::Bitmode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Timer prescaler register"]
        #[inline(always)]
        pub const fn prescaler(self) -> crate::common::Reg<regs::Prescaler, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "Description collection: Capture/Compare register n"]
        #[inline(always)]
        pub const fn cc(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 6usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize + n * 4usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Configure the number of bits used by the TIMER"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Bitmode(pub u32);
        impl Bitmode {
            #[doc = "Timer bit width"]
            #[inline(always)]
            pub const fn bitmode(&self) -> super::vals::Bitmode {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Bitmode::from_bits(val as u8)
            }
            #[doc = "Timer bit width"]
            #[inline(always)]
            pub fn set_bitmode(&mut self, val: super::vals::Bitmode) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Bitmode {
            #[inline(always)]
            fn default() -> Bitmode {
                Bitmode(0)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Write '1' to disable interrupt for event COMPARE\\[0\\]"]
            #[inline(always)]
            pub const fn compare(&self, n: usize) -> bool {
                assert!(n < 6usize);
                let offs = 16usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event COMPARE\\[0\\]"]
            #[inline(always)]
            pub fn set_compare(&mut self, n: usize, val: bool) {
                assert!(n < 6usize);
                let offs = 16usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
        #[doc = "Timer mode selection"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Mode(pub u32);
        impl Mode {
            #[doc = "Timer mode"]
            #[inline(always)]
            pub const fn mode(&self) -> super::vals::Mode {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Mode::from_bits(val as u8)
            }
            #[doc = "Timer mode"]
            #[inline(always)]
            pub fn set_mode(&mut self, val: super::vals::Mode) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Mode {
            #[inline(always)]
            fn default() -> Mode {
                Mode(0)
            }
        }
        #[doc = "Timer prescaler register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Prescaler(pub u32);
        impl Prescaler {
            #[doc = "Prescaler value"]
            #[inline(always)]
            pub const fn prescaler(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "Prescaler value"]
            #[inline(always)]
            pub fn set_prescaler(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Prescaler {
            #[inline(always)]
            fn default() -> Prescaler {
                Prescaler(0)
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event COMPARE\\[0\\] and task CLEAR"]
            #[inline(always)]
            pub const fn compare_clear(&self, n: usize) -> bool {
                assert!(n < 6usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event COMPARE\\[0\\] and task CLEAR"]
            #[inline(always)]
            pub fn set_compare_clear(&mut self, n: usize, val: bool) {
                assert!(n < 6usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Shortcut between event COMPARE\\[0\\] and task STOP"]
            #[inline(always)]
            pub const fn compare_stop(&self, n: usize) -> bool {
                assert!(n < 6usize);
                let offs = 8usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event COMPARE\\[0\\] and task STOP"]
            #[inline(always)]
            pub fn set_compare_stop(&mut self, n: usize, val: bool) {
                assert!(n < 6usize);
                let offs = 8usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
pub mod twi {
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
        #[doc = "Pin select for SCL"]
        #[inline(always)]
        pub const fn scl(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Pin select for SDA"]
        #[inline(always)]
        pub const fn sda(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    #[doc = "I2C compatible Two-Wire Interface"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Twi {
        ptr: *mut u8,
    }
    unsafe impl Send for Twi {}
    unsafe impl Sync for Twi {}
    impl Twi {
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Start TWI transmit sequence"]
        #[inline(always)]
        pub const fn tasks_starttx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Stop TWI transaction"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Suspend TWI transaction"]
        #[inline(always)]
        pub const fn tasks_suspend(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "Resume TWI transaction"]
        #[inline(always)]
        pub const fn tasks_resume(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "TWI stopped"]
        #[inline(always)]
        pub const fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "TWI RXD byte received"]
        #[inline(always)]
        pub const fn events_rxdready(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "TWI TXD byte sent"]
        #[inline(always)]
        pub const fn events_txdsent(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
        }
        #[doc = "TWI error"]
        #[inline(always)]
        pub const fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
        }
        #[doc = "TWI byte boundary, generated before each byte that is sent or received"]
        #[inline(always)]
        pub const fn events_bb(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
        }
        #[doc = "TWI entered the suspended state"]
        #[inline(always)]
        pub const fn events_suspended(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Error source"]
        #[inline(always)]
        pub const fn errorsrc(self) -> crate::common::Reg<regs::Errorsrc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04c4usize) as _) }
        }
        #[doc = "Enable TWI"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn psel(self) -> Psel {
            unsafe { Psel::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "RXD register"]
        #[inline(always)]
        pub const fn rxd(self) -> crate::common::Reg<regs::Rxd, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
        #[doc = "TXD register"]
        #[inline(always)]
        pub const fn txd(self) -> crate::common::Reg<regs::Txd, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
        }
        #[doc = "TWI frequency. Accuracy depends on the HFCLK source selected."]
        #[inline(always)]
        pub const fn frequency(self) -> crate::common::Reg<regs::Frequency, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
        }
        #[doc = "Address used in the TWI transfer"]
        #[inline(always)]
        pub const fn address(self) -> crate::common::Reg<regs::Address, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0588usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Address used in the TWI transfer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Address(pub u32);
        impl Address {
            #[doc = "Address used in the TWI transfer"]
            #[inline(always)]
            pub const fn address(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "Address used in the TWI transfer"]
            #[inline(always)]
            pub fn set_address(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
            }
        }
        impl Default for Address {
            #[inline(always)]
            fn default() -> Address {
                Address(0)
            }
        }
        #[doc = "Enable TWI"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable TWI"]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable TWI"]
            #[inline(always)]
            pub fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        #[doc = "Error source"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Errorsrc(pub u32);
        impl Errorsrc {
            #[doc = "Overrun error"]
            #[inline(always)]
            pub const fn overrun(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun error"]
            #[inline(always)]
            pub fn set_overrun(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "NACK received after sending the address (write '1' to clear)"]
            #[inline(always)]
            pub const fn anack(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "NACK received after sending the address (write '1' to clear)"]
            #[inline(always)]
            pub fn set_anack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "NACK received after sending a data byte (write '1' to clear)"]
            #[inline(always)]
            pub const fn dnack(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "NACK received after sending a data byte (write '1' to clear)"]
            #[inline(always)]
            pub fn set_dnack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Errorsrc {
            #[inline(always)]
            fn default() -> Errorsrc {
                Errorsrc(0)
            }
        }
        #[doc = "TWI frequency. Accuracy depends on the HFCLK source selected."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Frequency(pub u32);
        impl Frequency {
            #[doc = "TWI master clock frequency"]
            #[inline(always)]
            pub const fn frequency(&self) -> super::vals::Frequency {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Frequency::from_bits(val as u32)
            }
            #[doc = "TWI master clock frequency"]
            #[inline(always)]
            pub fn set_frequency(&mut self, val: super::vals::Frequency) {
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
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Write '1' to disable interrupt for event STOPPED"]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event STOPPED"]
            #[inline(always)]
            pub fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to disable interrupt for event RXDREADY"]
            #[inline(always)]
            pub const fn rxdready(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event RXDREADY"]
            #[inline(always)]
            pub fn set_rxdready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Write '1' to disable interrupt for event TXDSENT"]
            #[inline(always)]
            pub const fn txdsent(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event TXDSENT"]
            #[inline(always)]
            pub fn set_txdsent(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Write '1' to disable interrupt for event ERROR"]
            #[inline(always)]
            pub const fn error(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ERROR"]
            #[inline(always)]
            pub fn set_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Write '1' to disable interrupt for event BB"]
            #[inline(always)]
            pub const fn bb(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event BB"]
            #[inline(always)]
            pub fn set_bb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Write '1' to disable interrupt for event SUSPENDED"]
            #[inline(always)]
            pub const fn suspended(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event SUSPENDED"]
            #[inline(always)]
            pub fn set_suspended(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
        #[doc = "RXD register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxd(pub u32);
        impl Rxd {
            #[doc = "RXD register"]
            #[inline(always)]
            pub const fn rxd(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "RXD register"]
            #[inline(always)]
            pub fn set_rxd(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Rxd {
            #[inline(always)]
            fn default() -> Rxd {
                Rxd(0)
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event BB and task SUSPEND"]
            #[inline(always)]
            pub const fn bb_suspend(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event BB and task SUSPEND"]
            #[inline(always)]
            pub fn set_bb_suspend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Shortcut between event BB and task STOP"]
            #[inline(always)]
            pub const fn bb_stop(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event BB and task STOP"]
            #[inline(always)]
            pub fn set_bb_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        #[doc = "TXD register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Txd(pub u32);
        impl Txd {
            #[doc = "TXD register"]
            #[inline(always)]
            pub const fn txd(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "TXD register"]
            #[inline(always)]
            pub fn set_txd(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Txd {
            #[inline(always)]
            fn default() -> Txd {
                Txd(0)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Enable {
            #[doc = "Disable TWI"]
            DISABLED = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            #[doc = "Enable TWI"]
            ENABLED = 0x05,
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
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Frequency(pub u32);
        impl Frequency {
            #[doc = "100 kbps"]
            pub const K100: Self = Self(0x0198_0000);
            #[doc = "250 kbps"]
            pub const K250: Self = Self(0x0400_0000);
            #[doc = "400 kbps (actual rate 410.256 kbps)"]
            pub const K400: Self = Self(0x0668_0000);
        }
        impl Frequency {
            pub const fn from_bits(val: u32) -> Frequency {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Pin select for SDA signal"]
        #[inline(always)]
        pub const fn sda(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in receive buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::RxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::RxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "EasyDMA list type"]
        #[inline(always)]
        pub const fn list(self) -> crate::common::Reg<regs::RxdList, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Start TWI transmit sequence"]
        #[inline(always)]
        pub const fn tasks_starttx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Stop TWI transaction. Must be issued while the TWI master is not suspended."]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Suspend TWI transaction"]
        #[inline(always)]
        pub const fn tasks_suspend(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "Resume TWI transaction"]
        #[inline(always)]
        pub const fn tasks_resume(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "TWI stopped"]
        #[inline(always)]
        pub const fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "TWI error"]
        #[inline(always)]
        pub const fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
        }
        #[doc = "Last byte has been sent out after the SUSPEND task has been issued, TWI traffic is now suspended."]
        #[inline(always)]
        pub const fn events_suspended(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
        }
        #[doc = "Receive sequence started"]
        #[inline(always)]
        pub const fn events_rxstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
        }
        #[doc = "Transmit sequence started"]
        #[inline(always)]
        pub const fn events_txstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
        }
        #[doc = "Byte boundary, starting to receive the last byte"]
        #[inline(always)]
        pub const fn events_lastrx(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x015cusize) as _) }
        }
        #[doc = "Byte boundary, starting to transmit the last byte"]
        #[inline(always)]
        pub const fn events_lasttx(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Error source"]
        #[inline(always)]
        pub const fn errorsrc(self) -> crate::common::Reg<regs::Errorsrc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04c4usize) as _) }
        }
        #[doc = "Enable TWIM"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn psel(self) -> Psel {
            unsafe { Psel::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "TWI frequency. Accuracy depends on the HFCLK source selected."]
        #[inline(always)]
        pub const fn frequency(self) -> crate::common::Reg<regs::Frequency, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
        }
        #[doc = "RXD EasyDMA channel"]
        #[inline(always)]
        pub const fn rxd(self) -> Rxd {
            unsafe { Rxd::from_ptr(self.ptr.add(0x0534usize) as _) }
        }
        #[doc = "TXD EasyDMA channel"]
        #[inline(always)]
        pub const fn txd(self) -> Txd {
            unsafe { Txd::from_ptr(self.ptr.add(0x0544usize) as _) }
        }
        #[doc = "Address used in the TWI transfer"]
        #[inline(always)]
        pub const fn address(self) -> crate::common::Reg<regs::Address, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0588usize) as _) }
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in transmit buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::TxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::TxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "EasyDMA list type"]
        #[inline(always)]
        pub const fn list(self) -> crate::common::Reg<regs::TxdList, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Address used in the TWI transfer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Address(pub u32);
        impl Address {
            #[doc = "Address used in the TWI transfer"]
            #[inline(always)]
            pub const fn address(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "Address used in the TWI transfer"]
            #[inline(always)]
            pub fn set_address(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
            }
        }
        impl Default for Address {
            #[inline(always)]
            fn default() -> Address {
                Address(0)
            }
        }
        #[doc = "Enable TWIM"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable TWIM"]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable TWIM"]
            #[inline(always)]
            pub fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        #[doc = "Error source"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Errorsrc(pub u32);
        impl Errorsrc {
            #[doc = "Overrun error"]
            #[inline(always)]
            pub const fn overrun(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun error"]
            #[inline(always)]
            pub fn set_overrun(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "NACK received after sending the address (write '1' to clear)"]
            #[inline(always)]
            pub const fn anack(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "NACK received after sending the address (write '1' to clear)"]
            #[inline(always)]
            pub fn set_anack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "NACK received after sending a data byte (write '1' to clear)"]
            #[inline(always)]
            pub const fn dnack(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "NACK received after sending a data byte (write '1' to clear)"]
            #[inline(always)]
            pub fn set_dnack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Errorsrc {
            #[inline(always)]
            fn default() -> Errorsrc {
                Errorsrc(0)
            }
        }
        #[doc = "TWI frequency. Accuracy depends on the HFCLK source selected."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Frequency(pub u32);
        impl Frequency {
            #[doc = "TWI master clock frequency"]
            #[inline(always)]
            pub const fn frequency(&self) -> super::vals::Frequency {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Frequency::from_bits(val as u32)
            }
            #[doc = "TWI master clock frequency"]
            #[inline(always)]
            pub fn set_frequency(&mut self, val: super::vals::Frequency) {
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
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Enable or disable interrupt for event STOPPED"]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event STOPPED"]
            #[inline(always)]
            pub fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for event ERROR"]
            #[inline(always)]
            pub const fn error(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event ERROR"]
            #[inline(always)]
            pub fn set_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Enable or disable interrupt for event SUSPENDED"]
            #[inline(always)]
            pub const fn suspended(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event SUSPENDED"]
            #[inline(always)]
            pub fn set_suspended(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Enable or disable interrupt for event RXSTARTED"]
            #[inline(always)]
            pub const fn rxstarted(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RXSTARTED"]
            #[inline(always)]
            pub fn set_rxstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "Enable or disable interrupt for event TXSTARTED"]
            #[inline(always)]
            pub const fn txstarted(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event TXSTARTED"]
            #[inline(always)]
            pub fn set_txstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "Enable or disable interrupt for event LASTRX"]
            #[inline(always)]
            pub const fn lastrx(&self) -> bool {
                let val = (self.0 >> 23usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event LASTRX"]
            #[inline(always)]
            pub fn set_lastrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
            }
            #[doc = "Enable or disable interrupt for event LASTTX"]
            #[inline(always)]
            pub const fn lasttx(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event LASTTX"]
            #[inline(always)]
            pub fn set_lasttx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdAmount(pub u32);
        impl RxdAmount {
            #[doc = "Number of bytes transferred in the last transaction. In case of NACK error, includes the NACK'ed byte."]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last transaction. In case of NACK error, includes the NACK'ed byte."]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
        }
        impl Default for RxdAmount {
            #[inline(always)]
            fn default() -> RxdAmount {
                RxdAmount(0)
            }
        }
        #[doc = "EasyDMA list type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdList(pub u32);
        impl RxdList {
            #[doc = "List type"]
            #[inline(always)]
            pub const fn list(&self) -> super::vals::RxdListList {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::RxdListList::from_bits(val as u8)
            }
            #[doc = "List type"]
            #[inline(always)]
            pub fn set_list(&mut self, val: super::vals::RxdListList) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
            }
        }
        impl Default for RxdList {
            #[inline(always)]
            fn default() -> RxdList {
                RxdList(0)
            }
        }
        #[doc = "Maximum number of bytes in receive buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdMaxcnt(pub u32);
        impl RxdMaxcnt {
            #[doc = "Maximum number of bytes in receive buffer"]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Maximum number of bytes in receive buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
        }
        impl Default for RxdMaxcnt {
            #[inline(always)]
            fn default() -> RxdMaxcnt {
                RxdMaxcnt(0)
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event LASTTX and task STARTRX"]
            #[inline(always)]
            pub const fn lasttx_startrx(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event LASTTX and task STARTRX"]
            #[inline(always)]
            pub fn set_lasttx_startrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Shortcut between event LASTTX and task SUSPEND"]
            #[inline(always)]
            pub const fn lasttx_suspend(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event LASTTX and task SUSPEND"]
            #[inline(always)]
            pub fn set_lasttx_suspend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Shortcut between event LASTTX and task STOP"]
            #[inline(always)]
            pub const fn lasttx_stop(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event LASTTX and task STOP"]
            #[inline(always)]
            pub fn set_lasttx_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Shortcut between event LASTRX and task STARTTX"]
            #[inline(always)]
            pub const fn lastrx_starttx(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event LASTRX and task STARTTX"]
            #[inline(always)]
            pub fn set_lastrx_starttx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Shortcut between event LASTRX and task SUSPEND"]
            #[inline(always)]
            pub const fn lastrx_suspend(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event LASTRX and task SUSPEND"]
            #[inline(always)]
            pub fn set_lastrx_suspend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "Shortcut between event LASTRX and task STOP"]
            #[inline(always)]
            pub const fn lastrx_stop(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event LASTRX and task STOP"]
            #[inline(always)]
            pub fn set_lastrx_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdAmount(pub u32);
        impl TxdAmount {
            #[doc = "Number of bytes transferred in the last transaction. In case of NACK error, includes the NACK'ed byte."]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last transaction. In case of NACK error, includes the NACK'ed byte."]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
        }
        impl Default for TxdAmount {
            #[inline(always)]
            fn default() -> TxdAmount {
                TxdAmount(0)
            }
        }
        #[doc = "EasyDMA list type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdList(pub u32);
        impl TxdList {
            #[doc = "List type"]
            #[inline(always)]
            pub const fn list(&self) -> super::vals::TxdListList {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::TxdListList::from_bits(val as u8)
            }
            #[doc = "List type"]
            #[inline(always)]
            pub fn set_list(&mut self, val: super::vals::TxdListList) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
            }
        }
        impl Default for TxdList {
            #[inline(always)]
            fn default() -> TxdList {
                TxdList(0)
            }
        }
        #[doc = "Maximum number of bytes in transmit buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdMaxcnt(pub u32);
        impl TxdMaxcnt {
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
        }
        impl Default for TxdMaxcnt {
            #[inline(always)]
            fn default() -> TxdMaxcnt {
                TxdMaxcnt(0)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        pub struct Frequency(pub u32);
        impl Frequency {
            #[doc = "100 kbps"]
            pub const K100: Self = Self(0x0198_0000);
            #[doc = "250 kbps"]
            pub const K250: Self = Self(0x0400_0000);
            #[doc = "400 kbps"]
            pub const K400: Self = Self(0x0640_0000);
        }
        impl Frequency {
            pub const fn from_bits(val: u32) -> Frequency {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Pin select for SDA signal"]
        #[inline(always)]
        pub const fn sda(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in RXD buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::RxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Number of bytes transferred in the last RXD transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::RxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "EasyDMA list type"]
        #[inline(always)]
        pub const fn list(self) -> crate::common::Reg<regs::RxdList, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Suspend TWI transaction"]
        #[inline(always)]
        pub const fn tasks_suspend(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "Resume TWI transaction"]
        #[inline(always)]
        pub const fn tasks_resume(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "Prepare the TWI slave to respond to a write command"]
        #[inline(always)]
        pub const fn tasks_preparerx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
        }
        #[doc = "Prepare the TWI slave to respond to a read command"]
        #[inline(always)]
        pub const fn tasks_preparetx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
        }
        #[doc = "TWI stopped"]
        #[inline(always)]
        pub const fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "TWI error"]
        #[inline(always)]
        pub const fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
        }
        #[doc = "Receive sequence started"]
        #[inline(always)]
        pub const fn events_rxstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
        }
        #[doc = "Transmit sequence started"]
        #[inline(always)]
        pub const fn events_txstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
        }
        #[doc = "Write command received"]
        #[inline(always)]
        pub const fn events_write(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0164usize) as _) }
        }
        #[doc = "Read command received"]
        #[inline(always)]
        pub const fn events_read(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0168usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Error source"]
        #[inline(always)]
        pub const fn errorsrc(self) -> crate::common::Reg<regs::Errorsrc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04d0usize) as _) }
        }
        #[doc = "Status register indicating which address had a match"]
        #[inline(always)]
        pub const fn match_(self) -> crate::common::Reg<regs::Match, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04d4usize) as _) }
        }
        #[doc = "Enable TWIS"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn psel(self) -> Psel {
            unsafe { Psel::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "RXD EasyDMA channel"]
        #[inline(always)]
        pub const fn rxd(self) -> Rxd {
            unsafe { Rxd::from_ptr(self.ptr.add(0x0534usize) as _) }
        }
        #[doc = "TXD EasyDMA channel"]
        #[inline(always)]
        pub const fn txd(self) -> Txd {
            unsafe { Txd::from_ptr(self.ptr.add(0x0544usize) as _) }
        }
        #[doc = "Description collection: TWI slave address n"]
        #[inline(always)]
        pub const fn address(
            self,
            n: usize,
        ) -> crate::common::Reg<regs::Address, crate::common::RW> {
            assert!(n < 2usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0588usize + n * 4usize) as _) }
        }
        #[doc = "Configuration register for the address match mechanism"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0594usize) as _) }
        }
        #[doc = "Over-read character. Character sent out in case of an over-read of the transmit buffer."]
        #[inline(always)]
        pub const fn orc(self) -> crate::common::Reg<regs::Orc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05c0usize) as _) }
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in TXD buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::TxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Number of bytes transferred in the last TXD transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::TxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "EasyDMA list type"]
        #[inline(always)]
        pub const fn list(self) -> crate::common::Reg<regs::TxdList, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Description collection: TWI slave address n"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Address(pub u32);
        impl Address {
            #[doc = "TWI slave address"]
            #[inline(always)]
            pub const fn address(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "TWI slave address"]
            #[inline(always)]
            pub fn set_address(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
            }
        }
        impl Default for Address {
            #[inline(always)]
            fn default() -> Address {
                Address(0)
            }
        }
        #[doc = "Configuration register for the address match mechanism"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Enable or disable address matching on ADDRESS\\[0\\]"]
            #[inline(always)]
            pub const fn address0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable address matching on ADDRESS\\[0\\]"]
            #[inline(always)]
            pub fn set_address0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable address matching on ADDRESS\\[1\\]"]
            #[inline(always)]
            pub const fn address1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable address matching on ADDRESS\\[1\\]"]
            #[inline(always)]
            pub fn set_address1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        #[doc = "Enable TWIS"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable TWIS"]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable TWIS"]
            #[inline(always)]
            pub fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        #[doc = "Error source"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Errorsrc(pub u32);
        impl Errorsrc {
            #[doc = "RX buffer overflow detected, and prevented"]
            #[inline(always)]
            pub const fn overflow(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "RX buffer overflow detected, and prevented"]
            #[inline(always)]
            pub fn set_overflow(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "NACK sent after receiving a data byte"]
            #[inline(always)]
            pub const fn dnack(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "NACK sent after receiving a data byte"]
            #[inline(always)]
            pub fn set_dnack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "TX buffer over-read detected, and prevented"]
            #[inline(always)]
            pub const fn overread(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "TX buffer over-read detected, and prevented"]
            #[inline(always)]
            pub fn set_overread(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Errorsrc {
            #[inline(always)]
            fn default() -> Errorsrc {
                Errorsrc(0)
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Enable or disable interrupt for event STOPPED"]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event STOPPED"]
            #[inline(always)]
            pub fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for event ERROR"]
            #[inline(always)]
            pub const fn error(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event ERROR"]
            #[inline(always)]
            pub fn set_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Enable or disable interrupt for event RXSTARTED"]
            #[inline(always)]
            pub const fn rxstarted(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RXSTARTED"]
            #[inline(always)]
            pub fn set_rxstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "Enable or disable interrupt for event TXSTARTED"]
            #[inline(always)]
            pub const fn txstarted(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event TXSTARTED"]
            #[inline(always)]
            pub fn set_txstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "Enable or disable interrupt for event WRITE"]
            #[inline(always)]
            pub const fn write(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event WRITE"]
            #[inline(always)]
            pub fn set_write(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
            #[doc = "Enable or disable interrupt for event READ"]
            #[inline(always)]
            pub const fn read(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event READ"]
            #[inline(always)]
            pub fn set_read(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
        #[doc = "Status register indicating which address had a match"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Match(pub u32);
        impl Match {
            #[doc = "Which of the addresses in {ADDRESS} matched the incoming address"]
            #[inline(always)]
            pub const fn match_(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Which of the addresses in {ADDRESS} matched the incoming address"]
            #[inline(always)]
            pub fn set_match_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Match {
            #[inline(always)]
            fn default() -> Match {
                Match(0)
            }
        }
        #[doc = "Over-read character. Character sent out in case of an over-read of the transmit buffer."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Orc(pub u32);
        impl Orc {
            #[doc = "Over-read character. Character sent out in case of an over-read of the transmit buffer."]
            #[inline(always)]
            pub const fn orc(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Over-read character. Character sent out in case of an over-read of the transmit buffer."]
            #[inline(always)]
            pub fn set_orc(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Orc {
            #[inline(always)]
            fn default() -> Orc {
                Orc(0)
            }
        }
        #[doc = "Number of bytes transferred in the last RXD transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdAmount(pub u32);
        impl RxdAmount {
            #[doc = "Number of bytes transferred in the last RXD transaction"]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last RXD transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
        }
        impl Default for RxdAmount {
            #[inline(always)]
            fn default() -> RxdAmount {
                RxdAmount(0)
            }
        }
        #[doc = "EasyDMA list type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdList(pub u32);
        impl RxdList {
            #[doc = "List type"]
            #[inline(always)]
            pub const fn list(&self) -> super::vals::RxdListList {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::RxdListList::from_bits(val as u8)
            }
            #[doc = "List type"]
            #[inline(always)]
            pub fn set_list(&mut self, val: super::vals::RxdListList) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for RxdList {
            #[inline(always)]
            fn default() -> RxdList {
                RxdList(0)
            }
        }
        #[doc = "Maximum number of bytes in RXD buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdMaxcnt(pub u32);
        impl RxdMaxcnt {
            #[doc = "Maximum number of bytes in RXD buffer"]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Maximum number of bytes in RXD buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
        }
        impl Default for RxdMaxcnt {
            #[inline(always)]
            fn default() -> RxdMaxcnt {
                RxdMaxcnt(0)
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event WRITE and task SUSPEND"]
            #[inline(always)]
            pub const fn write_suspend(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event WRITE and task SUSPEND"]
            #[inline(always)]
            pub fn set_write_suspend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Shortcut between event READ and task SUSPEND"]
            #[inline(always)]
            pub const fn read_suspend(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event READ and task SUSPEND"]
            #[inline(always)]
            pub fn set_read_suspend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        #[doc = "Number of bytes transferred in the last TXD transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdAmount(pub u32);
        impl TxdAmount {
            #[doc = "Number of bytes transferred in the last TXD transaction"]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last TXD transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
        }
        impl Default for TxdAmount {
            #[inline(always)]
            fn default() -> TxdAmount {
                TxdAmount(0)
            }
        }
        #[doc = "EasyDMA list type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdList(pub u32);
        impl TxdList {
            #[doc = "List type"]
            #[inline(always)]
            pub const fn list(&self) -> super::vals::TxdListList {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::TxdListList::from_bits(val as u8)
            }
            #[doc = "List type"]
            #[inline(always)]
            pub fn set_list(&mut self, val: super::vals::TxdListList) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for TxdList {
            #[inline(always)]
            fn default() -> TxdList {
                TxdList(0)
            }
        }
        #[doc = "Maximum number of bytes in TXD buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdMaxcnt(pub u32);
        impl TxdMaxcnt {
            #[doc = "Maximum number of bytes in TXD buffer"]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Maximum number of bytes in TXD buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
        }
        impl Default for TxdMaxcnt {
            #[inline(always)]
            fn default() -> TxdMaxcnt {
                TxdMaxcnt(0)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
pub mod uart {
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
        #[doc = "Pin select for RTS"]
        #[inline(always)]
        pub const fn rts(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Pin select for TXD"]
        #[inline(always)]
        pub const fn txd(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Pin select for CTS"]
        #[inline(always)]
        pub const fn cts(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Pin select for RXD"]
        #[inline(always)]
        pub const fn rxd(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uart {
        ptr: *mut u8,
    }
    unsafe impl Send for Uart {}
    unsafe impl Sync for Uart {}
    impl Uart {
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop UART receiver"]
        #[inline(always)]
        pub const fn tasks_stoprx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Start UART transmitter"]
        #[inline(always)]
        pub const fn tasks_starttx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Stop UART transmitter"]
        #[inline(always)]
        pub const fn tasks_stoptx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Suspend UART"]
        #[inline(always)]
        pub const fn tasks_suspend(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "CTS is activated (set low). Clear To Send."]
        #[inline(always)]
        pub const fn events_cts(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "CTS is deactivated (set high). Not Clear To Send."]
        #[inline(always)]
        pub const fn events_ncts(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Data received in RXD"]
        #[inline(always)]
        pub const fn events_rxdrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "Data sent from TXD"]
        #[inline(always)]
        pub const fn events_txdrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
        }
        #[doc = "Error detected"]
        #[inline(always)]
        pub const fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
        }
        #[doc = "Receiver timeout"]
        #[inline(always)]
        pub const fn events_rxto(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Error source"]
        #[inline(always)]
        pub const fn errorsrc(self) -> crate::common::Reg<regs::Errorsrc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0480usize) as _) }
        }
        #[doc = "Enable UART"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn psel(self) -> Psel {
            unsafe { Psel::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "RXD register"]
        #[inline(always)]
        pub const fn rxd(self) -> crate::common::Reg<regs::Rxd, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
        #[doc = "TXD register"]
        #[inline(always)]
        pub const fn txd(self) -> crate::common::Reg<regs::Txd, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
        }
        #[doc = "Baud rate. Accuracy depends on the HFCLK source selected."]
        #[inline(always)]
        pub const fn baudrate(self) -> crate::common::Reg<regs::Baudrate, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
        }
        #[doc = "Configuration of parity and hardware flow control"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x056cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Baud rate. Accuracy depends on the HFCLK source selected."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Baudrate(pub u32);
        impl Baudrate {
            #[doc = "Baud rate"]
            #[inline(always)]
            pub const fn baudrate(&self) -> super::vals::Baudrate {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Baudrate::from_bits(val as u32)
            }
            #[doc = "Baud rate"]
            #[inline(always)]
            pub fn set_baudrate(&mut self, val: super::vals::Baudrate) {
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
        #[doc = "Configuration of parity and hardware flow control"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Hardware flow control"]
            #[inline(always)]
            pub const fn hwfc(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Hardware flow control"]
            #[inline(always)]
            pub fn set_hwfc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Parity"]
            #[inline(always)]
            pub const fn parity(&self) -> super::vals::ConfigParity {
                let val = (self.0 >> 1usize) & 0x07;
                super::vals::ConfigParity::from_bits(val as u8)
            }
            #[doc = "Parity"]
            #[inline(always)]
            pub fn set_parity(&mut self, val: super::vals::ConfigParity) {
                self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
            }
            #[doc = "Stop bits"]
            #[inline(always)]
            pub const fn stop(&self) -> super::vals::Stop {
                let val = (self.0 >> 4usize) & 0x01;
                super::vals::Stop::from_bits(val as u8)
            }
            #[doc = "Stop bits"]
            #[inline(always)]
            pub fn set_stop(&mut self, val: super::vals::Stop) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        #[doc = "Enable UART"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable UART"]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable UART"]
            #[inline(always)]
            pub fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        #[doc = "Error source"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Errorsrc(pub u32);
        impl Errorsrc {
            #[doc = "Overrun error"]
            #[inline(always)]
            pub const fn overrun(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun error"]
            #[inline(always)]
            pub fn set_overrun(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Parity error"]
            #[inline(always)]
            pub const fn parity(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Parity error"]
            #[inline(always)]
            pub fn set_parity(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Framing error occurred"]
            #[inline(always)]
            pub const fn framing(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Framing error occurred"]
            #[inline(always)]
            pub fn set_framing(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Break condition"]
            #[inline(always)]
            pub const fn break_(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Break condition"]
            #[inline(always)]
            pub fn set_break_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Errorsrc {
            #[inline(always)]
            fn default() -> Errorsrc {
                Errorsrc(0)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Write '1' to disable interrupt for event CTS"]
            #[inline(always)]
            pub const fn cts(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event CTS"]
            #[inline(always)]
            pub fn set_cts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to disable interrupt for event NCTS"]
            #[inline(always)]
            pub const fn ncts(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event NCTS"]
            #[inline(always)]
            pub fn set_ncts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to disable interrupt for event RXDRDY"]
            #[inline(always)]
            pub const fn rxdrdy(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event RXDRDY"]
            #[inline(always)]
            pub fn set_rxdrdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Write '1' to disable interrupt for event TXDRDY"]
            #[inline(always)]
            pub const fn txdrdy(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event TXDRDY"]
            #[inline(always)]
            pub fn set_txdrdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Write '1' to disable interrupt for event ERROR"]
            #[inline(always)]
            pub const fn error(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ERROR"]
            #[inline(always)]
            pub fn set_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Write '1' to disable interrupt for event RXTO"]
            #[inline(always)]
            pub const fn rxto(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event RXTO"]
            #[inline(always)]
            pub fn set_rxto(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
        #[doc = "RXD register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxd(pub u32);
        impl Rxd {
            #[doc = "RX data received in previous transfers, double buffered"]
            #[inline(always)]
            pub const fn rxd(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "RX data received in previous transfers, double buffered"]
            #[inline(always)]
            pub fn set_rxd(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Rxd {
            #[inline(always)]
            fn default() -> Rxd {
                Rxd(0)
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event CTS and task STARTRX"]
            #[inline(always)]
            pub const fn cts_startrx(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event CTS and task STARTRX"]
            #[inline(always)]
            pub fn set_cts_startrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Shortcut between event NCTS and task STOPRX"]
            #[inline(always)]
            pub const fn ncts_stoprx(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event NCTS and task STOPRX"]
            #[inline(always)]
            pub fn set_ncts_stoprx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        #[doc = "TXD register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Txd(pub u32);
        impl Txd {
            #[doc = "TX data to be transferred"]
            #[inline(always)]
            pub const fn txd(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "TX data to be transferred"]
            #[inline(always)]
            pub fn set_txd(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Txd {
            #[inline(always)]
            fn default() -> Txd {
                Txd(0)
            }
        }
    }
    pub mod vals {
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Baudrate(pub u32);
        impl Baudrate {
            #[doc = "1200 baud (actual rate: 1205)"]
            pub const BAUD1200: Self = Self(0x0004_f000);
            #[doc = "2400 baud (actual rate: 2396)"]
            pub const BAUD2400: Self = Self(0x0009_d000);
            #[doc = "4800 baud (actual rate: 4808)"]
            pub const BAUD4800: Self = Self(0x0013_b000);
            #[doc = "9600 baud (actual rate: 9598)"]
            pub const BAUD9600: Self = Self(0x0027_5000);
            #[doc = "14400 baud (actual rate: 14414)"]
            pub const BAUD14400: Self = Self(0x003b_0000);
            #[doc = "19200 baud (actual rate: 19208)"]
            pub const BAUD19200: Self = Self(0x004e_a000);
            #[doc = "28800 baud (actual rate: 28829)"]
            pub const BAUD28800: Self = Self(0x0075_f000);
            #[doc = "31250 baud"]
            pub const BAUD31250: Self = Self(0x0080_0000);
            #[doc = "38400 baud (actual rate: 38462)"]
            pub const BAUD38400: Self = Self(0x009d_5000);
            #[doc = "56000 baud (actual rate: 55944)"]
            pub const BAUD56000: Self = Self(0x00e5_0000);
            #[doc = "57600 baud (actual rate: 57762)"]
            pub const BAUD57600: Self = Self(0x00eb_f000);
            #[doc = "76800 baud (actual rate: 76923)"]
            pub const BAUD76800: Self = Self(0x013a_9000);
            #[doc = "115200 baud (actual rate: 115942)"]
            pub const BAUD115200: Self = Self(0x01d7_e000);
            #[doc = "230400 baud (actual rate: 231884)"]
            pub const BAUD230400: Self = Self(0x03af_b000);
            #[doc = "250000 baud"]
            pub const BAUD250000: Self = Self(0x0400_0000);
            #[doc = "460800 baud (actual rate: 470588)"]
            pub const BAUD460800: Self = Self(0x075f_7000);
            #[doc = "921600 baud (actual rate: 941176)"]
            pub const BAUD921600: Self = Self(0x0ebe_d000);
            #[doc = "1Mega baud"]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum ConfigParity {
            #[doc = "Exclude parity bit"]
            EXCLUDED = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            #[doc = "Include parity bit"]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Enable {
            #[doc = "Disable UART"]
            DISABLED = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            #[doc = "Enable UART"]
            ENABLED = 0x04,
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Pin select for TXD signal"]
        #[inline(always)]
        pub const fn txd(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Pin select for CTS signal"]
        #[inline(always)]
        pub const fn cts(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Pin select for RXD signal"]
        #[inline(always)]
        pub const fn rxd(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in receive buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::RxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::RxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in transmit buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::TxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::TxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop UART receiver"]
        #[inline(always)]
        pub const fn tasks_stoprx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Start UART transmitter"]
        #[inline(always)]
        pub const fn tasks_starttx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Stop UART transmitter"]
        #[inline(always)]
        pub const fn tasks_stoptx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Flush RX FIFO into RX buffer"]
        #[inline(always)]
        pub const fn tasks_flushrx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
        }
        #[doc = "CTS is activated (set low). Clear To Send."]
        #[inline(always)]
        pub const fn events_cts(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "CTS is deactivated (set high). Not Clear To Send."]
        #[inline(always)]
        pub const fn events_ncts(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Data received in RXD (but potentially not yet transferred to Data RAM)"]
        #[inline(always)]
        pub const fn events_rxdrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "Receive buffer is filled up"]
        #[inline(always)]
        pub const fn events_endrx(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
        }
        #[doc = "Data sent from TXD"]
        #[inline(always)]
        pub const fn events_txdrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
        }
        #[doc = "Last TX byte transmitted"]
        #[inline(always)]
        pub const fn events_endtx(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
        }
        #[doc = "Error detected"]
        #[inline(always)]
        pub const fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
        }
        #[doc = "Receiver timeout"]
        #[inline(always)]
        pub const fn events_rxto(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
        }
        #[doc = "UART receiver has started"]
        #[inline(always)]
        pub const fn events_rxstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
        }
        #[doc = "UART transmitter has started"]
        #[inline(always)]
        pub const fn events_txstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
        }
        #[doc = "Transmitter stopped"]
        #[inline(always)]
        pub const fn events_txstopped(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Error source Note : this register is read / write one to clear."]
        #[inline(always)]
        pub const fn errorsrc(self) -> crate::common::Reg<regs::Errorsrc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0480usize) as _) }
        }
        #[doc = "Enable UART"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn psel(self) -> Psel {
            unsafe { Psel::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Baud rate. Accuracy depends on the HFCLK source selected."]
        #[inline(always)]
        pub const fn baudrate(self) -> crate::common::Reg<regs::Baudrate, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
        }
        #[doc = "RXD EasyDMA channel"]
        #[inline(always)]
        pub const fn rxd(self) -> Rxd {
            unsafe { Rxd::from_ptr(self.ptr.add(0x0534usize) as _) }
        }
        #[doc = "TXD EasyDMA channel"]
        #[inline(always)]
        pub const fn txd(self) -> Txd {
            unsafe { Txd::from_ptr(self.ptr.add(0x0544usize) as _) }
        }
        #[doc = "Configuration of parity and hardware flow control"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x056cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Baud rate. Accuracy depends on the HFCLK source selected."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Baudrate(pub u32);
        impl Baudrate {
            #[doc = "Baud rate"]
            #[inline(always)]
            pub const fn baudrate(&self) -> super::vals::Baudrate {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Baudrate::from_bits(val as u32)
            }
            #[doc = "Baud rate"]
            #[inline(always)]
            pub fn set_baudrate(&mut self, val: super::vals::Baudrate) {
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
        #[doc = "Configuration of parity and hardware flow control"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Hardware flow control"]
            #[inline(always)]
            pub const fn hwfc(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Hardware flow control"]
            #[inline(always)]
            pub fn set_hwfc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Parity"]
            #[inline(always)]
            pub const fn parity(&self) -> super::vals::ConfigParity {
                let val = (self.0 >> 1usize) & 0x07;
                super::vals::ConfigParity::from_bits(val as u8)
            }
            #[doc = "Parity"]
            #[inline(always)]
            pub fn set_parity(&mut self, val: super::vals::ConfigParity) {
                self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
            }
            #[doc = "Stop bits"]
            #[inline(always)]
            pub const fn stop(&self) -> super::vals::Stop {
                let val = (self.0 >> 4usize) & 0x01;
                super::vals::Stop::from_bits(val as u8)
            }
            #[doc = "Stop bits"]
            #[inline(always)]
            pub fn set_stop(&mut self, val: super::vals::Stop) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        #[doc = "Enable UART"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable UARTE"]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable UARTE"]
            #[inline(always)]
            pub fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        #[doc = "Error source Note : this register is read / write one to clear."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Errorsrc(pub u32);
        impl Errorsrc {
            #[doc = "Overrun error"]
            #[inline(always)]
            pub const fn overrun(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun error"]
            #[inline(always)]
            pub fn set_overrun(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Parity error"]
            #[inline(always)]
            pub const fn parity(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Parity error"]
            #[inline(always)]
            pub fn set_parity(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Framing error occurred"]
            #[inline(always)]
            pub const fn framing(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Framing error occurred"]
            #[inline(always)]
            pub fn set_framing(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Break condition"]
            #[inline(always)]
            pub const fn break_(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Break condition"]
            #[inline(always)]
            pub fn set_break_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Errorsrc {
            #[inline(always)]
            fn default() -> Errorsrc {
                Errorsrc(0)
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Enable or disable interrupt for event CTS"]
            #[inline(always)]
            pub const fn cts(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event CTS"]
            #[inline(always)]
            pub fn set_cts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable interrupt for event NCTS"]
            #[inline(always)]
            pub const fn ncts(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event NCTS"]
            #[inline(always)]
            pub fn set_ncts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for event RXDRDY"]
            #[inline(always)]
            pub const fn rxdrdy(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RXDRDY"]
            #[inline(always)]
            pub fn set_rxdrdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable or disable interrupt for event ENDRX"]
            #[inline(always)]
            pub const fn endrx(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event ENDRX"]
            #[inline(always)]
            pub fn set_endrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable or disable interrupt for event TXDRDY"]
            #[inline(always)]
            pub const fn txdrdy(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event TXDRDY"]
            #[inline(always)]
            pub fn set_txdrdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Enable or disable interrupt for event ENDTX"]
            #[inline(always)]
            pub const fn endtx(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event ENDTX"]
            #[inline(always)]
            pub fn set_endtx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Enable or disable interrupt for event ERROR"]
            #[inline(always)]
            pub const fn error(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event ERROR"]
            #[inline(always)]
            pub fn set_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Enable or disable interrupt for event RXTO"]
            #[inline(always)]
            pub const fn rxto(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RXTO"]
            #[inline(always)]
            pub fn set_rxto(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Enable or disable interrupt for event RXSTARTED"]
            #[inline(always)]
            pub const fn rxstarted(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RXSTARTED"]
            #[inline(always)]
            pub fn set_rxstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "Enable or disable interrupt for event TXSTARTED"]
            #[inline(always)]
            pub const fn txstarted(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event TXSTARTED"]
            #[inline(always)]
            pub fn set_txstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "Enable or disable interrupt for event TXSTOPPED"]
            #[inline(always)]
            pub const fn txstopped(&self) -> bool {
                let val = (self.0 >> 22usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event TXSTOPPED"]
            #[inline(always)]
            pub fn set_txstopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdAmount(pub u32);
        impl RxdAmount {
            #[doc = "Number of bytes transferred in the last transaction"]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
        }
        impl Default for RxdAmount {
            #[inline(always)]
            fn default() -> RxdAmount {
                RxdAmount(0)
            }
        }
        #[doc = "Maximum number of bytes in receive buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdMaxcnt(pub u32);
        impl RxdMaxcnt {
            #[doc = "Maximum number of bytes in receive buffer"]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Maximum number of bytes in receive buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
        }
        impl Default for RxdMaxcnt {
            #[inline(always)]
            fn default() -> RxdMaxcnt {
                RxdMaxcnt(0)
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event ENDRX and task STARTRX"]
            #[inline(always)]
            pub const fn endrx_startrx(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event ENDRX and task STARTRX"]
            #[inline(always)]
            pub fn set_endrx_startrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Shortcut between event ENDRX and task STOPRX"]
            #[inline(always)]
            pub const fn endrx_stoprx(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event ENDRX and task STOPRX"]
            #[inline(always)]
            pub fn set_endrx_stoprx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdAmount(pub u32);
        impl TxdAmount {
            #[doc = "Number of bytes transferred in the last transaction"]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
        }
        impl Default for TxdAmount {
            #[inline(always)]
            fn default() -> TxdAmount {
                TxdAmount(0)
            }
        }
        #[doc = "Maximum number of bytes in transmit buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdMaxcnt(pub u32);
        impl TxdMaxcnt {
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
        }
        impl Default for TxdMaxcnt {
            #[inline(always)]
            fn default() -> TxdMaxcnt {
                TxdMaxcnt(0)
            }
        }
    }
    pub mod vals {
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Baudrate(pub u32);
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
            #[doc = "1Mega baud"]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
    #[doc = "User information configuration registers"]
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
        #[doc = "Description collection: Reserved for Nordic firmware design"]
        #[inline(always)]
        pub const fn nrffw(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 13usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Reserved for Nordic hardware design"]
        #[inline(always)]
        pub const fn nrfhw(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 12usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Reserved for customer"]
        #[inline(always)]
        pub const fn customer(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 32usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Reserved for Nordic MDK"]
        #[inline(always)]
        pub const fn nrfmdk(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Mapping of the nRESET function (see POWER chapter for details)"]
        #[inline(always)]
        pub const fn pselreset(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            assert!(n < 2usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize + n * 4usize) as _) }
        }
        #[doc = "Access port protection"]
        #[inline(always)]
        pub const fn approtect(self) -> crate::common::Reg<regs::Approtect, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Access port protection"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Approtect(pub u32);
        impl Approtect {
            #[doc = "Enable or disable access port protection."]
            #[inline(always)]
            pub const fn pall(&self) -> super::vals::Pall {
                let val = (self.0 >> 0usize) & 0xff;
                super::vals::Pall::from_bits(val as u8)
            }
            #[doc = "Enable or disable access port protection."]
            #[inline(always)]
            pub fn set_pall(&mut self, val: super::vals::Pall) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Approtect {
            #[inline(always)]
            fn default() -> Approtect {
                Approtect(0)
            }
        }
    }
    pub mod vals {
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pall(pub u8);
        impl Pall {
            #[doc = "Enable"]
            pub const ENABLED: Self = Self(0x0);
            #[doc = "Hardware disable of access port protection for devices where access port protection is controlled by hardware and software"]
            pub const HW_DISABLED: Self = Self(0x5a);
            #[doc = "Hardware disable of access port protection for devices where access port protection is controlled by hardware"]
            pub const DISABLED: Self = Self(0xff);
        }
        impl Pall {
            pub const fn from_bits(val: u8) -> Pall {
                Self(val & 0xff)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl From<u8> for Pall {
            #[inline(always)]
            fn from(val: u8) -> Pall {
                Pall::from_bits(val)
            }
        }
        impl From<Pall> for u8 {
            #[inline(always)]
            fn from(val: Pall) -> u8 {
                Pall::to_bits(val)
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
        #[doc = "Start the watchdog"]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Watchdog timeout"]
        #[inline(always)]
        pub const fn events_timeout(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Run status"]
        #[inline(always)]
        pub const fn runstatus(self) -> crate::common::Reg<regs::Runstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "Request status"]
        #[inline(always)]
        pub const fn reqstatus(self) -> crate::common::Reg<regs::Reqstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0404usize) as _) }
        }
        #[doc = "Counter reload value"]
        #[inline(always)]
        pub const fn crv(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Enable register for reload request registers"]
        #[inline(always)]
        pub const fn rren(self) -> crate::common::Reg<regs::Rren, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Configuration register"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Description collection: Reload request n"]
        #[inline(always)]
        pub const fn rr(self, n: usize) -> crate::common::Reg<regs::Rr, crate::common::W> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize + n * 4usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Configure the watchdog to either be paused, or kept running, while the CPU is sleeping"]
            #[inline(always)]
            pub const fn sleep(&self) -> super::vals::Sleep {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Sleep::from_bits(val as u8)
            }
            #[doc = "Configure the watchdog to either be paused, or kept running, while the CPU is sleeping"]
            #[inline(always)]
            pub fn set_sleep(&mut self, val: super::vals::Sleep) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Configure the watchdog to either be paused, or kept running, while the CPU is halted by the debugger"]
            #[inline(always)]
            pub const fn halt(&self) -> super::vals::Halt {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Halt::from_bits(val as u8)
            }
            #[doc = "Configure the watchdog to either be paused, or kept running, while the CPU is halted by the debugger"]
            #[inline(always)]
            pub fn set_halt(&mut self, val: super::vals::Halt) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Write '1' to disable interrupt for event TIMEOUT"]
            #[inline(always)]
            pub const fn timeout(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event TIMEOUT"]
            #[inline(always)]
            pub fn set_timeout(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
        #[doc = "Request status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Reqstatus(pub u32);
        impl Reqstatus {
            #[doc = "Request status for RR\\[0\\] register"]
            #[inline(always)]
            pub const fn rr(&self, n: usize) -> bool {
                assert!(n < 8usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Request status for RR\\[0\\] register"]
            #[inline(always)]
            pub fn set_rr(&mut self, n: usize, val: bool) {
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
        #[doc = "Description collection: Reload request n"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rr(pub u32);
        impl Rr {
            #[doc = "Reload request register"]
            #[inline(always)]
            pub const fn rr(&self) -> super::vals::Rr {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Rr::from_bits(val as u32)
            }
            #[doc = "Reload request register"]
            #[inline(always)]
            pub fn set_rr(&mut self, val: super::vals::Rr) {
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
        #[doc = "Enable register for reload request registers"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rren(pub u32);
        impl Rren {
            #[doc = "Enable or disable RR\\[0\\] register"]
            #[inline(always)]
            pub const fn rr(&self, n: usize) -> bool {
                assert!(n < 8usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable RR\\[0\\] register"]
            #[inline(always)]
            pub fn set_rr(&mut self, n: usize, val: bool) {
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
        #[doc = "Run status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Runstatus(pub u32);
        impl Runstatus {
            #[doc = "Indicates whether or not the watchdog is running"]
            #[inline(always)]
            pub const fn runstatus(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Indicates whether or not the watchdog is running"]
            #[inline(always)]
            pub fn set_runstatus(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Runstatus {
            #[inline(always)]
            fn default() -> Runstatus {
                Runstatus(0)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Halt {
            #[doc = "Pause watchdog while the CPU is halted by the debugger"]
            PAUSE = 0x0,
            #[doc = "Keep the watchdog running while the CPU is halted by the debugger"]
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
        pub struct Rr(pub u32);
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Sleep {
            #[doc = "Pause watchdog while the CPU is sleeping"]
            PAUSE = 0x0,
            #[doc = "Keep the watchdog running while the CPU is sleeping"]
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
    }
}
