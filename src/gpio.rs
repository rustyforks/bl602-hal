//! General Purpose Input/Output
use core::marker::PhantomData;
use crate::pac;

/// Extension trait to split GLB peripheral into independent pins, registers and other modules
pub trait GlbExt {
    /// Splits the register block into independent pins and modules
    fn split(self) -> Parts;
}

pub use uart_sig::*;

/// Uart signals
pub mod uart_sig {
    use core::marker::PhantomData;
    use crate::pac;

    /// Uart0 RTS (type state)
    pub struct Uart0Rts;
    
    /// Uart0 CTS (type state)
    pub struct Uart0Cts;

    /// Uart0 TXD (type state)
    pub struct Uart0Tx;

    /// Uart0 RXD (type state)
    pub struct Uart0Rx;

    /// Uart1 RXD (type state)
    pub struct Uart1Rx;

    /// Uart1 RTS (type state)
    pub struct Uart1Rts;
    
    /// Uart1 CTS (type state)
    pub struct Uart1Cts;

    /// Uart1 TXD (type state)
    pub struct Uart1Tx;


macro_rules! impl_uart_sig {
    ($UartSigi: ident, $doc1: expr, $UartMuxi: ident, $doc2: expr) => {

    #[doc = $doc1]
    pub struct $UartSigi;

    #[doc = $doc2]
    pub struct $UartMuxi<MODE> {
        pub(crate) _mode: PhantomData<MODE>,
    }

    impl<MODE> $UartMuxi<MODE> {
        /// Configure the internal UART signal to UART0-RTS
        pub fn into_uart0_rts(self) -> $UartMuxi<Uart0Rts> {
            self.into_uart_mode(0)
        }
        /// Configure the internal UART signal to UART0-CTS
        pub fn into_uart0_cts(self) -> $UartMuxi<Uart0Cts> {
            self.into_uart_mode(1)
        }
        /// Configure the internal UART signal to UART0-TX
        pub fn into_uart0_tx(self) -> $UartMuxi<Uart0Tx> {
            self.into_uart_mode(2)
        }
        /// Configure the internal UART signal to UART0-RX
        pub fn into_uart0_rx(self) -> $UartMuxi<Uart0Rx> {
            self.into_uart_mode(3)
        }
        /// Configure the internal UART signal to UART1-RTS
        pub fn into_uart1_rts(self) -> $UartMuxi<Uart1Rts> {
            self.into_uart_mode(4)
        }
        /// Configure the internal UART signal to UART1-CTS
        pub fn into_uart1_cts(self) -> $UartMuxi<Uart1Cts> {
            self.into_uart_mode(5)
        }
        /// Configure the internal UART signal to UART1-TX
        pub fn into_uart1_tx(self) -> $UartMuxi<Uart1Tx> {
            self.into_uart_mode(6)
        }
        /// Configure the internal UART signal to UART1-RX
        pub fn into_uart1_rx(self) -> $UartMuxi<Uart1Rx> {
            self.into_uart_mode(7)
        }
        #[inline]
        fn into_uart_mode<T>(self, mode: u8) -> $UartMuxi<T> {
            let glb = unsafe { &*pac::GLB::ptr() };
            glb.uart_sig_sel_0.write(|w| unsafe { w
                .uart_sig_5_sel().bits(mode)
            });
            $UartMuxi { _mode: PhantomData }
        }
    }
    };
}    
    impl_uart_sig!(UartSig0, "Uart signal 0 (type state)", UartMux0, "Uart multiplexer peripherals for signal 0");
    impl_uart_sig!(UartSig1, "Uart signal 1 (type state)", UartMux1, "Uart multiplexer peripherals for signal 1");
    impl_uart_sig!(UartSig2, "Uart signal 2 (type state)", UartMux2, "Uart multiplexer peripherals for signal 2");
    impl_uart_sig!(UartSig3, "Uart signal 3 (type state)", UartMux3, "Uart multiplexer peripherals for signal 3");
    impl_uart_sig!(UartSig4, "Uart signal 4 (type state)", UartMux4, "Uart multiplexer peripherals for signal 4");
    impl_uart_sig!(UartSig5, "Uart signal 5 (type state)", UartMux5, "Uart multiplexer peripherals for signal 5");
    impl_uart_sig!(UartSig6, "Uart signal 6 (type state)", UartMux6, "Uart multiplexer peripherals for signal 6");
    impl_uart_sig!(UartSig7, "Uart signal 7 (type state)", UartMux7, "Uart multiplexer peripherals for signal 7");
}

