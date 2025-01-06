#![doc = "Peripheral access API (generated using chiptool v0.1.0 (26983da 2025-01-02))"]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "3 - SPU"]
    SPU = 3,
    #[doc = "5 - CLOCK_POWER"]
    CLOCK_POWER = 5,
    #[doc = "8 - SERIAL0"]
    SERIAL0 = 8,
    #[doc = "9 - SERIAL1"]
    SERIAL1 = 9,
    #[doc = "10 - SERIAL2"]
    SERIAL2 = 10,
    #[doc = "11 - SERIAL3"]
    SERIAL3 = 11,
    #[doc = "13 - GPIOTE0"]
    GPIOTE0 = 13,
    #[doc = "14 - SAADC"]
    SAADC = 14,
    #[doc = "15 - TIMER0"]
    TIMER0 = 15,
    #[doc = "16 - TIMER1"]
    TIMER1 = 16,
    #[doc = "17 - TIMER2"]
    TIMER2 = 17,
    #[doc = "20 - RTC0"]
    RTC0 = 20,
    #[doc = "21 - RTC1"]
    RTC1 = 21,
    #[doc = "24 - WDT"]
    WDT = 24,
    #[doc = "27 - EGU0"]
    EGU0 = 27,
    #[doc = "28 - EGU1"]
    EGU1 = 28,
    #[doc = "29 - EGU2"]
    EGU2 = 29,
    #[doc = "30 - EGU3"]
    EGU3 = 30,
    #[doc = "31 - EGU4"]
    EGU4 = 31,
    #[doc = "32 - EGU5"]
    EGU5 = 32,
    #[doc = "33 - PWM0"]
    PWM0 = 33,
    #[doc = "34 - PWM1"]
    PWM1 = 34,
    #[doc = "35 - PWM2"]
    PWM2 = 35,
    #[doc = "36 - PWM3"]
    PWM3 = 36,
    #[doc = "38 - PDM"]
    PDM = 38,
    #[doc = "40 - I2S"]
    I2S = 40,
    #[doc = "42 - IPC"]
    IPC = 42,
    #[doc = "44 - FPU"]
    FPU = 44,
    #[doc = "49 - GPIOTE1"]
    GPIOTE1 = 49,
    #[doc = "57 - KMU"]
    KMU = 57,
    #[doc = "64 - CRYPTOCELL"]
    CRYPTOCELL = 64,
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
        fn SPU();
        fn CLOCK_POWER();
        fn SERIAL0();
        fn SERIAL1();
        fn SERIAL2();
        fn SERIAL3();
        fn GPIOTE0();
        fn SAADC();
        fn TIMER0();
        fn TIMER1();
        fn TIMER2();
        fn RTC0();
        fn RTC1();
        fn WDT();
        fn EGU0();
        fn EGU1();
        fn EGU2();
        fn EGU3();
        fn EGU4();
        fn EGU5();
        fn PWM0();
        fn PWM1();
        fn PWM2();
        fn PWM3();
        fn PDM();
        fn I2S();
        fn IPC();
        fn FPU();
        fn GPIOTE1();
        fn KMU();
        fn CRYPTOCELL();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[link_section = ".vector_table.interrupts"]
    #[no_mangle]
    pub static __INTERRUPTS: [Vector; 65] = [
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: SPU },
        Vector { _reserved: 0 },
        Vector {
            _handler: CLOCK_POWER,
        },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: SERIAL0 },
        Vector { _handler: SERIAL1 },
        Vector { _handler: SERIAL2 },
        Vector { _handler: SERIAL3 },
        Vector { _reserved: 0 },
        Vector { _handler: GPIOTE0 },
        Vector { _handler: SAADC },
        Vector { _handler: TIMER0 },
        Vector { _handler: TIMER1 },
        Vector { _handler: TIMER2 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: RTC0 },
        Vector { _handler: RTC1 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: WDT },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: EGU0 },
        Vector { _handler: EGU1 },
        Vector { _handler: EGU2 },
        Vector { _handler: EGU3 },
        Vector { _handler: EGU4 },
        Vector { _handler: EGU5 },
        Vector { _handler: PWM0 },
        Vector { _handler: PWM1 },
        Vector { _handler: PWM2 },
        Vector { _handler: PWM3 },
        Vector { _reserved: 0 },
        Vector { _handler: PDM },
        Vector { _reserved: 0 },
        Vector { _handler: I2S },
        Vector { _reserved: 0 },
        Vector { _handler: IPC },
        Vector { _reserved: 0 },
        Vector { _handler: FPU },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: GPIOTE1 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: KMU },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector {
            _handler: CRYPTOCELL,
        },
    ];
}
#[doc = "Factory Information Configuration Registers"]
pub const FICR_S: ficr::Ficr = unsafe { ficr::Ficr::from_ptr(0x00ff_0000usize as _) };
#[doc = "User information configuration registers User information configuration registers"]
pub const UICR_S: uicr::Uicr = unsafe { uicr::Uicr::from_ptr(0x00ff_8000usize as _) };
#[doc = "Voltage regulators control 0"]
pub const REGULATORS_NS: regulators::Regulators =
    unsafe { regulators::Regulators::from_ptr(0x4000_4000usize as _) };
#[doc = "Clock management 0"]
pub const CLOCK_NS: clock::Clock = unsafe { clock::Clock::from_ptr(0x4000_5000usize as _) };
#[doc = "Power control 0"]
pub const POWER_NS: power::Power = unsafe { power::Power::from_ptr(0x4000_5000usize as _) };
#[doc = "Serial Peripheral Interface Master with EasyDMA 0"]
pub const SPIM0_NS: spim::Spim = unsafe { spim::Spim::from_ptr(0x4000_8000usize as _) };
#[doc = "SPI Slave 0"]
pub const SPIS0_NS: spis::Spis = unsafe { spis::Spis::from_ptr(0x4000_8000usize as _) };
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 0"]
pub const TWIM0_NS: twim::Twim = unsafe { twim::Twim::from_ptr(0x4000_8000usize as _) };
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 0"]
pub const TWIS0_NS: twis::Twis = unsafe { twis::Twis::from_ptr(0x4000_8000usize as _) };
#[doc = "UART with EasyDMA 0"]
pub const UARTE0_NS: uarte::Uarte = unsafe { uarte::Uarte::from_ptr(0x4000_8000usize as _) };
#[doc = "Serial Peripheral Interface Master with EasyDMA 2"]
pub const SPIM1_NS: spim::Spim = unsafe { spim::Spim::from_ptr(0x4000_9000usize as _) };
#[doc = "SPI Slave 2"]
pub const SPIS1_NS: spis::Spis = unsafe { spis::Spis::from_ptr(0x4000_9000usize as _) };
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 2"]
pub const TWIM1_NS: twim::Twim = unsafe { twim::Twim::from_ptr(0x4000_9000usize as _) };
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 2"]
pub const TWIS1_NS: twis::Twis = unsafe { twis::Twis::from_ptr(0x4000_9000usize as _) };
#[doc = "UART with EasyDMA 2"]
pub const UARTE1_NS: uarte::Uarte = unsafe { uarte::Uarte::from_ptr(0x4000_9000usize as _) };
#[doc = "Serial Peripheral Interface Master with EasyDMA 4"]
pub const SPIM2_NS: spim::Spim = unsafe { spim::Spim::from_ptr(0x4000_a000usize as _) };
#[doc = "SPI Slave 4"]
pub const SPIS2_NS: spis::Spis = unsafe { spis::Spis::from_ptr(0x4000_a000usize as _) };
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 4"]
pub const TWIM2_NS: twim::Twim = unsafe { twim::Twim::from_ptr(0x4000_a000usize as _) };
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 4"]
pub const TWIS2_NS: twis::Twis = unsafe { twis::Twis::from_ptr(0x4000_a000usize as _) };
#[doc = "UART with EasyDMA 4"]
pub const UARTE2_NS: uarte::Uarte = unsafe { uarte::Uarte::from_ptr(0x4000_a000usize as _) };
#[doc = "Serial Peripheral Interface Master with EasyDMA 6"]
pub const SPIM3_NS: spim::Spim = unsafe { spim::Spim::from_ptr(0x4000_b000usize as _) };
#[doc = "SPI Slave 6"]
pub const SPIS3_NS: spis::Spis = unsafe { spis::Spis::from_ptr(0x4000_b000usize as _) };
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 6"]
pub const TWIM3_NS: twim::Twim = unsafe { twim::Twim::from_ptr(0x4000_b000usize as _) };
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 6"]
pub const TWIS3_NS: twis::Twis = unsafe { twis::Twis::from_ptr(0x4000_b000usize as _) };
#[doc = "UART with EasyDMA 6"]
pub const UARTE3_NS: uarte::Uarte = unsafe { uarte::Uarte::from_ptr(0x4000_b000usize as _) };
#[doc = "Analog to Digital Converter 0"]
pub const SAADC_NS: saadc::Saadc = unsafe { saadc::Saadc::from_ptr(0x4000_e000usize as _) };
#[doc = "Timer/Counter 0"]
pub const TIMER0_NS: timer::Timer = unsafe { timer::Timer::from_ptr(0x4000_f000usize as _) };
#[doc = "Timer/Counter 2"]
pub const TIMER1_NS: timer::Timer = unsafe { timer::Timer::from_ptr(0x4001_0000usize as _) };
#[doc = "Timer/Counter 4"]
pub const TIMER2_NS: timer::Timer = unsafe { timer::Timer::from_ptr(0x4001_1000usize as _) };
#[doc = "Real-time counter 0"]
pub const RTC0_NS: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4001_4000usize as _) };
#[doc = "Real-time counter 2"]
pub const RTC1_NS: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4001_5000usize as _) };
#[doc = "Distributed programmable peripheral interconnect controller 0"]
pub const DPPIC_NS: dppic::Dppic = unsafe { dppic::Dppic::from_ptr(0x4001_7000usize as _) };
#[doc = "Watchdog Timer 0"]
pub const WDT_NS: wdt::Wdt = unsafe { wdt::Wdt::from_ptr(0x4001_8000usize as _) };
#[doc = "Event generator unit 0"]
pub const EGU0_NS: egu::Egu = unsafe { egu::Egu::from_ptr(0x4001_b000usize as _) };
#[doc = "Event generator unit 2"]
pub const EGU1_NS: egu::Egu = unsafe { egu::Egu::from_ptr(0x4001_c000usize as _) };
#[doc = "Event generator unit 4"]
pub const EGU2_NS: egu::Egu = unsafe { egu::Egu::from_ptr(0x4001_d000usize as _) };
#[doc = "Event generator unit 6"]
pub const EGU3_NS: egu::Egu = unsafe { egu::Egu::from_ptr(0x4001_e000usize as _) };
#[doc = "Event generator unit 8"]
pub const EGU4_NS: egu::Egu = unsafe { egu::Egu::from_ptr(0x4001_f000usize as _) };
#[doc = "Event generator unit 10"]
pub const EGU5_NS: egu::Egu = unsafe { egu::Egu::from_ptr(0x4002_0000usize as _) };
#[doc = "Pulse width modulation unit 0"]
pub const PWM0_NS: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x4002_1000usize as _) };
#[doc = "Pulse width modulation unit 2"]
pub const PWM1_NS: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x4002_2000usize as _) };
#[doc = "Pulse width modulation unit 4"]
pub const PWM2_NS: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x4002_3000usize as _) };
#[doc = "Pulse width modulation unit 6"]
pub const PWM3_NS: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x4002_4000usize as _) };
#[doc = "Pulse Density Modulation (Digital Microphone) Interface 0"]
pub const PDM_NS: pdm::Pdm = unsafe { pdm::Pdm::from_ptr(0x4002_6000usize as _) };
#[doc = "Inter-IC Sound 0"]
pub const I2S_NS: i2s::I2s = unsafe { i2s::I2s::from_ptr(0x4002_8000usize as _) };
#[doc = "Interprocessor communication 0"]
pub const IPC_NS: ipc::Ipc = unsafe { ipc::Ipc::from_ptr(0x4002_a000usize as _) };
#[doc = "FPU"]
pub const FPU_NS: fpu::Fpu = unsafe { fpu::Fpu::from_ptr(0x4002_c000usize as _) };
#[doc = "GPIO Tasks and Events 1"]
pub const GPIOTE1_NS: gpiote::Gpiote = unsafe { gpiote::Gpiote::from_ptr(0x4003_1000usize as _) };
#[doc = "Access Port Protection 0"]
pub const APPROTECT_NS: approtect::Approtect =
    unsafe { approtect::Approtect::from_ptr(0x4003_9000usize as _) };
#[doc = "Key management unit 0"]
pub const KMU_NS: kmu::Kmu = unsafe { kmu::Kmu::from_ptr(0x4003_9000usize as _) };
#[doc = "Non-volatile memory controller 0"]
pub const NVMC_NS: nvmc::Nvmc = unsafe { nvmc::Nvmc::from_ptr(0x4003_9000usize as _) };
#[doc = "Volatile Memory controller 0"]
pub const VMC_NS: vmc::Vmc = unsafe { vmc::Vmc::from_ptr(0x4003_a000usize as _) };
#[doc = "GPIO Port 0"]
pub const P0_NS: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4084_2500usize as _) };
#[doc = "System protection unit"]
pub const SPU_S: spu::Spu = unsafe { spu::Spu::from_ptr(0x5000_3000usize as _) };
#[doc = "Voltage regulators control 1"]
pub const REGULATORS_S: regulators::Regulators =
    unsafe { regulators::Regulators::from_ptr(0x5000_4000usize as _) };
#[doc = "Clock management 1"]
pub const CLOCK_S: clock::Clock = unsafe { clock::Clock::from_ptr(0x5000_5000usize as _) };
#[doc = "Power control 1"]
pub const POWER_S: power::Power = unsafe { power::Power::from_ptr(0x5000_5000usize as _) };
#[doc = "Control access port"]
pub const CTRL_AP_PERI_S: ctrlapperi::Ctrlapperi =
    unsafe { ctrlapperi::Ctrlapperi::from_ptr(0x5000_6000usize as _) };
#[doc = "Serial Peripheral Interface Master with EasyDMA 1"]
pub const SPIM0_S: spim::Spim = unsafe { spim::Spim::from_ptr(0x5000_8000usize as _) };
#[doc = "SPI Slave 1"]
pub const SPIS0_S: spis::Spis = unsafe { spis::Spis::from_ptr(0x5000_8000usize as _) };
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 1"]
pub const TWIM0_S: twim::Twim = unsafe { twim::Twim::from_ptr(0x5000_8000usize as _) };
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 1"]
pub const TWIS0_S: twis::Twis = unsafe { twis::Twis::from_ptr(0x5000_8000usize as _) };
#[doc = "UART with EasyDMA 1"]
pub const UARTE0_S: uarte::Uarte = unsafe { uarte::Uarte::from_ptr(0x5000_8000usize as _) };
#[doc = "Serial Peripheral Interface Master with EasyDMA 3"]
pub const SPIM1_S: spim::Spim = unsafe { spim::Spim::from_ptr(0x5000_9000usize as _) };
#[doc = "SPI Slave 3"]
pub const SPIS1_S: spis::Spis = unsafe { spis::Spis::from_ptr(0x5000_9000usize as _) };
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 3"]
pub const TWIM1_S: twim::Twim = unsafe { twim::Twim::from_ptr(0x5000_9000usize as _) };
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 3"]
pub const TWIS1_S: twis::Twis = unsafe { twis::Twis::from_ptr(0x5000_9000usize as _) };
#[doc = "UART with EasyDMA 3"]
pub const UARTE1_S: uarte::Uarte = unsafe { uarte::Uarte::from_ptr(0x5000_9000usize as _) };
#[doc = "Serial Peripheral Interface Master with EasyDMA 5"]
pub const SPIM2_S: spim::Spim = unsafe { spim::Spim::from_ptr(0x5000_a000usize as _) };
#[doc = "SPI Slave 5"]
pub const SPIS2_S: spis::Spis = unsafe { spis::Spis::from_ptr(0x5000_a000usize as _) };
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 5"]
pub const TWIM2_S: twim::Twim = unsafe { twim::Twim::from_ptr(0x5000_a000usize as _) };
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 5"]
pub const TWIS2_S: twis::Twis = unsafe { twis::Twis::from_ptr(0x5000_a000usize as _) };
#[doc = "UART with EasyDMA 5"]
pub const UARTE2_S: uarte::Uarte = unsafe { uarte::Uarte::from_ptr(0x5000_a000usize as _) };
#[doc = "Serial Peripheral Interface Master with EasyDMA 7"]
pub const SPIM3_S: spim::Spim = unsafe { spim::Spim::from_ptr(0x5000_b000usize as _) };
#[doc = "SPI Slave 7"]
pub const SPIS3_S: spis::Spis = unsafe { spis::Spis::from_ptr(0x5000_b000usize as _) };
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 7"]
pub const TWIM3_S: twim::Twim = unsafe { twim::Twim::from_ptr(0x5000_b000usize as _) };
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 7"]
pub const TWIS3_S: twis::Twis = unsafe { twis::Twis::from_ptr(0x5000_b000usize as _) };
#[doc = "UART with EasyDMA 7"]
pub const UARTE3_S: uarte::Uarte = unsafe { uarte::Uarte::from_ptr(0x5000_b000usize as _) };
#[doc = "GPIO Tasks and Events 0"]
pub const GPIOTE0_S: gpiote::Gpiote = unsafe { gpiote::Gpiote::from_ptr(0x5000_d000usize as _) };
#[doc = "Analog to Digital Converter 1"]
pub const SAADC_S: saadc::Saadc = unsafe { saadc::Saadc::from_ptr(0x5000_e000usize as _) };
#[doc = "Timer/Counter 1"]
pub const TIMER0_S: timer::Timer = unsafe { timer::Timer::from_ptr(0x5000_f000usize as _) };
#[doc = "Timer/Counter 3"]
pub const TIMER1_S: timer::Timer = unsafe { timer::Timer::from_ptr(0x5001_0000usize as _) };
#[doc = "Timer/Counter 5"]
pub const TIMER2_S: timer::Timer = unsafe { timer::Timer::from_ptr(0x5001_1000usize as _) };
#[doc = "Real-time counter 1"]
pub const RTC0_S: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x5001_4000usize as _) };
#[doc = "Real-time counter 3"]
pub const RTC1_S: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x5001_5000usize as _) };
#[doc = "Distributed programmable peripheral interconnect controller 1"]
pub const DPPIC_S: dppic::Dppic = unsafe { dppic::Dppic::from_ptr(0x5001_7000usize as _) };
#[doc = "Watchdog Timer 1"]
pub const WDT_S: wdt::Wdt = unsafe { wdt::Wdt::from_ptr(0x5001_8000usize as _) };
#[doc = "Event generator unit 1"]
pub const EGU0_S: egu::Egu = unsafe { egu::Egu::from_ptr(0x5001_b000usize as _) };
#[doc = "Event generator unit 3"]
pub const EGU1_S: egu::Egu = unsafe { egu::Egu::from_ptr(0x5001_c000usize as _) };
#[doc = "Event generator unit 5"]
pub const EGU2_S: egu::Egu = unsafe { egu::Egu::from_ptr(0x5001_d000usize as _) };
#[doc = "Event generator unit 7"]
pub const EGU3_S: egu::Egu = unsafe { egu::Egu::from_ptr(0x5001_e000usize as _) };
#[doc = "Event generator unit 9"]
pub const EGU4_S: egu::Egu = unsafe { egu::Egu::from_ptr(0x5001_f000usize as _) };
#[doc = "Event generator unit 11"]
pub const EGU5_S: egu::Egu = unsafe { egu::Egu::from_ptr(0x5002_0000usize as _) };
#[doc = "Pulse width modulation unit 1"]
pub const PWM0_S: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x5002_1000usize as _) };
#[doc = "Pulse width modulation unit 3"]
pub const PWM1_S: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x5002_2000usize as _) };
#[doc = "Pulse width modulation unit 5"]
pub const PWM2_S: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x5002_3000usize as _) };
#[doc = "Pulse width modulation unit 7"]
pub const PWM3_S: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x5002_4000usize as _) };
#[doc = "Pulse Density Modulation (Digital Microphone) Interface 1"]
pub const PDM_S: pdm::Pdm = unsafe { pdm::Pdm::from_ptr(0x5002_6000usize as _) };
#[doc = "Inter-IC Sound 1"]
pub const I2S_S: i2s::I2s = unsafe { i2s::I2s::from_ptr(0x5002_8000usize as _) };
#[doc = "Interprocessor communication 1"]
pub const IPC_S: ipc::Ipc = unsafe { ipc::Ipc::from_ptr(0x5002_a000usize as _) };
#[doc = "Access Port Protection 1"]
pub const APPROTECT_S: approtect::Approtect =
    unsafe { approtect::Approtect::from_ptr(0x5003_9000usize as _) };
#[doc = "Key management unit 1"]
pub const KMU_S: kmu::Kmu = unsafe { kmu::Kmu::from_ptr(0x5003_9000usize as _) };
#[doc = "Non-volatile memory controller 1"]
pub const NVMC_S: nvmc::Nvmc = unsafe { nvmc::Nvmc::from_ptr(0x5003_9000usize as _) };
#[doc = "Volatile Memory controller 1"]
pub const VMC_S: vmc::Vmc = unsafe { vmc::Vmc::from_ptr(0x5003_a000usize as _) };
#[doc = "CRYPTOCELL HOST_RGF interface"]
pub const CC_HOST_RGF_S: cc_host_rgf::CcHostRgf =
    unsafe { cc_host_rgf::CcHostRgf::from_ptr(0x5084_0000usize as _) };
#[doc = "ARM TrustZone CryptoCell register interface"]
pub const CRYPTOCELL_S: cryptocell::Cryptocell =
    unsafe { cryptocell::Cryptocell::from_ptr(0x5084_0000usize as _) };
