#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (26983da 2025-01-02))"]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - CLOCK_POWER"]
    CLOCK_POWER = 0,
    #[doc = "1 - RADIO"]
    RADIO = 1,
    #[doc = "2 - UART0"]
    UART0 = 2,
    #[doc = "3 - TWISPI0"]
    TWISPI0 = 3,
    #[doc = "4 - TWISPI1"]
    TWISPI1 = 4,
    #[doc = "6 - GPIOTE"]
    GPIOTE = 6,
    #[doc = "7 - ADC"]
    ADC = 7,
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
    #[doc = "15 - AAR_CCM"]
    AAR_CCM = 15,
    #[doc = "16 - WDT"]
    WDT = 16,
    #[doc = "17 - RTC1"]
    RTC1 = 17,
    #[doc = "18 - QDEC"]
    QDEC = 18,
    #[doc = "19 - LPCOMP"]
    LPCOMP = 19,
    #[doc = "20 - SWI0"]
    SWI0 = 20,
    #[doc = "21 - SWI1"]
    SWI1 = 21,
    #[doc = "22 - SWI2"]
    SWI2 = 22,
    #[doc = "23 - SWI3"]
    SWI3 = 23,
    #[doc = "24 - SWI4"]
    SWI4 = 24,
    #[doc = "25 - SWI5"]
    SWI5 = 25,
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
        fn UART0();
        fn TWISPI0();
        fn TWISPI1();
        fn GPIOTE();
        fn ADC();
        fn TIMER0();
        fn TIMER1();
        fn TIMER2();
        fn RTC0();
        fn TEMP();
        fn RNG();
        fn ECB();
        fn AAR_CCM();
        fn WDT();
        fn RTC1();
        fn QDEC();
        fn LPCOMP();
        fn SWI0();
        fn SWI1();
        fn SWI2();
        fn SWI3();
        fn SWI4();
        fn SWI5();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 26] = [
        Vector {
            _handler: CLOCK_POWER,
        },
        Vector { _handler: RADIO },
        Vector { _handler: UART0 },
        Vector { _handler: TWISPI0 },
        Vector { _handler: TWISPI1 },
        Vector { _reserved: 0 },
        Vector { _handler: GPIOTE },
        Vector { _handler: ADC },
        Vector { _handler: TIMER0 },
        Vector { _handler: TIMER1 },
        Vector { _handler: TIMER2 },
        Vector { _handler: RTC0 },
        Vector { _handler: TEMP },
        Vector { _handler: RNG },
        Vector { _handler: ECB },
        Vector { _handler: AAR_CCM },
        Vector { _handler: WDT },
        Vector { _handler: RTC1 },
        Vector { _handler: QDEC },
        Vector { _handler: LPCOMP },
        Vector { _handler: SWI0 },
        Vector { _handler: SWI1 },
        Vector { _handler: SWI2 },
        Vector { _handler: SWI3 },
        Vector { _handler: SWI4 },
        Vector { _handler: SWI5 },
    ];
}
#[doc = "Factory Information Configuration."]
pub const FICR: ficr::Ficr = unsafe { ficr::Ficr::from_ptr(0x1000_0000usize as _) };
#[doc = "User Information Configuration."]
pub const UICR: uicr::Uicr = unsafe { uicr::Uicr::from_ptr(0x1000_1000usize as _) };
#[doc = "Power Control."]
pub const POWER: power::Power = unsafe { power::Power::from_ptr(0x4000_0000usize as _) };
#[doc = "Clock control."]
pub const CLOCK: clock::Clock = unsafe { clock::Clock::from_ptr(0x4000_0000usize as _) };
#[doc = "Memory Protection Unit."]
pub const MPU: mpu::Mpu = unsafe { mpu::Mpu::from_ptr(0x4000_0000usize as _) };
#[doc = "The radio."]
pub const RADIO: radio::Radio = unsafe { radio::Radio::from_ptr(0x4000_1000usize as _) };
#[doc = "Universal Asynchronous Receiver/Transmitter."]
pub const UART0: uart::Uart = unsafe { uart::Uart::from_ptr(0x4000_2000usize as _) };
#[doc = "SPI master 0."]
pub const SPI0: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3000usize as _) };
#[doc = "Two-wire interface master 0."]
pub const TWI0: twi::Twi = unsafe { twi::Twi::from_ptr(0x4000_3000usize as _) };
#[doc = "SPI master 1."]
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_4000usize as _) };
#[doc = "Two-wire interface master 1."]
pub const TWI1: twi::Twi = unsafe { twi::Twi::from_ptr(0x4000_4000usize as _) };
#[doc = "SPI slave 1."]
pub const SPIS1: spis::Spis = unsafe { spis::Spis::from_ptr(0x4000_4000usize as _) };
#[doc = "GPIO tasks and events."]
pub const GPIOTE: gpiote::Gpiote = unsafe { gpiote::Gpiote::from_ptr(0x4000_6000usize as _) };
#[doc = "Analog to digital converter."]
pub const ADC: adc::Adc = unsafe { adc::Adc::from_ptr(0x4000_7000usize as _) };
#[doc = "Timer 0."]
pub const TIMER0: timer::Timer = unsafe { timer::Timer::from_ptr(0x4000_8000usize as _) };
#[doc = "Timer 1."]
pub const TIMER1: timer::Timer = unsafe { timer::Timer::from_ptr(0x4000_9000usize as _) };
#[doc = "Timer 2."]
pub const TIMER2: timer::Timer = unsafe { timer::Timer::from_ptr(0x4000_a000usize as _) };
#[doc = "Real time counter 0."]
pub const RTC0: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4000_b000usize as _) };
#[doc = "Temperature Sensor."]
pub const TEMP: temp::Temp = unsafe { temp::Temp::from_ptr(0x4000_c000usize as _) };
#[doc = "Random Number Generator."]
pub const RNG: rng::Rng = unsafe { rng::Rng::from_ptr(0x4000_d000usize as _) };
#[doc = "AES ECB Mode Encryption."]
pub const ECB: ecb::Ecb = unsafe { ecb::Ecb::from_ptr(0x4000_e000usize as _) };
#[doc = "Accelerated Address Resolver."]
pub const AAR: aar::Aar = unsafe { aar::Aar::from_ptr(0x4000_f000usize as _) };
#[doc = "AES CCM Mode Encryption."]
pub const CCM: ccm::Ccm = unsafe { ccm::Ccm::from_ptr(0x4000_f000usize as _) };
#[doc = "Watchdog Timer."]
pub const WDT: wdt::Wdt = unsafe { wdt::Wdt::from_ptr(0x4001_0000usize as _) };
#[doc = "Real time counter 1."]
pub const RTC1: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4001_1000usize as _) };
#[doc = "Rotary decoder."]
pub const QDEC: qdec::Qdec = unsafe { qdec::Qdec::from_ptr(0x4001_2000usize as _) };
#[doc = "Low power comparator."]
pub const LPCOMP: lpcomp::Lpcomp = unsafe { lpcomp::Lpcomp::from_ptr(0x4001_3000usize as _) };
#[doc = "SW Interrupts."]
pub const SWI: swi::Swi = unsafe { swi::Swi::from_ptr(0x4001_4000usize as _) };
#[doc = "Non Volatile Memory Controller."]
pub const NVMC: nvmc::Nvmc = unsafe { nvmc::Nvmc::from_ptr(0x4001_e000usize as _) };
#[doc = "PPI controller."]
pub const PPI: ppi::Ppi = unsafe { ppi::Ppi::from_ptr(0x4001_f000usize as _) };
#[doc = "General purpose input and output."]
pub const GPIO: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5000_0000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub mod aar {
    #[doc = "Accelerated Address Resolver."]
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
        #[doc = "Start resolving addresses based on IRKs specified in the IRK data structure."]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop resolving addresses."]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Address resolution procedure completed."]
        #[inline(always)]
        pub const fn events_end(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Address resolved."]
        #[inline(always)]
        pub const fn events_resolved(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Address not resolved."]
        #[inline(always)]
        pub const fn events_notresolved(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "Interrupt enable set register."]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Interrupt enable clear register."]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Resolution status."]
        #[inline(always)]
        pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "Enable AAR."]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Number of Identity root Keys in the IRK data structure."]
        #[inline(always)]
        pub const fn nirk(self) -> crate::common::Reg<regs::Nirk, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Pointer to the IRK data structure."]
        #[inline(always)]
        pub const fn irkptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Pointer to the resolvable address (6 bytes)."]
        #[inline(always)]
        pub const fn addrptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "Pointer to a scratch data area used for temporary storage during resolution. A minimum of 3 bytes must be reserved."]
        #[inline(always)]
        pub const fn scratchptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
        }
        #[doc = "Peripheral power control."]
        #[inline(always)]
        pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Enable AAR."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable AAR."]
            #[must_use]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable AAR."]
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
        #[doc = "Interrupt enable clear register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Disable interrupt on ENDKSGEN event."]
            #[must_use]
            #[inline(always)]
            pub const fn end(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on ENDKSGEN event."]
            #[inline(always)]
            pub const fn set_end(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Disable interrupt on RESOLVED event."]
            #[must_use]
            #[inline(always)]
            pub const fn resolved(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on RESOLVED event."]
            #[inline(always)]
            pub const fn set_resolved(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Disable interrupt on NOTRESOLVED event."]
            #[must_use]
            #[inline(always)]
            pub const fn notresolved(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on NOTRESOLVED event."]
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
        #[doc = "Number of Identity root Keys in the IRK data structure."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Nirk(pub u32);
        impl Nirk {
            #[doc = "Number of Identity root Keys in the IRK data structure."]
            #[must_use]
            #[inline(always)]
            pub const fn nirk(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x1f;
                val as u8
            }
            #[doc = "Number of Identity root Keys in the IRK data structure."]
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
        #[doc = "Peripheral power control."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Power(pub u32);
        impl Power {
            #[doc = "Peripheral power control."]
            #[must_use]
            #[inline(always)]
            pub const fn power(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Peripheral power control."]
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
        #[doc = "Resolution status."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Status(pub u32);
        impl Status {
            #[doc = "The IRK used last time an address was resolved."]
            #[must_use]
            #[inline(always)]
            pub const fn status(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "The IRK used last time an address was resolved."]
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
            #[doc = "Disabled AAR."]
            DISABLED = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            #[doc = "Enable AAR."]
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
pub mod adc {
    #[doc = "Analog to digital converter."]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Adc {
        ptr: *mut u8,
    }
    unsafe impl Send for Adc {}
    unsafe impl Sync for Adc {}
    impl Adc {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start an ADC conversion."]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop ADC."]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "ADC conversion complete."]
        #[inline(always)]
        pub const fn events_end(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Interrupt enable set register."]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Interrupt enable clear register."]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "ADC busy register."]
        #[inline(always)]
        pub const fn busy(self) -> crate::common::Reg<regs::Busy, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "ADC enable."]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "ADC configuration register."]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Result of ADC conversion."]
        #[inline(always)]
        pub const fn result(self) -> crate::common::Reg<regs::Result, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Peripheral power control."]
        #[inline(always)]
        pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "ADC busy register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Busy(pub u32);
        impl Busy {
            #[doc = "ADC busy register."]
            #[must_use]
            #[inline(always)]
            pub const fn busy(&self) -> super::vals::Busy {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Busy::from_bits(val as u8)
            }
            #[doc = "ADC busy register."]
            #[inline(always)]
            pub const fn set_busy(&mut self, val: super::vals::Busy) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Busy {
            #[inline(always)]
            fn default() -> Busy {
                Busy(0)
            }
        }
        impl core::fmt::Debug for Busy {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Busy").field("busy", &self.busy()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Busy {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Busy {{ busy: {:?} }}", self.busy())
            }
        }
        #[doc = "ADC configuration register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "ADC resolution."]
            #[must_use]
            #[inline(always)]
            pub const fn res(&self) -> super::vals::Res {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Res::from_bits(val as u8)
            }
            #[doc = "ADC resolution."]
            #[inline(always)]
            pub const fn set_res(&mut self, val: super::vals::Res) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
            #[doc = "ADC input selection."]
            #[must_use]
            #[inline(always)]
            pub const fn inpsel(&self) -> super::vals::Inpsel {
                let val = (self.0 >> 2usize) & 0x07;
                super::vals::Inpsel::from_bits(val as u8)
            }
            #[doc = "ADC input selection."]
            #[inline(always)]
            pub const fn set_inpsel(&mut self, val: super::vals::Inpsel) {
                self.0 = (self.0 & !(0x07 << 2usize)) | (((val.to_bits() as u32) & 0x07) << 2usize);
            }
            #[doc = "ADC reference selection."]
            #[must_use]
            #[inline(always)]
            pub const fn refsel(&self) -> super::vals::Refsel {
                let val = (self.0 >> 5usize) & 0x03;
                super::vals::Refsel::from_bits(val as u8)
            }
            #[doc = "ADC reference selection."]
            #[inline(always)]
            pub const fn set_refsel(&mut self, val: super::vals::Refsel) {
                self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
            }
            #[doc = "ADC analog pin selection."]
            #[must_use]
            #[inline(always)]
            pub const fn psel(&self) -> super::vals::Psel {
                let val = (self.0 >> 8usize) & 0xff;
                super::vals::Psel::from_bits(val as u8)
            }
            #[doc = "ADC analog pin selection."]
            #[inline(always)]
            pub const fn set_psel(&mut self, val: super::vals::Psel) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
            }
            #[doc = "ADC external reference pin selection."]
            #[must_use]
            #[inline(always)]
            pub const fn extrefsel(&self) -> super::vals::Extrefsel {
                let val = (self.0 >> 16usize) & 0x03;
                super::vals::Extrefsel::from_bits(val as u8)
            }
            #[doc = "ADC external reference pin selection."]
            #[inline(always)]
            pub const fn set_extrefsel(&mut self, val: super::vals::Extrefsel) {
                self.0 =
                    (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
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
                    .field("res", &self.res())
                    .field("inpsel", &self.inpsel())
                    .field("refsel", &self.refsel())
                    .field("psel", &self.psel())
                    .field("extrefsel", &self.extrefsel())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Config {{ res: {:?}, inpsel: {:?}, refsel: {:?}, psel: {:?}, extrefsel: {:?} }}" , self . res () , self . inpsel () , self . refsel () , self . psel () , self . extrefsel ())
            }
        }
        #[doc = "ADC enable."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "ADC enable."]
            #[must_use]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "ADC enable."]
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
        #[doc = "Interrupt enable clear register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Disable interrupt on END event."]
            #[must_use]
            #[inline(always)]
            pub const fn end(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on END event."]
            #[inline(always)]
            pub const fn set_end(&mut self, val: bool) {
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
                f.debug_struct("Int").field("end", &self.end()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Int {{ end: {=bool:?} }}", self.end())
            }
        }
        #[doc = "Peripheral power control."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Power(pub u32);
        impl Power {
            #[doc = "Peripheral power control."]
            #[must_use]
            #[inline(always)]
            pub const fn power(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Peripheral power control."]
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
        #[doc = "Result of ADC conversion."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Result(pub u32);
        impl Result {
            #[doc = "Result of ADC conversion."]
            #[must_use]
            #[inline(always)]
            pub const fn result(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Result of ADC conversion."]
            #[inline(always)]
            pub const fn set_result(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
        }
        impl Default for Result {
            #[inline(always)]
            fn default() -> Result {
                Result(0)
            }
        }
        impl core::fmt::Debug for Result {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Result")
                    .field("result", &self.result())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Result {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Result {{ result: {=u16:?} }}", self.result())
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Busy {
            #[doc = "No ongoing ADC conversion is taking place. ADC is ready."]
            READY = 0x0,
            #[doc = "An ADC conversion is taking place. ADC is busy."]
            BUSY = 0x01,
        }
        impl Busy {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Busy {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Busy {
            #[inline(always)]
            fn from(val: u8) -> Busy {
                Busy::from_bits(val)
            }
        }
        impl From<Busy> for u8 {
            #[inline(always)]
            fn from(val: Busy) -> u8 {
                Busy::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Enable {
            #[doc = "ADC is disabled."]
            DISABLED = 0x0,
            #[doc = "ADC is enabled. If an analog input pin is selected as source of the conversion, the selected pin is configured as an analog input."]
            ENABLED = 0x01,
            _RESERVED_2 = 0x02,
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
        pub enum Extrefsel {
            #[doc = "Analog external reference inputs disabled."]
            NONE = 0x0,
            #[doc = "Use analog reference 0 as reference."]
            ANALOG_REFERENCE0 = 0x01,
            #[doc = "Use analog reference 1 as reference."]
            ANALOG_REFERENCE1 = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Extrefsel {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Extrefsel {
                unsafe { core::mem::transmute(val & 0x03) }
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Inpsel {
            #[doc = "Analog input specified by PSEL with no prescaling used as input for the conversion."]
            ANALOG_INPUT_NO_PRESCALING = 0x0,
            #[doc = "Analog input specified by PSEL with 2/3 prescaling used as input for the conversion."]
            ANALOG_INPUT_TWO_THIRDS_PRESCALING = 0x01,
            #[doc = "Analog input specified by PSEL with 1/3 prescaling used as input for the conversion."]
            ANALOG_INPUT_ONE_THIRD_PRESCALING = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            #[doc = "Supply voltage with 2/3 prescaling used as input for the conversion."]
            SUPPLY_TWO_THIRDS_PRESCALING = 0x05,
            #[doc = "Supply voltage with 1/3 prescaling used as input for the conversion."]
            SUPPLY_ONE_THIRD_PRESCALING = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl Inpsel {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Inpsel {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Inpsel {
            #[inline(always)]
            fn from(val: u8) -> Inpsel {
                Inpsel::from_bits(val)
            }
        }
        impl From<Inpsel> for u8 {
            #[inline(always)]
            fn from(val: Inpsel) -> u8 {
                Inpsel::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Psel(u8);
        impl Psel {
            #[doc = "Analog input pins disabled."]
            pub const DISABLED: Self = Self(0x0);
            #[doc = "Use analog input 0 as analog input."]
            pub const ANALOG_INPUT0: Self = Self(0x01);
            #[doc = "Use analog input 1 as analog input."]
            pub const ANALOG_INPUT1: Self = Self(0x02);
            #[doc = "Use analog input 2 as analog input."]
            pub const ANALOG_INPUT2: Self = Self(0x04);
            #[doc = "Use analog input 3 as analog input."]
            pub const ANALOG_INPUT3: Self = Self(0x08);
            #[doc = "Use analog input 4 as analog input."]
            pub const ANALOG_INPUT4: Self = Self(0x10);
            #[doc = "Use analog input 5 as analog input."]
            pub const ANALOG_INPUT5: Self = Self(0x20);
            #[doc = "Use analog input 6 as analog input."]
            pub const ANALOG_INPUT6: Self = Self(0x40);
            #[doc = "Use analog input 7 as analog input."]
            pub const ANALOG_INPUT7: Self = Self(0x80);
        }
        impl Psel {
            pub const fn from_bits(val: u8) -> Psel {
                Self(val & 0xff)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl core::fmt::Debug for Psel {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0 => f.write_str("DISABLED"),
                    0x01 => f.write_str("ANALOG_INPUT0"),
                    0x02 => f.write_str("ANALOG_INPUT1"),
                    0x04 => f.write_str("ANALOG_INPUT2"),
                    0x08 => f.write_str("ANALOG_INPUT3"),
                    0x10 => f.write_str("ANALOG_INPUT4"),
                    0x20 => f.write_str("ANALOG_INPUT5"),
                    0x40 => f.write_str("ANALOG_INPUT6"),
                    0x80 => f.write_str("ANALOG_INPUT7"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Psel {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0 => defmt::write!(f, "DISABLED"),
                    0x01 => defmt::write!(f, "ANALOG_INPUT0"),
                    0x02 => defmt::write!(f, "ANALOG_INPUT1"),
                    0x04 => defmt::write!(f, "ANALOG_INPUT2"),
                    0x08 => defmt::write!(f, "ANALOG_INPUT3"),
                    0x10 => defmt::write!(f, "ANALOG_INPUT4"),
                    0x20 => defmt::write!(f, "ANALOG_INPUT5"),
                    0x40 => defmt::write!(f, "ANALOG_INPUT6"),
                    0x80 => defmt::write!(f, "ANALOG_INPUT7"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Refsel {
            #[doc = "Use internal 1.2V bandgap voltage as reference for conversion."]
            VBG = 0x0,
            #[doc = "Use external source configured by EXTREFSEL as reference for conversion."]
            EXTERNAL = 0x01,
            #[doc = "Use supply voltage with 1/2 prescaling as reference for conversion. Only usable when supply voltage is between 1.7V and 2.6V."]
            SUPPLY_ONE_HALF_PRESCALING = 0x02,
            #[doc = "Use supply voltage with 1/3 prescaling as reference for conversion. Only usable when supply voltage is between 2.5V and 3.6V."]
            SUPPLY_ONE_THIRD_PRESCALING = 0x03,
        }
        impl Refsel {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Refsel {
                unsafe { core::mem::transmute(val & 0x03) }
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Res {
            #[doc = "8bit ADC resolution."]
            _8BIT = 0x0,
            #[doc = "9bit ADC resolution."]
            _9BIT = 0x01,
            #[doc = "10bit ADC resolution."]
            _10BIT = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Res {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Res {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Res {
            #[inline(always)]
            fn from(val: u8) -> Res {
                Res::from_bits(val)
            }
        }
        impl From<Res> for u8 {
            #[inline(always)]
            fn from(val: Res) -> u8 {
                Res::to_bits(val)
            }
        }
    }
}
pub mod ccm {
    #[doc = "AES CCM Mode Encryption."]
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
        #[doc = "Start encrypt/decrypt. This operation will stop by itself when completed."]
        #[inline(always)]
        pub const fn tasks_crypt(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Stop encrypt/decrypt."]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Keystream generation completed."]
        #[inline(always)]
        pub const fn events_endksgen(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Encrypt/decrypt completed."]
        #[inline(always)]
        pub const fn events_endcrypt(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Error happened."]
        #[inline(always)]
        pub const fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "Shortcuts for the CCM."]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Interrupt enable set register."]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Interrupt enable clear register."]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "CCM RX MIC check result."]
        #[inline(always)]
        pub const fn micstatus(self) -> crate::common::Reg<regs::Micstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "CCM enable."]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Operation mode."]
        #[inline(always)]
        pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Pointer to a data structure holding AES key and NONCE vector."]
        #[inline(always)]
        pub const fn cnfptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Pointer to the input packet."]
        #[inline(always)]
        pub const fn inptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Pointer to the output packet."]
        #[inline(always)]
        pub const fn outptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "Pointer to a scratch data area used for temporary storage during resolution. A minimum of 43 bytes must be reserved."]
        #[inline(always)]
        pub const fn scratchptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
        }
        #[doc = "Peripheral power control."]
        #[inline(always)]
        pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "CCM enable."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "CCM enable."]
            #[must_use]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "CCM enable."]
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
        #[doc = "Interrupt enable clear register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Disable interrupt on ENDKSGEN event."]
            #[must_use]
            #[inline(always)]
            pub const fn endksgen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on ENDKSGEN event."]
            #[inline(always)]
            pub const fn set_endksgen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Disable interrupt on ENDCRYPT event."]
            #[must_use]
            #[inline(always)]
            pub const fn endcrypt(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on ENDCRYPT event."]
            #[inline(always)]
            pub const fn set_endcrypt(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Disable interrupt on ERROR event."]
            #[must_use]
            #[inline(always)]
            pub const fn error(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on ERROR event."]
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
        #[doc = "CCM RX MIC check result."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Micstatus(pub u32);
        impl Micstatus {
            #[doc = "Result of the MIC check performed during the previous CCM RX STARTCRYPT"]
            #[must_use]
            #[inline(always)]
            pub const fn micstatus(&self) -> super::vals::Micstatus {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Micstatus::from_bits(val as u8)
            }
            #[doc = "Result of the MIC check performed during the previous CCM RX STARTCRYPT"]
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
        #[doc = "Operation mode."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Mode(pub u32);
        impl Mode {
            #[doc = "CCM mode operation."]
            #[must_use]
            #[inline(always)]
            pub const fn mode(&self) -> super::vals::Mode {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Mode::from_bits(val as u8)
            }
            #[doc = "CCM mode operation."]
            #[inline(always)]
            pub const fn set_mode(&mut self, val: super::vals::Mode) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
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
        #[doc = "Peripheral power control."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Power(pub u32);
        impl Power {
            #[doc = "Peripheral power control."]
            #[must_use]
            #[inline(always)]
            pub const fn power(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Peripheral power control."]
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
        #[doc = "Shortcuts for the CCM."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between ENDKSGEN event and CRYPT task."]
            #[must_use]
            #[inline(always)]
            pub const fn endksgen_crypt(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between ENDKSGEN event and CRYPT task."]
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
        pub enum Enable {
            #[doc = "CCM is disabled."]
            DISABLED = 0x0,
            _RESERVED_1 = 0x01,
            #[doc = "CCM is enabled."]
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
        pub enum Micstatus {
            #[doc = "MIC check failed."]
            CHECK_FAILED = 0x0,
            #[doc = "MIC check passed."]
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
            #[doc = "CCM mode TX"]
            ENCRYPTION = 0x0,
            #[doc = "CCM mode TX"]
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
    }
}
pub mod clock {
    #[doc = "Clock control."]
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
        #[doc = "Start HFCLK clock source."]
        #[inline(always)]
        pub const fn tasks_hfclkstart(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop HFCLK clock source."]
        #[inline(always)]
        pub const fn tasks_hfclkstop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Start LFCLK clock source."]
        #[inline(always)]
        pub const fn tasks_lfclkstart(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Stop LFCLK clock source."]
        #[inline(always)]
        pub const fn tasks_lfclkstop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Start calibration of LFCLK RC oscillator."]
        #[inline(always)]
        pub const fn tasks_cal(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Start calibration timer."]
        #[inline(always)]
        pub const fn tasks_ctstart(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Stop calibration timer."]
        #[inline(always)]
        pub const fn tasks_ctstop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }
        #[doc = "HFCLK oscillator started."]
        #[inline(always)]
        pub const fn events_hfclkstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "LFCLK oscillator started."]
        #[inline(always)]
        pub const fn events_lfclkstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Calibration of LFCLK RC oscillator completed."]
        #[inline(always)]
        pub const fn events_done(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
        }
        #[doc = "Calibration timer timeout."]
        #[inline(always)]
        pub const fn events_ctto(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
        }
        #[doc = "Interrupt enable set register."]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Interrupt enable clear register."]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Task HFCLKSTART trigger status."]
        #[inline(always)]
        pub const fn hfclkrun(self) -> crate::common::Reg<regs::Hfclkrun, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0408usize) as _) }
        }
        #[doc = "High frequency clock status."]
        #[inline(always)]
        pub const fn hfclkstat(self) -> crate::common::Reg<regs::Hfclkstat, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x040cusize) as _) }
        }
        #[doc = "Task LFCLKSTART triggered status."]
        #[inline(always)]
        pub const fn lfclkrun(self) -> crate::common::Reg<regs::Lfclkrun, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0414usize) as _) }
        }
        #[doc = "Low frequency clock status."]
        #[inline(always)]
        pub const fn lfclkstat(self) -> crate::common::Reg<regs::Lfclkstat, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0418usize) as _) }
        }
        #[doc = "Clock source for the LFCLK clock, set when task LKCLKSTART is triggered."]
        #[inline(always)]
        pub const fn lfclksrccopy(
            self,
        ) -> crate::common::Reg<regs::Lfclksrccopy, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x041cusize) as _) }
        }
        #[doc = "Clock source for the LFCLK clock."]
        #[inline(always)]
        pub const fn lfclksrc(self) -> crate::common::Reg<regs::Lfclksrc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
        #[doc = "Calibration timer interval."]
        #[inline(always)]
        pub const fn ctiv(self) -> crate::common::Reg<regs::Ctiv, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0538usize) as _) }
        }
        #[doc = "Crystal frequency."]
        #[inline(always)]
        pub const fn xtalfreq(self) -> crate::common::Reg<regs::Xtalfreq, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0550usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Calibration timer interval."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctiv(pub u32);
        impl Ctiv {
            #[doc = "Calibration timer interval in 0.25s resolution."]
            #[must_use]
            #[inline(always)]
            pub const fn ctiv(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "Calibration timer interval in 0.25s resolution."]
            #[inline(always)]
            pub const fn set_ctiv(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
            }
        }
        impl Default for Ctiv {
            #[inline(always)]
            fn default() -> Ctiv {
                Ctiv(0)
            }
        }
        impl core::fmt::Debug for Ctiv {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctiv").field("ctiv", &self.ctiv()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctiv {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Ctiv {{ ctiv: {=u8:?} }}", self.ctiv())
            }
        }
        #[doc = "Task HFCLKSTART trigger status."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Hfclkrun(pub u32);
        impl Hfclkrun {
            #[doc = "Task HFCLKSTART trigger status."]
            #[must_use]
            #[inline(always)]
            pub const fn status(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Task HFCLKSTART trigger status."]
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
        #[doc = "High frequency clock status."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Hfclkstat(pub u32);
        impl Hfclkstat {
            #[doc = "Active clock source for the HF clock."]
            #[must_use]
            #[inline(always)]
            pub const fn src(&self) -> super::vals::HfclkstatSrc {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::HfclkstatSrc::from_bits(val as u8)
            }
            #[doc = "Active clock source for the HF clock."]
            #[inline(always)]
            pub const fn set_src(&mut self, val: super::vals::HfclkstatSrc) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "State for the HFCLK."]
            #[must_use]
            #[inline(always)]
            pub const fn state(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "State for the HFCLK."]
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
                    .field("state", &self.state())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Hfclkstat {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Hfclkstat {{ src: {:?}, state: {=bool:?} }}",
                    self.src(),
                    self.state()
                )
            }
        }
        #[doc = "Interrupt enable clear register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Disable interrupt on HFCLKSTARTED event."]
            #[must_use]
            #[inline(always)]
            pub const fn hfclkstarted(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on HFCLKSTARTED event."]
            #[inline(always)]
            pub const fn set_hfclkstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Disable interrupt on LFCLKSTARTED event."]
            #[must_use]
            #[inline(always)]
            pub const fn lfclkstarted(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on LFCLKSTARTED event."]
            #[inline(always)]
            pub const fn set_lfclkstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Disable interrupt on DONE event."]
            #[must_use]
            #[inline(always)]
            pub const fn done(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on DONE event."]
            #[inline(always)]
            pub const fn set_done(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Disable interrupt on CTTO event."]
            #[must_use]
            #[inline(always)]
            pub const fn ctto(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on CTTO event."]
            #[inline(always)]
            pub const fn set_ctto(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
                    .field("ctto", &self.ctto())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Int {{ hfclkstarted: {=bool:?}, lfclkstarted: {=bool:?}, done: {=bool:?}, ctto: {=bool:?} }}" , self . hfclkstarted () , self . lfclkstarted () , self . done () , self . ctto ())
            }
        }
        #[doc = "Task LFCLKSTART triggered status."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Lfclkrun(pub u32);
        impl Lfclkrun {
            #[doc = "Task LFCLKSTART triggered status."]
            #[must_use]
            #[inline(always)]
            pub const fn status(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Task LFCLKSTART triggered status."]
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
        #[doc = "Clock source for the LFCLK clock."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Lfclksrc(pub u32);
        impl Lfclksrc {
            #[doc = "Clock source."]
            #[must_use]
            #[inline(always)]
            pub const fn src(&self) -> super::vals::Lfclksrc {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Lfclksrc::from_bits(val as u8)
            }
            #[doc = "Clock source."]
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
        #[doc = "Clock source for the LFCLK clock, set when task LKCLKSTART is triggered."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Lfclksrccopy(pub u32);
        impl Lfclksrccopy {
            #[doc = "Clock source for the LFCLK clock, set when task LKCLKSTART is triggered."]
            #[must_use]
            #[inline(always)]
            pub const fn src(&self) -> super::vals::Lfclksrc {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Lfclksrc::from_bits(val as u8)
            }
            #[doc = "Clock source for the LFCLK clock, set when task LKCLKSTART is triggered."]
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
        #[doc = "Low frequency clock status."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Lfclkstat(pub u32);
        impl Lfclkstat {
            #[doc = "Active clock source for the LF clock."]
            #[must_use]
            #[inline(always)]
            pub const fn src(&self) -> super::vals::Lfclksrc {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Lfclksrc::from_bits(val as u8)
            }
            #[doc = "Active clock source for the LF clock."]
            #[inline(always)]
            pub const fn set_src(&mut self, val: super::vals::Lfclksrc) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
            #[doc = "State for the LF clock."]
            #[must_use]
            #[inline(always)]
            pub const fn state(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "State for the LF clock."]
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
                    .field("state", &self.state())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Lfclkstat {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Lfclkstat {{ src: {:?}, state: {=bool:?} }}",
                    self.src(),
                    self.state()
                )
            }
        }
        #[doc = "Crystal frequency."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Xtalfreq(pub u32);
        impl Xtalfreq {
            #[doc = "External Xtal frequency selection."]
            #[must_use]
            #[inline(always)]
            pub const fn xtalfreq(&self) -> super::vals::Xtalfreq {
                let val = (self.0 >> 0usize) & 0xff;
                super::vals::Xtalfreq::from_bits(val as u8)
            }
            #[doc = "External Xtal frequency selection."]
            #[inline(always)]
            pub const fn set_xtalfreq(&mut self, val: super::vals::Xtalfreq) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Xtalfreq {
            #[inline(always)]
            fn default() -> Xtalfreq {
                Xtalfreq(0)
            }
        }
        impl core::fmt::Debug for Xtalfreq {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Xtalfreq")
                    .field("xtalfreq", &self.xtalfreq())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Xtalfreq {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Xtalfreq {{ xtalfreq: {:?} }}", self.xtalfreq())
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum HfclkstatSrc {
            #[doc = "Internal 16MHz RC oscillator running and generating the HFCLK clock."]
            RC = 0x0,
            #[doc = "External 16MHz/32MHz crystal oscillator running and generating the HFCLK clock."]
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Lfclksrc {
            #[doc = "Internal 32KiHz RC oscillator."]
            RC = 0x0,
            #[doc = "External 32KiHz crystal."]
            XTAL = 0x01,
            #[doc = "Internal 32KiHz synthesizer from HFCLK system clock."]
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
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Xtalfreq(u8);
        impl Xtalfreq {
            #[doc = "32MHz xtal is used as source for the HFCLK oscillator."]
            pub const _32MHZ: Self = Self(0x0);
            #[doc = "16MHz xtal is used as source for the HFCLK oscillator."]
            pub const _16MHZ: Self = Self(0xff);
        }
        impl Xtalfreq {
            pub const fn from_bits(val: u8) -> Xtalfreq {
                Self(val & 0xff)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl core::fmt::Debug for Xtalfreq {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0 => f.write_str("_32MHZ"),
                    0xff => f.write_str("_16MHZ"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Xtalfreq {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0 => defmt::write!(f, "_32MHZ"),
                    0xff => defmt::write!(f, "_16MHZ"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u8> for Xtalfreq {
            #[inline(always)]
            fn from(val: u8) -> Xtalfreq {
                Xtalfreq::from_bits(val)
            }
        }
        impl From<Xtalfreq> for u8 {
            #[inline(always)]
            fn from(val: Xtalfreq) -> u8 {
                Xtalfreq::to_bits(val)
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
pub mod ecb {
    #[doc = "AES ECB Mode Encryption."]
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
        #[doc = "Start ECB block encrypt. If a crypto operation is running, this will not initiate a new encryption and the ERRORECB event will be triggered."]
        #[inline(always)]
        pub const fn tasks_startecb(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop current ECB encryption. If a crypto operation is running, this will will trigger the ERRORECB event."]
        #[inline(always)]
        pub const fn tasks_stopecb(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "ECB block encrypt complete."]
        #[inline(always)]
        pub const fn events_endecb(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "ECB block encrypt aborted due to a STOPECB task or due to an error."]
        #[inline(always)]
        pub const fn events_errorecb(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Interrupt enable set register."]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Interrupt enable clear register."]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "ECB block encrypt memory pointer."]
        #[inline(always)]
        pub const fn ecbdataptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Peripheral power control."]
        #[inline(always)]
        pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Interrupt enable clear register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Disable interrupt on ENDECB event."]
            #[must_use]
            #[inline(always)]
            pub const fn endecb(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on ENDECB event."]
            #[inline(always)]
            pub const fn set_endecb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Disable interrupt on ERRORECB event."]
            #[must_use]
            #[inline(always)]
            pub const fn errorecb(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on ERRORECB event."]
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
        #[doc = "Peripheral power control."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Power(pub u32);
        impl Power {
            #[doc = "Peripheral power control."]
            #[must_use]
            #[inline(always)]
            pub const fn power(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Peripheral power control."]
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
    }
}
pub mod ficr {
    #[doc = "Factory Information Configuration."]
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
        #[doc = "Code memory page size in bytes."]
        #[inline(always)]
        pub const fn codepagesize(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Code memory size in pages."]
        #[inline(always)]
        pub const fn codesize(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Length of code region 0 in bytes."]
        #[inline(always)]
        pub const fn clenr0(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
        }
        #[doc = "Pre-programmed factory code present."]
        #[inline(always)]
        pub const fn ppfc(self) -> crate::common::Reg<regs::Ppfc, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
        }
        #[doc = "Number of individualy controllable RAM blocks."]
        #[inline(always)]
        pub const fn numramblock(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
        }
        #[doc = "Deprecated array of size of RAM block in bytes. This name is kept for backward compatinility purposes. Use SIZERAMBLOCKS instead."]
        #[inline(always)]
        pub const fn sizeramblock(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize + n * 4usize) as _) }
        }
        #[doc = "Size of RAM blocks in bytes."]
        #[inline(always)]
        pub const fn sizeramblocks(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
        }
        #[doc = "Configuration identifier."]
        #[inline(always)]
        pub const fn configid(self) -> crate::common::Reg<regs::Configid, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
        }
        #[doc = "Device identifier."]
        #[inline(always)]
        pub const fn deviceid(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
            assert!(n < 2usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize + n * 4usize) as _) }
        }
        #[doc = "Encryption root."]
        #[inline(always)]
        pub const fn er(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
        }
        #[doc = "Identity root."]
        #[inline(always)]
        pub const fn ir(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize + n * 4usize) as _) }
        }
        #[doc = "Device address type."]
        #[inline(always)]
        pub const fn deviceaddrtype(
            self,
        ) -> crate::common::Reg<regs::Deviceaddrtype, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
        }
        #[doc = "Device address."]
        #[inline(always)]
        pub const fn deviceaddr(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
            assert!(n < 2usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize + n * 4usize) as _) }
        }
        #[doc = "Radio calibration override enable."]
        #[inline(always)]
        pub const fn overrideen(self) -> crate::common::Reg<regs::Overrideen, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
        }
        #[doc = "Override values for the OVERRIDEn registers in RADIO for NRF_1Mbit mode."]
        #[inline(always)]
        pub const fn nrf_1mbit(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
            assert!(n < 5usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize + n * 4usize) as _) }
        }
        #[doc = "Override values for the OVERRIDEn registers in RADIO for BLE_1Mbit mode."]
        #[inline(always)]
        pub const fn ble_1mbit(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
            assert!(n < 5usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xecusize + n * 4usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Configuration identifier."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Configid(pub u32);
        impl Configid {
            #[doc = "Hardware Identification Number."]
            #[must_use]
            #[inline(always)]
            pub const fn hwid(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Hardware Identification Number."]
            #[inline(always)]
            pub const fn set_hwid(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
            #[doc = "Firmware Identification Number pre-loaded into the flash."]
            #[must_use]
            #[inline(always)]
            pub const fn fwid(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0xffff;
                val as u16
            }
            #[doc = "Firmware Identification Number pre-loaded into the flash."]
            #[inline(always)]
            pub const fn set_fwid(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
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
                    .field("fwid", &self.fwid())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Configid {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Configid {{ hwid: {=u16:?}, fwid: {=u16:?} }}",
                    self.hwid(),
                    self.fwid()
                )
            }
        }
        #[doc = "Device address type."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Deviceaddrtype(pub u32);
        impl Deviceaddrtype {
            #[doc = "Device address type."]
            #[must_use]
            #[inline(always)]
            pub const fn deviceaddrtype(&self) -> super::vals::Deviceaddrtype {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Deviceaddrtype::from_bits(val as u8)
            }
            #[doc = "Device address type."]
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
        #[doc = "Radio calibration override enable."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Overrideen(pub u32);
        impl Overrideen {
            #[doc = "Override default values for NRF_1Mbit mode."]
            #[must_use]
            #[inline(always)]
            pub const fn nrf_1mbit(&self) -> super::vals::Nrf1mbit {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Nrf1mbit::from_bits(val as u8)
            }
            #[doc = "Override default values for NRF_1Mbit mode."]
            #[inline(always)]
            pub const fn set_nrf_1mbit(&mut self, val: super::vals::Nrf1mbit) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Override default values for BLE_1Mbit mode."]
            #[must_use]
            #[inline(always)]
            pub const fn ble_1mbit(&self) -> super::vals::Ble1mbit {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Ble1mbit::from_bits(val as u8)
            }
            #[doc = "Override default values for BLE_1Mbit mode."]
            #[inline(always)]
            pub const fn set_ble_1mbit(&mut self, val: super::vals::Ble1mbit) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Overrideen {
            #[inline(always)]
            fn default() -> Overrideen {
                Overrideen(0)
            }
        }
        impl core::fmt::Debug for Overrideen {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Overrideen")
                    .field("nrf_1mbit", &self.nrf_1mbit())
                    .field("ble_1mbit", &self.ble_1mbit())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Overrideen {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Overrideen {{ nrf_1mbit: {:?}, ble_1mbit: {:?} }}",
                    self.nrf_1mbit(),
                    self.ble_1mbit()
                )
            }
        }
        #[doc = "Pre-programmed factory code present."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ppfc(pub u32);
        impl Ppfc {
            #[doc = "Pre-programmed factory code present."]
            #[must_use]
            #[inline(always)]
            pub const fn ppfc(&self) -> super::vals::Ppfc {
                let val = (self.0 >> 0usize) & 0xff;
                super::vals::Ppfc::from_bits(val as u8)
            }
            #[doc = "Pre-programmed factory code present."]
            #[inline(always)]
            pub const fn set_ppfc(&mut self, val: super::vals::Ppfc) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Ppfc {
            #[inline(always)]
            fn default() -> Ppfc {
                Ppfc(0)
            }
        }
        impl core::fmt::Debug for Ppfc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ppfc").field("ppfc", &self.ppfc()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ppfc {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Ppfc {{ ppfc: {:?} }}", self.ppfc())
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Ble1mbit {
            #[doc = "Override the default values for BLE_1Mbit mode."]
            OVERRIDE = 0x0,
            #[doc = "Do not override the default values for BLE_1Mbit mode."]
            NOT_OVERRIDE = 0x01,
        }
        impl Ble1mbit {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Ble1mbit {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Ble1mbit {
            #[inline(always)]
            fn from(val: u8) -> Ble1mbit {
                Ble1mbit::from_bits(val)
            }
        }
        impl From<Ble1mbit> for u8 {
            #[inline(always)]
            fn from(val: Ble1mbit) -> u8 {
                Ble1mbit::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Deviceaddrtype {
            #[doc = "Public address."]
            PUBLIC = 0x0,
            #[doc = "Random address."]
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
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Nrf1mbit {
            #[doc = "Override the default values for NRF_1Mbit mode."]
            OVERRIDE = 0x0,
            #[doc = "Do not override the default values for NRF_1Mbit mode."]
            NOT_OVERRIDE = 0x01,
        }
        impl Nrf1mbit {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Nrf1mbit {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Nrf1mbit {
            #[inline(always)]
            fn from(val: u8) -> Nrf1mbit {
                Nrf1mbit::from_bits(val)
            }
        }
        impl From<Nrf1mbit> for u8 {
            #[inline(always)]
            fn from(val: Nrf1mbit) -> u8 {
                Nrf1mbit::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ppfc(u8);
        impl Ppfc {
            #[doc = "Present."]
            pub const PRESENT: Self = Self(0x0);
            #[doc = "Not present."]
            pub const NOT_PRESENT: Self = Self(0xff);
        }
        impl Ppfc {
            pub const fn from_bits(val: u8) -> Ppfc {
                Self(val & 0xff)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl core::fmt::Debug for Ppfc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0 => f.write_str("PRESENT"),
                    0xff => f.write_str("NOT_PRESENT"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ppfc {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0 => defmt::write!(f, "PRESENT"),
                    0xff => defmt::write!(f, "NOT_PRESENT"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u8> for Ppfc {
            #[inline(always)]
            fn from(val: u8) -> Ppfc {
                Ppfc::from_bits(val)
            }
        }
        impl From<Ppfc> for u8 {
            #[inline(always)]
            fn from(val: Ppfc) -> u8 {
                Ppfc::to_bits(val)
            }
        }
    }
}
pub mod gpio {
    #[doc = "General purpose input and output."]
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
        #[doc = "Write GPIO port."]
        #[inline(always)]
        pub const fn out(self) -> crate::common::Reg<regs::Out, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Set individual bits in GPIO port."]
        #[inline(always)]
        pub const fn outset(self) -> crate::common::Reg<regs::Outset, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Clear individual bits in GPIO port."]
        #[inline(always)]
        pub const fn outclr(self) -> crate::common::Reg<regs::Outclr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Read GPIO port."]
        #[inline(always)]
        pub const fn in_(self) -> crate::common::Reg<regs::In, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "Direction of GPIO pins."]
        #[inline(always)]
        pub const fn dir(self) -> crate::common::Reg<regs::Dir, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
        }
        #[doc = "DIR set register."]
        #[inline(always)]
        pub const fn dirset(self) -> crate::common::Reg<regs::Dirset, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
        #[doc = "DIR clear register."]
        #[inline(always)]
        pub const fn dirclr(self) -> crate::common::Reg<regs::Dirclr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
        }
        #[doc = "Configuration of GPIO pins."]
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
        #[doc = "Direction of GPIO pins."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dir(pub u32);
        impl Dir {
            #[doc = "Pin 0."]
            #[must_use]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> super::vals::Dir {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Dir::from_bits(val as u8)
            }
            #[doc = "Pin 0."]
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
        #[doc = "DIR clear register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dirclr(pub u32);
        impl Dirclr {
            #[doc = "Set as input pin 0."]
            #[must_use]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Set as input pin 0."]
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
        #[doc = "DIR set register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dirset(pub u32);
        impl Dirset {
            #[doc = "Set as output pin 0."]
            #[must_use]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Set as output pin 0."]
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
        #[doc = "Read GPIO port."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct In(pub u32);
        impl In {
            #[doc = "Pin 0."]
            #[must_use]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Pin 0."]
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
        #[doc = "Write GPIO port."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Out(pub u32);
        impl Out {
            #[doc = "Pin 0."]
            #[must_use]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Pin 0."]
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
        #[doc = "Clear individual bits in GPIO port."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Outclr(pub u32);
        impl Outclr {
            #[doc = "Pin 0."]
            #[must_use]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Pin 0."]
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
        #[doc = "Set individual bits in GPIO port."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Outset(pub u32);
        impl Outset {
            #[doc = "Pin 0."]
            #[must_use]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Pin 0."]
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
        #[doc = "Configuration of GPIO pins."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct PinCnf(pub u32);
        impl PinCnf {
            #[doc = "Pin direction."]
            #[must_use]
            #[inline(always)]
            pub const fn dir(&self) -> super::vals::Dir {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Dir::from_bits(val as u8)
            }
            #[doc = "Pin direction."]
            #[inline(always)]
            pub const fn set_dir(&mut self, val: super::vals::Dir) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Connect or disconnect input path."]
            #[must_use]
            #[inline(always)]
            pub const fn input(&self) -> super::vals::Input {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Input::from_bits(val as u8)
            }
            #[doc = "Connect or disconnect input path."]
            #[inline(always)]
            pub const fn set_input(&mut self, val: super::vals::Input) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Pull-up or -down configuration."]
            #[must_use]
            #[inline(always)]
            pub const fn pull(&self) -> super::vals::Pull {
                let val = (self.0 >> 2usize) & 0x03;
                super::vals::Pull::from_bits(val as u8)
            }
            #[doc = "Pull-up or -down configuration."]
            #[inline(always)]
            pub const fn set_pull(&mut self, val: super::vals::Pull) {
                self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
            }
            #[doc = "Drive configuration."]
            #[must_use]
            #[inline(always)]
            pub const fn drive(&self) -> super::vals::Drive {
                let val = (self.0 >> 8usize) & 0x07;
                super::vals::Drive::from_bits(val as u8)
            }
            #[doc = "Drive configuration."]
            #[inline(always)]
            pub const fn set_drive(&mut self, val: super::vals::Drive) {
                self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
            }
            #[doc = "Pin sensing mechanism."]
            #[must_use]
            #[inline(always)]
            pub const fn sense(&self) -> super::vals::Sense {
                let val = (self.0 >> 16usize) & 0x03;
                super::vals::Sense::from_bits(val as u8)
            }
            #[doc = "Pin sensing mechanism."]
            #[inline(always)]
            pub const fn set_sense(&mut self, val: super::vals::Sense) {
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
        impl core::fmt::Debug for PinCnf {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("PinCnf")
                    .field("dir", &self.dir())
                    .field("input", &self.input())
                    .field("pull", &self.pull())
                    .field("drive", &self.drive())
                    .field("sense", &self.sense())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for PinCnf {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "PinCnf {{ dir: {:?}, input: {:?}, pull: {:?}, drive: {:?}, sense: {:?} }}",
                    self.dir(),
                    self.input(),
                    self.pull(),
                    self.drive(),
                    self.sense()
                )
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Dir {
            #[doc = "Configure pin as an input pin."]
            INPUT = 0x0,
            #[doc = "Configure pin as an output pin."]
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
            #[doc = "Standard '0', Standard '1'."]
            S0S1 = 0x0,
            #[doc = "High '0', Standard '1'."]
            H0S1 = 0x01,
            #[doc = "Standard '0', High '1'."]
            S0H1 = 0x02,
            #[doc = "High '0', High '1'."]
            H0H1 = 0x03,
            #[doc = "Disconnected '0', Standard '1'."]
            D0S1 = 0x04,
            #[doc = "Disconnected '0', High '1'."]
            D0H1 = 0x05,
            #[doc = "Standard '0', Disconnected '1'."]
            S0D1 = 0x06,
            #[doc = "High '0', Disconnected '1'."]
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Input {
            #[doc = "Connect input pin."]
            CONNECT = 0x0,
            #[doc = "Disconnect input pin."]
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
        pub enum Pull {
            #[doc = "No pull."]
            DISABLED = 0x0,
            #[doc = "Pulldown on pin."]
            PULLDOWN = 0x01,
            _RESERVED_2 = 0x02,
            #[doc = "Pullup on pin."]
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
            #[doc = "Disabled."]
            DISABLED = 0x0,
            _RESERVED_1 = 0x01,
            #[doc = "Wakeup on high level."]
            HIGH = 0x02,
            #[doc = "Wakeup on low level."]
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
    #[doc = "GPIO tasks and events."]
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
        #[doc = "Tasks asssociated with GPIOTE channels."]
        #[inline(always)]
        pub const fn tasks_out(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
        }
        #[doc = "Tasks asssociated with GPIOTE channels."]
        #[inline(always)]
        pub const fn events_in(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
        }
        #[doc = "Event generated from multiple pins."]
        #[inline(always)]
        pub const fn events_port(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x017cusize) as _) }
        }
        #[doc = "Interrupt enable set register."]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Interrupt enable clear register."]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Channel configuration registers."]
        #[inline(always)]
        pub const fn config(self, n: usize) -> crate::common::Reg<regs::Config, crate::common::RW> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize + n * 4usize) as _) }
        }
        #[doc = "Peripheral power control."]
        #[inline(always)]
        pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Channel configuration registers."]
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
            #[doc = "Pin select."]
            #[must_use]
            #[inline(always)]
            pub const fn psel(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0x1f;
                val as u8
            }
            #[doc = "Pin select."]
            #[inline(always)]
            pub const fn set_psel(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
            }
            #[doc = "Effects on output when in Task mode, or events on input that generates an event."]
            #[must_use]
            #[inline(always)]
            pub const fn polarity(&self) -> super::vals::Polarity {
                let val = (self.0 >> 16usize) & 0x03;
                super::vals::Polarity::from_bits(val as u8)
            }
            #[doc = "Effects on output when in Task mode, or events on input that generates an event."]
            #[inline(always)]
            pub const fn set_polarity(&mut self, val: super::vals::Polarity) {
                self.0 =
                    (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
            }
            #[doc = "Initial value of the output when the GPIOTE channel is configured as a Task."]
            #[must_use]
            #[inline(always)]
            pub const fn outinit(&self) -> super::vals::Outinit {
                let val = (self.0 >> 20usize) & 0x01;
                super::vals::Outinit::from_bits(val as u8)
            }
            #[doc = "Initial value of the output when the GPIOTE channel is configured as a Task."]
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
                    .field("polarity", &self.polarity())
                    .field("outinit", &self.outinit())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Config {{ mode: {:?}, psel: {=u8:?}, polarity: {:?}, outinit: {:?} }}",
                    self.mode(),
                    self.psel(),
                    self.polarity(),
                    self.outinit()
                )
            }
        }
        #[doc = "Interrupt enable clear register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Disable interrupt on IN\\[0\\] event."]
            #[must_use]
            #[inline(always)]
            pub const fn in_(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on IN\\[0\\] event."]
            #[inline(always)]
            pub const fn set_in_(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Disable interrupt on PORT event."]
            #[must_use]
            #[inline(always)]
            pub const fn port(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on PORT event."]
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
                    .field("port", &self.port())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Int {{ in_[0]: {=bool:?}, in_[1]: {=bool:?}, in_[2]: {=bool:?}, in_[3]: {=bool:?}, port: {=bool:?} }}" , self . in_ (0usize) , self . in_ (1usize) , self . in_ (2usize) , self . in_ (3usize) , self . port ())
            }
        }
        #[doc = "Peripheral power control."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Power(pub u32);
        impl Power {
            #[doc = "Peripheral power control."]
            #[must_use]
            #[inline(always)]
            pub const fn power(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Peripheral power control."]
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
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Mode {
            #[doc = "Disabled."]
            DISABLED = 0x0,
            #[doc = "Channel configure in event mode."]
            EVENT = 0x01,
            _RESERVED_2 = 0x02,
            #[doc = "Channel configure in task mode."]
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
            #[doc = "Initial low output when in task mode."]
            LOW = 0x0,
            #[doc = "Initial high output when in task mode."]
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
            #[doc = "No task or event."]
            NONE = 0x0,
            #[doc = "Low to high."]
            LO_TO_HI = 0x01,
            #[doc = "High to low."]
            HI_TO_LO = 0x02,
            #[doc = "Toggle."]
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
pub mod lpcomp {
    #[doc = "Low power comparator."]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lpcomp {
        ptr: *mut u8,
    }
    unsafe impl Send for Lpcomp {}
    unsafe impl Sync for Lpcomp {}
    impl Lpcomp {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start the comparator."]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop the comparator."]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Sample comparator value."]
        #[inline(always)]
        pub const fn tasks_sample(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "LPCOMP is ready and output is valid."]
        #[inline(always)]
        pub const fn events_ready(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Input voltage crossed the threshold going down."]
        #[inline(always)]
        pub const fn events_down(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Input voltage crossed the threshold going up."]
        #[inline(always)]
        pub const fn events_up(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "Input voltage crossed the threshold in any direction."]
        #[inline(always)]
        pub const fn events_cross(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
        }
        #[doc = "Shortcuts for the LPCOMP."]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Interrupt enable set register."]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Interrupt enable clear register."]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Result of last compare."]
        #[inline(always)]
        pub const fn result(self) -> crate::common::Reg<regs::Result, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "Enable the LPCOMP."]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Input pin select."]
        #[inline(always)]
        pub const fn psel(self) -> crate::common::Reg<regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Reference select."]
        #[inline(always)]
        pub const fn refsel(self) -> crate::common::Reg<regs::Refsel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "External reference select."]
        #[inline(always)]
        pub const fn extrefsel(self) -> crate::common::Reg<regs::Extrefsel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Analog detect configuration."]
        #[inline(always)]
        pub const fn anadetect(self) -> crate::common::Reg<regs::Anadetect, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize) as _) }
        }
        #[doc = "Peripheral power control."]
        #[inline(always)]
        pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Analog detect configuration."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Anadetect(pub u32);
        impl Anadetect {
            #[doc = "Analog detect configuration."]
            #[must_use]
            #[inline(always)]
            pub const fn anadetect(&self) -> super::vals::Anadetect {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Anadetect::from_bits(val as u8)
            }
            #[doc = "Analog detect configuration."]
            #[inline(always)]
            pub const fn set_anadetect(&mut self, val: super::vals::Anadetect) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Anadetect {
            #[inline(always)]
            fn default() -> Anadetect {
                Anadetect(0)
            }
        }
        impl core::fmt::Debug for Anadetect {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Anadetect")
                    .field("anadetect", &self.anadetect())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Anadetect {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Anadetect {{ anadetect: {:?} }}", self.anadetect())
            }
        }
        #[doc = "Enable the LPCOMP."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable LPCOMP."]
            #[must_use]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable LPCOMP."]
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
        #[doc = "External reference select."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Extrefsel(pub u32);
        impl Extrefsel {
            #[doc = "External analog reference pin selection."]
            #[must_use]
            #[inline(always)]
            pub const fn extrefsel(&self) -> super::vals::Extrefsel {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Extrefsel::from_bits(val as u8)
            }
            #[doc = "External analog reference pin selection."]
            #[inline(always)]
            pub const fn set_extrefsel(&mut self, val: super::vals::Extrefsel) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Extrefsel {
            #[inline(always)]
            fn default() -> Extrefsel {
                Extrefsel(0)
            }
        }
        impl core::fmt::Debug for Extrefsel {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Extrefsel")
                    .field("extrefsel", &self.extrefsel())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Extrefsel {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Extrefsel {{ extrefsel: {:?} }}", self.extrefsel())
            }
        }
        #[doc = "Interrupt enable clear register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Disable interrupt on READY event."]
            #[must_use]
            #[inline(always)]
            pub const fn ready(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on READY event."]
            #[inline(always)]
            pub const fn set_ready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Disable interrupt on DOWN event."]
            #[must_use]
            #[inline(always)]
            pub const fn down(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on DOWN event."]
            #[inline(always)]
            pub const fn set_down(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Disable interrupt on UP event."]
            #[must_use]
            #[inline(always)]
            pub const fn up(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on UP event."]
            #[inline(always)]
            pub const fn set_up(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Disable interrupt on CROSS event."]
            #[must_use]
            #[inline(always)]
            pub const fn cross(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on CROSS event."]
            #[inline(always)]
            pub const fn set_cross(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
                    .field("down", &self.down())
                    .field("up", &self.up())
                    .field("cross", &self.cross())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Int {{ ready: {=bool:?}, down: {=bool:?}, up: {=bool:?}, cross: {=bool:?} }}",
                    self.ready(),
                    self.down(),
                    self.up(),
                    self.cross()
                )
            }
        }
        #[doc = "Peripheral power control."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Power(pub u32);
        impl Power {
            #[doc = "Peripheral power control."]
            #[must_use]
            #[inline(always)]
            pub const fn power(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Peripheral power control."]
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
        #[doc = "Input pin select."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Psel(pub u32);
        impl Psel {
            #[doc = "Analog input pin select."]
            #[must_use]
            #[inline(always)]
            pub const fn psel(&self) -> super::vals::PselPsel {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::PselPsel::from_bits(val as u8)
            }
            #[doc = "Analog input pin select."]
            #[inline(always)]
            pub const fn set_psel(&mut self, val: super::vals::PselPsel) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
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
                f.debug_struct("Psel").field("psel", &self.psel()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Psel {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Psel {{ psel: {:?} }}", self.psel())
            }
        }
        #[doc = "Reference select."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Refsel(pub u32);
        impl Refsel {
            #[doc = "Reference select."]
            #[must_use]
            #[inline(always)]
            pub const fn refsel(&self) -> super::vals::Refsel {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::Refsel::from_bits(val as u8)
            }
            #[doc = "Reference select."]
            #[inline(always)]
            pub const fn set_refsel(&mut self, val: super::vals::Refsel) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
            }
        }
        impl Default for Refsel {
            #[inline(always)]
            fn default() -> Refsel {
                Refsel(0)
            }
        }
        impl core::fmt::Debug for Refsel {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Refsel")
                    .field("refsel", &self.refsel())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Refsel {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Refsel {{ refsel: {:?} }}", self.refsel())
            }
        }
        #[doc = "Result of last compare."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Result(pub u32);
        impl Result {
            #[doc = "Result of last compare. Decision point SAMPLE task."]
            #[must_use]
            #[inline(always)]
            pub const fn result(&self) -> super::vals::Result {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Result::from_bits(val as u8)
            }
            #[doc = "Result of last compare. Decision point SAMPLE task."]
            #[inline(always)]
            pub const fn set_result(&mut self, val: super::vals::Result) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Result {
            #[inline(always)]
            fn default() -> Result {
                Result(0)
            }
        }
        impl core::fmt::Debug for Result {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Result")
                    .field("result", &self.result())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Result {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Result {{ result: {:?} }}", self.result())
            }
        }
        #[doc = "Shortcuts for the LPCOMP."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between READY event and SAMPLE task."]
            #[must_use]
            #[inline(always)]
            pub const fn ready_sample(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between READY event and SAMPLE task."]
            #[inline(always)]
            pub const fn set_ready_sample(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Shortcut between RADY event and STOP task."]
            #[must_use]
            #[inline(always)]
            pub const fn ready_stop(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between RADY event and STOP task."]
            #[inline(always)]
            pub const fn set_ready_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Shortcut between DOWN event and STOP task."]
            #[must_use]
            #[inline(always)]
            pub const fn down_stop(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between DOWN event and STOP task."]
            #[inline(always)]
            pub const fn set_down_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Shortcut between UP event and STOP task."]
            #[must_use]
            #[inline(always)]
            pub const fn up_stop(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between UP event and STOP task."]
            #[inline(always)]
            pub const fn set_up_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Shortcut between CROSS event and STOP task."]
            #[must_use]
            #[inline(always)]
            pub const fn cross_stop(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between CROSS event and STOP task."]
            #[inline(always)]
            pub const fn set_cross_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
                    .field("ready_sample", &self.ready_sample())
                    .field("ready_stop", &self.ready_stop())
                    .field("down_stop", &self.down_stop())
                    .field("up_stop", &self.up_stop())
                    .field("cross_stop", &self.cross_stop())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Shorts {{ ready_sample: {=bool:?}, ready_stop: {=bool:?}, down_stop: {=bool:?}, up_stop: {=bool:?}, cross_stop: {=bool:?} }}" , self . ready_sample () , self . ready_stop () , self . down_stop () , self . up_stop () , self . cross_stop ())
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Anadetect {
            #[doc = "Generate ANADETEC on crossing, both upwards and downwards crossing."]
            CROSS = 0x0,
            #[doc = "Generate ANADETEC on upwards crossing only."]
            UP = 0x01,
            #[doc = "Generate ANADETEC on downwards crossing only."]
            DOWN = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Anadetect {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Anadetect {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Anadetect {
            #[inline(always)]
            fn from(val: u8) -> Anadetect {
                Anadetect::from_bits(val)
            }
        }
        impl From<Anadetect> for u8 {
            #[inline(always)]
            fn from(val: Anadetect) -> u8 {
                Anadetect::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Enable {
            #[doc = "Disabled LPCOMP."]
            DISABLED = 0x0,
            #[doc = "Enable LPCOMP."]
            ENABLED = 0x01,
            _RESERVED_2 = 0x02,
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
        pub enum Extrefsel {
            #[doc = "Use analog reference 0 as reference."]
            ANALOG_REFERENCE0 = 0x0,
            #[doc = "Use analog reference 1 as reference."]
            ANALOG_REFERENCE1 = 0x01,
        }
        impl Extrefsel {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Extrefsel {
                unsafe { core::mem::transmute(val & 0x01) }
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum PselPsel {
            #[doc = "Use analog input 0 as analog input."]
            ANALOG_INPUT0 = 0x0,
            #[doc = "Use analog input 1 as analog input."]
            ANALOG_INPUT1 = 0x01,
            #[doc = "Use analog input 2 as analog input."]
            ANALOG_INPUT2 = 0x02,
            #[doc = "Use analog input 3 as analog input."]
            ANALOG_INPUT3 = 0x03,
            #[doc = "Use analog input 4 as analog input."]
            ANALOG_INPUT4 = 0x04,
            #[doc = "Use analog input 5 as analog input."]
            ANALOG_INPUT5 = 0x05,
            #[doc = "Use analog input 6 as analog input."]
            ANALOG_INPUT6 = 0x06,
            #[doc = "Use analog input 7 as analog input."]
            ANALOG_INPUT7 = 0x07,
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Refsel {
            #[doc = "Use supply with a 1/8 prescaler as reference."]
            SUPPLY_ONE_EIGHTH_PRESCALING = 0x0,
            #[doc = "Use supply with a 2/8 prescaler as reference."]
            SUPPLY_TWO_EIGHTHS_PRESCALING = 0x01,
            #[doc = "Use supply with a 3/8 prescaler as reference."]
            SUPPLY_THREE_EIGHTHS_PRESCALING = 0x02,
            #[doc = "Use supply with a 4/8 prescaler as reference."]
            SUPPLY_FOUR_EIGHTHS_PRESCALING = 0x03,
            #[doc = "Use supply with a 5/8 prescaler as reference."]
            SUPPLY_FIVE_EIGHTHS_PRESCALING = 0x04,
            #[doc = "Use supply with a 6/8 prescaler as reference."]
            SUPPLY_SIX_EIGHTHS_PRESCALING = 0x05,
            #[doc = "Use supply with a 7/8 prescaler as reference."]
            SUPPLY_SEVEN_EIGHTHS_PRESCALING = 0x06,
            #[doc = "Use external analog reference as reference."]
            AREF = 0x07,
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Result {
            #[doc = "Input voltage is bellow the reference threshold."]
            BELOW = 0x0,
            #[doc = "Input voltage is above the reference threshold."]
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
    }
}
pub mod mpu {
    #[doc = "Memory Protection Unit."]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mpu {
        ptr: *mut u8,
    }
    unsafe impl Send for Mpu {}
    unsafe impl Sync for Mpu {}
    impl Mpu {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Configuration of peripherals in mpu regions."]
        #[inline(always)]
        pub const fn perr0(self) -> crate::common::Reg<regs::Perr0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0528usize) as _) }
        }
        #[doc = "Length of RAM region 0."]
        #[inline(always)]
        pub const fn rlenr0(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x052cusize) as _) }
        }
        #[doc = "Erase and write protection bit enable set register."]
        #[inline(always)]
        pub const fn protenset0(self) -> crate::common::Reg<regs::Protenset0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize) as _) }
        }
        #[doc = "Erase and write protection bit enable set register."]
        #[inline(always)]
        pub const fn protenset1(self) -> crate::common::Reg<regs::Protenset1, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0604usize) as _) }
        }
        #[doc = "Disable erase and write protection mechanism in debug mode."]
        #[inline(always)]
        pub const fn disableindebug(
            self,
        ) -> crate::common::Reg<regs::Disableindebug, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0608usize) as _) }
        }
        #[doc = "Erase and write protection block size."]
        #[inline(always)]
        pub const fn protblocksize(
            self,
        ) -> crate::common::Reg<regs::Protblocksize, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x060cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Disable erase and write protection mechanism in debug mode."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Disableindebug(pub u32);
        impl Disableindebug {
            #[doc = "Disable protection mechanism in debug mode."]
            #[must_use]
            #[inline(always)]
            pub const fn disableindebug(&self) -> super::vals::Disableindebug {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Disableindebug::from_bits(val as u8)
            }
            #[doc = "Disable protection mechanism in debug mode."]
            #[inline(always)]
            pub const fn set_disableindebug(&mut self, val: super::vals::Disableindebug) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Disableindebug {
            #[inline(always)]
            fn default() -> Disableindebug {
                Disableindebug(0)
            }
        }
        impl core::fmt::Debug for Disableindebug {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Disableindebug")
                    .field("disableindebug", &self.disableindebug())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Disableindebug {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Disableindebug {{ disableindebug: {:?} }}",
                    self.disableindebug()
                )
            }
        }
        #[doc = "Configuration of peripherals in mpu regions."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Perr0(pub u32);
        impl Perr0 {
            #[doc = "POWER_CLOCK region configuration."]
            #[must_use]
            #[inline(always)]
            pub const fn power_clock(&self) -> super::vals::PowerClock {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::PowerClock::from_bits(val as u8)
            }
            #[doc = "POWER_CLOCK region configuration."]
            #[inline(always)]
            pub const fn set_power_clock(&mut self, val: super::vals::PowerClock) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "RADIO region configuration."]
            #[must_use]
            #[inline(always)]
            pub const fn radio(&self) -> super::vals::Radio {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Radio::from_bits(val as u8)
            }
            #[doc = "RADIO region configuration."]
            #[inline(always)]
            pub const fn set_radio(&mut self, val: super::vals::Radio) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "UART0 region configuration."]
            #[must_use]
            #[inline(always)]
            pub const fn uart0(&self) -> super::vals::Uart0 {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Uart0::from_bits(val as u8)
            }
            #[doc = "UART0 region configuration."]
            #[inline(always)]
            pub const fn set_uart0(&mut self, val: super::vals::Uart0) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
            #[doc = "SPI0 and TWI0 region configuration."]
            #[must_use]
            #[inline(always)]
            pub const fn spi0_twi0(&self) -> super::vals::Spi0Twi0 {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Spi0Twi0::from_bits(val as u8)
            }
            #[doc = "SPI0 and TWI0 region configuration."]
            #[inline(always)]
            pub const fn set_spi0_twi0(&mut self, val: super::vals::Spi0Twi0) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
            }
            #[doc = "SPI1 and TWI1 region configuration."]
            #[must_use]
            #[inline(always)]
            pub const fn spi1_twi1(&self) -> super::vals::Spi1Twi1 {
                let val = (self.0 >> 4usize) & 0x01;
                super::vals::Spi1Twi1::from_bits(val as u8)
            }
            #[doc = "SPI1 and TWI1 region configuration."]
            #[inline(always)]
            pub const fn set_spi1_twi1(&mut self, val: super::vals::Spi1Twi1) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
            }
            #[doc = "GPIOTE region configuration."]
            #[must_use]
            #[inline(always)]
            pub const fn gpiote(&self) -> super::vals::Gpiote {
                let val = (self.0 >> 6usize) & 0x01;
                super::vals::Gpiote::from_bits(val as u8)
            }
            #[doc = "GPIOTE region configuration."]
            #[inline(always)]
            pub const fn set_gpiote(&mut self, val: super::vals::Gpiote) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
            }
            #[doc = "ADC region configuration."]
            #[must_use]
            #[inline(always)]
            pub const fn adc(&self) -> super::vals::Adc {
                let val = (self.0 >> 7usize) & 0x01;
                super::vals::Adc::from_bits(val as u8)
            }
            #[doc = "ADC region configuration."]
            #[inline(always)]
            pub const fn set_adc(&mut self, val: super::vals::Adc) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
            }
            #[doc = "TIMER0 region configuration."]
            #[must_use]
            #[inline(always)]
            pub const fn timer0(&self) -> super::vals::Timer0 {
                let val = (self.0 >> 8usize) & 0x01;
                super::vals::Timer0::from_bits(val as u8)
            }
            #[doc = "TIMER0 region configuration."]
            #[inline(always)]
            pub const fn set_timer0(&mut self, val: super::vals::Timer0) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
            }
            #[doc = "TIMER1 region configuration."]
            #[must_use]
            #[inline(always)]
            pub const fn timer1(&self) -> super::vals::Timer1 {
                let val = (self.0 >> 9usize) & 0x01;
                super::vals::Timer1::from_bits(val as u8)
            }
            #[doc = "TIMER1 region configuration."]
            #[inline(always)]
            pub const fn set_timer1(&mut self, val: super::vals::Timer1) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
            }
            #[doc = "TIMER2 region configuration."]
            #[must_use]
            #[inline(always)]
            pub const fn timer2(&self) -> super::vals::Timer2 {
                let val = (self.0 >> 10usize) & 0x01;
                super::vals::Timer2::from_bits(val as u8)
            }
            #[doc = "TIMER2 region configuration."]
            #[inline(always)]
            pub const fn set_timer2(&mut self, val: super::vals::Timer2) {
                self.0 =
                    (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
            }
            #[doc = "RTC0 region configuration."]
            #[must_use]
            #[inline(always)]
            pub const fn rtc0(&self) -> super::vals::Rtc0 {
                let val = (self.0 >> 11usize) & 0x01;
                super::vals::Rtc0::from_bits(val as u8)
            }
            #[doc = "RTC0 region configuration."]
            #[inline(always)]
            pub const fn set_rtc0(&mut self, val: super::vals::Rtc0) {
                self.0 =
                    (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
            }
            #[doc = "TEMP region configuration."]
            #[must_use]
            #[inline(always)]
            pub const fn temp(&self) -> super::vals::Temp {
                let val = (self.0 >> 12usize) & 0x01;
                super::vals::Temp::from_bits(val as u8)
            }
            #[doc = "TEMP region configuration."]
            #[inline(always)]
            pub const fn set_temp(&mut self, val: super::vals::Temp) {
                self.0 =
                    (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
            }
            #[doc = "RNG region configuration."]
            #[must_use]
            #[inline(always)]
            pub const fn rng(&self) -> super::vals::Rng {
                let val = (self.0 >> 13usize) & 0x01;
                super::vals::Rng::from_bits(val as u8)
            }
            #[doc = "RNG region configuration."]
            #[inline(always)]
            pub const fn set_rng(&mut self, val: super::vals::Rng) {
                self.0 =
                    (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
            }
            #[doc = "ECB region configuration."]
            #[must_use]
            #[inline(always)]
            pub const fn ecb(&self) -> super::vals::Ecb {
                let val = (self.0 >> 14usize) & 0x01;
                super::vals::Ecb::from_bits(val as u8)
            }
            #[doc = "ECB region configuration."]
            #[inline(always)]
            pub const fn set_ecb(&mut self, val: super::vals::Ecb) {
                self.0 =
                    (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
            }
            #[doc = "CCM and AAR region configuration."]
            #[must_use]
            #[inline(always)]
            pub const fn ccm_aar(&self) -> super::vals::CcmAar {
                let val = (self.0 >> 15usize) & 0x01;
                super::vals::CcmAar::from_bits(val as u8)
            }
            #[doc = "CCM and AAR region configuration."]
            #[inline(always)]
            pub const fn set_ccm_aar(&mut self, val: super::vals::CcmAar) {
                self.0 =
                    (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
            }
            #[doc = "WDT region configuration."]
            #[must_use]
            #[inline(always)]
            pub const fn wdt(&self) -> super::vals::Wdt {
                let val = (self.0 >> 16usize) & 0x01;
                super::vals::Wdt::from_bits(val as u8)
            }
            #[doc = "WDT region configuration."]
            #[inline(always)]
            pub const fn set_wdt(&mut self, val: super::vals::Wdt) {
                self.0 =
                    (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
            }
            #[doc = "RTC1 region configuration."]
            #[must_use]
            #[inline(always)]
            pub const fn rtc1(&self) -> super::vals::Rtc1 {
                let val = (self.0 >> 17usize) & 0x01;
                super::vals::Rtc1::from_bits(val as u8)
            }
            #[doc = "RTC1 region configuration."]
            #[inline(always)]
            pub const fn set_rtc1(&mut self, val: super::vals::Rtc1) {
                self.0 =
                    (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
            }
            #[doc = "QDEC region configuration."]
            #[must_use]
            #[inline(always)]
            pub const fn qdec(&self) -> super::vals::Qdec {
                let val = (self.0 >> 18usize) & 0x01;
                super::vals::Qdec::from_bits(val as u8)
            }
            #[doc = "QDEC region configuration."]
            #[inline(always)]
            pub const fn set_qdec(&mut self, val: super::vals::Qdec) {
                self.0 =
                    (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
            }
            #[doc = "LPCOMP region configuration."]
            #[must_use]
            #[inline(always)]
            pub const fn lpcomp(&self) -> super::vals::Lpcomp {
                let val = (self.0 >> 19usize) & 0x01;
                super::vals::Lpcomp::from_bits(val as u8)
            }
            #[doc = "LPCOMP region configuration."]
            #[inline(always)]
            pub const fn set_lpcomp(&mut self, val: super::vals::Lpcomp) {
                self.0 =
                    (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
            }
            #[doc = "NVMC region configuration."]
            #[must_use]
            #[inline(always)]
            pub const fn nvmc(&self) -> super::vals::Nvmc {
                let val = (self.0 >> 30usize) & 0x01;
                super::vals::Nvmc::from_bits(val as u8)
            }
            #[doc = "NVMC region configuration."]
            #[inline(always)]
            pub const fn set_nvmc(&mut self, val: super::vals::Nvmc) {
                self.0 =
                    (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
            }
            #[doc = "PPI region configuration."]
            #[must_use]
            #[inline(always)]
            pub const fn ppi(&self) -> super::vals::Ppi {
                let val = (self.0 >> 31usize) & 0x01;
                super::vals::Ppi::from_bits(val as u8)
            }
            #[doc = "PPI region configuration."]
            #[inline(always)]
            pub const fn set_ppi(&mut self, val: super::vals::Ppi) {
                self.0 =
                    (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
            }
        }
        impl Default for Perr0 {
            #[inline(always)]
            fn default() -> Perr0 {
                Perr0(0)
            }
        }
        impl core::fmt::Debug for Perr0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Perr0")
                    .field("power_clock", &self.power_clock())
                    .field("radio", &self.radio())
                    .field("uart0", &self.uart0())
                    .field("spi0_twi0", &self.spi0_twi0())
                    .field("spi1_twi1", &self.spi1_twi1())
                    .field("gpiote", &self.gpiote())
                    .field("adc", &self.adc())
                    .field("timer0", &self.timer0())
                    .field("timer1", &self.timer1())
                    .field("timer2", &self.timer2())
                    .field("rtc0", &self.rtc0())
                    .field("temp", &self.temp())
                    .field("rng", &self.rng())
                    .field("ecb", &self.ecb())
                    .field("ccm_aar", &self.ccm_aar())
                    .field("wdt", &self.wdt())
                    .field("rtc1", &self.rtc1())
                    .field("qdec", &self.qdec())
                    .field("lpcomp", &self.lpcomp())
                    .field("nvmc", &self.nvmc())
                    .field("ppi", &self.ppi())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Perr0 {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Perr0 {{ power_clock: {:?}, radio: {:?}, uart0: {:?}, spi0_twi0: {:?}, spi1_twi1: {:?}, gpiote: {:?}, adc: {:?}, timer0: {:?}, timer1: {:?}, timer2: {:?}, rtc0: {:?}, temp: {:?}, rng: {:?}, ecb: {:?}, ccm_aar: {:?}, wdt: {:?}, rtc1: {:?}, qdec: {:?}, lpcomp: {:?}, nvmc: {:?}, ppi: {:?} }}" , self . power_clock () , self . radio () , self . uart0 () , self . spi0_twi0 () , self . spi1_twi1 () , self . gpiote () , self . adc () , self . timer0 () , self . timer1 () , self . timer2 () , self . rtc0 () , self . temp () , self . rng () , self . ecb () , self . ccm_aar () , self . wdt () , self . rtc1 () , self . qdec () , self . lpcomp () , self . nvmc () , self . ppi ())
            }
        }
        #[doc = "Erase and write protection block size."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Protblocksize(pub u32);
        impl Protblocksize {
            #[doc = "Erase and write protection block size."]
            #[must_use]
            #[inline(always)]
            pub const fn protblocksize(&self) -> super::vals::Protblocksize {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Protblocksize::from_bits(val as u8)
            }
            #[doc = "Erase and write protection block size."]
            #[inline(always)]
            pub const fn set_protblocksize(&mut self, val: super::vals::Protblocksize) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Protblocksize {
            #[inline(always)]
            fn default() -> Protblocksize {
                Protblocksize(0)
            }
        }
        impl core::fmt::Debug for Protblocksize {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Protblocksize")
                    .field("protblocksize", &self.protblocksize())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Protblocksize {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Protblocksize {{ protblocksize: {:?} }}",
                    self.protblocksize()
                )
            }
        }
        #[doc = "Erase and write protection bit enable set register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Protenset0(pub u32);
        impl Protenset0 {
            #[doc = "Protection enable for region 0."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg0(&self) -> super::vals::Protreg0 {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Protreg0::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 0."]
            #[inline(always)]
            pub const fn set_protreg0(&mut self, val: super::vals::Protreg0) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Protection enable for region 1."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg1(&self) -> super::vals::Protreg1 {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Protreg1::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 1."]
            #[inline(always)]
            pub const fn set_protreg1(&mut self, val: super::vals::Protreg1) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Protection enable for region 2."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg2(&self) -> super::vals::Protreg2 {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Protreg2::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 2."]
            #[inline(always)]
            pub const fn set_protreg2(&mut self, val: super::vals::Protreg2) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
            #[doc = "Protection enable for region 3."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg3(&self) -> super::vals::Protreg3 {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Protreg3::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 3."]
            #[inline(always)]
            pub const fn set_protreg3(&mut self, val: super::vals::Protreg3) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
            }
            #[doc = "Protection enable for region 4."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg4(&self) -> super::vals::Protreg4 {
                let val = (self.0 >> 4usize) & 0x01;
                super::vals::Protreg4::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 4."]
            #[inline(always)]
            pub const fn set_protreg4(&mut self, val: super::vals::Protreg4) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
            }
            #[doc = "Protection enable for region 5."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg5(&self) -> super::vals::Protreg5 {
                let val = (self.0 >> 5usize) & 0x01;
                super::vals::Protreg5::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 5."]
            #[inline(always)]
            pub const fn set_protreg5(&mut self, val: super::vals::Protreg5) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
            }
            #[doc = "Protection enable for region 6."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg6(&self) -> super::vals::Protreg6 {
                let val = (self.0 >> 6usize) & 0x01;
                super::vals::Protreg6::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 6."]
            #[inline(always)]
            pub const fn set_protreg6(&mut self, val: super::vals::Protreg6) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
            }
            #[doc = "Protection enable for region 7."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg7(&self) -> super::vals::Protreg7 {
                let val = (self.0 >> 7usize) & 0x01;
                super::vals::Protreg7::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 7."]
            #[inline(always)]
            pub const fn set_protreg7(&mut self, val: super::vals::Protreg7) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
            }
            #[doc = "Protection enable for region 8."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg8(&self) -> super::vals::Protreg8 {
                let val = (self.0 >> 8usize) & 0x01;
                super::vals::Protreg8::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 8."]
            #[inline(always)]
            pub const fn set_protreg8(&mut self, val: super::vals::Protreg8) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
            }
            #[doc = "Protection enable for region 9."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg9(&self) -> super::vals::Protreg9 {
                let val = (self.0 >> 9usize) & 0x01;
                super::vals::Protreg9::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 9."]
            #[inline(always)]
            pub const fn set_protreg9(&mut self, val: super::vals::Protreg9) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
            }
            #[doc = "Protection enable for region 10."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg10(&self) -> super::vals::Protreg10 {
                let val = (self.0 >> 10usize) & 0x01;
                super::vals::Protreg10::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 10."]
            #[inline(always)]
            pub const fn set_protreg10(&mut self, val: super::vals::Protreg10) {
                self.0 =
                    (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
            }
            #[doc = "Protection enable for region 11."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg11(&self) -> super::vals::Protreg11 {
                let val = (self.0 >> 11usize) & 0x01;
                super::vals::Protreg11::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 11."]
            #[inline(always)]
            pub const fn set_protreg11(&mut self, val: super::vals::Protreg11) {
                self.0 =
                    (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
            }
            #[doc = "Protection enable for region 12."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg12(&self) -> super::vals::Protreg12 {
                let val = (self.0 >> 12usize) & 0x01;
                super::vals::Protreg12::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 12."]
            #[inline(always)]
            pub const fn set_protreg12(&mut self, val: super::vals::Protreg12) {
                self.0 =
                    (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
            }
            #[doc = "Protection enable for region 13."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg13(&self) -> super::vals::Protreg13 {
                let val = (self.0 >> 13usize) & 0x01;
                super::vals::Protreg13::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 13."]
            #[inline(always)]
            pub const fn set_protreg13(&mut self, val: super::vals::Protreg13) {
                self.0 =
                    (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
            }
            #[doc = "Protection enable for region 14."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg14(&self) -> super::vals::Protreg14 {
                let val = (self.0 >> 14usize) & 0x01;
                super::vals::Protreg14::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 14."]
            #[inline(always)]
            pub const fn set_protreg14(&mut self, val: super::vals::Protreg14) {
                self.0 =
                    (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
            }
            #[doc = "Protection enable for region 15."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg15(&self) -> super::vals::Protreg15 {
                let val = (self.0 >> 15usize) & 0x01;
                super::vals::Protreg15::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 15."]
            #[inline(always)]
            pub const fn set_protreg15(&mut self, val: super::vals::Protreg15) {
                self.0 =
                    (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
            }
            #[doc = "Protection enable for region 16."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg16(&self) -> super::vals::Protreg16 {
                let val = (self.0 >> 16usize) & 0x01;
                super::vals::Protreg16::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 16."]
            #[inline(always)]
            pub const fn set_protreg16(&mut self, val: super::vals::Protreg16) {
                self.0 =
                    (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
            }
            #[doc = "Protection enable for region 17."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg17(&self) -> super::vals::Protreg17 {
                let val = (self.0 >> 17usize) & 0x01;
                super::vals::Protreg17::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 17."]
            #[inline(always)]
            pub const fn set_protreg17(&mut self, val: super::vals::Protreg17) {
                self.0 =
                    (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
            }
            #[doc = "Protection enable for region 18."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg18(&self) -> super::vals::Protreg18 {
                let val = (self.0 >> 18usize) & 0x01;
                super::vals::Protreg18::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 18."]
            #[inline(always)]
            pub const fn set_protreg18(&mut self, val: super::vals::Protreg18) {
                self.0 =
                    (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
            }
            #[doc = "Protection enable for region 19."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg19(&self) -> super::vals::Protreg19 {
                let val = (self.0 >> 19usize) & 0x01;
                super::vals::Protreg19::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 19."]
            #[inline(always)]
            pub const fn set_protreg19(&mut self, val: super::vals::Protreg19) {
                self.0 =
                    (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
            }
            #[doc = "Protection enable for region 20."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg20(&self) -> super::vals::Protreg20 {
                let val = (self.0 >> 20usize) & 0x01;
                super::vals::Protreg20::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 20."]
            #[inline(always)]
            pub const fn set_protreg20(&mut self, val: super::vals::Protreg20) {
                self.0 =
                    (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
            }
            #[doc = "Protection enable for region 21."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg21(&self) -> super::vals::Protreg21 {
                let val = (self.0 >> 21usize) & 0x01;
                super::vals::Protreg21::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 21."]
            #[inline(always)]
            pub const fn set_protreg21(&mut self, val: super::vals::Protreg21) {
                self.0 =
                    (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
            }
            #[doc = "Protection enable for region 22."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg22(&self) -> super::vals::Protreg22 {
                let val = (self.0 >> 22usize) & 0x01;
                super::vals::Protreg22::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 22."]
            #[inline(always)]
            pub const fn set_protreg22(&mut self, val: super::vals::Protreg22) {
                self.0 =
                    (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
            }
            #[doc = "Protection enable for region 23."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg23(&self) -> super::vals::Protreg23 {
                let val = (self.0 >> 23usize) & 0x01;
                super::vals::Protreg23::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 23."]
            #[inline(always)]
            pub const fn set_protreg23(&mut self, val: super::vals::Protreg23) {
                self.0 =
                    (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
            }
            #[doc = "Protection enable for region 24."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg24(&self) -> super::vals::Protreg24 {
                let val = (self.0 >> 24usize) & 0x01;
                super::vals::Protreg24::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 24."]
            #[inline(always)]
            pub const fn set_protreg24(&mut self, val: super::vals::Protreg24) {
                self.0 =
                    (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
            }
            #[doc = "Protection enable for region 25."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg25(&self) -> super::vals::Protreg25 {
                let val = (self.0 >> 25usize) & 0x01;
                super::vals::Protreg25::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 25."]
            #[inline(always)]
            pub const fn set_protreg25(&mut self, val: super::vals::Protreg25) {
                self.0 =
                    (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
            }
            #[doc = "Protection enable for region 26."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg26(&self) -> super::vals::Protreg26 {
                let val = (self.0 >> 26usize) & 0x01;
                super::vals::Protreg26::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 26."]
            #[inline(always)]
            pub const fn set_protreg26(&mut self, val: super::vals::Protreg26) {
                self.0 =
                    (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
            }
            #[doc = "Protection enable for region 27."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg27(&self) -> super::vals::Protreg27 {
                let val = (self.0 >> 27usize) & 0x01;
                super::vals::Protreg27::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 27."]
            #[inline(always)]
            pub const fn set_protreg27(&mut self, val: super::vals::Protreg27) {
                self.0 =
                    (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
            }
            #[doc = "Protection enable for region 28."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg28(&self) -> super::vals::Protreg28 {
                let val = (self.0 >> 28usize) & 0x01;
                super::vals::Protreg28::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 28."]
            #[inline(always)]
            pub const fn set_protreg28(&mut self, val: super::vals::Protreg28) {
                self.0 =
                    (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
            }
            #[doc = "Protection enable for region 29."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg29(&self) -> super::vals::Protreg29 {
                let val = (self.0 >> 29usize) & 0x01;
                super::vals::Protreg29::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 29."]
            #[inline(always)]
            pub const fn set_protreg29(&mut self, val: super::vals::Protreg29) {
                self.0 =
                    (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
            }
            #[doc = "Protection enable for region 30."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg30(&self) -> super::vals::Protreg30 {
                let val = (self.0 >> 30usize) & 0x01;
                super::vals::Protreg30::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 30."]
            #[inline(always)]
            pub const fn set_protreg30(&mut self, val: super::vals::Protreg30) {
                self.0 =
                    (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
            }
            #[doc = "Protection enable for region 31."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg31(&self) -> super::vals::Protreg31 {
                let val = (self.0 >> 31usize) & 0x01;
                super::vals::Protreg31::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 31."]
            #[inline(always)]
            pub const fn set_protreg31(&mut self, val: super::vals::Protreg31) {
                self.0 =
                    (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
            }
        }
        impl Default for Protenset0 {
            #[inline(always)]
            fn default() -> Protenset0 {
                Protenset0(0)
            }
        }
        impl core::fmt::Debug for Protenset0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Protenset0")
                    .field("protreg0", &self.protreg0())
                    .field("protreg1", &self.protreg1())
                    .field("protreg2", &self.protreg2())
                    .field("protreg3", &self.protreg3())
                    .field("protreg4", &self.protreg4())
                    .field("protreg5", &self.protreg5())
                    .field("protreg6", &self.protreg6())
                    .field("protreg7", &self.protreg7())
                    .field("protreg8", &self.protreg8())
                    .field("protreg9", &self.protreg9())
                    .field("protreg10", &self.protreg10())
                    .field("protreg11", &self.protreg11())
                    .field("protreg12", &self.protreg12())
                    .field("protreg13", &self.protreg13())
                    .field("protreg14", &self.protreg14())
                    .field("protreg15", &self.protreg15())
                    .field("protreg16", &self.protreg16())
                    .field("protreg17", &self.protreg17())
                    .field("protreg18", &self.protreg18())
                    .field("protreg19", &self.protreg19())
                    .field("protreg20", &self.protreg20())
                    .field("protreg21", &self.protreg21())
                    .field("protreg22", &self.protreg22())
                    .field("protreg23", &self.protreg23())
                    .field("protreg24", &self.protreg24())
                    .field("protreg25", &self.protreg25())
                    .field("protreg26", &self.protreg26())
                    .field("protreg27", &self.protreg27())
                    .field("protreg28", &self.protreg28())
                    .field("protreg29", &self.protreg29())
                    .field("protreg30", &self.protreg30())
                    .field("protreg31", &self.protreg31())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Protenset0 {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Protenset0 {{ protreg0: {:?}, protreg1: {:?}, protreg2: {:?}, protreg3: {:?}, protreg4: {:?}, protreg5: {:?}, protreg6: {:?}, protreg7: {:?}, protreg8: {:?}, protreg9: {:?}, protreg10: {:?}, protreg11: {:?}, protreg12: {:?}, protreg13: {:?}, protreg14: {:?}, protreg15: {:?}, protreg16: {:?}, protreg17: {:?}, protreg18: {:?}, protreg19: {:?}, protreg20: {:?}, protreg21: {:?}, protreg22: {:?}, protreg23: {:?}, protreg24: {:?}, protreg25: {:?}, protreg26: {:?}, protreg27: {:?}, protreg28: {:?}, protreg29: {:?}, protreg30: {:?}, protreg31: {:?} }}" , self . protreg0 () , self . protreg1 () , self . protreg2 () , self . protreg3 () , self . protreg4 () , self . protreg5 () , self . protreg6 () , self . protreg7 () , self . protreg8 () , self . protreg9 () , self . protreg10 () , self . protreg11 () , self . protreg12 () , self . protreg13 () , self . protreg14 () , self . protreg15 () , self . protreg16 () , self . protreg17 () , self . protreg18 () , self . protreg19 () , self . protreg20 () , self . protreg21 () , self . protreg22 () , self . protreg23 () , self . protreg24 () , self . protreg25 () , self . protreg26 () , self . protreg27 () , self . protreg28 () , self . protreg29 () , self . protreg30 () , self . protreg31 ())
            }
        }
        #[doc = "Erase and write protection bit enable set register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Protenset1(pub u32);
        impl Protenset1 {
            #[doc = "Protection enable for region 32."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg32(&self) -> super::vals::Protreg32 {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Protreg32::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 32."]
            #[inline(always)]
            pub const fn set_protreg32(&mut self, val: super::vals::Protreg32) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Protection enable for region 33."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg33(&self) -> super::vals::Protreg33 {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Protreg33::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 33."]
            #[inline(always)]
            pub const fn set_protreg33(&mut self, val: super::vals::Protreg33) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Protection enable for region 34."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg34(&self) -> super::vals::Protreg34 {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Protreg34::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 34."]
            #[inline(always)]
            pub const fn set_protreg34(&mut self, val: super::vals::Protreg34) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
            #[doc = "Protection enable for region 35."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg35(&self) -> super::vals::Protreg35 {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Protreg35::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 35."]
            #[inline(always)]
            pub const fn set_protreg35(&mut self, val: super::vals::Protreg35) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
            }
            #[doc = "Protection enable for region 36."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg36(&self) -> super::vals::Protreg36 {
                let val = (self.0 >> 4usize) & 0x01;
                super::vals::Protreg36::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 36."]
            #[inline(always)]
            pub const fn set_protreg36(&mut self, val: super::vals::Protreg36) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
            }
            #[doc = "Protection enable for region 37."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg37(&self) -> super::vals::Protreg37 {
                let val = (self.0 >> 5usize) & 0x01;
                super::vals::Protreg37::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 37."]
            #[inline(always)]
            pub const fn set_protreg37(&mut self, val: super::vals::Protreg37) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
            }
            #[doc = "Protection enable for region 38."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg38(&self) -> super::vals::Protreg38 {
                let val = (self.0 >> 6usize) & 0x01;
                super::vals::Protreg38::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 38."]
            #[inline(always)]
            pub const fn set_protreg38(&mut self, val: super::vals::Protreg38) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
            }
            #[doc = "Protection enable for region 39."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg39(&self) -> super::vals::Protreg39 {
                let val = (self.0 >> 7usize) & 0x01;
                super::vals::Protreg39::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 39."]
            #[inline(always)]
            pub const fn set_protreg39(&mut self, val: super::vals::Protreg39) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
            }
            #[doc = "Protection enable for region 40."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg40(&self) -> super::vals::Protreg40 {
                let val = (self.0 >> 8usize) & 0x01;
                super::vals::Protreg40::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 40."]
            #[inline(always)]
            pub const fn set_protreg40(&mut self, val: super::vals::Protreg40) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
            }
            #[doc = "Protection enable for region 41."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg41(&self) -> super::vals::Protreg41 {
                let val = (self.0 >> 9usize) & 0x01;
                super::vals::Protreg41::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 41."]
            #[inline(always)]
            pub const fn set_protreg41(&mut self, val: super::vals::Protreg41) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
            }
            #[doc = "Protection enable for region 42."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg42(&self) -> super::vals::Protreg42 {
                let val = (self.0 >> 10usize) & 0x01;
                super::vals::Protreg42::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 42."]
            #[inline(always)]
            pub const fn set_protreg42(&mut self, val: super::vals::Protreg42) {
                self.0 =
                    (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
            }
            #[doc = "Protection enable for region 43."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg43(&self) -> super::vals::Protreg43 {
                let val = (self.0 >> 11usize) & 0x01;
                super::vals::Protreg43::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 43."]
            #[inline(always)]
            pub const fn set_protreg43(&mut self, val: super::vals::Protreg43) {
                self.0 =
                    (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
            }
            #[doc = "Protection enable for region 44."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg44(&self) -> super::vals::Protreg44 {
                let val = (self.0 >> 12usize) & 0x01;
                super::vals::Protreg44::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 44."]
            #[inline(always)]
            pub const fn set_protreg44(&mut self, val: super::vals::Protreg44) {
                self.0 =
                    (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
            }
            #[doc = "Protection enable for region 45."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg45(&self) -> super::vals::Protreg45 {
                let val = (self.0 >> 13usize) & 0x01;
                super::vals::Protreg45::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 45."]
            #[inline(always)]
            pub const fn set_protreg45(&mut self, val: super::vals::Protreg45) {
                self.0 =
                    (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
            }
            #[doc = "Protection enable for region 46."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg46(&self) -> super::vals::Protreg46 {
                let val = (self.0 >> 14usize) & 0x01;
                super::vals::Protreg46::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 46."]
            #[inline(always)]
            pub const fn set_protreg46(&mut self, val: super::vals::Protreg46) {
                self.0 =
                    (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
            }
            #[doc = "Protection enable for region 47."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg47(&self) -> super::vals::Protreg47 {
                let val = (self.0 >> 15usize) & 0x01;
                super::vals::Protreg47::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 47."]
            #[inline(always)]
            pub const fn set_protreg47(&mut self, val: super::vals::Protreg47) {
                self.0 =
                    (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
            }
            #[doc = "Protection enable for region 48."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg48(&self) -> super::vals::Protreg48 {
                let val = (self.0 >> 16usize) & 0x01;
                super::vals::Protreg48::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 48."]
            #[inline(always)]
            pub const fn set_protreg48(&mut self, val: super::vals::Protreg48) {
                self.0 =
                    (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
            }
            #[doc = "Protection enable for region 49."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg49(&self) -> super::vals::Protreg49 {
                let val = (self.0 >> 17usize) & 0x01;
                super::vals::Protreg49::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 49."]
            #[inline(always)]
            pub const fn set_protreg49(&mut self, val: super::vals::Protreg49) {
                self.0 =
                    (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
            }
            #[doc = "Protection enable for region 50."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg50(&self) -> super::vals::Protreg50 {
                let val = (self.0 >> 18usize) & 0x01;
                super::vals::Protreg50::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 50."]
            #[inline(always)]
            pub const fn set_protreg50(&mut self, val: super::vals::Protreg50) {
                self.0 =
                    (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
            }
            #[doc = "Protection enable for region 51."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg51(&self) -> super::vals::Protreg51 {
                let val = (self.0 >> 19usize) & 0x01;
                super::vals::Protreg51::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 51."]
            #[inline(always)]
            pub const fn set_protreg51(&mut self, val: super::vals::Protreg51) {
                self.0 =
                    (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
            }
            #[doc = "Protection enable for region 52."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg52(&self) -> super::vals::Protreg52 {
                let val = (self.0 >> 20usize) & 0x01;
                super::vals::Protreg52::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 52."]
            #[inline(always)]
            pub const fn set_protreg52(&mut self, val: super::vals::Protreg52) {
                self.0 =
                    (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
            }
            #[doc = "Protection enable for region 53."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg53(&self) -> super::vals::Protreg53 {
                let val = (self.0 >> 21usize) & 0x01;
                super::vals::Protreg53::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 53."]
            #[inline(always)]
            pub const fn set_protreg53(&mut self, val: super::vals::Protreg53) {
                self.0 =
                    (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
            }
            #[doc = "Protection enable for region 54."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg54(&self) -> super::vals::Protreg54 {
                let val = (self.0 >> 22usize) & 0x01;
                super::vals::Protreg54::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 54."]
            #[inline(always)]
            pub const fn set_protreg54(&mut self, val: super::vals::Protreg54) {
                self.0 =
                    (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
            }
            #[doc = "Protection enable for region 55."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg55(&self) -> super::vals::Protreg55 {
                let val = (self.0 >> 23usize) & 0x01;
                super::vals::Protreg55::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 55."]
            #[inline(always)]
            pub const fn set_protreg55(&mut self, val: super::vals::Protreg55) {
                self.0 =
                    (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
            }
            #[doc = "Protection enable for region 56."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg56(&self) -> super::vals::Protreg56 {
                let val = (self.0 >> 24usize) & 0x01;
                super::vals::Protreg56::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 56."]
            #[inline(always)]
            pub const fn set_protreg56(&mut self, val: super::vals::Protreg56) {
                self.0 =
                    (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
            }
            #[doc = "Protection enable for region 57."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg57(&self) -> super::vals::Protreg57 {
                let val = (self.0 >> 25usize) & 0x01;
                super::vals::Protreg57::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 57."]
            #[inline(always)]
            pub const fn set_protreg57(&mut self, val: super::vals::Protreg57) {
                self.0 =
                    (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
            }
            #[doc = "Protection enable for region 58."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg58(&self) -> super::vals::Protreg58 {
                let val = (self.0 >> 26usize) & 0x01;
                super::vals::Protreg58::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 58."]
            #[inline(always)]
            pub const fn set_protreg58(&mut self, val: super::vals::Protreg58) {
                self.0 =
                    (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
            }
            #[doc = "Protection enable for region 59."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg59(&self) -> super::vals::Protreg59 {
                let val = (self.0 >> 27usize) & 0x01;
                super::vals::Protreg59::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 59."]
            #[inline(always)]
            pub const fn set_protreg59(&mut self, val: super::vals::Protreg59) {
                self.0 =
                    (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
            }
            #[doc = "Protection enable for region 60."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg60(&self) -> super::vals::Protreg60 {
                let val = (self.0 >> 28usize) & 0x01;
                super::vals::Protreg60::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 60."]
            #[inline(always)]
            pub const fn set_protreg60(&mut self, val: super::vals::Protreg60) {
                self.0 =
                    (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
            }
            #[doc = "Protection enable for region 61."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg61(&self) -> super::vals::Protreg61 {
                let val = (self.0 >> 29usize) & 0x01;
                super::vals::Protreg61::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 61."]
            #[inline(always)]
            pub const fn set_protreg61(&mut self, val: super::vals::Protreg61) {
                self.0 =
                    (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
            }
            #[doc = "Protection enable for region 62."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg62(&self) -> super::vals::Protreg62 {
                let val = (self.0 >> 30usize) & 0x01;
                super::vals::Protreg62::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 62."]
            #[inline(always)]
            pub const fn set_protreg62(&mut self, val: super::vals::Protreg62) {
                self.0 =
                    (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
            }
            #[doc = "Protection enable for region 63."]
            #[must_use]
            #[inline(always)]
            pub const fn protreg63(&self) -> super::vals::Protreg63 {
                let val = (self.0 >> 31usize) & 0x01;
                super::vals::Protreg63::from_bits(val as u8)
            }
            #[doc = "Protection enable for region 63."]
            #[inline(always)]
            pub const fn set_protreg63(&mut self, val: super::vals::Protreg63) {
                self.0 =
                    (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
            }
        }
        impl Default for Protenset1 {
            #[inline(always)]
            fn default() -> Protenset1 {
                Protenset1(0)
            }
        }
        impl core::fmt::Debug for Protenset1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Protenset1")
                    .field("protreg32", &self.protreg32())
                    .field("protreg33", &self.protreg33())
                    .field("protreg34", &self.protreg34())
                    .field("protreg35", &self.protreg35())
                    .field("protreg36", &self.protreg36())
                    .field("protreg37", &self.protreg37())
                    .field("protreg38", &self.protreg38())
                    .field("protreg39", &self.protreg39())
                    .field("protreg40", &self.protreg40())
                    .field("protreg41", &self.protreg41())
                    .field("protreg42", &self.protreg42())
                    .field("protreg43", &self.protreg43())
                    .field("protreg44", &self.protreg44())
                    .field("protreg45", &self.protreg45())
                    .field("protreg46", &self.protreg46())
                    .field("protreg47", &self.protreg47())
                    .field("protreg48", &self.protreg48())
                    .field("protreg49", &self.protreg49())
                    .field("protreg50", &self.protreg50())
                    .field("protreg51", &self.protreg51())
                    .field("protreg52", &self.protreg52())
                    .field("protreg53", &self.protreg53())
                    .field("protreg54", &self.protreg54())
                    .field("protreg55", &self.protreg55())
                    .field("protreg56", &self.protreg56())
                    .field("protreg57", &self.protreg57())
                    .field("protreg58", &self.protreg58())
                    .field("protreg59", &self.protreg59())
                    .field("protreg60", &self.protreg60())
                    .field("protreg61", &self.protreg61())
                    .field("protreg62", &self.protreg62())
                    .field("protreg63", &self.protreg63())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Protenset1 {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Protenset1 {{ protreg32: {:?}, protreg33: {:?}, protreg34: {:?}, protreg35: {:?}, protreg36: {:?}, protreg37: {:?}, protreg38: {:?}, protreg39: {:?}, protreg40: {:?}, protreg41: {:?}, protreg42: {:?}, protreg43: {:?}, protreg44: {:?}, protreg45: {:?}, protreg46: {:?}, protreg47: {:?}, protreg48: {:?}, protreg49: {:?}, protreg50: {:?}, protreg51: {:?}, protreg52: {:?}, protreg53: {:?}, protreg54: {:?}, protreg55: {:?}, protreg56: {:?}, protreg57: {:?}, protreg58: {:?}, protreg59: {:?}, protreg60: {:?}, protreg61: {:?}, protreg62: {:?}, protreg63: {:?} }}" , self . protreg32 () , self . protreg33 () , self . protreg34 () , self . protreg35 () , self . protreg36 () , self . protreg37 () , self . protreg38 () , self . protreg39 () , self . protreg40 () , self . protreg41 () , self . protreg42 () , self . protreg43 () , self . protreg44 () , self . protreg45 () , self . protreg46 () , self . protreg47 () , self . protreg48 () , self . protreg49 () , self . protreg50 () , self . protreg51 () , self . protreg52 () , self . protreg53 () , self . protreg54 () , self . protreg55 () , self . protreg56 () , self . protreg57 () , self . protreg58 () , self . protreg59 () , self . protreg60 () , self . protreg61 () , self . protreg62 () , self . protreg63 ())
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Adc {
            #[doc = "Peripheral configured in region 1."]
            IN_REGION1 = 0x0,
            #[doc = "Peripheral configured in region 0."]
            IN_REGION0 = 0x01,
        }
        impl Adc {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Adc {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Adc {
            #[inline(always)]
            fn from(val: u8) -> Adc {
                Adc::from_bits(val)
            }
        }
        impl From<Adc> for u8 {
            #[inline(always)]
            fn from(val: Adc) -> u8 {
                Adc::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum CcmAar {
            #[doc = "Peripheral configured in region 1."]
            IN_REGION1 = 0x0,
            #[doc = "Peripheral configured in region 0."]
            IN_REGION0 = 0x01,
        }
        impl CcmAar {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> CcmAar {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for CcmAar {
            #[inline(always)]
            fn from(val: u8) -> CcmAar {
                CcmAar::from_bits(val)
            }
        }
        impl From<CcmAar> for u8 {
            #[inline(always)]
            fn from(val: CcmAar) -> u8 {
                CcmAar::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Disableindebug {
            #[doc = "Protection enabled."]
            ENABLED = 0x0,
            #[doc = "Protection disabled."]
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
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Ecb {
            #[doc = "Peripheral configured in region 1."]
            IN_REGION1 = 0x0,
            #[doc = "Peripheral configured in region 0."]
            IN_REGION0 = 0x01,
        }
        impl Ecb {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Ecb {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Ecb {
            #[inline(always)]
            fn from(val: u8) -> Ecb {
                Ecb::from_bits(val)
            }
        }
        impl From<Ecb> for u8 {
            #[inline(always)]
            fn from(val: Ecb) -> u8 {
                Ecb::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Gpiote {
            #[doc = "Peripheral configured in region 1."]
            IN_REGION1 = 0x0,
            #[doc = "Peripheral configured in region 0."]
            IN_REGION0 = 0x01,
        }
        impl Gpiote {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Gpiote {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Gpiote {
            #[inline(always)]
            fn from(val: u8) -> Gpiote {
                Gpiote::from_bits(val)
            }
        }
        impl From<Gpiote> for u8 {
            #[inline(always)]
            fn from(val: Gpiote) -> u8 {
                Gpiote::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Lpcomp {
            #[doc = "Peripheral configured in region 1."]
            IN_REGION1 = 0x0,
            #[doc = "Peripheral configured in region 0."]
            IN_REGION0 = 0x01,
        }
        impl Lpcomp {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Lpcomp {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Lpcomp {
            #[inline(always)]
            fn from(val: u8) -> Lpcomp {
                Lpcomp::from_bits(val)
            }
        }
        impl From<Lpcomp> for u8 {
            #[inline(always)]
            fn from(val: Lpcomp) -> u8 {
                Lpcomp::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Nvmc {
            #[doc = "Peripheral configured in region 1."]
            IN_REGION1 = 0x0,
            #[doc = "Peripheral configured in region 0."]
            IN_REGION0 = 0x01,
        }
        impl Nvmc {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Nvmc {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Nvmc {
            #[inline(always)]
            fn from(val: u8) -> Nvmc {
                Nvmc::from_bits(val)
            }
        }
        impl From<Nvmc> for u8 {
            #[inline(always)]
            fn from(val: Nvmc) -> u8 {
                Nvmc::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum PowerClock {
            #[doc = "Peripheral configured in region 1."]
            IN_REGION1 = 0x0,
            #[doc = "Peripheral configured in region 0."]
            IN_REGION0 = 0x01,
        }
        impl PowerClock {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> PowerClock {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for PowerClock {
            #[inline(always)]
            fn from(val: u8) -> PowerClock {
                PowerClock::from_bits(val)
            }
        }
        impl From<PowerClock> for u8 {
            #[inline(always)]
            fn from(val: PowerClock) -> u8 {
                PowerClock::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Ppi {
            #[doc = "Peripheral configured in region 1."]
            IN_REGION1 = 0x0,
            #[doc = "Peripheral configured in region 0."]
            IN_REGION0 = 0x01,
        }
        impl Ppi {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Ppi {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Ppi {
            #[inline(always)]
            fn from(val: u8) -> Ppi {
                Ppi::from_bits(val)
            }
        }
        impl From<Ppi> for u8 {
            #[inline(always)]
            fn from(val: Ppi) -> u8 {
                Ppi::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protblocksize {
            #[doc = "Erase and write protection block size is 4k."]
            _4K = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Protblocksize {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protblocksize {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protblocksize {
            #[inline(always)]
            fn from(val: u8) -> Protblocksize {
                Protblocksize::from_bits(val)
            }
        }
        impl From<Protblocksize> for u8 {
            #[inline(always)]
            fn from(val: Protblocksize) -> u8 {
                Protblocksize::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg0 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg0 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg0 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg0 {
            #[inline(always)]
            fn from(val: u8) -> Protreg0 {
                Protreg0::from_bits(val)
            }
        }
        impl From<Protreg0> for u8 {
            #[inline(always)]
            fn from(val: Protreg0) -> u8 {
                Protreg0::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg1 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg1 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg1 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg1 {
            #[inline(always)]
            fn from(val: u8) -> Protreg1 {
                Protreg1::from_bits(val)
            }
        }
        impl From<Protreg1> for u8 {
            #[inline(always)]
            fn from(val: Protreg1) -> u8 {
                Protreg1::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg10 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg10 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg10 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg10 {
            #[inline(always)]
            fn from(val: u8) -> Protreg10 {
                Protreg10::from_bits(val)
            }
        }
        impl From<Protreg10> for u8 {
            #[inline(always)]
            fn from(val: Protreg10) -> u8 {
                Protreg10::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg11 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg11 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg11 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg11 {
            #[inline(always)]
            fn from(val: u8) -> Protreg11 {
                Protreg11::from_bits(val)
            }
        }
        impl From<Protreg11> for u8 {
            #[inline(always)]
            fn from(val: Protreg11) -> u8 {
                Protreg11::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg12 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg12 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg12 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg12 {
            #[inline(always)]
            fn from(val: u8) -> Protreg12 {
                Protreg12::from_bits(val)
            }
        }
        impl From<Protreg12> for u8 {
            #[inline(always)]
            fn from(val: Protreg12) -> u8 {
                Protreg12::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg13 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg13 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg13 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg13 {
            #[inline(always)]
            fn from(val: u8) -> Protreg13 {
                Protreg13::from_bits(val)
            }
        }
        impl From<Protreg13> for u8 {
            #[inline(always)]
            fn from(val: Protreg13) -> u8 {
                Protreg13::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg14 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg14 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg14 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg14 {
            #[inline(always)]
            fn from(val: u8) -> Protreg14 {
                Protreg14::from_bits(val)
            }
        }
        impl From<Protreg14> for u8 {
            #[inline(always)]
            fn from(val: Protreg14) -> u8 {
                Protreg14::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg15 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg15 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg15 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg15 {
            #[inline(always)]
            fn from(val: u8) -> Protreg15 {
                Protreg15::from_bits(val)
            }
        }
        impl From<Protreg15> for u8 {
            #[inline(always)]
            fn from(val: Protreg15) -> u8 {
                Protreg15::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg16 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg16 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg16 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg16 {
            #[inline(always)]
            fn from(val: u8) -> Protreg16 {
                Protreg16::from_bits(val)
            }
        }
        impl From<Protreg16> for u8 {
            #[inline(always)]
            fn from(val: Protreg16) -> u8 {
                Protreg16::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg17 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg17 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg17 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg17 {
            #[inline(always)]
            fn from(val: u8) -> Protreg17 {
                Protreg17::from_bits(val)
            }
        }
        impl From<Protreg17> for u8 {
            #[inline(always)]
            fn from(val: Protreg17) -> u8 {
                Protreg17::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg18 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg18 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg18 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg18 {
            #[inline(always)]
            fn from(val: u8) -> Protreg18 {
                Protreg18::from_bits(val)
            }
        }
        impl From<Protreg18> for u8 {
            #[inline(always)]
            fn from(val: Protreg18) -> u8 {
                Protreg18::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg19 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg19 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg19 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg19 {
            #[inline(always)]
            fn from(val: u8) -> Protreg19 {
                Protreg19::from_bits(val)
            }
        }
        impl From<Protreg19> for u8 {
            #[inline(always)]
            fn from(val: Protreg19) -> u8 {
                Protreg19::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg2 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg2 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg2 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg2 {
            #[inline(always)]
            fn from(val: u8) -> Protreg2 {
                Protreg2::from_bits(val)
            }
        }
        impl From<Protreg2> for u8 {
            #[inline(always)]
            fn from(val: Protreg2) -> u8 {
                Protreg2::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg20 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg20 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg20 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg20 {
            #[inline(always)]
            fn from(val: u8) -> Protreg20 {
                Protreg20::from_bits(val)
            }
        }
        impl From<Protreg20> for u8 {
            #[inline(always)]
            fn from(val: Protreg20) -> u8 {
                Protreg20::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg21 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg21 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg21 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg21 {
            #[inline(always)]
            fn from(val: u8) -> Protreg21 {
                Protreg21::from_bits(val)
            }
        }
        impl From<Protreg21> for u8 {
            #[inline(always)]
            fn from(val: Protreg21) -> u8 {
                Protreg21::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg22 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg22 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg22 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg22 {
            #[inline(always)]
            fn from(val: u8) -> Protreg22 {
                Protreg22::from_bits(val)
            }
        }
        impl From<Protreg22> for u8 {
            #[inline(always)]
            fn from(val: Protreg22) -> u8 {
                Protreg22::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg23 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg23 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg23 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg23 {
            #[inline(always)]
            fn from(val: u8) -> Protreg23 {
                Protreg23::from_bits(val)
            }
        }
        impl From<Protreg23> for u8 {
            #[inline(always)]
            fn from(val: Protreg23) -> u8 {
                Protreg23::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg24 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg24 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg24 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg24 {
            #[inline(always)]
            fn from(val: u8) -> Protreg24 {
                Protreg24::from_bits(val)
            }
        }
        impl From<Protreg24> for u8 {
            #[inline(always)]
            fn from(val: Protreg24) -> u8 {
                Protreg24::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg25 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg25 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg25 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg25 {
            #[inline(always)]
            fn from(val: u8) -> Protreg25 {
                Protreg25::from_bits(val)
            }
        }
        impl From<Protreg25> for u8 {
            #[inline(always)]
            fn from(val: Protreg25) -> u8 {
                Protreg25::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg26 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg26 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg26 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg26 {
            #[inline(always)]
            fn from(val: u8) -> Protreg26 {
                Protreg26::from_bits(val)
            }
        }
        impl From<Protreg26> for u8 {
            #[inline(always)]
            fn from(val: Protreg26) -> u8 {
                Protreg26::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg27 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg27 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg27 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg27 {
            #[inline(always)]
            fn from(val: u8) -> Protreg27 {
                Protreg27::from_bits(val)
            }
        }
        impl From<Protreg27> for u8 {
            #[inline(always)]
            fn from(val: Protreg27) -> u8 {
                Protreg27::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg28 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg28 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg28 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg28 {
            #[inline(always)]
            fn from(val: u8) -> Protreg28 {
                Protreg28::from_bits(val)
            }
        }
        impl From<Protreg28> for u8 {
            #[inline(always)]
            fn from(val: Protreg28) -> u8 {
                Protreg28::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg29 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg29 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg29 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg29 {
            #[inline(always)]
            fn from(val: u8) -> Protreg29 {
                Protreg29::from_bits(val)
            }
        }
        impl From<Protreg29> for u8 {
            #[inline(always)]
            fn from(val: Protreg29) -> u8 {
                Protreg29::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg3 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg3 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg3 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg3 {
            #[inline(always)]
            fn from(val: u8) -> Protreg3 {
                Protreg3::from_bits(val)
            }
        }
        impl From<Protreg3> for u8 {
            #[inline(always)]
            fn from(val: Protreg3) -> u8 {
                Protreg3::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg30 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg30 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg30 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg30 {
            #[inline(always)]
            fn from(val: u8) -> Protreg30 {
                Protreg30::from_bits(val)
            }
        }
        impl From<Protreg30> for u8 {
            #[inline(always)]
            fn from(val: Protreg30) -> u8 {
                Protreg30::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg31 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg31 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg31 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg31 {
            #[inline(always)]
            fn from(val: u8) -> Protreg31 {
                Protreg31::from_bits(val)
            }
        }
        impl From<Protreg31> for u8 {
            #[inline(always)]
            fn from(val: Protreg31) -> u8 {
                Protreg31::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg32 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg32 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg32 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg32 {
            #[inline(always)]
            fn from(val: u8) -> Protreg32 {
                Protreg32::from_bits(val)
            }
        }
        impl From<Protreg32> for u8 {
            #[inline(always)]
            fn from(val: Protreg32) -> u8 {
                Protreg32::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg33 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg33 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg33 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg33 {
            #[inline(always)]
            fn from(val: u8) -> Protreg33 {
                Protreg33::from_bits(val)
            }
        }
        impl From<Protreg33> for u8 {
            #[inline(always)]
            fn from(val: Protreg33) -> u8 {
                Protreg33::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg34 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg34 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg34 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg34 {
            #[inline(always)]
            fn from(val: u8) -> Protreg34 {
                Protreg34::from_bits(val)
            }
        }
        impl From<Protreg34> for u8 {
            #[inline(always)]
            fn from(val: Protreg34) -> u8 {
                Protreg34::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg35 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg35 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg35 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg35 {
            #[inline(always)]
            fn from(val: u8) -> Protreg35 {
                Protreg35::from_bits(val)
            }
        }
        impl From<Protreg35> for u8 {
            #[inline(always)]
            fn from(val: Protreg35) -> u8 {
                Protreg35::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg36 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg36 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg36 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg36 {
            #[inline(always)]
            fn from(val: u8) -> Protreg36 {
                Protreg36::from_bits(val)
            }
        }
        impl From<Protreg36> for u8 {
            #[inline(always)]
            fn from(val: Protreg36) -> u8 {
                Protreg36::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg37 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg37 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg37 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg37 {
            #[inline(always)]
            fn from(val: u8) -> Protreg37 {
                Protreg37::from_bits(val)
            }
        }
        impl From<Protreg37> for u8 {
            #[inline(always)]
            fn from(val: Protreg37) -> u8 {
                Protreg37::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg38 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg38 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg38 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg38 {
            #[inline(always)]
            fn from(val: u8) -> Protreg38 {
                Protreg38::from_bits(val)
            }
        }
        impl From<Protreg38> for u8 {
            #[inline(always)]
            fn from(val: Protreg38) -> u8 {
                Protreg38::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg39 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg39 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg39 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg39 {
            #[inline(always)]
            fn from(val: u8) -> Protreg39 {
                Protreg39::from_bits(val)
            }
        }
        impl From<Protreg39> for u8 {
            #[inline(always)]
            fn from(val: Protreg39) -> u8 {
                Protreg39::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg4 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg4 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg4 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg4 {
            #[inline(always)]
            fn from(val: u8) -> Protreg4 {
                Protreg4::from_bits(val)
            }
        }
        impl From<Protreg4> for u8 {
            #[inline(always)]
            fn from(val: Protreg4) -> u8 {
                Protreg4::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg40 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg40 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg40 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg40 {
            #[inline(always)]
            fn from(val: u8) -> Protreg40 {
                Protreg40::from_bits(val)
            }
        }
        impl From<Protreg40> for u8 {
            #[inline(always)]
            fn from(val: Protreg40) -> u8 {
                Protreg40::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg41 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg41 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg41 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg41 {
            #[inline(always)]
            fn from(val: u8) -> Protreg41 {
                Protreg41::from_bits(val)
            }
        }
        impl From<Protreg41> for u8 {
            #[inline(always)]
            fn from(val: Protreg41) -> u8 {
                Protreg41::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg42 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg42 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg42 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg42 {
            #[inline(always)]
            fn from(val: u8) -> Protreg42 {
                Protreg42::from_bits(val)
            }
        }
        impl From<Protreg42> for u8 {
            #[inline(always)]
            fn from(val: Protreg42) -> u8 {
                Protreg42::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg43 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg43 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg43 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg43 {
            #[inline(always)]
            fn from(val: u8) -> Protreg43 {
                Protreg43::from_bits(val)
            }
        }
        impl From<Protreg43> for u8 {
            #[inline(always)]
            fn from(val: Protreg43) -> u8 {
                Protreg43::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg44 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg44 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg44 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg44 {
            #[inline(always)]
            fn from(val: u8) -> Protreg44 {
                Protreg44::from_bits(val)
            }
        }
        impl From<Protreg44> for u8 {
            #[inline(always)]
            fn from(val: Protreg44) -> u8 {
                Protreg44::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg45 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg45 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg45 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg45 {
            #[inline(always)]
            fn from(val: u8) -> Protreg45 {
                Protreg45::from_bits(val)
            }
        }
        impl From<Protreg45> for u8 {
            #[inline(always)]
            fn from(val: Protreg45) -> u8 {
                Protreg45::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg46 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg46 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg46 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg46 {
            #[inline(always)]
            fn from(val: u8) -> Protreg46 {
                Protreg46::from_bits(val)
            }
        }
        impl From<Protreg46> for u8 {
            #[inline(always)]
            fn from(val: Protreg46) -> u8 {
                Protreg46::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg47 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg47 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg47 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg47 {
            #[inline(always)]
            fn from(val: u8) -> Protreg47 {
                Protreg47::from_bits(val)
            }
        }
        impl From<Protreg47> for u8 {
            #[inline(always)]
            fn from(val: Protreg47) -> u8 {
                Protreg47::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg48 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg48 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg48 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg48 {
            #[inline(always)]
            fn from(val: u8) -> Protreg48 {
                Protreg48::from_bits(val)
            }
        }
        impl From<Protreg48> for u8 {
            #[inline(always)]
            fn from(val: Protreg48) -> u8 {
                Protreg48::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg49 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg49 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg49 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg49 {
            #[inline(always)]
            fn from(val: u8) -> Protreg49 {
                Protreg49::from_bits(val)
            }
        }
        impl From<Protreg49> for u8 {
            #[inline(always)]
            fn from(val: Protreg49) -> u8 {
                Protreg49::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg5 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg5 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg5 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg5 {
            #[inline(always)]
            fn from(val: u8) -> Protreg5 {
                Protreg5::from_bits(val)
            }
        }
        impl From<Protreg5> for u8 {
            #[inline(always)]
            fn from(val: Protreg5) -> u8 {
                Protreg5::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg50 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg50 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg50 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg50 {
            #[inline(always)]
            fn from(val: u8) -> Protreg50 {
                Protreg50::from_bits(val)
            }
        }
        impl From<Protreg50> for u8 {
            #[inline(always)]
            fn from(val: Protreg50) -> u8 {
                Protreg50::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg51 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg51 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg51 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg51 {
            #[inline(always)]
            fn from(val: u8) -> Protreg51 {
                Protreg51::from_bits(val)
            }
        }
        impl From<Protreg51> for u8 {
            #[inline(always)]
            fn from(val: Protreg51) -> u8 {
                Protreg51::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg52 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg52 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg52 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg52 {
            #[inline(always)]
            fn from(val: u8) -> Protreg52 {
                Protreg52::from_bits(val)
            }
        }
        impl From<Protreg52> for u8 {
            #[inline(always)]
            fn from(val: Protreg52) -> u8 {
                Protreg52::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg53 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg53 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg53 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg53 {
            #[inline(always)]
            fn from(val: u8) -> Protreg53 {
                Protreg53::from_bits(val)
            }
        }
        impl From<Protreg53> for u8 {
            #[inline(always)]
            fn from(val: Protreg53) -> u8 {
                Protreg53::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg54 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg54 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg54 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg54 {
            #[inline(always)]
            fn from(val: u8) -> Protreg54 {
                Protreg54::from_bits(val)
            }
        }
        impl From<Protreg54> for u8 {
            #[inline(always)]
            fn from(val: Protreg54) -> u8 {
                Protreg54::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg55 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg55 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg55 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg55 {
            #[inline(always)]
            fn from(val: u8) -> Protreg55 {
                Protreg55::from_bits(val)
            }
        }
        impl From<Protreg55> for u8 {
            #[inline(always)]
            fn from(val: Protreg55) -> u8 {
                Protreg55::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg56 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg56 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg56 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg56 {
            #[inline(always)]
            fn from(val: u8) -> Protreg56 {
                Protreg56::from_bits(val)
            }
        }
        impl From<Protreg56> for u8 {
            #[inline(always)]
            fn from(val: Protreg56) -> u8 {
                Protreg56::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg57 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg57 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg57 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg57 {
            #[inline(always)]
            fn from(val: u8) -> Protreg57 {
                Protreg57::from_bits(val)
            }
        }
        impl From<Protreg57> for u8 {
            #[inline(always)]
            fn from(val: Protreg57) -> u8 {
                Protreg57::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg58 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg58 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg58 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg58 {
            #[inline(always)]
            fn from(val: u8) -> Protreg58 {
                Protreg58::from_bits(val)
            }
        }
        impl From<Protreg58> for u8 {
            #[inline(always)]
            fn from(val: Protreg58) -> u8 {
                Protreg58::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg59 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg59 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg59 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg59 {
            #[inline(always)]
            fn from(val: u8) -> Protreg59 {
                Protreg59::from_bits(val)
            }
        }
        impl From<Protreg59> for u8 {
            #[inline(always)]
            fn from(val: Protreg59) -> u8 {
                Protreg59::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg6 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg6 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg6 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg6 {
            #[inline(always)]
            fn from(val: u8) -> Protreg6 {
                Protreg6::from_bits(val)
            }
        }
        impl From<Protreg6> for u8 {
            #[inline(always)]
            fn from(val: Protreg6) -> u8 {
                Protreg6::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg60 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg60 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg60 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg60 {
            #[inline(always)]
            fn from(val: u8) -> Protreg60 {
                Protreg60::from_bits(val)
            }
        }
        impl From<Protreg60> for u8 {
            #[inline(always)]
            fn from(val: Protreg60) -> u8 {
                Protreg60::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg61 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg61 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg61 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg61 {
            #[inline(always)]
            fn from(val: u8) -> Protreg61 {
                Protreg61::from_bits(val)
            }
        }
        impl From<Protreg61> for u8 {
            #[inline(always)]
            fn from(val: Protreg61) -> u8 {
                Protreg61::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg62 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg62 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg62 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg62 {
            #[inline(always)]
            fn from(val: u8) -> Protreg62 {
                Protreg62::from_bits(val)
            }
        }
        impl From<Protreg62> for u8 {
            #[inline(always)]
            fn from(val: Protreg62) -> u8 {
                Protreg62::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg63 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg63 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg63 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg63 {
            #[inline(always)]
            fn from(val: u8) -> Protreg63 {
                Protreg63::from_bits(val)
            }
        }
        impl From<Protreg63> for u8 {
            #[inline(always)]
            fn from(val: Protreg63) -> u8 {
                Protreg63::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg7 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg7 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg7 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg7 {
            #[inline(always)]
            fn from(val: u8) -> Protreg7 {
                Protreg7::from_bits(val)
            }
        }
        impl From<Protreg7> for u8 {
            #[inline(always)]
            fn from(val: Protreg7) -> u8 {
                Protreg7::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg8 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg8 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg8 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg8 {
            #[inline(always)]
            fn from(val: u8) -> Protreg8 {
                Protreg8::from_bits(val)
            }
        }
        impl From<Protreg8> for u8 {
            #[inline(always)]
            fn from(val: Protreg8) -> u8 {
                Protreg8::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Protreg9 {
            #[doc = "Protection disabled."]
            DISABLED = 0x0,
            #[doc = "Protection enabled."]
            R_ENABLED_W_SET = 0x01,
        }
        impl Protreg9 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protreg9 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protreg9 {
            #[inline(always)]
            fn from(val: u8) -> Protreg9 {
                Protreg9::from_bits(val)
            }
        }
        impl From<Protreg9> for u8 {
            #[inline(always)]
            fn from(val: Protreg9) -> u8 {
                Protreg9::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Qdec {
            #[doc = "Peripheral configured in region 1."]
            IN_REGION1 = 0x0,
            #[doc = "Peripheral configured in region 0."]
            IN_REGION0 = 0x01,
        }
        impl Qdec {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Qdec {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Qdec {
            #[inline(always)]
            fn from(val: u8) -> Qdec {
                Qdec::from_bits(val)
            }
        }
        impl From<Qdec> for u8 {
            #[inline(always)]
            fn from(val: Qdec) -> u8 {
                Qdec::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Radio {
            #[doc = "Peripheral configured in region 1."]
            IN_REGION1 = 0x0,
            #[doc = "Peripheral configured in region 0."]
            IN_REGION0 = 0x01,
        }
        impl Radio {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Radio {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Radio {
            #[inline(always)]
            fn from(val: u8) -> Radio {
                Radio::from_bits(val)
            }
        }
        impl From<Radio> for u8 {
            #[inline(always)]
            fn from(val: Radio) -> u8 {
                Radio::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rng {
            #[doc = "Peripheral configured in region 1."]
            IN_REGION1 = 0x0,
            #[doc = "Peripheral configured in region 0."]
            IN_REGION0 = 0x01,
        }
        impl Rng {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rng {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rng {
            #[inline(always)]
            fn from(val: u8) -> Rng {
                Rng::from_bits(val)
            }
        }
        impl From<Rng> for u8 {
            #[inline(always)]
            fn from(val: Rng) -> u8 {
                Rng::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rtc0 {
            #[doc = "Peripheral configured in region 1."]
            IN_REGION1 = 0x0,
            #[doc = "Peripheral configured in region 0."]
            IN_REGION0 = 0x01,
        }
        impl Rtc0 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rtc0 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rtc0 {
            #[inline(always)]
            fn from(val: u8) -> Rtc0 {
                Rtc0::from_bits(val)
            }
        }
        impl From<Rtc0> for u8 {
            #[inline(always)]
            fn from(val: Rtc0) -> u8 {
                Rtc0::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rtc1 {
            #[doc = "Peripheral configured in region 1."]
            IN_REGION1 = 0x0,
            #[doc = "Peripheral configured in region 0."]
            IN_REGION0 = 0x01,
        }
        impl Rtc1 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rtc1 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rtc1 {
            #[inline(always)]
            fn from(val: u8) -> Rtc1 {
                Rtc1::from_bits(val)
            }
        }
        impl From<Rtc1> for u8 {
            #[inline(always)]
            fn from(val: Rtc1) -> u8 {
                Rtc1::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Spi0Twi0 {
            #[doc = "Peripheral configured in region 1."]
            IN_REGION1 = 0x0,
            #[doc = "Peripheral configured in region 0."]
            IN_REGION0 = 0x01,
        }
        impl Spi0Twi0 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Spi0Twi0 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Spi0Twi0 {
            #[inline(always)]
            fn from(val: u8) -> Spi0Twi0 {
                Spi0Twi0::from_bits(val)
            }
        }
        impl From<Spi0Twi0> for u8 {
            #[inline(always)]
            fn from(val: Spi0Twi0) -> u8 {
                Spi0Twi0::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Spi1Twi1 {
            #[doc = "Peripheral configured in region 1."]
            IN_REGION1 = 0x0,
            #[doc = "Peripheral configured in region 0."]
            IN_REGION0 = 0x01,
        }
        impl Spi1Twi1 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Spi1Twi1 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Spi1Twi1 {
            #[inline(always)]
            fn from(val: u8) -> Spi1Twi1 {
                Spi1Twi1::from_bits(val)
            }
        }
        impl From<Spi1Twi1> for u8 {
            #[inline(always)]
            fn from(val: Spi1Twi1) -> u8 {
                Spi1Twi1::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Temp {
            #[doc = "Peripheral configured in region 1."]
            IN_REGION1 = 0x0,
            #[doc = "Peripheral configured in region 0."]
            IN_REGION0 = 0x01,
        }
        impl Temp {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Temp {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Temp {
            #[inline(always)]
            fn from(val: u8) -> Temp {
                Temp::from_bits(val)
            }
        }
        impl From<Temp> for u8 {
            #[inline(always)]
            fn from(val: Temp) -> u8 {
                Temp::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Timer0 {
            #[doc = "Peripheral configured in region 1."]
            IN_REGION1 = 0x0,
            #[doc = "Peripheral configured in region 0."]
            IN_REGION0 = 0x01,
        }
        impl Timer0 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Timer0 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Timer0 {
            #[inline(always)]
            fn from(val: u8) -> Timer0 {
                Timer0::from_bits(val)
            }
        }
        impl From<Timer0> for u8 {
            #[inline(always)]
            fn from(val: Timer0) -> u8 {
                Timer0::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Timer1 {
            #[doc = "Peripheral configured in region 1."]
            IN_REGION1 = 0x0,
            #[doc = "Peripheral configured in region 0."]
            IN_REGION0 = 0x01,
        }
        impl Timer1 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Timer1 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Timer1 {
            #[inline(always)]
            fn from(val: u8) -> Timer1 {
                Timer1::from_bits(val)
            }
        }
        impl From<Timer1> for u8 {
            #[inline(always)]
            fn from(val: Timer1) -> u8 {
                Timer1::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Timer2 {
            #[doc = "Peripheral configured in region 1."]
            IN_REGION1 = 0x0,
            #[doc = "Peripheral configured in region 0."]
            IN_REGION0 = 0x01,
        }
        impl Timer2 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Timer2 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Timer2 {
            #[inline(always)]
            fn from(val: u8) -> Timer2 {
                Timer2::from_bits(val)
            }
        }
        impl From<Timer2> for u8 {
            #[inline(always)]
            fn from(val: Timer2) -> u8 {
                Timer2::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Uart0 {
            #[doc = "Peripheral configured in region 1."]
            IN_REGION1 = 0x0,
            #[doc = "Peripheral configured in region 0."]
            IN_REGION0 = 0x01,
        }
        impl Uart0 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Uart0 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Uart0 {
            #[inline(always)]
            fn from(val: u8) -> Uart0 {
                Uart0::from_bits(val)
            }
        }
        impl From<Uart0> for u8 {
            #[inline(always)]
            fn from(val: Uart0) -> u8 {
                Uart0::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Wdt {
            #[doc = "Peripheral configured in region 1."]
            IN_REGION1 = 0x0,
            #[doc = "Peripheral configured in region 0."]
            IN_REGION0 = 0x01,
        }
        impl Wdt {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Wdt {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Wdt {
            #[inline(always)]
            fn from(val: u8) -> Wdt {
                Wdt::from_bits(val)
            }
        }
        impl From<Wdt> for u8 {
            #[inline(always)]
            fn from(val: Wdt) -> u8 {
                Wdt::to_bits(val)
            }
        }
    }
}
pub mod nvmc {
    #[doc = "Non Volatile Memory Controller."]
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
        #[doc = "Ready flag."]
        #[inline(always)]
        pub const fn ready(self) -> crate::common::Reg<regs::Ready, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "Configuration register."]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Register for erasing a non-protected non-volatile memory page."]
        #[inline(always)]
        pub const fn erasepage(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Register for erasing a non-protected non-volatile memory page."]
        #[inline(always)]
        pub const fn erasepcr1(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Register for erasing all non-volatile user memory."]
        #[inline(always)]
        pub const fn eraseall(self) -> crate::common::Reg<regs::Eraseall, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Register for erasing a protected non-volatile memory page."]
        #[inline(always)]
        pub const fn erasepcr0(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "Register for start erasing User Information Congfiguration Registers."]
        #[inline(always)]
        pub const fn eraseuicr(self) -> crate::common::Reg<regs::Eraseuicr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Configuration register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Program write enable."]
            #[must_use]
            #[inline(always)]
            pub const fn wen(&self) -> super::vals::Wen {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Wen::from_bits(val as u8)
            }
            #[doc = "Program write enable."]
            #[inline(always)]
            pub const fn set_wen(&mut self, val: super::vals::Wen) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
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
        #[doc = "Register for erasing all non-volatile user memory."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Eraseall(pub u32);
        impl Eraseall {
            #[doc = "Starts the erasing of all user NVM (code region 0/1 and UICR registers)."]
            #[must_use]
            #[inline(always)]
            pub const fn eraseall(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Starts the erasing of all user NVM (code region 0/1 and UICR registers)."]
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
        #[doc = "Register for start erasing User Information Congfiguration Registers."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Eraseuicr(pub u32);
        impl Eraseuicr {
            #[doc = "It can only be used when all contents of code region 1 are erased."]
            #[must_use]
            #[inline(always)]
            pub const fn eraseuicr(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "It can only be used when all contents of code region 1 are erased."]
            #[inline(always)]
            pub const fn set_eraseuicr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Eraseuicr {
            #[inline(always)]
            fn default() -> Eraseuicr {
                Eraseuicr(0)
            }
        }
        impl core::fmt::Debug for Eraseuicr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Eraseuicr")
                    .field("eraseuicr", &self.eraseuicr())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Eraseuicr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Eraseuicr {{ eraseuicr: {=bool:?} }}", self.eraseuicr())
            }
        }
        #[doc = "Ready flag."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ready(pub u32);
        impl Ready {
            #[doc = "NVMC ready."]
            #[must_use]
            #[inline(always)]
            pub const fn ready(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "NVMC ready."]
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
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Wen {
            #[doc = "Read only access."]
            REN = 0x0,
            #[doc = "Write enabled."]
            WEN = 0x01,
            #[doc = "Erase enabled."]
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
pub mod power {
    #[doc = "Power Control."]
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
        #[doc = "Enable constant latency mode."]
        #[inline(always)]
        pub const fn tasks_constlat(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
        }
        #[doc = "Enable low power mode (variable latency)."]
        #[inline(always)]
        pub const fn tasks_lowpwr(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
        }
        #[doc = "Power failure warning."]
        #[inline(always)]
        pub const fn events_pofwarn(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "Interrupt enable set register."]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Interrupt enable clear register."]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Reset reason."]
        #[inline(always)]
        pub const fn resetreas(self) -> crate::common::Reg<regs::Resetreas, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "Ram status register."]
        #[inline(always)]
        pub const fn ramstatus(self) -> crate::common::Reg<regs::Ramstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0428usize) as _) }
        }
        #[doc = "System off register."]
        #[inline(always)]
        pub const fn systemoff(self) -> crate::common::Reg<regs::Systemoff, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Power failure configuration."]
        #[inline(always)]
        pub const fn pofcon(self) -> crate::common::Reg<regs::Pofcon, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "General purpose retention register. This register is a retained register."]
        #[inline(always)]
        pub const fn gpregret(self) -> crate::common::Reg<regs::Gpregret, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
        }
        #[doc = "Ram on/off."]
        #[inline(always)]
        pub const fn ramon(self) -> crate::common::Reg<regs::Ramon, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
        }
        #[doc = "Pin reset functionality configuration register. This register is a retained register."]
        #[inline(always)]
        pub const fn reset(self) -> crate::common::Reg<regs::Reset, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0544usize) as _) }
        }
        #[doc = "Ram on/off."]
        #[inline(always)]
        pub const fn ramonb(self) -> crate::common::Reg<regs::Ramonb, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0554usize) as _) }
        }
        #[doc = "DCDC converter enable configuration register."]
        #[inline(always)]
        pub const fn dcdcen(self) -> crate::common::Reg<regs::Dcdcen, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0578usize) as _) }
        }
        #[doc = "DCDC power-up force register."]
        #[inline(always)]
        pub const fn dcdcforce(self) -> crate::common::Reg<regs::Dcdcforce, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a08usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "DCDC converter enable configuration register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dcdcen(pub u32);
        impl Dcdcen {
            #[doc = "Enable DCDC converter."]
            #[must_use]
            #[inline(always)]
            pub const fn dcdcen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable DCDC converter."]
            #[inline(always)]
            pub const fn set_dcdcen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Dcdcen {
            #[inline(always)]
            fn default() -> Dcdcen {
                Dcdcen(0)
            }
        }
        impl core::fmt::Debug for Dcdcen {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dcdcen")
                    .field("dcdcen", &self.dcdcen())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dcdcen {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Dcdcen {{ dcdcen: {=bool:?} }}", self.dcdcen())
            }
        }
        #[doc = "DCDC power-up force register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dcdcforce(pub u32);
        impl Dcdcforce {
            #[doc = "DCDC power-up force off."]
            #[must_use]
            #[inline(always)]
            pub const fn forceoff(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "DCDC power-up force off."]
            #[inline(always)]
            pub const fn set_forceoff(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "DCDC power-up force on."]
            #[must_use]
            #[inline(always)]
            pub const fn forceon(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "DCDC power-up force on."]
            #[inline(always)]
            pub const fn set_forceon(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
        }
        impl Default for Dcdcforce {
            #[inline(always)]
            fn default() -> Dcdcforce {
                Dcdcforce(0)
            }
        }
        impl core::fmt::Debug for Dcdcforce {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dcdcforce")
                    .field("forceoff", &self.forceoff())
                    .field("forceon", &self.forceon())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dcdcforce {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Dcdcforce {{ forceoff: {=bool:?}, forceon: {=bool:?} }}",
                    self.forceoff(),
                    self.forceon()
                )
            }
        }
        #[doc = "General purpose retention register. This register is a retained register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Gpregret(pub u32);
        impl Gpregret {
            #[doc = "General purpose retention register."]
            #[must_use]
            #[inline(always)]
            pub const fn gpregret(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "General purpose retention register."]
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
        #[doc = "Interrupt enable clear register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Disable interrupt on POFWARN event."]
            #[must_use]
            #[inline(always)]
            pub const fn pofwarn(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on POFWARN event."]
            #[inline(always)]
            pub const fn set_pofwarn(&mut self, val: bool) {
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
                    .field("pofwarn", &self.pofwarn())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Int {{ pofwarn: {=bool:?} }}", self.pofwarn())
            }
        }
        #[doc = "Power failure configuration."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pofcon(pub u32);
        impl Pofcon {
            #[doc = "Power failure comparator enable."]
            #[must_use]
            #[inline(always)]
            pub const fn pof(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Power failure comparator enable."]
            #[inline(always)]
            pub const fn set_pof(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Set threshold level."]
            #[must_use]
            #[inline(always)]
            pub const fn threshold(&self) -> super::vals::Threshold {
                let val = (self.0 >> 1usize) & 0x03;
                super::vals::Threshold::from_bits(val as u8)
            }
            #[doc = "Set threshold level."]
            #[inline(always)]
            pub const fn set_threshold(&mut self, val: super::vals::Threshold) {
                self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
            }
        }
        impl Default for Pofcon {
            #[inline(always)]
            fn default() -> Pofcon {
                Pofcon(0)
            }
        }
        impl core::fmt::Debug for Pofcon {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Pofcon")
                    .field("pof", &self.pof())
                    .field("threshold", &self.threshold())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Pofcon {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Pofcon {{ pof: {=bool:?}, threshold: {:?} }}",
                    self.pof(),
                    self.threshold()
                )
            }
        }
        #[doc = "Ram on/off."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ramon(pub u32);
        impl Ramon {
            #[doc = "RAM block 0 behaviour in ON mode."]
            #[must_use]
            #[inline(always)]
            pub const fn onram0(&self) -> super::vals::Onram0 {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Onram0::from_bits(val as u8)
            }
            #[doc = "RAM block 0 behaviour in ON mode."]
            #[inline(always)]
            pub const fn set_onram0(&mut self, val: super::vals::Onram0) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "RAM block 1 behaviour in ON mode."]
            #[must_use]
            #[inline(always)]
            pub const fn onram1(&self) -> super::vals::Onram1 {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Onram1::from_bits(val as u8)
            }
            #[doc = "RAM block 1 behaviour in ON mode."]
            #[inline(always)]
            pub const fn set_onram1(&mut self, val: super::vals::Onram1) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "RAM block 0 behaviour in OFF mode."]
            #[must_use]
            #[inline(always)]
            pub const fn offram0(&self) -> super::vals::Offram0 {
                let val = (self.0 >> 16usize) & 0x01;
                super::vals::Offram0::from_bits(val as u8)
            }
            #[doc = "RAM block 0 behaviour in OFF mode."]
            #[inline(always)]
            pub const fn set_offram0(&mut self, val: super::vals::Offram0) {
                self.0 =
                    (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
            }
            #[doc = "RAM block 1 behaviour in OFF mode."]
            #[must_use]
            #[inline(always)]
            pub const fn offram1(&self) -> super::vals::Offram1 {
                let val = (self.0 >> 17usize) & 0x01;
                super::vals::Offram1::from_bits(val as u8)
            }
            #[doc = "RAM block 1 behaviour in OFF mode."]
            #[inline(always)]
            pub const fn set_offram1(&mut self, val: super::vals::Offram1) {
                self.0 =
                    (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
            }
        }
        impl Default for Ramon {
            #[inline(always)]
            fn default() -> Ramon {
                Ramon(0)
            }
        }
        impl core::fmt::Debug for Ramon {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ramon")
                    .field("onram0", &self.onram0())
                    .field("onram1", &self.onram1())
                    .field("offram0", &self.offram0())
                    .field("offram1", &self.offram1())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ramon {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Ramon {{ onram0: {:?}, onram1: {:?}, offram0: {:?}, offram1: {:?} }}",
                    self.onram0(),
                    self.onram1(),
                    self.offram0(),
                    self.offram1()
                )
            }
        }
        #[doc = "Ram on/off."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ramonb(pub u32);
        impl Ramonb {
            #[doc = "RAM block 2 behaviour in ON mode."]
            #[must_use]
            #[inline(always)]
            pub const fn onram2(&self) -> super::vals::Onram2 {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Onram2::from_bits(val as u8)
            }
            #[doc = "RAM block 2 behaviour in ON mode."]
            #[inline(always)]
            pub const fn set_onram2(&mut self, val: super::vals::Onram2) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "RAM block 3 behaviour in ON mode."]
            #[must_use]
            #[inline(always)]
            pub const fn onram3(&self) -> super::vals::Onram3 {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Onram3::from_bits(val as u8)
            }
            #[doc = "RAM block 3 behaviour in ON mode."]
            #[inline(always)]
            pub const fn set_onram3(&mut self, val: super::vals::Onram3) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "RAM block 2 behaviour in OFF mode."]
            #[must_use]
            #[inline(always)]
            pub const fn offram2(&self) -> super::vals::Offram2 {
                let val = (self.0 >> 16usize) & 0x01;
                super::vals::Offram2::from_bits(val as u8)
            }
            #[doc = "RAM block 2 behaviour in OFF mode."]
            #[inline(always)]
            pub const fn set_offram2(&mut self, val: super::vals::Offram2) {
                self.0 =
                    (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
            }
            #[doc = "RAM block 3 behaviour in OFF mode."]
            #[must_use]
            #[inline(always)]
            pub const fn offram3(&self) -> super::vals::Offram3 {
                let val = (self.0 >> 17usize) & 0x01;
                super::vals::Offram3::from_bits(val as u8)
            }
            #[doc = "RAM block 3 behaviour in OFF mode."]
            #[inline(always)]
            pub const fn set_offram3(&mut self, val: super::vals::Offram3) {
                self.0 =
                    (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
            }
        }
        impl Default for Ramonb {
            #[inline(always)]
            fn default() -> Ramonb {
                Ramonb(0)
            }
        }
        impl core::fmt::Debug for Ramonb {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ramonb")
                    .field("onram2", &self.onram2())
                    .field("onram3", &self.onram3())
                    .field("offram2", &self.offram2())
                    .field("offram3", &self.offram3())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ramonb {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Ramonb {{ onram2: {:?}, onram3: {:?}, offram2: {:?}, offram3: {:?} }}",
                    self.onram2(),
                    self.onram3(),
                    self.offram2(),
                    self.offram3()
                )
            }
        }
        #[doc = "Ram status register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ramstatus(pub u32);
        impl Ramstatus {
            #[doc = "RAM block 0 status."]
            #[must_use]
            #[inline(always)]
            pub const fn ramblock0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "RAM block 0 status."]
            #[inline(always)]
            pub const fn set_ramblock0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "RAM block 1 status."]
            #[must_use]
            #[inline(always)]
            pub const fn ramblock1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "RAM block 1 status."]
            #[inline(always)]
            pub const fn set_ramblock1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "RAM block 2 status."]
            #[must_use]
            #[inline(always)]
            pub const fn ramblock2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "RAM block 2 status."]
            #[inline(always)]
            pub const fn set_ramblock2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "RAM block 3 status."]
            #[must_use]
            #[inline(always)]
            pub const fn ramblock3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "RAM block 3 status."]
            #[inline(always)]
            pub const fn set_ramblock3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Ramstatus {
            #[inline(always)]
            fn default() -> Ramstatus {
                Ramstatus(0)
            }
        }
        impl core::fmt::Debug for Ramstatus {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ramstatus")
                    .field("ramblock0", &self.ramblock0())
                    .field("ramblock1", &self.ramblock1())
                    .field("ramblock2", &self.ramblock2())
                    .field("ramblock3", &self.ramblock3())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ramstatus {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Ramstatus {{ ramblock0: {=bool:?}, ramblock1: {=bool:?}, ramblock2: {=bool:?}, ramblock3: {=bool:?} }}" , self . ramblock0 () , self . ramblock1 () , self . ramblock2 () , self . ramblock3 ())
            }
        }
        #[doc = "Pin reset functionality configuration register. This register is a retained register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Reset(pub u32);
        impl Reset {
            #[doc = "Enable or disable pin reset in debug interface mode."]
            #[must_use]
            #[inline(always)]
            pub const fn reset(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable pin reset in debug interface mode."]
            #[inline(always)]
            pub const fn set_reset(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Reset {
            #[inline(always)]
            fn default() -> Reset {
                Reset(0)
            }
        }
        impl core::fmt::Debug for Reset {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Reset")
                    .field("reset", &self.reset())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Reset {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Reset {{ reset: {=bool:?} }}", self.reset())
            }
        }
        #[doc = "Reset reason."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Resetreas(pub u32);
        impl Resetreas {
            #[doc = "Reset from pin-reset detected."]
            #[must_use]
            #[inline(always)]
            pub const fn resetpin(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from pin-reset detected."]
            #[inline(always)]
            pub const fn set_resetpin(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Reset from watchdog detected."]
            #[must_use]
            #[inline(always)]
            pub const fn dog(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from watchdog detected."]
            #[inline(always)]
            pub const fn set_dog(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Reset from AIRCR.SYSRESETREQ detected."]
            #[must_use]
            #[inline(always)]
            pub const fn sreq(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from AIRCR.SYSRESETREQ detected."]
            #[inline(always)]
            pub const fn set_sreq(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Reset from CPU lock-up detected."]
            #[must_use]
            #[inline(always)]
            pub const fn lockup(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from CPU lock-up detected."]
            #[inline(always)]
            pub const fn set_lockup(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Reset from wake-up from OFF mode detected by the use of DETECT signal from GPIO."]
            #[must_use]
            #[inline(always)]
            pub const fn off(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from wake-up from OFF mode detected by the use of DETECT signal from GPIO."]
            #[inline(always)]
            pub const fn set_off(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Reset from wake-up from OFF mode detected by the use of ANADETECT signal from LPCOMP."]
            #[must_use]
            #[inline(always)]
            pub const fn lpcomp(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from wake-up from OFF mode detected by the use of ANADETECT signal from LPCOMP."]
            #[inline(always)]
            pub const fn set_lpcomp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Reset from wake-up from OFF mode detected by entering into debug interface mode."]
            #[must_use]
            #[inline(always)]
            pub const fn dif(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from wake-up from OFF mode detected by entering into debug interface mode."]
            #[inline(always)]
            pub const fn set_dif(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
                    .field("dog", &self.dog())
                    .field("sreq", &self.sreq())
                    .field("lockup", &self.lockup())
                    .field("off", &self.off())
                    .field("lpcomp", &self.lpcomp())
                    .field("dif", &self.dif())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Resetreas {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Resetreas {{ resetpin: {=bool:?}, dog: {=bool:?}, sreq: {=bool:?}, lockup: {=bool:?}, off: {=bool:?}, lpcomp: {=bool:?}, dif: {=bool:?} }}" , self . resetpin () , self . dog () , self . sreq () , self . lockup () , self . off () , self . lpcomp () , self . dif ())
            }
        }
        #[doc = "System off register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Systemoff(pub u32);
        impl Systemoff {
            #[doc = "Enter system off mode."]
            #[must_use]
            #[inline(always)]
            pub const fn systemoff(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enter system off mode."]
            #[inline(always)]
            pub const fn set_systemoff(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Systemoff {
            #[inline(always)]
            fn default() -> Systemoff {
                Systemoff(0)
            }
        }
        impl core::fmt::Debug for Systemoff {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Systemoff")
                    .field("systemoff", &self.systemoff())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Systemoff {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Systemoff {{ systemoff: {=bool:?} }}", self.systemoff())
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Offram0 {
            #[doc = "RAM block 0 OFF in OFF mode."]
            RAM0OFF = 0x0,
            #[doc = "RAM block 0 ON in OFF mode."]
            RAM0ON = 0x01,
        }
        impl Offram0 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Offram0 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Offram0 {
            #[inline(always)]
            fn from(val: u8) -> Offram0 {
                Offram0::from_bits(val)
            }
        }
        impl From<Offram0> for u8 {
            #[inline(always)]
            fn from(val: Offram0) -> u8 {
                Offram0::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Offram1 {
            #[doc = "RAM block 1 OFF in OFF mode."]
            RAM1OFF = 0x0,
            #[doc = "RAM block 1 ON in OFF mode."]
            RAM1ON = 0x01,
        }
        impl Offram1 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Offram1 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Offram1 {
            #[inline(always)]
            fn from(val: u8) -> Offram1 {
                Offram1::from_bits(val)
            }
        }
        impl From<Offram1> for u8 {
            #[inline(always)]
            fn from(val: Offram1) -> u8 {
                Offram1::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Offram2 {
            #[doc = "RAM block 2 OFF in OFF mode."]
            RAM2OFF = 0x0,
            #[doc = "RAM block 2 ON in OFF mode."]
            RAM2ON = 0x01,
        }
        impl Offram2 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Offram2 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Offram2 {
            #[inline(always)]
            fn from(val: u8) -> Offram2 {
                Offram2::from_bits(val)
            }
        }
        impl From<Offram2> for u8 {
            #[inline(always)]
            fn from(val: Offram2) -> u8 {
                Offram2::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Offram3 {
            #[doc = "RAM block 3 OFF in OFF mode."]
            RAM3OFF = 0x0,
            #[doc = "RAM block 3 ON in OFF mode."]
            RAM3ON = 0x01,
        }
        impl Offram3 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Offram3 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Offram3 {
            #[inline(always)]
            fn from(val: u8) -> Offram3 {
                Offram3::from_bits(val)
            }
        }
        impl From<Offram3> for u8 {
            #[inline(always)]
            fn from(val: Offram3) -> u8 {
                Offram3::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Onram0 {
            #[doc = "RAM block 0 OFF in ON mode."]
            RAM0OFF = 0x0,
            #[doc = "RAM block 0 ON in ON mode."]
            RAM0ON = 0x01,
        }
        impl Onram0 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Onram0 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Onram0 {
            #[inline(always)]
            fn from(val: u8) -> Onram0 {
                Onram0::from_bits(val)
            }
        }
        impl From<Onram0> for u8 {
            #[inline(always)]
            fn from(val: Onram0) -> u8 {
                Onram0::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Onram1 {
            #[doc = "RAM block 1 OFF in ON mode."]
            RAM1OFF = 0x0,
            #[doc = "RAM block 1 ON in ON mode."]
            RAM1ON = 0x01,
        }
        impl Onram1 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Onram1 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Onram1 {
            #[inline(always)]
            fn from(val: u8) -> Onram1 {
                Onram1::from_bits(val)
            }
        }
        impl From<Onram1> for u8 {
            #[inline(always)]
            fn from(val: Onram1) -> u8 {
                Onram1::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Onram2 {
            #[doc = "RAM block 2 OFF in ON mode."]
            RAM2OFF = 0x0,
            #[doc = "RAM block 2 ON in ON mode."]
            RAM2ON = 0x01,
        }
        impl Onram2 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Onram2 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Onram2 {
            #[inline(always)]
            fn from(val: u8) -> Onram2 {
                Onram2::from_bits(val)
            }
        }
        impl From<Onram2> for u8 {
            #[inline(always)]
            fn from(val: Onram2) -> u8 {
                Onram2::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Onram3 {
            #[doc = "RAM block 33 OFF in ON mode."]
            RAM3OFF = 0x0,
            #[doc = "RAM block 3 ON in ON mode."]
            RAM3ON = 0x01,
        }
        impl Onram3 {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Onram3 {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Onram3 {
            #[inline(always)]
            fn from(val: u8) -> Onram3 {
                Onram3::from_bits(val)
            }
        }
        impl From<Onram3> for u8 {
            #[inline(always)]
            fn from(val: Onram3) -> u8 {
                Onram3::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Threshold {
            #[doc = "Set threshold to 2.1Volts."]
            V21 = 0x0,
            #[doc = "Set threshold to 2.3Volts."]
            V23 = 0x01,
            #[doc = "Set threshold to 2.5Volts."]
            V25 = 0x02,
            #[doc = "Set threshold to 2.7Volts."]
            V27 = 0x03,
        }
        impl Threshold {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Threshold {
                unsafe { core::mem::transmute(val & 0x03) }
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
    #[doc = "PPI Channel."]
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
        #[doc = "Channel event end-point."]
        #[inline(always)]
        pub const fn eep(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Channel task end-point."]
        #[inline(always)]
        pub const fn tep(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    #[doc = "PPI controller."]
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
        #[doc = "Channel group tasks."]
        #[inline(always)]
        pub const fn tasks_chg(self, n: usize) -> TasksChg {
            assert!(n < 4usize);
            unsafe { TasksChg::from_ptr(self.ptr.add(0x0usize + n * 8usize) as _) }
        }
        #[doc = "Channel enable."]
        #[inline(always)]
        pub const fn chen(self) -> crate::common::Reg<regs::Chen, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Channel enable set."]
        #[inline(always)]
        pub const fn chenset(self) -> crate::common::Reg<regs::Chen, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Channel enable clear."]
        #[inline(always)]
        pub const fn chenclr(self) -> crate::common::Reg<regs::Chen, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "PPI Channel."]
        #[inline(always)]
        pub const fn ch(self, n: usize) -> Ch {
            assert!(n < 16usize);
            unsafe { Ch::from_ptr(self.ptr.add(0x0510usize + n * 8usize) as _) }
        }
        #[doc = "Channel group configuration."]
        #[inline(always)]
        pub const fn chg(self, n: usize) -> crate::common::Reg<regs::Chg, crate::common::RW> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize + n * 4usize) as _) }
        }
    }
    #[doc = "Channel group tasks."]
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
        #[doc = "Enable channel group."]
        #[inline(always)]
        pub const fn en(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Disable channel group."]
        #[inline(always)]
        pub const fn dis(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Channel enable."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Chen(pub u32);
        impl Chen {
            #[doc = "Enable PPI channel 0."]
            #[must_use]
            #[inline(always)]
            pub const fn ch(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Enable PPI channel 0."]
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
        #[doc = "Channel group configuration."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Chg(pub u32);
        impl Chg {
            #[doc = "Include CH0 in channel group."]
            #[must_use]
            #[inline(always)]
            pub const fn ch(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Include CH0 in channel group."]
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
pub mod qdec {
    #[doc = "Rotary decoder."]
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
        #[doc = "Start the quadrature decoder."]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop the quadrature decoder."]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Transfers the content from ACC registers to ACCREAD registers, and clears the ACC registers."]
        #[inline(always)]
        pub const fn tasks_readclracc(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "A new sample is written to the sample register."]
        #[inline(always)]
        pub const fn events_samplerdy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "REPORTPER number of samples accumulated in ACC register, and ACC register different than zero."]
        #[inline(always)]
        pub const fn events_reportrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "ACC or ACCDBL register overflow."]
        #[inline(always)]
        pub const fn events_accof(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "Shortcuts for the QDEC."]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Interrupt enable set register."]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Interrupt enable clear register."]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Enable the QDEC."]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "LED output pin polarity."]
        #[inline(always)]
        pub const fn ledpol(self) -> crate::common::Reg<regs::Ledpol, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Sample period."]
        #[inline(always)]
        pub const fn sampleper(self) -> crate::common::Reg<regs::Sampleper, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Motion sample value."]
        #[inline(always)]
        pub const fn sample(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Number of samples to generate an EVENT_REPORTRDY."]
        #[inline(always)]
        pub const fn reportper(self) -> crate::common::Reg<regs::Reportper, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "Accumulated valid transitions register."]
        #[inline(always)]
        pub const fn acc(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
        }
        #[doc = "Snapshot of ACC register. Value generated by the TASKS_READCLEACC task."]
        #[inline(always)]
        pub const fn accread(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
        #[doc = "Pin select for LED output."]
        #[inline(always)]
        pub const fn pselled(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
        }
        #[doc = "Pin select for phase A input."]
        #[inline(always)]
        pub const fn psela(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize) as _) }
        }
        #[doc = "Pin select for phase B input."]
        #[inline(always)]
        pub const fn pselb(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
        }
        #[doc = "Enable debouncer input filters."]
        #[inline(always)]
        pub const fn dbfen(self) -> crate::common::Reg<regs::Dbfen, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0528usize) as _) }
        }
        #[doc = "Time LED is switched ON before the sample."]
        #[inline(always)]
        pub const fn ledpre(self) -> crate::common::Reg<regs::Ledpre, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize) as _) }
        }
        #[doc = "Accumulated double (error) transitions register."]
        #[inline(always)]
        pub const fn accdbl(self) -> crate::common::Reg<regs::Accdbl, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0544usize) as _) }
        }
        #[doc = "Snapshot of ACCDBL register. Value generated by the TASKS_READCLEACC task."]
        #[inline(always)]
        pub const fn accdblread(self) -> crate::common::Reg<regs::Accdblread, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0548usize) as _) }
        }
        #[doc = "Peripheral power control."]
        #[inline(always)]
        pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Accumulated double (error) transitions register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Accdbl(pub u32);
        impl Accdbl {
            #[doc = "Accumulated double (error) transitions."]
            #[must_use]
            #[inline(always)]
            pub const fn accdbl(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "Accumulated double (error) transitions."]
            #[inline(always)]
            pub const fn set_accdbl(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Accdbl {
            #[inline(always)]
            fn default() -> Accdbl {
                Accdbl(0)
            }
        }
        impl core::fmt::Debug for Accdbl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Accdbl")
                    .field("accdbl", &self.accdbl())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Accdbl {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Accdbl {{ accdbl: {=u8:?} }}", self.accdbl())
            }
        }
        #[doc = "Snapshot of ACCDBL register. Value generated by the TASKS_READCLEACC task."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Accdblread(pub u32);
        impl Accdblread {
            #[doc = "Snapshot of accumulated double (error) transitions."]
            #[must_use]
            #[inline(always)]
            pub const fn accdblread(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "Snapshot of accumulated double (error) transitions."]
            #[inline(always)]
            pub const fn set_accdblread(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Accdblread {
            #[inline(always)]
            fn default() -> Accdblread {
                Accdblread(0)
            }
        }
        impl core::fmt::Debug for Accdblread {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Accdblread")
                    .field("accdblread", &self.accdblread())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Accdblread {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Accdblread {{ accdblread: {=u8:?} }}", self.accdblread())
            }
        }
        #[doc = "Enable debouncer input filters."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dbfen(pub u32);
        impl Dbfen {
            #[doc = "Enable debounce input filters."]
            #[must_use]
            #[inline(always)]
            pub const fn dbfen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable debounce input filters."]
            #[inline(always)]
            pub const fn set_dbfen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Dbfen {
            #[inline(always)]
            fn default() -> Dbfen {
                Dbfen(0)
            }
        }
        impl core::fmt::Debug for Dbfen {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dbfen")
                    .field("dbfen", &self.dbfen())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dbfen {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Dbfen {{ dbfen: {=bool:?} }}", self.dbfen())
            }
        }
        #[doc = "Enable the QDEC."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable QDEC."]
            #[must_use]
            #[inline(always)]
            pub const fn enable(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable QDEC."]
            #[inline(always)]
            pub const fn set_enable(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
                defmt::write!(f, "Enable {{ enable: {=bool:?} }}", self.enable())
            }
        }
        #[doc = "Interrupt enable clear register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Disable interrupt on SAMPLERDY event."]
            #[must_use]
            #[inline(always)]
            pub const fn samplerdy(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on SAMPLERDY event."]
            #[inline(always)]
            pub const fn set_samplerdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Disable interrupt on REPORTRDY event."]
            #[must_use]
            #[inline(always)]
            pub const fn reportrdy(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on REPORTRDY event."]
            #[inline(always)]
            pub const fn set_reportrdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Disable interrupt on ACCOF event."]
            #[must_use]
            #[inline(always)]
            pub const fn accof(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on ACCOF event."]
            #[inline(always)]
            pub const fn set_accof(&mut self, val: bool) {
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
                    .field("samplerdy", &self.samplerdy())
                    .field("reportrdy", &self.reportrdy())
                    .field("accof", &self.accof())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Int {{ samplerdy: {=bool:?}, reportrdy: {=bool:?}, accof: {=bool:?} }}",
                    self.samplerdy(),
                    self.reportrdy(),
                    self.accof()
                )
            }
        }
        #[doc = "LED output pin polarity."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ledpol(pub u32);
        impl Ledpol {
            #[doc = "LED output pin polarity."]
            #[must_use]
            #[inline(always)]
            pub const fn ledpol(&self) -> super::vals::Ledpol {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Ledpol::from_bits(val as u8)
            }
            #[doc = "LED output pin polarity."]
            #[inline(always)]
            pub const fn set_ledpol(&mut self, val: super::vals::Ledpol) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Ledpol {
            #[inline(always)]
            fn default() -> Ledpol {
                Ledpol(0)
            }
        }
        impl core::fmt::Debug for Ledpol {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ledpol")
                    .field("ledpol", &self.ledpol())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ledpol {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Ledpol {{ ledpol: {:?} }}", self.ledpol())
            }
        }
        #[doc = "Time LED is switched ON before the sample."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ledpre(pub u32);
        impl Ledpre {
            #[doc = "Period in us the LED in switched on prior to sampling."]
            #[must_use]
            #[inline(always)]
            pub const fn ledpre(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x01ff;
                val as u16
            }
            #[doc = "Period in us the LED in switched on prior to sampling."]
            #[inline(always)]
            pub const fn set_ledpre(&mut self, val: u16) {
                self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
            }
        }
        impl Default for Ledpre {
            #[inline(always)]
            fn default() -> Ledpre {
                Ledpre(0)
            }
        }
        impl core::fmt::Debug for Ledpre {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ledpre")
                    .field("ledpre", &self.ledpre())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ledpre {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Ledpre {{ ledpre: {=u16:?} }}", self.ledpre())
            }
        }
        #[doc = "Peripheral power control."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Power(pub u32);
        impl Power {
            #[doc = "Peripheral power control."]
            #[must_use]
            #[inline(always)]
            pub const fn power(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Peripheral power control."]
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
        #[doc = "Number of samples to generate an EVENT_REPORTRDY."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Reportper(pub u32);
        impl Reportper {
            #[doc = "Number of samples to generate an EVENT_REPORTRDY."]
            #[must_use]
            #[inline(always)]
            pub const fn reportper(&self) -> super::vals::Reportper {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::Reportper::from_bits(val as u8)
            }
            #[doc = "Number of samples to generate an EVENT_REPORTRDY."]
            #[inline(always)]
            pub const fn set_reportper(&mut self, val: super::vals::Reportper) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
            }
        }
        impl Default for Reportper {
            #[inline(always)]
            fn default() -> Reportper {
                Reportper(0)
            }
        }
        impl core::fmt::Debug for Reportper {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Reportper")
                    .field("reportper", &self.reportper())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Reportper {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Reportper {{ reportper: {:?} }}", self.reportper())
            }
        }
        #[doc = "Sample period."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sampleper(pub u32);
        impl Sampleper {
            #[doc = "Sample period."]
            #[must_use]
            #[inline(always)]
            pub const fn sampleper(&self) -> super::vals::Sampleper {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::Sampleper::from_bits(val as u8)
            }
            #[doc = "Sample period."]
            #[inline(always)]
            pub const fn set_sampleper(&mut self, val: super::vals::Sampleper) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
            }
        }
        impl Default for Sampleper {
            #[inline(always)]
            fn default() -> Sampleper {
                Sampleper(0)
            }
        }
        impl core::fmt::Debug for Sampleper {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sampleper")
                    .field("sampleper", &self.sampleper())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sampleper {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Sampleper {{ sampleper: {:?} }}", self.sampleper())
            }
        }
        #[doc = "Shortcuts for the QDEC."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between REPORTRDY event and READCLRACC task."]
            #[must_use]
            #[inline(always)]
            pub const fn reportrdy_readclracc(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between REPORTRDY event and READCLRACC task."]
            #[inline(always)]
            pub const fn set_reportrdy_readclracc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Shortcut between SAMPLERDY event and STOP task."]
            #[must_use]
            #[inline(always)]
            pub const fn samplerdy_stop(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between SAMPLERDY event and STOP task."]
            #[inline(always)]
            pub const fn set_samplerdy_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
                    .field("reportrdy_readclracc", &self.reportrdy_readclracc())
                    .field("samplerdy_stop", &self.samplerdy_stop())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Shorts {{ reportrdy_readclracc: {=bool:?}, samplerdy_stop: {=bool:?} }}",
                    self.reportrdy_readclracc(),
                    self.samplerdy_stop()
                )
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Ledpol {
            #[doc = "LED output is active low."]
            ACTIVE_LOW = 0x0,
            #[doc = "LED output is active high."]
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Reportper {
            #[doc = "10 samples per report."]
            _10SMPL = 0x0,
            #[doc = "40 samples per report."]
            _40SMPL = 0x01,
            #[doc = "80 samples per report."]
            _80SMPL = 0x02,
            #[doc = "120 samples per report."]
            _120SMPL = 0x03,
            #[doc = "160 samples per report."]
            _160SMPL = 0x04,
            #[doc = "200 samples per report."]
            _200SMPL = 0x05,
            #[doc = "240 samples per report."]
            _240SMPL = 0x06,
            #[doc = "280 samples per report."]
            _280SMPL = 0x07,
        }
        impl Reportper {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Reportper {
                unsafe { core::mem::transmute(val & 0x07) }
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Sampleper {
            #[doc = "128us sample period."]
            _128US = 0x0,
            #[doc = "256us sample period."]
            _256US = 0x01,
            #[doc = "512us sample period."]
            _512US = 0x02,
            #[doc = "1024us sample period."]
            _1024US = 0x03,
            #[doc = "2048us sample period."]
            _2048US = 0x04,
            #[doc = "4096us sample period."]
            _4096US = 0x05,
            #[doc = "8192us sample period."]
            _8192US = 0x06,
            #[doc = "16384us sample period."]
            _16384US = 0x07,
        }
        impl Sampleper {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Sampleper {
                unsafe { core::mem::transmute(val & 0x07) }
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
    #[doc = "The radio."]
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
        #[doc = "Enable radio in TX mode."]
        #[inline(always)]
        pub const fn tasks_txen(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Enable radio in RX mode."]
        #[inline(always)]
        pub const fn tasks_rxen(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Start radio."]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Stop radio."]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Disable radio."]
        #[inline(always)]
        pub const fn tasks_disable(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Start the RSSI and take one sample of the receive signal strength."]
        #[inline(always)]
        pub const fn tasks_rssistart(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Stop the RSSI measurement."]
        #[inline(always)]
        pub const fn tasks_rssistop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }
        #[doc = "Start the bit counter."]
        #[inline(always)]
        pub const fn tasks_bcstart(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "Stop the bit counter."]
        #[inline(always)]
        pub const fn tasks_bcstop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "Ready event."]
        #[inline(always)]
        pub const fn events_ready(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Address event."]
        #[inline(always)]
        pub const fn events_address(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Payload event."]
        #[inline(always)]
        pub const fn events_payload(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "End event."]
        #[inline(always)]
        pub const fn events_end(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
        }
        #[doc = "Disable event."]
        #[inline(always)]
        pub const fn events_disabled(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
        }
        #[doc = "A device address match occurred on the last received packet."]
        #[inline(always)]
        pub const fn events_devmatch(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
        }
        #[doc = "No device address match occurred on the last received packet."]
        #[inline(always)]
        pub const fn events_devmiss(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
        }
        #[doc = "Sampling of the receive signal strength complete. A new RSSI sample is ready for readout at the RSSISAMPLE register."]
        #[inline(always)]
        pub const fn events_rssiend(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
        }
        #[doc = "Bit counter reached bit count value specified in BCC register."]
        #[inline(always)]
        pub const fn events_bcmatch(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
        }
        #[doc = "Shortcuts for the radio."]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Interrupt enable set register."]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Interrupt enable clear register."]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "CRC status of received packet."]
        #[inline(always)]
        pub const fn crcstatus(self) -> crate::common::Reg<regs::Crcstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "Received address."]
        #[inline(always)]
        pub const fn rxmatch(self) -> crate::common::Reg<regs::Rxmatch, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0408usize) as _) }
        }
        #[doc = "Received CRC."]
        #[inline(always)]
        pub const fn rxcrc(self) -> crate::common::Reg<regs::Rxcrc, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x040cusize) as _) }
        }
        #[doc = "Device address match index."]
        #[inline(always)]
        pub const fn dai(self) -> crate::common::Reg<regs::Dai, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0410usize) as _) }
        }
        #[doc = "Packet pointer. Decision point: START task."]
        #[inline(always)]
        pub const fn packetptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Frequency."]
        #[inline(always)]
        pub const fn frequency(self) -> crate::common::Reg<regs::Frequency, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Output power."]
        #[inline(always)]
        pub const fn txpower(self) -> crate::common::Reg<regs::Txpower, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Data rate and modulation."]
        #[inline(always)]
        pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "Packet configuration 0."]
        #[inline(always)]
        pub const fn pcnf0(self) -> crate::common::Reg<regs::Pcnf0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
        }
        #[doc = "Packet configuration 1."]
        #[inline(always)]
        pub const fn pcnf1(self) -> crate::common::Reg<regs::Pcnf1, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
        #[doc = "Radio base address 0. Decision point: START task."]
        #[inline(always)]
        pub const fn base0(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
        }
        #[doc = "Radio base address 1. Decision point: START task."]
        #[inline(always)]
        pub const fn base1(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize) as _) }
        }
        #[doc = "Prefixes bytes for logical addresses 0 to 3."]
        #[inline(always)]
        pub const fn prefix0(self) -> crate::common::Reg<regs::Prefix0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
        }
        #[doc = "Prefixes bytes for logical addresses 4 to 7."]
        #[inline(always)]
        pub const fn prefix1(self) -> crate::common::Reg<regs::Prefix1, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0528usize) as _) }
        }
        #[doc = "Transmit address select."]
        #[inline(always)]
        pub const fn txaddress(self) -> crate::common::Reg<regs::Txaddress, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x052cusize) as _) }
        }
        #[doc = "Receive address select."]
        #[inline(always)]
        pub const fn rxaddresses(self) -> crate::common::Reg<regs::Rxaddresses, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0530usize) as _) }
        }
        #[doc = "CRC configuration."]
        #[inline(always)]
        pub const fn crccnf(self) -> crate::common::Reg<regs::Crccnf, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0534usize) as _) }
        }
        #[doc = "CRC polynomial."]
        #[inline(always)]
        pub const fn crcpoly(self) -> crate::common::Reg<regs::Crcpoly, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0538usize) as _) }
        }
        #[doc = "CRC initial value."]
        #[inline(always)]
        pub const fn crcinit(self) -> crate::common::Reg<regs::Crcinit, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x053cusize) as _) }
        }
        #[doc = "Test features enable register."]
        #[inline(always)]
        pub const fn test(self) -> crate::common::Reg<regs::Test, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize) as _) }
        }
        #[doc = "Inter Frame Spacing in microseconds."]
        #[inline(always)]
        pub const fn tifs(self) -> crate::common::Reg<regs::Tifs, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0544usize) as _) }
        }
        #[doc = "RSSI sample."]
        #[inline(always)]
        pub const fn rssisample(self) -> crate::common::Reg<regs::Rssisample, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0548usize) as _) }
        }
        #[doc = "Current radio state."]
        #[inline(always)]
        pub const fn state(self) -> crate::common::Reg<regs::State, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0550usize) as _) }
        }
        #[doc = "Data whitening initial value."]
        #[inline(always)]
        pub const fn datawhiteiv(self) -> crate::common::Reg<regs::Datawhiteiv, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0554usize) as _) }
        }
        #[doc = "Bit counter compare."]
        #[inline(always)]
        pub const fn bcc(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0560usize) as _) }
        }
        #[doc = "Device address base segment."]
        #[inline(always)]
        pub const fn dab(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize + n * 4usize) as _) }
        }
        #[doc = "Device address prefix."]
        #[inline(always)]
        pub const fn dap(self, n: usize) -> crate::common::Reg<regs::Dap, crate::common::RW> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0620usize + n * 4usize) as _) }
        }
        #[doc = "Device address match configuration."]
        #[inline(always)]
        pub const fn dacnf(self) -> crate::common::Reg<regs::Dacnf, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0640usize) as _) }
        }
        #[doc = "Trim value override register 0."]
        #[inline(always)]
        pub const fn override0(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0724usize) as _) }
        }
        #[doc = "Trim value override register 1."]
        #[inline(always)]
        pub const fn override1(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0728usize) as _) }
        }
        #[doc = "Trim value override register 2."]
        #[inline(always)]
        pub const fn override2(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x072cusize) as _) }
        }
        #[doc = "Trim value override register 3."]
        #[inline(always)]
        pub const fn override3(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0730usize) as _) }
        }
        #[doc = "Trim value override register 4."]
        #[inline(always)]
        pub const fn override4(self) -> crate::common::Reg<regs::Override4, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0734usize) as _) }
        }
        #[doc = "Peripheral power control."]
        #[inline(always)]
        pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "CRC configuration."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Crccnf(pub u32);
        impl Crccnf {
            #[doc = "CRC length. Decision point: START task."]
            #[must_use]
            #[inline(always)]
            pub const fn len(&self) -> super::vals::Len {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Len::from_bits(val as u8)
            }
            #[doc = "CRC length. Decision point: START task."]
            #[inline(always)]
            pub const fn set_len(&mut self, val: super::vals::Len) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
            #[doc = "Leave packet address field out of the CRC calculation. Decision point: START task."]
            #[must_use]
            #[inline(always)]
            pub const fn skipaddr(&self) -> super::vals::Skipaddr {
                let val = (self.0 >> 8usize) & 0x01;
                super::vals::Skipaddr::from_bits(val as u8)
            }
            #[doc = "Leave packet address field out of the CRC calculation. Decision point: START task."]
            #[inline(always)]
            pub const fn set_skipaddr(&mut self, val: super::vals::Skipaddr) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
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
        #[doc = "CRC initial value."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Crcinit(pub u32);
        impl Crcinit {
            #[doc = "Initial value for CRC calculation. Decision point: START task."]
            #[must_use]
            #[inline(always)]
            pub const fn crcinit(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x00ff_ffff;
                val as u32
            }
            #[doc = "Initial value for CRC calculation. Decision point: START task."]
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
        #[doc = "CRC polynomial."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Crcpoly(pub u32);
        impl Crcpoly {
            #[doc = "CRC polynomial. Decision point: START task."]
            #[must_use]
            #[inline(always)]
            pub const fn crcpoly(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x00ff_ffff;
                val as u32
            }
            #[doc = "CRC polynomial. Decision point: START task."]
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
        #[doc = "CRC status of received packet."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Crcstatus(pub u32);
        impl Crcstatus {
            #[doc = "CRC status of received packet."]
            #[must_use]
            #[inline(always)]
            pub const fn crcstatus(&self) -> super::vals::Crcstatus {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Crcstatus::from_bits(val as u8)
            }
            #[doc = "CRC status of received packet."]
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
        #[doc = "Device address match configuration."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dacnf(pub u32);
        impl Dacnf {
            #[doc = "Enable or disable device address matching using device address 0."]
            #[must_use]
            #[inline(always)]
            pub const fn ena0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 0."]
            #[inline(always)]
            pub const fn set_ena0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable device address matching using device address 1."]
            #[must_use]
            #[inline(always)]
            pub const fn ena1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 1."]
            #[inline(always)]
            pub const fn set_ena1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable device address matching using device address 2."]
            #[must_use]
            #[inline(always)]
            pub const fn ena2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 2."]
            #[inline(always)]
            pub const fn set_ena2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable or disable device address matching using device address 3."]
            #[must_use]
            #[inline(always)]
            pub const fn ena3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 3."]
            #[inline(always)]
            pub const fn set_ena3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable or disable device address matching using device address 4."]
            #[must_use]
            #[inline(always)]
            pub const fn ena4(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 4."]
            #[inline(always)]
            pub const fn set_ena4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable or disable device address matching using device address 5."]
            #[must_use]
            #[inline(always)]
            pub const fn ena5(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 5."]
            #[inline(always)]
            pub const fn set_ena5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable or disable device address matching using device address 6."]
            #[must_use]
            #[inline(always)]
            pub const fn ena6(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 6."]
            #[inline(always)]
            pub const fn set_ena6(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Enable or disable device address matching using device address 7."]
            #[must_use]
            #[inline(always)]
            pub const fn ena7(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 7."]
            #[inline(always)]
            pub const fn set_ena7(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "TxAdd for device address 0."]
            #[must_use]
            #[inline(always)]
            pub const fn txadd0(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 0."]
            #[inline(always)]
            pub const fn set_txadd0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "TxAdd for device address 1."]
            #[must_use]
            #[inline(always)]
            pub const fn txadd1(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 1."]
            #[inline(always)]
            pub const fn set_txadd1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "TxAdd for device address 2."]
            #[must_use]
            #[inline(always)]
            pub const fn txadd2(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 2."]
            #[inline(always)]
            pub const fn set_txadd2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "TxAdd for device address 3."]
            #[must_use]
            #[inline(always)]
            pub const fn txadd3(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 3."]
            #[inline(always)]
            pub const fn set_txadd3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "TxAdd for device address 4."]
            #[must_use]
            #[inline(always)]
            pub const fn txadd4(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 4."]
            #[inline(always)]
            pub const fn set_txadd4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "TxAdd for device address 5."]
            #[must_use]
            #[inline(always)]
            pub const fn txadd5(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 5."]
            #[inline(always)]
            pub const fn set_txadd5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "TxAdd for device address 6."]
            #[must_use]
            #[inline(always)]
            pub const fn txadd6(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 6."]
            #[inline(always)]
            pub const fn set_txadd6(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "TxAdd for device address 7."]
            #[must_use]
            #[inline(always)]
            pub const fn txadd7(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 7."]
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
        #[doc = "Device address match index."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dai(pub u32);
        impl Dai {
            #[doc = "Index (n) of device address (see DAB\\[n\\] and DAP\\[n\\]) that obtained an address match."]
            #[must_use]
            #[inline(always)]
            pub const fn dai(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x07;
                val as u8
            }
            #[doc = "Index (n) of device address (see DAB\\[n\\] and DAP\\[n\\]) that obtained an address match."]
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
        #[doc = "Device address prefix."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dap(pub u32);
        impl Dap {
            #[doc = "Device address prefix."]
            #[must_use]
            #[inline(always)]
            pub const fn dap(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Device address prefix."]
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
        #[doc = "Data whitening initial value."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Datawhiteiv(pub u32);
        impl Datawhiteiv {
            #[doc = "Data whitening initial value. Bit 0 corresponds to Position 0 of the LSFR, Bit 1 to position 5... Decision point: TXEN or RXEN task."]
            #[must_use]
            #[inline(always)]
            pub const fn datawhiteiv(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "Data whitening initial value. Bit 0 corresponds to Position 0 of the LSFR, Bit 1 to position 5... Decision point: TXEN or RXEN task."]
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
        #[doc = "Frequency."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Frequency(pub u32);
        impl Frequency {
            #[doc = "Radio channel frequency offset in MHz: RF Frequency = 2400 + FREQUENCY (MHz). Decision point: TXEN or RXEN task."]
            #[must_use]
            #[inline(always)]
            pub const fn frequency(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "Radio channel frequency offset in MHz: RF Frequency = 2400 + FREQUENCY (MHz). Decision point: TXEN or RXEN task."]
            #[inline(always)]
            pub const fn set_frequency(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
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
                defmt::write!(f, "Frequency {{ frequency: {=u8:?} }}", self.frequency())
            }
        }
        #[doc = "Interrupt enable clear register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Disable interrupt on READY event."]
            #[must_use]
            #[inline(always)]
            pub const fn ready(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on READY event."]
            #[inline(always)]
            pub const fn set_ready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Disable interrupt on ADDRESS event."]
            #[must_use]
            #[inline(always)]
            pub const fn address(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on ADDRESS event."]
            #[inline(always)]
            pub const fn set_address(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Disable interrupt on PAYLOAD event."]
            #[must_use]
            #[inline(always)]
            pub const fn payload(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on PAYLOAD event."]
            #[inline(always)]
            pub const fn set_payload(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Disable interrupt on END event."]
            #[must_use]
            #[inline(always)]
            pub const fn end(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on END event."]
            #[inline(always)]
            pub const fn set_end(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Disable interrupt on DISABLED event."]
            #[must_use]
            #[inline(always)]
            pub const fn disabled(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on DISABLED event."]
            #[inline(always)]
            pub const fn set_disabled(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Disable interrupt on DEVMATCH event."]
            #[must_use]
            #[inline(always)]
            pub const fn devmatch(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on DEVMATCH event."]
            #[inline(always)]
            pub const fn set_devmatch(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Disable interrupt on DEVMISS event."]
            #[must_use]
            #[inline(always)]
            pub const fn devmiss(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on DEVMISS event."]
            #[inline(always)]
            pub const fn set_devmiss(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Disable interrupt on RSSIEND event."]
            #[must_use]
            #[inline(always)]
            pub const fn rssiend(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on RSSIEND event."]
            #[inline(always)]
            pub const fn set_rssiend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Disable interrupt on BCMATCH event."]
            #[must_use]
            #[inline(always)]
            pub const fn bcmatch(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on BCMATCH event."]
            #[inline(always)]
            pub const fn set_bcmatch(&mut self, val: bool) {
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
                    .field("ready", &self.ready())
                    .field("address", &self.address())
                    .field("payload", &self.payload())
                    .field("end", &self.end())
                    .field("disabled", &self.disabled())
                    .field("devmatch", &self.devmatch())
                    .field("devmiss", &self.devmiss())
                    .field("rssiend", &self.rssiend())
                    .field("bcmatch", &self.bcmatch())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Int {{ ready: {=bool:?}, address: {=bool:?}, payload: {=bool:?}, end: {=bool:?}, disabled: {=bool:?}, devmatch: {=bool:?}, devmiss: {=bool:?}, rssiend: {=bool:?}, bcmatch: {=bool:?} }}" , self . ready () , self . address () , self . payload () , self . end () , self . disabled () , self . devmatch () , self . devmiss () , self . rssiend () , self . bcmatch ())
            }
        }
        #[doc = "Data rate and modulation."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Mode(pub u32);
        impl Mode {
            #[doc = "Radio data rate and modulation setting. Decision point: TXEN or RXEN task."]
            #[must_use]
            #[inline(always)]
            pub const fn mode(&self) -> super::vals::Mode {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Mode::from_bits(val as u8)
            }
            #[doc = "Radio data rate and modulation setting. Decision point: TXEN or RXEN task."]
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
        #[doc = "Trim value override register 4."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Override4(pub u32);
        impl Override4 {
            #[doc = "Trim value override 4."]
            #[must_use]
            #[inline(always)]
            pub const fn override4(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x0fff_ffff;
                val as u32
            }
            #[doc = "Trim value override 4."]
            #[inline(always)]
            pub const fn set_override4(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
            }
            #[doc = "Enable or disable override of default trim values."]
            #[must_use]
            #[inline(always)]
            pub const fn enable(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable override of default trim values."]
            #[inline(always)]
            pub const fn set_enable(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
            }
        }
        impl Default for Override4 {
            #[inline(always)]
            fn default() -> Override4 {
                Override4(0)
            }
        }
        impl core::fmt::Debug for Override4 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Override4")
                    .field("override4", &self.override4())
                    .field("enable", &self.enable())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Override4 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Override4 {{ override4: {=u32:?}, enable: {=bool:?} }}",
                    self.override4(),
                    self.enable()
                )
            }
        }
        #[doc = "Packet configuration 0."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pcnf0(pub u32);
        impl Pcnf0 {
            #[doc = "Length of length field in number of bits. Decision point: START task."]
            #[must_use]
            #[inline(always)]
            pub const fn lflen(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "Length of length field in number of bits. Decision point: START task."]
            #[inline(always)]
            pub const fn set_lflen(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
            #[doc = "Length of S0 field in number of bytes. Decision point: START task."]
            #[must_use]
            #[inline(always)]
            pub const fn s0len(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Length of S0 field in number of bytes. Decision point: START task."]
            #[inline(always)]
            pub const fn set_s0len(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Length of S1 field in number of bits. Decision point: START task."]
            #[must_use]
            #[inline(always)]
            pub const fn s1len(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0x0f;
                val as u8
            }
            #[doc = "Length of S1 field in number of bits. Decision point: START task."]
            #[inline(always)]
            pub const fn set_s1len(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
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
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Pcnf0 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Pcnf0 {{ lflen: {=u8:?}, s0len: {=bool:?}, s1len: {=u8:?} }}",
                    self.lflen(),
                    self.s0len(),
                    self.s1len()
                )
            }
        }
        #[doc = "Packet configuration 1."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pcnf1(pub u32);
        impl Pcnf1 {
            #[doc = "Maximum length of packet payload in number of bytes."]
            #[must_use]
            #[inline(always)]
            pub const fn maxlen(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Maximum length of packet payload in number of bytes."]
            #[inline(always)]
            pub const fn set_maxlen(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "Static length in number of bytes. Decision point: START task."]
            #[must_use]
            #[inline(always)]
            pub const fn statlen(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "Static length in number of bytes. Decision point: START task."]
            #[inline(always)]
            pub const fn set_statlen(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[doc = "Base address length in number of bytes. Decision point: START task."]
            #[must_use]
            #[inline(always)]
            pub const fn balen(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0x07;
                val as u8
            }
            #[doc = "Base address length in number of bytes. Decision point: START task."]
            #[inline(always)]
            pub const fn set_balen(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
            }
            #[doc = "On air endianness of packet length field. Decision point: START task."]
            #[must_use]
            #[inline(always)]
            pub const fn endian(&self) -> super::vals::Endian {
                let val = (self.0 >> 24usize) & 0x01;
                super::vals::Endian::from_bits(val as u8)
            }
            #[doc = "On air endianness of packet length field. Decision point: START task."]
            #[inline(always)]
            pub const fn set_endian(&mut self, val: super::vals::Endian) {
                self.0 =
                    (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
            }
            #[doc = "Packet whitening enable."]
            #[must_use]
            #[inline(always)]
            pub const fn whiteen(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "Packet whitening enable."]
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
        #[doc = "Peripheral power control."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Power(pub u32);
        impl Power {
            #[doc = "Peripheral power control."]
            #[must_use]
            #[inline(always)]
            pub const fn power(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Peripheral power control."]
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
        #[doc = "Prefixes bytes for logical addresses 0 to 3."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Prefix0(pub u32);
        impl Prefix0 {
            #[doc = "Address prefix 0. Decision point: START task."]
            #[must_use]
            #[inline(always)]
            pub const fn ap0(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 0. Decision point: START task."]
            #[inline(always)]
            pub const fn set_ap0(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "Address prefix 1. Decision point: START task."]
            #[must_use]
            #[inline(always)]
            pub const fn ap1(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 1. Decision point: START task."]
            #[inline(always)]
            pub const fn set_ap1(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[doc = "Address prefix 2. Decision point: START task."]
            #[must_use]
            #[inline(always)]
            pub const fn ap2(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 2. Decision point: START task."]
            #[inline(always)]
            pub const fn set_ap2(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
            }
            #[doc = "Address prefix 3. Decision point: START task."]
            #[must_use]
            #[inline(always)]
            pub const fn ap3(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 3. Decision point: START task."]
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
        #[doc = "Prefixes bytes for logical addresses 4 to 7."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Prefix1(pub u32);
        impl Prefix1 {
            #[doc = "Address prefix 4. Decision point: START task."]
            #[must_use]
            #[inline(always)]
            pub const fn ap4(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 4. Decision point: START task."]
            #[inline(always)]
            pub const fn set_ap4(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "Address prefix 5. Decision point: START task."]
            #[must_use]
            #[inline(always)]
            pub const fn ap5(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 5. Decision point: START task."]
            #[inline(always)]
            pub const fn set_ap5(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[doc = "Address prefix 6. Decision point: START task."]
            #[must_use]
            #[inline(always)]
            pub const fn ap6(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 6. Decision point: START task."]
            #[inline(always)]
            pub const fn set_ap6(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
            }
            #[doc = "Address prefix 7. Decision point: START task."]
            #[must_use]
            #[inline(always)]
            pub const fn ap7(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 7. Decision point: START task."]
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
        #[doc = "RSSI sample."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rssisample(pub u32);
        impl Rssisample {
            #[doc = "RSSI sample result. The result is read as a positive value so that ReceivedSignalStrength = -RSSISAMPLE dBm"]
            #[must_use]
            #[inline(always)]
            pub const fn rssisample(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "RSSI sample result. The result is read as a positive value so that ReceivedSignalStrength = -RSSISAMPLE dBm"]
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
        #[doc = "Receive address select."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxaddresses(pub u32);
        impl Rxaddresses {
            #[doc = "Enable reception on logical address 0. Decision point: START task."]
            #[must_use]
            #[inline(always)]
            pub const fn addr0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable reception on logical address 0. Decision point: START task."]
            #[inline(always)]
            pub const fn set_addr0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable reception on logical address 1. Decision point: START task."]
            #[must_use]
            #[inline(always)]
            pub const fn addr1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable reception on logical address 1. Decision point: START task."]
            #[inline(always)]
            pub const fn set_addr1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable reception on logical address 2. Decision point: START task."]
            #[must_use]
            #[inline(always)]
            pub const fn addr2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable reception on logical address 2. Decision point: START task."]
            #[inline(always)]
            pub const fn set_addr2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable reception on logical address 3. Decision point: START task."]
            #[must_use]
            #[inline(always)]
            pub const fn addr3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable reception on logical address 3. Decision point: START task."]
            #[inline(always)]
            pub const fn set_addr3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable reception on logical address 4. Decision point: START task."]
            #[must_use]
            #[inline(always)]
            pub const fn addr4(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable reception on logical address 4. Decision point: START task."]
            #[inline(always)]
            pub const fn set_addr4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable reception on logical address 5. Decision point: START task."]
            #[must_use]
            #[inline(always)]
            pub const fn addr5(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable reception on logical address 5. Decision point: START task."]
            #[inline(always)]
            pub const fn set_addr5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable reception on logical address 6. Decision point: START task."]
            #[must_use]
            #[inline(always)]
            pub const fn addr6(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable reception on logical address 6. Decision point: START task."]
            #[inline(always)]
            pub const fn set_addr6(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Enable reception on logical address 7. Decision point: START task."]
            #[must_use]
            #[inline(always)]
            pub const fn addr7(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable reception on logical address 7. Decision point: START task."]
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
        #[doc = "Received CRC."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxcrc(pub u32);
        impl Rxcrc {
            #[doc = "CRC field of previously received packet."]
            #[must_use]
            #[inline(always)]
            pub const fn rxcrc(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x00ff_ffff;
                val as u32
            }
            #[doc = "CRC field of previously received packet."]
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
        #[doc = "Received address."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxmatch(pub u32);
        impl Rxmatch {
            #[doc = "Logical address in which previous packet was received."]
            #[must_use]
            #[inline(always)]
            pub const fn rxmatch(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x07;
                val as u8
            }
            #[doc = "Logical address in which previous packet was received."]
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
        #[doc = "Shortcuts for the radio."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between READY event and START task."]
            #[must_use]
            #[inline(always)]
            pub const fn ready_start(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between READY event and START task."]
            #[inline(always)]
            pub const fn set_ready_start(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Shortcut between END event and DISABLE task."]
            #[must_use]
            #[inline(always)]
            pub const fn end_disable(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between END event and DISABLE task."]
            #[inline(always)]
            pub const fn set_end_disable(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Shortcut between DISABLED event and TXEN task."]
            #[must_use]
            #[inline(always)]
            pub const fn disabled_txen(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between DISABLED event and TXEN task."]
            #[inline(always)]
            pub const fn set_disabled_txen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Shortcut between DISABLED event and RXEN task."]
            #[must_use]
            #[inline(always)]
            pub const fn disabled_rxen(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between DISABLED event and RXEN task."]
            #[inline(always)]
            pub const fn set_disabled_rxen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Shortcut between ADDRESS event and RSSISTART task."]
            #[must_use]
            #[inline(always)]
            pub const fn address_rssistart(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between ADDRESS event and RSSISTART task."]
            #[inline(always)]
            pub const fn set_address_rssistart(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Shortcut between END event and START task."]
            #[must_use]
            #[inline(always)]
            pub const fn end_start(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between END event and START task."]
            #[inline(always)]
            pub const fn set_end_start(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Shortcut between ADDRESS event and BCSTART task."]
            #[must_use]
            #[inline(always)]
            pub const fn address_bcstart(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between ADDRESS event and BCSTART task."]
            #[inline(always)]
            pub const fn set_address_bcstart(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Shortcut between DISABLED event and RSSISTOP task."]
            #[must_use]
            #[inline(always)]
            pub const fn disabled_rssistop(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between DISABLED event and RSSISTOP task."]
            #[inline(always)]
            pub const fn set_disabled_rssistop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Shorts {{ ready_start: {=bool:?}, end_disable: {=bool:?}, disabled_txen: {=bool:?}, disabled_rxen: {=bool:?}, address_rssistart: {=bool:?}, end_start: {=bool:?}, address_bcstart: {=bool:?}, disabled_rssistop: {=bool:?} }}" , self . ready_start () , self . end_disable () , self . disabled_txen () , self . disabled_rxen () , self . address_rssistart () , self . end_start () , self . address_bcstart () , self . disabled_rssistop ())
            }
        }
        #[doc = "Current radio state."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct State(pub u32);
        impl State {
            #[doc = "Current radio state."]
            #[must_use]
            #[inline(always)]
            pub const fn state(&self) -> super::vals::State {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::State::from_bits(val as u8)
            }
            #[doc = "Current radio state."]
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
        #[doc = "Test features enable register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Test(pub u32);
        impl Test {
            #[doc = "Constant carrier. Decision point: TXEN task."]
            #[must_use]
            #[inline(always)]
            pub const fn constcarrier(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Constant carrier. Decision point: TXEN task."]
            #[inline(always)]
            pub const fn set_constcarrier(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "PLL lock. Decision point: TXEN or RXEN task."]
            #[must_use]
            #[inline(always)]
            pub const fn plllock(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "PLL lock. Decision point: TXEN or RXEN task."]
            #[inline(always)]
            pub const fn set_plllock(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
        }
        impl Default for Test {
            #[inline(always)]
            fn default() -> Test {
                Test(0)
            }
        }
        impl core::fmt::Debug for Test {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Test")
                    .field("constcarrier", &self.constcarrier())
                    .field("plllock", &self.plllock())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Test {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Test {{ constcarrier: {=bool:?}, plllock: {=bool:?} }}",
                    self.constcarrier(),
                    self.plllock()
                )
            }
        }
        #[doc = "Inter Frame Spacing in microseconds."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Tifs(pub u32);
        impl Tifs {
            #[doc = "Inter frame spacing in microseconds. Decision point: START rask"]
            #[must_use]
            #[inline(always)]
            pub const fn tifs(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Inter frame spacing in microseconds. Decision point: START rask"]
            #[inline(always)]
            pub const fn set_tifs(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
                defmt::write!(f, "Tifs {{ tifs: {=u8:?} }}", self.tifs())
            }
        }
        #[doc = "Transmit address select."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Txaddress(pub u32);
        impl Txaddress {
            #[doc = "Logical address to be used when transmitting a packet. Decision point: START task."]
            #[must_use]
            #[inline(always)]
            pub const fn txaddress(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x07;
                val as u8
            }
            #[doc = "Logical address to be used when transmitting a packet. Decision point: START task."]
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
        #[doc = "Output power."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Txpower(pub u32);
        impl Txpower {
            #[doc = "Radio output power. Decision point: TXEN task."]
            #[must_use]
            #[inline(always)]
            pub const fn txpower(&self) -> super::vals::Txpower {
                let val = (self.0 >> 0usize) & 0xff;
                super::vals::Txpower::from_bits(val as u8)
            }
            #[doc = "Radio output power. Decision point: TXEN task."]
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
        pub enum Crcstatus {
            #[doc = "Packet received with CRC error."]
            CRCERROR = 0x0,
            #[doc = "Packet received with CRC ok."]
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
            #[doc = "CRC calculation disabled."]
            DISABLED = 0x0,
            #[doc = "One byte long CRC."]
            ONE = 0x01,
            #[doc = "Two bytes long CRC."]
            TWO = 0x02,
            #[doc = "Three bytes long CRC."]
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
        pub enum Mode {
            #[doc = "1Mbit/s Nordic propietary radio mode."]
            NRF_1MBIT = 0x0,
            #[doc = "2Mbit/s Nordic propietary radio mode."]
            NRF_2MBIT = 0x01,
            #[doc = "250kbit/s Nordic propietary radio mode."]
            NRF_250KBIT = 0x02,
            #[doc = "1Mbit/s Bluetooth Low Energy"]
            BLE_1MBIT = 0x03,
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
        pub enum Skipaddr {
            #[doc = "Include packet address in CRC calculation."]
            INCLUDE = 0x0,
            #[doc = "Packet address is skipped in CRC calculation. The CRC calculation will start at the first byte after the address."]
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum State {
            #[doc = "Radio is in the Disabled state."]
            DISABLED = 0x0,
            #[doc = "Radio is in the Rx Ramp Up state."]
            RX_RU = 0x01,
            #[doc = "Radio is in the Rx Idle state."]
            RX_IDLE = 0x02,
            #[doc = "Radio is in the Rx state."]
            RX = 0x03,
            #[doc = "Radio is in the Rx Disable state."]
            RX_DISABLE = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
            _RESERVED_8 = 0x08,
            #[doc = "Radio is in the Tx Ramp Up state."]
            TX_RU = 0x09,
            #[doc = "Radio is in the Tx Idle state."]
            TX_IDLE = 0x0a,
            #[doc = "Radio is in the Tx state."]
            TX = 0x0b,
            #[doc = "Radio is in the Tx Disable state."]
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
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Txpower(u8);
        impl Txpower {
            #[doc = "0dBm."]
            pub const _0_DBM: Self = Self(0x0);
            #[doc = "+4dBm."]
            pub const POS4_DBM: Self = Self(0x04);
            #[doc = "-30dBm."]
            pub const NEG30_DBM: Self = Self(0xd8);
            #[doc = "-20dBm."]
            pub const NEG20_DBM: Self = Self(0xec);
            #[doc = "-16dBm."]
            pub const NEG16_DBM: Self = Self(0xf0);
            #[doc = "-12dBm."]
            pub const NEG12_DBM: Self = Self(0xf4);
            #[doc = "-8dBm."]
            pub const NEG8_DBM: Self = Self(0xf8);
            #[doc = "-4dBm."]
            pub const NEG4_DBM: Self = Self(0xfc);
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
                    0x04 => f.write_str("POS4_DBM"),
                    0xd8 => f.write_str("NEG30_DBM"),
                    0xec => f.write_str("NEG20_DBM"),
                    0xf0 => f.write_str("NEG16_DBM"),
                    0xf4 => f.write_str("NEG12_DBM"),
                    0xf8 => f.write_str("NEG8_DBM"),
                    0xfc => f.write_str("NEG4_DBM"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Txpower {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0 => defmt::write!(f, "_0_DBM"),
                    0x04 => defmt::write!(f, "POS4_DBM"),
                    0xd8 => defmt::write!(f, "NEG30_DBM"),
                    0xec => defmt::write!(f, "NEG20_DBM"),
                    0xf0 => defmt::write!(f, "NEG16_DBM"),
                    0xf4 => defmt::write!(f, "NEG12_DBM"),
                    0xf8 => defmt::write!(f, "NEG8_DBM"),
                    0xfc => defmt::write!(f, "NEG4_DBM"),
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
pub mod rng {
    #[doc = "Random Number Generator."]
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
        #[doc = "Start the random number generator."]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop the random number generator."]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "New random number generated and written to VALUE register."]
        #[inline(always)]
        pub const fn events_valrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Shortcuts for the RNG."]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Interrupt enable set register"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Interrupt enable clear register"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Configuration register."]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "RNG random number."]
        #[inline(always)]
        pub const fn value(self) -> crate::common::Reg<regs::Value, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Peripheral power control."]
        #[inline(always)]
        pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Configuration register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Digital error correction enable."]
            #[must_use]
            #[inline(always)]
            pub const fn dercen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Digital error correction enable."]
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
        #[doc = "Interrupt enable clear register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Disable interrupt on VALRDY event."]
            #[must_use]
            #[inline(always)]
            pub const fn valrdy(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on VALRDY event."]
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
        #[doc = "Peripheral power control."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Power(pub u32);
        impl Power {
            #[doc = "Peripheral power control."]
            #[must_use]
            #[inline(always)]
            pub const fn power(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Peripheral power control."]
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
        #[doc = "Shortcuts for the RNG."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between VALRDY event and STOP task."]
            #[must_use]
            #[inline(always)]
            pub const fn valrdy_stop(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between VALRDY event and STOP task."]
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
        #[doc = "RNG random number."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Value(pub u32);
        impl Value {
            #[doc = "Generated random number."]
            #[must_use]
            #[inline(always)]
            pub const fn value(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Generated random number."]
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
    #[doc = "Real time counter 0."]
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
        #[doc = "Start RTC Counter."]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop RTC Counter."]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Clear RTC Counter."]
        #[inline(always)]
        pub const fn tasks_clear(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Set COUNTER to 0xFFFFFFF0."]
        #[inline(always)]
        pub const fn tasks_trigovrflw(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Event on COUNTER increment."]
        #[inline(always)]
        pub const fn events_tick(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Event on COUNTER overflow."]
        #[inline(always)]
        pub const fn events_ovrflw(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Compare event on CC\\[n\\] match."]
        #[inline(always)]
        pub const fn events_compare(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize + n * 4usize) as _) }
        }
        #[doc = "Interrupt enable set register."]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Interrupt enable clear register."]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Configures event enable routing to PPI for each RTC event."]
        #[inline(always)]
        pub const fn evten(self) -> crate::common::Reg<regs::Evt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0340usize) as _) }
        }
        #[doc = "Enable events routing to PPI. The reading of this register gives the value of EVTEN."]
        #[inline(always)]
        pub const fn evtenset(self) -> crate::common::Reg<regs::Evt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0344usize) as _) }
        }
        #[doc = "Disable events routing to PPI. The reading of this register gives the value of EVTEN."]
        #[inline(always)]
        pub const fn evtenclr(self) -> crate::common::Reg<regs::Evt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0348usize) as _) }
        }
        #[doc = "Current COUNTER value."]
        #[inline(always)]
        pub const fn counter(self) -> crate::common::Reg<regs::Counter, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "12-bit prescaler for COUNTER frequency (32768/(PRESCALER+1)). Must be written when RTC is STOPed."]
        #[inline(always)]
        pub const fn prescaler(self) -> crate::common::Reg<regs::Prescaler, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Capture/compare registers."]
        #[inline(always)]
        pub const fn cc(self, n: usize) -> crate::common::Reg<regs::Cc, crate::common::RW> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize + n * 4usize) as _) }
        }
        #[doc = "Peripheral power control."]
        #[inline(always)]
        pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Capture/compare registers."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cc(pub u32);
        impl Cc {
            #[doc = "Compare value."]
            #[must_use]
            #[inline(always)]
            pub const fn compare(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x00ff_ffff;
                val as u32
            }
            #[doc = "Compare value."]
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
        #[doc = "Current COUNTER value."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Counter(pub u32);
        impl Counter {
            #[doc = "Counter value."]
            #[must_use]
            #[inline(always)]
            pub const fn counter(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x00ff_ffff;
                val as u32
            }
            #[doc = "Counter value."]
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
        #[doc = "Configures event enable routing to PPI for each RTC event."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Evt(pub u32);
        impl Evt {
            #[doc = "TICK event enable."]
            #[must_use]
            #[inline(always)]
            pub const fn tick(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "TICK event enable."]
            #[inline(always)]
            pub const fn set_tick(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "OVRFLW event enable."]
            #[must_use]
            #[inline(always)]
            pub const fn ovrflw(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "OVRFLW event enable."]
            #[inline(always)]
            pub const fn set_ovrflw(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "COMPARE\\[0\\] event enable."]
            #[must_use]
            #[inline(always)]
            pub const fn compare(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 16usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "COMPARE\\[0\\] event enable."]
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
        #[doc = "Interrupt enable clear register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Disable interrupt on TICK event."]
            #[must_use]
            #[inline(always)]
            pub const fn tick(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on TICK event."]
            #[inline(always)]
            pub const fn set_tick(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Disable interrupt on OVRFLW event."]
            #[must_use]
            #[inline(always)]
            pub const fn ovrflw(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on OVRFLW event."]
            #[inline(always)]
            pub const fn set_ovrflw(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Disable interrupt on COMPARE\\[0\\] event."]
            #[must_use]
            #[inline(always)]
            pub const fn compare(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 16usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on COMPARE\\[0\\] event."]
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
        #[doc = "Peripheral power control."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Power(pub u32);
        impl Power {
            #[doc = "Peripheral power control."]
            #[must_use]
            #[inline(always)]
            pub const fn power(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Peripheral power control."]
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
        #[doc = "12-bit prescaler for COUNTER frequency (32768/(PRESCALER+1)). Must be written when RTC is STOPed."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Prescaler(pub u32);
        impl Prescaler {
            #[doc = "RTC PRESCALER value."]
            #[must_use]
            #[inline(always)]
            pub const fn prescaler(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "RTC PRESCALER value."]
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
    }
}
pub mod spi {
    #[doc = "SPI master 0."]
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
        #[doc = "TXD byte sent and RXD byte received."]
        #[inline(always)]
        pub const fn events_ready(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "Interrupt enable set register."]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Interrupt enable clear register."]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Enable SPI."]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Pin select for SCK."]
        #[inline(always)]
        pub const fn pselsck(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Pin select for MOSI."]
        #[inline(always)]
        pub const fn pselmosi(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Pin select for MISO."]
        #[inline(always)]
        pub const fn pselmiso(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "RX data."]
        #[inline(always)]
        pub const fn rxd(self) -> crate::common::Reg<regs::Rxd, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
        #[doc = "TX data."]
        #[inline(always)]
        pub const fn txd(self) -> crate::common::Reg<regs::Txd, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
        }
        #[doc = "SPI frequency"]
        #[inline(always)]
        pub const fn frequency(self) -> crate::common::Reg<regs::Frequency, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
        }
        #[doc = "Configuration register."]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0554usize) as _) }
        }
        #[doc = "Peripheral power control."]
        #[inline(always)]
        pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Configuration register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Bit order."]
            #[must_use]
            #[inline(always)]
            pub const fn order(&self) -> super::vals::Order {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Order::from_bits(val as u8)
            }
            #[doc = "Bit order."]
            #[inline(always)]
            pub const fn set_order(&mut self, val: super::vals::Order) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Serial clock (SCK) phase."]
            #[must_use]
            #[inline(always)]
            pub const fn cpha(&self) -> super::vals::Cpha {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Cpha::from_bits(val as u8)
            }
            #[doc = "Serial clock (SCK) phase."]
            #[inline(always)]
            pub const fn set_cpha(&mut self, val: super::vals::Cpha) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Serial clock (SCK) polarity."]
            #[must_use]
            #[inline(always)]
            pub const fn cpol(&self) -> super::vals::Cpol {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Cpol::from_bits(val as u8)
            }
            #[doc = "Serial clock (SCK) polarity."]
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
        #[doc = "Enable SPI."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable SPI."]
            #[must_use]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable SPI."]
            #[inline(always)]
            pub const fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
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
        #[doc = "SPI frequency"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Frequency(pub u32);
        impl Frequency {
            #[doc = "SPI data rate."]
            #[must_use]
            #[inline(always)]
            pub const fn frequency(&self) -> super::vals::Frequency {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Frequency::from_bits(val as u32)
            }
            #[doc = "SPI data rate."]
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
        #[doc = "Interrupt enable clear register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Disable interrupt on READY event."]
            #[must_use]
            #[inline(always)]
            pub const fn ready(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on READY event."]
            #[inline(always)]
            pub const fn set_ready(&mut self, val: bool) {
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
                f.debug_struct("Int").field("ready", &self.ready()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Int {{ ready: {=bool:?} }}", self.ready())
            }
        }
        #[doc = "Peripheral power control."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Power(pub u32);
        impl Power {
            #[doc = "Peripheral power control."]
            #[must_use]
            #[inline(always)]
            pub const fn power(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Peripheral power control."]
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
        #[doc = "RX data."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxd(pub u32);
        impl Rxd {
            #[doc = "RX data from last transfer."]
            #[must_use]
            #[inline(always)]
            pub const fn rxd(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "RX data from last transfer."]
            #[inline(always)]
            pub const fn set_rxd(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Rxd {
            #[inline(always)]
            fn default() -> Rxd {
                Rxd(0)
            }
        }
        impl core::fmt::Debug for Rxd {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rxd").field("rxd", &self.rxd()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rxd {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Rxd {{ rxd: {=u8:?} }}", self.rxd())
            }
        }
        #[doc = "TX data."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Txd(pub u32);
        impl Txd {
            #[doc = "TX data for next transfer."]
            #[must_use]
            #[inline(always)]
            pub const fn txd(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "TX data for next transfer."]
            #[inline(always)]
            pub const fn set_txd(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Txd {
            #[inline(always)]
            fn default() -> Txd {
                Txd(0)
            }
        }
        impl core::fmt::Debug for Txd {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Txd").field("txd", &self.txd()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Txd {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Txd {{ txd: {=u8:?} }}", self.txd())
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Cpha {
            #[doc = "Sample on leading edge of the clock. Shift serial data on trailing edge."]
            LEADING = 0x0,
            #[doc = "Sample on trailing edge of the clock. Shift serial data on leading edge."]
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
            #[doc = "Active high."]
            ACTIVE_HIGH = 0x0,
            #[doc = "Active low."]
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
            #[doc = "Disabled SPI."]
            DISABLED = 0x0,
            #[doc = "Enable SPI."]
            ENABLED = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl Enable {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Enable {
                unsafe { core::mem::transmute(val & 0x07) }
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
            #[doc = "125kbps."]
            pub const K125: Self = Self(0x0200_0000);
            #[doc = "250kbps."]
            pub const K250: Self = Self(0x0400_0000);
            #[doc = "500kbps."]
            pub const K500: Self = Self(0x0800_0000);
            #[doc = "1Mbps."]
            pub const M1: Self = Self(0x1000_0000);
            #[doc = "2Mbps."]
            pub const M2: Self = Self(0x2000_0000);
            #[doc = "4Mbps."]
            pub const M4: Self = Self(0x4000_0000);
            #[doc = "8Mbps."]
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
                    0x1000_0000 => f.write_str("M1"),
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
                    0x1000_0000 => defmt::write!(f, "M1"),
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
            #[doc = "Most significant bit transmitted out first."]
            MSB_FIRST = 0x0,
            #[doc = "Least significant bit transmitted out first."]
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
pub mod spis {
    #[doc = "SPI slave 1."]
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
        #[doc = "Acquire SPI semaphore."]
        #[inline(always)]
        pub const fn tasks_acquire(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
        }
        #[doc = "Release SPI semaphore."]
        #[inline(always)]
        pub const fn tasks_release(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
        }
        #[doc = "Granted transaction completed."]
        #[inline(always)]
        pub const fn events_end(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "End of RXD buffer reached"]
        #[inline(always)]
        pub const fn events_endrx(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
        }
        #[doc = "Semaphore acquired."]
        #[inline(always)]
        pub const fn events_acquired(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
        }
        #[doc = "Shortcuts for SPIS."]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Interrupt enable set register."]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Interrupt enable clear register."]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Semaphore status."]
        #[inline(always)]
        pub const fn semstat(self) -> crate::common::Reg<regs::Semstat, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "Status from last transaction."]
        #[inline(always)]
        pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0440usize) as _) }
        }
        #[doc = "Enable SPIS."]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Pin select for SCK."]
        #[inline(always)]
        pub const fn pselsck(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Pin select for MISO."]
        #[inline(always)]
        pub const fn pselmiso(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Pin select for MOSI."]
        #[inline(always)]
        pub const fn pselmosi(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "Pin select for CSN."]
        #[inline(always)]
        pub const fn pselcsn(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
        }
        #[doc = "RX data pointer."]
        #[inline(always)]
        pub const fn rxdptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0534usize) as _) }
        }
        #[doc = "Maximum number of bytes in the receive buffer."]
        #[inline(always)]
        pub const fn maxrx(self) -> crate::common::Reg<regs::Maxrx, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0538usize) as _) }
        }
        #[doc = "Number of bytes received in last granted transaction."]
        #[inline(always)]
        pub const fn amountrx(self) -> crate::common::Reg<regs::Amountrx, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x053cusize) as _) }
        }
        #[doc = "TX data pointer."]
        #[inline(always)]
        pub const fn txdptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0544usize) as _) }
        }
        #[doc = "Maximum number of bytes in the transmit buffer."]
        #[inline(always)]
        pub const fn maxtx(self) -> crate::common::Reg<regs::Maxtx, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0548usize) as _) }
        }
        #[doc = "Number of bytes transmitted in last granted transaction."]
        #[inline(always)]
        pub const fn amounttx(self) -> crate::common::Reg<regs::Amounttx, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x054cusize) as _) }
        }
        #[doc = "Configuration register."]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0554usize) as _) }
        }
        #[doc = "Default character."]
        #[inline(always)]
        pub const fn def(self) -> crate::common::Reg<regs::Def, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x055cusize) as _) }
        }
        #[doc = "Over-read character."]
        #[inline(always)]
        pub const fn orc(self) -> crate::common::Reg<regs::Orc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05c0usize) as _) }
        }
        #[doc = "Peripheral power control."]
        #[inline(always)]
        pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Number of bytes received in last granted transaction."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Amountrx(pub u32);
        impl Amountrx {
            #[doc = "Number of bytes received in last granted transaction."]
            #[must_use]
            #[inline(always)]
            pub const fn amountrx(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Number of bytes received in last granted transaction."]
            #[inline(always)]
            pub const fn set_amountrx(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Amountrx {
            #[inline(always)]
            fn default() -> Amountrx {
                Amountrx(0)
            }
        }
        impl core::fmt::Debug for Amountrx {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Amountrx")
                    .field("amountrx", &self.amountrx())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Amountrx {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Amountrx {{ amountrx: {=u8:?} }}", self.amountrx())
            }
        }
        #[doc = "Number of bytes transmitted in last granted transaction."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Amounttx(pub u32);
        impl Amounttx {
            #[doc = "Number of bytes transmitted in last granted transaction."]
            #[must_use]
            #[inline(always)]
            pub const fn amounttx(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Number of bytes transmitted in last granted transaction."]
            #[inline(always)]
            pub const fn set_amounttx(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Amounttx {
            #[inline(always)]
            fn default() -> Amounttx {
                Amounttx(0)
            }
        }
        impl core::fmt::Debug for Amounttx {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Amounttx")
                    .field("amounttx", &self.amounttx())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Amounttx {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Amounttx {{ amounttx: {=u8:?} }}", self.amounttx())
            }
        }
        #[doc = "Configuration register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Bit order."]
            #[must_use]
            #[inline(always)]
            pub const fn order(&self) -> super::vals::Order {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Order::from_bits(val as u8)
            }
            #[doc = "Bit order."]
            #[inline(always)]
            pub const fn set_order(&mut self, val: super::vals::Order) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Serial clock (SCK) phase."]
            #[must_use]
            #[inline(always)]
            pub const fn cpha(&self) -> super::vals::Cpha {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Cpha::from_bits(val as u8)
            }
            #[doc = "Serial clock (SCK) phase."]
            #[inline(always)]
            pub const fn set_cpha(&mut self, val: super::vals::Cpha) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Serial clock (SCK) polarity."]
            #[must_use]
            #[inline(always)]
            pub const fn cpol(&self) -> super::vals::Cpol {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Cpol::from_bits(val as u8)
            }
            #[doc = "Serial clock (SCK) polarity."]
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
        #[doc = "Default character."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Def(pub u32);
        impl Def {
            #[doc = "Default character."]
            #[must_use]
            #[inline(always)]
            pub const fn def(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Default character."]
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
        #[doc = "Enable SPIS."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable SPIS."]
            #[must_use]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable SPIS."]
            #[inline(always)]
            pub const fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
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
        #[doc = "Interrupt enable clear register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Disable interrupt on END event."]
            #[must_use]
            #[inline(always)]
            pub const fn end(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on END event."]
            #[inline(always)]
            pub const fn set_end(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Disable interrupt on ENDRX event."]
            #[must_use]
            #[inline(always)]
            pub const fn endrx(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on ENDRX event."]
            #[inline(always)]
            pub const fn set_endrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Disable interrupt on ACQUIRED event."]
            #[must_use]
            #[inline(always)]
            pub const fn acquired(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on ACQUIRED event."]
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
        #[doc = "Maximum number of bytes in the receive buffer."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Maxrx(pub u32);
        impl Maxrx {
            #[doc = "Maximum number of bytes in the receive buffer."]
            #[must_use]
            #[inline(always)]
            pub const fn maxrx(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Maximum number of bytes in the receive buffer."]
            #[inline(always)]
            pub const fn set_maxrx(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Maxrx {
            #[inline(always)]
            fn default() -> Maxrx {
                Maxrx(0)
            }
        }
        impl core::fmt::Debug for Maxrx {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Maxrx")
                    .field("maxrx", &self.maxrx())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Maxrx {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Maxrx {{ maxrx: {=u8:?} }}", self.maxrx())
            }
        }
        #[doc = "Maximum number of bytes in the transmit buffer."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Maxtx(pub u32);
        impl Maxtx {
            #[doc = "Maximum number of bytes in the transmit buffer."]
            #[must_use]
            #[inline(always)]
            pub const fn maxtx(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Maximum number of bytes in the transmit buffer."]
            #[inline(always)]
            pub const fn set_maxtx(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Maxtx {
            #[inline(always)]
            fn default() -> Maxtx {
                Maxtx(0)
            }
        }
        impl core::fmt::Debug for Maxtx {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Maxtx")
                    .field("maxtx", &self.maxtx())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Maxtx {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Maxtx {{ maxtx: {=u8:?} }}", self.maxtx())
            }
        }
        #[doc = "Over-read character."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Orc(pub u32);
        impl Orc {
            #[doc = "Over-read character."]
            #[must_use]
            #[inline(always)]
            pub const fn orc(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Over-read character."]
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
        #[doc = "Peripheral power control."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Power(pub u32);
        impl Power {
            #[doc = "Peripheral power control."]
            #[must_use]
            #[inline(always)]
            pub const fn power(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Peripheral power control."]
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
        #[doc = "Semaphore status."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Semstat(pub u32);
        impl Semstat {
            #[doc = "Semaphore status."]
            #[must_use]
            #[inline(always)]
            pub const fn semstat(&self) -> super::vals::Semstat {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Semstat::from_bits(val as u8)
            }
            #[doc = "Semaphore status."]
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
        #[doc = "Shortcuts for SPIS."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between END event and the ACQUIRE task."]
            #[must_use]
            #[inline(always)]
            pub const fn end_acquire(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between END event and the ACQUIRE task."]
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
        #[doc = "Status from last transaction."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Status(pub u32);
        impl Status {
            #[doc = "TX buffer overread detected, and prevented."]
            #[must_use]
            #[inline(always)]
            pub const fn overread(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "TX buffer overread detected, and prevented."]
            #[inline(always)]
            pub const fn set_overread(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "RX buffer overflow detected, and prevented."]
            #[must_use]
            #[inline(always)]
            pub const fn overflow(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "RX buffer overflow detected, and prevented."]
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
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Cpha {
            #[doc = "Sample on leading edge of the clock. Shift serial data on trailing edge."]
            LEADING = 0x0,
            #[doc = "Sample on trailing edge of the clock. Shift serial data on leading edge."]
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
            #[doc = "Active high."]
            ACTIVE_HIGH = 0x0,
            #[doc = "Active low."]
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
            #[doc = "Disabled SPIS."]
            DISABLED = 0x0,
            _RESERVED_1 = 0x01,
            #[doc = "Enable SPIS."]
            ENABLED = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl Enable {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Enable {
                unsafe { core::mem::transmute(val & 0x07) }
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
            #[doc = "Most significant bit transmitted out first."]
            MSB_FIRST = 0x0,
            #[doc = "Least significant bit transmitted out first."]
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
        pub enum Semstat {
            #[doc = "Semaphore is free."]
            FREE = 0x0,
            #[doc = "Semaphore is assigned to the CPU."]
            CPU = 0x01,
            #[doc = "Semaphore is assigned to the SPIS."]
            SPIS = 0x02,
            #[doc = "Semaphore is assigned to the SPIS, but a handover to the CPU is pending."]
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
    }
}
pub mod swi {
    #[doc = "SW Interrupts."]
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
    #[doc = "Temperature Sensor."]
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
        #[doc = "Start temperature measurement."]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop temperature measurement."]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Temperature measurement complete, data ready event."]
        #[inline(always)]
        pub const fn events_datardy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Interrupt enable set register."]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Interrupt enable clear register."]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Die temperature in degC, 2's complement format, 0.25 degC pecision."]
        #[inline(always)]
        pub const fn temp(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Peripheral power control."]
        #[inline(always)]
        pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Interrupt enable clear register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Disable interrupt on DATARDY event."]
            #[must_use]
            #[inline(always)]
            pub const fn datardy(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on DATARDY event."]
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
        #[doc = "Peripheral power control."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Power(pub u32);
        impl Power {
            #[doc = "Peripheral power control."]
            #[must_use]
            #[inline(always)]
            pub const fn power(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Peripheral power control."]
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
    }
}
pub mod timer {
    #[doc = "Timer 0."]
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
        #[doc = "Start Timer."]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop Timer."]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Increment Timer (In counter mode)."]
        #[inline(always)]
        pub const fn tasks_count(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Clear timer."]
        #[inline(always)]
        pub const fn tasks_clear(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Shutdown timer."]
        #[inline(always)]
        pub const fn tasks_shutdown(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Capture Timer value to CC\\[n\\] registers."]
        #[inline(always)]
        pub const fn tasks_capture(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize + n * 4usize) as _) }
        }
        #[doc = "Compare event on CC\\[n\\] match."]
        #[inline(always)]
        pub const fn events_compare(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize + n * 4usize) as _) }
        }
        #[doc = "Shortcuts for Timer."]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Interrupt enable set register."]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Interrupt enable clear register."]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Timer Mode selection."]
        #[inline(always)]
        pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Sets timer behaviour."]
        #[inline(always)]
        pub const fn bitmode(self) -> crate::common::Reg<regs::Bitmode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "4-bit prescaler to source clock frequency (max value 9). Source clock frequency is divided by 2^SCALE."]
        #[inline(always)]
        pub const fn prescaler(self) -> crate::common::Reg<regs::Prescaler, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "Capture/compare registers."]
        #[inline(always)]
        pub const fn cc(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize + n * 4usize) as _) }
        }
        #[doc = "Peripheral power control."]
        #[inline(always)]
        pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Sets timer behaviour."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Bitmode(pub u32);
        impl Bitmode {
            #[doc = "Sets timer behaviour ro be like the implementation of a timer with width as indicated."]
            #[must_use]
            #[inline(always)]
            pub const fn bitmode(&self) -> super::vals::Bitmode {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Bitmode::from_bits(val as u8)
            }
            #[doc = "Sets timer behaviour ro be like the implementation of a timer with width as indicated."]
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
        #[doc = "Interrupt enable clear register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Disable interrupt on COMPARE\\[0\\]"]
            #[must_use]
            #[inline(always)]
            pub const fn compare(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 16usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on COMPARE\\[0\\]"]
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
                defmt :: write ! (f , "Int {{ compare[0]: {=bool:?}, compare[1]: {=bool:?}, compare[2]: {=bool:?}, compare[3]: {=bool:?} }}" , self . compare (0usize) , self . compare (1usize) , self . compare (2usize) , self . compare (3usize))
            }
        }
        #[doc = "Timer Mode selection."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Mode(pub u32);
        impl Mode {
            #[doc = "Select Normal or Counter mode."]
            #[must_use]
            #[inline(always)]
            pub const fn mode(&self) -> super::vals::Mode {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Mode::from_bits(val as u8)
            }
            #[doc = "Select Normal or Counter mode."]
            #[inline(always)]
            pub const fn set_mode(&mut self, val: super::vals::Mode) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
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
        #[doc = "Peripheral power control."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Power(pub u32);
        impl Power {
            #[doc = "Peripheral power control."]
            #[must_use]
            #[inline(always)]
            pub const fn power(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Peripheral power control."]
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
        #[doc = "4-bit prescaler to source clock frequency (max value 9). Source clock frequency is divided by 2^SCALE."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Prescaler(pub u32);
        impl Prescaler {
            #[doc = "Timer PRESCALER value. Max value is 9."]
            #[must_use]
            #[inline(always)]
            pub const fn prescaler(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "Timer PRESCALER value. Max value is 9."]
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
        #[doc = "Shortcuts for Timer."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between CC\\[0\\] event and the CLEAR task."]
            #[must_use]
            #[inline(always)]
            pub const fn compare_clear(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between CC\\[0\\] event and the CLEAR task."]
            #[inline(always)]
            pub const fn set_compare_clear(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Shortcut between CC\\[0\\] event and the STOP task."]
            #[must_use]
            #[inline(always)]
            pub const fn compare_stop(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 8usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between CC\\[0\\] event and the STOP task."]
            #[inline(always)]
            pub const fn set_compare_stop(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
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
        impl core::fmt::Debug for Shorts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Shorts")
                    .field("compare_clear[0]", &self.compare_clear(0usize))
                    .field("compare_clear[1]", &self.compare_clear(1usize))
                    .field("compare_clear[2]", &self.compare_clear(2usize))
                    .field("compare_clear[3]", &self.compare_clear(3usize))
                    .field("compare_stop[0]", &self.compare_stop(0usize))
                    .field("compare_stop[1]", &self.compare_stop(1usize))
                    .field("compare_stop[2]", &self.compare_stop(2usize))
                    .field("compare_stop[3]", &self.compare_stop(3usize))
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Shorts {{ compare_clear[0]: {=bool:?}, compare_clear[1]: {=bool:?}, compare_clear[2]: {=bool:?}, compare_clear[3]: {=bool:?}, compare_stop[0]: {=bool:?}, compare_stop[1]: {=bool:?}, compare_stop[2]: {=bool:?}, compare_stop[3]: {=bool:?} }}" , self . compare_clear (0usize) , self . compare_clear (1usize) , self . compare_clear (2usize) , self . compare_clear (3usize) , self . compare_stop (0usize) , self . compare_stop (1usize) , self . compare_stop (2usize) , self . compare_stop (3usize))
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Bitmode {
            #[doc = "16-bit timer behaviour."]
            _16BIT = 0x0,
            #[doc = "8-bit timer behaviour."]
            _08BIT = 0x01,
            #[doc = "24-bit timer behaviour."]
            _24BIT = 0x02,
            #[doc = "32-bit timer behaviour."]
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
            #[doc = "Timer in Normal mode."]
            TIMER = 0x0,
            #[doc = "Timer in Counter mode."]
            COUNTER = 0x01,
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
    }
}
pub mod twi {
    #[doc = "Two-wire interface master 0."]
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
        #[doc = "Start 2-Wire master receive sequence."]
        #[inline(always)]
        pub const fn tasks_startrx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Start 2-Wire master transmit sequence."]
        #[inline(always)]
        pub const fn tasks_starttx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Stop 2-Wire transaction."]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Suspend 2-Wire transaction."]
        #[inline(always)]
        pub const fn tasks_suspend(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "Resume 2-Wire transaction."]
        #[inline(always)]
        pub const fn tasks_resume(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "Two-wire stopped."]
        #[inline(always)]
        pub const fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Two-wire ready to deliver new RXD byte received."]
        #[inline(always)]
        pub const fn events_rxdready(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "Two-wire finished sending last TXD byte."]
        #[inline(always)]
        pub const fn events_txdsent(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
        }
        #[doc = "Two-wire error detected."]
        #[inline(always)]
        pub const fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
        }
        #[doc = "Two-wire byte boundary."]
        #[inline(always)]
        pub const fn events_bb(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
        }
        #[doc = "Two-wire suspended."]
        #[inline(always)]
        pub const fn events_suspended(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
        }
        #[doc = "Shortcuts for TWI."]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Interrupt enable set register."]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Interrupt enable clear register."]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Two-wire error source. Write error field to 1 to clear error."]
        #[inline(always)]
        pub const fn errorsrc(self) -> crate::common::Reg<regs::Errorsrc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04c4usize) as _) }
        }
        #[doc = "Enable two-wire master."]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Pin select for SCL."]
        #[inline(always)]
        pub const fn pselscl(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Pin select for SDA."]
        #[inline(always)]
        pub const fn pselsda(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "RX data register."]
        #[inline(always)]
        pub const fn rxd(self) -> crate::common::Reg<regs::Rxd, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
        #[doc = "TX data register."]
        #[inline(always)]
        pub const fn txd(self) -> crate::common::Reg<regs::Txd, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
        }
        #[doc = "Two-wire frequency."]
        #[inline(always)]
        pub const fn frequency(self) -> crate::common::Reg<regs::Frequency, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
        }
        #[doc = "Address used in the two-wire transfer."]
        #[inline(always)]
        pub const fn address(self) -> crate::common::Reg<regs::Address, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0588usize) as _) }
        }
        #[doc = "Peripheral power control."]
        #[inline(always)]
        pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Address used in the two-wire transfer."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Address(pub u32);
        impl Address {
            #[doc = "Two-wire address."]
            #[must_use]
            #[inline(always)]
            pub const fn address(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "Two-wire address."]
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
        #[doc = "Enable two-wire master."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable W2M"]
            #[must_use]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable W2M"]
            #[inline(always)]
            pub const fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
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
        #[doc = "Two-wire error source. Write error field to 1 to clear error."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Errorsrc(pub u32);
        impl Errorsrc {
            #[doc = "Byte received in RXD register before read of the last received byte (data loss)."]
            #[must_use]
            #[inline(always)]
            pub const fn overrun(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Byte received in RXD register before read of the last received byte (data loss)."]
            #[inline(always)]
            pub const fn set_overrun(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "NACK received after sending the address."]
            #[must_use]
            #[inline(always)]
            pub const fn anack(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "NACK received after sending the address."]
            #[inline(always)]
            pub const fn set_anack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "NACK received after sending a data byte."]
            #[must_use]
            #[inline(always)]
            pub const fn dnack(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "NACK received after sending a data byte."]
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
        #[doc = "Two-wire frequency."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Frequency(pub u32);
        impl Frequency {
            #[doc = "Two-wire master clock frequency."]
            #[must_use]
            #[inline(always)]
            pub const fn frequency(&self) -> super::vals::Frequency {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Frequency::from_bits(val as u32)
            }
            #[doc = "Two-wire master clock frequency."]
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
        #[doc = "Interrupt enable clear register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Disable interrupt on STOPPED event."]
            #[must_use]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on STOPPED event."]
            #[inline(always)]
            pub const fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Disable interrupt on RXDREADY event."]
            #[must_use]
            #[inline(always)]
            pub const fn rxdready(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on RXDREADY event."]
            #[inline(always)]
            pub const fn set_rxdready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Disable interrupt on TXDSENT event."]
            #[must_use]
            #[inline(always)]
            pub const fn txdsent(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on TXDSENT event."]
            #[inline(always)]
            pub const fn set_txdsent(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Disable interrupt on ERROR event."]
            #[must_use]
            #[inline(always)]
            pub const fn error(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on ERROR event."]
            #[inline(always)]
            pub const fn set_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Disable interrupt on BB event."]
            #[must_use]
            #[inline(always)]
            pub const fn bb(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on BB event."]
            #[inline(always)]
            pub const fn set_bb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Disable interrupt on SUSPENDED event."]
            #[must_use]
            #[inline(always)]
            pub const fn suspended(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on SUSPENDED event."]
            #[inline(always)]
            pub const fn set_suspended(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
                    .field("rxdready", &self.rxdready())
                    .field("txdsent", &self.txdsent())
                    .field("error", &self.error())
                    .field("bb", &self.bb())
                    .field("suspended", &self.suspended())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Int {{ stopped: {=bool:?}, rxdready: {=bool:?}, txdsent: {=bool:?}, error: {=bool:?}, bb: {=bool:?}, suspended: {=bool:?} }}" , self . stopped () , self . rxdready () , self . txdsent () , self . error () , self . bb () , self . suspended ())
            }
        }
        #[doc = "Peripheral power control."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Power(pub u32);
        impl Power {
            #[doc = "Peripheral power control."]
            #[must_use]
            #[inline(always)]
            pub const fn power(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Peripheral power control."]
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
        #[doc = "RX data register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxd(pub u32);
        impl Rxd {
            #[doc = "RX data from last transfer."]
            #[must_use]
            #[inline(always)]
            pub const fn rxd(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "RX data from last transfer."]
            #[inline(always)]
            pub const fn set_rxd(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Rxd {
            #[inline(always)]
            fn default() -> Rxd {
                Rxd(0)
            }
        }
        impl core::fmt::Debug for Rxd {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rxd").field("rxd", &self.rxd()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rxd {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Rxd {{ rxd: {=u8:?} }}", self.rxd())
            }
        }
        #[doc = "Shortcuts for TWI."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between BB event and the SUSPEND task."]
            #[must_use]
            #[inline(always)]
            pub const fn bb_suspend(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between BB event and the SUSPEND task."]
            #[inline(always)]
            pub const fn set_bb_suspend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Shortcut between BB event and the STOP task."]
            #[must_use]
            #[inline(always)]
            pub const fn bb_stop(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between BB event and the STOP task."]
            #[inline(always)]
            pub const fn set_bb_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
                    .field("bb_suspend", &self.bb_suspend())
                    .field("bb_stop", &self.bb_stop())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Shorts {{ bb_suspend: {=bool:?}, bb_stop: {=bool:?} }}",
                    self.bb_suspend(),
                    self.bb_stop()
                )
            }
        }
        #[doc = "TX data register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Txd(pub u32);
        impl Txd {
            #[doc = "TX data for next transfer."]
            #[must_use]
            #[inline(always)]
            pub const fn txd(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "TX data for next transfer."]
            #[inline(always)]
            pub const fn set_txd(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Txd {
            #[inline(always)]
            fn default() -> Txd {
                Txd(0)
            }
        }
        impl core::fmt::Debug for Txd {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Txd").field("txd", &self.txd()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Txd {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Txd {{ txd: {=u8:?} }}", self.txd())
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Enable {
            #[doc = "Disabled."]
            DISABLED = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            #[doc = "Enabled."]
            ENABLED = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl Enable {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Enable {
                unsafe { core::mem::transmute(val & 0x07) }
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
            #[doc = "100 kbps."]
            pub const K100: Self = Self(0x0198_0000);
            #[doc = "250 kbps."]
            pub const K250: Self = Self(0x0400_0000);
            #[doc = "400 kbps (actual rate 410.256 kbps)."]
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
        impl core::fmt::Debug for Frequency {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0198_0000 => f.write_str("K100"),
                    0x0400_0000 => f.write_str("K250"),
                    0x0668_0000 => f.write_str("K400"),
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
                    0x0668_0000 => defmt::write!(f, "K400"),
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
    }
}
pub mod uart {
    #[doc = "Universal Asynchronous Receiver/Transmitter."]
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
        #[doc = "Start UART receiver."]
        #[inline(always)]
        pub const fn tasks_startrx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop UART receiver."]
        #[inline(always)]
        pub const fn tasks_stoprx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Start UART transmitter."]
        #[inline(always)]
        pub const fn tasks_starttx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Stop UART transmitter."]
        #[inline(always)]
        pub const fn tasks_stoptx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Suspend UART."]
        #[inline(always)]
        pub const fn tasks_suspend(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "CTS activated."]
        #[inline(always)]
        pub const fn events_cts(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "CTS deactivated."]
        #[inline(always)]
        pub const fn events_ncts(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Data received in RXD."]
        #[inline(always)]
        pub const fn events_rxdrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "Data sent from TXD."]
        #[inline(always)]
        pub const fn events_txdrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
        }
        #[doc = "Error detected."]
        #[inline(always)]
        pub const fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
        }
        #[doc = "Receiver timeout."]
        #[inline(always)]
        pub const fn events_rxto(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
        }
        #[doc = "Shortcuts for UART."]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Interrupt enable set register."]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Interrupt enable clear register."]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Error source. Write error field to 1 to clear error."]
        #[inline(always)]
        pub const fn errorsrc(self) -> crate::common::Reg<regs::Errorsrc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0480usize) as _) }
        }
        #[doc = "Enable UART and acquire IOs."]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Pin select for RTS."]
        #[inline(always)]
        pub const fn pselrts(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Pin select for TXD."]
        #[inline(always)]
        pub const fn pseltxd(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Pin select for CTS."]
        #[inline(always)]
        pub const fn pselcts(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "Pin select for RXD."]
        #[inline(always)]
        pub const fn pselrxd(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
        }
        #[doc = "RXD register. On read action the buffer pointer is displaced. Once read the character is consumed. If read when no character available, the UART will stop working."]
        #[inline(always)]
        pub const fn rxd(self) -> crate::common::Reg<regs::Rxd, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
        #[doc = "TXD register."]
        #[inline(always)]
        pub const fn txd(self) -> crate::common::Reg<regs::Txd, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
        }
        #[doc = "UART Baudrate."]
        #[inline(always)]
        pub const fn baudrate(self) -> crate::common::Reg<regs::Baudrate, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
        }
        #[doc = "Configuration of parity and hardware flow control register."]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x056cusize) as _) }
        }
        #[doc = "Peripheral power control."]
        #[inline(always)]
        pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "UART Baudrate."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Baudrate(pub u32);
        impl Baudrate {
            #[doc = "UART baudrate."]
            #[must_use]
            #[inline(always)]
            pub const fn baudrate(&self) -> super::vals::Baudrate {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Baudrate::from_bits(val as u32)
            }
            #[doc = "UART baudrate."]
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
        #[doc = "Configuration of parity and hardware flow control register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Hardware flow control."]
            #[must_use]
            #[inline(always)]
            pub const fn hwfc(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Hardware flow control."]
            #[inline(always)]
            pub const fn set_hwfc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Include parity bit."]
            #[must_use]
            #[inline(always)]
            pub const fn parity(&self) -> super::vals::ConfigParity {
                let val = (self.0 >> 1usize) & 0x07;
                super::vals::ConfigParity::from_bits(val as u8)
            }
            #[doc = "Include parity bit."]
            #[inline(always)]
            pub const fn set_parity(&mut self, val: super::vals::ConfigParity) {
                self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
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
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Config {{ hwfc: {=bool:?}, parity: {:?} }}",
                    self.hwfc(),
                    self.parity()
                )
            }
        }
        #[doc = "Enable UART and acquire IOs."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable UART and acquire IOs."]
            #[must_use]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable UART and acquire IOs."]
            #[inline(always)]
            pub const fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
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
        #[doc = "Error source. Write error field to 1 to clear error."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Errorsrc(pub u32);
        impl Errorsrc {
            #[doc = "A start bit is received while the previous data still lies in RXD. (Data loss)."]
            #[must_use]
            #[inline(always)]
            pub const fn overrun(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "A start bit is received while the previous data still lies in RXD. (Data loss)."]
            #[inline(always)]
            pub const fn set_overrun(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "A character with bad parity is received. Only checked if HW parity control is enabled."]
            #[must_use]
            #[inline(always)]
            pub const fn parity(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "A character with bad parity is received. Only checked if HW parity control is enabled."]
            #[inline(always)]
            pub const fn set_parity(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "A valid stop bit is not detected on the serial data input after all bits in a character have been received."]
            #[must_use]
            #[inline(always)]
            pub const fn framing(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "A valid stop bit is not detected on the serial data input after all bits in a character have been received."]
            #[inline(always)]
            pub const fn set_framing(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "The serial data input is '0' for longer than the length of a data frame."]
            #[must_use]
            #[inline(always)]
            pub const fn break_(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "The serial data input is '0' for longer than the length of a data frame."]
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
        #[doc = "Interrupt enable clear register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Disable interrupt on CTS event."]
            #[must_use]
            #[inline(always)]
            pub const fn cts(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on CTS event."]
            #[inline(always)]
            pub const fn set_cts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Disable interrupt on NCTS event."]
            #[must_use]
            #[inline(always)]
            pub const fn ncts(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on NCTS event."]
            #[inline(always)]
            pub const fn set_ncts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Disable interrupt on RXRDY event."]
            #[must_use]
            #[inline(always)]
            pub const fn rxdrdy(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on RXRDY event."]
            #[inline(always)]
            pub const fn set_rxdrdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Disable interrupt on TXRDY event."]
            #[must_use]
            #[inline(always)]
            pub const fn txdrdy(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on TXRDY event."]
            #[inline(always)]
            pub const fn set_txdrdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Disable interrupt on ERROR event."]
            #[must_use]
            #[inline(always)]
            pub const fn error(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on ERROR event."]
            #[inline(always)]
            pub const fn set_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Disable interrupt on RXTO event."]
            #[must_use]
            #[inline(always)]
            pub const fn rxto(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on RXTO event."]
            #[inline(always)]
            pub const fn set_rxto(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
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
                    .field("txdrdy", &self.txdrdy())
                    .field("error", &self.error())
                    .field("rxto", &self.rxto())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Int {{ cts: {=bool:?}, ncts: {=bool:?}, rxdrdy: {=bool:?}, txdrdy: {=bool:?}, error: {=bool:?}, rxto: {=bool:?} }}" , self . cts () , self . ncts () , self . rxdrdy () , self . txdrdy () , self . error () , self . rxto ())
            }
        }
        #[doc = "Peripheral power control."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Power(pub u32);
        impl Power {
            #[doc = "Peripheral power control."]
            #[must_use]
            #[inline(always)]
            pub const fn power(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Peripheral power control."]
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
        #[doc = "RXD register. On read action the buffer pointer is displaced. Once read the character is consumed. If read when no character available, the UART will stop working."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxd(pub u32);
        impl Rxd {
            #[doc = "RX data from previous transfer. Double buffered."]
            #[must_use]
            #[inline(always)]
            pub const fn rxd(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "RX data from previous transfer. Double buffered."]
            #[inline(always)]
            pub const fn set_rxd(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Rxd {
            #[inline(always)]
            fn default() -> Rxd {
                Rxd(0)
            }
        }
        impl core::fmt::Debug for Rxd {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rxd").field("rxd", &self.rxd()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rxd {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Rxd {{ rxd: {=u8:?} }}", self.rxd())
            }
        }
        #[doc = "Shortcuts for UART."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between CTS event and STARTRX task."]
            #[must_use]
            #[inline(always)]
            pub const fn cts_startrx(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between CTS event and STARTRX task."]
            #[inline(always)]
            pub const fn set_cts_startrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Shortcut between NCTS event and STOPRX task."]
            #[must_use]
            #[inline(always)]
            pub const fn ncts_stoprx(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between NCTS event and STOPRX task."]
            #[inline(always)]
            pub const fn set_ncts_stoprx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
                    .field("cts_startrx", &self.cts_startrx())
                    .field("ncts_stoprx", &self.ncts_stoprx())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Shorts {{ cts_startrx: {=bool:?}, ncts_stoprx: {=bool:?} }}",
                    self.cts_startrx(),
                    self.ncts_stoprx()
                )
            }
        }
        #[doc = "TXD register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Txd(pub u32);
        impl Txd {
            #[doc = "TX data for transfer."]
            #[must_use]
            #[inline(always)]
            pub const fn txd(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "TX data for transfer."]
            #[inline(always)]
            pub const fn set_txd(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Txd {
            #[inline(always)]
            fn default() -> Txd {
                Txd(0)
            }
        }
        impl core::fmt::Debug for Txd {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Txd").field("txd", &self.txd()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Txd {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Txd {{ txd: {=u8:?} }}", self.txd())
            }
        }
    }
    pub mod vals {
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Baudrate(u32);
        impl Baudrate {
            #[doc = "1200 baud."]
            pub const BAUD1200: Self = Self(0x0004_f000);
            #[doc = "2400 baud."]
            pub const BAUD2400: Self = Self(0x0009_d000);
            #[doc = "4800 baud."]
            pub const BAUD4800: Self = Self(0x0013_b000);
            #[doc = "9600 baud."]
            pub const BAUD9600: Self = Self(0x0027_5000);
            #[doc = "14400 baud."]
            pub const BAUD14400: Self = Self(0x003b_0000);
            #[doc = "19200 baud."]
            pub const BAUD19200: Self = Self(0x004e_a000);
            #[doc = "28800 baud."]
            pub const BAUD28800: Self = Self(0x0075_f000);
            #[doc = "31250 baud."]
            pub const BAUD31250: Self = Self(0x0080_0000);
            #[doc = "38400 baud."]
            pub const BAUD38400: Self = Self(0x009d_5000);
            #[doc = "56000 baud."]
            pub const BAUD56000: Self = Self(0x00e5_0000);
            #[doc = "57600 baud."]
            pub const BAUD57600: Self = Self(0x00eb_f000);
            #[doc = "76800 baud."]
            pub const BAUD76800: Self = Self(0x013a_9000);
            #[doc = "115200 baud."]
            pub const BAUD115200: Self = Self(0x01d7_e000);
            #[doc = "230400 baud."]
            pub const BAUD230400: Self = Self(0x03af_b000);
            #[doc = "250000 baud."]
            pub const BAUD250000: Self = Self(0x0400_0000);
            #[doc = "460800 baud."]
            pub const BAUD460800: Self = Self(0x075f_7000);
            #[doc = "921600 baud."]
            pub const BAUD921600: Self = Self(0x0ebe_d000);
            #[doc = "1M baud."]
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
                    0x003b_0000 => f.write_str("BAUD14400"),
                    0x004e_a000 => f.write_str("BAUD19200"),
                    0x0075_f000 => f.write_str("BAUD28800"),
                    0x0080_0000 => f.write_str("BAUD31250"),
                    0x009d_5000 => f.write_str("BAUD38400"),
                    0x00e5_0000 => f.write_str("BAUD56000"),
                    0x00eb_f000 => f.write_str("BAUD57600"),
                    0x013a_9000 => f.write_str("BAUD76800"),
                    0x01d7_e000 => f.write_str("BAUD115200"),
                    0x03af_b000 => f.write_str("BAUD230400"),
                    0x0400_0000 => f.write_str("BAUD250000"),
                    0x075f_7000 => f.write_str("BAUD460800"),
                    0x0ebe_d000 => f.write_str("BAUD921600"),
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
                    0x003b_0000 => defmt::write!(f, "BAUD14400"),
                    0x004e_a000 => defmt::write!(f, "BAUD19200"),
                    0x0075_f000 => defmt::write!(f, "BAUD28800"),
                    0x0080_0000 => defmt::write!(f, "BAUD31250"),
                    0x009d_5000 => defmt::write!(f, "BAUD38400"),
                    0x00e5_0000 => defmt::write!(f, "BAUD56000"),
                    0x00eb_f000 => defmt::write!(f, "BAUD57600"),
                    0x013a_9000 => defmt::write!(f, "BAUD76800"),
                    0x01d7_e000 => defmt::write!(f, "BAUD115200"),
                    0x03af_b000 => defmt::write!(f, "BAUD230400"),
                    0x0400_0000 => defmt::write!(f, "BAUD250000"),
                    0x075f_7000 => defmt::write!(f, "BAUD460800"),
                    0x0ebe_d000 => defmt::write!(f, "BAUD921600"),
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
            #[doc = "Parity bit excluded."]
            EXCLUDED = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            #[doc = "Parity bit included."]
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
            #[doc = "UART disabled."]
            DISABLED = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            #[doc = "UART enabled."]
            ENABLED = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl Enable {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Enable {
                unsafe { core::mem::transmute(val & 0x07) }
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
pub mod uicr {
    #[doc = "User Information Configuration."]
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
        #[doc = "Length of code region 0."]
        #[inline(always)]
        pub const fn clenr0(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Readback protection configuration."]
        #[inline(always)]
        pub const fn rbpconf(self) -> crate::common::Reg<regs::Rbpconf, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Reset value for CLOCK XTALFREQ register."]
        #[inline(always)]
        pub const fn xtalfreq(self) -> crate::common::Reg<regs::Xtalfreq, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Firmware ID."]
        #[inline(always)]
        pub const fn fwid(self) -> crate::common::Reg<regs::Fwid, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Bootloader start address."]
        #[inline(always)]
        pub const fn bootloaderaddr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Reserved for Nordic firmware design."]
        #[inline(always)]
        pub const fn nrffw(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 15usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize + n * 4usize) as _) }
        }
        #[doc = "Reserved for Nordic hardware design."]
        #[inline(always)]
        pub const fn nrfhw(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 12usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize + n * 4usize) as _) }
        }
        #[doc = "Reserved for customer."]
        #[inline(always)]
        pub const fn customer(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 32usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Firmware ID."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Fwid(pub u32);
        impl Fwid {
            #[doc = "Identification number for the firmware loaded into the chip."]
            #[must_use]
            #[inline(always)]
            pub const fn fwid(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Identification number for the firmware loaded into the chip."]
            #[inline(always)]
            pub const fn set_fwid(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Fwid {
            #[inline(always)]
            fn default() -> Fwid {
                Fwid(0)
            }
        }
        impl core::fmt::Debug for Fwid {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Fwid").field("fwid", &self.fwid()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Fwid {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Fwid {{ fwid: {=u16:?} }}", self.fwid())
            }
        }
        #[doc = "Readback protection configuration."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rbpconf(pub u32);
        impl Rbpconf {
            #[doc = "Readback protect region 0. Will be ignored if pre-programmed factory code is present on the chip."]
            #[must_use]
            #[inline(always)]
            pub const fn pr0(&self) -> super::vals::Pr0 {
                let val = (self.0 >> 0usize) & 0xff;
                super::vals::Pr0::from_bits(val as u8)
            }
            #[doc = "Readback protect region 0. Will be ignored if pre-programmed factory code is present on the chip."]
            #[inline(always)]
            pub const fn set_pr0(&mut self, val: super::vals::Pr0) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
            }
            #[doc = "Readback protect all code in the device."]
            #[must_use]
            #[inline(always)]
            pub const fn pall(&self) -> super::vals::Pall {
                let val = (self.0 >> 8usize) & 0xff;
                super::vals::Pall::from_bits(val as u8)
            }
            #[doc = "Readback protect all code in the device."]
            #[inline(always)]
            pub const fn set_pall(&mut self, val: super::vals::Pall) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
            }
        }
        impl Default for Rbpconf {
            #[inline(always)]
            fn default() -> Rbpconf {
                Rbpconf(0)
            }
        }
        impl core::fmt::Debug for Rbpconf {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rbpconf")
                    .field("pr0", &self.pr0())
                    .field("pall", &self.pall())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rbpconf {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Rbpconf {{ pr0: {:?}, pall: {:?} }}",
                    self.pr0(),
                    self.pall()
                )
            }
        }
        #[doc = "Reset value for CLOCK XTALFREQ register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Xtalfreq(pub u32);
        impl Xtalfreq {
            #[doc = "Reset value for CLOCK XTALFREQ register."]
            #[must_use]
            #[inline(always)]
            pub const fn xtalfreq(&self) -> super::vals::Xtalfreq {
                let val = (self.0 >> 0usize) & 0xff;
                super::vals::Xtalfreq::from_bits(val as u8)
            }
            #[doc = "Reset value for CLOCK XTALFREQ register."]
            #[inline(always)]
            pub const fn set_xtalfreq(&mut self, val: super::vals::Xtalfreq) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Xtalfreq {
            #[inline(always)]
            fn default() -> Xtalfreq {
                Xtalfreq(0)
            }
        }
        impl core::fmt::Debug for Xtalfreq {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Xtalfreq")
                    .field("xtalfreq", &self.xtalfreq())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Xtalfreq {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Xtalfreq {{ xtalfreq: {:?} }}", self.xtalfreq())
            }
        }
    }
    pub mod vals {
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pall(u8);
        impl Pall {
            #[doc = "Enabled."]
            pub const ENABLED: Self = Self(0x0);
            #[doc = "Disabled."]
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
        impl core::fmt::Debug for Pall {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0 => f.write_str("ENABLED"),
                    0xff => f.write_str("DISABLED"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Pall {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0 => defmt::write!(f, "ENABLED"),
                    0xff => defmt::write!(f, "DISABLED"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
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
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pr0(u8);
        impl Pr0 {
            #[doc = "Enabled."]
            pub const ENABLED: Self = Self(0x0);
            #[doc = "Disabled."]
            pub const DISABLED: Self = Self(0xff);
        }
        impl Pr0 {
            pub const fn from_bits(val: u8) -> Pr0 {
                Self(val & 0xff)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl core::fmt::Debug for Pr0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0 => f.write_str("ENABLED"),
                    0xff => f.write_str("DISABLED"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Pr0 {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0 => defmt::write!(f, "ENABLED"),
                    0xff => defmt::write!(f, "DISABLED"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u8> for Pr0 {
            #[inline(always)]
            fn from(val: u8) -> Pr0 {
                Pr0::from_bits(val)
            }
        }
        impl From<Pr0> for u8 {
            #[inline(always)]
            fn from(val: Pr0) -> u8 {
                Pr0::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Xtalfreq(u8);
        impl Xtalfreq {
            #[doc = "32MHz Xtal is used."]
            pub const _32MHZ: Self = Self(0x0);
            #[doc = "16MHz Xtal is used."]
            pub const _16MHZ: Self = Self(0xff);
        }
        impl Xtalfreq {
            pub const fn from_bits(val: u8) -> Xtalfreq {
                Self(val & 0xff)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl core::fmt::Debug for Xtalfreq {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0 => f.write_str("_32MHZ"),
                    0xff => f.write_str("_16MHZ"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Xtalfreq {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0 => defmt::write!(f, "_32MHZ"),
                    0xff => defmt::write!(f, "_16MHZ"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u8> for Xtalfreq {
            #[inline(always)]
            fn from(val: u8) -> Xtalfreq {
                Xtalfreq::from_bits(val)
            }
        }
        impl From<Xtalfreq> for u8 {
            #[inline(always)]
            fn from(val: Xtalfreq) -> u8 {
                Xtalfreq::to_bits(val)
            }
        }
    }
}
pub mod wdt {
    #[doc = "Watchdog Timer."]
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
        #[doc = "Start the watchdog."]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Watchdog timeout."]
        #[inline(always)]
        pub const fn events_timeout(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Interrupt enable set register."]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Interrupt enable clear register."]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Watchdog running status."]
        #[inline(always)]
        pub const fn runstatus(self) -> crate::common::Reg<regs::Runstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "Request status."]
        #[inline(always)]
        pub const fn reqstatus(self) -> crate::common::Reg<regs::Reqstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0404usize) as _) }
        }
        #[doc = "Counter reload value in number of 32kiHz clock cycles."]
        #[inline(always)]
        pub const fn crv(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Reload request enable."]
        #[inline(always)]
        pub const fn rren(self) -> crate::common::Reg<regs::Rren, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Configuration register."]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Reload requests registers."]
        #[inline(always)]
        pub const fn rr(self, n: usize) -> crate::common::Reg<regs::Rr, crate::common::W> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize + n * 4usize) as _) }
        }
        #[doc = "Peripheral power control."]
        #[inline(always)]
        pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Configuration register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Configure the watchdog to pause or not while the CPU is sleeping."]
            #[must_use]
            #[inline(always)]
            pub const fn sleep(&self) -> super::vals::Sleep {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Sleep::from_bits(val as u8)
            }
            #[doc = "Configure the watchdog to pause or not while the CPU is sleeping."]
            #[inline(always)]
            pub const fn set_sleep(&mut self, val: super::vals::Sleep) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Configure the watchdog to pause or not while the CPU is halted by the debugger."]
            #[must_use]
            #[inline(always)]
            pub const fn halt(&self) -> super::vals::Halt {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Halt::from_bits(val as u8)
            }
            #[doc = "Configure the watchdog to pause or not while the CPU is halted by the debugger."]
            #[inline(always)]
            pub const fn set_halt(&mut self, val: super::vals::Halt) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
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
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Config {{ sleep: {:?}, halt: {:?} }}",
                    self.sleep(),
                    self.halt()
                )
            }
        }
        #[doc = "Interrupt enable clear register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Disable interrupt on TIMEOUT event."]
            #[must_use]
            #[inline(always)]
            pub const fn timeout(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Disable interrupt on TIMEOUT event."]
            #[inline(always)]
            pub const fn set_timeout(&mut self, val: bool) {
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
                    .field("timeout", &self.timeout())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Int {{ timeout: {=bool:?} }}", self.timeout())
            }
        }
        #[doc = "Peripheral power control."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Power(pub u32);
        impl Power {
            #[doc = "Peripheral power control."]
            #[must_use]
            #[inline(always)]
            pub const fn power(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Peripheral power control."]
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
        #[doc = "Request status."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Reqstatus(pub u32);
        impl Reqstatus {
            #[doc = "Request status for RR\\[0\\]."]
            #[must_use]
            #[inline(always)]
            pub const fn rr(&self, n: usize) -> bool {
                assert!(n < 8usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Request status for RR\\[0\\]."]
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
        #[doc = "Reload requests registers."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rr(pub u32);
        impl Rr {
            #[doc = "Reload register."]
            #[must_use]
            #[inline(always)]
            pub const fn rr(&self) -> super::vals::Rr {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Rr::from_bits(val as u32)
            }
            #[doc = "Reload register."]
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
        #[doc = "Reload request enable."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rren(pub u32);
        impl Rren {
            #[doc = "Enable or disable RR\\[0\\] register."]
            #[must_use]
            #[inline(always)]
            pub const fn rr(&self, n: usize) -> bool {
                assert!(n < 8usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable RR\\[0\\] register."]
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
        #[doc = "Watchdog running status."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Runstatus(pub u32);
        impl Runstatus {
            #[doc = "Watchdog running status."]
            #[must_use]
            #[inline(always)]
            pub const fn runstatus(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Watchdog running status."]
            #[inline(always)]
            pub const fn set_runstatus(&mut self, val: bool) {
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
                    .field("runstatus", &self.runstatus())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Runstatus {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Runstatus {{ runstatus: {=bool:?} }}", self.runstatus())
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Halt {
            #[doc = "Pause watchdog while the CPU is halted by the debugger."]
            PAUSE = 0x0,
            #[doc = "Do not pause watchdog while the CPU is halted by the debugger."]
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
            #[doc = "Value to request a reload of the watchdog timer."]
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
            #[doc = "Pause watchdog while the CPU is asleep."]
            PAUSE = 0x0,
            #[doc = "Do not pause watchdog while the CPU is asleep."]
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