/// Clock configurator registers
pub struct ClkCfg {
    pub(crate) _ownership: (),
}

/*
// todo: english
    在GPIO模式下，可以设置内部上下拉，以类型状态机模式设计
    SPI、UART、I2C等数字功能下，可以设置内部上下拉，但不会影响返回类型的状态
    ADC、DAC下，软件禁止设置内部上下拉。HAL库不会生成此类函数，以免出错。
*/

/// Hi-Z Floating pin (type state)
pub struct Floating;
/// Pulled down pin (type state)
pub struct PullDown;
/// Pulled up pin (type state)
pub struct PullUp;

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Alternate function (type state)
pub struct Alternate<MODE> {
    _mode: PhantomData<MODE>,
}

/// Alternate function 1 (type state)
pub struct AF1;
/// Alternate function 2 (type state)
pub struct AF2;
/// Alternate function 4 (type state)
pub struct AF4;
/// Alternate function 6 (type state)
pub struct AF6;
/// Alternate function 7 (type state)
pub struct Uart;
/// Alternate function 8 (type state)
pub struct AF8;
/// Alternate function 9 (type state)
pub struct AF9;
// AF11 is SwGpio, ignore
/// Alternate function 14 (type state)
pub struct AF14;

/// Alternate function 10 (type state)
pub struct Analog;

#[doc(hidden)]
pub trait UartPin<SIG> {}

// There are Pin0 to Pin22, totally 23 pins

pub use self::pin::*;