#[doc = "GPIO Port 1"]
pub const P0_S: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5084_2500usize as _) };
#[doc = "Trace and debug control"]
pub const TAD_S: tad::Tad = unsafe { tad::Tad::from_ptr(0xe008_0000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub mod approtect {
    #[doc = "Access Port Protection 0"]
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
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn secureapprotect(self) -> Secureapprotect {
            unsafe { Secureapprotect::from_ptr(self.ptr.add(0x0e00usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn approtect(self) -> ApprotectApprotect {
            unsafe { ApprotectApprotect::from_ptr(self.ptr.add(0x0e10usize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ApprotectApprotect {
        ptr: *mut u8,
    }
    unsafe impl Send for ApprotectApprotect {}
    unsafe impl Sync for ApprotectApprotect {}
    impl ApprotectApprotect {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Software disable APPROTECT mechanism"]
        #[inline(always)]
        pub const fn disable(
            self,
        ) -> crate::common::Reg<regs::ApprotectDisable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Software force APPROTECT mechanism"]
        #[inline(always)]
        pub const fn forceprotect(
            self,
        ) -> crate::common::Reg<regs::ApprotectForceprotect, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secureapprotect {
        ptr: *mut u8,
    }
    unsafe impl Send for Secureapprotect {}
    unsafe impl Sync for Secureapprotect {}
    impl Secureapprotect {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Software disable SECUREAPPROTECT mechanism"]
        #[inline(always)]
        pub const fn disable(
            self,
        ) -> crate::common::Reg<regs::SecureapprotectDisable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Software force SECUREAPPROTECT mechanism"]
        #[inline(always)]
        pub const fn forceprotect(
            self,
        ) -> crate::common::Reg<regs::SecureapprotectForceprotect, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Software disable APPROTECT mechanism"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct ApprotectDisable(pub u32);
        impl ApprotectDisable {
            #[doc = "Software disable APPROTECT mechanism"]
            #[inline(always)]
            pub const fn disable(&self) -> super::vals::ApprotectDisableDisable {
                let val = (self.0 >> 0usize) & 0xff;
                super::vals::ApprotectDisableDisable::from_bits(val as u8)
            }
            #[doc = "Software disable APPROTECT mechanism"]
            #[inline(always)]
            pub fn set_disable(&mut self, val: super::vals::ApprotectDisableDisable) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
            }
        }
        impl Default for ApprotectDisable {
            #[inline(always)]
            fn default() -> ApprotectDisable {
                ApprotectDisable(0)
            }
        }
        impl core::fmt::Debug for ApprotectDisable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("ApprotectDisable")
                    .field("disable", &self.disable())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for ApprotectDisable {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct ApprotectDisable {
                    disable: super::vals::ApprotectDisableDisable,
                }
                let proxy = ApprotectDisable {
                    disable: self.disable(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Software force APPROTECT mechanism"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct ApprotectForceprotect(pub u32);
        impl ApprotectForceprotect {
            #[doc = "Write 0x1 to force enable APPROTECT mechanism"]
            #[inline(always)]
            pub const fn forceprotect(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Write 0x1 to force enable APPROTECT mechanism"]
            #[inline(always)]
            pub fn set_forceprotect(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
        }
        impl Default for ApprotectForceprotect {
            #[inline(always)]
            fn default() -> ApprotectForceprotect {
                ApprotectForceprotect(0)
            }
        }
        impl core::fmt::Debug for ApprotectForceprotect {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("ApprotectForceprotect")
                    .field("forceprotect", &self.forceprotect())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for ApprotectForceprotect {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct ApprotectForceprotect {
                    forceprotect: bool,
                }
                let proxy = ApprotectForceprotect {
                    forceprotect: self.forceprotect(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Software disable SECUREAPPROTECT mechanism"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SecureapprotectDisable(pub u32);
        impl SecureapprotectDisable {
            #[doc = "Software disable SECUREAPPROTECT mechanism"]
            #[inline(always)]
            pub const fn disable(&self) -> super::vals::SecureapprotectDisableDisable {
                let val = (self.0 >> 0usize) & 0xff;
                super::vals::SecureapprotectDisableDisable::from_bits(val as u8)
            }
            #[doc = "Software disable SECUREAPPROTECT mechanism"]
            #[inline(always)]
            pub fn set_disable(&mut self, val: super::vals::SecureapprotectDisableDisable) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
            }
        }
        impl Default for SecureapprotectDisable {
            #[inline(always)]
            fn default() -> SecureapprotectDisable {
                SecureapprotectDisable(0)
            }
        }
        impl core::fmt::Debug for SecureapprotectDisable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SecureapprotectDisable")
                    .field("disable", &self.disable())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SecureapprotectDisable {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct SecureapprotectDisable {
                    disable: super::vals::SecureapprotectDisableDisable,
                }
                let proxy = SecureapprotectDisable {
                    disable: self.disable(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Software force SECUREAPPROTECT mechanism"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SecureapprotectForceprotect(pub u32);
        impl SecureapprotectForceprotect {
            #[doc = "Write 0x1 to force enable SECUREAPPROTECT mechanism"]
            #[inline(always)]
            pub const fn forceprotect(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Write 0x1 to force enable SECUREAPPROTECT mechanism"]
            #[inline(always)]
            pub fn set_forceprotect(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
        }
        impl Default for SecureapprotectForceprotect {
            #[inline(always)]
            fn default() -> SecureapprotectForceprotect {
                SecureapprotectForceprotect(0)
            }
        }
        impl core::fmt::Debug for SecureapprotectForceprotect {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SecureapprotectForceprotect")
                    .field("forceprotect", &self.forceprotect())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SecureapprotectForceprotect {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct SecureapprotectForceprotect {
                    forceprotect: bool,
                }
                let proxy = SecureapprotectForceprotect {
                    forceprotect: self.forceprotect(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct ApprotectDisableDisable(pub u8);
        impl ApprotectDisableDisable {
            #[doc = "Software disable APPROTECT mechanism"]
            pub const SW_UNPROTECTED: Self = Self(0x5a);
        }
        impl ApprotectDisableDisable {
            pub const fn from_bits(val: u8) -> ApprotectDisableDisable {
                Self(val & 0xff)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl core::fmt::Debug for ApprotectDisableDisable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x5a => f.write_str("SW_UNPROTECTED"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for ApprotectDisableDisable {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x5a => defmt::write!(f, "SW_UNPROTECTED"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u8> for ApprotectDisableDisable {
            #[inline(always)]
            fn from(val: u8) -> ApprotectDisableDisable {
                ApprotectDisableDisable::from_bits(val)
            }
        }
        impl From<ApprotectDisableDisable> for u8 {
            #[inline(always)]
            fn from(val: ApprotectDisableDisable) -> u8 {
                ApprotectDisableDisable::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct SecureapprotectDisableDisable(pub u8);
        impl SecureapprotectDisableDisable {
            #[doc = "Software disable SECUREAPPROTECT mechanism"]
            pub const SW_UNPROTECTED: Self = Self(0x5a);
        }
        impl SecureapprotectDisableDisable {
            pub const fn from_bits(val: u8) -> SecureapprotectDisableDisable {
                Self(val & 0xff)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl core::fmt::Debug for SecureapprotectDisableDisable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x5a => f.write_str("SW_UNPROTECTED"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SecureapprotectDisableDisable {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x5a => defmt::write!(f, "SW_UNPROTECTED"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u8> for SecureapprotectDisableDisable {
            #[inline(always)]
            fn from(val: u8) -> SecureapprotectDisableDisable {
                SecureapprotectDisableDisable::from_bits(val)
            }
        }
        impl From<SecureapprotectDisableDisable> for u8 {
            #[inline(always)]
            fn from(val: SecureapprotectDisableDisable) -> u8 {
                SecureapprotectDisableDisable::to_bits(val)
            }
        }
    }
}
pub mod cc_host_rgf {
    #[doc = "CRYPTOCELL HOST_RGF interface"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcHostRgf {
        ptr: *mut u8,
    }
    unsafe impl Send for CcHostRgf {}
    unsafe impl Sync for CcHostRgf {}
    impl CcHostRgf {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "AES hardware key select"]
        #[inline(always)]
        pub const fn host_cryptokey_sel(
            self,
        ) -> crate::common::Reg<regs::HostCryptokeySel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1a38usize) as _) }
        }
        #[doc = "This write-once register is the K_PRTL lock register. When this register is set, K_PRTL cannot be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain."]
        #[inline(always)]
        pub const fn host_iot_kprtl_lock(
            self,
        ) -> crate::common::Reg<regs::HostIotKprtlLock, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1a4cusize) as _) }
        }
        #[doc = "This register holds bits 31:0 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain. Reading from this address returns the K_DR valid status indicating if K_DR is successfully retained."]
        #[inline(always)]
        pub const fn host_iot_kdr0(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1a50usize) as _) }
        }
        #[doc = "This register holds bits 63:32 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
        #[inline(always)]
        pub const fn host_iot_kdr1(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1a54usize) as _) }
        }
        #[doc = "This register holds bits 95:64 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
        #[inline(always)]
        pub const fn host_iot_kdr2(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1a58usize) as _) }
        }
        #[doc = "This register holds bits 127:96 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
        #[inline(always)]
        pub const fn host_iot_kdr3(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1a5cusize) as _) }
        }
        #[doc = "Controls lifecycle state (LCS) for CRYPTOCELL subsystem"]
        #[inline(always)]
        pub const fn host_iot_lcs(self) -> crate::common::Reg<regs::HostIotLcs, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1a60usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "AES hardware key select"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct HostCryptokeySel(pub u32);
        impl HostCryptokeySel {
            #[doc = "Select the source of the HW key that is used by the AES engine"]
            #[inline(always)]
            pub const fn host_cryptokey_sel(&self) -> super::vals::HostCryptokeySel {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::HostCryptokeySel::from_bits(val as u8)
            }
            #[doc = "Select the source of the HW key that is used by the AES engine"]
            #[inline(always)]
            pub fn set_host_cryptokey_sel(&mut self, val: super::vals::HostCryptokeySel) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for HostCryptokeySel {
            #[inline(always)]
            fn default() -> HostCryptokeySel {
                HostCryptokeySel(0)
            }
        }
        impl core::fmt::Debug for HostCryptokeySel {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("HostCryptokeySel")
                    .field("host_cryptokey_sel", &self.host_cryptokey_sel())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for HostCryptokeySel {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct HostCryptokeySel {
                    host_cryptokey_sel: super::vals::HostCryptokeySel,
                }
                let proxy = HostCryptokeySel {
                    host_cryptokey_sel: self.host_cryptokey_sel(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "This write-once register is the K_PRTL lock register. When this register is set, K_PRTL cannot be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct HostIotKprtlLock(pub u32);
        impl HostIotKprtlLock {
            #[doc = "This register is the K_PRTL lock register. When this register is set, K_PRTL cannot be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain."]
            #[inline(always)]
            pub const fn host_iot_kprtl_lock(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This register is the K_PRTL lock register. When this register is set, K_PRTL cannot be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain."]
            #[inline(always)]
            pub fn set_host_iot_kprtl_lock(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for HostIotKprtlLock {
            #[inline(always)]
            fn default() -> HostIotKprtlLock {
                HostIotKprtlLock(0)
            }
        }
        impl core::fmt::Debug for HostIotKprtlLock {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("HostIotKprtlLock")
                    .field("host_iot_kprtl_lock", &self.host_iot_kprtl_lock())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for HostIotKprtlLock {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct HostIotKprtlLock {
                    host_iot_kprtl_lock: bool,
                }
                let proxy = HostIotKprtlLock {
                    host_iot_kprtl_lock: self.host_iot_kprtl_lock(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Controls lifecycle state (LCS) for CRYPTOCELL subsystem"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct HostIotLcs(pub u32);
        impl HostIotLcs {
            #[doc = "Lifecycle state value. This field is write-once per reset."]
            #[inline(always)]
            pub const fn lcs(&self) -> super::vals::Lcs {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::Lcs::from_bits(val as u8)
            }
            #[doc = "Lifecycle state value. This field is write-once per reset."]
            #[inline(always)]
            pub fn set_lcs(&mut self, val: super::vals::Lcs) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
            }
            #[doc = "Read-only field. Indicates if CRYPTOCELL LCS has been successfully configured since last reset."]
            #[inline(always)]
            pub const fn lcs_is_valid(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Read-only field. Indicates if CRYPTOCELL LCS has been successfully configured since last reset."]
            #[inline(always)]
            pub fn set_lcs_is_valid(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for HostIotLcs {
            #[inline(always)]
            fn default() -> HostIotLcs {
                HostIotLcs(0)
            }
        }
        impl core::fmt::Debug for HostIotLcs {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("HostIotLcs")
                    .field("lcs", &self.lcs())
                    .field("lcs_is_valid", &self.lcs_is_valid())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for HostIotLcs {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct HostIotLcs {
                    lcs: super::vals::Lcs,
                    lcs_is_valid: bool,
                }
                let proxy = HostIotLcs {
                    lcs: self.lcs(),
                    lcs_is_valid: self.lcs_is_valid(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum HostCryptokeySel {
            #[doc = "Use device root key K_DR from CRYPTOCELL AO power domain"]
            K_DR = 0x0,
            #[doc = "Use hard-coded RTL key K_PRTL"]
            K_PRTL = 0x01,
            #[doc = "Use provided session key"]
            SESSION = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl HostCryptokeySel {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> HostCryptokeySel {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for HostCryptokeySel {
            #[inline(always)]
            fn from(val: u8) -> HostCryptokeySel {
                HostCryptokeySel::from_bits(val)
            }
        }
        impl From<HostCryptokeySel> for u8 {
            #[inline(always)]
            fn from(val: HostCryptokeySel) -> u8 {
                HostCryptokeySel::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Lcs {
            #[doc = "CC310 operates in debug mode"]
            DEBUG = 0x0,
            _RESERVED_1 = 0x01,
            #[doc = "CC310 operates in secure mode"]
            SECURE = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl Lcs {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Lcs {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Lcs {
            #[inline(always)]
            fn from(val: u8) -> Lcs {
                Lcs::from_bits(val)
            }
        }
        impl From<Lcs> for u8 {
            #[inline(always)]
            fn from(val: Lcs) -> u8 {
                Lcs::to_bits(val)
            }
        }
    }
}
pub mod clock {
    #[doc = "Clock management 0"]
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
        #[doc = "Start HFCLK source"]
        #[inline(always)]
        pub const fn tasks_hfclkstart(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop HFCLK source"]
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
        #[doc = "Subscribe configuration for task HFCLKSTART"]
        #[inline(always)]
        pub const fn subscribe_hfclkstart(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
        }
        #[doc = "Subscribe configuration for task HFCLKSTOP"]
        #[inline(always)]
        pub const fn subscribe_hfclkstop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
        }
        #[doc = "Subscribe configuration for task LFCLKSTART"]
        #[inline(always)]
        pub const fn subscribe_lfclkstart(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
        }
        #[doc = "Subscribe configuration for task LFCLKSTOP"]
        #[inline(always)]
        pub const fn subscribe_lfclkstop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
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
        #[doc = "Publish configuration for event HFCLKSTARTED"]
        #[inline(always)]
        pub const fn publish_hfclkstarted(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
        }
        #[doc = "Publish configuration for event LFCLKSTARTED"]
        #[inline(always)]
        pub const fn publish_lfclkstarted(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Pending interrupts"]
        #[inline(always)]
        pub const fn intpend(self) -> crate::common::Reg<regs::Int, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x030cusize) as _) }
        }
        #[doc = "Status indicating that HFCLKSTART task has been triggered"]
        #[inline(always)]
        pub const fn hfclkrun(self) -> crate::common::Reg<regs::Hfclkrun, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0408usize) as _) }
        }
        #[doc = "The register shows if HFXO has been requested by triggering HFCLKSTART task and if it has been started (STATE)"]
        #[inline(always)]
        pub const fn hfclkstat(self) -> crate::common::Reg<regs::Hfclkstat, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x040cusize) as _) }
        }
        #[doc = "Status indicating that LFCLKSTART task has been triggered"]
        #[inline(always)]
        pub const fn lfclkrun(self) -> crate::common::Reg<regs::Lfclkrun, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0414usize) as _) }
        }
        #[doc = "The register shows which LFCLK source has been requested (SRC) when triggering LFCLKSTART task and if the source has been started (STATE)"]
        #[inline(always)]
        pub const fn lfclkstat(self) -> crate::common::Reg<regs::Lfclkstat, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0418usize) as _) }
        }
        #[doc = "Copy of LFCLKSRC register, set after LFCLKSTART task has been triggered"]
        #[inline(always)]
        pub const fn lfclksrccopy(
            self,
        ) -> crate::common::Reg<regs::Lfclksrccopy, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x041cusize) as _) }
        }
        #[doc = "Clock source for the LFCLK. LFCLKSTART task starts starts a clock source selected with this register."]
        #[inline(always)]
        pub const fn lfclksrc(self) -> crate::common::Reg<regs::Lfclksrc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
    }
    pub mod regs {
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
                #[derive(defmt :: Format)]
                struct Hfclkrun {
                    status: bool,
                }
                let proxy = Hfclkrun {
                    status: self.status(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "The register shows if HFXO has been requested by triggering HFCLKSTART task and if it has been started (STATE)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Hfclkstat(pub u32);
        impl Hfclkstat {
            #[doc = "Active clock source"]
            #[inline(always)]
            pub const fn src(&self) -> super::vals::HfclkstatSrc {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::HfclkstatSrc::from_bits(val as u8)
            }
            #[doc = "Active clock source"]
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
                #[derive(defmt :: Format)]
                struct Hfclkstat {
                    src: super::vals::HfclkstatSrc,
                    state: bool,
                }
                let proxy = Hfclkstat {
                    src: self.src(),
                    state: self.state(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Enable or disable interrupt for event HFCLKSTARTED"]
            #[inline(always)]
            pub const fn hfclkstarted(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event HFCLKSTARTED"]
            #[inline(always)]
            pub fn set_hfclkstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable interrupt for event LFCLKSTARTED"]
            #[inline(always)]
            pub const fn lfclkstarted(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event LFCLKSTARTED"]
            #[inline(always)]
            pub fn set_lfclkstarted(&mut self, val: bool) {
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
                    .field("hfclkstarted", &self.hfclkstarted())
                    .field("lfclkstarted", &self.lfclkstarted())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    hfclkstarted: bool,
                    lfclkstarted: bool,
                }
                let proxy = Int {
                    hfclkstarted: self.hfclkstarted(),
                    lfclkstarted: self.lfclkstarted(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Lfclkrun {
                    status: bool,
                }
                let proxy = Lfclkrun {
                    status: self.status(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Clock source for the LFCLK. LFCLKSTART task starts starts a clock source selected with this register."]
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
                #[derive(defmt :: Format)]
                struct Lfclksrc {
                    src: super::vals::Lfclksrc,
                }
                let proxy = Lfclksrc { src: self.src() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Copy of LFCLKSRC register, set after LFCLKSTART task has been triggered"]
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
                #[derive(defmt :: Format)]
                struct Lfclksrccopy {
                    src: super::vals::Lfclksrc,
                }
                let proxy = Lfclksrccopy { src: self.src() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "The register shows which LFCLK source has been requested (SRC) when triggering LFCLKSTART task and if the source has been started (STATE)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Lfclkstat(pub u32);
        impl Lfclkstat {
            #[doc = "Active clock source"]
            #[inline(always)]
            pub const fn src(&self) -> super::vals::Lfclksrc {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Lfclksrc::from_bits(val as u8)
            }
            #[doc = "Active clock source"]
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
                #[derive(defmt :: Format)]
                struct Lfclkstat {
                    src: super::vals::Lfclksrc,
                    state: bool,
                }
                let proxy = Lfclkstat {
                    src: self.src(),
                    state: self.state(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum HfclkstatSrc {
            #[doc = "HFINT - 64 MHz on-chip oscillator"]
            HFINT = 0x0,
            #[doc = "HFXO - 64 MHz clock derived from external 32 MHz crystal oscillator"]
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
        pub enum Lfclksrc {
            #[doc = "Reserved for future use (equals selecting LFRC)"]
            RFU = 0x0,
            #[doc = "32.768 kHz RC oscillator"]
            LFRC = 0x01,
            #[doc = "32.768 kHz crystal oscillator"]
            LFXO = 0x02,
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
pub mod cryptocell {
    #[doc = "ARM TrustZone CryptoCell register interface"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cryptocell {
        ptr: *mut u8,
    }
    unsafe impl Send for Cryptocell {}
    unsafe impl Sync for Cryptocell {}
    impl Cryptocell {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Enable CRYPTOCELL subsystem"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Enable CRYPTOCELL subsystem"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable the CRYPTOCELL subsystem"]
            #[inline(always)]
            pub const fn enable(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable the CRYPTOCELL subsystem"]
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
                #[derive(defmt :: Format)]
                struct Enable {
                    enable: bool,
                }
                let proxy = Enable {
                    enable: self.enable(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
}
pub mod ctrlapperi {
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
            unsafe { Mailbox::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn eraseprotect(self) -> Eraseprotect {
            unsafe { Eraseprotect::from_ptr(self.ptr.add(0x0500usize) as _) }
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
        pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "This register disables the ERASEPROTECT register and performs an ERASEALL operation."]
        #[inline(always)]
        pub const fn disable(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "This register shows a status that indicates if data sent from the debugger to the CPU has been read."]
        #[inline(always)]
        pub const fn rxstatus(self) -> crate::common::Reg<regs::Rxstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Data sent from the CPU to the debugger."]
        #[inline(always)]
        pub const fn txdata(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
        }
        #[doc = "This register shows a status that indicates if the data sent from the CPU to the debugger has been read."]
        #[inline(always)]
        pub const fn txstatus(self) -> crate::common::Reg<regs::Txstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "This register locks the ERASEPROTECT.DISABLE register from being written until next reset."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Lock(pub u32);
        impl Lock {
            #[doc = "Lock ERASEPROTECT.DISABLE register from being written until next reset"]
            #[inline(always)]
            pub const fn lock(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Lock ERASEPROTECT.DISABLE register from being written until next reset"]
            #[inline(always)]
            pub fn set_lock(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Lock {
            #[inline(always)]
            fn default() -> Lock {
                Lock(0)
            }
        }
        impl core::fmt::Debug for Lock {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Lock").field("lock", &self.lock()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Lock {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Lock {
                    lock: bool,
                }
                let proxy = Lock { lock: self.lock() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "This register shows a status that indicates if data sent from the debugger to the CPU has been read."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxstatus(pub u32);
        impl Rxstatus {
            #[doc = "Status of data in register RXDATA"]
            #[inline(always)]
            pub const fn rxstatus(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Status of data in register RXDATA"]
            #[inline(always)]
            pub fn set_rxstatus(&mut self, val: bool) {
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
                #[derive(defmt :: Format)]
                struct Rxstatus {
                    rxstatus: bool,
                }
                let proxy = Rxstatus {
                    rxstatus: self.rxstatus(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "This register shows a status that indicates if the data sent from the CPU to the debugger has been read."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Txstatus(pub u32);
        impl Txstatus {
            #[doc = "Status of data in register TXDATA"]
            #[inline(always)]
            pub const fn txstatus(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Status of data in register TXDATA"]
            #[inline(always)]
            pub fn set_txstatus(&mut self, val: bool) {
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
                #[derive(defmt :: Format)]
                struct Txstatus {
                    txstatus: bool,
                }
                let proxy = Txstatus {
                    txstatus: self.txstatus(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
}
pub mod dppic {
    #[doc = "Distributed programmable peripheral interconnect controller 0"]
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
            unsafe { TasksChg::from_ptr(self.ptr.add(0x0usize + n * 8usize) as _) }
        }
        #[doc = "Subscribe configuration for tasks"]
        #[inline(always)]
        pub const fn subscribe_chg(self, n: usize) -> SubscribeChg {
            assert!(n < 6usize);
            unsafe { SubscribeChg::from_ptr(self.ptr.add(0x80usize + n * 8usize) as _) }
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
        #[doc = "Description collection: Channel group n Note: Writes to this register are ignored if either SUBSCRIBE_CHG\\[n\\].EN or SUBSCRIBE_CHG\\[n\\].DIS is enabled"]
        #[inline(always)]
        pub const fn chg(self, n: usize) -> crate::common::Reg<regs::Chg, crate::common::RW> {
            assert!(n < 6usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize + n * 4usize) as _) }
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster: Subscribe configuration for task CHG\\[n\\].DIS"]
        #[inline(always)]
        pub const fn dis(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
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
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable channel 0"]
            #[inline(always)]
            pub fn set_ch(&mut self, n: usize, val: bool) {
                assert!(n < 16usize);
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
                    .field(
                        "ch",
                        &[
                            self.ch(0usize),
                            self.ch(1usize),
                            self.ch(2usize),
                            self.ch(3usize),
                            self.ch(4usize),
                            self.ch(5usize),
                            self.ch(6usize),
                            self.ch(7usize),
                            self.ch(8usize),
                            self.ch(9usize),
                            self.ch(10usize),
                            self.ch(11usize),
                            self.ch(12usize),
                            self.ch(13usize),
                            self.ch(14usize),
                            self.ch(15usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Chen {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Chen {
                    ch: [bool; 16usize],
                }
                let proxy = Chen {
                    ch: [
                        self.ch(0usize),
                        self.ch(1usize),
                        self.ch(2usize),
                        self.ch(3usize),
                        self.ch(4usize),
                        self.ch(5usize),
                        self.ch(6usize),
                        self.ch(7usize),
                        self.ch(8usize),
                        self.ch(9usize),
                        self.ch(10usize),
                        self.ch(11usize),
                        self.ch(12usize),
                        self.ch(13usize),
                        self.ch(14usize),
                        self.ch(15usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description collection: Channel group n Note: Writes to this register are ignored if either SUBSCRIBE_CHG\\[n\\].EN or SUBSCRIBE_CHG\\[n\\].DIS is enabled"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Chg(pub u32);
        impl Chg {
            #[doc = "Include or exclude channel 0"]
            #[inline(always)]
            pub const fn ch(&self, n: usize) -> bool {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Include or exclude channel 0"]
            #[inline(always)]
            pub fn set_ch(&mut self, n: usize, val: bool) {
                assert!(n < 16usize);
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
                    .field(
                        "ch",
                        &[
                            self.ch(0usize),
                            self.ch(1usize),
                            self.ch(2usize),
                            self.ch(3usize),
                            self.ch(4usize),
                            self.ch(5usize),
                            self.ch(6usize),
                            self.ch(7usize),
                            self.ch(8usize),
                            self.ch(9usize),
                            self.ch(10usize),
                            self.ch(11usize),
                            self.ch(12usize),
                            self.ch(13usize),
                            self.ch(14usize),
                            self.ch(15usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Chg {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Chg {
                    ch: [bool; 16usize],
                }
                let proxy = Chg {
                    ch: [
                        self.ch(0usize),
                        self.ch(1usize),
                        self.ch(2usize),
                        self.ch(3usize),
                        self.ch(4usize),
                        self.ch(5usize),
                        self.ch(6usize),
                        self.ch(7usize),
                        self.ch(8usize),
                        self.ch(9usize),
                        self.ch(10usize),
                        self.ch(11usize),
                        self.ch(12usize),
                        self.ch(13usize),
                        self.ch(14usize),
                        self.ch(15usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
}
pub mod egu {
    #[doc = "Event generator unit 0"]
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
        #[doc = "Description collection: Subscribe configuration for task TRIGGER\\[n\\]"]
        #[inline(always)]
        pub const fn subscribe_trigger(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            assert!(n < 16usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
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
        #[doc = "Description collection: Publish configuration for event TRIGGERED\\[n\\]"]
        #[inline(always)]
        pub const fn publish_triggered(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            assert!(n < 16usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize + n * 4usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
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
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field(
                        "triggered",
                        &[
                            self.triggered(0usize),
                            self.triggered(1usize),
                            self.triggered(2usize),
                            self.triggered(3usize),
                            self.triggered(4usize),
                            self.triggered(5usize),
                            self.triggered(6usize),
                            self.triggered(7usize),
                            self.triggered(8usize),
                            self.triggered(9usize),
                            self.triggered(10usize),
                            self.triggered(11usize),
                            self.triggered(12usize),
                            self.triggered(13usize),
                            self.triggered(14usize),
                            self.triggered(15usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    triggered: [bool; 16usize],
                }
                let proxy = Int {
                    triggered: [
                        self.triggered(0usize),
                        self.triggered(1usize),
                        self.triggered(2usize),
                        self.triggered(3usize),
                        self.triggered(4usize),
                        self.triggered(5usize),
                        self.triggered(6usize),
                        self.triggered(7usize),
                        self.triggered(8usize),
                        self.triggered(9usize),
                        self.triggered(10usize),
                        self.triggered(11usize),
                        self.triggered(12usize),
                        self.triggered(13usize),
                        self.triggered(14usize),
                        self.triggered(15usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
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
        #[doc = "SIP-specific device info"]
        #[inline(always)]
        pub const fn sipinfo(self) -> Sipinfo {
            unsafe { Sipinfo::from_ptr(self.ptr.add(0x0140usize) as _) }
        }
        #[doc = "Device info"]
        #[inline(always)]
        pub const fn info(self) -> Info {
            unsafe { Info::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn trimcnf(self, n: usize) -> Trimcnf {
            assert!(n < 256usize);
            unsafe { Trimcnf::from_ptr(self.ptr.add(0x0300usize + n * 8usize) as _) }
        }
        #[doc = "NIST800-90B RNG calibration data"]
        #[inline(always)]
        pub const fn trng90b(self) -> Trng90b {
            unsafe { Trng90b::from_ptr(self.ptr.add(0x0c00usize) as _) }
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
        #[doc = "Description collection: Device identifier"]
        #[inline(always)]
        pub const fn deviceid(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
            assert!(n < 2usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize + n * 4usize) as _) }
        }
        #[doc = "Part code"]
        #[inline(always)]
        pub const fn part(self) -> crate::common::Reg<regs::Part, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Part Variant, Hardware version and Production configuration"]
        #[inline(always)]
        pub const fn variant(self) -> crate::common::Reg<regs::InfoVariant, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Package option"]
        #[inline(always)]
        pub const fn package(self) -> crate::common::Reg<regs::Package, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "RAM variant"]
        #[inline(always)]
        pub const fn ram(self) -> crate::common::Reg<regs::Ram, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }
        #[doc = "Flash variant"]
        #[inline(always)]
        pub const fn flash(self) -> crate::common::Reg<regs::Flash, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "Code memory page size"]
        #[inline(always)]
        pub const fn codepagesize(
            self,
        ) -> crate::common::Reg<regs::Codepagesize, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "Code memory size"]
        #[inline(always)]
        pub const fn codesize(self) -> crate::common::Reg<regs::Codesize, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
        }
        #[doc = "Device type"]
        #[inline(always)]
        pub const fn devicetype(self) -> crate::common::Reg<regs::Devicetype, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
        }
    }
    #[doc = "SIP-specific device info"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sipinfo {
        ptr: *mut u8,
    }
    unsafe impl Send for Sipinfo {}
    unsafe impl Sync for Sipinfo {}
    impl Sipinfo {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "SIP part number"]
        #[inline(always)]
        pub const fn partno(self) -> crate::common::Reg<regs::Partno, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description collection: SIP hardware revision, encoded in ASCII, ex B0A or B1A"]
        #[inline(always)]
        pub const fn hwrevision(self, n: usize) -> crate::common::Reg<u8, crate::common::R> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize + n * 1usize) as _) }
        }
        #[doc = "Description collection: SIP VARIANT, encoded in ASCII, ex SIAA, SIBA or SICA"]
        #[inline(always)]
        pub const fn variant(self, n: usize) -> crate::common::Reg<u8, crate::common::R> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 1usize) as _) }
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster: Data"]
        #[inline(always)]
        pub const fn data(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    #[doc = "NIST800-90B RNG calibration data"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trng90b {
        ptr: *mut u8,
    }
    unsafe impl Send for Trng90b {}
    unsafe impl Sync for Trng90b {}
    impl Trng90b {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Amount of bytes for the required entropy bits"]
        #[inline(always)]
        pub const fn bytes(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Repetition counter cutoff"]
        #[inline(always)]
        pub const fn rccutoff(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Adaptive proportion cutoff"]
        #[inline(always)]
        pub const fn apcutoff(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Amount of bytes for the startup tests"]
        #[inline(always)]
        pub const fn startup(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Sample count for ring oscillator 1"]
        #[inline(always)]
        pub const fn rosc1(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Sample count for ring oscillator 2"]
        #[inline(always)]
        pub const fn rosc2(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Sample count for ring oscillator 3"]
        #[inline(always)]
        pub const fn rosc3(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }
        #[doc = "Sample count for ring oscillator 4"]
        #[inline(always)]
        pub const fn rosc4(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Code memory page size"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Codepagesize(pub u32);
        impl Codepagesize {
            #[doc = "Code memory page size"]
            #[inline(always)]
            pub const fn codepagesize(&self) -> super::vals::Codepagesize {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Codepagesize::from_bits(val as u32)
            }
            #[doc = "Code memory page size"]
            #[inline(always)]
            pub fn set_codepagesize(&mut self, val: super::vals::Codepagesize) {
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
                #[derive(defmt :: Format)]
                struct Codepagesize {
                    codepagesize: super::vals::Codepagesize,
                }
                let proxy = Codepagesize {
                    codepagesize: self.codepagesize(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Code memory size"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Codesize(pub u32);
        impl Codesize {
            #[doc = "Code memory size in number of pages Total code space is: CODEPAGESIZE * CODESIZE"]
            #[inline(always)]
            pub const fn codesize(&self) -> super::vals::Codesize {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Codesize::from_bits(val as u32)
            }
            #[doc = "Code memory size in number of pages Total code space is: CODEPAGESIZE * CODESIZE"]
            #[inline(always)]
            pub fn set_codesize(&mut self, val: super::vals::Codesize) {
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
                #[derive(defmt :: Format)]
                struct Codesize {
                    codesize: super::vals::Codesize,
                }
                let proxy = Codesize {
                    codesize: self.codesize(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Device type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Devicetype(pub u32);
        impl Devicetype {
            #[doc = "Device type"]
            #[inline(always)]
            pub const fn devicetype(&self) -> super::vals::Devicetype {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Devicetype::from_bits(val as u32)
            }
            #[doc = "Device type"]
            #[inline(always)]
            pub fn set_devicetype(&mut self, val: super::vals::Devicetype) {
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
                #[derive(defmt :: Format)]
                struct Devicetype {
                    devicetype: super::vals::Devicetype,
                }
                let proxy = Devicetype {
                    devicetype: self.devicetype(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Flash {
                    flash: super::vals::Flash,
                }
                let proxy = Flash {
                    flash: self.flash(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Part Variant, Hardware version and Production configuration"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct InfoVariant(pub u32);
        impl InfoVariant {
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
        impl Default for InfoVariant {
            #[inline(always)]
            fn default() -> InfoVariant {
                InfoVariant(0)
            }
        }
        impl core::fmt::Debug for InfoVariant {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("InfoVariant")
                    .field("variant", &self.variant())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for InfoVariant {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct InfoVariant {
                    variant: super::vals::Variant,
                }
                let proxy = InfoVariant {
                    variant: self.variant(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Package {
                    package: super::vals::Package,
                }
                let proxy = Package {
                    package: self.package(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Part {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Part").field("part", &self.part()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Part {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Part {
                    part: super::vals::Part,
                }
                let proxy = Part { part: self.part() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "SIP part number"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Partno(pub u32);
        impl Partno {
            #[inline(always)]
            pub const fn partno(&self) -> super::vals::Partno {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Partno::from_bits(val as u32)
            }
            #[inline(always)]
            pub fn set_partno(&mut self, val: super::vals::Partno) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Partno {
            #[inline(always)]
            fn default() -> Partno {
                Partno(0)
            }
        }
        impl core::fmt::Debug for Partno {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Partno")
                    .field("partno", &self.partno())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Partno {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Partno {
                    partno: super::vals::Partno,
                }
                let proxy = Partno {
                    partno: self.partno(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Ram {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ram").field("ram", &self.ram()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ram {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Ram {
                    ram: super::vals::Ram,
                }
                let proxy = Ram { ram: self.ram() };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Codepagesize(pub u32);
        impl Codepagesize {
            #[doc = "4 kByte"]
            pub const K4096: Self = Self(0x1000);
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
                    0x1000 => f.write_str("K4096"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Codepagesize {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x1000 => defmt::write!(f, "K4096"),
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
        pub struct Codesize(pub u32);
        impl Codesize {
            #[doc = "256 pages"]
            pub const P256: Self = Self(0x0100);
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
                    0x0100 => f.write_str("P256"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Codesize {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0100 => defmt::write!(f, "P256"),
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
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Devicetype(pub u32);
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
        pub struct Flash(pub u32);
        impl Flash {
            #[doc = "1 MByte FLASH"]
            pub const K1024: Self = Self(0x0400);
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
                    0x0400 => f.write_str("K1024"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Flash {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0400 => defmt::write!(f, "K1024"),
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
        pub struct Package(pub u32);
        impl Package {
            #[doc = "CFxx - 236 ball wlCSP"]
            pub const CF: Self = Self(0x2002);
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
                    0x2002 => f.write_str("CF"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Package {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x2002 => defmt::write!(f, "CF"),
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
        pub struct Part(pub u32);
        impl Part {
            #[doc = "nRF9120"]
            pub const N9120: Self = Self(0x9120);
            #[doc = "nRF9160"]
            pub const N9160: Self = Self(0x9160);
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
                    0x9120 => f.write_str("N9120"),
                    0x9160 => f.write_str("N9160"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Part {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x9120 => defmt::write!(f, "N9120"),
                    0x9160 => defmt::write!(f, "N9160"),
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
        pub struct Partno(pub u32);
        impl Partno {
            #[doc = "Device is an nRF9160 sip"]
            pub const _9160: Self = Self(0x9160);
        }
        impl Partno {
            pub const fn from_bits(val: u32) -> Partno {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Partno {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x9160 => f.write_str("_9160"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Partno {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x9160 => defmt::write!(f, "_9160"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for Partno {
            #[inline(always)]
            fn from(val: u32) -> Partno {
                Partno::from_bits(val)
            }
        }
        impl From<Partno> for u32 {
            #[inline(always)]
            fn from(val: Partno) -> u32 {
                Partno::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ram(pub u32);
        impl Ram {
            #[doc = "256 kByte RAM"]
            pub const K256: Self = Self(0x0100);
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
                    0x0100 => f.write_str("K256"),
                    0xffff_ffff => f.write_str("UNSPECIFIED"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ram {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0100 => defmt::write!(f, "K256"),
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
        pub struct Variant(pub u32);
        impl Variant {
            #[doc = "AAA0"]
            pub const AAA0: Self = Self(0x4141_4130);
            #[doc = "AAAA"]
            pub const AAAA: Self = Self(0x4141_4141);
            #[doc = "AAB0"]
            pub const AAB0: Self = Self(0x4141_4230);
            #[doc = "AAC0"]
            pub const AAC0: Self = Self(0x4141_4330);
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
                    0x4141_4130 => f.write_str("AAA0"),
                    0x4141_4141 => f.write_str("AAAA"),
                    0x4141_4230 => f.write_str("AAB0"),
                    0x4141_4330 => f.write_str("AAC0"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Variant {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x4141_4130 => defmt::write!(f, "AAA0"),
                    0x4141_4141 => defmt::write!(f, "AAAA"),
                    0x4141_4230 => defmt::write!(f, "AAB0"),
                    0x4141_4330 => defmt::write!(f, "AAC0"),
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Set individual bits in GPIO port"]
        #[inline(always)]
        pub const fn outset(self) -> crate::common::Reg<regs::Outset, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Clear individual bits in GPIO port"]
        #[inline(always)]
        pub const fn outclr(self) -> crate::common::Reg<regs::Outclr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Read GPIO port"]
        #[inline(always)]
        pub const fn in_(self) -> crate::common::Reg<regs::In, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Direction of GPIO pins"]
        #[inline(always)]
        pub const fn dir(self) -> crate::common::Reg<regs::Dir, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "DIR set register"]
        #[inline(always)]
        pub const fn dirset(self) -> crate::common::Reg<regs::Dirset, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }
        #[doc = "DIR clear register"]
        #[inline(always)]
        pub const fn dirclr(self) -> crate::common::Reg<regs::Dirclr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF\\[n\\].SENSE registers"]
        #[inline(always)]
        pub const fn latch(self) -> crate::common::Reg<regs::Latch, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "Select between default DETECT signal behavior and LDETECT mode (For non-secure pin only)"]
        #[inline(always)]
        pub const fn detectmode(self) -> crate::common::Reg<regs::Detectmode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
        }
        #[doc = "Select between default DETECT signal behavior and LDETECT mode (For secure pin only)"]
        #[inline(always)]
        pub const fn detectmode_sec(
            self,
        ) -> crate::common::Reg<regs::DetectmodeSec, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
        }
        #[doc = "Description collection: Configuration of GPIO pins"]
        #[inline(always)]
        pub const fn pin_cnf(
            self,
            n: usize,
        ) -> crate::common::Reg<regs::PinCnf, crate::common::RW> {
            assert!(n < 32usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize + n * 4usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Select between default DETECT signal behavior and LDETECT mode (For non-secure pin only)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Detectmode(pub u32);
        impl Detectmode {
            #[doc = "Select between default DETECT signal behavior and LDETECT mode"]
            #[inline(always)]
            pub const fn detectmode(&self) -> super::vals::Detectmode {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Detectmode::from_bits(val as u8)
            }
            #[doc = "Select between default DETECT signal behavior and LDETECT mode"]
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
                #[derive(defmt :: Format)]
                struct Detectmode {
                    detectmode: super::vals::Detectmode,
                }
                let proxy = Detectmode {
                    detectmode: self.detectmode(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Select between default DETECT signal behavior and LDETECT mode (For secure pin only)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DetectmodeSec(pub u32);
        impl DetectmodeSec {
            #[doc = "Select between default DETECT signal behavior and LDETECT mode"]
            #[inline(always)]
            pub const fn detectmode(&self) -> super::vals::Detectmode {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Detectmode::from_bits(val as u8)
            }
            #[doc = "Select between default DETECT signal behavior and LDETECT mode"]
            #[inline(always)]
            pub fn set_detectmode(&mut self, val: super::vals::Detectmode) {
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
                #[derive(defmt :: Format)]
                struct DetectmodeSec {
                    detectmode: super::vals::Detectmode,
                }
                let proxy = DetectmodeSec {
                    detectmode: self.detectmode(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Dir {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dir")
                    .field(
                        "pin",
                        &[
                            self.pin(0usize),
                            self.pin(1usize),
                            self.pin(2usize),
                            self.pin(3usize),
                            self.pin(4usize),
                            self.pin(5usize),
                            self.pin(6usize),
                            self.pin(7usize),
                            self.pin(8usize),
                            self.pin(9usize),
                            self.pin(10usize),
                            self.pin(11usize),
                            self.pin(12usize),
                            self.pin(13usize),
                            self.pin(14usize),
                            self.pin(15usize),
                            self.pin(16usize),
                            self.pin(17usize),
                            self.pin(18usize),
                            self.pin(19usize),
                            self.pin(20usize),
                            self.pin(21usize),
                            self.pin(22usize),
                            self.pin(23usize),
                            self.pin(24usize),
                            self.pin(25usize),
                            self.pin(26usize),
                            self.pin(27usize),
                            self.pin(28usize),
                            self.pin(29usize),
                            self.pin(30usize),
                            self.pin(31usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dir {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Dir {
                    pin: [super::vals::Dir; 32usize],
                }
                let proxy = Dir {
                    pin: [
                        self.pin(0usize),
                        self.pin(1usize),
                        self.pin(2usize),
                        self.pin(3usize),
                        self.pin(4usize),
                        self.pin(5usize),
                        self.pin(6usize),
                        self.pin(7usize),
                        self.pin(8usize),
                        self.pin(9usize),
                        self.pin(10usize),
                        self.pin(11usize),
                        self.pin(12usize),
                        self.pin(13usize),
                        self.pin(14usize),
                        self.pin(15usize),
                        self.pin(16usize),
                        self.pin(17usize),
                        self.pin(18usize),
                        self.pin(19usize),
                        self.pin(20usize),
                        self.pin(21usize),
                        self.pin(22usize),
                        self.pin(23usize),
                        self.pin(24usize),
                        self.pin(25usize),
                        self.pin(26usize),
                        self.pin(27usize),
                        self.pin(28usize),
                        self.pin(29usize),
                        self.pin(30usize),
                        self.pin(31usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Dirclr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dirclr")
                    .field(
                        "pin",
                        &[
                            self.pin(0usize),
                            self.pin(1usize),
                            self.pin(2usize),
                            self.pin(3usize),
                            self.pin(4usize),
                            self.pin(5usize),
                            self.pin(6usize),
                            self.pin(7usize),
                            self.pin(8usize),
                            self.pin(9usize),
                            self.pin(10usize),
                            self.pin(11usize),
                            self.pin(12usize),
                            self.pin(13usize),
                            self.pin(14usize),
                            self.pin(15usize),
                            self.pin(16usize),
                            self.pin(17usize),
                            self.pin(18usize),
                            self.pin(19usize),
                            self.pin(20usize),
                            self.pin(21usize),
                            self.pin(22usize),
                            self.pin(23usize),
                            self.pin(24usize),
                            self.pin(25usize),
                            self.pin(26usize),
                            self.pin(27usize),
                            self.pin(28usize),
                            self.pin(29usize),
                            self.pin(30usize),
                            self.pin(31usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dirclr {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Dirclr {
                    pin: [bool; 32usize],
                }
                let proxy = Dirclr {
                    pin: [
                        self.pin(0usize),
                        self.pin(1usize),
                        self.pin(2usize),
                        self.pin(3usize),
                        self.pin(4usize),
                        self.pin(5usize),
                        self.pin(6usize),
                        self.pin(7usize),
                        self.pin(8usize),
                        self.pin(9usize),
                        self.pin(10usize),
                        self.pin(11usize),
                        self.pin(12usize),
                        self.pin(13usize),
                        self.pin(14usize),
                        self.pin(15usize),
                        self.pin(16usize),
                        self.pin(17usize),
                        self.pin(18usize),
                        self.pin(19usize),
                        self.pin(20usize),
                        self.pin(21usize),
                        self.pin(22usize),
                        self.pin(23usize),
                        self.pin(24usize),
                        self.pin(25usize),
                        self.pin(26usize),
                        self.pin(27usize),
                        self.pin(28usize),
                        self.pin(29usize),
                        self.pin(30usize),
                        self.pin(31usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Dirset {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dirset")
                    .field(
                        "pin",
                        &[
                            self.pin(0usize),
                            self.pin(1usize),
                            self.pin(2usize),
                            self.pin(3usize),
                            self.pin(4usize),
                            self.pin(5usize),
                            self.pin(6usize),
                            self.pin(7usize),
                            self.pin(8usize),
                            self.pin(9usize),
                            self.pin(10usize),
                            self.pin(11usize),
                            self.pin(12usize),
                            self.pin(13usize),
                            self.pin(14usize),
                            self.pin(15usize),
                            self.pin(16usize),
                            self.pin(17usize),
                            self.pin(18usize),
                            self.pin(19usize),
                            self.pin(20usize),
                            self.pin(21usize),
                            self.pin(22usize),
                            self.pin(23usize),
                            self.pin(24usize),
                            self.pin(25usize),
                            self.pin(26usize),
                            self.pin(27usize),
                            self.pin(28usize),
                            self.pin(29usize),
                            self.pin(30usize),
                            self.pin(31usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dirset {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Dirset {
                    pin: [bool; 32usize],
                }
                let proxy = Dirset {
                    pin: [
                        self.pin(0usize),
                        self.pin(1usize),
                        self.pin(2usize),
                        self.pin(3usize),
                        self.pin(4usize),
                        self.pin(5usize),
                        self.pin(6usize),
                        self.pin(7usize),
                        self.pin(8usize),
                        self.pin(9usize),
                        self.pin(10usize),
                        self.pin(11usize),
                        self.pin(12usize),
                        self.pin(13usize),
                        self.pin(14usize),
                        self.pin(15usize),
                        self.pin(16usize),
                        self.pin(17usize),
                        self.pin(18usize),
                        self.pin(19usize),
                        self.pin(20usize),
                        self.pin(21usize),
                        self.pin(22usize),
                        self.pin(23usize),
                        self.pin(24usize),
                        self.pin(25usize),
                        self.pin(26usize),
                        self.pin(27usize),
                        self.pin(28usize),
                        self.pin(29usize),
                        self.pin(30usize),
                        self.pin(31usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for In {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("In")
                    .field(
                        "pin",
                        &[
                            self.pin(0usize),
                            self.pin(1usize),
                            self.pin(2usize),
                            self.pin(3usize),
                            self.pin(4usize),
                            self.pin(5usize),
                            self.pin(6usize),
                            self.pin(7usize),
                            self.pin(8usize),
                            self.pin(9usize),
                            self.pin(10usize),
                            self.pin(11usize),
                            self.pin(12usize),
                            self.pin(13usize),
                            self.pin(14usize),
                            self.pin(15usize),
                            self.pin(16usize),
                            self.pin(17usize),
                            self.pin(18usize),
                            self.pin(19usize),
                            self.pin(20usize),
                            self.pin(21usize),
                            self.pin(22usize),
                            self.pin(23usize),
                            self.pin(24usize),
                            self.pin(25usize),
                            self.pin(26usize),
                            self.pin(27usize),
                            self.pin(28usize),
                            self.pin(29usize),
                            self.pin(30usize),
                            self.pin(31usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for In {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct In {
                    pin: [bool; 32usize],
                }
                let proxy = In {
                    pin: [
                        self.pin(0usize),
                        self.pin(1usize),
                        self.pin(2usize),
                        self.pin(3usize),
                        self.pin(4usize),
                        self.pin(5usize),
                        self.pin(6usize),
                        self.pin(7usize),
                        self.pin(8usize),
                        self.pin(9usize),
                        self.pin(10usize),
                        self.pin(11usize),
                        self.pin(12usize),
                        self.pin(13usize),
                        self.pin(14usize),
                        self.pin(15usize),
                        self.pin(16usize),
                        self.pin(17usize),
                        self.pin(18usize),
                        self.pin(19usize),
                        self.pin(20usize),
                        self.pin(21usize),
                        self.pin(22usize),
                        self.pin(23usize),
                        self.pin(24usize),
                        self.pin(25usize),
                        self.pin(26usize),
                        self.pin(27usize),
                        self.pin(28usize),
                        self.pin(29usize),
                        self.pin(30usize),
                        self.pin(31usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF\\[n\\].SENSE registers"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Latch(pub u32);
        impl Latch {
            #[doc = "Status on whether PIN\\[0\\] has met criteria set in PIN_CNF\\[0\\].SENSE register. Write '1' to clear."]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Status on whether PIN\\[0\\] has met criteria set in PIN_CNF\\[0\\].SENSE register. Write '1' to clear."]
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
        impl core::fmt::Debug for Latch {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Latch")
                    .field(
                        "pin",
                        &[
                            self.pin(0usize),
                            self.pin(1usize),
                            self.pin(2usize),
                            self.pin(3usize),
                            self.pin(4usize),
                            self.pin(5usize),
                            self.pin(6usize),
                            self.pin(7usize),
                            self.pin(8usize),
                            self.pin(9usize),
                            self.pin(10usize),
                            self.pin(11usize),
                            self.pin(12usize),
                            self.pin(13usize),
                            self.pin(14usize),
                            self.pin(15usize),
                            self.pin(16usize),
                            self.pin(17usize),
                            self.pin(18usize),
                            self.pin(19usize),
                            self.pin(20usize),
                            self.pin(21usize),
                            self.pin(22usize),
                            self.pin(23usize),
                            self.pin(24usize),
                            self.pin(25usize),
                            self.pin(26usize),
                            self.pin(27usize),
                            self.pin(28usize),
                            self.pin(29usize),
                            self.pin(30usize),
                            self.pin(31usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Latch {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Latch {
                    pin: [bool; 32usize],
                }
                let proxy = Latch {
                    pin: [
                        self.pin(0usize),
                        self.pin(1usize),
                        self.pin(2usize),
                        self.pin(3usize),
                        self.pin(4usize),
                        self.pin(5usize),
                        self.pin(6usize),
                        self.pin(7usize),
                        self.pin(8usize),
                        self.pin(9usize),
                        self.pin(10usize),
                        self.pin(11usize),
                        self.pin(12usize),
                        self.pin(13usize),
                        self.pin(14usize),
                        self.pin(15usize),
                        self.pin(16usize),
                        self.pin(17usize),
                        self.pin(18usize),
                        self.pin(19usize),
                        self.pin(20usize),
                        self.pin(21usize),
                        self.pin(22usize),
                        self.pin(23usize),
                        self.pin(24usize),
                        self.pin(25usize),
                        self.pin(26usize),
                        self.pin(27usize),
                        self.pin(28usize),
                        self.pin(29usize),
                        self.pin(30usize),
                        self.pin(31usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Out {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Out")
                    .field(
                        "pin",
                        &[
                            self.pin(0usize),
                            self.pin(1usize),
                            self.pin(2usize),
                            self.pin(3usize),
                            self.pin(4usize),
                            self.pin(5usize),
                            self.pin(6usize),
                            self.pin(7usize),
                            self.pin(8usize),
                            self.pin(9usize),
                            self.pin(10usize),
                            self.pin(11usize),
                            self.pin(12usize),
                            self.pin(13usize),
                            self.pin(14usize),
                            self.pin(15usize),
                            self.pin(16usize),
                            self.pin(17usize),
                            self.pin(18usize),
                            self.pin(19usize),
                            self.pin(20usize),
                            self.pin(21usize),
                            self.pin(22usize),
                            self.pin(23usize),
                            self.pin(24usize),
                            self.pin(25usize),
                            self.pin(26usize),
                            self.pin(27usize),
                            self.pin(28usize),
                            self.pin(29usize),
                            self.pin(30usize),
                            self.pin(31usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Out {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Out {
                    pin: [bool; 32usize],
                }
                let proxy = Out {
                    pin: [
                        self.pin(0usize),
                        self.pin(1usize),
                        self.pin(2usize),
                        self.pin(3usize),
                        self.pin(4usize),
                        self.pin(5usize),
                        self.pin(6usize),
                        self.pin(7usize),
                        self.pin(8usize),
                        self.pin(9usize),
                        self.pin(10usize),
                        self.pin(11usize),
                        self.pin(12usize),
                        self.pin(13usize),
                        self.pin(14usize),
                        self.pin(15usize),
                        self.pin(16usize),
                        self.pin(17usize),
                        self.pin(18usize),
                        self.pin(19usize),
                        self.pin(20usize),
                        self.pin(21usize),
                        self.pin(22usize),
                        self.pin(23usize),
                        self.pin(24usize),
                        self.pin(25usize),
                        self.pin(26usize),
                        self.pin(27usize),
                        self.pin(28usize),
                        self.pin(29usize),
                        self.pin(30usize),
                        self.pin(31usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Outclr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Outclr")
                    .field(
                        "pin",
                        &[
                            self.pin(0usize),
                            self.pin(1usize),
                            self.pin(2usize),
                            self.pin(3usize),
                            self.pin(4usize),
                            self.pin(5usize),
                            self.pin(6usize),
                            self.pin(7usize),
                            self.pin(8usize),
                            self.pin(9usize),
                            self.pin(10usize),
                            self.pin(11usize),
                            self.pin(12usize),
                            self.pin(13usize),
                            self.pin(14usize),
                            self.pin(15usize),
                            self.pin(16usize),
                            self.pin(17usize),
                            self.pin(18usize),
                            self.pin(19usize),
                            self.pin(20usize),
                            self.pin(21usize),
                            self.pin(22usize),
                            self.pin(23usize),
                            self.pin(24usize),
                            self.pin(25usize),
                            self.pin(26usize),
                            self.pin(27usize),
                            self.pin(28usize),
                            self.pin(29usize),
                            self.pin(30usize),
                            self.pin(31usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Outclr {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Outclr {
                    pin: [bool; 32usize],
                }
                let proxy = Outclr {
                    pin: [
                        self.pin(0usize),
                        self.pin(1usize),
                        self.pin(2usize),
                        self.pin(3usize),
                        self.pin(4usize),
                        self.pin(5usize),
                        self.pin(6usize),
                        self.pin(7usize),
                        self.pin(8usize),
                        self.pin(9usize),
                        self.pin(10usize),
                        self.pin(11usize),
                        self.pin(12usize),
                        self.pin(13usize),
                        self.pin(14usize),
                        self.pin(15usize),
                        self.pin(16usize),
                        self.pin(17usize),
                        self.pin(18usize),
                        self.pin(19usize),
                        self.pin(20usize),
                        self.pin(21usize),
                        self.pin(22usize),
                        self.pin(23usize),
                        self.pin(24usize),
                        self.pin(25usize),
                        self.pin(26usize),
                        self.pin(27usize),
                        self.pin(28usize),
                        self.pin(29usize),
                        self.pin(30usize),
                        self.pin(31usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Outset {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Outset")
                    .field(
                        "pin",
                        &[
                            self.pin(0usize),
                            self.pin(1usize),
                            self.pin(2usize),
                            self.pin(3usize),
                            self.pin(4usize),
                            self.pin(5usize),
                            self.pin(6usize),
                            self.pin(7usize),
                            self.pin(8usize),
                            self.pin(9usize),
                            self.pin(10usize),
                            self.pin(11usize),
                            self.pin(12usize),
                            self.pin(13usize),
                            self.pin(14usize),
                            self.pin(15usize),
                            self.pin(16usize),
                            self.pin(17usize),
                            self.pin(18usize),
                            self.pin(19usize),
                            self.pin(20usize),
                            self.pin(21usize),
                            self.pin(22usize),
                            self.pin(23usize),
                            self.pin(24usize),
                            self.pin(25usize),
                            self.pin(26usize),
                            self.pin(27usize),
                            self.pin(28usize),
                            self.pin(29usize),
                            self.pin(30usize),
                            self.pin(31usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Outset {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Outset {
                    pin: [bool; 32usize],
                }
                let proxy = Outset {
                    pin: [
                        self.pin(0usize),
                        self.pin(1usize),
                        self.pin(2usize),
                        self.pin(3usize),
                        self.pin(4usize),
                        self.pin(5usize),
                        self.pin(6usize),
                        self.pin(7usize),
                        self.pin(8usize),
                        self.pin(9usize),
                        self.pin(10usize),
                        self.pin(11usize),
                        self.pin(12usize),
                        self.pin(13usize),
                        self.pin(14usize),
                        self.pin(15usize),
                        self.pin(16usize),
                        self.pin(17usize),
                        self.pin(18usize),
                        self.pin(19usize),
                        self.pin(20usize),
                        self.pin(21usize),
                        self.pin(22usize),
                        self.pin(23usize),
                        self.pin(24usize),
                        self.pin(25usize),
                        self.pin(26usize),
                        self.pin(27usize),
                        self.pin(28usize),
                        self.pin(29usize),
                        self.pin(30usize),
                        self.pin(31usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct PinCnf {
                    dir: super::vals::Dir,
                    input: super::vals::Input,
                    pull: super::vals::Pull,
                    drive: super::vals::Drive,
                    sense: super::vals::Sense,
                }
                let proxy = PinCnf {
                    dir: self.dir(),
                    input: self.input(),
                    pull: self.pull(),
                    drive: self.drive(),
                    sense: self.sense(),
                };
                defmt::write!(f, "{}", proxy)
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
    #[doc = "GPIO Tasks and Events 0"]
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
        #[doc = "Description collection: Subscribe configuration for task OUT\\[n\\]"]
        #[inline(always)]
        pub const fn subscribe_out(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Subscribe configuration for task SET\\[n\\]"]
        #[inline(always)]
        pub const fn subscribe_set(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Subscribe configuration for task CLR\\[n\\]"]
        #[inline(always)]
        pub const fn subscribe_clr(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + n * 4usize) as _) }
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
        #[doc = "Description collection: Publish configuration for event IN\\[n\\]"]
        #[inline(always)]
        pub const fn publish_in(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize + n * 4usize) as _) }
        }
        #[doc = "Publish configuration for event PORT"]
        #[inline(always)]
        pub const fn publish_port(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01fcusize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Description collection: Configuration for OUT\\[n\\], SET\\[n\\], and CLR\\[n\\] tasks and IN\\[n\\] event"]
        #[inline(always)]
        pub const fn config(self, n: usize) -> crate::common::Reg<regs::Config, crate::common::RW> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize + n * 4usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Description collection: Configuration for OUT\\[n\\], SET\\[n\\], and CLR\\[n\\] tasks and IN\\[n\\] event"]
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
            #[doc = "GPIO number associated with SET\\[n\\], CLR\\[n\\], and OUT\\[n\\] tasks and IN\\[n\\] event"]
            #[inline(always)]
            pub const fn psel(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0x1f;
                val as u8
            }
            #[doc = "GPIO number associated with SET\\[n\\], CLR\\[n\\], and OUT\\[n\\] tasks and IN\\[n\\] event"]
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
                #[derive(defmt :: Format)]
                struct Config {
                    mode: super::vals::Mode,
                    psel: u8,
                    polarity: super::vals::Polarity,
                    outinit: super::vals::Outinit,
                }
                let proxy = Config {
                    mode: self.mode(),
                    psel: self.psel(),
                    polarity: self.polarity(),
                    outinit: self.outinit(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event IN\\[0\\]"]
            #[inline(always)]
            pub const fn in_(&self, n: usize) -> bool {
                assert!(n < 8usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event IN\\[0\\]"]
            #[inline(always)]
            pub fn set_in_(&mut self, n: usize, val: bool) {
                assert!(n < 8usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field(
                        "in_",
                        &[
                            self.in_(0usize),
                            self.in_(1usize),
                            self.in_(2usize),
                            self.in_(3usize),
                            self.in_(4usize),
                            self.in_(5usize),
                            self.in_(6usize),
                            self.in_(7usize),
                        ],
                    )
                    .field("port", &self.port())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    in_: [bool; 8usize],
                    port: bool,
                }
                let proxy = Int {
                    in_: [
                        self.in_(0usize),
                        self.in_(1usize),
                        self.in_(2usize),
                        self.in_(3usize),
                        self.in_(4usize),
                        self.in_(5usize),
                        self.in_(6usize),
                        self.in_(7usize),
                    ],
                    port: self.port(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
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
    #[doc = "Inter-IC Sound 0"]
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
        #[doc = "Stops I2S transfer. Also stops MCK generator. Triggering this task will cause the STOPPED event to be generated."]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Subscribe configuration for task START"]
        #[inline(always)]
        pub const fn subscribe_start(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
        }
        #[doc = "Subscribe configuration for task STOP"]
        #[inline(always)]
        pub const fn subscribe_stop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
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
        #[doc = "Publish configuration for event RXPTRUPD"]
        #[inline(always)]
        pub const fn publish_rxptrupd(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
        }
        #[doc = "Publish configuration for event STOPPED"]
        #[inline(always)]
        pub const fn publish_stopped(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
        }
        #[doc = "Publish configuration for event TXPTRUPD"]
        #[inline(always)]
        pub const fn publish_txptrupd(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0194usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
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
        impl core::fmt::Debug for Align {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Align")
                    .field("align", &self.align())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Align {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Align {
                    align: super::vals::Align,
                }
                let proxy = Align {
                    align: self.align(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Channels {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Channels")
                    .field("channels", &self.channels())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Channels {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Channels {
                    channels: super::vals::Channels,
                }
                let proxy = Channels {
                    channels: self.channels(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Enable {
                    enable: bool,
                }
                let proxy = Enable {
                    enable: self.enable(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Format {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Format")
                    .field("format", &self.format())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Format {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Format {
                    format: super::vals::Format,
                }
                let proxy = Format {
                    format: self.format(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Enable or disable interrupt for event RXPTRUPD"]
            #[inline(always)]
            pub const fn rxptrupd(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RXPTRUPD"]
            #[inline(always)]
            pub fn set_rxptrupd(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for event STOPPED"]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event STOPPED"]
            #[inline(always)]
            pub fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable or disable interrupt for event TXPTRUPD"]
            #[inline(always)]
            pub const fn txptrupd(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event TXPTRUPD"]
            #[inline(always)]
            pub fn set_txptrupd(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                    .field("rxptrupd", &self.rxptrupd())
                    .field("stopped", &self.stopped())
                    .field("txptrupd", &self.txptrupd())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    rxptrupd: bool,
                    stopped: bool,
                    txptrupd: bool,
                }
                let proxy = Int {
                    rxptrupd: self.rxptrupd(),
                    stopped: self.stopped(),
                    txptrupd: self.txptrupd(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Maxcnt {
                    maxcnt: u16,
                }
                let proxy = Maxcnt {
                    maxcnt: self.maxcnt(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Mcken {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Mcken")
                    .field("mcken", &self.mcken())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Mcken {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Mcken {
                    mcken: bool,
                }
                let proxy = Mcken {
                    mcken: self.mcken(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Mckfreq {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Mckfreq")
                    .field("mckfreq", &self.mckfreq())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Mckfreq {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Mckfreq {
                    mckfreq: super::vals::Mckfreq,
                }
                let proxy = Mckfreq {
                    mckfreq: self.mckfreq(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Mode {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Mode").field("mode", &self.mode()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Mode {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Mode {
                    mode: super::vals::Mode,
                }
                let proxy = Mode { mode: self.mode() };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Ratio {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ratio")
                    .field("ratio", &self.ratio())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ratio {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Ratio {
                    ratio: super::vals::Ratio,
                }
                let proxy = Ratio {
                    ratio: self.ratio(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Rxen {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rxen").field("rxen", &self.rxen()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rxen {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Rxen {
                    rxen: bool,
                }
                let proxy = Rxen { rxen: self.rxen() };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Swidth {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Swidth")
                    .field("swidth", &self.swidth())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Swidth {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Swidth {
                    swidth: super::vals::Swidth,
                }
                let proxy = Swidth {
                    swidth: self.swidth(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Txen {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Txen").field("txen", &self.txen()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Txen {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Txen {
                    txen: bool,
                }
                let proxy = Txen { txen: self.txen() };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
        }
        impl Mckfreq {
            pub const fn from_bits(val: u32) -> Mckfreq {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Mckfreq {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x020c_0000 => f.write_str("_32MDIV125"),
                    0x0410_0000 => f.write_str("_32MDIV63"),
                    0x0600_0000 => f.write_str("_32MDIV42"),
                    0x0800_0000 => f.write_str("_32MDIV32"),
                    0x0840_0000 => f.write_str("_32MDIV31"),
                    0x0880_0000 => f.write_str("_32MDIV30"),
                    0x0b00_0000 => f.write_str("_32MDIV23"),
                    0x0c00_0000 => f.write_str("_32MDIV21"),
                    0x1000_0000 => f.write_str("_32MDIV16"),
                    0x1100_0000 => f.write_str("_32MDIV15"),
                    0x1600_0000 => f.write_str("_32MDIV11"),
                    0x1800_0000 => f.write_str("_32MDIV10"),
                    0x2000_0000 => f.write_str("_32MDIV8"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Mckfreq {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x020c_0000 => defmt::write!(f, "_32MDIV125"),
                    0x0410_0000 => defmt::write!(f, "_32MDIV63"),
                    0x0600_0000 => defmt::write!(f, "_32MDIV42"),
                    0x0800_0000 => defmt::write!(f, "_32MDIV32"),
                    0x0840_0000 => defmt::write!(f, "_32MDIV31"),
                    0x0880_0000 => defmt::write!(f, "_32MDIV30"),
                    0x0b00_0000 => defmt::write!(f, "_32MDIV23"),
                    0x0c00_0000 => defmt::write!(f, "_32MDIV21"),
                    0x1000_0000 => defmt::write!(f, "_32MDIV16"),
                    0x1100_0000 => defmt::write!(f, "_32MDIV15"),
                    0x1600_0000 => defmt::write!(f, "_32MDIV11"),
                    0x1800_0000 => defmt::write!(f, "_32MDIV10"),
                    0x2000_0000 => defmt::write!(f, "_32MDIV8"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
pub mod ipc {
    #[doc = "Interprocessor communication 0"]
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
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Subscribe configuration for task SEND\\[n\\]"]
        #[inline(always)]
        pub const fn subscribe_send(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Event received on one or more of the enabled IPC channels in RECEIVE_CNF\\[n\\]"]
        #[inline(always)]
        pub const fn events_receive(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Publish configuration for event RECEIVE\\[n\\]"]
        #[inline(always)]
        pub const fn publish_receive(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize + n * 4usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Pending interrupts"]
        #[inline(always)]
        pub const fn intpend(self) -> crate::common::Reg<regs::Int, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x030cusize) as _) }
        }
        #[doc = "Description collection: Send event configuration for TASKS_SEND\\[n\\]"]
        #[inline(always)]
        pub const fn send_cnf(
            self,
            n: usize,
        ) -> crate::common::Reg<regs::SendCnf, crate::common::RW> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Receive event configuration for EVENTS_RECEIVE\\[n\\]"]
        #[inline(always)]
        pub const fn receive_cnf(
            self,
            n: usize,
        ) -> crate::common::Reg<regs::ReceiveCnf, crate::common::RW> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0590usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: General purpose memory"]
        #[inline(always)]
        pub const fn gpmem(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0610usize + n * 4usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Enable or disable interrupt for event RECEIVE\\[0\\]"]
            #[inline(always)]
            pub const fn receive0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[0\\]"]
            #[inline(always)]
            pub fn set_receive0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[1\\]"]
            #[inline(always)]
            pub const fn receive1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[1\\]"]
            #[inline(always)]
            pub fn set_receive1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[2\\]"]
            #[inline(always)]
            pub const fn receive2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[2\\]"]
            #[inline(always)]
            pub fn set_receive2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[3\\]"]
            #[inline(always)]
            pub const fn receive3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[3\\]"]
            #[inline(always)]
            pub fn set_receive3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[4\\]"]
            #[inline(always)]
            pub const fn receive4(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[4\\]"]
            #[inline(always)]
            pub fn set_receive4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[5\\]"]
            #[inline(always)]
            pub const fn receive5(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[5\\]"]
            #[inline(always)]
            pub fn set_receive5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[6\\]"]
            #[inline(always)]
            pub const fn receive6(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[6\\]"]
            #[inline(always)]
            pub fn set_receive6(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[7\\]"]
            #[inline(always)]
            pub const fn receive7(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RECEIVE\\[7\\]"]
            #[inline(always)]
            pub fn set_receive7(&mut self, val: bool) {
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
                    .field("receive0", &self.receive0())
                    .field("receive1", &self.receive1())
                    .field("receive2", &self.receive2())
                    .field("receive3", &self.receive3())
                    .field("receive4", &self.receive4())
                    .field("receive5", &self.receive5())
                    .field("receive6", &self.receive6())
                    .field("receive7", &self.receive7())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    receive0: bool,
                    receive1: bool,
                    receive2: bool,
                    receive3: bool,
                    receive4: bool,
                    receive5: bool,
                    receive6: bool,
                    receive7: bool,
                }
                let proxy = Int {
                    receive0: self.receive0(),
                    receive1: self.receive1(),
                    receive2: self.receive2(),
                    receive3: self.receive3(),
                    receive4: self.receive4(),
                    receive5: self.receive5(),
                    receive6: self.receive6(),
                    receive7: self.receive7(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description collection: Receive event configuration for EVENTS_RECEIVE\\[n\\]"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct ReceiveCnf(pub u32);
        impl ReceiveCnf {
            #[doc = "Enable subscription to IPC channel 0"]
            #[inline(always)]
            pub const fn chen0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable subscription to IPC channel 0"]
            #[inline(always)]
            pub fn set_chen0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable subscription to IPC channel 1"]
            #[inline(always)]
            pub const fn chen1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable subscription to IPC channel 1"]
            #[inline(always)]
            pub fn set_chen1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable subscription to IPC channel 2"]
            #[inline(always)]
            pub const fn chen2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable subscription to IPC channel 2"]
            #[inline(always)]
            pub fn set_chen2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable subscription to IPC channel 3"]
            #[inline(always)]
            pub const fn chen3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable subscription to IPC channel 3"]
            #[inline(always)]
            pub fn set_chen3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable subscription to IPC channel 4"]
            #[inline(always)]
            pub const fn chen4(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable subscription to IPC channel 4"]
            #[inline(always)]
            pub fn set_chen4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable subscription to IPC channel 5"]
            #[inline(always)]
            pub const fn chen5(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable subscription to IPC channel 5"]
            #[inline(always)]
            pub fn set_chen5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable subscription to IPC channel 6"]
            #[inline(always)]
            pub const fn chen6(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable subscription to IPC channel 6"]
            #[inline(always)]
            pub fn set_chen6(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Enable subscription to IPC channel 7"]
            #[inline(always)]
            pub const fn chen7(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable subscription to IPC channel 7"]
            #[inline(always)]
            pub fn set_chen7(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for ReceiveCnf {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct ReceiveCnf {
                    chen0: bool,
                    chen1: bool,
                    chen2: bool,
                    chen3: bool,
                    chen4: bool,
                    chen5: bool,
                    chen6: bool,
                    chen7: bool,
                }
                let proxy = ReceiveCnf {
                    chen0: self.chen0(),
                    chen1: self.chen1(),
                    chen2: self.chen2(),
                    chen3: self.chen3(),
                    chen4: self.chen4(),
                    chen5: self.chen5(),
                    chen6: self.chen6(),
                    chen7: self.chen7(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description collection: Send event configuration for TASKS_SEND\\[n\\]"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SendCnf(pub u32);
        impl SendCnf {
            #[doc = "Enable broadcasting on IPC channel 0"]
            #[inline(always)]
            pub const fn chen0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable broadcasting on IPC channel 0"]
            #[inline(always)]
            pub fn set_chen0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable broadcasting on IPC channel 1"]
            #[inline(always)]
            pub const fn chen1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable broadcasting on IPC channel 1"]
            #[inline(always)]
            pub fn set_chen1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable broadcasting on IPC channel 2"]
            #[inline(always)]
            pub const fn chen2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable broadcasting on IPC channel 2"]
            #[inline(always)]
            pub fn set_chen2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable broadcasting on IPC channel 3"]
            #[inline(always)]
            pub const fn chen3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable broadcasting on IPC channel 3"]
            #[inline(always)]
            pub fn set_chen3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable broadcasting on IPC channel 4"]
            #[inline(always)]
            pub const fn chen4(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable broadcasting on IPC channel 4"]
            #[inline(always)]
            pub fn set_chen4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable broadcasting on IPC channel 5"]
            #[inline(always)]
            pub const fn chen5(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable broadcasting on IPC channel 5"]
            #[inline(always)]
            pub fn set_chen5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable broadcasting on IPC channel 6"]
            #[inline(always)]
            pub const fn chen6(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable broadcasting on IPC channel 6"]
            #[inline(always)]
            pub fn set_chen6(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Enable broadcasting on IPC channel 7"]
            #[inline(always)]
            pub const fn chen7(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable broadcasting on IPC channel 7"]
            #[inline(always)]
            pub fn set_chen7(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SendCnf {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct SendCnf {
                    chen0: bool,
                    chen1: bool,
                    chen2: bool,
                    chen3: bool,
                    chen4: bool,
                    chen5: bool,
                    chen6: bool,
                    chen7: bool,
                }
                let proxy = SendCnf {
                    chen0: self.chen0(),
                    chen1: self.chen1(),
                    chen2: self.chen2(),
                    chen3: self.chen3(),
                    chen4: self.chen4(),
                    chen5: self.chen5(),
                    chen6: self.chen6(),
                    chen7: self.chen7(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
}
pub mod kmu {
    #[doc = "Key management unit 0"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Kmu {
        ptr: *mut u8,
    }
    unsafe impl Send for Kmu {}
    unsafe impl Sync for Kmu {}
    impl Kmu {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Push a key slot over secure APB"]
        #[inline(always)]
        pub const fn tasks_push_keyslot(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Key slot successfully pushed over secure APB"]
        #[inline(always)]
        pub const fn events_keyslot_pushed(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Key slot has been revoked and cannot be tasked for selection"]
        #[inline(always)]
        pub const fn events_keyslot_revoked(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "No key slot selected, no destination address defined, or error during push operation"]
        #[inline(always)]
        pub const fn events_keyslot_error(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Pending interrupts"]
        #[inline(always)]
        pub const fn intpend(self) -> crate::common::Reg<regs::Int, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x030cusize) as _) }
        }
        #[doc = "Status bits for KMU operation"]
        #[inline(always)]
        pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x040cusize) as _) }
        }
        #[doc = "Select key slot to be read over AHB or pushed over secure APB when TASKS_PUSH_KEYSLOT is started"]
        #[inline(always)]
        pub const fn selectkeyslot(
            self,
        ) -> crate::common::Reg<regs::Selectkeyslot, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Enable or disable interrupt for event KEYSLOT_PUSHED"]
            #[inline(always)]
            pub const fn keyslot_pushed(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event KEYSLOT_PUSHED"]
            #[inline(always)]
            pub fn set_keyslot_pushed(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable interrupt for event KEYSLOT_REVOKED"]
            #[inline(always)]
            pub const fn keyslot_revoked(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event KEYSLOT_REVOKED"]
            #[inline(always)]
            pub fn set_keyslot_revoked(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for event KEYSLOT_ERROR"]
            #[inline(always)]
            pub const fn keyslot_error(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event KEYSLOT_ERROR"]
            #[inline(always)]
            pub fn set_keyslot_error(&mut self, val: bool) {
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
                    .field("keyslot_pushed", &self.keyslot_pushed())
                    .field("keyslot_revoked", &self.keyslot_revoked())
                    .field("keyslot_error", &self.keyslot_error())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    keyslot_pushed: bool,
                    keyslot_revoked: bool,
                    keyslot_error: bool,
                }
                let proxy = Int {
                    keyslot_pushed: self.keyslot_pushed(),
                    keyslot_revoked: self.keyslot_revoked(),
                    keyslot_error: self.keyslot_error(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Select key slot to be read over AHB or pushed over secure APB when TASKS_PUSH_KEYSLOT is started"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Selectkeyslot(pub u32);
        impl Selectkeyslot {
            #[doc = "Select key slot ID to be read over AHB, or pushed over secure APB, when TASKS_PUSH_KEYSLOT is started. NOTE: ID=0 is not a valid key slot ID. The 0 ID should be used when the KMU is idle or not in use. NOTE: Index N in UICR-&gt;KEYSLOT.KEY\\[N\\] and UICR-&gt;KEYSLOT.CONFIG\\[N\\] corresponds to KMU key slot ID=N+1."]
            #[inline(always)]
            pub const fn id(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Select key slot ID to be read over AHB, or pushed over secure APB, when TASKS_PUSH_KEYSLOT is started. NOTE: ID=0 is not a valid key slot ID. The 0 ID should be used when the KMU is idle or not in use. NOTE: Index N in UICR-&gt;KEYSLOT.KEY\\[N\\] and UICR-&gt;KEYSLOT.CONFIG\\[N\\] corresponds to KMU key slot ID=N+1."]
            #[inline(always)]
            pub fn set_id(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Selectkeyslot {
            #[inline(always)]
            fn default() -> Selectkeyslot {
                Selectkeyslot(0)
            }
        }
        impl core::fmt::Debug for Selectkeyslot {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Selectkeyslot")
                    .field("id", &self.id())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Selectkeyslot {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Selectkeyslot {
                    id: u8,
                }
                let proxy = Selectkeyslot { id: self.id() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Status bits for KMU operation"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Status(pub u32);
        impl Status {
            #[doc = "Key slot ID successfully selected by the KMU"]
            #[inline(always)]
            pub const fn selected(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Key slot ID successfully selected by the KMU"]
            #[inline(always)]
            pub fn set_selected(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Violation status"]
            #[inline(always)]
            pub const fn blocked(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Violation status"]
            #[inline(always)]
            pub fn set_blocked(&mut self, val: bool) {
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
                    .field("selected", &self.selected())
                    .field("blocked", &self.blocked())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Status {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Status {
                    selected: bool,
                    blocked: bool,
                }
                let proxy = Status {
                    selected: self.selected(),
                    blocked: self.blocked(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
}
pub mod nvmc {
    #[doc = "Non-volatile memory controller 0"]
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
        #[doc = "Ready flag"]
        #[inline(always)]
        pub const fn readynext(self) -> crate::common::Reg<regs::Readynext, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0408usize) as _) }
        }
        #[doc = "Configuration register"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Register for erasing all non-volatile user memory"]
        #[inline(always)]
        pub const fn eraseall(self) -> crate::common::Reg<regs::Eraseall, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Register for partial erase configuration"]
        #[inline(always)]
        pub const fn erasepagepartialcfg(
            self,
        ) -> crate::common::Reg<regs::Erasepagepartialcfg, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
        }
        #[doc = "I-code cache configuration register"]
        #[inline(always)]
        pub const fn icachecnf(self) -> crate::common::Reg<regs::Icachecnf, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize) as _) }
        }
        #[doc = "I-code cache hit counter"]
        #[inline(always)]
        pub const fn ihit(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0548usize) as _) }
        }
        #[doc = "I-code cache miss counter"]
        #[inline(always)]
        pub const fn imiss(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x054cusize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn configns(self) -> crate::common::Reg<regs::Configns, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0584usize) as _) }
        }
        #[doc = "Non-secure APPROTECT enable register"]
        #[inline(always)]
        pub const fn writeuicrns(self) -> crate::common::Reg<regs::Writeuicrns, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0588usize) as _) }
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
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::Wen::from_bits(val as u8)
            }
            #[doc = "Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated."]
            #[inline(always)]
            pub fn set_wen(&mut self, val: super::vals::Wen) {
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
                #[derive(defmt :: Format)]
                struct Config {
                    wen: super::vals::Wen,
                }
                let proxy = Config { wen: self.wen() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Unspecified"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Configns(pub u32);
        impl Configns {
            #[doc = "Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated."]
            #[inline(always)]
            pub const fn wen(&self) -> super::vals::ConfignsWen {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::ConfignsWen::from_bits(val as u8)
            }
            #[doc = "Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated."]
            #[inline(always)]
            pub fn set_wen(&mut self, val: super::vals::ConfignsWen) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Configns {
            #[inline(always)]
            fn default() -> Configns {
                Configns(0)
            }
        }
        impl core::fmt::Debug for Configns {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Configns")
                    .field("wen", &self.wen())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Configns {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Configns {
                    wen: super::vals::ConfignsWen,
                }
                let proxy = Configns { wen: self.wen() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Register for erasing all non-volatile user memory"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Eraseall(pub u32);
        impl Eraseall {
            #[doc = "Erase all non-volatile memory including UICR registers. Note that erasing must be enabled by setting CONFIG.WEN = Een before the non-volatile memory can be erased."]
            #[inline(always)]
            pub const fn eraseall(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Erase all non-volatile memory including UICR registers. Note that erasing must be enabled by setting CONFIG.WEN = Een before the non-volatile memory can be erased."]
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
                #[derive(defmt :: Format)]
                struct Eraseall {
                    eraseall: bool,
                }
                let proxy = Eraseall {
                    eraseall: self.eraseall(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Erasepagepartialcfg {
                    duration: u8,
                }
                let proxy = Erasepagepartialcfg {
                    duration: self.duration(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "I-code cache configuration register"]
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
                #[derive(defmt :: Format)]
                struct Icachecnf {
                    cacheen: bool,
                    cacheprofen: bool,
                }
                let proxy = Icachecnf {
                    cacheen: self.cacheen(),
                    cacheprofen: self.cacheprofen(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Ready {
                    ready: bool,
                }
                let proxy = Ready {
                    ready: self.ready(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Ready flag"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Readynext(pub u32);
        impl Readynext {
            #[doc = "NVMC can accept a new write operation"]
            #[inline(always)]
            pub const fn readynext(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "NVMC can accept a new write operation"]
            #[inline(always)]
            pub fn set_readynext(&mut self, val: bool) {
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
                #[derive(defmt :: Format)]
                struct Readynext {
                    readynext: bool,
                }
                let proxy = Readynext {
                    readynext: self.readynext(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Non-secure APPROTECT enable register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Writeuicrns(pub u32);
        impl Writeuicrns {
            #[doc = "Allow non-secure code to set APPROTECT"]
            #[inline(always)]
            pub const fn set(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Allow non-secure code to set APPROTECT"]
            #[inline(always)]
            pub fn set_set(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Key to write in order to validate the write operation"]
            #[inline(always)]
            pub const fn key(&self) -> super::vals::Key {
                let val = (self.0 >> 4usize) & 0x0fff_ffff;
                super::vals::Key::from_bits(val as u32)
            }
            #[doc = "Key to write in order to validate the write operation"]
            #[inline(always)]
            pub fn set_key(&mut self, val: super::vals::Key) {
                self.0 = (self.0 & !(0x0fff_ffff << 4usize))
                    | (((val.to_bits() as u32) & 0x0fff_ffff) << 4usize);
            }
        }
        impl Default for Writeuicrns {
            #[inline(always)]
            fn default() -> Writeuicrns {
                Writeuicrns(0)
            }
        }
        impl core::fmt::Debug for Writeuicrns {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Writeuicrns")
                    .field("set", &self.set())
                    .field("key", &self.key())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Writeuicrns {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Writeuicrns {
                    set: bool,
                    key: super::vals::Key,
                }
                let proxy = Writeuicrns {
                    set: self.set(),
                    key: self.key(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum ConfignsWen {
            #[doc = "Read only access"]
            REN = 0x0,
            #[doc = "Write enabled"]
            WEN = 0x01,
            #[doc = "Erase enabled"]
            EEN = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl ConfignsWen {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> ConfignsWen {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for ConfignsWen {
            #[inline(always)]
            fn from(val: u8) -> ConfignsWen {
                ConfignsWen::from_bits(val)
            }
        }
        impl From<ConfignsWen> for u8 {
            #[inline(always)]
            fn from(val: ConfignsWen) -> u8 {
                ConfignsWen::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Key(pub u32);
        impl Key {
            #[doc = "Key value"]
            pub const KEYVALID: Self = Self(0x0afb_e5a7);
        }
        impl Key {
            pub const fn from_bits(val: u32) -> Key {
                Self(val & 0x0fff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Key {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0afb_e5a7 => f.write_str("KEYVALID"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Key {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0afb_e5a7 => defmt::write!(f, "KEYVALID"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for Key {
            #[inline(always)]
            fn from(val: u32) -> Key {
                Key::from_bits(val)
            }
        }
        impl From<Key> for u32 {
            #[inline(always)]
            fn from(val: Key) -> u32 {
                Key::to_bits(val)
            }
        }
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
pub mod pdm {
    #[doc = "Pulse Density Modulation (Digital Microphone) Interface 0"]
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
        #[doc = "Subscribe configuration for task START"]
        #[inline(always)]
        pub const fn subscribe_start(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
        }
        #[doc = "Subscribe configuration for task STOP"]
        #[inline(always)]
        pub const fn subscribe_stop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
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
        #[doc = "Publish configuration for event STARTED"]
        #[inline(always)]
        pub const fn publish_started(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
        }
        #[doc = "Publish configuration for event STOPPED"]
        #[inline(always)]
        pub const fn publish_stopped(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
        }
        #[doc = "Publish configuration for event END"]
        #[inline(always)]
        pub const fn publish_end(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
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
        #[doc = "Selects the ratio between PDM_CLK and output sample rate. Change PDMCLKCTRL accordingly."]
        #[inline(always)]
        pub const fn ratio(self) -> crate::common::Reg<regs::Ratio, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize) as _) }
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
                #[derive(defmt :: Format)]
                struct Enable {
                    enable: bool,
                }
                let proxy = Enable {
                    enable: self.enable(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Gainl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Gainl")
                    .field("gainl", &self.gainl())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Gainl {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Gainl {
                    gainl: super::vals::Gain,
                }
                let proxy = Gainl {
                    gainl: self.gainl(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Gainr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Gainr")
                    .field("gainr", &self.gainr())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Gainr {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Gainr {
                    gainr: super::vals::Gain,
                }
                let proxy = Gainr {
                    gainr: self.gainr(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
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
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("started", &self.started())
                    .field("stopped", &self.stopped())
                    .field("end", &self.end())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    started: bool,
                    stopped: bool,
                    end: bool,
                }
                let proxy = Int {
                    started: self.started(),
                    stopped: self.stopped(),
                    end: self.end(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Maxcnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Maxcnt")
                    .field("buffsize", &self.buffsize())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Maxcnt {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Maxcnt {
                    buffsize: u16,
                }
                let proxy = Maxcnt {
                    buffsize: self.buffsize(),
                };
                defmt::write!(f, "{}", proxy)
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
            #[doc = "Defines on which PDM_CLK edge left (or mono) is sampled"]
            #[inline(always)]
            pub const fn edge(&self) -> super::vals::Edge {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Edge::from_bits(val as u8)
            }
            #[doc = "Defines on which PDM_CLK edge left (or mono) is sampled"]
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
        impl core::fmt::Debug for Mode {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Mode")
                    .field("operation", &self.operation())
                    .field("edge", &self.edge())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Mode {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Mode {
                    operation: super::vals::Operation,
                    edge: super::vals::Edge,
                }
                let proxy = Mode {
                    operation: self.operation(),
                    edge: self.edge(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "PDM clock generator control"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pdmclkctrl(pub u32);
        impl Pdmclkctrl {
            #[doc = "PDM_CLK frequency configuration."]
            #[inline(always)]
            pub const fn freq(&self) -> super::vals::Freq {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Freq::from_bits(val as u32)
            }
            #[doc = "PDM_CLK frequency configuration."]
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
        impl core::fmt::Debug for Pdmclkctrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Pdmclkctrl")
                    .field("freq", &self.freq())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Pdmclkctrl {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Pdmclkctrl {
                    freq: super::vals::Freq,
                }
                let proxy = Pdmclkctrl { freq: self.freq() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Selects the ratio between PDM_CLK and output sample rate. Change PDMCLKCTRL accordingly."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ratio(pub u32);
        impl Ratio {
            #[doc = "Selects the ratio between PDM_CLK and output sample rate"]
            #[inline(always)]
            pub const fn ratio(&self) -> super::vals::Ratio {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Ratio::from_bits(val as u8)
            }
            #[doc = "Selects the ratio between PDM_CLK and output sample rate"]
            #[inline(always)]
            pub fn set_ratio(&mut self, val: super::vals::Ratio) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Ratio {
            #[inline(always)]
            fn default() -> Ratio {
                Ratio(0)
            }
        }
        impl core::fmt::Debug for Ratio {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ratio")
                    .field("ratio", &self.ratio())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ratio {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Ratio {
                    ratio: super::vals::Ratio,
                }
                let proxy = Ratio {
                    ratio: self.ratio(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
            #[doc = "PDM_CLK = 32 MHz / 31 = 1.032 MHz. Nominal clock for RATIO=Ratio64."]
            pub const DEFAULT: Self = Self(0x0840_0000);
            #[doc = "PDM_CLK = 32 MHz / 30 = 1.067 MHz"]
            pub const _1067K: Self = Self(0x0880_0000);
            #[doc = "PDM_CLK = 32 MHz / 26 = 1.231 MHz"]
            pub const _1231K: Self = Self(0x0980_0000);
            #[doc = "PDM_CLK = 32 MHz / 25 = 1.280 MHz. Nominal clock for RATIO=Ratio80."]
            pub const _1280K: Self = Self(0x0a00_0000);
            #[doc = "PDM_CLK = 32 MHz / 24 = 1.333 MHz"]
            pub const _1333K: Self = Self(0x0a80_0000);
        }
        impl Freq {
            pub const fn from_bits(val: u32) -> Freq {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Freq {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0800_0000 => f.write_str("_1000K"),
                    0x0840_0000 => f.write_str("DEFAULT"),
                    0x0880_0000 => f.write_str("_1067K"),
                    0x0980_0000 => f.write_str("_1231K"),
                    0x0a00_0000 => f.write_str("_1280K"),
                    0x0a80_0000 => f.write_str("_1333K"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Freq {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0800_0000 => defmt::write!(f, "_1000K"),
                    0x0840_0000 => defmt::write!(f, "DEFAULT"),
                    0x0880_0000 => defmt::write!(f, "_1067K"),
                    0x0980_0000 => defmt::write!(f, "_1231K"),
                    0x0a00_0000 => defmt::write!(f, "_1280K"),
                    0x0a80_0000 => defmt::write!(f, "_1333K"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
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
            #[doc = "-20 dB gain adjustment (minimum)"]
            pub const MIN_GAIN: Self = Self(0x0);
            #[doc = "0 dB gain adjustment"]
            pub const DEFAULT_GAIN: Self = Self(0x28);
            #[doc = "+20 dB gain adjustment (maximum)"]
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
        impl core::fmt::Debug for Gain {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0 => f.write_str("MIN_GAIN"),
                    0x28 => f.write_str("DEFAULT_GAIN"),
                    0x50 => f.write_str("MAX_GAIN"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Gain {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0 => defmt::write!(f, "MIN_GAIN"),
                    0x28 => defmt::write!(f, "DEFAULT_GAIN"),
                    0x50 => defmt::write!(f, "MAX_GAIN"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Operation {
            #[doc = "Sample and store one pair (left + right) of 16-bit samples per RAM word R=\\[31:16\\]; L=\\[15:0\\]"]
            STEREO = 0x0,
            #[doc = "Sample and store two successive left samples (16 bits each) per RAM word L1=\\[31:16\\]; L0=\\[15:0\\]"]
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
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Ratio {
            #[doc = "Ratio of 64"]
            RATIO64 = 0x0,
            #[doc = "Ratio of 80"]
            RATIO80 = 0x01,
        }
        impl Ratio {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Ratio {
                unsafe { core::mem::transmute(val & 0x01) }
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
    }
}
pub mod power {
    #[doc = "LTE Modem"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ltemodem {
        ptr: *mut u8,
    }
    unsafe impl Send for Ltemodem {}
    unsafe impl Sync for Ltemodem {}
    impl Ltemodem {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start LTE modem"]
        #[inline(always)]
        pub const fn startn(self) -> crate::common::Reg<regs::Startn, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Force off LTE modem"]
        #[inline(always)]
        pub const fn forceoff(self) -> crate::common::Reg<regs::Forceoff, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    #[doc = "Power control 0"]
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
        #[doc = "Request forcing PWM mode in external DC/DC voltage regulator. (Drives FPWM_DCDC pin high or low depending on a setting in UICR)."]
        #[inline(always)]
        pub const fn tasks_pwmreqstart(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
        }
        #[doc = "Stop requesting forcing PWM mode in external DC/DC voltage regulator"]
        #[inline(always)]
        pub const fn tasks_pwmreqstop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
        }
        #[doc = "Enable constant latency mode."]
        #[inline(always)]
        pub const fn tasks_constlat(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
        }
        #[doc = "Enable low power mode (variable latency)"]
        #[inline(always)]
        pub const fn tasks_lowpwr(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
        }
        #[doc = "Subscribe configuration for task PWMREQSTART"]
        #[inline(always)]
        pub const fn subscribe_pwmreqstart(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
        }
        #[doc = "Subscribe configuration for task PWMREQSTOP"]
        #[inline(always)]
        pub const fn subscribe_pwmreqstop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
        }
        #[doc = "Subscribe configuration for task CONSTLAT"]
        #[inline(always)]
        pub const fn subscribe_constlat(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
        }
        #[doc = "Subscribe configuration for task LOWPWR"]
        #[inline(always)]
        pub const fn subscribe_lowpwr(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
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
        #[doc = "Publish configuration for event POFWARN"]
        #[inline(always)]
        pub const fn publish_pofwarn(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
        }
        #[doc = "Publish configuration for event SLEEPENTER"]
        #[inline(always)]
        pub const fn publish_sleepenter(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0194usize) as _) }
        }
        #[doc = "Publish configuration for event SLEEPEXIT"]
        #[inline(always)]
        pub const fn publish_sleepexit(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0198usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Reset reason"]
        #[inline(always)]
        pub const fn resetreas(self) -> crate::common::Reg<regs::Resetreas, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "Modem domain power status"]
        #[inline(always)]
        pub const fn powerstatus(self) -> crate::common::Reg<regs::Power, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0440usize) as _) }
        }
        #[doc = "Description collection: General purpose retention register"]
        #[inline(always)]
        pub const fn gpregret(
            self,
            n: usize,
        ) -> crate::common::Reg<regs::Gpregret, crate::common::RW> {
            assert!(n < 2usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize + n * 4usize) as _) }
        }
        #[doc = "LTE Modem"]
        #[inline(always)]
        pub const fn ltemodem(self) -> Ltemodem {
            unsafe { Ltemodem::from_ptr(self.ptr.add(0x0610usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Force off LTE modem"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Forceoff(pub u32);
        impl Forceoff {
            #[doc = "Force off LTE modem"]
            #[inline(always)]
            pub const fn forceoff(&self) -> super::vals::Forceoff {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Forceoff::from_bits(val as u8)
            }
            #[doc = "Force off LTE modem"]
            #[inline(always)]
            pub fn set_forceoff(&mut self, val: super::vals::Forceoff) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Forceoff {
            #[inline(always)]
            fn default() -> Forceoff {
                Forceoff(0)
            }
        }
        impl core::fmt::Debug for Forceoff {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Forceoff")
                    .field("forceoff", &self.forceoff())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Forceoff {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Forceoff {
                    forceoff: super::vals::Forceoff,
                }
                let proxy = Forceoff {
                    forceoff: self.forceoff(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description collection: General purpose retention register"]
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
                #[derive(defmt :: Format)]
                struct Gpregret {
                    gpregret: u8,
                }
                let proxy = Gpregret {
                    gpregret: self.gpregret(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Enable or disable interrupt for event POFWARN"]
            #[inline(always)]
            pub const fn pofwarn(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event POFWARN"]
            #[inline(always)]
            pub fn set_pofwarn(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable or disable interrupt for event SLEEPENTER"]
            #[inline(always)]
            pub const fn sleepenter(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event SLEEPENTER"]
            #[inline(always)]
            pub fn set_sleepenter(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable or disable interrupt for event SLEEPEXIT"]
            #[inline(always)]
            pub const fn sleepexit(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event SLEEPEXIT"]
            #[inline(always)]
            pub fn set_sleepexit(&mut self, val: bool) {
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
                #[derive(defmt :: Format)]
                struct Int {
                    pofwarn: bool,
                    sleepenter: bool,
                    sleepexit: bool,
                }
                let proxy = Int {
                    pofwarn: self.pofwarn(),
                    sleepenter: self.sleepenter(),
                    sleepexit: self.sleepexit(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Modem domain power status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Power(pub u32);
        impl Power {
            #[doc = "LTE modem domain status"]
            #[inline(always)]
            pub const fn ltemodem(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "LTE modem domain status"]
            #[inline(always)]
            pub fn set_ltemodem(&mut self, val: bool) {
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
                    .field("ltemodem", &self.ltemodem())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Power {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Power {
                    ltemodem: bool,
                }
                let proxy = Power {
                    ltemodem: self.ltemodem(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Reset reason"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Resetreas(pub u32);
        impl Resetreas {
            #[doc = "Reset from pin reset detected"]
            #[inline(always)]
            pub const fn resetpin(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from pin reset detected"]
            #[inline(always)]
            pub fn set_resetpin(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Reset from global watchdog detected"]
            #[inline(always)]
            pub const fn dog(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from global watchdog detected"]
            #[inline(always)]
            pub fn set_dog(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Reset due to wakeup from System OFF mode, when wakeup is triggered by DETECT signal from GPIO"]
            #[inline(always)]
            pub const fn off(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Reset due to wakeup from System OFF mode, when wakeup is triggered by DETECT signal from GPIO"]
            #[inline(always)]
            pub fn set_off(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Reset due to wakeup from System OFF mode, when wakeup is triggered by entering debug interface mode"]
            #[inline(always)]
            pub const fn dif(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Reset due to wakeup from System OFF mode, when wakeup is triggered by entering debug interface mode"]
            #[inline(always)]
            pub fn set_dif(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Reset from AIRCR.SYSRESETREQ detected"]
            #[inline(always)]
            pub const fn sreq(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from AIRCR.SYSRESETREQ detected"]
            #[inline(always)]
            pub fn set_sreq(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Reset from CPU lock-up detected"]
            #[inline(always)]
            pub const fn lockup(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from CPU lock-up detected"]
            #[inline(always)]
            pub fn set_lockup(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Reset triggered through CTRL-AP"]
            #[inline(always)]
            pub const fn ctrlap(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Reset triggered through CTRL-AP"]
            #[inline(always)]
            pub fn set_ctrlap(&mut self, val: bool) {
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
                    .field("off", &self.off())
                    .field("dif", &self.dif())
                    .field("sreq", &self.sreq())
                    .field("lockup", &self.lockup())
                    .field("ctrlap", &self.ctrlap())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Resetreas {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Resetreas {
                    resetpin: bool,
                    dog: bool,
                    off: bool,
                    dif: bool,
                    sreq: bool,
                    lockup: bool,
                    ctrlap: bool,
                }
                let proxy = Resetreas {
                    resetpin: self.resetpin(),
                    dog: self.dog(),
                    off: self.off(),
                    dif: self.dif(),
                    sreq: self.sreq(),
                    lockup: self.lockup(),
                    ctrlap: self.ctrlap(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Start LTE modem"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Startn(pub u32);
        impl Startn {
            #[doc = "Start LTE modem"]
            #[inline(always)]
            pub const fn startn(&self) -> super::vals::Startn {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Startn::from_bits(val as u8)
            }
            #[doc = "Start LTE modem"]
            #[inline(always)]
            pub fn set_startn(&mut self, val: super::vals::Startn) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Startn {
            #[inline(always)]
            fn default() -> Startn {
                Startn(0)
            }
        }
        impl core::fmt::Debug for Startn {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Startn")
                    .field("startn", &self.startn())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Startn {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Startn {
                    startn: super::vals::Startn,
                }
                let proxy = Startn {
                    startn: self.startn(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Forceoff {
            #[doc = "Release force off"]
            RELEASE = 0x0,
            #[doc = "Hold force off active"]
            HOLD = 0x01,
        }
        impl Forceoff {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Forceoff {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Forceoff {
            #[inline(always)]
            fn from(val: u8) -> Forceoff {
                Forceoff::from_bits(val)
            }
        }
        impl From<Forceoff> for u8 {
            #[inline(always)]
            fn from(val: Forceoff) -> u8 {
                Forceoff::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Startn {
            #[doc = "Start LTE modem"]
            START = 0x0,
            #[doc = "Hold LTE modem disabled"]
            HOLD = 0x01,
        }
        impl Startn {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Startn {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Startn {
            #[inline(always)]
            fn from(val: u8) -> Startn {
                Startn::from_bits(val)
            }
        }
        impl From<Startn> for u8 {
            #[inline(always)]
            fn from(val: Startn) -> u8 {
                Startn::to_bits(val)
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
    #[doc = "Pulse width modulation unit 0"]
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
        #[doc = "Subscribe configuration for task STOP"]
        #[inline(always)]
        pub const fn subscribe_stop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
        }
        #[doc = "Description collection: Subscribe configuration for task SEQSTART\\[n\\]"]
        #[inline(always)]
        pub const fn subscribe_seqstart(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            assert!(n < 2usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize + n * 4usize) as _) }
        }
        #[doc = "Subscribe configuration for task NEXTSTEP"]
        #[inline(always)]
        pub const fn subscribe_nextstep(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
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
        #[doc = "Publish configuration for event STOPPED"]
        #[inline(always)]
        pub const fn publish_stopped(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
        }
        #[doc = "Description collection: Publish configuration for event SEQSTARTED\\[n\\]"]
        #[inline(always)]
        pub const fn publish_seqstarted(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            assert!(n < 2usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Publish configuration for event SEQEND\\[n\\]"]
        #[inline(always)]
        pub const fn publish_seqend(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            assert!(n < 2usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize + n * 4usize) as _) }
        }
        #[doc = "Publish configuration for event PWMPERIODEND"]
        #[inline(always)]
        pub const fn publish_pwmperiodend(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0198usize) as _) }
        }
        #[doc = "Publish configuration for event LOOPSDONE"]
        #[inline(always)]
        pub const fn publish_loopsdone(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x019cusize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
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
        impl core::fmt::Debug for Cnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Cnt").field("cnt", &self.cnt()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Cnt {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Cnt {
                    cnt: super::vals::CntCnt,
                }
                let proxy = Cnt { cnt: self.cnt() };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Countertop {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Countertop")
                    .field("countertop", &self.countertop())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Countertop {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Countertop {
                    countertop: u16,
                }
                let proxy = Countertop {
                    countertop: self.countertop(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Decoder {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Decoder")
                    .field("load", &self.load())
                    .field("mode", &self.mode())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Decoder {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Decoder {
                    load: super::vals::Load,
                    mode: super::vals::Mode,
                }
                let proxy = Decoder {
                    load: self.load(),
                    mode: self.mode(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Enable {
                    enable: bool,
                }
                let proxy = Enable {
                    enable: self.enable(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Enddelay {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Enddelay")
                    .field("cnt", &self.cnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Enddelay {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Enddelay {
                    cnt: u32,
                }
                let proxy = Enddelay { cnt: self.cnt() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
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
                    .field("seqstarted0", &self.seqstarted0())
                    .field("seqstarted1", &self.seqstarted1())
                    .field("seqend0", &self.seqend0())
                    .field("seqend1", &self.seqend1())
                    .field("pwmperiodend", &self.pwmperiodend())
                    .field("loopsdone", &self.loopsdone())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    stopped: bool,
                    seqstarted0: bool,
                    seqstarted1: bool,
                    seqend0: bool,
                    seqend1: bool,
                    pwmperiodend: bool,
                    loopsdone: bool,
                }
                let proxy = Int {
                    stopped: self.stopped(),
                    seqstarted0: self.seqstarted0(),
                    seqstarted1: self.seqstarted1(),
                    seqend0: self.seqend0(),
                    seqend1: self.seqend1(),
                    pwmperiodend: self.pwmperiodend(),
                    loopsdone: self.loopsdone(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Loop {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Loop").field("cnt", &self.cnt()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Loop {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Loop {
                    cnt: super::vals::LoopCnt,
                }
                let proxy = Loop { cnt: self.cnt() };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Mode {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Mode")
                    .field("updown", &self.updown())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Mode {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Mode {
                    updown: super::vals::Updown,
                }
                let proxy = Mode {
                    updown: self.updown(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Prescaler {
                    prescaler: super::vals::Prescaler,
                }
                let proxy = Prescaler {
                    prescaler: self.prescaler(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Refresh {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Refresh").field("cnt", &self.cnt()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Refresh {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Refresh {
                    cnt: super::vals::RefreshCnt,
                }
                let proxy = Refresh { cnt: self.cnt() };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Shorts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Shorts")
                    .field("seqend0_stop", &self.seqend0_stop())
                    .field("seqend1_stop", &self.seqend1_stop())
                    .field("loopsdone_seqstart0", &self.loopsdone_seqstart0())
                    .field("loopsdone_seqstart1", &self.loopsdone_seqstart1())
                    .field("loopsdone_stop", &self.loopsdone_stop())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Shorts {
                    seqend0_stop: bool,
                    seqend1_stop: bool,
                    loopsdone_seqstart0: bool,
                    loopsdone_seqstart1: bool,
                    loopsdone_stop: bool,
                }
                let proxy = Shorts {
                    seqend0_stop: self.seqend0_stop(),
                    seqend1_stop: self.seqend1_stop(),
                    loopsdone_seqstart0: self.loopsdone_seqstart0(),
                    loopsdone_seqstart1: self.loopsdone_seqstart1(),
                    loopsdone_stop: self.loopsdone_stop(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for CntCnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0 => f.write_str("DISABLED"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for CntCnt {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0 => defmt::write!(f, "DISABLED"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
        impl core::fmt::Debug for LoopCnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0 => f.write_str("DISABLED"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for LoopCnt {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0 => defmt::write!(f, "DISABLED"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
        impl core::fmt::Debug for RefreshCnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0 => f.write_str("CONTINUOUS"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RefreshCnt {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0 => defmt::write!(f, "CONTINUOUS"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
pub mod regulators {
    #[doc = "Voltage regulators control 0"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Regulators {
        ptr: *mut u8,
    }
    unsafe impl Send for Regulators {}
    unsafe impl Sync for Regulators {}
    impl Regulators {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "System OFF register"]
        #[inline(always)]
        pub const fn systemoff(self) -> crate::common::Reg<regs::Systemoff, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "External power failure warning configuration"]
        #[inline(always)]
        pub const fn extpofcon(self) -> crate::common::Reg<regs::Extpofcon, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
        }
        #[doc = "Enable DC/DC mode of the main voltage regulator."]
        #[inline(always)]
        pub const fn dcdcen(self) -> crate::common::Reg<regs::Dcdcen, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0578usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Enable DC/DC mode of the main voltage regulator."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dcdcen(pub u32);
        impl Dcdcen {
            #[doc = "Enable DC/DC converter"]
            #[inline(always)]
            pub const fn dcdcen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable DC/DC converter"]
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
                #[derive(defmt :: Format)]
                struct Dcdcen {
                    dcdcen: bool,
                }
                let proxy = Dcdcen {
                    dcdcen: self.dcdcen(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "External power failure warning configuration"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Extpofcon(pub u32);
        impl Extpofcon {
            #[doc = "Enable or disable external power failure warning"]
            #[inline(always)]
            pub const fn pof(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable external power failure warning"]
            #[inline(always)]
            pub fn set_pof(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Extpofcon {
            #[inline(always)]
            fn default() -> Extpofcon {
                Extpofcon(0)
            }
        }
        impl core::fmt::Debug for Extpofcon {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Extpofcon")
                    .field("pof", &self.pof())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Extpofcon {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Extpofcon {
                    pof: bool,
                }
                let proxy = Extpofcon { pof: self.pof() };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Systemoff {
                    systemoff: bool,
                }
                let proxy = Systemoff {
                    systemoff: self.systemoff(),
                };
                defmt::write!(f, "{}", proxy)
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop RTC counter"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Clear RTC counter"]
        #[inline(always)]
        pub const fn tasks_clear(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Set counter to 0xFFFFF0"]
        #[inline(always)]
        pub const fn tasks_trigovrflw(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Subscribe configuration for task START"]
        #[inline(always)]
        pub const fn subscribe_start(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
        }
        #[doc = "Subscribe configuration for task STOP"]
        #[inline(always)]
        pub const fn subscribe_stop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
        }
        #[doc = "Subscribe configuration for task CLEAR"]
        #[inline(always)]
        pub const fn subscribe_clear(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
        }
        #[doc = "Subscribe configuration for task TRIGOVRFLW"]
        #[inline(always)]
        pub const fn subscribe_trigovrflw(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
        }
        #[doc = "Event on counter increment"]
        #[inline(always)]
        pub const fn events_tick(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Event on counter overflow"]
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
        #[doc = "Publish configuration for event TICK"]
        #[inline(always)]
        pub const fn publish_tick(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
        }
        #[doc = "Publish configuration for event OVRFLW"]
        #[inline(always)]
        pub const fn publish_ovrflw(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
        }
        #[doc = "Description collection: Publish configuration for event COMPARE\\[n\\]"]
        #[inline(always)]
        pub const fn publish_compare(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c0usize + n * 4usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Enable or disable event routing"]
        #[inline(always)]
        pub const fn evten(self) -> crate::common::Reg<regs::Evt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0340usize) as _) }
        }
        #[doc = "Enable event routing"]
        #[inline(always)]
        pub const fn evtenset(self) -> crate::common::Reg<regs::Evt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0344usize) as _) }
        }
        #[doc = "Disable event routing"]
        #[inline(always)]
        pub const fn evtenclr(self) -> crate::common::Reg<regs::Evt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0348usize) as _) }
        }
        #[doc = "Current counter value"]
        #[inline(always)]
        pub const fn counter(self) -> crate::common::Reg<regs::Counter, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "12-bit prescaler for counter frequency (32768/(PRESCALER+1)). Must be written when RTC is stopped."]
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
                #[derive(defmt :: Format)]
                struct Cc {
                    compare: u32,
                }
                let proxy = Cc {
                    compare: self.compare(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Current counter value"]
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
                #[derive(defmt :: Format)]
                struct Counter {
                    counter: u32,
                }
                let proxy = Counter {
                    counter: self.counter(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Enable or disable event routing"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Evt(pub u32);
        impl Evt {
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
            pub const fn compare(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 16usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable event routing for event COMPARE\\[0\\]"]
            #[inline(always)]
            pub fn set_compare(&mut self, n: usize, val: bool) {
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
                    .field(
                        "compare",
                        &[
                            self.compare(0usize),
                            self.compare(1usize),
                            self.compare(2usize),
                            self.compare(3usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Evt {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Evt {
                    tick: bool,
                    ovrflw: bool,
                    compare: [bool; 4usize],
                }
                let proxy = Evt {
                    tick: self.tick(),
                    ovrflw: self.ovrflw(),
                    compare: [
                        self.compare(0usize),
                        self.compare(1usize),
                        self.compare(2usize),
                        self.compare(3usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
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
            pub const fn compare(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 16usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event COMPARE\\[0\\]"]
            #[inline(always)]
            pub fn set_compare(&mut self, n: usize, val: bool) {
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
                    .field(
                        "compare",
                        &[
                            self.compare(0usize),
                            self.compare(1usize),
                            self.compare(2usize),
                            self.compare(3usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    tick: bool,
                    ovrflw: bool,
                    compare: [bool; 4usize],
                }
                let proxy = Int {
                    tick: self.tick(),
                    ovrflw: self.ovrflw(),
                    compare: [
                        self.compare(0usize),
                        self.compare(1usize),
                        self.compare(2usize),
                        self.compare(3usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "12-bit prescaler for counter frequency (32768/(PRESCALER+1)). Must be written when RTC is stopped."]
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
                #[derive(defmt :: Format)]
                struct Prescaler {
                    prescaler: u16,
                }
                let proxy = Prescaler {
                    prescaler: self.prescaler(),
                };
                defmt::write!(f, "{}", proxy)
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
        pub const fn limith(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster: Last results is equal or below CH\\[n\\].LIMIT.LOW"]
        #[inline(always)]
        pub const fn limitl(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    #[doc = "Publish configuration for events"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PublishCh {
        ptr: *mut u8,
    }
    unsafe impl Send for PublishCh {}
    unsafe impl Sync for PublishCh {}
    impl PublishCh {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster: Publish configuration for event CH\\[n\\].LIMITH"]
        #[inline(always)]
        pub const fn limith(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster: Publish configuration for event CH\\[n\\].LIMITL"]
        #[inline(always)]
        pub const fn limitl(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
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
    #[doc = "Analog to Digital Converter 0"]
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
        #[doc = "Subscribe configuration for task START"]
        #[inline(always)]
        pub const fn subscribe_start(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
        }
        #[doc = "Subscribe configuration for task SAMPLE"]
        #[inline(always)]
        pub const fn subscribe_sample(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
        }
        #[doc = "Subscribe configuration for task STOP"]
        #[inline(always)]
        pub const fn subscribe_stop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
        }
        #[doc = "Subscribe configuration for task CALIBRATEOFFSET"]
        #[inline(always)]
        pub const fn subscribe_calibrateoffset(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
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
        #[doc = "Publish configuration for event STARTED"]
        #[inline(always)]
        pub const fn publish_started(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
        }
        #[doc = "Publish configuration for event END"]
        #[inline(always)]
        pub const fn publish_end(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
        }
        #[doc = "Publish configuration for event DONE"]
        #[inline(always)]
        pub const fn publish_done(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
        }
        #[doc = "Publish configuration for event RESULTDONE"]
        #[inline(always)]
        pub const fn publish_resultdone(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x018cusize) as _) }
        }
        #[doc = "Publish configuration for event CALIBRATEDONE"]
        #[inline(always)]
        pub const fn publish_calibratedone(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize) as _) }
        }
        #[doc = "Publish configuration for event STOPPED"]
        #[inline(always)]
        pub const fn publish_stopped(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0194usize) as _) }
        }
        #[doc = "Publish configuration for events"]
        #[inline(always)]
        pub const fn publish_ch(self, n: usize) -> PublishCh {
            assert!(n < 8usize);
            unsafe { PublishCh::from_ptr(self.ptr.add(0x0198usize + n * 8usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
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
                #[derive(defmt :: Format)]
                struct Amount {
                    amount: u16,
                }
                let proxy = Amount {
                    amount: self.amount(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Config {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Config")
                    .field("resp", &self.resp())
                    .field("resn", &self.resn())
                    .field("gain", &self.gain())
                    .field("refsel", &self.refsel())
                    .field("tacq", &self.tacq())
                    .field("mode", &self.mode())
                    .field("burst", &self.burst())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Config {
                    resp: super::vals::Resp,
                    resn: super::vals::Resn,
                    gain: super::vals::Gain,
                    refsel: super::vals::Refsel,
                    tacq: super::vals::Tacq,
                    mode: super::vals::ConfigMode,
                    burst: bool,
                }
                let proxy = Config {
                    resp: self.resp(),
                    resn: self.resn(),
                    gain: self.gain(),
                    refsel: self.refsel(),
                    tacq: self.tacq(),
                    mode: self.mode(),
                    burst: self.burst(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Enable {
                    enable: bool,
                }
                let proxy = Enable {
                    enable: self.enable(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
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
            pub const fn chlimith(&self, n: usize) -> bool {
                assert!(n < 8usize);
                let offs = 6usize + n * 2usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event CH0LIMITH"]
            #[inline(always)]
            pub fn set_chlimith(&mut self, n: usize, val: bool) {
                assert!(n < 8usize);
                let offs = 6usize + n * 2usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Enable or disable interrupt for event CH0LIMITL"]
            #[inline(always)]
            pub const fn chlimitl(&self, n: usize) -> bool {
                assert!(n < 8usize);
                let offs = 7usize + n * 2usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event CH0LIMITL"]
            #[inline(always)]
            pub fn set_chlimitl(&mut self, n: usize, val: bool) {
                assert!(n < 8usize);
                let offs = 7usize + n * 2usize;
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
                    .field("started", &self.started())
                    .field("end", &self.end())
                    .field("done", &self.done())
                    .field("resultdone", &self.resultdone())
                    .field("calibratedone", &self.calibratedone())
                    .field("stopped", &self.stopped())
                    .field(
                        "chlimith",
                        &[
                            self.chlimith(0usize),
                            self.chlimith(1usize),
                            self.chlimith(2usize),
                            self.chlimith(3usize),
                            self.chlimith(4usize),
                            self.chlimith(5usize),
                            self.chlimith(6usize),
                            self.chlimith(7usize),
                        ],
                    )
                    .field(
                        "chlimitl",
                        &[
                            self.chlimitl(0usize),
                            self.chlimitl(1usize),
                            self.chlimitl(2usize),
                            self.chlimitl(3usize),
                            self.chlimitl(4usize),
                            self.chlimitl(5usize),
                            self.chlimitl(6usize),
                            self.chlimitl(7usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    started: bool,
                    end: bool,
                    done: bool,
                    resultdone: bool,
                    calibratedone: bool,
                    stopped: bool,
                    chlimith: [bool; 8usize],
                    chlimitl: [bool; 8usize],
                }
                let proxy = Int {
                    started: self.started(),
                    end: self.end(),
                    done: self.done(),
                    resultdone: self.resultdone(),
                    calibratedone: self.calibratedone(),
                    stopped: self.stopped(),
                    chlimith: [
                        self.chlimith(0usize),
                        self.chlimith(1usize),
                        self.chlimith(2usize),
                        self.chlimith(3usize),
                        self.chlimith(4usize),
                        self.chlimith(5usize),
                        self.chlimith(6usize),
                        self.chlimith(7usize),
                    ],
                    chlimitl: [
                        self.chlimitl(0usize),
                        self.chlimitl(1usize),
                        self.chlimitl(2usize),
                        self.chlimitl(3usize),
                        self.chlimitl(4usize),
                        self.chlimitl(5usize),
                        self.chlimitl(6usize),
                        self.chlimitl(7usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Limit {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Limit")
                    .field("low", &self.low())
                    .field("high", &self.high())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Limit {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Limit {
                    low: u16,
                    high: u16,
                }
                let proxy = Limit {
                    low: self.low(),
                    high: self.high(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Maxcnt {
                    maxcnt: u16,
                }
                let proxy = Maxcnt {
                    maxcnt: self.maxcnt(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Oversample {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Oversample")
                    .field("oversample", &self.oversample())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Oversample {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Oversample {
                    oversample: super::vals::Oversample,
                }
                let proxy = Oversample {
                    oversample: self.oversample(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Pseln {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Pseln")
                    .field("pseln", &self.pseln())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Pseln {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Pseln {
                    pseln: super::vals::Psel,
                }
                let proxy = Pseln {
                    pseln: self.pseln(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Pselp {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Pselp")
                    .field("pselp", &self.pselp())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Pselp {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Pselp {
                    pselp: super::vals::Psel,
                }
                let proxy = Pselp {
                    pselp: self.pselp(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Resolution {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Resolution")
                    .field("val", &self.val())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Resolution {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Resolution {
                    val: super::vals::Val,
                }
                let proxy = Resolution { val: self.val() };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Samplerate {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Samplerate")
                    .field("cc", &self.cc())
                    .field("mode", &self.mode())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Samplerate {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Samplerate {
                    cc: u16,
                    mode: super::vals::SamplerateMode,
                }
                let proxy = Samplerate {
                    cc: self.cc(),
                    mode: self.mode(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Status {
                    status: super::vals::Status,
                }
                let proxy = Status {
                    status: self.status(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
            #[doc = "VDD_GPIO"]
            VDD_GPIO = 0x09,
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Refsel {
            #[doc = "Internal reference (0.6 V)"]
            INTERNAL = 0x0,
            #[doc = "VDD_GPIO/4 as reference"]
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Resn {
            #[doc = "Bypass resistor ladder"]
            BYPASS = 0x0,
            #[doc = "Pull-down to GND"]
            PULLDOWN = 0x01,
            #[doc = "Pull-up to VDD_GPIO"]
            PULLUP = 0x02,
            #[doc = "Set input at VDD_GPIO/2"]
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Resp {
            #[doc = "Bypass resistor ladder"]
            BYPASS = 0x0,
            #[doc = "Pull-down to GND"]
            PULLDOWN = 0x01,
            #[doc = "Pull-up to VDD_GPIO"]
            PULLUP = 0x02,
            #[doc = "Set input at VDD_GPIO/2"]
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Status {
            #[doc = "ADC is ready. No on-going conversion."]
            READY = 0x0,
            #[doc = "ADC is busy. Single conversion in progress."]
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
        impl core::fmt::Debug for Psel {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Psel")
                    .field("pin", &self.pin())
                    .field("connect", &self.connect())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Psel {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Psel {
                    pin: u8,
                    connect: super::vals::Connect,
                }
                let proxy = Psel {
                    pin: self.pin(),
                    connect: self.connect(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Publish configuration for event HFCLKSTARTED"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Publish(pub u32);
        impl Publish {
            #[doc = "DPPI channel that event HFCLKSTARTED will publish to"]
            #[inline(always)]
            pub const fn chidx(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "DPPI channel that event HFCLKSTARTED will publish to"]
            #[inline(always)]
            pub fn set_chidx(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[inline(always)]
            pub const fn en(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub fn set_en(&mut self, val: bool) {
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
                #[derive(defmt :: Format)]
                struct Publish {
                    chidx: u8,
                    en: bool,
                }
                let proxy = Publish {
                    chidx: self.chidx(),
                    en: self.en(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Subscribe configuration for task HFCLKSTART"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Subscribe(pub u32);
        impl Subscribe {
            #[doc = "DPPI channel that task HFCLKSTART will subscribe to"]
            #[inline(always)]
            pub const fn chidx(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "DPPI channel that task HFCLKSTART will subscribe to"]
            #[inline(always)]
            pub fn set_chidx(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[inline(always)]
            pub const fn en(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub fn set_en(&mut self, val: bool) {
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
                #[derive(defmt :: Format)]
                struct Subscribe {
                    chidx: u8,
                    en: bool,
                }
                let proxy = Subscribe {
                    chidx: self.chidx(),
                    en: self.en(),
                };
                defmt::write!(f, "{}", proxy)
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
        #[doc = "Subscribe configuration for task START"]
        #[inline(always)]
        pub const fn subscribe_start(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
        }
        #[doc = "Subscribe configuration for task STOP"]
        #[inline(always)]
        pub const fn subscribe_stop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
        }
        #[doc = "Subscribe configuration for task SUSPEND"]
        #[inline(always)]
        pub const fn subscribe_suspend(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
        }
        #[doc = "Subscribe configuration for task RESUME"]
        #[inline(always)]
        pub const fn subscribe_resume(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
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
        #[doc = "Publish configuration for event STOPPED"]
        #[inline(always)]
        pub const fn publish_stopped(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
        }
        #[doc = "Publish configuration for event ENDRX"]
        #[inline(always)]
        pub const fn publish_endrx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize) as _) }
        }
        #[doc = "Publish configuration for event END"]
        #[inline(always)]
        pub const fn publish_end(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0198usize) as _) }
        }
        #[doc = "Publish configuration for event ENDTX"]
        #[inline(always)]
        pub const fn publish_endtx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a0usize) as _) }
        }
        #[doc = "Publish configuration for event STARTED"]
        #[inline(always)]
        pub const fn publish_started(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01ccusize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
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
        #[doc = "Over-read character. Character clocked out in case an over-read of the TXD buffer."]
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
                #[derive(defmt :: Format)]
                struct Config {
                    order: super::vals::Order,
                    cpha: super::vals::Cpha,
                    cpol: super::vals::Cpol,
                }
                let proxy = Config {
                    order: self.order(),
                    cpha: self.cpha(),
                    cpol: self.cpol(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Enable {
                    enable: super::vals::Enable,
                }
                let proxy = Enable {
                    enable: self.enable(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Frequency {
                    frequency: super::vals::Frequency,
                }
                let proxy = Frequency {
                    frequency: self.frequency(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
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
                #[derive(defmt :: Format)]
                struct Int {
                    stopped: bool,
                    endrx: bool,
                    end: bool,
                    endtx: bool,
                    started: bool,
                }
                let proxy = Int {
                    stopped: self.stopped(),
                    endrx: self.endrx(),
                    end: self.end(),
                    endtx: self.endtx(),
                    started: self.started(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Over-read character. Character clocked out in case an over-read of the TXD buffer."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Orc(pub u32);
        impl Orc {
            #[doc = "Over-read character. Character clocked out in case an over-read of the TXD buffer."]
            #[inline(always)]
            pub const fn orc(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Over-read character. Character clocked out in case an over-read of the TXD buffer."]
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
        impl core::fmt::Debug for Orc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Orc").field("orc", &self.orc()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Orc {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Orc {
                    orc: u8,
                }
                let proxy = Orc { orc: self.orc() };
                defmt::write!(f, "{}", proxy)
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
                let val = (self.0 >> 0usize) & 0x1fff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
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
                #[derive(defmt :: Format)]
                struct RxdAmount {
                    amount: u16,
                }
                let proxy = RxdAmount {
                    amount: self.amount(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct RxdList {
                    list: super::vals::RxdListList,
                }
                let proxy = RxdList { list: self.list() };
                defmt::write!(f, "{}", proxy)
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
                let val = (self.0 >> 0usize) & 0x1fff;
                val as u16
            }
            #[doc = "Maximum number of bytes in receive buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
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
                #[derive(defmt :: Format)]
                struct RxdMaxcnt {
                    maxcnt: u16,
                }
                let proxy = RxdMaxcnt {
                    maxcnt: self.maxcnt(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Shorts {
                    end_start: bool,
                }
                let proxy = Shorts {
                    end_start: self.end_start(),
                };
                defmt::write!(f, "{}", proxy)
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
                let val = (self.0 >> 0usize) & 0x1fff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
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
                #[derive(defmt :: Format)]
                struct TxdAmount {
                    amount: u16,
                }
                let proxy = TxdAmount {
                    amount: self.amount(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct TxdList {
                    list: super::vals::TxdListList,
                }
                let proxy = TxdList { list: self.list() };
                defmt::write!(f, "{}", proxy)
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
                let val = (self.0 >> 0usize) & 0x1fff;
                val as u16
            }
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
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
                #[derive(defmt :: Format)]
                struct TxdMaxcnt {
                    maxcnt: u16,
                }
                let proxy = TxdMaxcnt {
                    maxcnt: self.maxcnt(),
                };
                defmt::write!(f, "{}", proxy)
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
        #[doc = "Subscribe configuration for task ACQUIRE"]
        #[inline(always)]
        pub const fn subscribe_acquire(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
        }
        #[doc = "Subscribe configuration for task RELEASE"]
        #[inline(always)]
        pub const fn subscribe_release(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
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
        #[doc = "Publish configuration for event END"]
        #[inline(always)]
        pub const fn publish_end(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
        }
        #[doc = "Publish configuration for event ENDRX"]
        #[inline(always)]
        pub const fn publish_endrx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize) as _) }
        }
        #[doc = "Publish configuration for event ACQUIRED"]
        #[inline(always)]
        pub const fn publish_acquired(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a8usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
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
                #[derive(defmt :: Format)]
                struct Config {
                    order: super::vals::Order,
                    cpha: super::vals::Cpha,
                    cpol: super::vals::Cpol,
                }
                let proxy = Config {
                    order: self.order(),
                    cpha: self.cpha(),
                    cpol: self.cpol(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Def {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Def").field("def", &self.def()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Def {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Def {
                    def: u8,
                }
                let proxy = Def { def: self.def() };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Enable {
                    enable: super::vals::Enable,
                }
                let proxy = Enable {
                    enable: self.enable(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
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
                #[derive(defmt :: Format)]
                struct Int {
                    end: bool,
                    endrx: bool,
                    acquired: bool,
                }
                let proxy = Int {
                    end: self.end(),
                    endrx: self.endrx(),
                    acquired: self.acquired(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Orc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Orc").field("orc", &self.orc()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Orc {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Orc {
                    orc: u8,
                }
                let proxy = Orc { orc: self.orc() };
                defmt::write!(f, "{}", proxy)
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
                let val = (self.0 >> 0usize) & 0x1fff;
                val as u16
            }
            #[doc = "Number of bytes received in the last granted transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
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
                #[derive(defmt :: Format)]
                struct RxdAmount {
                    amount: u16,
                }
                let proxy = RxdAmount {
                    amount: self.amount(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct RxdList {
                    list: super::vals::RxdListList,
                }
                let proxy = RxdList { list: self.list() };
                defmt::write!(f, "{}", proxy)
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
                let val = (self.0 >> 0usize) & 0x1fff;
                val as u16
            }
            #[doc = "Maximum number of bytes in receive buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
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
                #[derive(defmt :: Format)]
                struct RxdMaxcnt {
                    maxcnt: u16,
                }
                let proxy = RxdMaxcnt {
                    maxcnt: self.maxcnt(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Semstat {
                    semstat: super::vals::Semstat,
                }
                let proxy = Semstat {
                    semstat: self.semstat(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Shorts {
                    end_acquire: bool,
                }
                let proxy = Shorts {
                    end_acquire: self.end_acquire(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Status {
                    overread: bool,
                    overflow: bool,
                }
                let proxy = Status {
                    overread: self.overread(),
                    overflow: self.overflow(),
                };
                defmt::write!(f, "{}", proxy)
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
                let val = (self.0 >> 0usize) & 0x1fff;
                val as u16
            }
            #[doc = "Number of bytes transmitted in last granted transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
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
                #[derive(defmt :: Format)]
                struct TxdAmount {
                    amount: u16,
                }
                let proxy = TxdAmount {
                    amount: self.amount(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct TxdList {
                    list: super::vals::TxdListList,
                }
                let proxy = TxdList { list: self.list() };
                defmt::write!(f, "{}", proxy)
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
                let val = (self.0 >> 0usize) & 0x1fff;
                val as u16
            }
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
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
                #[derive(defmt :: Format)]
                struct TxdMaxcnt {
                    maxcnt: u16,
                }
                let proxy = TxdMaxcnt {
                    maxcnt: self.maxcnt(),
                };
                defmt::write!(f, "{}", proxy)
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
pub mod spu {
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dppi {
        ptr: *mut u8,
    }
    unsafe impl Send for Dppi {}
    unsafe impl Sync for Dppi {}
    impl Dppi {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster: Select between secure and non-secure attribute for the DPPI channels."]
        #[inline(always)]
        pub const fn perm(self) -> crate::common::Reg<regs::DppiPerm, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster: Prevent further modification of the corresponding PERM register"]
        #[inline(always)]
        pub const fn lock(self) -> crate::common::Reg<regs::DppiLock, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Extdomain {
        ptr: *mut u8,
    }
    unsafe impl Send for Extdomain {}
    unsafe impl Sync for Extdomain {}
    impl Extdomain {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster: Access for bus access generated from the external domain n List capabilities of the external domain n"]
        #[inline(always)]
        pub const fn perm(self) -> crate::common::Reg<regs::ExtdomainPerm, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Flashnsc {
        ptr: *mut u8,
    }
    unsafe impl Send for Flashnsc {}
    unsafe impl Sync for Flashnsc {}
    impl Flashnsc {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster: Define which flash region can contain the non-secure callable (NSC) region n"]
        #[inline(always)]
        pub const fn region(self) -> crate::common::Reg<regs::FlashnscRegion, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster: Define the size of the non-secure callable (NSC) region n"]
        #[inline(always)]
        pub const fn size(self) -> crate::common::Reg<regs::FlashnscSize, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Flashregion {
        ptr: *mut u8,
    }
    unsafe impl Send for Flashregion {}
    unsafe impl Sync for Flashregion {}
    impl Flashregion {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster: Access permissions for flash region n"]
        #[inline(always)]
        pub const fn perm(self) -> crate::common::Reg<regs::FlashregionPerm, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpioport {
        ptr: *mut u8,
    }
    unsafe impl Send for Gpioport {}
    unsafe impl Sync for Gpioport {}
    impl Gpioport {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster: Select between secure and non-secure attribute for pins 0 to 31 of port n."]
        #[inline(always)]
        pub const fn perm(self) -> crate::common::Reg<regs::GpioportPerm, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster: Prevent further modification of the corresponding PERM register"]
        #[inline(always)]
        pub const fn lock(self) -> crate::common::Reg<regs::GpioportLock, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Periphid {
        ptr: *mut u8,
    }
    unsafe impl Send for Periphid {}
    unsafe impl Sync for Periphid {}
    impl Periphid {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster: List capabilities and access permissions for the peripheral with ID n"]
        #[inline(always)]
        pub const fn perm(self) -> crate::common::Reg<regs::PeriphidPerm, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ramnsc {
        ptr: *mut u8,
    }
    unsafe impl Send for Ramnsc {}
    unsafe impl Sync for Ramnsc {}
    impl Ramnsc {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster: Define which RAM region can contain the non-secure callable (NSC) region n"]
        #[inline(always)]
        pub const fn region(self) -> crate::common::Reg<regs::RamnscRegion, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster: Define the size of the non-secure callable (NSC) region n"]
        #[inline(always)]
        pub const fn size(self) -> crate::common::Reg<regs::RamnscSize, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ramregion {
        ptr: *mut u8,
    }
    unsafe impl Send for Ramregion {}
    unsafe impl Sync for Ramregion {}
    impl Ramregion {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster: Access permissions for RAM region n"]
        #[inline(always)]
        pub const fn perm(self) -> crate::common::Reg<regs::RamregionPerm, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
    }
    #[doc = "System protection unit"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Spu {
        ptr: *mut u8,
    }
    unsafe impl Send for Spu {}
    unsafe impl Sync for Spu {}
    impl Spu {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "A security violation has been detected for the RAM memory space"]
        #[inline(always)]
        pub const fn events_ramaccerr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "A security violation has been detected for the flash memory space"]
        #[inline(always)]
        pub const fn events_flashaccerr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "A security violation has been detected on one or several peripherals"]
        #[inline(always)]
        pub const fn events_periphaccerr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "Publish configuration for event RAMACCERR"]
        #[inline(always)]
        pub const fn publish_ramaccerr(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
        }
        #[doc = "Publish configuration for event FLASHACCERR"]
        #[inline(always)]
        pub const fn publish_flashaccerr(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
        }
        #[doc = "Publish configuration for event PERIPHACCERR"]
        #[inline(always)]
        pub const fn publish_periphaccerr(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Show implemented features for the current device"]
        #[inline(always)]
        pub const fn cap(self) -> crate::common::Reg<regs::Cap, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn extdomain(self, n: usize) -> Extdomain {
            assert!(n < 1usize);
            unsafe { Extdomain::from_ptr(self.ptr.add(0x0440usize + n * 4usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn dppi(self, n: usize) -> Dppi {
            assert!(n < 1usize);
            unsafe { Dppi::from_ptr(self.ptr.add(0x0480usize + n * 8usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn gpioport(self, n: usize) -> Gpioport {
            assert!(n < 1usize);
            unsafe { Gpioport::from_ptr(self.ptr.add(0x04c0usize + n * 8usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn flashnsc(self, n: usize) -> Flashnsc {
            assert!(n < 2usize);
            unsafe { Flashnsc::from_ptr(self.ptr.add(0x0500usize + n * 8usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn ramnsc(self, n: usize) -> Ramnsc {
            assert!(n < 2usize);
            unsafe { Ramnsc::from_ptr(self.ptr.add(0x0540usize + n * 8usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn flashregion(self, n: usize) -> Flashregion {
            assert!(n < 32usize);
            unsafe { Flashregion::from_ptr(self.ptr.add(0x0600usize + n * 4usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn ramregion(self, n: usize) -> Ramregion {
            assert!(n < 32usize);
            unsafe { Ramregion::from_ptr(self.ptr.add(0x0700usize + n * 4usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn periphid(self, n: usize) -> Periphid {
            assert!(n < 67usize);
            unsafe { Periphid::from_ptr(self.ptr.add(0x0800usize + n * 4usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Show implemented features for the current device"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cap(pub u32);
        impl Cap {
            #[doc = "Show ARM TrustZone status"]
            #[inline(always)]
            pub const fn tzm(&self) -> super::vals::Tzm {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Tzm::from_bits(val as u8)
            }
            #[doc = "Show ARM TrustZone status"]
            #[inline(always)]
            pub fn set_tzm(&mut self, val: super::vals::Tzm) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Cap {
            #[inline(always)]
            fn default() -> Cap {
                Cap(0)
            }
        }
        impl core::fmt::Debug for Cap {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Cap").field("tzm", &self.tzm()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Cap {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Cap {
                    tzm: super::vals::Tzm,
                }
                let proxy = Cap { tzm: self.tzm() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description cluster: Prevent further modification of the corresponding PERM register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DppiLock(pub u32);
        impl DppiLock {
            #[inline(always)]
            pub const fn lock(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub fn set_lock(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for DppiLock {
            #[inline(always)]
            fn default() -> DppiLock {
                DppiLock(0)
            }
        }
        impl core::fmt::Debug for DppiLock {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DppiLock")
                    .field("lock", &self.lock())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DppiLock {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct DppiLock {
                    lock: bool,
                }
                let proxy = DppiLock { lock: self.lock() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description cluster: Select between secure and non-secure attribute for the DPPI channels."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DppiPerm(pub u32);
        impl DppiPerm {
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub const fn channel0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub fn set_channel0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub const fn channel1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub fn set_channel1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub const fn channel2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub fn set_channel2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub const fn channel3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub fn set_channel3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub const fn channel4(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub fn set_channel4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub const fn channel5(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub fn set_channel5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub const fn channel6(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub fn set_channel6(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub const fn channel7(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub fn set_channel7(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub const fn channel8(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub fn set_channel8(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub const fn channel9(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub fn set_channel9(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub const fn channel10(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub fn set_channel10(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub const fn channel11(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub fn set_channel11(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub const fn channel12(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub fn set_channel12(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub const fn channel13(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub fn set_channel13(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub const fn channel14(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub fn set_channel14(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub const fn channel15(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute."]
            #[inline(always)]
            pub fn set_channel15(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
        }
        impl Default for DppiPerm {
            #[inline(always)]
            fn default() -> DppiPerm {
                DppiPerm(0)
            }
        }
        impl core::fmt::Debug for DppiPerm {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DppiPerm")
                    .field("channel0", &self.channel0())
                    .field("channel1", &self.channel1())
                    .field("channel2", &self.channel2())
                    .field("channel3", &self.channel3())
                    .field("channel4", &self.channel4())
                    .field("channel5", &self.channel5())
                    .field("channel6", &self.channel6())
                    .field("channel7", &self.channel7())
                    .field("channel8", &self.channel8())
                    .field("channel9", &self.channel9())
                    .field("channel10", &self.channel10())
                    .field("channel11", &self.channel11())
                    .field("channel12", &self.channel12())
                    .field("channel13", &self.channel13())
                    .field("channel14", &self.channel14())
                    .field("channel15", &self.channel15())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DppiPerm {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct DppiPerm {
                    channel0: bool,
                    channel1: bool,
                    channel2: bool,
                    channel3: bool,
                    channel4: bool,
                    channel5: bool,
                    channel6: bool,
                    channel7: bool,
                    channel8: bool,
                    channel9: bool,
                    channel10: bool,
                    channel11: bool,
                    channel12: bool,
                    channel13: bool,
                    channel14: bool,
                    channel15: bool,
                }
                let proxy = DppiPerm {
                    channel0: self.channel0(),
                    channel1: self.channel1(),
                    channel2: self.channel2(),
                    channel3: self.channel3(),
                    channel4: self.channel4(),
                    channel5: self.channel5(),
                    channel6: self.channel6(),
                    channel7: self.channel7(),
                    channel8: self.channel8(),
                    channel9: self.channel9(),
                    channel10: self.channel10(),
                    channel11: self.channel11(),
                    channel12: self.channel12(),
                    channel13: self.channel13(),
                    channel14: self.channel14(),
                    channel15: self.channel15(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description cluster: Access for bus access generated from the external domain n List capabilities of the external domain n"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct ExtdomainPerm(pub u32);
        impl ExtdomainPerm {
            #[doc = "Define configuration capabilities for TrustZone Cortex-M secure attribute"]
            #[inline(always)]
            pub const fn securemapping(&self) -> super::vals::ExtdomainPermSecuremapping {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::ExtdomainPermSecuremapping::from_bits(val as u8)
            }
            #[doc = "Define configuration capabilities for TrustZone Cortex-M secure attribute"]
            #[inline(always)]
            pub fn set_securemapping(&mut self, val: super::vals::ExtdomainPermSecuremapping) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
            #[doc = "Peripheral security mapping"]
            #[inline(always)]
            pub const fn secattr(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Peripheral security mapping"]
            #[inline(always)]
            pub fn set_secattr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[inline(always)]
            pub const fn lock(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub fn set_lock(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for ExtdomainPerm {
            #[inline(always)]
            fn default() -> ExtdomainPerm {
                ExtdomainPerm(0)
            }
        }
        impl core::fmt::Debug for ExtdomainPerm {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("ExtdomainPerm")
                    .field("securemapping", &self.securemapping())
                    .field("secattr", &self.secattr())
                    .field("lock", &self.lock())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for ExtdomainPerm {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct ExtdomainPerm {
                    securemapping: super::vals::ExtdomainPermSecuremapping,
                    secattr: bool,
                    lock: bool,
                }
                let proxy = ExtdomainPerm {
                    securemapping: self.securemapping(),
                    secattr: self.secattr(),
                    lock: self.lock(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description cluster: Define which flash region can contain the non-secure callable (NSC) region n"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct FlashnscRegion(pub u32);
        impl FlashnscRegion {
            #[doc = "Region number"]
            #[inline(always)]
            pub const fn region(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x1f;
                val as u8
            }
            #[doc = "Region number"]
            #[inline(always)]
            pub fn set_region(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
            }
            #[inline(always)]
            pub const fn lock(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub fn set_lock(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for FlashnscRegion {
            #[inline(always)]
            fn default() -> FlashnscRegion {
                FlashnscRegion(0)
            }
        }
        impl core::fmt::Debug for FlashnscRegion {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("FlashnscRegion")
                    .field("region", &self.region())
                    .field("lock", &self.lock())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for FlashnscRegion {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct FlashnscRegion {
                    region: u8,
                    lock: bool,
                }
                let proxy = FlashnscRegion {
                    region: self.region(),
                    lock: self.lock(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description cluster: Define the size of the non-secure callable (NSC) region n"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct FlashnscSize(pub u32);
        impl FlashnscSize {
            #[doc = "Size of the non-secure callable (NSC) region n"]
            #[inline(always)]
            pub const fn size(&self) -> super::vals::FlashnscSizeSize {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::FlashnscSizeSize::from_bits(val as u8)
            }
            #[doc = "Size of the non-secure callable (NSC) region n"]
            #[inline(always)]
            pub fn set_size(&mut self, val: super::vals::FlashnscSizeSize) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
            #[inline(always)]
            pub const fn lock(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub fn set_lock(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for FlashnscSize {
            #[inline(always)]
            fn default() -> FlashnscSize {
                FlashnscSize(0)
            }
        }
        impl core::fmt::Debug for FlashnscSize {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("FlashnscSize")
                    .field("size", &self.size())
                    .field("lock", &self.lock())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for FlashnscSize {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct FlashnscSize {
                    size: super::vals::FlashnscSizeSize,
                    lock: bool,
                }
                let proxy = FlashnscSize {
                    size: self.size(),
                    lock: self.lock(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description cluster: Access permissions for flash region n"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct FlashregionPerm(pub u32);
        impl FlashregionPerm {
            #[doc = "Configure instruction fetch permissions from flash region n"]
            #[inline(always)]
            pub const fn execute(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Configure instruction fetch permissions from flash region n"]
            #[inline(always)]
            pub fn set_execute(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Configure write permission for flash region n"]
            #[inline(always)]
            pub const fn write(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Configure write permission for flash region n"]
            #[inline(always)]
            pub fn set_write(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Configure read permissions for flash region n"]
            #[inline(always)]
            pub const fn read(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Configure read permissions for flash region n"]
            #[inline(always)]
            pub fn set_read(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Security attribute for flash region n"]
            #[inline(always)]
            pub const fn secattr(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Security attribute for flash region n"]
            #[inline(always)]
            pub fn set_secattr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[inline(always)]
            pub const fn lock(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub fn set_lock(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for FlashregionPerm {
            #[inline(always)]
            fn default() -> FlashregionPerm {
                FlashregionPerm(0)
            }
        }
        impl core::fmt::Debug for FlashregionPerm {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("FlashregionPerm")
                    .field("execute", &self.execute())
                    .field("write", &self.write())
                    .field("read", &self.read())
                    .field("secattr", &self.secattr())
                    .field("lock", &self.lock())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for FlashregionPerm {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct FlashregionPerm {
                    execute: bool,
                    write: bool,
                    read: bool,
                    secattr: bool,
                    lock: bool,
                }
                let proxy = FlashregionPerm {
                    execute: self.execute(),
                    write: self.write(),
                    read: self.read(),
                    secattr: self.secattr(),
                    lock: self.lock(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description cluster: Prevent further modification of the corresponding PERM register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct GpioportLock(pub u32);
        impl GpioportLock {
            #[inline(always)]
            pub const fn lock(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub fn set_lock(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for GpioportLock {
            #[inline(always)]
            fn default() -> GpioportLock {
                GpioportLock(0)
            }
        }
        impl core::fmt::Debug for GpioportLock {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("GpioportLock")
                    .field("lock", &self.lock())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for GpioportLock {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct GpioportLock {
                    lock: bool,
                }
                let proxy = GpioportLock { lock: self.lock() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description cluster: Select between secure and non-secure attribute for pins 0 to 31 of port n."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct GpioportPerm(pub u32);
        impl GpioportPerm {
            #[doc = "Select secure attribute attribute for PIN 0."]
            #[inline(always)]
            pub const fn pin0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 0."]
            #[inline(always)]
            pub fn set_pin0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Select secure attribute attribute for PIN 1."]
            #[inline(always)]
            pub const fn pin1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 1."]
            #[inline(always)]
            pub fn set_pin1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Select secure attribute attribute for PIN 2."]
            #[inline(always)]
            pub const fn pin2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 2."]
            #[inline(always)]
            pub fn set_pin2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Select secure attribute attribute for PIN 3."]
            #[inline(always)]
            pub const fn pin3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 3."]
            #[inline(always)]
            pub fn set_pin3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Select secure attribute attribute for PIN 4."]
            #[inline(always)]
            pub const fn pin4(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 4."]
            #[inline(always)]
            pub fn set_pin4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Select secure attribute attribute for PIN 5."]
            #[inline(always)]
            pub const fn pin5(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 5."]
            #[inline(always)]
            pub fn set_pin5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Select secure attribute attribute for PIN 6."]
            #[inline(always)]
            pub const fn pin6(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 6."]
            #[inline(always)]
            pub fn set_pin6(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Select secure attribute attribute for PIN 7."]
            #[inline(always)]
            pub const fn pin7(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 7."]
            #[inline(always)]
            pub fn set_pin7(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Select secure attribute attribute for PIN 8."]
            #[inline(always)]
            pub const fn pin8(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 8."]
            #[inline(always)]
            pub fn set_pin8(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Select secure attribute attribute for PIN 9."]
            #[inline(always)]
            pub const fn pin9(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 9."]
            #[inline(always)]
            pub fn set_pin9(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Select secure attribute attribute for PIN 10."]
            #[inline(always)]
            pub const fn pin10(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 10."]
            #[inline(always)]
            pub fn set_pin10(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Select secure attribute attribute for PIN 11."]
            #[inline(always)]
            pub const fn pin11(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 11."]
            #[inline(always)]
            pub fn set_pin11(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "Select secure attribute attribute for PIN 12."]
            #[inline(always)]
            pub const fn pin12(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 12."]
            #[inline(always)]
            pub fn set_pin12(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "Select secure attribute attribute for PIN 13."]
            #[inline(always)]
            pub const fn pin13(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 13."]
            #[inline(always)]
            pub fn set_pin13(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Select secure attribute attribute for PIN 14."]
            #[inline(always)]
            pub const fn pin14(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 14."]
            #[inline(always)]
            pub fn set_pin14(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Select secure attribute attribute for PIN 15."]
            #[inline(always)]
            pub const fn pin15(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 15."]
            #[inline(always)]
            pub fn set_pin15(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
            #[doc = "Select secure attribute attribute for PIN 16."]
            #[inline(always)]
            pub const fn pin16(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 16."]
            #[inline(always)]
            pub fn set_pin16(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Select secure attribute attribute for PIN 17."]
            #[inline(always)]
            pub const fn pin17(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 17."]
            #[inline(always)]
            pub fn set_pin17(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Select secure attribute attribute for PIN 18."]
            #[inline(always)]
            pub const fn pin18(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 18."]
            #[inline(always)]
            pub fn set_pin18(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Select secure attribute attribute for PIN 19."]
            #[inline(always)]
            pub const fn pin19(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 19."]
            #[inline(always)]
            pub fn set_pin19(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "Select secure attribute attribute for PIN 20."]
            #[inline(always)]
            pub const fn pin20(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 20."]
            #[inline(always)]
            pub fn set_pin20(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "Select secure attribute attribute for PIN 21."]
            #[inline(always)]
            pub const fn pin21(&self) -> bool {
                let val = (self.0 >> 21usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 21."]
            #[inline(always)]
            pub fn set_pin21(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
            }
            #[doc = "Select secure attribute attribute for PIN 22."]
            #[inline(always)]
            pub const fn pin22(&self) -> bool {
                let val = (self.0 >> 22usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 22."]
            #[inline(always)]
            pub fn set_pin22(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
            }
            #[doc = "Select secure attribute attribute for PIN 23."]
            #[inline(always)]
            pub const fn pin23(&self) -> bool {
                let val = (self.0 >> 23usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 23."]
            #[inline(always)]
            pub fn set_pin23(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
            }
            #[doc = "Select secure attribute attribute for PIN 24."]
            #[inline(always)]
            pub const fn pin24(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 24."]
            #[inline(always)]
            pub fn set_pin24(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
            #[doc = "Select secure attribute attribute for PIN 25."]
            #[inline(always)]
            pub const fn pin25(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 25."]
            #[inline(always)]
            pub fn set_pin25(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
            #[doc = "Select secure attribute attribute for PIN 26."]
            #[inline(always)]
            pub const fn pin26(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 26."]
            #[inline(always)]
            pub fn set_pin26(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
            #[doc = "Select secure attribute attribute for PIN 27."]
            #[inline(always)]
            pub const fn pin27(&self) -> bool {
                let val = (self.0 >> 27usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 27."]
            #[inline(always)]
            pub fn set_pin27(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
            }
            #[doc = "Select secure attribute attribute for PIN 28."]
            #[inline(always)]
            pub const fn pin28(&self) -> bool {
                let val = (self.0 >> 28usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 28."]
            #[inline(always)]
            pub fn set_pin28(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
            }
            #[doc = "Select secure attribute attribute for PIN 29."]
            #[inline(always)]
            pub const fn pin29(&self) -> bool {
                let val = (self.0 >> 29usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 29."]
            #[inline(always)]
            pub fn set_pin29(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
            }
            #[doc = "Select secure attribute attribute for PIN 30."]
            #[inline(always)]
            pub const fn pin30(&self) -> bool {
                let val = (self.0 >> 30usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 30."]
            #[inline(always)]
            pub fn set_pin30(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
            }
            #[doc = "Select secure attribute attribute for PIN 31."]
            #[inline(always)]
            pub const fn pin31(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[doc = "Select secure attribute attribute for PIN 31."]
            #[inline(always)]
            pub fn set_pin31(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
            }
        }
        impl Default for GpioportPerm {
            #[inline(always)]
            fn default() -> GpioportPerm {
                GpioportPerm(0)
            }
        }
        impl core::fmt::Debug for GpioportPerm {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("GpioportPerm")
                    .field("pin0", &self.pin0())
                    .field("pin1", &self.pin1())
                    .field("pin2", &self.pin2())
                    .field("pin3", &self.pin3())
                    .field("pin4", &self.pin4())
                    .field("pin5", &self.pin5())
                    .field("pin6", &self.pin6())
                    .field("pin7", &self.pin7())
                    .field("pin8", &self.pin8())
                    .field("pin9", &self.pin9())
                    .field("pin10", &self.pin10())
                    .field("pin11", &self.pin11())
                    .field("pin12", &self.pin12())
                    .field("pin13", &self.pin13())
                    .field("pin14", &self.pin14())
                    .field("pin15", &self.pin15())
                    .field("pin16", &self.pin16())
                    .field("pin17", &self.pin17())
                    .field("pin18", &self.pin18())
                    .field("pin19", &self.pin19())
                    .field("pin20", &self.pin20())
                    .field("pin21", &self.pin21())
                    .field("pin22", &self.pin22())
                    .field("pin23", &self.pin23())
                    .field("pin24", &self.pin24())
                    .field("pin25", &self.pin25())
                    .field("pin26", &self.pin26())
                    .field("pin27", &self.pin27())
                    .field("pin28", &self.pin28())
                    .field("pin29", &self.pin29())
                    .field("pin30", &self.pin30())
                    .field("pin31", &self.pin31())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for GpioportPerm {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct GpioportPerm {
                    pin0: bool,
                    pin1: bool,
                    pin2: bool,
                    pin3: bool,
                    pin4: bool,
                    pin5: bool,
                    pin6: bool,
                    pin7: bool,
                    pin8: bool,
                    pin9: bool,
                    pin10: bool,
                    pin11: bool,
                    pin12: bool,
                    pin13: bool,
                    pin14: bool,
                    pin15: bool,
                    pin16: bool,
                    pin17: bool,
                    pin18: bool,
                    pin19: bool,
                    pin20: bool,
                    pin21: bool,
                    pin22: bool,
                    pin23: bool,
                    pin24: bool,
                    pin25: bool,
                    pin26: bool,
                    pin27: bool,
                    pin28: bool,
                    pin29: bool,
                    pin30: bool,
                    pin31: bool,
                }
                let proxy = GpioportPerm {
                    pin0: self.pin0(),
                    pin1: self.pin1(),
                    pin2: self.pin2(),
                    pin3: self.pin3(),
                    pin4: self.pin4(),
                    pin5: self.pin5(),
                    pin6: self.pin6(),
                    pin7: self.pin7(),
                    pin8: self.pin8(),
                    pin9: self.pin9(),
                    pin10: self.pin10(),
                    pin11: self.pin11(),
                    pin12: self.pin12(),
                    pin13: self.pin13(),
                    pin14: self.pin14(),
                    pin15: self.pin15(),
                    pin16: self.pin16(),
                    pin17: self.pin17(),
                    pin18: self.pin18(),
                    pin19: self.pin19(),
                    pin20: self.pin20(),
                    pin21: self.pin21(),
                    pin22: self.pin22(),
                    pin23: self.pin23(),
                    pin24: self.pin24(),
                    pin25: self.pin25(),
                    pin26: self.pin26(),
                    pin27: self.pin27(),
                    pin28: self.pin28(),
                    pin29: self.pin29(),
                    pin30: self.pin30(),
                    pin31: self.pin31(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Enable or disable interrupt for event RAMACCERR"]
            #[inline(always)]
            pub const fn ramaccerr(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RAMACCERR"]
            #[inline(always)]
            pub fn set_ramaccerr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable interrupt for event FLASHACCERR"]
            #[inline(always)]
            pub const fn flashaccerr(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event FLASHACCERR"]
            #[inline(always)]
            pub fn set_flashaccerr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for event PERIPHACCERR"]
            #[inline(always)]
            pub const fn periphaccerr(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event PERIPHACCERR"]
            #[inline(always)]
            pub fn set_periphaccerr(&mut self, val: bool) {
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
                    .field("ramaccerr", &self.ramaccerr())
                    .field("flashaccerr", &self.flashaccerr())
                    .field("periphaccerr", &self.periphaccerr())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    ramaccerr: bool,
                    flashaccerr: bool,
                    periphaccerr: bool,
                }
                let proxy = Int {
                    ramaccerr: self.ramaccerr(),
                    flashaccerr: self.flashaccerr(),
                    periphaccerr: self.periphaccerr(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description cluster: List capabilities and access permissions for the peripheral with ID n"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct PeriphidPerm(pub u32);
        impl PeriphidPerm {
            #[doc = "Define configuration capabilities for TrustZone Cortex-M secure attribute"]
            #[inline(always)]
            pub const fn securemapping(&self) -> super::vals::PeriphidPermSecuremapping {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::PeriphidPermSecuremapping::from_bits(val as u8)
            }
            #[doc = "Define configuration capabilities for TrustZone Cortex-M secure attribute"]
            #[inline(always)]
            pub fn set_securemapping(&mut self, val: super::vals::PeriphidPermSecuremapping) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
            #[doc = "Indicate if the peripheral has DMA capabilities and if DMA transfer can be assigned to a different security attribute than the peripheral itself"]
            #[inline(always)]
            pub const fn dma(&self) -> super::vals::Dma {
                let val = (self.0 >> 2usize) & 0x03;
                super::vals::Dma::from_bits(val as u8)
            }
            #[doc = "Indicate if the peripheral has DMA capabilities and if DMA transfer can be assigned to a different security attribute than the peripheral itself"]
            #[inline(always)]
            pub fn set_dma(&mut self, val: super::vals::Dma) {
                self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
            }
            #[doc = "Peripheral security mapping"]
            #[inline(always)]
            pub const fn secattr(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Peripheral security mapping"]
            #[inline(always)]
            pub fn set_secattr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Security attribution for the DMA transfer"]
            #[inline(always)]
            pub const fn dmasec(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Security attribution for the DMA transfer"]
            #[inline(always)]
            pub fn set_dmasec(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[inline(always)]
            pub const fn lock(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub fn set_lock(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Indicate if a peripheral is present with ID n"]
            #[inline(always)]
            pub const fn present(&self) -> super::vals::Present {
                let val = (self.0 >> 31usize) & 0x01;
                super::vals::Present::from_bits(val as u8)
            }
            #[doc = "Indicate if a peripheral is present with ID n"]
            #[inline(always)]
            pub fn set_present(&mut self, val: super::vals::Present) {
                self.0 =
                    (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
            }
        }
        impl Default for PeriphidPerm {
            #[inline(always)]
            fn default() -> PeriphidPerm {
                PeriphidPerm(0)
            }
        }
        impl core::fmt::Debug for PeriphidPerm {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("PeriphidPerm")
                    .field("securemapping", &self.securemapping())
                    .field("dma", &self.dma())
                    .field("secattr", &self.secattr())
                    .field("dmasec", &self.dmasec())
                    .field("lock", &self.lock())
                    .field("present", &self.present())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for PeriphidPerm {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct PeriphidPerm {
                    securemapping: super::vals::PeriphidPermSecuremapping,
                    dma: super::vals::Dma,
                    secattr: bool,
                    dmasec: bool,
                    lock: bool,
                    present: super::vals::Present,
                }
                let proxy = PeriphidPerm {
                    securemapping: self.securemapping(),
                    dma: self.dma(),
                    secattr: self.secattr(),
                    dmasec: self.dmasec(),
                    lock: self.lock(),
                    present: self.present(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description cluster: Define which RAM region can contain the non-secure callable (NSC) region n"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RamnscRegion(pub u32);
        impl RamnscRegion {
            #[doc = "Region number"]
            #[inline(always)]
            pub const fn region(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x1f;
                val as u8
            }
            #[doc = "Region number"]
            #[inline(always)]
            pub fn set_region(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
            }
            #[inline(always)]
            pub const fn lock(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub fn set_lock(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for RamnscRegion {
            #[inline(always)]
            fn default() -> RamnscRegion {
                RamnscRegion(0)
            }
        }
        impl core::fmt::Debug for RamnscRegion {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RamnscRegion")
                    .field("region", &self.region())
                    .field("lock", &self.lock())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RamnscRegion {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct RamnscRegion {
                    region: u8,
                    lock: bool,
                }
                let proxy = RamnscRegion {
                    region: self.region(),
                    lock: self.lock(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description cluster: Define the size of the non-secure callable (NSC) region n"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RamnscSize(pub u32);
        impl RamnscSize {
            #[doc = "Size of the non-secure callable (NSC) region n"]
            #[inline(always)]
            pub const fn size(&self) -> super::vals::RamnscSizeSize {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::RamnscSizeSize::from_bits(val as u8)
            }
            #[doc = "Size of the non-secure callable (NSC) region n"]
            #[inline(always)]
            pub fn set_size(&mut self, val: super::vals::RamnscSizeSize) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
            #[inline(always)]
            pub const fn lock(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub fn set_lock(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for RamnscSize {
            #[inline(always)]
            fn default() -> RamnscSize {
                RamnscSize(0)
            }
        }
        impl core::fmt::Debug for RamnscSize {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RamnscSize")
                    .field("size", &self.size())
                    .field("lock", &self.lock())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RamnscSize {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct RamnscSize {
                    size: super::vals::RamnscSizeSize,
                    lock: bool,
                }
                let proxy = RamnscSize {
                    size: self.size(),
                    lock: self.lock(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description cluster: Access permissions for RAM region n"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RamregionPerm(pub u32);
        impl RamregionPerm {
            #[doc = "Configure instruction fetch permissions from RAM region n"]
            #[inline(always)]
            pub const fn execute(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Configure instruction fetch permissions from RAM region n"]
            #[inline(always)]
            pub fn set_execute(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Configure write permission for RAM region n"]
            #[inline(always)]
            pub const fn write(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Configure write permission for RAM region n"]
            #[inline(always)]
            pub fn set_write(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Configure read permissions for RAM region n"]
            #[inline(always)]
            pub const fn read(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Configure read permissions for RAM region n"]
            #[inline(always)]
            pub fn set_read(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Security attribute for RAM region n"]
            #[inline(always)]
            pub const fn secattr(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Security attribute for RAM region n"]
            #[inline(always)]
            pub fn set_secattr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[inline(always)]
            pub const fn lock(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[inline(always)]
            pub fn set_lock(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for RamregionPerm {
            #[inline(always)]
            fn default() -> RamregionPerm {
                RamregionPerm(0)
            }
        }
        impl core::fmt::Debug for RamregionPerm {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RamregionPerm")
                    .field("execute", &self.execute())
                    .field("write", &self.write())
                    .field("read", &self.read())
                    .field("secattr", &self.secattr())
                    .field("lock", &self.lock())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RamregionPerm {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct RamregionPerm {
                    execute: bool,
                    write: bool,
                    read: bool,
                    secattr: bool,
                    lock: bool,
                }
                let proxy = RamregionPerm {
                    execute: self.execute(),
                    write: self.write(),
                    read: self.read(),
                    secattr: self.secattr(),
                    lock: self.lock(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Dma {
            #[doc = "Peripheral has no DMA capability"]
            NO_DMA = 0x0,
            #[doc = "Peripheral has DMA and DMA transfers always have the same security attribute as assigned to the peripheral"]
            NO_SEPARATE_ATTRIBUTE = 0x01,
            #[doc = "Peripheral has DMA and DMA transfers can have a different security attribute than the one assigned to the peripheral"]
            SEPARATE_ATTRIBUTE = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Dma {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Dma {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Dma {
            #[inline(always)]
            fn from(val: u8) -> Dma {
                Dma::from_bits(val)
            }
        }
        impl From<Dma> for u8 {
            #[inline(always)]
            fn from(val: Dma) -> u8 {
                Dma::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum ExtdomainPermSecuremapping {
            #[doc = "The bus access from this external domain always have the non-secure attribute set"]
            NON_SECURE = 0x0,
            #[doc = "The bus access from this external domain always have the secure attribute set"]
            SECURE = 0x01,
            #[doc = "Non-secure or secure attribute for bus access from this domain is defined by the EXTDOMAIN\\[n\\].PERM register"]
            USER_SELECTABLE = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl ExtdomainPermSecuremapping {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> ExtdomainPermSecuremapping {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for ExtdomainPermSecuremapping {
            #[inline(always)]
            fn from(val: u8) -> ExtdomainPermSecuremapping {
                ExtdomainPermSecuremapping::from_bits(val)
            }
        }
        impl From<ExtdomainPermSecuremapping> for u8 {
            #[inline(always)]
            fn from(val: ExtdomainPermSecuremapping) -> u8 {
                ExtdomainPermSecuremapping::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum FlashnscSizeSize {
            #[doc = "The region n is not defined as a non-secure callable region. Normal security attributes (secure or non-secure) are enforced."]
            DISABLED = 0x0,
            #[doc = "The region n is defined as non-secure callable with a 32-byte size"]
            _32 = 0x01,
            #[doc = "The region n is defined as non-secure callable with a 64-byte size"]
            _64 = 0x02,
            #[doc = "The region n is defined as non-secure callable with a 128-byte size"]
            _128 = 0x03,
            #[doc = "The region n is defined as non-secure callable with a 256-byte size"]
            _256 = 0x04,
            #[doc = "The region n is defined as non-secure callable with a 512-byte size"]
            _512 = 0x05,
            #[doc = "The region n is defined as non-secure callable with a 1024-byte size"]
            _1024 = 0x06,
            #[doc = "The region n is defined as non-secure callable with a 2048-byte size"]
            _2048 = 0x07,
            #[doc = "The region n is defined as non-secure callable with a 4096-byte size"]
            _4096 = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl FlashnscSizeSize {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> FlashnscSizeSize {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for FlashnscSizeSize {
            #[inline(always)]
            fn from(val: u8) -> FlashnscSizeSize {
                FlashnscSizeSize::from_bits(val)
            }
        }
        impl From<FlashnscSizeSize> for u8 {
            #[inline(always)]
            fn from(val: FlashnscSizeSize) -> u8 {
                FlashnscSizeSize::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum PeriphidPermSecuremapping {
            #[doc = "This peripheral is always accessible as a non-secure peripheral"]
            NON_SECURE = 0x0,
            #[doc = "This peripheral is always accessible as a secure peripheral"]
            SECURE = 0x01,
            #[doc = "Non-secure or secure attribute for this peripheral is defined by the PERIPHID\\[n\\].PERM register"]
            USER_SELECTABLE = 0x02,
            #[doc = "This peripheral implements the split security mechanism. Non-secure or secure attribute for this peripheral is defined by the PERIPHID\\[n\\].PERM register."]
            SPLIT = 0x03,
        }
        impl PeriphidPermSecuremapping {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> PeriphidPermSecuremapping {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for PeriphidPermSecuremapping {
            #[inline(always)]
            fn from(val: u8) -> PeriphidPermSecuremapping {
                PeriphidPermSecuremapping::from_bits(val)
            }
        }
        impl From<PeriphidPermSecuremapping> for u8 {
            #[inline(always)]
            fn from(val: PeriphidPermSecuremapping) -> u8 {
                PeriphidPermSecuremapping::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Present {
            #[doc = "Peripheral is not present"]
            NOT_PRESENT = 0x0,
            #[doc = "Peripheral is present"]
            IS_PRESENT = 0x01,
        }
        impl Present {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Present {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Present {
            #[inline(always)]
            fn from(val: u8) -> Present {
                Present::from_bits(val)
            }
        }
        impl From<Present> for u8 {
            #[inline(always)]
            fn from(val: Present) -> u8 {
                Present::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum RamnscSizeSize {
            #[doc = "The region n is not defined as a non-secure callable region. Normal security attributes (secure or non-secure) are enforced."]
            DISABLED = 0x0,
            #[doc = "The region n is defined as non-secure callable with a 32-byte size"]
            _32 = 0x01,
            #[doc = "The region n is defined as non-secure callable with a 64-byte size"]
            _64 = 0x02,
            #[doc = "The region n is defined as non-secure callable with a 128-byte size"]
            _128 = 0x03,
            #[doc = "The region n is defined as non-secure callable with a 256-byte size"]
            _256 = 0x04,
            #[doc = "The region n is defined as non-secure callable with a 512-byte size"]
            _512 = 0x05,
            #[doc = "The region n is defined as non-secure callable with a 1024-byte size"]
            _1024 = 0x06,
            #[doc = "The region n is defined as non-secure callable with a 2048-byte size"]
            _2048 = 0x07,
            #[doc = "The region n is defined as non-secure callable with a 4096-byte size"]
            _4096 = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl RamnscSizeSize {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> RamnscSizeSize {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for RamnscSizeSize {
            #[inline(always)]
            fn from(val: u8) -> RamnscSizeSize {
                RamnscSizeSize::from_bits(val)
            }
        }
        impl From<RamnscSizeSize> for u8 {
            #[inline(always)]
            fn from(val: RamnscSizeSize) -> u8 {
                RamnscSizeSize::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Tzm {
            #[doc = "ARM TrustZone support not available"]
            NOT_AVAILABLE = 0x0,
            #[doc = "ARM TrustZone support is available"]
            ENABLED = 0x01,
        }
        impl Tzm {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Tzm {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Tzm {
            #[inline(always)]
            fn from(val: u8) -> Tzm {
                Tzm::from_bits(val)
            }
        }
        impl From<Tzm> for u8 {
            #[inline(always)]
            fn from(val: Tzm) -> u8 {
                Tzm::to_bits(val)
            }
        }
    }
}
pub mod tad {
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
        #[doc = "Pin configuration for TRACECLK"]
        #[inline(always)]
        pub const fn traceclk(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Pin configuration for TRACEDATA\\[0\\]"]
        #[inline(always)]
        pub const fn tracedata0(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Pin configuration for TRACEDATA\\[1\\]"]
        #[inline(always)]
        pub const fn tracedata1(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Pin configuration for TRACEDATA\\[2\\]"]
        #[inline(always)]
        pub const fn tracedata2(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Pin configuration for TRACEDATA\\[3\\]"]
        #[inline(always)]
        pub const fn tracedata3(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
    }
    #[doc = "Trace and debug control"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tad {
        ptr: *mut u8,
    }
    unsafe impl Send for Tad {}
    unsafe impl Sync for Tad {}
    impl Tad {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start all trace and debug clocks."]
        #[inline(always)]
        pub const fn tasks_clockstart(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop all trace and debug clocks."]
        #[inline(always)]
        pub const fn tasks_clockstop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Enable debug domain and aquire selected GPIOs"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn psel(self) -> Psel {
            unsafe { Psel::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Clocking options for the Trace Port debug interface Reset behavior is the same as debug components"]
        #[inline(always)]
        pub const fn traceportspeed(
            self,
        ) -> crate::common::Reg<regs::Traceportspeed, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Enable debug domain and aquire selected GPIOs"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[inline(always)]
            pub const fn enable(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
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
                #[derive(defmt :: Format)]
                struct Enable {
                    enable: bool,
                }
                let proxy = Enable {
                    enable: self.enable(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Clocking options for the Trace Port debug interface Reset behavior is the same as debug components"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Traceportspeed(pub u32);
        impl Traceportspeed {
            #[doc = "Speed of Trace Port clock. Note that the TRACECLK pin output will be divided again by two from the Trace Port clock."]
            #[inline(always)]
            pub const fn traceportspeed(&self) -> super::vals::Traceportspeed {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Traceportspeed::from_bits(val as u8)
            }
            #[doc = "Speed of Trace Port clock. Note that the TRACECLK pin output will be divided again by two from the Trace Port clock."]
            #[inline(always)]
            pub fn set_traceportspeed(&mut self, val: super::vals::Traceportspeed) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Traceportspeed {
            #[inline(always)]
            fn default() -> Traceportspeed {
                Traceportspeed(0)
            }
        }
        impl core::fmt::Debug for Traceportspeed {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Traceportspeed")
                    .field("traceportspeed", &self.traceportspeed())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Traceportspeed {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Traceportspeed {
                    traceportspeed: super::vals::Traceportspeed,
                }
                let proxy = Traceportspeed {
                    traceportspeed: self.traceportspeed(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct PselTraceclkPin(pub u8);
        impl PselTraceclkPin {
            #[doc = "TRACECLK pin"]
            pub const TRACECLK: Self = Self(0x15);
        }
        impl PselTraceclkPin {
            pub const fn from_bits(val: u8) -> PselTraceclkPin {
                Self(val & 0x1f)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl core::fmt::Debug for PselTraceclkPin {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x15 => f.write_str("TRACECLK"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for PselTraceclkPin {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x15 => defmt::write!(f, "TRACECLK"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u8> for PselTraceclkPin {
            #[inline(always)]
            fn from(val: u8) -> PselTraceclkPin {
                PselTraceclkPin::from_bits(val)
            }
        }
        impl From<PselTraceclkPin> for u8 {
            #[inline(always)]
            fn from(val: PselTraceclkPin) -> u8 {
                PselTraceclkPin::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct PselTracedata0Pin(pub u8);
        impl PselTracedata0Pin {
            #[doc = "TRACEDATA0 pin"]
            pub const TRACEDATA0: Self = Self(0x16);
        }
        impl PselTracedata0Pin {
            pub const fn from_bits(val: u8) -> PselTracedata0Pin {
                Self(val & 0x1f)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl core::fmt::Debug for PselTracedata0Pin {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x16 => f.write_str("TRACEDATA0"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for PselTracedata0Pin {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x16 => defmt::write!(f, "TRACEDATA0"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u8> for PselTracedata0Pin {
            #[inline(always)]
            fn from(val: u8) -> PselTracedata0Pin {
                PselTracedata0Pin::from_bits(val)
            }
        }
        impl From<PselTracedata0Pin> for u8 {
            #[inline(always)]
            fn from(val: PselTracedata0Pin) -> u8 {
                PselTracedata0Pin::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct PselTracedata1Pin(pub u8);
        impl PselTracedata1Pin {
            #[doc = "TRACEDATA1 pin"]
            pub const TRACEDATA1: Self = Self(0x17);
        }
        impl PselTracedata1Pin {
            pub const fn from_bits(val: u8) -> PselTracedata1Pin {
                Self(val & 0x1f)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl core::fmt::Debug for PselTracedata1Pin {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x17 => f.write_str("TRACEDATA1"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for PselTracedata1Pin {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x17 => defmt::write!(f, "TRACEDATA1"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u8> for PselTracedata1Pin {
            #[inline(always)]
            fn from(val: u8) -> PselTracedata1Pin {
                PselTracedata1Pin::from_bits(val)
            }
        }
        impl From<PselTracedata1Pin> for u8 {
            #[inline(always)]
            fn from(val: PselTracedata1Pin) -> u8 {
                PselTracedata1Pin::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct PselTracedata2Pin(pub u8);
        impl PselTracedata2Pin {
            #[doc = "TRACEDATA2 pin"]
            pub const TRACEDATA2: Self = Self(0x18);
        }
        impl PselTracedata2Pin {
            pub const fn from_bits(val: u8) -> PselTracedata2Pin {
                Self(val & 0x1f)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl core::fmt::Debug for PselTracedata2Pin {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x18 => f.write_str("TRACEDATA2"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for PselTracedata2Pin {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x18 => defmt::write!(f, "TRACEDATA2"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u8> for PselTracedata2Pin {
            #[inline(always)]
            fn from(val: u8) -> PselTracedata2Pin {
                PselTracedata2Pin::from_bits(val)
            }
        }
        impl From<PselTracedata2Pin> for u8 {
            #[inline(always)]
            fn from(val: PselTracedata2Pin) -> u8 {
                PselTracedata2Pin::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct PselTracedata3Pin(pub u8);
        impl PselTracedata3Pin {
            #[doc = "TRACEDATA3 pin"]
            pub const TRACEDATA3: Self = Self(0x19);
        }
        impl PselTracedata3Pin {
            pub const fn from_bits(val: u8) -> PselTracedata3Pin {
                Self(val & 0x1f)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl core::fmt::Debug for PselTracedata3Pin {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x19 => f.write_str("TRACEDATA3"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for PselTracedata3Pin {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x19 => defmt::write!(f, "TRACEDATA3"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u8> for PselTracedata3Pin {
            #[inline(always)]
            fn from(val: u8) -> PselTracedata3Pin {
                PselTracedata3Pin::from_bits(val)
            }
        }
        impl From<PselTracedata3Pin> for u8 {
            #[inline(always)]
            fn from(val: PselTracedata3Pin) -> u8 {
                PselTracedata3Pin::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Traceportspeed {
            #[doc = "Trace Port clock is: 32MHz"]
            _32MHZ = 0x0,
            #[doc = "Trace Port clock is: 16MHz"]
            _16MHZ = 0x01,
            #[doc = "Trace Port clock is: 8MHz"]
            _8MHZ = 0x02,
            #[doc = "Trace Port clock is: 4MHz"]
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
        #[doc = "Subscribe configuration for task START"]
        #[inline(always)]
        pub const fn subscribe_start(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
        }
        #[doc = "Subscribe configuration for task STOP"]
        #[inline(always)]
        pub const fn subscribe_stop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
        }
        #[doc = "Subscribe configuration for task COUNT"]
        #[inline(always)]
        pub const fn subscribe_count(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
        }
        #[doc = "Subscribe configuration for task CLEAR"]
        #[inline(always)]
        pub const fn subscribe_clear(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
        }
        #[doc = "Deprecated register - Subscribe configuration for task SHUTDOWN"]
        #[inline(always)]
        pub const fn subscribe_shutdown(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
        }
        #[doc = "Description collection: Subscribe configuration for task CAPTURE\\[n\\]"]
        #[inline(always)]
        pub const fn subscribe_capture(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            assert!(n < 6usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Compare event on CC\\[n\\] match"]
        #[inline(always)]
        pub const fn events_compare(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 6usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Publish configuration for event COMPARE\\[n\\]"]
        #[inline(always)]
        pub const fn publish_compare(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            assert!(n < 6usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c0usize + n * 4usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
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
        #[doc = "Description collection: Enable one-shot operation for Capture/Compare channel n"]
        #[inline(always)]
        pub const fn oneshoten(
            self,
            n: usize,
        ) -> crate::common::Reg<regs::Oneshoten, crate::common::RW> {
            assert!(n < 6usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize + n * 4usize) as _) }
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
                #[derive(defmt :: Format)]
                struct Bitmode {
                    bitmode: super::vals::Bitmode,
                }
                let proxy = Bitmode {
                    bitmode: self.bitmode(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
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
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field(
                        "compare",
                        &[
                            self.compare(0usize),
                            self.compare(1usize),
                            self.compare(2usize),
                            self.compare(3usize),
                            self.compare(4usize),
                            self.compare(5usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    compare: [bool; 6usize],
                }
                let proxy = Int {
                    compare: [
                        self.compare(0usize),
                        self.compare(1usize),
                        self.compare(2usize),
                        self.compare(3usize),
                        self.compare(4usize),
                        self.compare(5usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Mode {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Mode").field("mode", &self.mode()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Mode {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Mode {
                    mode: super::vals::Mode,
                }
                let proxy = Mode { mode: self.mode() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description collection: Enable one-shot operation for Capture/Compare channel n"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Oneshoten(pub u32);
        impl Oneshoten {
            #[doc = "Enable one-shot operation"]
            #[inline(always)]
            pub const fn oneshoten(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable one-shot operation"]
            #[inline(always)]
            pub fn set_oneshoten(&mut self, val: bool) {
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
                #[derive(defmt :: Format)]
                struct Oneshoten {
                    oneshoten: bool,
                }
                let proxy = Oneshoten {
                    oneshoten: self.oneshoten(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Prescaler {
                    prescaler: u8,
                }
                let proxy = Prescaler {
                    prescaler: self.prescaler(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Shorts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Shorts")
                    .field(
                        "compare_clear",
                        &[
                            self.compare_clear(0usize),
                            self.compare_clear(1usize),
                            self.compare_clear(2usize),
                            self.compare_clear(3usize),
                            self.compare_clear(4usize),
                            self.compare_clear(5usize),
                        ],
                    )
                    .field(
                        "compare_stop",
                        &[
                            self.compare_stop(0usize),
                            self.compare_stop(1usize),
                            self.compare_stop(2usize),
                            self.compare_stop(3usize),
                            self.compare_stop(4usize),
                            self.compare_stop(5usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Shorts {
                    compare_clear: [bool; 6usize],
                    compare_stop: [bool; 6usize],
                }
                let proxy = Shorts {
                    compare_clear: [
                        self.compare_clear(0usize),
                        self.compare_clear(1usize),
                        self.compare_clear(2usize),
                        self.compare_clear(3usize),
                        self.compare_clear(4usize),
                        self.compare_clear(5usize),
                    ],
                    compare_stop: [
                        self.compare_stop(0usize),
                        self.compare_stop(1usize),
                        self.compare_stop(2usize),
                        self.compare_stop(3usize),
                        self.compare_stop(4usize),
                        self.compare_stop(5usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
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
        #[doc = "Subscribe configuration for task STARTRX"]
        #[inline(always)]
        pub const fn subscribe_startrx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
        }
        #[doc = "Subscribe configuration for task STARTTX"]
        #[inline(always)]
        pub const fn subscribe_starttx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
        }
        #[doc = "Subscribe configuration for task STOP"]
        #[inline(always)]
        pub const fn subscribe_stop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
        }
        #[doc = "Subscribe configuration for task SUSPEND"]
        #[inline(always)]
        pub const fn subscribe_suspend(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
        }
        #[doc = "Subscribe configuration for task RESUME"]
        #[inline(always)]
        pub const fn subscribe_resume(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
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
        #[doc = "SUSPEND task has been issued, TWI traffic is now suspended."]
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
        #[doc = "Publish configuration for event STOPPED"]
        #[inline(always)]
        pub const fn publish_stopped(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
        }
        #[doc = "Publish configuration for event ERROR"]
        #[inline(always)]
        pub const fn publish_error(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a4usize) as _) }
        }
        #[doc = "Publish configuration for event SUSPENDED"]
        #[inline(always)]
        pub const fn publish_suspended(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c8usize) as _) }
        }
        #[doc = "Publish configuration for event RXSTARTED"]
        #[inline(always)]
        pub const fn publish_rxstarted(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01ccusize) as _) }
        }
        #[doc = "Publish configuration for event TXSTARTED"]
        #[inline(always)]
        pub const fn publish_txstarted(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d0usize) as _) }
        }
        #[doc = "Publish configuration for event LASTRX"]
        #[inline(always)]
        pub const fn publish_lastrx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01dcusize) as _) }
        }
        #[doc = "Publish configuration for event LASTTX"]
        #[inline(always)]
        pub const fn publish_lasttx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e0usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
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
                #[derive(defmt :: Format)]
                struct Address {
                    address: u8,
                }
                let proxy = Address {
                    address: self.address(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Enable {
                    enable: super::vals::Enable,
                }
                let proxy = Enable {
                    enable: self.enable(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Errorsrc {
                    overrun: bool,
                    anack: bool,
                    dnack: bool,
                }
                let proxy = Errorsrc {
                    overrun: self.overrun(),
                    anack: self.anack(),
                    dnack: self.dnack(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Frequency {
                    frequency: super::vals::Frequency,
                }
                let proxy = Frequency {
                    frequency: self.frequency(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
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
                #[derive(defmt :: Format)]
                struct Int {
                    stopped: bool,
                    error: bool,
                    suspended: bool,
                    rxstarted: bool,
                    txstarted: bool,
                    lastrx: bool,
                    lasttx: bool,
                }
                let proxy = Int {
                    stopped: self.stopped(),
                    error: self.error(),
                    suspended: self.suspended(),
                    rxstarted: self.rxstarted(),
                    txstarted: self.txstarted(),
                    lastrx: self.lastrx(),
                    lasttx: self.lasttx(),
                };
                defmt::write!(f, "{}", proxy)
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
                let val = (self.0 >> 0usize) & 0x1fff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last transaction. In case of NACK error, includes the NACK'ed byte."]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
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
                #[derive(defmt :: Format)]
                struct RxdAmount {
                    amount: u16,
                }
                let proxy = RxdAmount {
                    amount: self.amount(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct RxdList {
                    list: super::vals::RxdListList,
                }
                let proxy = RxdList { list: self.list() };
                defmt::write!(f, "{}", proxy)
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
                let val = (self.0 >> 0usize) & 0x1fff;
                val as u16
            }
            #[doc = "Maximum number of bytes in receive buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
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
                #[derive(defmt :: Format)]
                struct RxdMaxcnt {
                    maxcnt: u16,
                }
                let proxy = RxdMaxcnt {
                    maxcnt: self.maxcnt(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Shorts {
                    lasttx_startrx: bool,
                    lasttx_suspend: bool,
                    lasttx_stop: bool,
                    lastrx_starttx: bool,
                    lastrx_stop: bool,
                }
                let proxy = Shorts {
                    lasttx_startrx: self.lasttx_startrx(),
                    lasttx_suspend: self.lasttx_suspend(),
                    lasttx_stop: self.lasttx_stop(),
                    lastrx_starttx: self.lastrx_starttx(),
                    lastrx_stop: self.lastrx_stop(),
                };
                defmt::write!(f, "{}", proxy)
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
                let val = (self.0 >> 0usize) & 0x1fff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last transaction. In case of NACK error, includes the NACK'ed byte."]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
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
                #[derive(defmt :: Format)]
                struct TxdAmount {
                    amount: u16,
                }
                let proxy = TxdAmount {
                    amount: self.amount(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct TxdList {
                    list: super::vals::TxdListList,
                }
                let proxy = TxdList { list: self.list() };
                defmt::write!(f, "{}", proxy)
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
                let val = (self.0 >> 0usize) & 0x1fff;
                val as u16
            }
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
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
                #[derive(defmt :: Format)]
                struct TxdMaxcnt {
                    maxcnt: u16,
                }
                let proxy = TxdMaxcnt {
                    maxcnt: self.maxcnt(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Frequency {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0198_0000 => f.write_str("K100"),
                    0x0400_0000 => f.write_str("K250"),
                    0x0640_0000 => f.write_str("K400"),
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
        #[doc = "Subscribe configuration for task STOP"]
        #[inline(always)]
        pub const fn subscribe_stop(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
        }
        #[doc = "Subscribe configuration for task SUSPEND"]
        #[inline(always)]
        pub const fn subscribe_suspend(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
        }
        #[doc = "Subscribe configuration for task RESUME"]
        #[inline(always)]
        pub const fn subscribe_resume(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
        }
        #[doc = "Subscribe configuration for task PREPARERX"]
        #[inline(always)]
        pub const fn subscribe_preparerx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
        }
        #[doc = "Subscribe configuration for task PREPARETX"]
        #[inline(always)]
        pub const fn subscribe_preparetx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
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
        #[doc = "Publish configuration for event STOPPED"]
        #[inline(always)]
        pub const fn publish_stopped(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
        }
        #[doc = "Publish configuration for event ERROR"]
        #[inline(always)]
        pub const fn publish_error(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a4usize) as _) }
        }
        #[doc = "Publish configuration for event RXSTARTED"]
        #[inline(always)]
        pub const fn publish_rxstarted(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01ccusize) as _) }
        }
        #[doc = "Publish configuration for event TXSTARTED"]
        #[inline(always)]
        pub const fn publish_txstarted(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d0usize) as _) }
        }
        #[doc = "Publish configuration for event WRITE"]
        #[inline(always)]
        pub const fn publish_write(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e4usize) as _) }
        }
        #[doc = "Publish configuration for event READ"]
        #[inline(always)]
        pub const fn publish_read(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e8usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
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
                #[derive(defmt :: Format)]
                struct Address {
                    address: u8,
                }
                let proxy = Address {
                    address: self.address(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Config {
                    address0: bool,
                    address1: bool,
                }
                let proxy = Config {
                    address0: self.address0(),
                    address1: self.address1(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Enable {
                    enable: super::vals::Enable,
                }
                let proxy = Enable {
                    enable: self.enable(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Errorsrc {
                    overflow: bool,
                    dnack: bool,
                    overread: bool,
                }
                let proxy = Errorsrc {
                    overflow: self.overflow(),
                    dnack: self.dnack(),
                    overread: self.overread(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
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
                #[derive(defmt :: Format)]
                struct Int {
                    stopped: bool,
                    error: bool,
                    rxstarted: bool,
                    txstarted: bool,
                    write: bool,
                    read: bool,
                }
                let proxy = Int {
                    stopped: self.stopped(),
                    error: self.error(),
                    rxstarted: self.rxstarted(),
                    txstarted: self.txstarted(),
                    write: self.write(),
                    read: self.read(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Status register indicating which address had a match"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Match(pub u32);
        impl Match {
            #[doc = "Indication of which address in ADDRESS that matched the incoming address"]
            #[inline(always)]
            pub const fn match_(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Indication of which address in ADDRESS that matched the incoming address"]
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
                #[derive(defmt :: Format)]
                struct Match {
                    match_: bool,
                }
                let proxy = Match {
                    match_: self.match_(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Orc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Orc").field("orc", &self.orc()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Orc {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Orc {
                    orc: u8,
                }
                let proxy = Orc { orc: self.orc() };
                defmt::write!(f, "{}", proxy)
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
                let val = (self.0 >> 0usize) & 0x1fff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last RXD transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
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
                #[derive(defmt :: Format)]
                struct RxdAmount {
                    amount: u16,
                }
                let proxy = RxdAmount {
                    amount: self.amount(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct RxdList {
                    list: super::vals::RxdListList,
                }
                let proxy = RxdList { list: self.list() };
                defmt::write!(f, "{}", proxy)
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
                let val = (self.0 >> 0usize) & 0x1fff;
                val as u16
            }
            #[doc = "Maximum number of bytes in RXD buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
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
                #[derive(defmt :: Format)]
                struct RxdMaxcnt {
                    maxcnt: u16,
                }
                let proxy = RxdMaxcnt {
                    maxcnt: self.maxcnt(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Shorts {
                    write_suspend: bool,
                    read_suspend: bool,
                }
                let proxy = Shorts {
                    write_suspend: self.write_suspend(),
                    read_suspend: self.read_suspend(),
                };
                defmt::write!(f, "{}", proxy)
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
                let val = (self.0 >> 0usize) & 0x1fff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last TXD transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
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
                #[derive(defmt :: Format)]
                struct TxdAmount {
                    amount: u16,
                }
                let proxy = TxdAmount {
                    amount: self.amount(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct TxdList {
                    list: super::vals::TxdListList,
                }
                let proxy = TxdList { list: self.list() };
                defmt::write!(f, "{}", proxy)
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
                let val = (self.0 >> 0usize) & 0x1fff;
                val as u16
            }
            #[doc = "Maximum number of bytes in TXD buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
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
                #[derive(defmt :: Format)]
                struct TxdMaxcnt {
                    maxcnt: u16,
                }
                let proxy = TxdMaxcnt {
                    maxcnt: self.maxcnt(),
                };
                defmt::write!(f, "{}", proxy)
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
    #[doc = "UART with EasyDMA 0"]
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
        #[doc = "Subscribe configuration for task STARTRX"]
        #[inline(always)]
        pub const fn subscribe_startrx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
        }
        #[doc = "Subscribe configuration for task STOPRX"]
        #[inline(always)]
        pub const fn subscribe_stoprx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
        }
        #[doc = "Subscribe configuration for task STARTTX"]
        #[inline(always)]
        pub const fn subscribe_starttx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
        }
        #[doc = "Subscribe configuration for task STOPTX"]
        #[inline(always)]
        pub const fn subscribe_stoptx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
        }
        #[doc = "Subscribe configuration for task FLUSHRX"]
        #[inline(always)]
        pub const fn subscribe_flushrx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
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
        #[doc = "Publish configuration for event CTS"]
        #[inline(always)]
        pub const fn publish_cts(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
        }
        #[doc = "Publish configuration for event NCTS"]
        #[inline(always)]
        pub const fn publish_ncts(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
        }
        #[doc = "Publish configuration for event RXDRDY"]
        #[inline(always)]
        pub const fn publish_rxdrdy(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
        }
        #[doc = "Publish configuration for event ENDRX"]
        #[inline(always)]
        pub const fn publish_endrx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize) as _) }
        }
        #[doc = "Publish configuration for event TXDRDY"]
        #[inline(always)]
        pub const fn publish_txdrdy(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x019cusize) as _) }
        }
        #[doc = "Publish configuration for event ENDTX"]
        #[inline(always)]
        pub const fn publish_endtx(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a0usize) as _) }
        }
        #[doc = "Publish configuration for event ERROR"]
        #[inline(always)]
        pub const fn publish_error(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a4usize) as _) }
        }
        #[doc = "Publish configuration for event RXTO"]
        #[inline(always)]
        pub const fn publish_rxto(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c4usize) as _) }
        }
        #[doc = "Publish configuration for event RXSTARTED"]
        #[inline(always)]
        pub const fn publish_rxstarted(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01ccusize) as _) }
        }
        #[doc = "Publish configuration for event TXSTARTED"]
        #[inline(always)]
        pub const fn publish_txstarted(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d0usize) as _) }
        }
        #[doc = "Publish configuration for event TXSTOPPED"]
        #[inline(always)]
        pub const fn publish_txstopped(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d8usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Error source This register is read/write one to clear."]
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
                #[derive(defmt :: Format)]
                struct Baudrate {
                    baudrate: super::vals::Baudrate,
                }
                let proxy = Baudrate {
                    baudrate: self.baudrate(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Config {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Config")
                    .field("hwfc", &self.hwfc())
                    .field("parity", &self.parity())
                    .field("stop", &self.stop())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Config {
                    hwfc: bool,
                    parity: super::vals::ConfigParity,
                    stop: super::vals::Stop,
                }
                let proxy = Config {
                    hwfc: self.hwfc(),
                    parity: self.parity(),
                    stop: self.stop(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Enable {
                    enable: super::vals::Enable,
                }
                let proxy = Enable {
                    enable: self.enable(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Error source This register is read/write one to clear."]
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
                #[derive(defmt :: Format)]
                struct Errorsrc {
                    overrun: bool,
                    parity: bool,
                    framing: bool,
                    break_: bool,
                }
                let proxy = Errorsrc {
                    overrun: self.overrun(),
                    parity: self.parity(),
                    framing: self.framing(),
                    break_: self.break_(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
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
                #[derive(defmt :: Format)]
                struct Int {
                    cts: bool,
                    ncts: bool,
                    rxdrdy: bool,
                    endrx: bool,
                    txdrdy: bool,
                    endtx: bool,
                    error: bool,
                    rxto: bool,
                    rxstarted: bool,
                    txstarted: bool,
                    txstopped: bool,
                }
                let proxy = Int {
                    cts: self.cts(),
                    ncts: self.ncts(),
                    rxdrdy: self.rxdrdy(),
                    endrx: self.endrx(),
                    txdrdy: self.txdrdy(),
                    endtx: self.endtx(),
                    error: self.error(),
                    rxto: self.rxto(),
                    rxstarted: self.rxstarted(),
                    txstarted: self.txstarted(),
                    txstopped: self.txstopped(),
                };
                defmt::write!(f, "{}", proxy)
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
                let val = (self.0 >> 0usize) & 0x1fff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
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
                #[derive(defmt :: Format)]
                struct RxdAmount {
                    amount: u16,
                }
                let proxy = RxdAmount {
                    amount: self.amount(),
                };
                defmt::write!(f, "{}", proxy)
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
                let val = (self.0 >> 0usize) & 0x1fff;
                val as u16
            }
            #[doc = "Maximum number of bytes in receive buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
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
                #[derive(defmt :: Format)]
                struct RxdMaxcnt {
                    maxcnt: u16,
                }
                let proxy = RxdMaxcnt {
                    maxcnt: self.maxcnt(),
                };
                defmt::write!(f, "{}", proxy)
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
                #[derive(defmt :: Format)]
                struct Shorts {
                    endrx_startrx: bool,
                    endrx_stoprx: bool,
                }
                let proxy = Shorts {
                    endrx_startrx: self.endrx_startrx(),
                    endrx_stoprx: self.endrx_stoprx(),
                };
                defmt::write!(f, "{}", proxy)
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
                let val = (self.0 >> 0usize) & 0x1fff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
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
                #[derive(defmt :: Format)]
                struct TxdAmount {
                    amount: u16,
                }
                let proxy = TxdAmount {
                    amount: self.amount(),
                };
                defmt::write!(f, "{}", proxy)
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
                let val = (self.0 >> 0usize) & 0x1fff;
                val as u16
            }
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
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
                #[derive(defmt :: Format)]
                struct TxdMaxcnt {
                    maxcnt: u16,
                }
                let proxy = TxdMaxcnt {
                    maxcnt: self.maxcnt(),
                };
                defmt::write!(f, "{}", proxy)
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
        #[doc = "Description cluster: Destination address where content of the key value registers (KEYSLOT.KEYn.VALUE\\[0-3\\]) will be pushed by KMU. Note that this address must match that of a peripherals APB mapped write-only key registers, else the KMU can push this key value into an address range which the CPU can potentially read."]
        #[inline(always)]
        pub const fn dest(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster: Define permissions for the key slot. Bits 0-15 and 16-31 can only be written when equal to 0xFFFF."]
        #[inline(always)]
        pub const fn perm(self) -> crate::common::Reg<regs::Perm, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Key {
        ptr: *mut u8,
    }
    unsafe impl Send for Key {}
    unsafe impl Sync for Key {}
    impl Key {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description collection: Define bits \\[31+o*32:0+o*32\\] of value assigned to KMU key slot."]
        #[inline(always)]
        pub const fn value(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Keyslot {
        ptr: *mut u8,
    }
    unsafe impl Send for Keyslot {}
    unsafe impl Sync for Keyslot {}
    impl Keyslot {
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
        pub const fn config(self, n: usize) -> Config {
            assert!(n < 128usize);
            unsafe { Config::from_ptr(self.ptr.add(0x0usize + n * 8usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn key(self, n: usize) -> Key {
            assert!(n < 128usize);
            unsafe { Key::from_ptr(self.ptr.add(0x0400usize + n * 16usize) as _) }
        }
    }
    #[doc = "User information configuration registers User information configuration registers"]
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
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Oscillator control"]
        #[inline(always)]
        pub const fn xosc32m(self) -> crate::common::Reg<regs::Xosc32m, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "HFXO clock source selection"]
        #[inline(always)]
        pub const fn hfxosrc(self) -> crate::common::Reg<regs::Hfxosrc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "HFXO startup counter"]
        #[inline(always)]
        pub const fn hfxocnt(self) -> crate::common::Reg<regs::Hfxocnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "Enable blocking NVM WRITE and aborting NVM ERASE for Application NVM in POFWARN condition ."]
        #[inline(always)]
        pub const fn appnvmcpofguard(
            self,
        ) -> crate::common::Reg<regs::Appnvmcpofguard, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
        }
        #[doc = "Polarity of PMIC polarity configuration signals."]
        #[inline(always)]
        pub const fn pmicconf(self) -> crate::common::Reg<regs::Pmicconf, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
        }
        #[doc = "Secure access port protection"]
        #[inline(always)]
        pub const fn secureapprotect(
            self,
        ) -> crate::common::Reg<regs::Secureapprotect, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
        }
        #[doc = "Erase protection"]
        #[inline(always)]
        pub const fn eraseprotect(
            self,
        ) -> crate::common::Reg<regs::Eraseprotect, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
        }
        #[doc = "Description collection: One time programmable memory"]
        #[inline(always)]
        pub const fn otp(self, n: usize) -> crate::common::Reg<regs::Otp, crate::common::RW> {
            assert!(n < 190usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize + n * 4usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn keyslot(self) -> Keyslot {
            unsafe { Keyslot::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Enable blocking NVM WRITE and aborting NVM ERASE for Application NVM in POFWARN condition ."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Appnvmcpofguard(pub u32);
        impl Appnvmcpofguard {
            #[doc = "Enable blocking NVM WRITE and aborting NVM ERASE in POFWARN condition"]
            #[inline(always)]
            pub const fn nvmcpofguarden(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable blocking NVM WRITE and aborting NVM ERASE in POFWARN condition"]
            #[inline(always)]
            pub fn set_nvmcpofguarden(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Appnvmcpofguard {
            #[inline(always)]
            fn default() -> Appnvmcpofguard {
                Appnvmcpofguard(0)
            }
        }
        impl core::fmt::Debug for Appnvmcpofguard {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Appnvmcpofguard")
                    .field("nvmcpofguarden", &self.nvmcpofguarden())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Appnvmcpofguard {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Appnvmcpofguard {
                    nvmcpofguarden: bool,
                }
                let proxy = Appnvmcpofguard {
                    nvmcpofguarden: self.nvmcpofguarden(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Access port protection"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Approtect(pub u32);
        impl Approtect {
            #[doc = "Blocks debugger read/write access to all CPU registers and memory mapped addresses"]
            #[inline(always)]
            pub const fn pall(&self) -> super::vals::ApprotectPall {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::ApprotectPall::from_bits(val as u32)
            }
            #[doc = "Blocks debugger read/write access to all CPU registers and memory mapped addresses"]
            #[inline(always)]
            pub fn set_pall(&mut self, val: super::vals::ApprotectPall) {
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
                #[derive(defmt :: Format)]
                struct Approtect {
                    pall: super::vals::ApprotectPall,
                }
                let proxy = Approtect { pall: self.pall() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Erase protection"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Eraseprotect(pub u32);
        impl Eraseprotect {
            #[doc = "Blocks NVMC ERASEALL and CTRLAP ERASEALL functionality"]
            #[inline(always)]
            pub const fn pall(&self) -> super::vals::EraseprotectPall {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::EraseprotectPall::from_bits(val as u32)
            }
            #[doc = "Blocks NVMC ERASEALL and CTRLAP ERASEALL functionality"]
            #[inline(always)]
            pub fn set_pall(&mut self, val: super::vals::EraseprotectPall) {
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
                #[derive(defmt :: Format)]
                struct Eraseprotect {
                    pall: super::vals::EraseprotectPall,
                }
                let proxy = Eraseprotect { pall: self.pall() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "HFXO startup counter"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Hfxocnt(pub u32);
        impl Hfxocnt {
            #[doc = "HFXO startup counter. Total debounce time = HFXOCNT*64 us + 0.5 us"]
            #[inline(always)]
            pub const fn hfxocnt(&self) -> super::vals::Hfxocnt {
                let val = (self.0 >> 0usize) & 0xff;
                super::vals::Hfxocnt::from_bits(val as u8)
            }
            #[doc = "HFXO startup counter. Total debounce time = HFXOCNT*64 us + 0.5 us"]
            #[inline(always)]
            pub fn set_hfxocnt(&mut self, val: super::vals::Hfxocnt) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Hfxocnt {
            #[inline(always)]
            fn default() -> Hfxocnt {
                Hfxocnt(0)
            }
        }
        impl core::fmt::Debug for Hfxocnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Hfxocnt")
                    .field("hfxocnt", &self.hfxocnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Hfxocnt {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Hfxocnt {
                    hfxocnt: super::vals::Hfxocnt,
                }
                let proxy = Hfxocnt {
                    hfxocnt: self.hfxocnt(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "HFXO clock source selection"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Hfxosrc(pub u32);
        impl Hfxosrc {
            #[doc = "HFXO clock source selection"]
            #[inline(always)]
            pub const fn hfxosrc(&self) -> super::vals::Hfxosrc {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Hfxosrc::from_bits(val as u8)
            }
            #[doc = "HFXO clock source selection"]
            #[inline(always)]
            pub fn set_hfxosrc(&mut self, val: super::vals::Hfxosrc) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Hfxosrc {
            #[inline(always)]
            fn default() -> Hfxosrc {
                Hfxosrc(0)
            }
        }
        impl core::fmt::Debug for Hfxosrc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Hfxosrc")
                    .field("hfxosrc", &self.hfxosrc())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Hfxosrc {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Hfxosrc {
                    hfxosrc: super::vals::Hfxosrc,
                }
                let proxy = Hfxosrc {
                    hfxosrc: self.hfxosrc(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description collection: One time programmable memory"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Otp(pub u32);
        impl Otp {
            #[doc = "Lower half word"]
            #[inline(always)]
            pub const fn lower(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Lower half word"]
            #[inline(always)]
            pub fn set_lower(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
            #[doc = "Upper half word"]
            #[inline(always)]
            pub const fn upper(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0xffff;
                val as u16
            }
            #[doc = "Upper half word"]
            #[inline(always)]
            pub fn set_upper(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
            }
        }
        impl Default for Otp {
            #[inline(always)]
            fn default() -> Otp {
                Otp(0)
            }
        }
        impl core::fmt::Debug for Otp {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Otp")
                    .field("lower", &self.lower())
                    .field("upper", &self.upper())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Otp {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Otp {
                    lower: u16,
                    upper: u16,
                }
                let proxy = Otp {
                    lower: self.lower(),
                    upper: self.upper(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description cluster: Define permissions for the key slot. Bits 0-15 and 16-31 can only be written when equal to 0xFFFF."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Perm(pub u32);
        impl Perm {
            #[doc = "Write permission for key slot"]
            #[inline(always)]
            pub const fn write(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write permission for key slot"]
            #[inline(always)]
            pub fn set_write(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Read permission for key slot"]
            #[inline(always)]
            pub const fn read(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Read permission for key slot"]
            #[inline(always)]
            pub fn set_read(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Push permission for key slot"]
            #[inline(always)]
            pub const fn push(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Push permission for key slot"]
            #[inline(always)]
            pub fn set_push(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Revocation state for the key slot"]
            #[inline(always)]
            pub const fn state(&self) -> super::vals::State {
                let val = (self.0 >> 16usize) & 0x01;
                super::vals::State::from_bits(val as u8)
            }
            #[doc = "Revocation state for the key slot"]
            #[inline(always)]
            pub fn set_state(&mut self, val: super::vals::State) {
                self.0 =
                    (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
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
                    .field("push", &self.push())
                    .field("state", &self.state())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Perm {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Perm {
                    write: bool,
                    read: bool,
                    push: bool,
                    state: super::vals::State,
                }
                let proxy = Perm {
                    write: self.write(),
                    read: self.read(),
                    push: self.push(),
                    state: self.state(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Polarity of PMIC polarity configuration signals."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pmicconf(pub u32);
        impl Pmicconf {
            #[doc = "Polarity of PMIC_FPWM signal."]
            #[inline(always)]
            pub const fn pmicfpwmpol(&self) -> super::vals::Pmicfpwmpol {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Pmicfpwmpol::from_bits(val as u8)
            }
            #[doc = "Polarity of PMIC_FPWM signal."]
            #[inline(always)]
            pub fn set_pmicfpwmpol(&mut self, val: super::vals::Pmicfpwmpol) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Pmicconf {
            #[inline(always)]
            fn default() -> Pmicconf {
                Pmicconf(0)
            }
        }
        impl core::fmt::Debug for Pmicconf {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Pmicconf")
                    .field("pmicfpwmpol", &self.pmicfpwmpol())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Pmicconf {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Pmicconf {
                    pmicfpwmpol: super::vals::Pmicfpwmpol,
                }
                let proxy = Pmicconf {
                    pmicfpwmpol: self.pmicfpwmpol(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Secure access port protection"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Secureapprotect(pub u32);
        impl Secureapprotect {
            #[doc = "Blocks debugger read/write access to all secure CPU registers and secure memory mapped addresses"]
            #[inline(always)]
            pub const fn pall(&self) -> super::vals::SecureapprotectPall {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::SecureapprotectPall::from_bits(val as u32)
            }
            #[doc = "Blocks debugger read/write access to all secure CPU registers and secure memory mapped addresses"]
            #[inline(always)]
            pub fn set_pall(&mut self, val: super::vals::SecureapprotectPall) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Secureapprotect {
            #[inline(always)]
            fn default() -> Secureapprotect {
                Secureapprotect(0)
            }
        }
        impl core::fmt::Debug for Secureapprotect {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Secureapprotect")
                    .field("pall", &self.pall())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Secureapprotect {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Secureapprotect {
                    pall: super::vals::SecureapprotectPall,
                }
                let proxy = Secureapprotect { pall: self.pall() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Oscillator control"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Xosc32m(pub u32);
        impl Xosc32m {
            #[doc = "Pierce current DAC control signals"]
            #[inline(always)]
            pub const fn ctrl(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x3f;
                val as u8
            }
            #[doc = "Pierce current DAC control signals"]
            #[inline(always)]
            pub fn set_ctrl(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
            }
        }
        impl Default for Xosc32m {
            #[inline(always)]
            fn default() -> Xosc32m {
                Xosc32m(0)
            }
        }
        impl core::fmt::Debug for Xosc32m {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Xosc32m")
                    .field("ctrl", &self.ctrl())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Xosc32m {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Xosc32m {
                    ctrl: u8,
                }
                let proxy = Xosc32m { ctrl: self.ctrl() };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct ApprotectPall(pub u32);
        impl ApprotectPall {
            #[doc = "Protected"]
            pub const PROTECTED: Self = Self(0x0);
            #[doc = "HwUnprotected"]
            pub const HW_UNPROTECTED: Self = Self(0x50fa_50fa);
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
                    0x50fa_50fa => f.write_str("HW_UNPROTECTED"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for ApprotectPall {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0 => defmt::write!(f, "PROTECTED"),
                    0x50fa_50fa => defmt::write!(f, "HW_UNPROTECTED"),
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
        pub struct EraseprotectPall(pub u32);
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
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Hfxocnt(pub u8);
        impl Hfxocnt {
            #[doc = "Min debounce time = (0*64 us + 0.5 us)"]
            pub const MIN_DEBOUNCE_TIME: Self = Self(0x0);
            #[doc = "Max debounce time = (255*64 us + 0.5 us)"]
            pub const MAX_DEBOUNCE_TIME: Self = Self(0xff);
        }
        impl Hfxocnt {
            pub const fn from_bits(val: u8) -> Hfxocnt {
                Self(val & 0xff)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl core::fmt::Debug for Hfxocnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0 => f.write_str("MIN_DEBOUNCE_TIME"),
                    0xff => f.write_str("MAX_DEBOUNCE_TIME"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Hfxocnt {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0 => defmt::write!(f, "MIN_DEBOUNCE_TIME"),
                    0xff => defmt::write!(f, "MAX_DEBOUNCE_TIME"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u8> for Hfxocnt {
            #[inline(always)]
            fn from(val: u8) -> Hfxocnt {
                Hfxocnt::from_bits(val)
            }
        }
        impl From<Hfxocnt> for u8 {
            #[inline(always)]
            fn from(val: Hfxocnt) -> u8 {
                Hfxocnt::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Hfxosrc {
            #[doc = "32 MHz temperature compensated crystal oscillator (TCXO)"]
            TCXO = 0x0,
            #[doc = "32 MHz crystal oscillator"]
            XTAL = 0x01,
        }
        impl Hfxosrc {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Hfxosrc {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Hfxosrc {
            #[inline(always)]
            fn from(val: u8) -> Hfxosrc {
                Hfxosrc::from_bits(val)
            }
        }
        impl From<Hfxosrc> for u8 {
            #[inline(always)]
            fn from(val: Hfxosrc) -> u8 {
                Hfxosrc::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Pmicfpwmpol {
            #[doc = "PMIC_FPWM output signal is active-low"]
            ACTIVE_LOW = 0x0,
            #[doc = "PMIC_FPWM output signal is active-high"]
            ACTIVE_HIGH = 0x01,
        }
        impl Pmicfpwmpol {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Pmicfpwmpol {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Pmicfpwmpol {
            #[inline(always)]
            fn from(val: u8) -> Pmicfpwmpol {
                Pmicfpwmpol::from_bits(val)
            }
        }
        impl From<Pmicfpwmpol> for u8 {
            #[inline(always)]
            fn from(val: Pmicfpwmpol) -> u8 {
                Pmicfpwmpol::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct SecureapprotectPall(pub u32);
        impl SecureapprotectPall {
            #[doc = "Protected"]
            pub const PROTECTED: Self = Self(0x0);
            #[doc = "HwUnprotected"]
            pub const HW_UNPROTECTED: Self = Self(0x50fa_50fa);
        }
        impl SecureapprotectPall {
            pub const fn from_bits(val: u32) -> SecureapprotectPall {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for SecureapprotectPall {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0 => f.write_str("PROTECTED"),
                    0x50fa_50fa => f.write_str("HW_UNPROTECTED"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SecureapprotectPall {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0 => defmt::write!(f, "PROTECTED"),
                    0x50fa_50fa => defmt::write!(f, "HW_UNPROTECTED"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for SecureapprotectPall {
            #[inline(always)]
            fn from(val: u32) -> SecureapprotectPall {
                SecureapprotectPall::from_bits(val)
            }
        }
        impl From<SecureapprotectPall> for u32 {
            #[inline(always)]
            fn from(val: SecureapprotectPall) -> u32 {
                SecureapprotectPall::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum State {
            #[doc = "Key value registers can no longer be read or pushed"]
            REVOKED = 0x0,
            #[doc = "Key value registers are readable (if enabled) and can be pushed (if enabled)"]
            ACTIVE = 0x01,
        }
        impl State {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> State {
                unsafe { core::mem::transmute(val & 0x01) }
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
        #[doc = "Description cluster: RAMn power control register"]
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
    #[doc = "Volatile Memory controller 0"]
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
            assert!(n < 8usize);
            unsafe { Ram::from_ptr(self.ptr.add(0x0600usize + n * 16usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Description cluster: RAMn power control register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Power(pub u32);
        impl Power {
            #[doc = "Keep RAM section S0 of RAM n on or off in System ON mode"]
            #[inline(always)]
            pub const fn spower(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Keep RAM section S0 of RAM n on or off in System ON mode"]
            #[inline(always)]
            pub fn set_spower(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Keep retention on RAM section S0 of RAM n when RAM section is switched off"]
            #[inline(always)]
            pub const fn sretention(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 16usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Keep retention on RAM section S0 of RAM n when RAM section is switched off"]
            #[inline(always)]
            pub fn set_sretention(&mut self, n: usize, val: bool) {
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
                    .field(
                        "spower",
                        &[
                            self.spower(0usize),
                            self.spower(1usize),
                            self.spower(2usize),
                            self.spower(3usize),
                        ],
                    )
                    .field(
                        "sretention",
                        &[
                            self.sretention(0usize),
                            self.sretention(1usize),
                            self.sretention(2usize),
                            self.sretention(3usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Power {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Power {
                    spower: [bool; 4usize],
                    sretention: [bool; 4usize],
                }
                let proxy = Power {
                    spower: [
                        self.spower(0usize),
                        self.spower(1usize),
                        self.spower(2usize),
                        self.spower(3usize),
                    ],
                    sretention: [
                        self.sretention(0usize),
                        self.sretention(1usize),
                        self.sretention(2usize),
                        self.sretention(3usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
}
pub mod wdt {
    #[doc = "Watchdog Timer 0"]
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
        #[doc = "Subscribe configuration for task START"]
        #[inline(always)]
        pub const fn subscribe_start(
            self,
        ) -> crate::common::Reg<super::shared::regs::Subscribe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
        }
        #[doc = "Watchdog timeout"]
        #[inline(always)]
        pub const fn events_timeout(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Publish configuration for event TIMEOUT"]
        #[inline(always)]
        pub const fn publish_timeout(
            self,
        ) -> crate::common::Reg<super::shared::regs::Publish, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
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
                #[derive(defmt :: Format)]
                struct Config {
                    sleep: super::vals::Sleep,
                    halt: super::vals::Halt,
                }
                let proxy = Config {
                    sleep: self.sleep(),
                    halt: self.halt(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
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
                #[derive(defmt :: Format)]
                struct Int {
                    timeout: bool,
                }
                let proxy = Int {
                    timeout: self.timeout(),
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Reqstatus {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Reqstatus")
                    .field(
                        "rr",
                        &[
                            self.rr(0usize),
                            self.rr(1usize),
                            self.rr(2usize),
                            self.rr(3usize),
                            self.rr(4usize),
                            self.rr(5usize),
                            self.rr(6usize),
                            self.rr(7usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Reqstatus {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Reqstatus {
                    rr: [bool; 8usize],
                }
                let proxy = Reqstatus {
                    rr: [
                        self.rr(0usize),
                        self.rr(1usize),
                        self.rr(2usize),
                        self.rr(3usize),
                        self.rr(4usize),
                        self.rr(5usize),
                        self.rr(6usize),
                        self.rr(7usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Rr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rr").field("rr", &self.rr()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rr {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Rr {
                    rr: super::vals::Rr,
                }
                let proxy = Rr { rr: self.rr() };
                defmt::write!(f, "{}", proxy)
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
        impl core::fmt::Debug for Rren {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rren")
                    .field(
                        "rr",
                        &[
                            self.rr(0usize),
                            self.rr(1usize),
                            self.rr(2usize),
                            self.rr(3usize),
                            self.rr(4usize),
                            self.rr(5usize),
                            self.rr(6usize),
                            self.rr(7usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rren {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Rren {
                    rr: [bool; 8usize],
                }
                let proxy = Rren {
                    rr: [
                        self.rr(0usize),
                        self.rr(1usize),
                        self.rr(2usize),
                        self.rr(3usize),
                        self.rr(4usize),
                        self.rr(5usize),
                        self.rr(6usize),
                        self.rr(7usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Run status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Runstatus(pub u32);
        impl Runstatus {
            #[doc = "Indicates whether or not the watchdog is running"]
            #[inline(always)]
            pub const fn runstatuswdt(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Indicates whether or not the watchdog is running"]
            #[inline(always)]
            pub fn set_runstatuswdt(&mut self, val: bool) {
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
                #[derive(defmt :: Format)]
                struct Runstatus {
                    runstatuswdt: bool,
                }
                let proxy = Runstatus {
                    runstatuswdt: self.runstatuswdt(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
