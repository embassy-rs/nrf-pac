#![doc = "Peripheral access API (generated using chiptool v0.1.0 (4df74f6 2024-11-11))"]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - POWER_CLOCK"]
    POWER_CLOCK = 0,
    #[doc = "1 - RADIO"]
    RADIO = 1,
    #[doc = "2 - UARTE0_UART0"]
    UARTE0_UART0 = 2,
    #[doc = "3 - SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0"]
    SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 = 3,
    #[doc = "4 - SPIM1_SPIS1_TWIM1_TWIS1_SPI1_TWI1"]
    SPIM1_SPIS1_TWIM1_TWIS1_SPI1_TWI1 = 4,
    #[doc = "5 - NFCT"]
    NFCT = 5,
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
    #[doc = "19 - COMP_LPCOMP"]
    COMP_LPCOMP = 19,
    #[doc = "20 - SWI0_EGU0"]
    SWI0_EGU0 = 20,
    #[doc = "21 - SWI1_EGU1"]
    SWI1_EGU1 = 21,
    #[doc = "22 - SWI2_EGU2"]
    SWI2_EGU2 = 22,
    #[doc = "23 - SWI3_EGU3"]
    SWI3_EGU3 = 23,
    #[doc = "24 - SWI4_EGU4"]
    SWI4_EGU4 = 24,
    #[doc = "25 - SWI5_EGU5"]
    SWI5_EGU5 = 25,
    #[doc = "26 - TIMER3"]
    TIMER3 = 26,
    #[doc = "27 - TIMER4"]
    TIMER4 = 27,
    #[doc = "28 - PWM0"]
    PWM0 = 28,
    #[doc = "29 - PDM"]
    PDM = 29,
    #[doc = "32 - MWU"]
    MWU = 32,
    #[doc = "33 - PWM1"]
    PWM1 = 33,
    #[doc = "34 - PWM2"]
    PWM2 = 34,
    #[doc = "35 - SPIM2_SPIS2_SPI2"]
    SPIM2_SPIS2_SPI2 = 35,
    #[doc = "36 - RTC2"]
    RTC2 = 36,
    #[doc = "37 - I2S"]
    I2S = 37,
    #[doc = "38 - FPU"]
    FPU = 38,
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
        fn SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0();
        fn SPIM1_SPIS1_TWIM1_TWIS1_SPI1_TWI1();
        fn NFCT();
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
        fn COMP_LPCOMP();
        fn SWI0_EGU0();
        fn SWI1_EGU1();
        fn SWI2_EGU2();
        fn SWI3_EGU3();
        fn SWI4_EGU4();
        fn SWI5_EGU5();
        fn TIMER3();
        fn TIMER4();
        fn PWM0();
        fn PDM();
        fn MWU();
        fn PWM1();
        fn PWM2();
        fn SPIM2_SPIS2_SPI2();
        fn RTC2();
        fn I2S();
        fn FPU();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[link_section = ".vector_table.interrupts"]
    #[no_mangle]
    pub static __INTERRUPTS: [Vector; 39] = [
        Vector {
            _handler: POWER_CLOCK,
        },
        Vector { _handler: RADIO },
        Vector {
            _handler: UARTE0_UART0,
        },
        Vector {
            _handler: SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0,
        },
        Vector {
            _handler: SPIM1_SPIS1_TWIM1_TWIS1_SPI1_TWI1,
        },
        Vector { _handler: NFCT },
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
        Vector {
            _handler: COMP_LPCOMP,
        },
        Vector {
            _handler: SWI0_EGU0,
        },
        Vector {
            _handler: SWI1_EGU1,
        },
        Vector {
            _handler: SWI2_EGU2,
        },
        Vector {
            _handler: SWI3_EGU3,
        },
        Vector {
            _handler: SWI4_EGU4,
        },
        Vector {
            _handler: SWI5_EGU5,
        },
        Vector { _handler: TIMER3 },
        Vector { _handler: TIMER4 },
        Vector { _handler: PWM0 },
        Vector { _handler: PDM },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: MWU },
        Vector { _handler: PWM1 },
        Vector { _handler: PWM2 },
        Vector {
            _handler: SPIM2_SPIS2_SPI2,
        },
        Vector { _handler: RTC2 },
        Vector { _handler: I2S },
        Vector { _handler: FPU },
    ];
}
#[doc = "Factory Information Configuration Registers"]
pub const FICR: ficr::Ficr = unsafe { ficr::Ficr::from_ptr(0x1000_0000usize as _) };
#[doc = "User Information Configuration Registers"]
pub const UICR: uicr::Uicr = unsafe { uicr::Uicr::from_ptr(0x1000_1000usize as _) };
#[doc = "Block Protect"]
pub const BPROT: bprot::Bprot = unsafe { bprot::Bprot::from_ptr(0x4000_0000usize as _) };
#[doc = "Power control"]
pub const POWER: power::Power = unsafe { power::Power::from_ptr(0x4000_0000usize as _) };
#[doc = "Clock control"]
pub const CLOCK: clock::Clock = unsafe { clock::Clock::from_ptr(0x4000_0000usize as _) };
#[doc = "2.4 GHz Radio"]
pub const RADIO: radio::Radio = unsafe { radio::Radio::from_ptr(0x4000_1000usize as _) };
#[doc = "UART with EasyDMA"]
pub const UARTE0: uarte::Uarte = unsafe { uarte::Uarte::from_ptr(0x4000_2000usize as _) };
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub const UART0: uart::Uart = unsafe { uart::Uart::from_ptr(0x4000_2000usize as _) };
#[doc = "Serial Peripheral Interface Master with EasyDMA 0"]
pub const SPIM0: spim::Spim = unsafe { spim::Spim::from_ptr(0x4000_3000usize as _) };
#[doc = "SPI Slave 0"]
pub const SPIS0: spis::Spis = unsafe { spis::Spis::from_ptr(0x4000_3000usize as _) };
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 0"]
pub const TWIM0: twim::Twim = unsafe { twim::Twim::from_ptr(0x4000_3000usize as _) };
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 0"]
pub const TWIS0: twis::Twis = unsafe { twis::Twis::from_ptr(0x4000_3000usize as _) };
#[doc = "Serial Peripheral Interface 0"]
pub const SPI0: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3000usize as _) };
#[doc = "I2C compatible Two-Wire Interface 0"]
pub const TWI0: twi::Twi = unsafe { twi::Twi::from_ptr(0x4000_3000usize as _) };
#[doc = "Serial Peripheral Interface Master with EasyDMA 1"]
pub const SPIM1: spim::Spim = unsafe { spim::Spim::from_ptr(0x4000_4000usize as _) };
#[doc = "SPI Slave 1"]
pub const SPIS1: spis::Spis = unsafe { spis::Spis::from_ptr(0x4000_4000usize as _) };
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 1"]
pub const TWIM1: twim::Twim = unsafe { twim::Twim::from_ptr(0x4000_4000usize as _) };
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 1"]
pub const TWIS1: twis::Twis = unsafe { twis::Twis::from_ptr(0x4000_4000usize as _) };
#[doc = "Serial Peripheral Interface 1"]
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_4000usize as _) };
#[doc = "I2C compatible Two-Wire Interface 1"]
pub const TWI1: twi::Twi = unsafe { twi::Twi::from_ptr(0x4000_4000usize as _) };
#[doc = "NFC-A compatible radio"]
pub const NFCT: nfct::Nfct = unsafe { nfct::Nfct::from_ptr(0x4000_5000usize as _) };
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
#[doc = "AES CCM Mode Encryption"]
pub const CCM: ccm::Ccm = unsafe { ccm::Ccm::from_ptr(0x4000_f000usize as _) };
#[doc = "Accelerated Address Resolver"]
pub const AAR: aar::Aar = unsafe { aar::Aar::from_ptr(0x4000_f000usize as _) };
#[doc = "Watchdog Timer"]
pub const WDT: wdt::Wdt = unsafe { wdt::Wdt::from_ptr(0x4001_0000usize as _) };
#[doc = "Real time counter 1"]
pub const RTC1: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4001_1000usize as _) };
#[doc = "Quadrature Decoder"]
pub const QDEC: qdec::Qdec = unsafe { qdec::Qdec::from_ptr(0x4001_2000usize as _) };
#[doc = "Comparator"]
pub const COMP: comp::Comp = unsafe { comp::Comp::from_ptr(0x4001_3000usize as _) };
#[doc = "Low Power Comparator"]
pub const LPCOMP: lpcomp::Lpcomp = unsafe { lpcomp::Lpcomp::from_ptr(0x4001_3000usize as _) };
#[doc = "Software interrupt 0"]
pub const SWI0: swi::Swi = unsafe { swi::Swi::from_ptr(0x4001_4000usize as _) };
#[doc = "Event Generator Unit 0"]
pub const EGU0: egu::Egu = unsafe { egu::Egu::from_ptr(0x4001_4000usize as _) };
#[doc = "Software interrupt 1"]
pub const SWI1: swi::Swi = unsafe { swi::Swi::from_ptr(0x4001_5000usize as _) };
#[doc = "Event Generator Unit 1"]
pub const EGU1: egu::Egu = unsafe { egu::Egu::from_ptr(0x4001_5000usize as _) };
#[doc = "Software interrupt 2"]
pub const SWI2: swi::Swi = unsafe { swi::Swi::from_ptr(0x4001_6000usize as _) };
#[doc = "Event Generator Unit 2"]
pub const EGU2: egu::Egu = unsafe { egu::Egu::from_ptr(0x4001_6000usize as _) };
#[doc = "Software interrupt 3"]
pub const SWI3: swi::Swi = unsafe { swi::Swi::from_ptr(0x4001_7000usize as _) };
#[doc = "Event Generator Unit 3"]
pub const EGU3: egu::Egu = unsafe { egu::Egu::from_ptr(0x4001_7000usize as _) };
#[doc = "Software interrupt 4"]
pub const SWI4: swi::Swi = unsafe { swi::Swi::from_ptr(0x4001_8000usize as _) };
#[doc = "Event Generator Unit 4"]
pub const EGU4: egu::Egu = unsafe { egu::Egu::from_ptr(0x4001_8000usize as _) };
#[doc = "Software interrupt 5"]
pub const SWI5: swi::Swi = unsafe { swi::Swi::from_ptr(0x4001_9000usize as _) };
#[doc = "Event Generator Unit 5"]
pub const EGU5: egu::Egu = unsafe { egu::Egu::from_ptr(0x4001_9000usize as _) };
#[doc = "Timer/Counter 3"]
pub const TIMER3: timer3::Timer3 = unsafe { timer3::Timer3::from_ptr(0x4001_a000usize as _) };
#[doc = "Timer/Counter 4"]
pub const TIMER4: timer3::Timer3 = unsafe { timer3::Timer3::from_ptr(0x4001_b000usize as _) };
#[doc = "Pulse Width Modulation Unit 0"]
pub const PWM0: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x4001_c000usize as _) };
#[doc = "Pulse Density Modulation (Digital Microphone) Interface"]
pub const PDM: pdm::Pdm = unsafe { pdm::Pdm::from_ptr(0x4001_d000usize as _) };
#[doc = "Non Volatile Memory Controller"]
pub const NVMC: nvmc::Nvmc = unsafe { nvmc::Nvmc::from_ptr(0x4001_e000usize as _) };
#[doc = "Programmable Peripheral Interconnect"]
pub const PPI: ppi::Ppi = unsafe { ppi::Ppi::from_ptr(0x4001_f000usize as _) };
#[doc = "Memory Watch Unit"]
pub const MWU: mwu::Mwu = unsafe { mwu::Mwu::from_ptr(0x4002_0000usize as _) };
#[doc = "Pulse Width Modulation Unit 1"]
pub const PWM1: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x4002_1000usize as _) };
#[doc = "Pulse Width Modulation Unit 2"]
pub const PWM2: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x4002_2000usize as _) };
#[doc = "Serial Peripheral Interface Master with EasyDMA 2"]
pub const SPIM2: spim::Spim = unsafe { spim::Spim::from_ptr(0x4002_3000usize as _) };
#[doc = "SPI Slave 2"]
pub const SPIS2: spis::Spis = unsafe { spis::Spis::from_ptr(0x4002_3000usize as _) };
#[doc = "Serial Peripheral Interface 2"]
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0x4002_3000usize as _) };
#[doc = "Real time counter 2"]
pub const RTC2: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4002_4000usize as _) };
#[doc = "Inter-IC Sound"]
pub const I2S: i2s::I2s = unsafe { i2s::I2s::from_ptr(0x4002_5000usize as _) };
#[doc = "FPU"]
pub const FPU: fpu::Fpu = unsafe { fpu::Fpu::from_ptr(0x4002_6000usize as _) };
#[doc = "GPIO Port 1"]
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
            #[doc = "Write '1' to Disable interrupt for END event"]
            #[inline(always)]
            pub const fn end(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for END event"]
            #[inline(always)]
            pub fn set_end(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to Disable interrupt for RESOLVED event"]
            #[inline(always)]
            pub const fn resolved(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for RESOLVED event"]
            #[inline(always)]
            pub fn set_resolved(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to Disable interrupt for NOTRESOLVED event"]
            #[inline(always)]
            pub const fn notresolved(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for NOTRESOLVED event"]
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
        #[doc = "Disable protection mechanism in debug interface mode"]
        #[inline(always)]
        pub const fn disableindebug(
            self,
        ) -> crate::common::Reg<regs::Disableindebug, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0608usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn unused0(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x060cusize) as _) }
        }
        #[doc = "Block protect configuration register 2"]
        #[inline(always)]
        pub const fn config2(self) -> crate::common::Reg<regs::Config2, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0610usize) as _) }
        }
        #[doc = "Block protect configuration register 3"]
        #[inline(always)]
        pub const fn config3(self) -> crate::common::Reg<regs::Config3, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0614usize) as _) }
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
            #[doc = "Enable protection for region 48. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region48(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 48. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region48(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Enable protection for region 49. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region49(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 49. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region49(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Enable protection for region 50. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region50(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 50. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region50(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Enable protection for region 51. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region51(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 51. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region51(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "Enable protection for region 52. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region52(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 52. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region52(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "Enable protection for region 53. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region53(&self) -> bool {
                let val = (self.0 >> 21usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 53. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region53(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
            }
            #[doc = "Enable protection for region 54. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region54(&self) -> bool {
                let val = (self.0 >> 22usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 54. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region54(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
            }
            #[doc = "Enable protection for region 55. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region55(&self) -> bool {
                let val = (self.0 >> 23usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 55. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region55(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
            }
            #[doc = "Enable protection for region 56. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region56(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 56. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region56(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
            #[doc = "Enable protection for region 57. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region57(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 57. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region57(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
            #[doc = "Enable protection for region 58. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region58(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 58. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region58(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
            #[doc = "Enable protection for region 59. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region59(&self) -> bool {
                let val = (self.0 >> 27usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 59. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region59(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
            }
            #[doc = "Enable protection for region 60. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region60(&self) -> bool {
                let val = (self.0 >> 28usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 60. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region60(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
            }
            #[doc = "Enable protection for region 61. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region61(&self) -> bool {
                let val = (self.0 >> 29usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 61. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region61(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
            }
            #[doc = "Enable protection for region 62. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region62(&self) -> bool {
                let val = (self.0 >> 30usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 62. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region62(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
            }
            #[doc = "Enable protection for region 63. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region63(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 63. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region63(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
            }
        }
        impl Default for Config1 {
            #[inline(always)]
            fn default() -> Config1 {
                Config1(0)
            }
        }
        #[doc = "Block protect configuration register 2"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config2(pub u32);
        impl Config2 {
            #[doc = "Enable protection for region 64. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region64(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 64. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region64(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable protection for region 65. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region65(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 65. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region65(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable protection for region 66. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region66(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 66. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region66(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable protection for region 67. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region67(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 67. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region67(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable protection for region 68. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region68(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 68. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region68(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable protection for region 69. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region69(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 69. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region69(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable protection for region 70. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region70(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 70. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region70(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Enable protection for region 71. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region71(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 71. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region71(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Enable protection for region 72. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region72(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 72. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region72(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Enable protection for region 73. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region73(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 73. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region73(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Enable protection for region 74. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region74(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 74. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region74(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Enable protection for region 75. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region75(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 75. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region75(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "Enable protection for region 76. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region76(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 76. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region76(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "Enable protection for region 77. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region77(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 77. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region77(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Enable protection for region 78. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region78(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 78. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region78(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Enable protection for region 79. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region79(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 79. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region79(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
            #[doc = "Enable protection for region 80. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region80(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 80. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region80(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Enable protection for region 81. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region81(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 81. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region81(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Enable protection for region 82. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region82(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 82. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region82(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Enable protection for region 83. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region83(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 83. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region83(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "Enable protection for region 84. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region84(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 84. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region84(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "Enable protection for region 85. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region85(&self) -> bool {
                let val = (self.0 >> 21usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 85. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region85(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
            }
            #[doc = "Enable protection for region 86. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region86(&self) -> bool {
                let val = (self.0 >> 22usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 86. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region86(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
            }
            #[doc = "Enable protection for region 87. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region87(&self) -> bool {
                let val = (self.0 >> 23usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 87. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region87(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
            }
            #[doc = "Enable protection for region 88. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region88(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 88. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region88(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
            #[doc = "Enable protection for region 89. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region89(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 89. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region89(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
            #[doc = "Enable protection for region 90. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region90(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 90. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region90(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
            #[doc = "Enable protection for region 91. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region91(&self) -> bool {
                let val = (self.0 >> 27usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 91. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region91(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
            }
            #[doc = "Enable protection for region 92. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region92(&self) -> bool {
                let val = (self.0 >> 28usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 92. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region92(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
            }
            #[doc = "Enable protection for region 93. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region93(&self) -> bool {
                let val = (self.0 >> 29usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 93. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region93(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
            }
            #[doc = "Enable protection for region 94. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region94(&self) -> bool {
                let val = (self.0 >> 30usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 94. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region94(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
            }
            #[doc = "Enable protection for region 95. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region95(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 95. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region95(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
            }
        }
        impl Default for Config2 {
            #[inline(always)]
            fn default() -> Config2 {
                Config2(0)
            }
        }
        #[doc = "Block protect configuration register 3"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config3(pub u32);
        impl Config3 {
            #[doc = "Enable protection for region 96. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region96(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 96. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region96(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable protection for region 97. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region97(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 97. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region97(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable protection for region 98. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region98(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 98. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region98(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable protection for region 99. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region99(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 99. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region99(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable protection for region 100. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region100(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 100. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region100(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable protection for region 101. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region101(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 101. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region101(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable protection for region 102. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region102(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 102. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region102(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Enable protection for region 103. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region103(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 103. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region103(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Enable protection for region 104. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region104(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 104. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region104(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Enable protection for region 105. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region105(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 105. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region105(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Enable protection for region 106. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region106(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 106. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region106(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Enable protection for region 107. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region107(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 107. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region107(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "Enable protection for region 108. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region108(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 108. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region108(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "Enable protection for region 109. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region109(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 109. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region109(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Enable protection for region 110. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region110(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 110. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region110(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Enable protection for region 111. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region111(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 111. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region111(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
            #[doc = "Enable protection for region 112. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region112(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 112. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region112(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Enable protection for region 113. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region113(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 113. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region113(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Enable protection for region 114. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region114(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 114. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region114(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Enable protection for region 115. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region115(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 115. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region115(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "Enable protection for region 116. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region116(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 116. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region116(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "Enable protection for region 117. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region117(&self) -> bool {
                let val = (self.0 >> 21usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 117. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region117(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
            }
            #[doc = "Enable protection for region 118. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region118(&self) -> bool {
                let val = (self.0 >> 22usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 118. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region118(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
            }
            #[doc = "Enable protection for region 119. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region119(&self) -> bool {
                let val = (self.0 >> 23usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 119. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region119(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
            }
            #[doc = "Enable protection for region 120. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region120(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 120. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region120(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
            #[doc = "Enable protection for region 121. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region121(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 121. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region121(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
            #[doc = "Enable protection for region 122. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region122(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 122. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region122(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
            #[doc = "Enable protection for region 123. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region123(&self) -> bool {
                let val = (self.0 >> 27usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 123. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region123(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
            }
            #[doc = "Enable protection for region 124. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region124(&self) -> bool {
                let val = (self.0 >> 28usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 124. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region124(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
            }
            #[doc = "Enable protection for region 125. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region125(&self) -> bool {
                let val = (self.0 >> 29usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 125. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region125(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
            }
            #[doc = "Enable protection for region 126. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region126(&self) -> bool {
                let val = (self.0 >> 30usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 126. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region126(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
            }
            #[doc = "Enable protection for region 127. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region127(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 127. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region127(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
            }
        }
        impl Default for Config3 {
            #[inline(always)]
            fn default() -> Config3 {
                Config3(0)
            }
        }
        #[doc = "Disable protection mechanism in debug interface mode"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Disableindebug(pub u32);
        impl Disableindebug {
            #[doc = "Disable the protection mechanism for NVM regions while in debug interface mode. This register will only disable the protection mechanism if the device is in debug interface mode."]
            #[inline(always)]
            pub const fn disableindebug(&self) -> super::vals::Disableindebug {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Disableindebug::from_bits(val as u8)
            }
            #[doc = "Disable the protection mechanism for NVM regions while in debug interface mode. This register will only disable the protection mechanism if the device is in debug interface mode."]
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
            #[doc = "Enable in debug"]
            ENABLED = 0x0,
            #[doc = "Disable in debug"]
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
        #[doc = "CCM error event"]
        #[inline(always)]
        pub const fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "Shortcut register"]
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
            #[doc = "Write '1' to Disable interrupt for ENDKSGEN event"]
            #[inline(always)]
            pub const fn endksgen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for ENDKSGEN event"]
            #[inline(always)]
            pub fn set_endksgen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to Disable interrupt for ENDCRYPT event"]
            #[inline(always)]
            pub const fn endcrypt(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for ENDCRYPT event"]
            #[inline(always)]
            pub fn set_endcrypt(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to Disable interrupt for ERROR event"]
            #[inline(always)]
            pub const fn error(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for ERROR event"]
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
            #[doc = "The mode of operation to be used"]
            #[inline(always)]
            pub const fn mode(&self) -> super::vals::Mode {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Mode::from_bits(val as u8)
            }
            #[doc = "The mode of operation to be used"]
            #[inline(always)]
            pub fn set_mode(&mut self, val: super::vals::Mode) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Data rate that the CCM shall run in synch with"]
            #[inline(always)]
            pub const fn datarate(&self) -> super::vals::Datarate {
                let val = (self.0 >> 16usize) & 0x01;
                super::vals::Datarate::from_bits(val as u8)
            }
            #[doc = "Data rate that the CCM shall run in synch with"]
            #[inline(always)]
            pub fn set_datarate(&mut self, val: super::vals::Datarate) {
                self.0 =
                    (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
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
        #[doc = "Shortcut register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between ENDKSGEN event and CRYPT task"]
            #[inline(always)]
            pub const fn endksgen_crypt(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between ENDKSGEN event and CRYPT task"]
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
            #[doc = "In synch with 1 Mbit data rate"]
            _1MBIT = 0x0,
            #[doc = "In synch with 2 Mbit data rate"]
            _2MBIT = 0x01,
        }
        impl Datarate {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Datarate {
                unsafe { core::mem::transmute(val & 0x01) }
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
            #[doc = "Default length. Effective length of LENGTH field is 5-bit"]
            DEFAULT = 0x0,
            #[doc = "Extended length. Effective length of LENGTH field is 8-bit"]
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
        #[doc = "Clocking options for the Trace Port debug interface"]
        #[inline(always)]
        pub const fn traceconfig(self) -> crate::common::Reg<regs::Traceconfig, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x055cusize) as _) }
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
            #[doc = "Write '1' to Disable interrupt for HFCLKSTARTED event"]
            #[inline(always)]
            pub const fn hfclkstarted(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for HFCLKSTARTED event"]
            #[inline(always)]
            pub fn set_hfclkstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to Disable interrupt for LFCLKSTARTED event"]
            #[inline(always)]
            pub const fn lfclkstarted(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for LFCLKSTARTED event"]
            #[inline(always)]
            pub fn set_lfclkstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to Disable interrupt for DONE event"]
            #[inline(always)]
            pub const fn done(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for DONE event"]
            #[inline(always)]
            pub fn set_done(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Write '1' to Disable interrupt for CTTO event"]
            #[inline(always)]
            pub const fn ctto(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for CTTO event"]
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
        #[doc = "Clocking options for the Trace Port debug interface"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Traceconfig(pub u32);
        impl Traceconfig {
            #[doc = "Speed of Trace Port clock. Note that the TRACECLK pin will output this clock divided by two."]
            #[inline(always)]
            pub const fn traceportspeed(&self) -> super::vals::Traceportspeed {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Traceportspeed::from_bits(val as u8)
            }
            #[doc = "Speed of Trace Port clock. Note that the TRACECLK pin will output this clock divided by two."]
            #[inline(always)]
            pub fn set_traceportspeed(&mut self, val: super::vals::Traceportspeed) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
            #[doc = "Pin multiplexing of trace signals."]
            #[inline(always)]
            pub const fn tracemux(&self) -> super::vals::Tracemux {
                let val = (self.0 >> 16usize) & 0x03;
                super::vals::Tracemux::from_bits(val as u8)
            }
            #[doc = "Pin multiplexing of trace signals."]
            #[inline(always)]
            pub fn set_tracemux(&mut self, val: super::vals::Tracemux) {
                self.0 =
                    (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
            }
        }
        impl Default for Traceconfig {
            #[inline(always)]
            fn default() -> Traceconfig {
                Traceconfig(0)
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
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Tracemux {
            #[doc = "GPIOs multiplexed onto all trace-pins"]
            GPIO = 0x0,
            #[doc = "SWO multiplexed onto P0.18, GPIO multiplexed onto other trace pins"]
            SERIAL = 0x01,
            #[doc = "TRACECLK and TRACEDATA multiplexed onto P0.20, P0.18, P0.16, P0.15 and P0.14."]
            PARALLEL = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Tracemux {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Tracemux {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Tracemux {
            #[inline(always)]
            fn from(val: u8) -> Tracemux {
                Tracemux::from_bits(val)
            }
        }
        impl From<Tracemux> for u8 {
            #[inline(always)]
            fn from(val: Tracemux) -> u8 {
                Tracemux::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Traceportspeed {
            #[doc = "32 MHz Trace Port clock (TRACECLK = 16 MHz)"]
            _32MHZ = 0x0,
            #[doc = "16 MHz Trace Port clock (TRACECLK = 8 MHz)"]
            _16MHZ = 0x01,
            #[doc = "8 MHz Trace Port clock (TRACECLK = 4 MHz)"]
            _8MHZ = 0x02,
            #[doc = "4 MHz Trace Port clock (TRACECLK = 2 MHz)"]
            _4MHZ = 0x03,
        }
        impl Traceportspeed {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Traceportspeed {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Traceportspeed {
            #[inline(always)]
            fn from(val: u8) -> Traceportspeed {
                Traceportspeed::from_bits(val)
            }
        }
        impl From<Traceportspeed> for u8 {
            #[inline(always)]
            fn from(val: Traceportspeed) -> u8 {
                Traceportspeed::to_bits(val)
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
        #[doc = "Shortcut register"]
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
        #[doc = "Current source select on analog input"]
        #[inline(always)]
        pub const fn isource(self) -> crate::common::Reg<regs::Isource, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x053cusize) as _) }
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
            #[doc = "Enable or disable interrupt for READY event"]
            #[inline(always)]
            pub const fn ready(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for READY event"]
            #[inline(always)]
            pub fn set_ready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable interrupt for DOWN event"]
            #[inline(always)]
            pub const fn down(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for DOWN event"]
            #[inline(always)]
            pub fn set_down(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for UP event"]
            #[inline(always)]
            pub const fn up(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for UP event"]
            #[inline(always)]
            pub fn set_up(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable or disable interrupt for CROSS event"]
            #[inline(always)]
            pub const fn cross(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for CROSS event"]
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
        #[doc = "Current source select on analog input"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Isource(pub u32);
        impl Isource {
            #[doc = "Comparator hysteresis"]
            #[inline(always)]
            pub const fn isource(&self) -> super::vals::Isource {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Isource::from_bits(val as u8)
            }
            #[doc = "Comparator hysteresis"]
            #[inline(always)]
            pub fn set_isource(&mut self, val: super::vals::Isource) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Isource {
            #[inline(always)]
            fn default() -> Isource {
                Isource(0)
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
        #[doc = "Shortcut register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between READY event and SAMPLE task"]
            #[inline(always)]
            pub const fn ready_sample(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between READY event and SAMPLE task"]
            #[inline(always)]
            pub fn set_ready_sample(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Shortcut between READY event and STOP task"]
            #[inline(always)]
            pub const fn ready_stop(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between READY event and STOP task"]
            #[inline(always)]
            pub fn set_ready_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Shortcut between DOWN event and STOP task"]
            #[inline(always)]
            pub const fn down_stop(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between DOWN event and STOP task"]
            #[inline(always)]
            pub fn set_down_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Shortcut between UP event and STOP task"]
            #[inline(always)]
            pub const fn up_stop(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between UP event and STOP task"]
            #[inline(always)]
            pub fn set_up_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Shortcut between CROSS event and STOP task"]
            #[inline(always)]
            pub const fn cross_stop(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between CROSS event and STOP task"]
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
        pub enum Isource {
            #[doc = "Current source disabled"]
            OFF = 0x0,
            #[doc = "Current source enabled (+/- 2.5 uA)"]
            IEN2M_A5 = 0x01,
            #[doc = "Current source enabled (+/- 5 uA)"]
            IEN5M_A = 0x02,
            #[doc = "Current source enabled (+/- 10 uA)"]
            IEN10M_A = 0x03,
        }
        impl Isource {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Isource {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Isource {
            #[inline(always)]
            fn from(val: u8) -> Isource {
                Isource::from_bits(val)
            }
        }
        impl From<Isource> for u8 {
            #[inline(always)]
            fn from(val: Isource) -> u8 {
                Isource::to_bits(val)
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
            #[doc = "AIN7 selected as analog input"]
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
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            #[doc = "VREF = AREF (VDD &gt;= VREF &gt;= AREFMIN)"]
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
            #[doc = "Write '1' to Disable interrupt for ENDECB event"]
            #[inline(always)]
            pub const fn endecb(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for ENDECB event"]
            #[inline(always)]
            pub fn set_endecb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to Disable interrupt for ERRORECB event"]
            #[inline(always)]
            pub const fn errorecb(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for ERRORECB event"]
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
        #[doc = "Description collection\\[0\\]: Trigger 0 for triggering the corresponding TRIGGERED\\[0\\] event"]
        #[inline(always)]
        pub const fn tasks_trigger(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
            assert!(n < 16usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
        }
        #[doc = "Description collection\\[0\\]: Event number 0 generated by triggering the corresponding TRIGGER\\[0\\] task"]
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
            #[doc = "Enable or disable interrupt for TRIGGERED\\[0\\] event"]
            #[inline(always)]
            pub const fn triggered(&self, n: usize) -> bool {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for TRIGGERED\\[0\\] event"]
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
        #[doc = "Description collection\\[0\\]: Device identifier"]
        #[inline(always)]
        pub const fn deviceid(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
            assert!(n < 2usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize + n * 4usize) as _) }
        }
        #[doc = "Description collection\\[0\\]: Encryption Root, word 0"]
        #[inline(always)]
        pub const fn er(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
        }
        #[doc = "Description collection\\[0\\]: Identity Root, word 0"]
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
        #[doc = "Description collection\\[0\\]: Device address 0"]
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
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn nfc(self) -> Nfc {
            unsafe { Nfc::from_ptr(self.ptr.add(0x0450usize) as _) }
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
        #[doc = "Part Variant, Hardware version and Production configuration"]
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
        #[doc = "Description collection\\[0\\]: Unspecified"]
        #[inline(always)]
        pub const fn unused0(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 3usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize + n * 4usize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nfc {
        ptr: *mut u8,
    }
    unsafe impl Send for Nfc {}
    unsafe impl Sync for Nfc {}
    impl Nfc {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
        #[inline(always)]
        pub const fn tagheader0(self) -> crate::common::Reg<regs::Tagheader0, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
        #[inline(always)]
        pub const fn tagheader1(self) -> crate::common::Reg<regs::Tagheader1, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
        #[inline(always)]
        pub const fn tagheader2(self) -> crate::common::Reg<regs::Tagheader2, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
        #[inline(always)]
        pub const fn tagheader3(self) -> crate::common::Reg<regs::Tagheader3, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
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
        #[doc = "Slope definition A0."]
        #[inline(always)]
        pub const fn a0(self) -> crate::common::Reg<regs::A0, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Slope definition A1."]
        #[inline(always)]
        pub const fn a1(self) -> crate::common::Reg<regs::A1, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Slope definition A2."]
        #[inline(always)]
        pub const fn a2(self) -> crate::common::Reg<regs::A2, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Slope definition A3."]
        #[inline(always)]
        pub const fn a3(self) -> crate::common::Reg<regs::A3, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Slope definition A4."]
        #[inline(always)]
        pub const fn a4(self) -> crate::common::Reg<regs::A4, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Slope definition A5."]
        #[inline(always)]
        pub const fn a5(self) -> crate::common::Reg<regs::A5, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "y-intercept B0."]
        #[inline(always)]
        pub const fn b0(self) -> crate::common::Reg<regs::B0, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }
        #[doc = "y-intercept B1."]
        #[inline(always)]
        pub const fn b1(self) -> crate::common::Reg<regs::B1, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "y-intercept B2."]
        #[inline(always)]
        pub const fn b2(self) -> crate::common::Reg<regs::B2, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "y-intercept B3."]
        #[inline(always)]
        pub const fn b3(self) -> crate::common::Reg<regs::B3, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
        }
        #[doc = "y-intercept B4."]
        #[inline(always)]
        pub const fn b4(self) -> crate::common::Reg<regs::B4, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
        }
        #[doc = "y-intercept B5."]
        #[inline(always)]
        pub const fn b5(self) -> crate::common::Reg<regs::B5, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
        }
        #[doc = "Segment end T0."]
        #[inline(always)]
        pub const fn t0(self) -> crate::common::Reg<regs::T0, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
        }
        #[doc = "Segment end T1."]
        #[inline(always)]
        pub const fn t1(self) -> crate::common::Reg<regs::T1, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
        }
        #[doc = "Segment end T2."]
        #[inline(always)]
        pub const fn t2(self) -> crate::common::Reg<regs::T2, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
        }
        #[doc = "Segment end T3."]
        #[inline(always)]
        pub const fn t3(self) -> crate::common::Reg<regs::T3, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
        }
        #[doc = "Segment end T4."]
        #[inline(always)]
        pub const fn t4(self) -> crate::common::Reg<regs::T4, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Slope definition A0."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct A0(pub u32);
        impl A0 {
            #[doc = "A (slope definition) register."]
            #[inline(always)]
            pub const fn a(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "A (slope definition) register."]
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
        #[doc = "Slope definition A1."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct A1(pub u32);
        impl A1 {
            #[doc = "A (slope definition) register."]
            #[inline(always)]
            pub const fn a(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "A (slope definition) register."]
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
        #[doc = "Slope definition A2."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct A2(pub u32);
        impl A2 {
            #[doc = "A (slope definition) register."]
            #[inline(always)]
            pub const fn a(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "A (slope definition) register."]
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
        #[doc = "Slope definition A3."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct A3(pub u32);
        impl A3 {
            #[doc = "A (slope definition) register."]
            #[inline(always)]
            pub const fn a(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "A (slope definition) register."]
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
        #[doc = "Slope definition A4."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct A4(pub u32);
        impl A4 {
            #[doc = "A (slope definition) register."]
            #[inline(always)]
            pub const fn a(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "A (slope definition) register."]
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
        #[doc = "Slope definition A5."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct A5(pub u32);
        impl A5 {
            #[doc = "A (slope definition) register."]
            #[inline(always)]
            pub const fn a(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "A (slope definition) register."]
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
        #[doc = "y-intercept B0."]
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
        #[doc = "y-intercept B1."]
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
        #[doc = "y-intercept B2."]
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
        #[doc = "y-intercept B3."]
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
        #[doc = "y-intercept B4."]
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
        #[doc = "y-intercept B5."]
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
        #[doc = "Segment end T0."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct T0(pub u32);
        impl T0 {
            #[doc = "T (segment end)register."]
            #[inline(always)]
            pub const fn t(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "T (segment end)register."]
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
        #[doc = "Segment end T1."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct T1(pub u32);
        impl T1 {
            #[doc = "T (segment end)register."]
            #[inline(always)]
            pub const fn t(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "T (segment end)register."]
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
        #[doc = "Segment end T2."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct T2(pub u32);
        impl T2 {
            #[doc = "T (segment end)register."]
            #[inline(always)]
            pub const fn t(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "T (segment end)register."]
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
        #[doc = "Segment end T3."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct T3(pub u32);
        impl T3 {
            #[doc = "T (segment end)register."]
            #[inline(always)]
            pub const fn t(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "T (segment end)register."]
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
        #[doc = "Segment end T4."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct T4(pub u32);
        impl T4 {
            #[doc = "T (segment end)register."]
            #[inline(always)]
            pub const fn t(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "T (segment end)register."]
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
        #[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Tagheader0(pub u32);
        impl Tagheader0 {
            #[doc = "Default Manufacturer ID: Nordic Semiconductor ASA has ICM 0x5F"]
            #[inline(always)]
            pub const fn mfgid(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Default Manufacturer ID: Nordic Semiconductor ASA has ICM 0x5F"]
            #[inline(always)]
            pub fn set_mfgid(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "Unique identifier byte 1"]
            #[inline(always)]
            pub const fn ud1(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "Unique identifier byte 1"]
            #[inline(always)]
            pub fn set_ud1(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[doc = "Unique identifier byte 2"]
            #[inline(always)]
            pub const fn ud2(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0xff;
                val as u8
            }
            #[doc = "Unique identifier byte 2"]
            #[inline(always)]
            pub fn set_ud2(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
            }
            #[doc = "Unique identifier byte 3"]
            #[inline(always)]
            pub const fn ud3(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0xff;
                val as u8
            }
            #[doc = "Unique identifier byte 3"]
            #[inline(always)]
            pub fn set_ud3(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
            }
        }
        impl Default for Tagheader0 {
            #[inline(always)]
            fn default() -> Tagheader0 {
                Tagheader0(0)
            }
        }
        #[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Tagheader1(pub u32);
        impl Tagheader1 {
            #[doc = "Unique identifier byte 4"]
            #[inline(always)]
            pub const fn ud4(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Unique identifier byte 4"]
            #[inline(always)]
            pub fn set_ud4(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "Unique identifier byte 5"]
            #[inline(always)]
            pub const fn ud5(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "Unique identifier byte 5"]
            #[inline(always)]
            pub fn set_ud5(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[doc = "Unique identifier byte 6"]
            #[inline(always)]
            pub const fn ud6(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0xff;
                val as u8
            }
            #[doc = "Unique identifier byte 6"]
            #[inline(always)]
            pub fn set_ud6(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
            }
            #[doc = "Unique identifier byte 7"]
            #[inline(always)]
            pub const fn ud7(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0xff;
                val as u8
            }
            #[doc = "Unique identifier byte 7"]
            #[inline(always)]
            pub fn set_ud7(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
            }
        }
        impl Default for Tagheader1 {
            #[inline(always)]
            fn default() -> Tagheader1 {
                Tagheader1(0)
            }
        }
        #[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Tagheader2(pub u32);
        impl Tagheader2 {
            #[doc = "Unique identifier byte 8"]
            #[inline(always)]
            pub const fn ud8(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Unique identifier byte 8"]
            #[inline(always)]
            pub fn set_ud8(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "Unique identifier byte 9"]
            #[inline(always)]
            pub const fn ud9(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "Unique identifier byte 9"]
            #[inline(always)]
            pub fn set_ud9(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[doc = "Unique identifier byte 10"]
            #[inline(always)]
            pub const fn ud10(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0xff;
                val as u8
            }
            #[doc = "Unique identifier byte 10"]
            #[inline(always)]
            pub fn set_ud10(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
            }
            #[doc = "Unique identifier byte 11"]
            #[inline(always)]
            pub const fn ud11(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0xff;
                val as u8
            }
            #[doc = "Unique identifier byte 11"]
            #[inline(always)]
            pub fn set_ud11(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
            }
        }
        impl Default for Tagheader2 {
            #[inline(always)]
            fn default() -> Tagheader2 {
                Tagheader2(0)
            }
        }
        #[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Tagheader3(pub u32);
        impl Tagheader3 {
            #[doc = "Unique identifier byte 12"]
            #[inline(always)]
            pub const fn ud12(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Unique identifier byte 12"]
            #[inline(always)]
            pub fn set_ud12(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "Unique identifier byte 13"]
            #[inline(always)]
            pub const fn ud13(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "Unique identifier byte 13"]
            #[inline(always)]
            pub fn set_ud13(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[doc = "Unique identifier byte 14"]
            #[inline(always)]
            pub const fn ud14(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0xff;
                val as u8
            }
            #[doc = "Unique identifier byte 14"]
            #[inline(always)]
            pub fn set_ud14(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
            }
            #[doc = "Unique identifier byte 15"]
            #[inline(always)]
            pub const fn ud15(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0xff;
                val as u8
            }
            #[doc = "Unique identifier byte 15"]
            #[inline(always)]
            pub fn set_ud15(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
            }
        }
        impl Default for Tagheader3 {
            #[inline(always)]
            fn default() -> Tagheader3 {
                Tagheader3(0)
            }
        }
        #[doc = "Part Variant, Hardware version and Production configuration"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Variant(pub u32);
        impl Variant {
            #[doc = "Part Variant, Hardware version and Production configuration, encoded as ASCII"]
            #[inline(always)]
            pub const fn variant(&self) -> super::vals::Variant {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Variant::from_bits(val as u32)
            }
            #[doc = "Part Variant, Hardware version and Production configuration, encoded as ASCII"]
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
            #[doc = "128 kByte FLASH"]
            pub const K128: Self = Self(0x80);
            #[doc = "256 kByte FLASH"]
            pub const K256: Self = Self(0x0100);
            #[doc = "512 kByte FLASH"]
            pub const K512: Self = Self(0x0200);
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
            #[doc = "CHxx - 7x8 WLCSP 56 balls"]
            pub const CH: Self = Self(0x2001);
            #[doc = "CIxx - 7x8 WLCSP 56 balls"]
            pub const CI: Self = Self(0x2002);
            #[doc = "CKxx - 7x8 WLCSP 56 balls with backside coating for light protection"]
            pub const CK: Self = Self(0x2005);
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
            #[doc = "16 kByte RAM"]
            pub const K16: Self = Self(0x10);
            #[doc = "32 kByte RAM"]
            pub const K32: Self = Self(0x20);
            #[doc = "64 kByte RAM"]
            pub const K64: Self = Self(0x40);
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
            #[doc = "AAAA"]
            pub const AAAA: Self = Self(0x4141_4141);
            #[doc = "AAAB"]
            pub const AAAB: Self = Self(0x4141_4142);
            #[doc = "AAB0"]
            pub const AAB0: Self = Self(0x4141_4230);
            #[doc = "AABA"]
            pub const AABA: Self = Self(0x4141_4241);
            #[doc = "AABB"]
            pub const AABB: Self = Self(0x4141_4242);
            #[doc = "AAE0"]
            pub const AAE0: Self = Self(0x4141_4530);
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
pub mod fpu {
    #[doc = "FPU"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fpu {
        ptr: *mut u8,
    }
    unsafe impl Send for Fpu {}
    unsafe impl Sync for Fpu {}
    impl Fpu {
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
pub mod gpio {
    #[doc = "GPIO Port 1"]
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
        #[doc = "Description collection\\[0\\]: Configuration of GPIO pins"]
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
        #[doc = "Description collection\\[0\\]: Configuration of GPIO pins"]
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
        #[doc = "Description collection\\[0\\]: Task for writing to pin specified in CONFIG\\[0\\].PSEL. Action on pin is configured in CONFIG\\[0\\].POLARITY."]
        #[inline(always)]
        pub const fn tasks_out(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
        }
        #[doc = "Description collection\\[0\\]: Task for writing to pin specified in CONFIG\\[0\\].PSEL. Action on pin is to set it high."]
        #[inline(always)]
        pub const fn tasks_set(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize + n * 4usize) as _) }
        }
        #[doc = "Description collection\\[0\\]: Task for writing to pin specified in CONFIG\\[0\\].PSEL. Action on pin is to set it low."]
        #[inline(always)]
        pub const fn tasks_clr(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize + n * 4usize) as _) }
        }
        #[doc = "Description collection\\[0\\]: Event generated from pin specified in CONFIG\\[0\\].PSEL"]
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
        #[doc = "Description collection\\[0\\]: Configuration for OUT\\[n\\], SET\\[n\\] and CLR\\[n\\] tasks and IN\\[n\\] event"]
        #[inline(always)]
        pub const fn config(self, n: usize) -> crate::common::Reg<regs::Config, crate::common::RW> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize + n * 4usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Description collection\\[0\\]: Configuration for OUT\\[n\\], SET\\[n\\] and CLR\\[n\\] tasks and IN\\[n\\] event"]
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
            #[doc = "Write '1' to Disable interrupt for IN\\[0\\] event"]
            #[inline(always)]
            pub const fn in0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for IN\\[0\\] event"]
            #[inline(always)]
            pub fn set_in0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to Disable interrupt for IN\\[1\\] event"]
            #[inline(always)]
            pub const fn in1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for IN\\[1\\] event"]
            #[inline(always)]
            pub fn set_in1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to Disable interrupt for IN\\[2\\] event"]
            #[inline(always)]
            pub const fn in2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for IN\\[2\\] event"]
            #[inline(always)]
            pub fn set_in2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Write '1' to Disable interrupt for IN\\[3\\] event"]
            #[inline(always)]
            pub const fn in3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for IN\\[3\\] event"]
            #[inline(always)]
            pub fn set_in3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Write '1' to Disable interrupt for IN\\[4\\] event"]
            #[inline(always)]
            pub const fn in4(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for IN\\[4\\] event"]
            #[inline(always)]
            pub fn set_in4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Write '1' to Disable interrupt for IN\\[5\\] event"]
            #[inline(always)]
            pub const fn in5(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for IN\\[5\\] event"]
            #[inline(always)]
            pub fn set_in5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Write '1' to Disable interrupt for IN\\[6\\] event"]
            #[inline(always)]
            pub const fn in6(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for IN\\[6\\] event"]
            #[inline(always)]
            pub fn set_in6(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Write '1' to Disable interrupt for IN\\[7\\] event"]
            #[inline(always)]
            pub const fn in7(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for IN\\[7\\] event"]
            #[inline(always)]
            pub fn set_in7(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Write '1' to Disable interrupt for PORT event"]
            #[inline(always)]
            pub const fn port(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for PORT event"]
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
pub mod i2s {
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Config {
        ptr: *mut u8,
    }
    unsafe impl Send for Config {}
    unsafe impl Sync for Config {}
    impl Config {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "I2S mode."]
        #[inline(always)]
        pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Reception (RX) enable."]
        #[inline(always)]
        pub const fn rxen(self) -> crate::common::Reg<regs::Rxen, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Transmission (TX) enable."]
        #[inline(always)]
        pub const fn txen(self) -> crate::common::Reg<regs::Txen, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Master clock generator enable."]
        #[inline(always)]
        pub const fn mcken(self) -> crate::common::Reg<regs::Mcken, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Master clock generator frequency."]
        #[inline(always)]
        pub const fn mckfreq(self) -> crate::common::Reg<regs::Mckfreq, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "MCK / LRCK ratio."]
        #[inline(always)]
        pub const fn ratio(self) -> crate::common::Reg<regs::Ratio, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Sample width."]
        #[inline(always)]
        pub const fn swidth(self) -> crate::common::Reg<regs::Swidth, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }
        #[doc = "Alignment of sample within a frame."]
        #[inline(always)]
        pub const fn align(self) -> crate::common::Reg<regs::Align, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "Frame format."]
        #[inline(always)]
        pub const fn format(self) -> crate::common::Reg<regs::Format, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "Enable channels."]
        #[inline(always)]
        pub const fn channels(self) -> crate::common::Reg<regs::Channels, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
        }
    }
    #[doc = "Inter-IC Sound"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I2s {
        ptr: *mut u8,
    }
    unsafe impl Send for I2s {}
    unsafe impl Sync for I2s {}
    impl I2s {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Starts continuous I2S transfer. Also starts MCK generator when this is enabled."]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stops I2S transfer. Also stops MCK generator. Triggering this task will cause the {event:STOPPED} event to be generated."]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words that are received on the SDIN pin."]
        #[inline(always)]
        pub const fn events_rxptrupd(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "I2S transfer stopped."]
        #[inline(always)]
        pub const fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin."]
        #[inline(always)]
        pub const fn events_txptrupd(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
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
        #[doc = "Enable I2S module."]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn config(self) -> Config {
            unsafe { Config::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn rxd(self) -> Rxd {
            unsafe { Rxd::from_ptr(self.ptr.add(0x0538usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn txd(self) -> Txd {
            unsafe { Txd::from_ptr(self.ptr.add(0x0540usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn rxtxd(self) -> Rxtxd {
            unsafe { Rxtxd::from_ptr(self.ptr.add(0x0550usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn psel(self) -> Psel {
            unsafe { Psel::from_ptr(self.ptr.add(0x0560usize) as _) }
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
        #[doc = "Pin select for MCK signal."]
        #[inline(always)]
        pub const fn mck(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Pin select for SCK signal."]
        #[inline(always)]
        pub const fn sck(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Pin select for LRCK signal."]
        #[inline(always)]
        pub const fn lrck(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Pin select for SDIN signal."]
        #[inline(always)]
        pub const fn sdin(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Pin select for SDOUT signal."]
        #[inline(always)]
        pub const fn sdout(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
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
        #[doc = "Receive buffer RAM start address."]
        #[inline(always)]
        pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxtxd {
        ptr: *mut u8,
    }
    unsafe impl Send for Rxtxd {}
    unsafe impl Sync for Rxtxd {}
    impl Rxtxd {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Size of RXD and TXD buffers."]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::Maxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
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
        #[doc = "Transmit buffer RAM start address."]
        #[inline(always)]
        pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Alignment of sample within a frame."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Align(pub u32);
        impl Align {
            #[doc = "Alignment of sample within a frame."]
            #[inline(always)]
            pub const fn align(&self) -> super::vals::Align {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Align::from_bits(val as u8)
            }
            #[doc = "Alignment of sample within a frame."]
            #[inline(always)]
            pub fn set_align(&mut self, val: super::vals::Align) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Align {
            #[inline(always)]
            fn default() -> Align {
                Align(0)
            }
        }
        #[doc = "Enable channels."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Channels(pub u32);
        impl Channels {
            #[doc = "Enable channels."]
            #[inline(always)]
            pub const fn channels(&self) -> super::vals::Channels {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Channels::from_bits(val as u8)
            }
            #[doc = "Enable channels."]
            #[inline(always)]
            pub fn set_channels(&mut self, val: super::vals::Channels) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Channels {
            #[inline(always)]
            fn default() -> Channels {
                Channels(0)
            }
        }
        #[doc = "Enable I2S module."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable I2S module."]
            #[inline(always)]
            pub const fn enable(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable I2S module."]
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
        #[doc = "Frame format."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Format(pub u32);
        impl Format {
            #[doc = "Frame format."]
            #[inline(always)]
            pub const fn format(&self) -> super::vals::Format {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Format::from_bits(val as u8)
            }
            #[doc = "Frame format."]
            #[inline(always)]
            pub fn set_format(&mut self, val: super::vals::Format) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Format {
            #[inline(always)]
            fn default() -> Format {
                Format(0)
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Enable or disable interrupt for RXPTRUPD event"]
            #[inline(always)]
            pub const fn rxptrupd(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for RXPTRUPD event"]
            #[inline(always)]
            pub fn set_rxptrupd(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for STOPPED event"]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for STOPPED event"]
            #[inline(always)]
            pub fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable or disable interrupt for TXPTRUPD event"]
            #[inline(always)]
            pub const fn txptrupd(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for TXPTRUPD event"]
            #[inline(always)]
            pub fn set_txptrupd(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
        #[doc = "Size of RXD and TXD buffers."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Maxcnt(pub u32);
        impl Maxcnt {
            #[doc = "Size of RXD and TXD buffers in number of 32 bit words."]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "Size of RXD and TXD buffers in number of 32 bit words."]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for Maxcnt {
            #[inline(always)]
            fn default() -> Maxcnt {
                Maxcnt(0)
            }
        }
        #[doc = "Master clock generator enable."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Mcken(pub u32);
        impl Mcken {
            #[doc = "Master clock generator enable."]
            #[inline(always)]
            pub const fn mcken(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Master clock generator enable."]
            #[inline(always)]
            pub fn set_mcken(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Mcken {
            #[inline(always)]
            fn default() -> Mcken {
                Mcken(0)
            }
        }
        #[doc = "Master clock generator frequency."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Mckfreq(pub u32);
        impl Mckfreq {
            #[doc = "Master clock generator frequency."]
            #[inline(always)]
            pub const fn mckfreq(&self) -> super::vals::Mckfreq {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Mckfreq::from_bits(val as u32)
            }
            #[doc = "Master clock generator frequency."]
            #[inline(always)]
            pub fn set_mckfreq(&mut self, val: super::vals::Mckfreq) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Mckfreq {
            #[inline(always)]
            fn default() -> Mckfreq {
                Mckfreq(0)
            }
        }
        #[doc = "I2S mode."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Mode(pub u32);
        impl Mode {
            #[doc = "I2S mode."]
            #[inline(always)]
            pub const fn mode(&self) -> super::vals::Mode {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Mode::from_bits(val as u8)
            }
            #[doc = "I2S mode."]
            #[inline(always)]
            pub fn set_mode(&mut self, val: super::vals::Mode) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Mode {
            #[inline(always)]
            fn default() -> Mode {
                Mode(0)
            }
        }
        #[doc = "MCK / LRCK ratio."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ratio(pub u32);
        impl Ratio {
            #[doc = "MCK / LRCK ratio."]
            #[inline(always)]
            pub const fn ratio(&self) -> super::vals::Ratio {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Ratio::from_bits(val as u8)
            }
            #[doc = "MCK / LRCK ratio."]
            #[inline(always)]
            pub fn set_ratio(&mut self, val: super::vals::Ratio) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Ratio {
            #[inline(always)]
            fn default() -> Ratio {
                Ratio(0)
            }
        }
        #[doc = "Reception (RX) enable."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxen(pub u32);
        impl Rxen {
            #[doc = "Reception (RX) enable."]
            #[inline(always)]
            pub const fn rxen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Reception (RX) enable."]
            #[inline(always)]
            pub fn set_rxen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Rxen {
            #[inline(always)]
            fn default() -> Rxen {
                Rxen(0)
            }
        }
        #[doc = "Sample width."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Swidth(pub u32);
        impl Swidth {
            #[doc = "Sample width."]
            #[inline(always)]
            pub const fn swidth(&self) -> super::vals::Swidth {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Swidth::from_bits(val as u8)
            }
            #[doc = "Sample width."]
            #[inline(always)]
            pub fn set_swidth(&mut self, val: super::vals::Swidth) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Swidth {
            #[inline(always)]
            fn default() -> Swidth {
                Swidth(0)
            }
        }
        #[doc = "Transmission (TX) enable."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Txen(pub u32);
        impl Txen {
            #[doc = "Transmission (TX) enable."]
            #[inline(always)]
            pub const fn txen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Transmission (TX) enable."]
            #[inline(always)]
            pub fn set_txen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Txen {
            #[inline(always)]
            fn default() -> Txen {
                Txen(0)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Align {
            #[doc = "Left-aligned."]
            LEFT = 0x0,
            #[doc = "Right-aligned."]
            RIGHT = 0x01,
        }
        impl Align {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Align {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Align {
            #[inline(always)]
            fn from(val: u8) -> Align {
                Align::from_bits(val)
            }
        }
        impl From<Align> for u8 {
            #[inline(always)]
            fn from(val: Align) -> u8 {
                Align::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Channels {
            #[doc = "Stereo."]
            STEREO = 0x0,
            #[doc = "Left only."]
            LEFT = 0x01,
            #[doc = "Right only."]
            RIGHT = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Channels {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Channels {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Channels {
            #[inline(always)]
            fn from(val: u8) -> Channels {
                Channels::from_bits(val)
            }
        }
        impl From<Channels> for u8 {
            #[inline(always)]
            fn from(val: Channels) -> u8 {
                Channels::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Format {
            #[doc = "Original I2S format."]
            I2S = 0x0,
            #[doc = "Alternate (left- or right-aligned) format."]
            ALIGNED = 0x01,
        }
        impl Format {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Format {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Format {
            #[inline(always)]
            fn from(val: u8) -> Format {
                Format::from_bits(val)
            }
        }
        impl From<Format> for u8 {
            #[inline(always)]
            fn from(val: Format) -> u8 {
                Format::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Mckfreq(pub u32);
        impl Mckfreq {
            #[doc = "32 MHz / 125 = 0.256 MHz"]
            pub const _32MDIV125: Self = Self(0x020c_0000);
            #[doc = "32 MHz / 63 = 0.5079365 MHz"]
            pub const _32MDIV63: Self = Self(0x0410_0000);
            #[doc = "32 MHz / 42 = 0.7619048 MHz"]
            pub const _32MDIV42: Self = Self(0x0600_0000);
            #[doc = "32 MHz / 32 = 1.0 MHz"]
            pub const _32MDIV32: Self = Self(0x0800_0000);
            #[doc = "32 MHz / 31 = 1.0322581 MHz"]
            pub const _32MDIV31: Self = Self(0x0840_0000);
            #[doc = "32 MHz / 30 = 1.0666667 MHz"]
            pub const _32MDIV30: Self = Self(0x0880_0000);
            #[doc = "32 MHz / 23 = 1.3913043 MHz"]
            pub const _32MDIV23: Self = Self(0x0b00_0000);
            #[doc = "32 MHz / 21 = 1.5238095"]
            pub const _32MDIV21: Self = Self(0x0c00_0000);
            #[doc = "32 MHz / 16 = 2.0 MHz"]
            pub const _32MDIV16: Self = Self(0x1000_0000);
            #[doc = "32 MHz / 15 = 2.1333333 MHz"]
            pub const _32MDIV15: Self = Self(0x1100_0000);
            #[doc = "32 MHz / 11 = 2.9090909 MHz"]
            pub const _32MDIV11: Self = Self(0x1600_0000);
            #[doc = "32 MHz / 10 = 3.2 MHz"]
            pub const _32MDIV10: Self = Self(0x1800_0000);
            #[doc = "32 MHz / 8 = 4.0 MHz"]
            pub const _32MDIV8: Self = Self(0x2000_0000);
            #[doc = "32 MHz / 6 = 5.3333333 MHz"]
            pub const _32MDIV6: Self = Self(0x2800_0000);
            #[doc = "32 MHz / 5 = 6.4 MHz"]
            pub const _32MDIV5: Self = Self(0x3000_0000);
            #[doc = "32 MHz / 4 = 8.0 MHz"]
            pub const _32MDIV4: Self = Self(0x4000_0000);
            #[doc = "32 MHz / 3 = 10.6666667 MHz"]
            pub const _32MDIV3: Self = Self(0x5000_0000);
            #[doc = "32 MHz / 2 = 16.0 MHz"]
            pub const _32MDIV2: Self = Self(0x8000_0000);
        }
        impl Mckfreq {
            pub const fn from_bits(val: u32) -> Mckfreq {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl From<u32> for Mckfreq {
            #[inline(always)]
            fn from(val: u32) -> Mckfreq {
                Mckfreq::from_bits(val)
            }
        }
        impl From<Mckfreq> for u32 {
            #[inline(always)]
            fn from(val: Mckfreq) -> u32 {
                Mckfreq::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Mode {
            #[doc = "Master mode. SCK and LRCK generated from internal master clcok (MCK) and output on pins defined by PSEL.xxx."]
            MASTER = 0x0,
            #[doc = "Slave mode. SCK and LRCK generated by external master and received on pins defined by PSEL.xxx"]
            SLAVE = 0x01,
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
        pub enum Ratio {
            #[doc = "LRCK = MCK / 32"]
            _32X = 0x0,
            #[doc = "LRCK = MCK / 48"]
            _48X = 0x01,
            #[doc = "LRCK = MCK / 64"]
            _64X = 0x02,
            #[doc = "LRCK = MCK / 96"]
            _96X = 0x03,
            #[doc = "LRCK = MCK / 128"]
            _128X = 0x04,
            #[doc = "LRCK = MCK / 192"]
            _192X = 0x05,
            #[doc = "LRCK = MCK / 256"]
            _256X = 0x06,
            #[doc = "LRCK = MCK / 384"]
            _384X = 0x07,
            #[doc = "LRCK = MCK / 512"]
            _512X = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Ratio {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Ratio {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Ratio {
            #[inline(always)]
            fn from(val: u8) -> Ratio {
                Ratio::from_bits(val)
            }
        }
        impl From<Ratio> for u8 {
            #[inline(always)]
            fn from(val: Ratio) -> u8 {
                Ratio::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Swidth {
            #[doc = "8 bit."]
            _8BIT = 0x0,
            #[doc = "16 bit."]
            _16BIT = 0x01,
            #[doc = "24 bit."]
            _24BIT = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Swidth {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Swidth {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Swidth {
            #[inline(always)]
            fn from(val: u8) -> Swidth {
                Swidth::from_bits(val)
            }
        }
        impl From<Swidth> for u8 {
            #[inline(always)]
            fn from(val: Swidth) -> u8 {
                Swidth::to_bits(val)
            }
        }
    }
}
pub mod lpcomp {
    #[doc = "Low Power Comparator"]
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
        #[doc = "LPCOMP is ready and output is valid"]
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
        #[doc = "Shortcut register"]
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
        #[doc = "Compare result"]
        #[inline(always)]
        pub const fn result(self) -> crate::common::Reg<regs::Result, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "Enable LPCOMP"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Input pin select"]
        #[inline(always)]
        pub const fn psel(self) -> crate::common::Reg<regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Reference select"]
        #[inline(always)]
        pub const fn refsel(self) -> crate::common::Reg<regs::Refsel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "External reference select"]
        #[inline(always)]
        pub const fn extrefsel(self) -> crate::common::Reg<regs::Extrefsel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Analog detect configuration"]
        #[inline(always)]
        pub const fn anadetect(self) -> crate::common::Reg<regs::Anadetect, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize) as _) }
        }
        #[doc = "Comparator hysteresis enable"]
        #[inline(always)]
        pub const fn hyst(self) -> crate::common::Reg<regs::Hyst, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0538usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Analog detect configuration"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Anadetect(pub u32);
        impl Anadetect {
            #[doc = "Analog detect configuration"]
            #[inline(always)]
            pub const fn anadetect(&self) -> super::vals::Anadetect {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Anadetect::from_bits(val as u8)
            }
            #[doc = "Analog detect configuration"]
            #[inline(always)]
            pub fn set_anadetect(&mut self, val: super::vals::Anadetect) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Anadetect {
            #[inline(always)]
            fn default() -> Anadetect {
                Anadetect(0)
            }
        }
        #[doc = "Enable LPCOMP"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable LPCOMP"]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable LPCOMP"]
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
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Extrefsel::from_bits(val as u8)
            }
            #[doc = "External analog reference select"]
            #[inline(always)]
            pub fn set_extrefsel(&mut self, val: super::vals::Extrefsel) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
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
            #[doc = "Comparator hysteresis enable"]
            #[inline(always)]
            pub const fn hyst(&self) -> super::vals::Hyst {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Hyst::from_bits(val as u8)
            }
            #[doc = "Comparator hysteresis enable"]
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
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Write '1' to Disable interrupt for READY event"]
            #[inline(always)]
            pub const fn ready(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for READY event"]
            #[inline(always)]
            pub fn set_ready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to Disable interrupt for DOWN event"]
            #[inline(always)]
            pub const fn down(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for DOWN event"]
            #[inline(always)]
            pub fn set_down(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to Disable interrupt for UP event"]
            #[inline(always)]
            pub const fn up(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for UP event"]
            #[inline(always)]
            pub fn set_up(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Write '1' to Disable interrupt for CROSS event"]
            #[inline(always)]
            pub const fn cross(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for CROSS event"]
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
        #[doc = "Input pin select"]
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
        #[doc = "Reference select"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Refsel(pub u32);
        impl Refsel {
            #[doc = "Reference select"]
            #[inline(always)]
            pub const fn refsel(&self) -> super::vals::Refsel {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Refsel::from_bits(val as u8)
            }
            #[doc = "Reference select"]
            #[inline(always)]
            pub fn set_refsel(&mut self, val: super::vals::Refsel) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
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
        #[doc = "Shortcut register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between READY event and SAMPLE task"]
            #[inline(always)]
            pub const fn ready_sample(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between READY event and SAMPLE task"]
            #[inline(always)]
            pub fn set_ready_sample(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Shortcut between READY event and STOP task"]
            #[inline(always)]
            pub const fn ready_stop(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between READY event and STOP task"]
            #[inline(always)]
            pub fn set_ready_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Shortcut between DOWN event and STOP task"]
            #[inline(always)]
            pub const fn down_stop(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between DOWN event and STOP task"]
            #[inline(always)]
            pub fn set_down_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Shortcut between UP event and STOP task"]
            #[inline(always)]
            pub const fn up_stop(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between UP event and STOP task"]
            #[inline(always)]
            pub fn set_up_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Shortcut between CROSS event and STOP task"]
            #[inline(always)]
            pub const fn cross_stop(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between CROSS event and STOP task"]
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
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Anadetect {
            #[doc = "Generate ANADETECT on crossing, both upward crossing and downward crossing"]
            CROSS = 0x0,
            #[doc = "Generate ANADETECT on upward crossing only"]
            UP = 0x01,
            #[doc = "Generate ANADETECT on downward crossing only"]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Enable {
            #[doc = "Disable"]
            DISABLED = 0x0,
            #[doc = "Enable"]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Extrefsel {
            #[doc = "Use AIN0 as external analog reference"]
            ANALOG_REFERENCE0 = 0x0,
            #[doc = "Use AIN1 as external analog reference"]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Hyst {
            #[doc = "Comparator hysteresis disabled"]
            NO_HYST = 0x0,
            #[doc = "Comparator hysteresis disabled (typ. 50 mV)"]
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
            #[doc = "AIN7 selected as analog input"]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Refsel {
            #[doc = "VDD * 1/8 selected as reference"]
            REF1_8VDD = 0x0,
            #[doc = "VDD * 2/8 selected as reference"]
            REF2_8VDD = 0x01,
            #[doc = "VDD * 3/8 selected as reference"]
            REF3_8VDD = 0x02,
            #[doc = "VDD * 4/8 selected as reference"]
            REF4_8VDD = 0x03,
            #[doc = "VDD * 5/8 selected as reference"]
            REF5_8VDD = 0x04,
            #[doc = "VDD * 6/8 selected as reference"]
            REF6_8VDD = 0x05,
            #[doc = "VDD * 7/8 selected as reference"]
            REF7_8VDD = 0x06,
            #[doc = "External analog reference selected"]
            AREF = 0x07,
            #[doc = "VDD * 1/16 selected as reference"]
            REF1_16VDD = 0x08,
            #[doc = "VDD * 3/16 selected as reference"]
            REF3_16VDD = 0x09,
            #[doc = "VDD * 5/16 selected as reference"]
            REF5_16VDD = 0x0a,
            #[doc = "VDD * 7/16 selected as reference"]
            REF7_16VDD = 0x0b,
            #[doc = "VDD * 9/16 selected as reference"]
            REF9_16VDD = 0x0c,
            #[doc = "VDD * 11/16 selected as reference"]
            REF11_16VDD = 0x0d,
            #[doc = "VDD * 13/16 selected as reference"]
            REF13_16VDD = 0x0e,
            #[doc = "VDD * 15/16 selected as reference"]
            REF15_16VDD = 0x0f,
        }
        impl Refsel {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Refsel {
                unsafe { core::mem::transmute(val & 0x0f) }
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
            #[doc = "Input voltage is below the reference threshold (VIN+ &lt; VIN-)."]
            BELOW = 0x0,
            #[doc = "Input voltage is above the reference threshold (VIN+ &gt; VIN-)."]
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
pub mod mwu {
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EventsPregion {
        ptr: *mut u8,
    }
    unsafe impl Send for EventsPregion {}
    unsafe impl Sync for EventsPregion {}
    impl EventsPregion {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster\\[0\\]: Write access to peripheral region 0 detected"]
        #[inline(always)]
        pub const fn wa(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster\\[0\\]: Read access to peripheral region 0 detected"]
        #[inline(always)]
        pub const fn ra(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EventsRegion {
        ptr: *mut u8,
    }
    unsafe impl Send for EventsRegion {}
    unsafe impl Sync for EventsRegion {}
    impl EventsRegion {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster\\[0\\]: Write access to region 0 detected"]
        #[inline(always)]
        pub const fn wa(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster\\[0\\]: Read access to region 0 detected"]
        #[inline(always)]
        pub const fn ra(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    #[doc = "Memory Watch Unit"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mwu {
        ptr: *mut u8,
    }
    unsafe impl Send for Mwu {}
    unsafe impl Sync for Mwu {}
    impl Mwu {
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
        pub const fn events_region(self, n: usize) -> EventsRegion {
            assert!(n < 4usize);
            unsafe { EventsRegion::from_ptr(self.ptr.add(0x0100usize + n * 8usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn events_pregion(self, n: usize) -> EventsPregion {
            assert!(n < 2usize);
            unsafe { EventsPregion::from_ptr(self.ptr.add(0x0160usize + n * 8usize) as _) }
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
        #[doc = "Enable or disable non-maskable interrupt"]
        #[inline(always)]
        pub const fn nmien(self) -> crate::common::Reg<regs::Nmien, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0320usize) as _) }
        }
        #[doc = "Enable non-maskable interrupt"]
        #[inline(always)]
        pub const fn nmienset(self) -> crate::common::Reg<regs::Nmien, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0324usize) as _) }
        }
        #[doc = "Disable non-maskable interrupt"]
        #[inline(always)]
        pub const fn nmienclr(self) -> crate::common::Reg<regs::Nmien, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0328usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn perregion(self, n: usize) -> Perregion {
            assert!(n < 2usize);
            unsafe { Perregion::from_ptr(self.ptr.add(0x0400usize + n * 8usize) as _) }
        }
        #[doc = "Enable/disable regions watch"]
        #[inline(always)]
        pub const fn regionen(self) -> crate::common::Reg<regs::Regionen, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "Enable regions watch"]
        #[inline(always)]
        pub const fn regionenset(self) -> crate::common::Reg<regs::Regionen, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
        }
        #[doc = "Disable regions watch"]
        #[inline(always)]
        pub const fn regionenclr(self) -> crate::common::Reg<regs::Regionen, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn region(self, n: usize) -> Region {
            assert!(n < 4usize);
            unsafe { Region::from_ptr(self.ptr.add(0x0600usize + n * 16usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn pregion(self, n: usize) -> Pregion {
            assert!(n < 2usize);
            unsafe { Pregion::from_ptr(self.ptr.add(0x06c0usize + n * 16usize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Perregion {
        ptr: *mut u8,
    }
    unsafe impl Send for Perregion {}
    unsafe impl Sync for Perregion {}
    impl Perregion {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster\\[0\\]: Source of event/interrupt in region 0, write access detected while corresponding subregion was enabled for watching"]
        #[inline(always)]
        pub const fn substatwa(self) -> crate::common::Reg<regs::Substatwa, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster\\[0\\]: Source of event/interrupt in region 0, read access detected while corresponding subregion was enabled for watching"]
        #[inline(always)]
        pub const fn substatra(self) -> crate::common::Reg<regs::Substatra, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pregion {
        ptr: *mut u8,
    }
    unsafe impl Send for Pregion {}
    unsafe impl Sync for Pregion {}
    impl Pregion {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster\\[0\\]: Reserved for future use"]
        #[inline(always)]
        pub const fn start(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster\\[0\\]: Reserved for future use"]
        #[inline(always)]
        pub const fn end(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Description cluster\\[0\\]: Subregions of region 0"]
        #[inline(always)]
        pub const fn subs(self) -> crate::common::Reg<regs::Subs, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Region {
        ptr: *mut u8,
    }
    unsafe impl Send for Region {}
    unsafe impl Sync for Region {}
    impl Region {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster\\[0\\]: Start address for region 0"]
        #[inline(always)]
        pub const fn start(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster\\[0\\]: End address of region 0"]
        #[inline(always)]
        pub const fn end(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Enable or disable interrupt for REGION\\[0\\].WA event"]
            #[inline(always)]
            pub const fn region0wa(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for REGION\\[0\\].WA event"]
            #[inline(always)]
            pub fn set_region0wa(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable interrupt for REGION\\[0\\].RA event"]
            #[inline(always)]
            pub const fn region0ra(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for REGION\\[0\\].RA event"]
            #[inline(always)]
            pub fn set_region0ra(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for REGION\\[1\\].WA event"]
            #[inline(always)]
            pub const fn region1wa(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for REGION\\[1\\].WA event"]
            #[inline(always)]
            pub fn set_region1wa(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable or disable interrupt for REGION\\[1\\].RA event"]
            #[inline(always)]
            pub const fn region1ra(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for REGION\\[1\\].RA event"]
            #[inline(always)]
            pub fn set_region1ra(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable or disable interrupt for REGION\\[2\\].WA event"]
            #[inline(always)]
            pub const fn region2wa(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for REGION\\[2\\].WA event"]
            #[inline(always)]
            pub fn set_region2wa(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable or disable interrupt for REGION\\[2\\].RA event"]
            #[inline(always)]
            pub const fn region2ra(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for REGION\\[2\\].RA event"]
            #[inline(always)]
            pub fn set_region2ra(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable or disable interrupt for REGION\\[3\\].WA event"]
            #[inline(always)]
            pub const fn region3wa(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for REGION\\[3\\].WA event"]
            #[inline(always)]
            pub fn set_region3wa(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Enable or disable interrupt for REGION\\[3\\].RA event"]
            #[inline(always)]
            pub const fn region3ra(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for REGION\\[3\\].RA event"]
            #[inline(always)]
            pub fn set_region3ra(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Enable or disable interrupt for PREGION\\[0\\].WA event"]
            #[inline(always)]
            pub const fn pregion0wa(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for PREGION\\[0\\].WA event"]
            #[inline(always)]
            pub fn set_pregion0wa(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
            #[doc = "Enable or disable interrupt for PREGION\\[0\\].RA event"]
            #[inline(always)]
            pub const fn pregion0ra(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for PREGION\\[0\\].RA event"]
            #[inline(always)]
            pub fn set_pregion0ra(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
            #[doc = "Enable or disable interrupt for PREGION\\[1\\].WA event"]
            #[inline(always)]
            pub const fn pregion1wa(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for PREGION\\[1\\].WA event"]
            #[inline(always)]
            pub fn set_pregion1wa(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
            #[doc = "Enable or disable interrupt for PREGION\\[1\\].RA event"]
            #[inline(always)]
            pub const fn pregion1ra(&self) -> bool {
                let val = (self.0 >> 27usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for PREGION\\[1\\].RA event"]
            #[inline(always)]
            pub fn set_pregion1ra(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
        #[doc = "Enable or disable non-maskable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Nmien(pub u32);
        impl Nmien {
            #[doc = "Enable or disable non-maskable interrupt for REGION\\[0\\].WA event"]
            #[inline(always)]
            pub const fn region0wa(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable non-maskable interrupt for REGION\\[0\\].WA event"]
            #[inline(always)]
            pub fn set_region0wa(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable non-maskable interrupt for REGION\\[0\\].RA event"]
            #[inline(always)]
            pub const fn region0ra(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable non-maskable interrupt for REGION\\[0\\].RA event"]
            #[inline(always)]
            pub fn set_region0ra(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable non-maskable interrupt for REGION\\[1\\].WA event"]
            #[inline(always)]
            pub const fn region1wa(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable non-maskable interrupt for REGION\\[1\\].WA event"]
            #[inline(always)]
            pub fn set_region1wa(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable or disable non-maskable interrupt for REGION\\[1\\].RA event"]
            #[inline(always)]
            pub const fn region1ra(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable non-maskable interrupt for REGION\\[1\\].RA event"]
            #[inline(always)]
            pub fn set_region1ra(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable or disable non-maskable interrupt for REGION\\[2\\].WA event"]
            #[inline(always)]
            pub const fn region2wa(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable non-maskable interrupt for REGION\\[2\\].WA event"]
            #[inline(always)]
            pub fn set_region2wa(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable or disable non-maskable interrupt for REGION\\[2\\].RA event"]
            #[inline(always)]
            pub const fn region2ra(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable non-maskable interrupt for REGION\\[2\\].RA event"]
            #[inline(always)]
            pub fn set_region2ra(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable or disable non-maskable interrupt for REGION\\[3\\].WA event"]
            #[inline(always)]
            pub const fn region3wa(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable non-maskable interrupt for REGION\\[3\\].WA event"]
            #[inline(always)]
            pub fn set_region3wa(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Enable or disable non-maskable interrupt for REGION\\[3\\].RA event"]
            #[inline(always)]
            pub const fn region3ra(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable non-maskable interrupt for REGION\\[3\\].RA event"]
            #[inline(always)]
            pub fn set_region3ra(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Enable or disable non-maskable interrupt for PREGION\\[0\\].WA event"]
            #[inline(always)]
            pub const fn pregion0wa(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable non-maskable interrupt for PREGION\\[0\\].WA event"]
            #[inline(always)]
            pub fn set_pregion0wa(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
            #[doc = "Enable or disable non-maskable interrupt for PREGION\\[0\\].RA event"]
            #[inline(always)]
            pub const fn pregion0ra(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable non-maskable interrupt for PREGION\\[0\\].RA event"]
            #[inline(always)]
            pub fn set_pregion0ra(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
            #[doc = "Enable or disable non-maskable interrupt for PREGION\\[1\\].WA event"]
            #[inline(always)]
            pub const fn pregion1wa(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable non-maskable interrupt for PREGION\\[1\\].WA event"]
            #[inline(always)]
            pub fn set_pregion1wa(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
            #[doc = "Enable or disable non-maskable interrupt for PREGION\\[1\\].RA event"]
            #[inline(always)]
            pub const fn pregion1ra(&self) -> bool {
                let val = (self.0 >> 27usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable non-maskable interrupt for PREGION\\[1\\].RA event"]
            #[inline(always)]
            pub fn set_pregion1ra(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
            }
        }
        impl Default for Nmien {
            #[inline(always)]
            fn default() -> Nmien {
                Nmien(0)
            }
        }
        #[doc = "Enable/disable regions watch"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Regionen(pub u32);
        impl Regionen {
            #[doc = "Enable/disable write access watch in region\\[0\\]"]
            #[inline(always)]
            pub const fn rgn_wa(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 0usize + n * 2usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Enable/disable write access watch in region\\[0\\]"]
            #[inline(always)]
            pub fn set_rgn_wa(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 0usize + n * 2usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Enable/disable read access watch in region\\[0\\]"]
            #[inline(always)]
            pub const fn rgn_ra(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 1usize + n * 2usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Enable/disable read access watch in region\\[0\\]"]
            #[inline(always)]
            pub fn set_rgn_ra(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 1usize + n * 2usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Enable/disable write access watch in PREGION\\[0\\]"]
            #[inline(always)]
            pub const fn prgn_wa(&self, n: usize) -> bool {
                assert!(n < 2usize);
                let offs = 24usize + n * 2usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Enable/disable write access watch in PREGION\\[0\\]"]
            #[inline(always)]
            pub fn set_prgn_wa(&mut self, n: usize, val: bool) {
                assert!(n < 2usize);
                let offs = 24usize + n * 2usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Enable/disable read access watch in PREGION\\[0\\]"]
            #[inline(always)]
            pub const fn prgn_ra(&self, n: usize) -> bool {
                assert!(n < 2usize);
                let offs = 25usize + n * 2usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Enable/disable read access watch in PREGION\\[0\\]"]
            #[inline(always)]
            pub fn set_prgn_ra(&mut self, n: usize, val: bool) {
                assert!(n < 2usize);
                let offs = 25usize + n * 2usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Regionen {
            #[inline(always)]
            fn default() -> Regionen {
                Regionen(0)
            }
        }
        #[doc = "Description cluster\\[0\\]: Subregions of region 0"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Subs(pub u32);
        impl Subs {
            #[doc = "Include or exclude subregion 0 in region"]
            #[inline(always)]
            pub const fn sr(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Include or exclude subregion 0 in region"]
            #[inline(always)]
            pub fn set_sr(&mut self, n: usize, val: bool) {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Subs {
            #[inline(always)]
            fn default() -> Subs {
                Subs(0)
            }
        }
        #[doc = "Description cluster\\[0\\]: Source of event/interrupt in region 0, read access detected while corresponding subregion was enabled for watching"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Substatra(pub u32);
        impl Substatra {
            #[doc = "Subregion 0 in region 0 (write '1' to clear)"]
            #[inline(always)]
            pub const fn sr(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Subregion 0 in region 0 (write '1' to clear)"]
            #[inline(always)]
            pub fn set_sr(&mut self, n: usize, val: bool) {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Substatra {
            #[inline(always)]
            fn default() -> Substatra {
                Substatra(0)
            }
        }
        #[doc = "Description cluster\\[0\\]: Source of event/interrupt in region 0, write access detected while corresponding subregion was enabled for watching"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Substatwa(pub u32);
        impl Substatwa {
            #[doc = "Subregion 0 in region 0 (write '1' to clear)"]
            #[inline(always)]
            pub const fn sr(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Subregion 0 in region 0 (write '1' to clear)"]
            #[inline(always)]
            pub fn set_sr(&mut self, n: usize, val: bool) {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Substatwa {
            #[inline(always)]
            fn default() -> Substatwa {
                Substatwa(0)
            }
        }
    }
}
pub mod nfct {
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Framestatus {
        ptr: *mut u8,
    }
    unsafe impl Send for Framestatus {}
    unsafe impl Sync for Framestatus {}
    impl Framestatus {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Result of last incoming frames"]
        #[inline(always)]
        pub const fn rx(self) -> crate::common::Reg<regs::Rx, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
    }
    #[doc = "NFC-A compatible radio"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nfct {
        ptr: *mut u8,
    }
    unsafe impl Send for Nfct {}
    unsafe impl Sync for Nfct {}
    impl Nfct {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Activate NFC peripheral for incoming and outgoing frames, change state to activated"]
        #[inline(always)]
        pub const fn tasks_activate(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Disable NFC peripheral"]
        #[inline(always)]
        pub const fn tasks_disable(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Enable NFC sense field mode, change state to sense mode"]
        #[inline(always)]
        pub const fn tasks_sense(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Start transmission of a outgoing frame, change state to transmit"]
        #[inline(always)]
        pub const fn tasks_starttx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Initializes the EasyDMA for receive."]
        #[inline(always)]
        pub const fn tasks_enablerxdata(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "Force state machine to IDLE state"]
        #[inline(always)]
        pub const fn tasks_goidle(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
        }
        #[doc = "Force state machine to SLEEP_A state"]
        #[inline(always)]
        pub const fn tasks_gosleep(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
        }
        #[doc = "The NFC peripheral is ready to receive and send frames"]
        #[inline(always)]
        pub const fn events_ready(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Remote NFC field detected"]
        #[inline(always)]
        pub const fn events_fielddetected(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Remote NFC field lost"]
        #[inline(always)]
        pub const fn events_fieldlost(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "Marks the start of the first symbol of a transmitted frame"]
        #[inline(always)]
        pub const fn events_txframestart(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
        }
        #[doc = "Marks the end of the last transmitted on-air symbol of a frame"]
        #[inline(always)]
        pub const fn events_txframeend(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
        }
        #[doc = "Marks the end of the first symbol of a received frame"]
        #[inline(always)]
        pub const fn events_rxframestart(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
        }
        #[doc = "Received data have been checked (CRC, parity) and transferred to RAM, and EasyDMA has ended accessing the RX buffer"]
        #[inline(always)]
        pub const fn events_rxframeend(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
        }
        #[doc = "NFC error reported. The ERRORSTATUS register contains details on the source of the error."]
        #[inline(always)]
        pub const fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
        }
        #[doc = "NFC RX frame error reported. The FRAMESTATUS.RX register contains details on the source of the error."]
        #[inline(always)]
        pub const fn events_rxerror(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
        }
        #[doc = "RX buffer (as defined by PACKETPTR and MAXLEN) in Data RAM full."]
        #[inline(always)]
        pub const fn events_endrx(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
        }
        #[doc = "Transmission of data in RAM has ended, and EasyDMA has ended accessing the TX buffer"]
        #[inline(always)]
        pub const fn events_endtx(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
        }
        #[doc = "Auto collision resolution process has started"]
        #[inline(always)]
        pub const fn events_autocolresstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
        }
        #[doc = "NFC Auto collision resolution error reported."]
        #[inline(always)]
        pub const fn events_collision(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
        }
        #[doc = "NFC Auto collision resolution successfully completed"]
        #[inline(always)]
        pub const fn events_selected(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
        }
        #[doc = "EasyDMA is ready to receive or send frames."]
        #[inline(always)]
        pub const fn events_started(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
        }
        #[doc = "Shortcut register"]
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
        #[doc = "NFC Error Status register"]
        #[inline(always)]
        pub const fn errorstatus(self) -> crate::common::Reg<regs::Errorstatus, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0404usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn framestatus(self) -> Framestatus {
            unsafe { Framestatus::from_ptr(self.ptr.add(0x040cusize) as _) }
        }
        #[doc = "Current value driven to the NFC Load Control"]
        #[inline(always)]
        pub const fn currentloadctrl(
            self,
        ) -> crate::common::Reg<regs::Currentloadctrl, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0430usize) as _) }
        }
        #[doc = "Indicates the presence or not of a valid field"]
        #[inline(always)]
        pub const fn fieldpresent(
            self,
        ) -> crate::common::Reg<regs::Fieldpresent, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x043cusize) as _) }
        }
        #[doc = "Minimum frame delay"]
        #[inline(always)]
        pub const fn framedelaymin(
            self,
        ) -> crate::common::Reg<regs::Framedelaymin, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Maximum frame delay"]
        #[inline(always)]
        pub const fn framedelaymax(
            self,
        ) -> crate::common::Reg<regs::Framedelaymax, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Configuration register for the Frame Delay Timer"]
        #[inline(always)]
        pub const fn framedelaymode(
            self,
        ) -> crate::common::Reg<regs::Framedelaymode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Packet pointer for TXD and RXD data storage in Data RAM"]
        #[inline(always)]
        pub const fn packetptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "Size of allocated for TXD and RXD data storage buffer in Data RAM"]
        #[inline(always)]
        pub const fn maxlen(self) -> crate::common::Reg<regs::Maxlen, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn txd(self) -> Txd {
            unsafe { Txd::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn rxd(self) -> Rxd {
            unsafe { Rxd::from_ptr(self.ptr.add(0x0520usize) as _) }
        }
        #[doc = "Last NFCID1 part (4, 7 or 10 bytes ID)"]
        #[inline(always)]
        pub const fn nfcid1_last(self) -> crate::common::Reg<regs::Nfcid1Last, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0590usize) as _) }
        }
        #[doc = "Second last NFCID1 part (7 or 10 bytes ID)"]
        #[inline(always)]
        pub const fn nfcid1_2nd_last(
            self,
        ) -> crate::common::Reg<regs::Nfcid12ndLast, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0594usize) as _) }
        }
        #[doc = "Third last NFCID1 part (10 bytes ID)"]
        #[inline(always)]
        pub const fn nfcid1_3rd_last(
            self,
        ) -> crate::common::Reg<regs::Nfcid13rdLast, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0598usize) as _) }
        }
        #[doc = "NFC-A SENS_RES auto-response settings"]
        #[inline(always)]
        pub const fn sensres(self) -> crate::common::Reg<regs::Sensres, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05a0usize) as _) }
        }
        #[doc = "NFC-A SEL_RES auto-response settings"]
        #[inline(always)]
        pub const fn selres(self) -> crate::common::Reg<regs::Selres, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05a4usize) as _) }
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
        #[doc = "Configuration of incoming frames"]
        #[inline(always)]
        pub const fn frameconfig(
            self,
        ) -> crate::common::Reg<regs::RxdFrameconfig, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Size of last incoming frame"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::RxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
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
        #[doc = "Configuration of outgoing frames"]
        #[inline(always)]
        pub const fn frameconfig(
            self,
        ) -> crate::common::Reg<regs::TxdFrameconfig, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Size of outgoing frame"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::TxdAmount, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Current value driven to the NFC Load Control"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Currentloadctrl(pub u32);
        impl Currentloadctrl {
            #[doc = "Current value driven to the NFC Load Control"]
            #[inline(always)]
            pub const fn currentloadctrl(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x3f;
                val as u8
            }
            #[doc = "Current value driven to the NFC Load Control"]
            #[inline(always)]
            pub fn set_currentloadctrl(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
            }
        }
        impl Default for Currentloadctrl {
            #[inline(always)]
            fn default() -> Currentloadctrl {
                Currentloadctrl(0)
            }
        }
        #[doc = "NFC Error Status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Errorstatus(pub u32);
        impl Errorstatus {
            #[doc = "No STARTTX task triggered before expiration of the time set in FRAMEDELAYMAX"]
            #[inline(always)]
            pub const fn framedelaytimeout(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "No STARTTX task triggered before expiration of the time set in FRAMEDELAYMAX"]
            #[inline(always)]
            pub fn set_framedelaytimeout(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Field level is too high at max load resistance"]
            #[inline(always)]
            pub const fn nfcfieldtoostrong(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Field level is too high at max load resistance"]
            #[inline(always)]
            pub fn set_nfcfieldtoostrong(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Field level is too low at min load resistance"]
            #[inline(always)]
            pub const fn nfcfieldtooweak(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Field level is too low at min load resistance"]
            #[inline(always)]
            pub fn set_nfcfieldtooweak(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Errorstatus {
            #[inline(always)]
            fn default() -> Errorstatus {
                Errorstatus(0)
            }
        }
        #[doc = "Indicates the presence or not of a valid field"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Fieldpresent(pub u32);
        impl Fieldpresent {
            #[doc = "Indicates the presence or not of a valid field. Available only in the activated state."]
            #[inline(always)]
            pub const fn fieldpresent(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Indicates the presence or not of a valid field. Available only in the activated state."]
            #[inline(always)]
            pub fn set_fieldpresent(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Indicates if the low level has locked to the field"]
            #[inline(always)]
            pub const fn lockdetect(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Indicates if the low level has locked to the field"]
            #[inline(always)]
            pub fn set_lockdetect(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
        }
        impl Default for Fieldpresent {
            #[inline(always)]
            fn default() -> Fieldpresent {
                Fieldpresent(0)
            }
        }
        #[doc = "Maximum frame delay"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Framedelaymax(pub u32);
        impl Framedelaymax {
            #[doc = "Maximum frame delay in number of 13.56 MHz clocks"]
            #[inline(always)]
            pub const fn framedelaymax(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Maximum frame delay in number of 13.56 MHz clocks"]
            #[inline(always)]
            pub fn set_framedelaymax(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Framedelaymax {
            #[inline(always)]
            fn default() -> Framedelaymax {
                Framedelaymax(0)
            }
        }
        #[doc = "Minimum frame delay"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Framedelaymin(pub u32);
        impl Framedelaymin {
            #[doc = "Minimum frame delay in number of 13.56 MHz clocks"]
            #[inline(always)]
            pub const fn framedelaymin(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Minimum frame delay in number of 13.56 MHz clocks"]
            #[inline(always)]
            pub fn set_framedelaymin(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Framedelaymin {
            #[inline(always)]
            fn default() -> Framedelaymin {
                Framedelaymin(0)
            }
        }
        #[doc = "Configuration register for the Frame Delay Timer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Framedelaymode(pub u32);
        impl Framedelaymode {
            #[doc = "Configuration register for the Frame Delay Timer"]
            #[inline(always)]
            pub const fn framedelaymode(&self) -> super::vals::Framedelaymode {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Framedelaymode::from_bits(val as u8)
            }
            #[doc = "Configuration register for the Frame Delay Timer"]
            #[inline(always)]
            pub fn set_framedelaymode(&mut self, val: super::vals::Framedelaymode) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Framedelaymode {
            #[inline(always)]
            fn default() -> Framedelaymode {
                Framedelaymode(0)
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Enable or disable interrupt for READY event"]
            #[inline(always)]
            pub const fn ready(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for READY event"]
            #[inline(always)]
            pub fn set_ready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable interrupt for FIELDDETECTED event"]
            #[inline(always)]
            pub const fn fielddetected(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for FIELDDETECTED event"]
            #[inline(always)]
            pub fn set_fielddetected(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for FIELDLOST event"]
            #[inline(always)]
            pub const fn fieldlost(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for FIELDLOST event"]
            #[inline(always)]
            pub fn set_fieldlost(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable or disable interrupt for TXFRAMESTART event"]
            #[inline(always)]
            pub const fn txframestart(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for TXFRAMESTART event"]
            #[inline(always)]
            pub fn set_txframestart(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable or disable interrupt for TXFRAMEEND event"]
            #[inline(always)]
            pub const fn txframeend(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for TXFRAMEEND event"]
            #[inline(always)]
            pub fn set_txframeend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable or disable interrupt for RXFRAMESTART event"]
            #[inline(always)]
            pub const fn rxframestart(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for RXFRAMESTART event"]
            #[inline(always)]
            pub fn set_rxframestart(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable or disable interrupt for RXFRAMEEND event"]
            #[inline(always)]
            pub const fn rxframeend(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for RXFRAMEEND event"]
            #[inline(always)]
            pub fn set_rxframeend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Enable or disable interrupt for ERROR event"]
            #[inline(always)]
            pub const fn error(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for ERROR event"]
            #[inline(always)]
            pub fn set_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Enable or disable interrupt for RXERROR event"]
            #[inline(always)]
            pub const fn rxerror(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for RXERROR event"]
            #[inline(always)]
            pub fn set_rxerror(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Enable or disable interrupt for ENDRX event"]
            #[inline(always)]
            pub const fn endrx(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for ENDRX event"]
            #[inline(always)]
            pub fn set_endrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "Enable or disable interrupt for ENDTX event"]
            #[inline(always)]
            pub const fn endtx(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for ENDTX event"]
            #[inline(always)]
            pub fn set_endtx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "Enable or disable interrupt for AUTOCOLRESSTARTED event"]
            #[inline(always)]
            pub const fn autocolresstarted(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for AUTOCOLRESSTARTED event"]
            #[inline(always)]
            pub fn set_autocolresstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Enable or disable interrupt for COLLISION event"]
            #[inline(always)]
            pub const fn collision(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for COLLISION event"]
            #[inline(always)]
            pub fn set_collision(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Enable or disable interrupt for SELECTED event"]
            #[inline(always)]
            pub const fn selected(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for SELECTED event"]
            #[inline(always)]
            pub fn set_selected(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "Enable or disable interrupt for STARTED event"]
            #[inline(always)]
            pub const fn started(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for STARTED event"]
            #[inline(always)]
            pub fn set_started(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
        }
        impl Default for Inten {
            #[inline(always)]
            fn default() -> Inten {
                Inten(0)
            }
        }
        #[doc = "Size of allocated for TXD and RXD data storage buffer in Data RAM"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Maxlen(pub u32);
        impl Maxlen {
            #[doc = "Size of allocated for TXD and RXD data storage buffer in Data RAM"]
            #[inline(always)]
            pub const fn maxlen(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x01ff;
                val as u16
            }
            #[doc = "Size of allocated for TXD and RXD data storage buffer in Data RAM"]
            #[inline(always)]
            pub fn set_maxlen(&mut self, val: u16) {
                self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
            }
        }
        impl Default for Maxlen {
            #[inline(always)]
            fn default() -> Maxlen {
                Maxlen(0)
            }
        }
        #[doc = "Second last NFCID1 part (7 or 10 bytes ID)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Nfcid12ndLast(pub u32);
        impl Nfcid12ndLast {
            #[doc = "NFCID1 byte V"]
            #[inline(always)]
            pub const fn nfcid1_v(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "NFCID1 byte V"]
            #[inline(always)]
            pub fn set_nfcid1_v(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "NFCID1 byte U"]
            #[inline(always)]
            pub const fn nfcid1_u(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "NFCID1 byte U"]
            #[inline(always)]
            pub fn set_nfcid1_u(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[doc = "NFCID1 byte T"]
            #[inline(always)]
            pub const fn nfcid1_t(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0xff;
                val as u8
            }
            #[doc = "NFCID1 byte T"]
            #[inline(always)]
            pub fn set_nfcid1_t(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
            }
        }
        impl Default for Nfcid12ndLast {
            #[inline(always)]
            fn default() -> Nfcid12ndLast {
                Nfcid12ndLast(0)
            }
        }
        #[doc = "Third last NFCID1 part (10 bytes ID)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Nfcid13rdLast(pub u32);
        impl Nfcid13rdLast {
            #[doc = "NFCID1 byte S"]
            #[inline(always)]
            pub const fn nfcid1_s(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "NFCID1 byte S"]
            #[inline(always)]
            pub fn set_nfcid1_s(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "NFCID1 byte R"]
            #[inline(always)]
            pub const fn nfcid1_r(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "NFCID1 byte R"]
            #[inline(always)]
            pub fn set_nfcid1_r(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[doc = "NFCID1 byte Q"]
            #[inline(always)]
            pub const fn nfcid1_q(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0xff;
                val as u8
            }
            #[doc = "NFCID1 byte Q"]
            #[inline(always)]
            pub fn set_nfcid1_q(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
            }
        }
        impl Default for Nfcid13rdLast {
            #[inline(always)]
            fn default() -> Nfcid13rdLast {
                Nfcid13rdLast(0)
            }
        }
        #[doc = "Last NFCID1 part (4, 7 or 10 bytes ID)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Nfcid1Last(pub u32);
        impl Nfcid1Last {
            #[doc = "NFCID1 byte Z (very last byte sent)"]
            #[inline(always)]
            pub const fn nfcid1_z(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "NFCID1 byte Z (very last byte sent)"]
            #[inline(always)]
            pub fn set_nfcid1_z(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "NFCID1 byte Y"]
            #[inline(always)]
            pub const fn nfcid1_y(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "NFCID1 byte Y"]
            #[inline(always)]
            pub fn set_nfcid1_y(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[doc = "NFCID1 byte X"]
            #[inline(always)]
            pub const fn nfcid1_x(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0xff;
                val as u8
            }
            #[doc = "NFCID1 byte X"]
            #[inline(always)]
            pub fn set_nfcid1_x(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
            }
            #[doc = "NFCID1 byte W"]
            #[inline(always)]
            pub const fn nfcid1_w(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0xff;
                val as u8
            }
            #[doc = "NFCID1 byte W"]
            #[inline(always)]
            pub fn set_nfcid1_w(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
            }
        }
        impl Default for Nfcid1Last {
            #[inline(always)]
            fn default() -> Nfcid1Last {
                Nfcid1Last(0)
            }
        }
        #[doc = "Result of last incoming frames"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rx(pub u32);
        impl Rx {
            #[doc = "No valid End of Frame detected"]
            #[inline(always)]
            pub const fn crcerror(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "No valid End of Frame detected"]
            #[inline(always)]
            pub fn set_crcerror(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Parity status of received frame"]
            #[inline(always)]
            pub const fn paritystatus(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Parity status of received frame"]
            #[inline(always)]
            pub fn set_paritystatus(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Overrun detected"]
            #[inline(always)]
            pub const fn overrun(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun detected"]
            #[inline(always)]
            pub fn set_overrun(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Rx {
            #[inline(always)]
            fn default() -> Rx {
                Rx(0)
            }
        }
        #[doc = "Size of last incoming frame"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdAmount(pub u32);
        impl RxdAmount {
            #[doc = "Number of bits in the last byte in the frame, if less than 8 (including CRC, but excluding parity and SoF/EoF framing)."]
            #[inline(always)]
            pub const fn rxdatabits(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x07;
                val as u8
            }
            #[doc = "Number of bits in the last byte in the frame, if less than 8 (including CRC, but excluding parity and SoF/EoF framing)."]
            #[inline(always)]
            pub fn set_rxdatabits(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
            }
            #[doc = "Number of complete bytes received in the frame (including CRC, but excluding parity and SoF/EoF framing)"]
            #[inline(always)]
            pub const fn rxdatabytes(&self) -> u16 {
                let val = (self.0 >> 3usize) & 0x01ff;
                val as u16
            }
            #[doc = "Number of complete bytes received in the frame (including CRC, but excluding parity and SoF/EoF framing)"]
            #[inline(always)]
            pub fn set_rxdatabytes(&mut self, val: u16) {
                self.0 = (self.0 & !(0x01ff << 3usize)) | (((val as u32) & 0x01ff) << 3usize);
            }
        }
        impl Default for RxdAmount {
            #[inline(always)]
            fn default() -> RxdAmount {
                RxdAmount(0)
            }
        }
        #[doc = "Configuration of incoming frames"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdFrameconfig(pub u32);
        impl RxdFrameconfig {
            #[doc = "Parity expected or not in RX frame"]
            #[inline(always)]
            pub const fn parity(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Parity expected or not in RX frame"]
            #[inline(always)]
            pub fn set_parity(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "SoF expected or not in RX frames"]
            #[inline(always)]
            pub const fn sof(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "SoF expected or not in RX frames"]
            #[inline(always)]
            pub fn set_sof(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "CRC mode for incoming frames"]
            #[inline(always)]
            pub const fn crcmoderx(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "CRC mode for incoming frames"]
            #[inline(always)]
            pub fn set_crcmoderx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
        }
        impl Default for RxdFrameconfig {
            #[inline(always)]
            fn default() -> RxdFrameconfig {
                RxdFrameconfig(0)
            }
        }
        #[doc = "NFC-A SEL_RES auto-response settings"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Selres(pub u32);
        impl Selres {
            #[doc = "Reserved for future use. Shall be 0."]
            #[inline(always)]
            pub const fn rfu10(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x03;
                val as u8
            }
            #[doc = "Reserved for future use. Shall be 0."]
            #[inline(always)]
            pub fn set_rfu10(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
            }
            #[doc = "Cascade bit (controlled by hardware, write has no effect)"]
            #[inline(always)]
            pub const fn cascade(&self) -> super::vals::Cascade {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Cascade::from_bits(val as u8)
            }
            #[doc = "Cascade bit (controlled by hardware, write has no effect)"]
            #[inline(always)]
            pub fn set_cascade(&mut self, val: super::vals::Cascade) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
            #[doc = "Reserved for future use. Shall be 0."]
            #[inline(always)]
            pub const fn rfu43(&self) -> u8 {
                let val = (self.0 >> 3usize) & 0x03;
                val as u8
            }
            #[doc = "Reserved for future use. Shall be 0."]
            #[inline(always)]
            pub fn set_rfu43(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
            }
            #[doc = "Protocol as defined by the b7:b6 of SEL_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
            #[inline(always)]
            pub const fn protocol(&self) -> u8 {
                let val = (self.0 >> 5usize) & 0x03;
                val as u8
            }
            #[doc = "Protocol as defined by the b7:b6 of SEL_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
            #[inline(always)]
            pub fn set_protocol(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
            }
            #[doc = "Reserved for future use. Shall be 0."]
            #[inline(always)]
            pub const fn rfu7(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Reserved for future use. Shall be 0."]
            #[inline(always)]
            pub fn set_rfu7(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
        }
        impl Default for Selres {
            #[inline(always)]
            fn default() -> Selres {
                Selres(0)
            }
        }
        #[doc = "NFC-A SENS_RES auto-response settings"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sensres(pub u32);
        impl Sensres {
            #[doc = "Bit frame SDD as defined by the b5:b1 of byte 1 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
            #[inline(always)]
            pub const fn bitframesdd(&self) -> super::vals::Bitframesdd {
                let val = (self.0 >> 0usize) & 0x1f;
                super::vals::Bitframesdd::from_bits(val as u8)
            }
            #[doc = "Bit frame SDD as defined by the b5:b1 of byte 1 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
            #[inline(always)]
            pub fn set_bitframesdd(&mut self, val: super::vals::Bitframesdd) {
                self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
            }
            #[doc = "Reserved for future use. Shall be 0."]
            #[inline(always)]
            pub const fn rfu5(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Reserved for future use. Shall be 0."]
            #[inline(always)]
            pub fn set_rfu5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "NFCID1 size. This value is used by the Auto collision resolution engine."]
            #[inline(always)]
            pub const fn nfcidsize(&self) -> super::vals::Nfcidsize {
                let val = (self.0 >> 6usize) & 0x03;
                super::vals::Nfcidsize::from_bits(val as u8)
            }
            #[doc = "NFCID1 size. This value is used by the Auto collision resolution engine."]
            #[inline(always)]
            pub fn set_nfcidsize(&mut self, val: super::vals::Nfcidsize) {
                self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
            }
            #[doc = "Tag platform configuration as defined by the b4:b1 of byte 2 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
            #[inline(always)]
            pub const fn platfconfig(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0x0f;
                val as u8
            }
            #[doc = "Tag platform configuration as defined by the b4:b1 of byte 2 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
            #[inline(always)]
            pub fn set_platfconfig(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
            }
            #[doc = "Reserved for future use. Shall be 0."]
            #[inline(always)]
            pub const fn rfu74(&self) -> u8 {
                let val = (self.0 >> 12usize) & 0x0f;
                val as u8
            }
            #[doc = "Reserved for future use. Shall be 0."]
            #[inline(always)]
            pub fn set_rfu74(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
            }
        }
        impl Default for Sensres {
            #[inline(always)]
            fn default() -> Sensres {
                Sensres(0)
            }
        }
        #[doc = "Shortcut register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between FIELDDETECTED event and ACTIVATE task"]
            #[inline(always)]
            pub const fn fielddetected_activate(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between FIELDDETECTED event and ACTIVATE task"]
            #[inline(always)]
            pub fn set_fielddetected_activate(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Shortcut between FIELDLOST event and SENSE task"]
            #[inline(always)]
            pub const fn fieldlost_sense(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between FIELDLOST event and SENSE task"]
            #[inline(always)]
            pub fn set_fieldlost_sense(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        #[doc = "Size of outgoing frame"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdAmount(pub u32);
        impl TxdAmount {
            #[doc = "Number of bits in the last or first byte read from RAM that shall be included in the frame (excluding parity bit)."]
            #[inline(always)]
            pub const fn txdatabits(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x07;
                val as u8
            }
            #[doc = "Number of bits in the last or first byte read from RAM that shall be included in the frame (excluding parity bit)."]
            #[inline(always)]
            pub fn set_txdatabits(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
            }
            #[doc = "Number of complete bytes that shall be included in the frame, excluding CRC, parity and framing"]
            #[inline(always)]
            pub const fn txdatabytes(&self) -> u16 {
                let val = (self.0 >> 3usize) & 0x01ff;
                val as u16
            }
            #[doc = "Number of complete bytes that shall be included in the frame, excluding CRC, parity and framing"]
            #[inline(always)]
            pub fn set_txdatabytes(&mut self, val: u16) {
                self.0 = (self.0 & !(0x01ff << 3usize)) | (((val as u32) & 0x01ff) << 3usize);
            }
        }
        impl Default for TxdAmount {
            #[inline(always)]
            fn default() -> TxdAmount {
                TxdAmount(0)
            }
        }
        #[doc = "Configuration of outgoing frames"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdFrameconfig(pub u32);
        impl TxdFrameconfig {
            #[doc = "Adding parity or not in the frame"]
            #[inline(always)]
            pub const fn parity(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Adding parity or not in the frame"]
            #[inline(always)]
            pub fn set_parity(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Discarding unused bits in start or at end of a Frame"]
            #[inline(always)]
            pub const fn discardmode(&self) -> super::vals::Discardmode {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Discardmode::from_bits(val as u8)
            }
            #[doc = "Discarding unused bits in start or at end of a Frame"]
            #[inline(always)]
            pub fn set_discardmode(&mut self, val: super::vals::Discardmode) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Adding SoF or not in TX frames"]
            #[inline(always)]
            pub const fn sof(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Adding SoF or not in TX frames"]
            #[inline(always)]
            pub fn set_sof(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "CRC mode for outgoing frames"]
            #[inline(always)]
            pub const fn crcmodetx(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "CRC mode for outgoing frames"]
            #[inline(always)]
            pub fn set_crcmodetx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
        }
        impl Default for TxdFrameconfig {
            #[inline(always)]
            fn default() -> TxdFrameconfig {
                TxdFrameconfig(0)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Bitframesdd {
            #[doc = "SDD pattern 00000"]
            SDD00000 = 0x0,
            #[doc = "SDD pattern 00001"]
            SDD00001 = 0x01,
            #[doc = "SDD pattern 00010"]
            SDD00010 = 0x02,
            _RESERVED_3 = 0x03,
            #[doc = "SDD pattern 00100"]
            SDD00100 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
            #[doc = "SDD pattern 01000"]
            SDD01000 = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
            #[doc = "SDD pattern 10000"]
            SDD10000 = 0x10,
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
        impl Bitframesdd {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Bitframesdd {
                unsafe { core::mem::transmute(val & 0x1f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Bitframesdd {
            #[inline(always)]
            fn from(val: u8) -> Bitframesdd {
                Bitframesdd::from_bits(val)
            }
        }
        impl From<Bitframesdd> for u8 {
            #[inline(always)]
            fn from(val: Bitframesdd) -> u8 {
                Bitframesdd::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Cascade {
            #[doc = "NFCID1 complete"]
            COMPLETE = 0x0,
            #[doc = "NFCID1 not complete"]
            NOT_COMPLETE = 0x01,
        }
        impl Cascade {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Cascade {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Cascade {
            #[inline(always)]
            fn from(val: u8) -> Cascade {
                Cascade::from_bits(val)
            }
        }
        impl From<Cascade> for u8 {
            #[inline(always)]
            fn from(val: Cascade) -> u8 {
                Cascade::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Discardmode {
            #[doc = "Unused bits is discarded at end of frame"]
            DISCARD_END = 0x0,
            #[doc = "Unused bits is discarded at start of frame"]
            DISCARD_START = 0x01,
        }
        impl Discardmode {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Discardmode {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Discardmode {
            #[inline(always)]
            fn from(val: u8) -> Discardmode {
                Discardmode::from_bits(val)
            }
        }
        impl From<Discardmode> for u8 {
            #[inline(always)]
            fn from(val: Discardmode) -> u8 {
                Discardmode::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Framedelaymode {
            #[doc = "Transmission is independent of frame timer and will start when the STARTTX task is triggered. No timeout."]
            FREE_RUN = 0x0,
            #[doc = "Frame is transmitted between FRAMEDELAYMIN and FRAMEDELAYMAX"]
            WINDOW = 0x01,
            #[doc = "Frame is transmitted exactly at FRAMEDELAYMAX"]
            EXACT_VAL = 0x02,
            #[doc = "Frame is transmitted on a bit grid between FRAMEDELAYMIN and FRAMEDELAYMAX"]
            WINDOW_GRID = 0x03,
        }
        impl Framedelaymode {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Framedelaymode {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Framedelaymode {
            #[inline(always)]
            fn from(val: u8) -> Framedelaymode {
                Framedelaymode::from_bits(val)
            }
        }
        impl From<Framedelaymode> for u8 {
            #[inline(always)]
            fn from(val: Framedelaymode) -> u8 {
                Framedelaymode::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Nfcidsize {
            #[doc = "NFCID1 size: single (4 bytes)"]
            NFCID1SINGLE = 0x0,
            #[doc = "NFCID1 size: double (7 bytes)"]
            NFCID1DOUBLE = 0x01,
            #[doc = "NFCID1 size: triple (10 bytes)"]
            NFCID1TRIPLE = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Nfcidsize {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Nfcidsize {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Nfcidsize {
            #[inline(always)]
            fn from(val: u8) -> Nfcidsize {
                Nfcidsize::from_bits(val)
            }
        }
        impl From<Nfcidsize> for u8 {
            #[inline(always)]
            fn from(val: Nfcidsize) -> u8 {
                Nfcidsize::to_bits(val)
            }
        }
    }
}
pub mod nvmc {
    #[doc = "Non Volatile Memory Controller"]
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
        #[doc = "Register for erasing a page in Code area"]
        #[inline(always)]
        pub const fn erasepage(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Deprecated register - Register for erasing a page in Code area. Equivalent to ERASEPAGE."]
        #[inline(always)]
        pub const fn erasepcr1(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Register for erasing all non-volatile user memory"]
        #[inline(always)]
        pub const fn eraseall(self) -> crate::common::Reg<regs::Eraseall, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Deprecated register - Register for erasing a page in Code area. Equivalent to ERASEPAGE."]
        #[inline(always)]
        pub const fn erasepcr0(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "Register for erasing User Information Configuration Registers"]
        #[inline(always)]
        pub const fn eraseuicr(self) -> crate::common::Reg<regs::Eraseuicr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
        }
        #[doc = "I-Code cache configuration register."]
        #[inline(always)]
        pub const fn icachecnf(self) -> crate::common::Reg<regs::Icachecnf, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize) as _) }
        }
        #[doc = "I-Code cache hit counter."]
        #[inline(always)]
        pub const fn ihit(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0548usize) as _) }
        }
        #[doc = "I-Code cache miss counter."]
        #[inline(always)]
        pub const fn imiss(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x054cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated."]
            #[inline(always)]
            pub const fn wen(&self) -> super::vals::Wen {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Wen::from_bits(val as u8)
            }
            #[doc = "Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated."]
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
            #[doc = "Erase all non-volatile memory including UICR registers. Note that code erase has to be enabled by CONFIG.EEN before the UICR can be erased."]
            #[inline(always)]
            pub const fn eraseall(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Erase all non-volatile memory including UICR registers. Note that code erase has to be enabled by CONFIG.EEN before the UICR can be erased."]
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
        #[doc = "Register for erasing User Information Configuration Registers"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Eraseuicr(pub u32);
        impl Eraseuicr {
            #[doc = "Register starting erase of all User Information Configuration Registers. Note that code erase has to be enabled by CONFIG.EEN before the UICR can be erased."]
            #[inline(always)]
            pub const fn eraseuicr(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Register starting erase of all User Information Configuration Registers. Note that code erase has to be enabled by CONFIG.EEN before the UICR can be erased."]
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
        #[doc = "I-Code cache configuration register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Icachecnf(pub u32);
        impl Icachecnf {
            #[doc = "Cache enable"]
            #[inline(always)]
            pub const fn cacheen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Cache enable"]
            #[inline(always)]
            pub fn set_cacheen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Cache profiling enable"]
            #[inline(always)]
            pub const fn cacheprofen(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Cache profiling enable"]
            #[inline(always)]
            pub fn set_cacheprofen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for Icachecnf {
            #[inline(always)]
            fn default() -> Icachecnf {
                Icachecnf(0)
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
            #[doc = "Write Enabled"]
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
            #[doc = "Enable or disable interrupt for STARTED event"]
            #[inline(always)]
            pub const fn started(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for STARTED event"]
            #[inline(always)]
            pub fn set_started(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable interrupt for STOPPED event"]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for STOPPED event"]
            #[inline(always)]
            pub fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for END event"]
            #[inline(always)]
            pub const fn end(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for END event"]
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
        #[doc = "Enable constant latency mode"]
        #[inline(always)]
        pub const fn tasks_constlat(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
        }
        #[doc = "Enable low power mode (variable latency)"]
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
        #[doc = "Deprecated register - RAM status register"]
        #[inline(always)]
        pub const fn ramstatus(self) -> crate::common::Reg<regs::Ramstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0428usize) as _) }
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
        #[doc = "Deprecated register - RAM on/off register (this register is retained)"]
        #[inline(always)]
        pub const fn ramon(self) -> crate::common::Reg<regs::Ramon, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
        }
        #[doc = "Deprecated register - RAM on/off register (this register is retained)"]
        #[inline(always)]
        pub const fn ramonb(self) -> crate::common::Reg<regs::Ramonb, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0554usize) as _) }
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
        #[doc = "Description cluster\\[0\\]: RAM0 power control register"]
        #[inline(always)]
        pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster\\[0\\]: RAM0 power control set register"]
        #[inline(always)]
        pub const fn powerset(self) -> crate::common::Reg<regs::Power, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Description cluster\\[0\\]: RAM0 power control clear register"]
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
            #[doc = "Write '1' to Disable interrupt for POFWARN event"]
            #[inline(always)]
            pub const fn pofwarn(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for POFWARN event"]
            #[inline(always)]
            pub fn set_pofwarn(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Write '1' to Disable interrupt for SLEEPENTER event"]
            #[inline(always)]
            pub const fn sleepenter(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for SLEEPENTER event"]
            #[inline(always)]
            pub fn set_sleepenter(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Write '1' to Disable interrupt for SLEEPEXIT event"]
            #[inline(always)]
            pub const fn sleepexit(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for SLEEPEXIT event"]
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
        #[doc = "Description cluster\\[0\\]: RAM0 power control register"]
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
        #[doc = "Deprecated register - RAM on/off register (this register is retained)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ramon(pub u32);
        impl Ramon {
            #[doc = "Keep RAM block 0 on or off in system ON Mode"]
            #[inline(always)]
            pub const fn onram0(&self) -> super::vals::Onram0 {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Onram0::from_bits(val as u8)
            }
            #[doc = "Keep RAM block 0 on or off in system ON Mode"]
            #[inline(always)]
            pub fn set_onram0(&mut self, val: super::vals::Onram0) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Keep RAM block 1 on or off in system ON Mode"]
            #[inline(always)]
            pub const fn onram1(&self) -> super::vals::Onram1 {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Onram1::from_bits(val as u8)
            }
            #[doc = "Keep RAM block 1 on or off in system ON Mode"]
            #[inline(always)]
            pub fn set_onram1(&mut self, val: super::vals::Onram1) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Keep retention on RAM block 0 when RAM block is switched off"]
            #[inline(always)]
            pub const fn offram0(&self) -> super::vals::Offram0 {
                let val = (self.0 >> 16usize) & 0x01;
                super::vals::Offram0::from_bits(val as u8)
            }
            #[doc = "Keep retention on RAM block 0 when RAM block is switched off"]
            #[inline(always)]
            pub fn set_offram0(&mut self, val: super::vals::Offram0) {
                self.0 =
                    (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
            }
            #[doc = "Keep retention on RAM block 1 when RAM block is switched off"]
            #[inline(always)]
            pub const fn offram1(&self) -> super::vals::Offram1 {
                let val = (self.0 >> 17usize) & 0x01;
                super::vals::Offram1::from_bits(val as u8)
            }
            #[doc = "Keep retention on RAM block 1 when RAM block is switched off"]
            #[inline(always)]
            pub fn set_offram1(&mut self, val: super::vals::Offram1) {
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
        #[doc = "Deprecated register - RAM on/off register (this register is retained)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ramonb(pub u32);
        impl Ramonb {
            #[doc = "Keep RAM block 2 on or off in system ON Mode"]
            #[inline(always)]
            pub const fn onram2(&self) -> super::vals::Onram2 {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Onram2::from_bits(val as u8)
            }
            #[doc = "Keep RAM block 2 on or off in system ON Mode"]
            #[inline(always)]
            pub fn set_onram2(&mut self, val: super::vals::Onram2) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Keep RAM block 3 on or off in system ON Mode"]
            #[inline(always)]
            pub const fn onram3(&self) -> super::vals::Onram3 {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Onram3::from_bits(val as u8)
            }
            #[doc = "Keep RAM block 3 on or off in system ON Mode"]
            #[inline(always)]
            pub fn set_onram3(&mut self, val: super::vals::Onram3) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Keep retention on RAM block 2 when RAM block is switched off"]
            #[inline(always)]
            pub const fn offram2(&self) -> super::vals::Offram2 {
                let val = (self.0 >> 16usize) & 0x01;
                super::vals::Offram2::from_bits(val as u8)
            }
            #[doc = "Keep retention on RAM block 2 when RAM block is switched off"]
            #[inline(always)]
            pub fn set_offram2(&mut self, val: super::vals::Offram2) {
                self.0 =
                    (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
            }
            #[doc = "Keep retention on RAM block 3 when RAM block is switched off"]
            #[inline(always)]
            pub const fn offram3(&self) -> super::vals::Offram3 {
                let val = (self.0 >> 17usize) & 0x01;
                super::vals::Offram3::from_bits(val as u8)
            }
            #[doc = "Keep retention on RAM block 3 when RAM block is switched off"]
            #[inline(always)]
            pub fn set_offram3(&mut self, val: super::vals::Offram3) {
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
        #[doc = "Deprecated register - RAM status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ramstatus(pub u32);
        impl Ramstatus {
            #[doc = "RAM block 0 is on or off/powering up"]
            #[inline(always)]
            pub const fn ramblock0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "RAM block 0 is on or off/powering up"]
            #[inline(always)]
            pub fn set_ramblock0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "RAM block 1 is on or off/powering up"]
            #[inline(always)]
            pub const fn ramblock1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "RAM block 1 is on or off/powering up"]
            #[inline(always)]
            pub fn set_ramblock1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "RAM block 2 is on or off/powering up"]
            #[inline(always)]
            pub const fn ramblock2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "RAM block 2 is on or off/powering up"]
            #[inline(always)]
            pub fn set_ramblock2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "RAM block 3 is on or off/powering up"]
            #[inline(always)]
            pub const fn ramblock3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "RAM block 3 is on or off/powering up"]
            #[inline(always)]
            pub fn set_ramblock3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Ramstatus {
            #[inline(always)]
            fn default() -> Ramstatus {
                Ramstatus(0)
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
            #[doc = "Reset due to wake up from System OFF mode when wakeup is triggered from ANADETECT signal from LPCOMP"]
            #[inline(always)]
            pub const fn lpcomp(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Reset due to wake up from System OFF mode when wakeup is triggered from ANADETECT signal from LPCOMP"]
            #[inline(always)]
            pub fn set_lpcomp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
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
            #[doc = "Reset due to wake up from System OFF mode by NFC field detect"]
            #[inline(always)]
            pub const fn nfc(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Reset due to wake up from System OFF mode by NFC field detect"]
            #[inline(always)]
            pub fn set_nfc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
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
        pub enum Offram0 {
            #[doc = "Off"]
            RAM0OFF = 0x0,
            #[doc = "On"]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Offram1 {
            #[doc = "Off"]
            RAM1OFF = 0x0,
            #[doc = "On"]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Offram2 {
            #[doc = "Off"]
            RAM2OFF = 0x0,
            #[doc = "On"]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Offram3 {
            #[doc = "Off"]
            RAM3OFF = 0x0,
            #[doc = "On"]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Onram0 {
            #[doc = "Off"]
            RAM0OFF = 0x0,
            #[doc = "On"]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Onram1 {
            #[doc = "Off"]
            RAM1OFF = 0x0,
            #[doc = "On"]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Onram2 {
            #[doc = "Off"]
            RAM2OFF = 0x0,
            #[doc = "On"]
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
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Onram3 {
            #[doc = "Off"]
            RAM3OFF = 0x0,
            #[doc = "On"]
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
        #[doc = "Description cluster\\[0\\]: Channel 0 event end-point"]
        #[inline(always)]
        pub const fn eep(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster\\[0\\]: Channel 0 task end-point"]
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
        #[doc = "Description cluster\\[0\\]: Channel 0 task end-point"]
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
        #[doc = "Description collection\\[0\\]: Channel group 0"]
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
        #[doc = "Description cluster\\[0\\]: Enable channel group 0"]
        #[inline(always)]
        pub const fn en(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster\\[0\\]: Disable channel group 0"]
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
        #[doc = "Description collection\\[0\\]: Channel group 0"]
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
        #[doc = "Description collection\\[0\\]: Output pin select for PWM channel 0"]
        #[inline(always)]
        pub const fn out(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
        }
    }
    #[doc = "Pulse Width Modulation Unit 0"]
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
        #[doc = "Description collection\\[0\\]: Loads the first PWM value on all enabled channels from sequence 0, and starts playing that sequence at the rate defined in SEQ\\[0\\]REFRESH and/or DECODER.MODE. Causes PWM generation to start it was not running."]
        #[inline(always)]
        pub const fn tasks_seqstart(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
            assert!(n < 2usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 4usize) as _) }
        }
        #[doc = "Steps by one value in the current sequence on all enabled channels if DECODER.MODE=NextStep. Does not cause PWM generation to start it was not running."]
        #[inline(always)]
        pub const fn tasks_nextstep(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Response to STOP task, emitted when PWM pulses are no longer generated"]
        #[inline(always)]
        pub const fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Description collection\\[0\\]: First PWM period started on sequence 0"]
        #[inline(always)]
        pub const fn events_seqstarted(
            self,
            n: usize,
        ) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 2usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize + n * 4usize) as _) }
        }
        #[doc = "Description collection\\[0\\]: Emitted at end of every sequence 0, when last value from RAM has been applied to wave counter"]
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
        #[doc = "Shortcut register"]
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
        #[doc = "Amount of playback of a loop"]
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
        #[doc = "Description cluster\\[0\\]: Beginning address in Data RAM of this sequence"]
        #[inline(always)]
        pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster\\[0\\]: Amount of values (duty cycles) in this sequence"]
        #[inline(always)]
        pub const fn cnt(self) -> crate::common::Reg<regs::Cnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Description cluster\\[0\\]: Amount of additional PWM periods between samples loaded into compare register"]
        #[inline(always)]
        pub const fn refresh(self) -> crate::common::Reg<regs::Refresh, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Description cluster\\[0\\]: Time added after the sequence"]
        #[inline(always)]
        pub const fn enddelay(self) -> crate::common::Reg<regs::Enddelay, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Description cluster\\[0\\]: Amount of values (duty cycles) in this sequence"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cnt(pub u32);
        impl Cnt {
            #[doc = "Amount of values (duty cycles) in this sequence"]
            #[inline(always)]
            pub const fn cnt(&self) -> super::vals::CntCnt {
                let val = (self.0 >> 0usize) & 0x7fff;
                super::vals::CntCnt::from_bits(val as u16)
            }
            #[doc = "Amount of values (duty cycles) in this sequence"]
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
            #[doc = "Value up to which the pulse generator counter counts. This register is ignored when DECODER.MODE=WaveForm and only values from RAM will be used."]
            #[inline(always)]
            pub const fn countertop(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x7fff;
                val as u16
            }
            #[doc = "Value up to which the pulse generator counter counts. This register is ignored when DECODER.MODE=WaveForm and only values from RAM will be used."]
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
        #[doc = "Description cluster\\[0\\]: Time added after the sequence"]
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
            #[doc = "Enable or disable interrupt for STOPPED event"]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for STOPPED event"]
            #[inline(always)]
            pub fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for SEQSTARTED\\[0\\] event"]
            #[inline(always)]
            pub const fn seqstarted0(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for SEQSTARTED\\[0\\] event"]
            #[inline(always)]
            pub fn set_seqstarted0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable or disable interrupt for SEQSTARTED\\[1\\] event"]
            #[inline(always)]
            pub const fn seqstarted1(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for SEQSTARTED\\[1\\] event"]
            #[inline(always)]
            pub fn set_seqstarted1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable or disable interrupt for SEQEND\\[0\\] event"]
            #[inline(always)]
            pub const fn seqend0(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for SEQEND\\[0\\] event"]
            #[inline(always)]
            pub fn set_seqend0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable or disable interrupt for SEQEND\\[1\\] event"]
            #[inline(always)]
            pub const fn seqend1(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for SEQEND\\[1\\] event"]
            #[inline(always)]
            pub fn set_seqend1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable or disable interrupt for PWMPERIODEND event"]
            #[inline(always)]
            pub const fn pwmperiodend(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for PWMPERIODEND event"]
            #[inline(always)]
            pub fn set_pwmperiodend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Enable or disable interrupt for LOOPSDONE event"]
            #[inline(always)]
            pub const fn loopsdone(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for LOOPSDONE event"]
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
        #[doc = "Amount of playback of a loop"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Loop(pub u32);
        impl Loop {
            #[doc = "Amount of playback of pattern cycles"]
            #[inline(always)]
            pub const fn cnt(&self) -> super::vals::LoopCnt {
                let val = (self.0 >> 0usize) & 0xffff;
                super::vals::LoopCnt::from_bits(val as u16)
            }
            #[doc = "Amount of playback of pattern cycles"]
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
            #[doc = "Selects up or up and down as wave counter mode"]
            #[inline(always)]
            pub const fn updown(&self) -> super::vals::Updown {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Updown::from_bits(val as u8)
            }
            #[doc = "Selects up or up and down as wave counter mode"]
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
            #[doc = "Pre-scaler of PWM_CLK"]
            #[inline(always)]
            pub const fn prescaler(&self) -> super::vals::Prescaler {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::Prescaler::from_bits(val as u8)
            }
            #[doc = "Pre-scaler of PWM_CLK"]
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
        #[doc = "Description cluster\\[0\\]: Amount of additional PWM periods between samples loaded into compare register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Refresh(pub u32);
        impl Refresh {
            #[doc = "Amount of additional PWM periods between samples loaded into compare register (load every REFRESH.CNT+1 PWM periods)"]
            #[inline(always)]
            pub const fn cnt(&self) -> super::vals::RefreshCnt {
                let val = (self.0 >> 0usize) & 0x00ff_ffff;
                super::vals::RefreshCnt::from_bits(val as u32)
            }
            #[doc = "Amount of additional PWM periods between samples loaded into compare register (load every REFRESH.CNT+1 PWM periods)"]
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
        #[doc = "Shortcut register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between SEQEND\\[0\\] event and STOP task"]
            #[inline(always)]
            pub const fn seqend0_stop(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between SEQEND\\[0\\] event and STOP task"]
            #[inline(always)]
            pub fn set_seqend0_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Shortcut between SEQEND\\[1\\] event and STOP task"]
            #[inline(always)]
            pub const fn seqend1_stop(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between SEQEND\\[1\\] event and STOP task"]
            #[inline(always)]
            pub fn set_seqend1_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Shortcut between LOOPSDONE event and SEQSTART\\[0\\] task"]
            #[inline(always)]
            pub const fn loopsdone_seqstart0(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between LOOPSDONE event and SEQSTART\\[0\\] task"]
            #[inline(always)]
            pub fn set_loopsdone_seqstart0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Shortcut between LOOPSDONE event and SEQSTART\\[1\\] task"]
            #[inline(always)]
            pub const fn loopsdone_seqstart1(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between LOOPSDONE event and SEQSTART\\[1\\] task"]
            #[inline(always)]
            pub fn set_loopsdone_seqstart1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Shortcut between LOOPSDONE event and STOP task"]
            #[inline(always)]
            pub const fn loopsdone_stop(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between LOOPSDONE event and STOP task"]
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
            #[doc = "Divide by 1 (16MHz)"]
            DIV_1 = 0x0,
            #[doc = "Divide by 2 ( 8MHz)"]
            DIV_2 = 0x01,
            #[doc = "Divide by 4 ( 4MHz)"]
            DIV_4 = 0x02,
            #[doc = "Divide by 8 ( 2MHz)"]
            DIV_8 = 0x03,
            #[doc = "Divide by 16 ( 1MHz)"]
            DIV_16 = 0x04,
            #[doc = "Divide by 32 ( 500kHz)"]
            DIV_32 = 0x05,
            #[doc = "Divide by 64 ( 250kHz)"]
            DIV_64 = 0x06,
            #[doc = "Divide by 128 ( 125kHz)"]
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
            #[doc = "Up counter - edge aligned PWM duty-cycle"]
            UP = 0x0,
            #[doc = "Up and down counter - center aligned PWM duty cycle"]
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
        #[doc = "Shortcut register"]
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
            #[doc = "Write '1' to Disable interrupt for SAMPLERDY event"]
            #[inline(always)]
            pub const fn samplerdy(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for SAMPLERDY event"]
            #[inline(always)]
            pub fn set_samplerdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to Disable interrupt for REPORTRDY event"]
            #[inline(always)]
            pub const fn reportrdy(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for REPORTRDY event"]
            #[inline(always)]
            pub fn set_reportrdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to Disable interrupt for ACCOF event"]
            #[inline(always)]
            pub const fn accof(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for ACCOF event"]
            #[inline(always)]
            pub fn set_accof(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Write '1' to Disable interrupt for DBLRDY event"]
            #[inline(always)]
            pub const fn dblrdy(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for DBLRDY event"]
            #[inline(always)]
            pub fn set_dblrdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Write '1' to Disable interrupt for STOPPED event"]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for STOPPED event"]
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
        #[doc = "Shortcut register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between REPORTRDY event and READCLRACC task"]
            #[inline(always)]
            pub const fn reportrdy_readclracc(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between REPORTRDY event and READCLRACC task"]
            #[inline(always)]
            pub fn set_reportrdy_readclracc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Shortcut between SAMPLERDY event and STOP task"]
            #[inline(always)]
            pub const fn samplerdy_stop(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between SAMPLERDY event and STOP task"]
            #[inline(always)]
            pub fn set_samplerdy_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Shortcut between REPORTRDY event and RDCLRACC task"]
            #[inline(always)]
            pub const fn reportrdy_rdclracc(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between REPORTRDY event and RDCLRACC task"]
            #[inline(always)]
            pub fn set_reportrdy_rdclracc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Shortcut between REPORTRDY event and STOP task"]
            #[inline(always)]
            pub const fn reportrdy_stop(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between REPORTRDY event and STOP task"]
            #[inline(always)]
            pub fn set_reportrdy_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Shortcut between DBLRDY event and RDCLRDBL task"]
            #[inline(always)]
            pub const fn dblrdy_rdclrdbl(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between DBLRDY event and RDCLRDBL task"]
            #[inline(always)]
            pub fn set_dblrdy_rdclrdbl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Shortcut between DBLRDY event and STOP task"]
            #[inline(always)]
            pub const fn dblrdy_stop(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between DBLRDY event and STOP task"]
            #[inline(always)]
            pub fn set_dblrdy_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Shortcut between SAMPLERDY event and READCLRACC task"]
            #[inline(always)]
            pub const fn samplerdy_readclracc(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between SAMPLERDY event and READCLRACC task"]
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
        #[doc = "Shortcut register"]
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
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn unused0(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize) as _) }
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
        #[doc = "Description collection\\[0\\]: Device address base segment 0"]
        #[inline(always)]
        pub const fn dab(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize + n * 4usize) as _) }
        }
        #[doc = "Description collection\\[0\\]: Device address prefix 0"]
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
        #[doc = "Description collection\\[0\\]: Device address prefix 0"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dap(pub u32);
        impl Dap {
            #[doc = "Device address prefix 0"]
            #[inline(always)]
            pub const fn dap(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Device address prefix 0"]
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
            #[doc = "Write '1' to Disable interrupt for READY event"]
            #[inline(always)]
            pub const fn ready(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for READY event"]
            #[inline(always)]
            pub fn set_ready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to Disable interrupt for ADDRESS event"]
            #[inline(always)]
            pub const fn address(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for ADDRESS event"]
            #[inline(always)]
            pub fn set_address(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to Disable interrupt for PAYLOAD event"]
            #[inline(always)]
            pub const fn payload(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for PAYLOAD event"]
            #[inline(always)]
            pub fn set_payload(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Write '1' to Disable interrupt for END event"]
            #[inline(always)]
            pub const fn end(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for END event"]
            #[inline(always)]
            pub fn set_end(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Write '1' to Disable interrupt for DISABLED event"]
            #[inline(always)]
            pub const fn disabled(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for DISABLED event"]
            #[inline(always)]
            pub fn set_disabled(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Write '1' to Disable interrupt for DEVMATCH event"]
            #[inline(always)]
            pub const fn devmatch(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for DEVMATCH event"]
            #[inline(always)]
            pub fn set_devmatch(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Write '1' to Disable interrupt for DEVMISS event"]
            #[inline(always)]
            pub const fn devmiss(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for DEVMISS event"]
            #[inline(always)]
            pub fn set_devmiss(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Write '1' to Disable interrupt for RSSIEND event"]
            #[inline(always)]
            pub const fn rssiend(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for RSSIEND event"]
            #[inline(always)]
            pub fn set_rssiend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Write '1' to Disable interrupt for BCMATCH event"]
            #[inline(always)]
            pub const fn bcmatch(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for BCMATCH event"]
            #[inline(always)]
            pub fn set_bcmatch(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Write '1' to Disable interrupt for CRCOK event"]
            #[inline(always)]
            pub const fn crcok(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for CRCOK event"]
            #[inline(always)]
            pub fn set_crcok(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "Write '1' to Disable interrupt for CRCERROR event"]
            #[inline(always)]
            pub const fn crcerror(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for CRCERROR event"]
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
        #[doc = "Shortcut register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between READY event and START task"]
            #[inline(always)]
            pub const fn ready_start(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between READY event and START task"]
            #[inline(always)]
            pub fn set_ready_start(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Shortcut between END event and DISABLE task"]
            #[inline(always)]
            pub const fn end_disable(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between END event and DISABLE task"]
            #[inline(always)]
            pub fn set_end_disable(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Shortcut between DISABLED event and TXEN task"]
            #[inline(always)]
            pub const fn disabled_txen(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between DISABLED event and TXEN task"]
            #[inline(always)]
            pub fn set_disabled_txen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Shortcut between DISABLED event and RXEN task"]
            #[inline(always)]
            pub const fn disabled_rxen(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between DISABLED event and RXEN task"]
            #[inline(always)]
            pub fn set_disabled_rxen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Shortcut between ADDRESS event and RSSISTART task"]
            #[inline(always)]
            pub const fn address_rssistart(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between ADDRESS event and RSSISTART task"]
            #[inline(always)]
            pub fn set_address_rssistart(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Shortcut between END event and START task"]
            #[inline(always)]
            pub const fn end_start(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between END event and START task"]
            #[inline(always)]
            pub fn set_end_start(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Shortcut between ADDRESS event and BCSTART task"]
            #[inline(always)]
            pub const fn address_bcstart(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between ADDRESS event and BCSTART task"]
            #[inline(always)]
            pub fn set_address_bcstart(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Shortcut between DISABLED event and RSSISTOP task"]
            #[inline(always)]
            pub const fn disabled_rssistop(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between DISABLED event and RSSISTOP task"]
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
            #[doc = "Deprecated enumerator - 250 kbit/s Nordic proprietary radio mode"]
            NRF_250KBIT = 0x02,
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
            _RESERVED_e2 = 0xe2,
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
        #[doc = "Shortcut register"]
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
            #[doc = "Write '1' to Disable interrupt for VALRDY event"]
            #[inline(always)]
            pub const fn valrdy(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for VALRDY event"]
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
        #[doc = "Shortcut register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between VALRDY event and STOP task"]
            #[inline(always)]
            pub const fn valrdy_stop(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between VALRDY event and STOP task"]
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
        #[doc = "Description collection\\[0\\]: Compare event on CC\\[0\\] match"]
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
        #[doc = "Description collection\\[0\\]: Compare register 0"]
        #[inline(always)]
        pub const fn cc(self, n: usize) -> crate::common::Reg<regs::Cc, crate::common::RW> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize + n * 4usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Description collection\\[0\\]: Compare register 0"]
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
            #[doc = "Enable or disable event routing for TICK event"]
            #[inline(always)]
            pub const fn tick(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable event routing for TICK event"]
            #[inline(always)]
            pub fn set_tick(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable event routing for OVRFLW event"]
            #[inline(always)]
            pub const fn ovrflw(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable event routing for OVRFLW event"]
            #[inline(always)]
            pub fn set_ovrflw(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable event routing for COMPARE\\[0\\] event"]
            #[inline(always)]
            pub const fn compare0(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable event routing for COMPARE\\[0\\] event"]
            #[inline(always)]
            pub fn set_compare0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Enable or disable event routing for COMPARE\\[1\\] event"]
            #[inline(always)]
            pub const fn compare1(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable event routing for COMPARE\\[1\\] event"]
            #[inline(always)]
            pub fn set_compare1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Enable or disable event routing for COMPARE\\[2\\] event"]
            #[inline(always)]
            pub const fn compare2(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable event routing for COMPARE\\[2\\] event"]
            #[inline(always)]
            pub fn set_compare2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Enable or disable event routing for COMPARE\\[3\\] event"]
            #[inline(always)]
            pub const fn compare3(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable event routing for COMPARE\\[3\\] event"]
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
            #[doc = "Write '1' to Disable event routing for TICK event"]
            #[inline(always)]
            pub const fn tick(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable event routing for TICK event"]
            #[inline(always)]
            pub fn set_tick(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to Disable event routing for OVRFLW event"]
            #[inline(always)]
            pub const fn ovrflw(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable event routing for OVRFLW event"]
            #[inline(always)]
            pub fn set_ovrflw(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to Disable event routing for COMPARE\\[0\\] event"]
            #[inline(always)]
            pub const fn compare0(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable event routing for COMPARE\\[0\\] event"]
            #[inline(always)]
            pub fn set_compare0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Write '1' to Disable event routing for COMPARE\\[1\\] event"]
            #[inline(always)]
            pub const fn compare1(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable event routing for COMPARE\\[1\\] event"]
            #[inline(always)]
            pub fn set_compare1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Write '1' to Disable event routing for COMPARE\\[2\\] event"]
            #[inline(always)]
            pub const fn compare2(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable event routing for COMPARE\\[2\\] event"]
            #[inline(always)]
            pub fn set_compare2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Write '1' to Disable event routing for COMPARE\\[3\\] event"]
            #[inline(always)]
            pub const fn compare3(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable event routing for COMPARE\\[3\\] event"]
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
            #[doc = "Write '1' to Enable event routing for TICK event"]
            #[inline(always)]
            pub const fn tick(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Enable event routing for TICK event"]
            #[inline(always)]
            pub fn set_tick(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to Enable event routing for OVRFLW event"]
            #[inline(always)]
            pub const fn ovrflw(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Enable event routing for OVRFLW event"]
            #[inline(always)]
            pub fn set_ovrflw(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to Enable event routing for COMPARE\\[0\\] event"]
            #[inline(always)]
            pub const fn compare0(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Enable event routing for COMPARE\\[0\\] event"]
            #[inline(always)]
            pub fn set_compare0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Write '1' to Enable event routing for COMPARE\\[1\\] event"]
            #[inline(always)]
            pub const fn compare1(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Enable event routing for COMPARE\\[1\\] event"]
            #[inline(always)]
            pub fn set_compare1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Write '1' to Enable event routing for COMPARE\\[2\\] event"]
            #[inline(always)]
            pub const fn compare2(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Enable event routing for COMPARE\\[2\\] event"]
            #[inline(always)]
            pub fn set_compare2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Write '1' to Enable event routing for COMPARE\\[3\\] event"]
            #[inline(always)]
            pub const fn compare3(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Enable event routing for COMPARE\\[3\\] event"]
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
            #[doc = "Write '1' to Disable interrupt for TICK event"]
            #[inline(always)]
            pub const fn tick(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for TICK event"]
            #[inline(always)]
            pub fn set_tick(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to Disable interrupt for OVRFLW event"]
            #[inline(always)]
            pub const fn ovrflw(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for OVRFLW event"]
            #[inline(always)]
            pub fn set_ovrflw(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to Disable interrupt for COMPARE\\[0\\] event"]
            #[inline(always)]
            pub const fn compare0(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for COMPARE\\[0\\] event"]
            #[inline(always)]
            pub fn set_compare0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Write '1' to Disable interrupt for COMPARE\\[1\\] event"]
            #[inline(always)]
            pub const fn compare1(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for COMPARE\\[1\\] event"]
            #[inline(always)]
            pub fn set_compare1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Write '1' to Disable interrupt for COMPARE\\[2\\] event"]
            #[inline(always)]
            pub const fn compare2(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for COMPARE\\[2\\] event"]
            #[inline(always)]
            pub fn set_compare2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Write '1' to Disable interrupt for COMPARE\\[3\\] event"]
            #[inline(always)]
            pub const fn compare3(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for COMPARE\\[3\\] event"]
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
        #[doc = "Description cluster\\[0\\]: Input positive pin selection for CH\\[0\\]"]
        #[inline(always)]
        pub const fn pselp(self) -> crate::common::Reg<regs::Pselp, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster\\[0\\]: Input negative pin selection for CH\\[0\\]"]
        #[inline(always)]
        pub const fn pseln(self) -> crate::common::Reg<regs::Pseln, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Description cluster\\[0\\]: Input configuration for CH\\[0\\]"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Description cluster\\[0\\]: High/low limits for event monitoring a channel"]
        #[inline(always)]
        pub const fn limit(self) -> crate::common::Reg<regs::Limit, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
    }
    #[doc = "Unspecified"]
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
        #[doc = "Description cluster\\[0\\]: Last results is equal or above CH\\[0\\].LIMIT.HIGH"]
        #[inline(always)]
        pub const fn limith(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster\\[0\\]: Last results is equal or below CH\\[0\\].LIMIT.LOW"]
        #[inline(always)]
        pub const fn limitl(self) -> crate::common::Reg<u32, crate::common::RW> {
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
        #[doc = "Unspecified"]
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
        #[doc = "Description cluster\\[0\\]: Input configuration for CH\\[0\\]"]
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
            #[doc = "Enable or disable interrupt for STARTED event"]
            #[inline(always)]
            pub const fn started(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for STARTED event"]
            #[inline(always)]
            pub fn set_started(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable interrupt for END event"]
            #[inline(always)]
            pub const fn end(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for END event"]
            #[inline(always)]
            pub fn set_end(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for DONE event"]
            #[inline(always)]
            pub const fn done(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for DONE event"]
            #[inline(always)]
            pub fn set_done(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable or disable interrupt for RESULTDONE event"]
            #[inline(always)]
            pub const fn resultdone(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for RESULTDONE event"]
            #[inline(always)]
            pub fn set_resultdone(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable or disable interrupt for CALIBRATEDONE event"]
            #[inline(always)]
            pub const fn calibratedone(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for CALIBRATEDONE event"]
            #[inline(always)]
            pub fn set_calibratedone(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable or disable interrupt for STOPPED event"]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for STOPPED event"]
            #[inline(always)]
            pub fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable or disable interrupt for CH\\[0\\].LIMITH event"]
            #[inline(always)]
            pub const fn ch0limith(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for CH\\[0\\].LIMITH event"]
            #[inline(always)]
            pub fn set_ch0limith(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Enable or disable interrupt for CH\\[0\\].LIMITL event"]
            #[inline(always)]
            pub const fn ch0limitl(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for CH\\[0\\].LIMITL event"]
            #[inline(always)]
            pub fn set_ch0limitl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Enable or disable interrupt for CH\\[1\\].LIMITH event"]
            #[inline(always)]
            pub const fn ch1limith(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for CH\\[1\\].LIMITH event"]
            #[inline(always)]
            pub fn set_ch1limith(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Enable or disable interrupt for CH\\[1\\].LIMITL event"]
            #[inline(always)]
            pub const fn ch1limitl(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for CH\\[1\\].LIMITL event"]
            #[inline(always)]
            pub fn set_ch1limitl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Enable or disable interrupt for CH\\[2\\].LIMITH event"]
            #[inline(always)]
            pub const fn ch2limith(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for CH\\[2\\].LIMITH event"]
            #[inline(always)]
            pub fn set_ch2limith(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Enable or disable interrupt for CH\\[2\\].LIMITL event"]
            #[inline(always)]
            pub const fn ch2limitl(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for CH\\[2\\].LIMITL event"]
            #[inline(always)]
            pub fn set_ch2limitl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "Enable or disable interrupt for CH\\[3\\].LIMITH event"]
            #[inline(always)]
            pub const fn ch3limith(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for CH\\[3\\].LIMITH event"]
            #[inline(always)]
            pub fn set_ch3limith(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "Enable or disable interrupt for CH\\[3\\].LIMITL event"]
            #[inline(always)]
            pub const fn ch3limitl(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for CH\\[3\\].LIMITL event"]
            #[inline(always)]
            pub fn set_ch3limitl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Enable or disable interrupt for CH\\[4\\].LIMITH event"]
            #[inline(always)]
            pub const fn ch4limith(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for CH\\[4\\].LIMITH event"]
            #[inline(always)]
            pub fn set_ch4limith(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Enable or disable interrupt for CH\\[4\\].LIMITL event"]
            #[inline(always)]
            pub const fn ch4limitl(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for CH\\[4\\].LIMITL event"]
            #[inline(always)]
            pub fn set_ch4limitl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
            #[doc = "Enable or disable interrupt for CH\\[5\\].LIMITH event"]
            #[inline(always)]
            pub const fn ch5limith(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for CH\\[5\\].LIMITH event"]
            #[inline(always)]
            pub fn set_ch5limith(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Enable or disable interrupt for CH\\[5\\].LIMITL event"]
            #[inline(always)]
            pub const fn ch5limitl(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for CH\\[5\\].LIMITL event"]
            #[inline(always)]
            pub fn set_ch5limitl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Enable or disable interrupt for CH\\[6\\].LIMITH event"]
            #[inline(always)]
            pub const fn ch6limith(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for CH\\[6\\].LIMITH event"]
            #[inline(always)]
            pub fn set_ch6limith(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Enable or disable interrupt for CH\\[6\\].LIMITL event"]
            #[inline(always)]
            pub const fn ch6limitl(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for CH\\[6\\].LIMITL event"]
            #[inline(always)]
            pub fn set_ch6limitl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "Enable or disable interrupt for CH\\[7\\].LIMITH event"]
            #[inline(always)]
            pub const fn ch7limith(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for CH\\[7\\].LIMITH event"]
            #[inline(always)]
            pub fn set_ch7limith(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "Enable or disable interrupt for CH\\[7\\].LIMITL event"]
            #[inline(always)]
            pub const fn ch7limitl(&self) -> bool {
                let val = (self.0 >> 21usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for CH\\[7\\].LIMITL event"]
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
        #[doc = "Description cluster\\[0\\]: High/low limits for event monitoring a channel"]
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
        #[doc = "Description cluster\\[0\\]: Input negative pin selection for CH\\[0\\]"]
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
        #[doc = "Description cluster\\[0\\]: Input positive pin selection for CH\\[0\\]"]
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
        #[doc = "Pin select for LRCK signal."]
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
        #[doc = "Pin select for MOSI"]
        #[inline(always)]
        pub const fn mosi(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Pin select for MISO"]
        #[inline(always)]
        pub const fn miso(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
    }
    #[doc = "Serial Peripheral Interface 0"]
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
        #[doc = "SPI frequency"]
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
        #[doc = "SPI frequency"]
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
            #[doc = "Write '1' to Disable interrupt for READY event"]
            #[inline(always)]
            pub const fn ready(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for READY event"]
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
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pselmiso(pub u32);
        impl Pselmiso {
            #[doc = "Disconnect"]
            pub const DISCONNECTED: Self = Self(0xffff_ffff);
        }
        impl Pselmiso {
            pub const fn from_bits(val: u32) -> Pselmiso {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl From<u32> for Pselmiso {
            #[inline(always)]
            fn from(val: u32) -> Pselmiso {
                Pselmiso::from_bits(val)
            }
        }
        impl From<Pselmiso> for u32 {
            #[inline(always)]
            fn from(val: Pselmiso) -> u32 {
                Pselmiso::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pselmosi(pub u32);
        impl Pselmosi {
            #[doc = "Disconnect"]
            pub const DISCONNECTED: Self = Self(0xffff_ffff);
        }
        impl Pselmosi {
            pub const fn from_bits(val: u32) -> Pselmosi {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl From<u32> for Pselmosi {
            #[inline(always)]
            fn from(val: u32) -> Pselmosi {
                Pselmosi::from_bits(val)
            }
        }
        impl From<Pselmosi> for u32 {
            #[inline(always)]
            fn from(val: Pselmosi) -> u32 {
                Pselmosi::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pselsck(pub u32);
        impl Pselsck {
            #[doc = "Disconnect"]
            pub const DISCONNECTED: Self = Self(0xffff_ffff);
        }
        impl Pselsck {
            pub const fn from_bits(val: u32) -> Pselsck {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl From<u32> for Pselsck {
            #[inline(always)]
            fn from(val: u32) -> Pselsck {
                Pselsck::from_bits(val)
            }
        }
        impl From<Pselsck> for u32 {
            #[inline(always)]
            fn from(val: Pselsck) -> u32 {
                Pselsck::to_bits(val)
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
    #[doc = "Serial Peripheral Interface Master with EasyDMA 0"]
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
        #[doc = "Shortcut register"]
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
            #[doc = "Write '1' to Disable interrupt for STOPPED event"]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for STOPPED event"]
            #[inline(always)]
            pub fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to Disable interrupt for ENDRX event"]
            #[inline(always)]
            pub const fn endrx(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for ENDRX event"]
            #[inline(always)]
            pub fn set_endrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Write '1' to Disable interrupt for END event"]
            #[inline(always)]
            pub const fn end(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for END event"]
            #[inline(always)]
            pub fn set_end(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Write '1' to Disable interrupt for ENDTX event"]
            #[inline(always)]
            pub const fn endtx(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for ENDTX event"]
            #[inline(always)]
            pub fn set_endtx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Write '1' to Disable interrupt for STARTED event"]
            #[inline(always)]
            pub const fn started(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for STARTED event"]
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
            pub const fn amount(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Number of bytes transferred in the last transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
            pub const fn maxcnt(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Maximum number of bytes in receive buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for RxdMaxcnt {
            #[inline(always)]
            fn default() -> RxdMaxcnt {
                RxdMaxcnt(0)
            }
        }
        #[doc = "Shortcut register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between END event and START task"]
            #[inline(always)]
            pub const fn end_start(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between END event and START task"]
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
            pub const fn amount(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Number of bytes transferred in the last transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
            pub const fn maxcnt(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    }
    #[doc = "SPI Slave 0"]
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
        #[doc = "Shortcut register"]
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
            #[doc = "Write '1' to Disable interrupt for END event"]
            #[inline(always)]
            pub const fn end(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for END event"]
            #[inline(always)]
            pub fn set_end(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to Disable interrupt for ENDRX event"]
            #[inline(always)]
            pub const fn endrx(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for ENDRX event"]
            #[inline(always)]
            pub fn set_endrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Write '1' to Disable interrupt for ACQUIRED event"]
            #[inline(always)]
            pub const fn acquired(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for ACQUIRED event"]
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
            pub const fn amount(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Number of bytes received in the last granted transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
            pub const fn maxcnt(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Maximum number of bytes in receive buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
        #[doc = "Shortcut register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between END event and ACQUIRE task"]
            #[inline(always)]
            pub const fn end_acquire(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between END event and ACQUIRE task"]
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
            pub const fn amount(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Number of bytes transmitted in last granted transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
            pub const fn maxcnt(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
            #[doc = "Write '1' to Disable interrupt for DATARDY event"]
            #[inline(always)]
            pub const fn datardy(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for DATARDY event"]
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
        #[doc = "Description collection\\[0\\]: Capture Timer value to CC\\[0\\] register"]
        #[inline(always)]
        pub const fn tasks_capture(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize + n * 4usize) as _) }
        }
        #[doc = "Description collection\\[0\\]: Compare event on CC\\[0\\] match"]
        #[inline(always)]
        pub const fn events_compare(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize + n * 4usize) as _) }
        }
        #[doc = "Shortcut register"]
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
        #[doc = "Description collection\\[0\\]: Capture/Compare register 0"]
        #[inline(always)]
        pub const fn cc(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 4usize);
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
            #[doc = "Write '1' to Disable interrupt for COMPARE\\[0\\] event"]
            #[inline(always)]
            pub const fn compare(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 16usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for COMPARE\\[0\\] event"]
            #[inline(always)]
            pub fn set_compare(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
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
        #[doc = "Shortcut register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between COMPARE\\[0\\] event and CLEAR task"]
            #[inline(always)]
            pub const fn compare_clear(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between COMPARE\\[0\\] event and CLEAR task"]
            #[inline(always)]
            pub fn set_compare_clear(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Shortcut between COMPARE\\[0\\] event and STOP task"]
            #[inline(always)]
            pub const fn compare_stop(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 8usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between COMPARE\\[0\\] event and STOP task"]
            #[inline(always)]
            pub fn set_compare_stop(&mut self, n: usize, val: bool) {
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
pub mod timer3 {
    #[doc = "Timer/Counter 3"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer3 {
        ptr: *mut u8,
    }
    unsafe impl Send for Timer3 {}
    unsafe impl Sync for Timer3 {}
    impl Timer3 {
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
        #[doc = "Description collection\\[0\\]: Capture Timer value to CC\\[0\\] register"]
        #[inline(always)]
        pub const fn tasks_capture(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
            assert!(n < 6usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize + n * 4usize) as _) }
        }
        #[doc = "Description collection\\[0\\]: Compare event on CC\\[0\\] match"]
        #[inline(always)]
        pub const fn events_compare(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 6usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize + n * 4usize) as _) }
        }
        #[doc = "Shortcut register"]
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
        #[doc = "Description collection\\[0\\]: Capture/Compare register 0"]
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
            #[doc = "Write '1' to Disable interrupt for COMPARE\\[0\\] event"]
            #[inline(always)]
            pub const fn compare(&self, n: usize) -> bool {
                assert!(n < 6usize);
                let offs = 16usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for COMPARE\\[0\\] event"]
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
        #[doc = "Shortcut register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between COMPARE\\[0\\] event and CLEAR task"]
            #[inline(always)]
            pub const fn compare_clear(&self, n: usize) -> bool {
                assert!(n < 6usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between COMPARE\\[0\\] event and CLEAR task"]
            #[inline(always)]
            pub fn set_compare_clear(&mut self, n: usize, val: bool) {
                assert!(n < 6usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Shortcut between COMPARE\\[0\\] event and STOP task"]
            #[inline(always)]
            pub const fn compare_stop(&self, n: usize) -> bool {
                assert!(n < 6usize);
                let offs = 8usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between COMPARE\\[0\\] event and STOP task"]
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
    #[doc = "I2C compatible Two-Wire Interface 0"]
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
        #[doc = "Shortcut register"]
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
        #[doc = "Pin select for SCL"]
        #[inline(always)]
        pub const fn pselscl(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Pin select for SDA"]
        #[inline(always)]
        pub const fn pselsda(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
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
        #[doc = "TWI frequency"]
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
        #[doc = "TWI frequency"]
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
            #[doc = "Write '1' to Disable interrupt for STOPPED event"]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for STOPPED event"]
            #[inline(always)]
            pub fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to Disable interrupt for RXDREADY event"]
            #[inline(always)]
            pub const fn rxdready(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for RXDREADY event"]
            #[inline(always)]
            pub fn set_rxdready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Write '1' to Disable interrupt for TXDSENT event"]
            #[inline(always)]
            pub const fn txdsent(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for TXDSENT event"]
            #[inline(always)]
            pub fn set_txdsent(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Write '1' to Disable interrupt for ERROR event"]
            #[inline(always)]
            pub const fn error(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for ERROR event"]
            #[inline(always)]
            pub fn set_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Write '1' to Disable interrupt for BB event"]
            #[inline(always)]
            pub const fn bb(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for BB event"]
            #[inline(always)]
            pub fn set_bb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Write '1' to Disable interrupt for SUSPENDED event"]
            #[inline(always)]
            pub const fn suspended(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for SUSPENDED event"]
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
        #[doc = "Shortcut register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between BB event and SUSPEND task"]
            #[inline(always)]
            pub const fn bb_suspend(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between BB event and SUSPEND task"]
            #[inline(always)]
            pub fn set_bb_suspend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Shortcut between BB event and STOP task"]
            #[inline(always)]
            pub const fn bb_stop(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between BB event and STOP task"]
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
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pselscl(pub u32);
        impl Pselscl {
            #[doc = "Disconnect"]
            pub const DISCONNECTED: Self = Self(0xffff_ffff);
        }
        impl Pselscl {
            pub const fn from_bits(val: u32) -> Pselscl {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl From<u32> for Pselscl {
            #[inline(always)]
            fn from(val: u32) -> Pselscl {
                Pselscl::from_bits(val)
            }
        }
        impl From<Pselscl> for u32 {
            #[inline(always)]
            fn from(val: Pselscl) -> u32 {
                Pselscl::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pselsda(pub u32);
        impl Pselsda {
            #[doc = "Disconnect"]
            pub const DISCONNECTED: Self = Self(0xffff_ffff);
        }
        impl Pselsda {
            pub const fn from_bits(val: u32) -> Pselsda {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl From<u32> for Pselsda {
            #[inline(always)]
            fn from(val: u32) -> Pselsda {
                Pselsda::from_bits(val)
            }
        }
        impl From<Pselsda> for u32 {
            #[inline(always)]
            fn from(val: Pselsda) -> u32 {
                Pselsda::to_bits(val)
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
    #[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 0"]
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
        #[doc = "Shortcut register"]
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
        #[doc = "TWI frequency"]
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
        #[doc = "TWI frequency"]
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
            #[doc = "Enable or disable interrupt for STOPPED event"]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for STOPPED event"]
            #[inline(always)]
            pub fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for ERROR event"]
            #[inline(always)]
            pub const fn error(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for ERROR event"]
            #[inline(always)]
            pub fn set_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Enable or disable interrupt for SUSPENDED event"]
            #[inline(always)]
            pub const fn suspended(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for SUSPENDED event"]
            #[inline(always)]
            pub fn set_suspended(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Enable or disable interrupt for RXSTARTED event"]
            #[inline(always)]
            pub const fn rxstarted(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for RXSTARTED event"]
            #[inline(always)]
            pub fn set_rxstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "Enable or disable interrupt for TXSTARTED event"]
            #[inline(always)]
            pub const fn txstarted(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for TXSTARTED event"]
            #[inline(always)]
            pub fn set_txstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "Enable or disable interrupt for LASTRX event"]
            #[inline(always)]
            pub const fn lastrx(&self) -> bool {
                let val = (self.0 >> 23usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for LASTRX event"]
            #[inline(always)]
            pub fn set_lastrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
            }
            #[doc = "Enable or disable interrupt for LASTTX event"]
            #[inline(always)]
            pub const fn lasttx(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for LASTTX event"]
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
            pub const fn amount(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Number of bytes transferred in the last transaction. In case of NACK error, includes the NACK'ed byte."]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
            pub const fn maxcnt(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Maximum number of bytes in receive buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for RxdMaxcnt {
            #[inline(always)]
            fn default() -> RxdMaxcnt {
                RxdMaxcnt(0)
            }
        }
        #[doc = "Shortcut register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between LASTTX event and STARTRX task"]
            #[inline(always)]
            pub const fn lasttx_startrx(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between LASTTX event and STARTRX task"]
            #[inline(always)]
            pub fn set_lasttx_startrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Shortcut between LASTTX event and SUSPEND task"]
            #[inline(always)]
            pub const fn lasttx_suspend(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between LASTTX event and SUSPEND task"]
            #[inline(always)]
            pub fn set_lasttx_suspend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Shortcut between LASTTX event and STOP task"]
            #[inline(always)]
            pub const fn lasttx_stop(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between LASTTX event and STOP task"]
            #[inline(always)]
            pub fn set_lasttx_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Shortcut between LASTRX event and STARTTX task"]
            #[inline(always)]
            pub const fn lastrx_starttx(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between LASTRX event and STARTTX task"]
            #[inline(always)]
            pub fn set_lastrx_starttx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Shortcut between LASTRX event and STOP task"]
            #[inline(always)]
            pub const fn lastrx_stop(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between LASTRX event and STOP task"]
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
            pub const fn amount(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Number of bytes transferred in the last transaction. In case of NACK error, includes the NACK'ed byte."]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
            pub const fn maxcnt(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    }
    #[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 0"]
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
        #[doc = "Shortcut register"]
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
        #[doc = "Description collection\\[0\\]: TWI slave address 0"]
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
    }
    pub mod regs {
        #[doc = "Description collection\\[0\\]: TWI slave address 0"]
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
            #[doc = "Enable or disable interrupt for STOPPED event"]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for STOPPED event"]
            #[inline(always)]
            pub fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for ERROR event"]
            #[inline(always)]
            pub const fn error(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for ERROR event"]
            #[inline(always)]
            pub fn set_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Enable or disable interrupt for RXSTARTED event"]
            #[inline(always)]
            pub const fn rxstarted(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for RXSTARTED event"]
            #[inline(always)]
            pub fn set_rxstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "Enable or disable interrupt for TXSTARTED event"]
            #[inline(always)]
            pub const fn txstarted(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for TXSTARTED event"]
            #[inline(always)]
            pub fn set_txstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "Enable or disable interrupt for WRITE event"]
            #[inline(always)]
            pub const fn write(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for WRITE event"]
            #[inline(always)]
            pub fn set_write(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
            #[doc = "Enable or disable interrupt for READ event"]
            #[inline(always)]
            pub const fn read(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for READ event"]
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
            pub const fn amount(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Number of bytes transferred in the last RXD transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for RxdAmount {
            #[inline(always)]
            fn default() -> RxdAmount {
                RxdAmount(0)
            }
        }
        #[doc = "Maximum number of bytes in RXD buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdMaxcnt(pub u32);
        impl RxdMaxcnt {
            #[doc = "Maximum number of bytes in RXD buffer"]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Maximum number of bytes in RXD buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for RxdMaxcnt {
            #[inline(always)]
            fn default() -> RxdMaxcnt {
                RxdMaxcnt(0)
            }
        }
        #[doc = "Shortcut register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between WRITE event and SUSPEND task"]
            #[inline(always)]
            pub const fn write_suspend(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between WRITE event and SUSPEND task"]
            #[inline(always)]
            pub fn set_write_suspend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Shortcut between READ event and SUSPEND task"]
            #[inline(always)]
            pub const fn read_suspend(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between READ event and SUSPEND task"]
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
            pub const fn amount(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Number of bytes transferred in the last TXD transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for TxdAmount {
            #[inline(always)]
            fn default() -> TxdAmount {
                TxdAmount(0)
            }
        }
        #[doc = "Maximum number of bytes in TXD buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdMaxcnt(pub u32);
        impl TxdMaxcnt {
            #[doc = "Maximum number of bytes in TXD buffer"]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Maximum number of bytes in TXD buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    }
}
pub mod uart {
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
        #[doc = "Shortcut register"]
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
        #[doc = "Pin select for RTS"]
        #[inline(always)]
        pub const fn pselrts(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Pin select for TXD"]
        #[inline(always)]
        pub const fn pseltxd(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Pin select for CTS"]
        #[inline(always)]
        pub const fn pselcts(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "Pin select for RXD"]
        #[inline(always)]
        pub const fn pselrxd(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
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
        #[doc = "Baud rate"]
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
        #[doc = "Baud rate"]
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
            #[doc = "Write '1' to Disable interrupt for CTS event"]
            #[inline(always)]
            pub const fn cts(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for CTS event"]
            #[inline(always)]
            pub fn set_cts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to Disable interrupt for NCTS event"]
            #[inline(always)]
            pub const fn ncts(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for NCTS event"]
            #[inline(always)]
            pub fn set_ncts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to Disable interrupt for RXDRDY event"]
            #[inline(always)]
            pub const fn rxdrdy(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for RXDRDY event"]
            #[inline(always)]
            pub fn set_rxdrdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Write '1' to Disable interrupt for TXDRDY event"]
            #[inline(always)]
            pub const fn txdrdy(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for TXDRDY event"]
            #[inline(always)]
            pub fn set_txdrdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Write '1' to Disable interrupt for ERROR event"]
            #[inline(always)]
            pub const fn error(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for ERROR event"]
            #[inline(always)]
            pub fn set_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Write '1' to Disable interrupt for RXTO event"]
            #[inline(always)]
            pub const fn rxto(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for RXTO event"]
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
        #[doc = "Shortcut register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between CTS event and STARTRX task"]
            #[inline(always)]
            pub const fn cts_startrx(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between CTS event and STARTRX task"]
            #[inline(always)]
            pub fn set_cts_startrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Shortcut between NCTS event and STOPRX task"]
            #[inline(always)]
            pub const fn ncts_stoprx(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between NCTS event and STOPRX task"]
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
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pselcts(pub u32);
        impl Pselcts {
            #[doc = "Disconnect"]
            pub const DISCONNECTED: Self = Self(0xffff_ffff);
        }
        impl Pselcts {
            pub const fn from_bits(val: u32) -> Pselcts {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl From<u32> for Pselcts {
            #[inline(always)]
            fn from(val: u32) -> Pselcts {
                Pselcts::from_bits(val)
            }
        }
        impl From<Pselcts> for u32 {
            #[inline(always)]
            fn from(val: Pselcts) -> u32 {
                Pselcts::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pselrts(pub u32);
        impl Pselrts {
            #[doc = "Disconnect"]
            pub const DISCONNECTED: Self = Self(0xffff_ffff);
        }
        impl Pselrts {
            pub const fn from_bits(val: u32) -> Pselrts {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl From<u32> for Pselrts {
            #[inline(always)]
            fn from(val: u32) -> Pselrts {
                Pselrts::from_bits(val)
            }
        }
        impl From<Pselrts> for u32 {
            #[inline(always)]
            fn from(val: Pselrts) -> u32 {
                Pselrts::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pselrxd(pub u32);
        impl Pselrxd {
            #[doc = "Disconnect"]
            pub const DISCONNECTED: Self = Self(0xffff_ffff);
        }
        impl Pselrxd {
            pub const fn from_bits(val: u32) -> Pselrxd {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl From<u32> for Pselrxd {
            #[inline(always)]
            fn from(val: u32) -> Pselrxd {
                Pselrxd::from_bits(val)
            }
        }
        impl From<Pselrxd> for u32 {
            #[inline(always)]
            fn from(val: Pselrxd) -> u32 {
                Pselrxd::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pseltxd(pub u32);
        impl Pseltxd {
            #[doc = "Disconnect"]
            pub const DISCONNECTED: Self = Self(0xffff_ffff);
        }
        impl Pseltxd {
            pub const fn from_bits(val: u32) -> Pseltxd {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl From<u32> for Pseltxd {
            #[inline(always)]
            fn from(val: u32) -> Pseltxd {
                Pseltxd::from_bits(val)
            }
        }
        impl From<Pseltxd> for u32 {
            #[inline(always)]
            fn from(val: Pseltxd) -> u32 {
                Pseltxd::to_bits(val)
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
        #[doc = "Shortcut register"]
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
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Inten(pub u32);
        impl Inten {
            #[doc = "Enable or disable interrupt for CTS event"]
            #[inline(always)]
            pub const fn cts(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for CTS event"]
            #[inline(always)]
            pub fn set_cts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable interrupt for NCTS event"]
            #[inline(always)]
            pub const fn ncts(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for NCTS event"]
            #[inline(always)]
            pub fn set_ncts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for RXDRDY event"]
            #[inline(always)]
            pub const fn rxdrdy(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for RXDRDY event"]
            #[inline(always)]
            pub fn set_rxdrdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable or disable interrupt for ENDRX event"]
            #[inline(always)]
            pub const fn endrx(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for ENDRX event"]
            #[inline(always)]
            pub fn set_endrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable or disable interrupt for TXDRDY event"]
            #[inline(always)]
            pub const fn txdrdy(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for TXDRDY event"]
            #[inline(always)]
            pub fn set_txdrdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Enable or disable interrupt for ENDTX event"]
            #[inline(always)]
            pub const fn endtx(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for ENDTX event"]
            #[inline(always)]
            pub fn set_endtx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Enable or disable interrupt for ERROR event"]
            #[inline(always)]
            pub const fn error(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for ERROR event"]
            #[inline(always)]
            pub fn set_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Enable or disable interrupt for RXTO event"]
            #[inline(always)]
            pub const fn rxto(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for RXTO event"]
            #[inline(always)]
            pub fn set_rxto(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Enable or disable interrupt for RXSTARTED event"]
            #[inline(always)]
            pub const fn rxstarted(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for RXSTARTED event"]
            #[inline(always)]
            pub fn set_rxstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "Enable or disable interrupt for TXSTARTED event"]
            #[inline(always)]
            pub const fn txstarted(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for TXSTARTED event"]
            #[inline(always)]
            pub fn set_txstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "Enable or disable interrupt for TXSTOPPED event"]
            #[inline(always)]
            pub const fn txstopped(&self) -> bool {
                let val = (self.0 >> 22usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for TXSTOPPED event"]
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
            pub const fn amount(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Number of bytes transferred in the last transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
            pub const fn maxcnt(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Maximum number of bytes in receive buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for RxdMaxcnt {
            #[inline(always)]
            fn default() -> RxdMaxcnt {
                RxdMaxcnt(0)
            }
        }
        #[doc = "Shortcut register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between ENDRX event and STARTRX task"]
            #[inline(always)]
            pub const fn endrx_startrx(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between ENDRX event and STARTRX task"]
            #[inline(always)]
            pub fn set_endrx_startrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Shortcut between ENDRX event and STOPRX task"]
            #[inline(always)]
            pub const fn endrx_stoprx(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between ENDRX event and STOPRX task"]
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
            pub const fn amount(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Number of bytes transferred in the last transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
            pub const fn maxcnt(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn unused0(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn unused1(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn unused2(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn unused3(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Description collection\\[0\\]: Reserved for Nordic firmware design"]
        #[inline(always)]
        pub const fn nrffw(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 15usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize + n * 4usize) as _) }
        }
        #[doc = "Description collection\\[0\\]: Reserved for Nordic hardware design"]
        #[inline(always)]
        pub const fn nrfhw(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 12usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize + n * 4usize) as _) }
        }
        #[doc = "Description collection\\[0\\]: Reserved for customer"]
        #[inline(always)]
        pub const fn customer(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 32usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
        }
        #[doc = "Description collection\\[0\\]: Mapping of the nRESET function (see POWER chapter for details)"]
        #[inline(always)]
        pub const fn pselreset(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            assert!(n < 2usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize + n * 4usize) as _) }
        }
        #[doc = "Access Port protection"]
        #[inline(always)]
        pub const fn approtect(self) -> crate::common::Reg<regs::Approtect, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
        }
        #[doc = "Setting of pins dedicated to NFC functionality: NFC antenna or GPIO"]
        #[inline(always)]
        pub const fn nfcpins(self) -> crate::common::Reg<regs::Nfcpins, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x020cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Access Port protection"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Approtect(pub u32);
        impl Approtect {
            #[doc = "Enable or disable Access Port protection. Any other value than 0xFF being written to this field will enable protection."]
            #[inline(always)]
            pub const fn pall(&self) -> super::vals::Pall {
                let val = (self.0 >> 0usize) & 0xff;
                super::vals::Pall::from_bits(val as u8)
            }
            #[doc = "Enable or disable Access Port protection. Any other value than 0xFF being written to this field will enable protection."]
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
        #[doc = "Setting of pins dedicated to NFC functionality: NFC antenna or GPIO"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Nfcpins(pub u32);
        impl Nfcpins {
            #[doc = "Setting of pins dedicated to NFC functionality"]
            #[inline(always)]
            pub const fn protect(&self) -> super::vals::Protect {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Protect::from_bits(val as u8)
            }
            #[doc = "Setting of pins dedicated to NFC functionality"]
            #[inline(always)]
            pub fn set_protect(&mut self, val: super::vals::Protect) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Nfcpins {
            #[inline(always)]
            fn default() -> Nfcpins {
                Nfcpins(0)
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
            #[doc = "Disable"]
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
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Protect {
            #[doc = "Operation as GPIO pins. Same protection as normal GPIO pins"]
            DISABLED = 0x0,
            #[doc = "Operation as NFC antenna pins. Configures the protection for NFC operation"]
            NFC = 0x01,
        }
        impl Protect {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Protect {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Protect {
            #[inline(always)]
            fn from(val: u8) -> Protect {
                Protect::from_bits(val)
            }
        }
        impl From<Protect> for u8 {
            #[inline(always)]
            fn from(val: Protect) -> u8 {
                Protect::to_bits(val)
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
        #[doc = "Description collection\\[0\\]: Reload request 0"]
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
            #[doc = "Write '1' to Disable interrupt for TIMEOUT event"]
            #[inline(always)]
            pub const fn timeout(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to Disable interrupt for TIMEOUT event"]
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
        #[doc = "Description collection\\[0\\]: Reload request 0"]
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