macro_rules! impl_glb {
    ($($Pini: ident: ($pini: ident, $gpio_cfgctli: ident, $UartSigi: ident, $sigi: ident, $gpio_i: ident) ,)+) => {

impl GlbExt for pac::GLB {
    fn split(self) -> Parts {
        Parts {
            $( $pini: $Pini { _mode: PhantomData }, )+
            uart_mux0: UartMux0 { _mode: PhantomData },
            uart_mux1: UartMux1 { _mode: PhantomData },
            uart_mux2: UartMux2 { _mode: PhantomData },
            uart_mux3: UartMux3 { _mode: PhantomData },
            uart_mux4: UartMux4 { _mode: PhantomData },
            uart_mux5: UartMux5 { _mode: PhantomData },
            uart_mux6: UartMux6 { _mode: PhantomData },
            uart_mux7: UartMux7 { _mode: PhantomData },
            clk_cfg: ClkCfg { _ownership: () },
        }
    }
}

/// Gpio parts
pub struct Parts {
    $( pub $pini: $Pini<Input<Floating>>, )+
    pub uart_mux0: UartMux0<Uart0Cts>,
    pub uart_mux1: UartMux1<Uart0Cts>,
    pub uart_mux2: UartMux2<Uart0Cts>,
    pub uart_mux3: UartMux3<Uart0Cts>,
    pub uart_mux4: UartMux4<Uart0Cts>,
    pub uart_mux5: UartMux5<Uart0Cts>,
    pub uart_mux6: UartMux6<Uart0Cts>,
    pub uart_mux7: UartMux7<Uart0Cts>,
    pub clk_cfg: ClkCfg,
}

/// Gpio pins
pub mod pin {
    use core::marker::PhantomData;
    use core::convert::Infallible;
    use crate::pac;
    use super::*;
    use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin, toggleable};

$(
    /// Pin
    pub struct $Pini<MODE> {
        pub(crate) _mode: PhantomData<MODE>,
    }

    impl<MODE> $Pini<MODE> {
        // 11 -> GPIO_FUN_SWGPIO
        /// Configures the pin to operate as a Hi-Z floating output pin.
        pub fn into_floating_output(self) -> $Pini<Output<Floating>> {
            self.into_pin_with_mode(11, false, false, false)
        }
        /// Configures the pin to operate as a pull-up output pin.
        pub fn into_pull_up_output(self) -> $Pini<Output<PullUp>> {
            self.into_pin_with_mode(11, true, false, false)
        }
        /// Configures the pin to operate as a pull-down output pin.
        pub fn into_pull_down_output(self) -> $Pini<Output<PullDown>> {
            self.into_pin_with_mode(11, false, true, false)
        }
        /// Configures the pin to operate as a Hi-Z floating input pin.
        pub fn into_floating_input(self) -> $Pini<Input<Floating>> {
            self.into_pin_with_mode(11, false, false, true)
        }
        /// Configures the pin to operate as a pull-up input pin.
        pub fn into_pull_up_input(self) -> $Pini<Input<PullUp>> {
            self.into_pin_with_mode(11, true, false, true)
        }
        /// Configures the pin to operate as a pull-down input pin.
        pub fn into_pull_down_input(self) -> $Pini<Input<PullDown>> {
            self.into_pin_with_mode(11, false, true, true)
        }
        #[inline] fn into_pin_with_mode<T>(&self, mode: u8, pu: bool, pd: bool, ie: bool) -> $Pini<T> {
            let glb = unsafe { &*pac::GLB::ptr() }; 
        paste::paste! {
            glb.$gpio_cfgctli.write(|w| unsafe { w
                .[<reg_ $gpio_i _func_sel>]().bits(mode) 
                .[<reg_ $gpio_i _ie>]().bit(ie) // output
                .[<reg_ $gpio_i _pu>]().bit(pu)
                .[<reg_ $gpio_i _pd>]().bit(pd)
                .[<reg_ $gpio_i _drv>]().bits(0) // disabled
                .[<reg_ $gpio_i _smt>]().clear_bit()
            });
        }
            $Pini { _mode: PhantomData }
        }
    }

    impl<MODE> $Pini<Input<MODE>> {
        /// Enable smitter GPIO input filter
        pub fn enable_smitter(&mut self) {
            let glb = unsafe { &*pac::GLB::ptr() };
            paste::paste! {
                glb.$gpio_cfgctli.modify(|_, w| w.[<reg_ $gpio_i _smt>]().set_bit());
            }
        }
        /// Enable smitter GPIO output filter
        pub fn disable_smitter(&mut self) {
            let glb = unsafe { &*pac::GLB::ptr() };
            paste::paste! {
                glb.$gpio_cfgctli.modify(|_, w| w.[<reg_ $gpio_i _smt>]().clear_bit());
            }
        }
    }

    impl<MODE> $Pini<MODE> {
        // todo: documents
        pub fn into_af1(self) -> $Pini<Alternate<AF1>> {
            todo!()
        }
        pub fn into_af2(self) -> $Pini<Alternate<AF2>> {
            todo!()
        }
        pub fn into_af4(self) -> $Pini<Alternate<AF4>> {
            todo!()
        }
        pub fn into_af6(self) -> $Pini<Alternate<AF6>> {
            todo!()
        }
        paste::paste! {
            /// Configures the pin to UART alternate mode
            pub fn [<into_uart_ $sigi>](self) -> $Pini<Uart> {
                // 7 -> GPIO_FUN_UART
                self.into_pin_with_mode(7, true, false, true)
            }
        }
        pub fn into_af8(self) -> $Pini<Alternate<AF8>> {
            todo!()
        }
        pub fn into_af9(self) -> $Pini<Alternate<AF9>> {
            todo!()
        }
        pub fn into_analog(self) -> $Pini<Analog> {
            todo!()
        }
        pub fn into_af14(self) -> $Pini<Alternate<AF14>> {
            todo!()
        }
    }

    impl UartPin<$UartSigi> for $Pini<Uart> {}
    
    impl<MODE> $Pini<Alternate<MODE>> {
        // 虽然有这些内部上下拉函数，内部上下拉很弱，大约44K，还是建议片外上拉
        // todo: english
        pub fn set_pull_up(&mut self) {
            todo!()
        }
        pub fn set_pull_down(&mut self) {
            todo!()
        }
        pub fn set_floating(&mut self) {
            todo!()
        }
    }

    impl<MODE> InputPin for $Pini<Input<MODE>> {
        type Error = Infallible;

        fn try_is_high(&self) -> Result<bool, Self::Error> {
            let glb = unsafe { &*pac::GLB::ptr() };
            paste::paste! {
                Ok(glb.gpio_cfgctl30.read().[<reg_ $gpio_i _i>]().bit_is_set())
            }
        }

        fn try_is_low(&self) -> Result<bool, Self::Error> {
            let glb = unsafe { &*pac::GLB::ptr() };
            paste::paste! {
                Ok(glb.gpio_cfgctl30.read().[<reg_ $gpio_i _i>]().bit_is_clear())
            }
        }
    }

    impl<MODE> OutputPin for $Pini<Output<MODE>> {
        type Error = Infallible;

        fn try_set_high(&mut self) -> Result<(), Self::Error> {
            let glb = unsafe { &*pac::GLB::ptr() };
            paste::paste! {
                glb.gpio_cfgctl32.modify(|_, w| w.[<reg_ $gpio_i _o>]().set_bit());
            }
            Ok(())
        }

        fn try_set_low(&mut self) -> Result<(), Self::Error> {
            let glb = unsafe { &*pac::GLB::ptr() };
            paste::paste! {
                glb.gpio_cfgctl32.modify(|_, w| w.[<reg_ $gpio_i _o>]().clear_bit());
            }
            Ok(())
        }
    }

    impl<MODE> StatefulOutputPin for $Pini<Output<MODE>> {
        fn try_is_set_high(&self) -> Result<bool, Self::Error> {
            let glb = unsafe { &*pac::GLB::ptr() };
            paste::paste! {
                Ok(glb.gpio_cfgctl32.read().[<reg_ $gpio_i _o>]().bit_is_set())
            }
        }

        fn try_is_set_low(&self) -> Result<bool, Self::Error> {
            let glb = unsafe { &*pac::GLB::ptr() };
            paste::paste! {
                Ok(glb.gpio_cfgctl32.read().[<reg_ $gpio_i _o>]().bit_is_clear())
            }
        }
    }

    impl<MODE> toggleable::Default for $Pini<Output<MODE>> {}
)+
}
    };
}

// There are Pin0 to Pin22, totally 23 pins
// todo: generate macros
impl_glb! {
    Pin0: (pin0, gpio_cfgctl0, UartSig0, sig0, gpio_0),
    Pin1: (pin1, gpio_cfgctl0, UartSig1, sig1, gpio_1),
    Pin2: (pin2, gpio_cfgctl1, UartSig2, sig2, gpio_2),
    Pin3: (pin3, gpio_cfgctl1, UartSig3, sig3, gpio_3),
    Pin4: (pin4, gpio_cfgctl2, UartSig4, sig4, gpio_4),
    Pin5: (pin5, gpio_cfgctl2, UartSig5, sig5, gpio_5),
    Pin6: (pin6, gpio_cfgctl3, UartSig6, sig6, gpio_6),
    Pin7: (pin7, gpio_cfgctl3, UartSig7, sig7, gpio_7),
    Pin8: (pin8, gpio_cfgctl4, UartSig0, sig0, gpio_8),
    Pin9: (pin9, gpio_cfgctl4, UartSig1, sig1, gpio_9),
    Pin10: (pin10, gpio_cfgctl5, UartSig2, sig2, gpio_10),
    Pin11: (pin11, gpio_cfgctl5, UartSig3, sig3, gpio_11),
    Pin12: (pin12, gpio_cfgctl6, UartSig4, sig4, gpio_12),
    Pin13: (pin13, gpio_cfgctl6, UartSig5, sig5, gpio_13),
    Pin14: (pin14, gpio_cfgctl7, UartSig6, sig6, gpio_14),
    Pin15: (pin15, gpio_cfgctl7, UartSig7, sig7, gpio_15),
    Pin16: (pin16, gpio_cfgctl8, UartSig0, sig0, gpio_16),
    Pin17: (pin17, gpio_cfgctl8, UartSig1, sig1, gpio_17),
    Pin18: (pin18, gpio_cfgctl9, UartSig2, sig2, gpio_18),
    Pin19: (pin19, gpio_cfgctl9, UartSig3, sig3, gpio_19),
    Pin20: (pin20, gpio_cfgctl10, UartSig4, sig4, gpio_20),
    Pin21: (pin21, gpio_cfgctl10, UartSig5, sig5, gpio_21),
    Pin22: (pin22, gpio_cfgctl11, UartSig6, sig6, gpio_22),
}
