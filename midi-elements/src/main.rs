#![feature(alloc_error_handler)]
#![no_std]
#![no_main]

extern crate alloc;
extern crate panic_halt;

use core::alloc::Layout;

use alloc_cortex_m::CortexMHeap;
use cortex_m::asm;

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use hal::delay::Delay;
use hal::gpio::*;
use hal::spi::*;
use hal::stm32;
use stm32f4::stm32f407::{interrupt, SPI2, TIM1, TIM2, TIM3, TIM5};
use stm32f4xx_hal as hal;
use stm32f4xx_hal::rcc::RccExt;

extern crate cty;

mod driver;
use driver::encoder::RotaryEncoder;

use st7920::ST7920;

use embedded_hal::digital::v2::InputPin;

mod ui;
use ui::{button::Button, framework::*, knob::Knob, panel::Panel};

mod elements_handlers;
use elements_handlers::*;

use alloc::boxed::Box;
use heapless::consts::U8;
use heapless::Vec;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
const HEAP_SIZE: usize = 1024; // in bytes

enum InputDeviceId {
    Button1,
    Button2,
    Button3,
    Button4,
    Button5,
    Knob1,
    Knob2,
    Knob3,
    Knob4,
}

struct App<'a> {
    button_pins: (
        gpioe::PE7<Input<PullUp>>,
        gpioe::PE15<Input<PullUp>>,
        gpiod::PD9<Input<PullUp>>,
        gpiod::PD11<Input<PullUp>>,
        gpiob::PB11<Input<PullUp>>,
    ),
    display: st7920::ST7920<
        Spi<
            SPI2,
            (
                gpiob::PB13<Alternate<AF5>>,
                NoMiso,
                gpiob::PB15<Alternate<AF5>>,
            ),
        >,
        gpioe::PE13<Output<PushPull>>,
        gpioe::PE13<Output<PushPull>>,
    >,
    encoders: (TIM2, TIM3, TIM5, TIM1),
    delay: Delay,
    panel: Option<Panel<'a>>,
}

impl<'a> App<'a> {
    fn new() -> Self {
        let p = stm32::Peripherals::take().unwrap();
        let cp = Peripherals::take().unwrap();
        let rcc = p.RCC.constrain();

        let clocks = rcc
            .cfgr
            .sysclk(stm32f4xx_hal::time::MegaHertz(168))
            .freeze();
        let mut delay = Delay::new(cp.SYST, clocks);

        let gpioa = p.GPIOA.split();
        let gpiob = p.GPIOB.split();
        let gpiod = p.GPIOD.split();
        let gpioe = p.GPIOE.split();

        p.TIM1.setup_enc(
            gpioa.pa8.into_alternate_af1(),
            gpioe.pe11.into_alternate_af1(),
        );
        p.TIM2.setup_enc(
            gpioa.pa15.into_alternate_af1(),
            gpiob.pb3.into_alternate_af1(),
        );
        p.TIM3.setup_enc(
            gpiob.pb5.into_alternate_af2(),
            gpiob.pb4.into_alternate_af2(),
        );
        p.TIM5.setup_enc(
            gpioa.pa1.into_alternate_af2(),
            gpioa.pa0.into_alternate_af2(),
        );

        let lcd_sck = gpiob.pb13.into_alternate_af5();
        let lcd_mosi = gpiob.pb15.into_alternate_af5();
        let lcd_reset = gpioe.pe13.into_push_pull_output();
        let spi = Spi::spi2(
            p.SPI2,
            (lcd_sck, NoMiso, lcd_mosi),
            Mode {
                polarity: Polarity::IdleLow,
                phase: Phase::CaptureOnFirstTransition,
            },
            stm32f4xx_hal::time::KiloHertz(1200).into(),
            clocks,
        );
        let button_pins = (
            gpioe.pe7.into_pull_up_input(),
            gpioe.pe15.into_pull_up_input(),
            gpiod.pd9.into_pull_up_input(),
            gpiod.pd11.into_pull_up_input(),
            gpiob.pb11.into_pull_up_input(),
        );

        let mut display = ST7920::new(
            spi,
            lcd_reset,
            None as Option<stm32f4xx_hal::gpio::gpioe::PE13<Output<PushPull>>>,
            true,
        );

        display.init(&mut delay).expect("could not  display");
        display.clear(&mut delay).expect("could not clear display");

        unsafe {
            Init(false);
        }

        App {
            button_pins,
            display,
            encoders: (p.TIM2, p.TIM3, p.TIM5, p.TIM1),
            delay,
            panel: None,
        }
    }

    fn setup_knobs(&mut self) -> Vec<Knob, U8> {
        let mut knobs = Vec::<_, U8>::new();

        knobs
            .push(Knob::new(
                Point::new(0, 40),
                InputDeviceId::Knob1 as InputId,
                create_knob_handler(Param::ExcStrikeLevel),
            ))
            .unwrap();
        knobs
            .push(Knob::new(
                Point::new(32, 40),
                InputDeviceId::Knob2 as InputId,
                create_knob_handler(Param::ExcStrikeTimbre),
            ))
            .unwrap();
        knobs
            .push(Knob::new(
                Point::new(96, 40),
                InputDeviceId::Knob4 as InputId,
                create_knob_handler(Param::ExcStrikeMeta),
            ))
            .unwrap();

        knobs
    }

    fn setup_buttons(&mut self) -> Vec<Button<'a>, U8> {
        let mut buttons = Vec::<_, U8>::new();

        buttons
            .push(Button::new(
                Point::new(0, 0),
                "P1",
                InputDeviceId::Button1 as InputId,
                Box::new(|value: bool| {
                    unsafe {
                        (*APP).trigger_note(value);
                    }
                    true
                }),
            ))
            .unwrap();
        buttons
            .push(Button::new(
                Point::new(26, 0),
                "P2",
                InputDeviceId::Button2 as InputId,
                Box::new(|_value: bool| true),
            ))
            .unwrap();
        buttons
            .push(Button::new(
                Point::new(51, 0),
                "P3",
                InputDeviceId::Button3 as InputId,
                Box::new(|_value: bool| true),
            ))
            .unwrap();
        buttons
            .push(Button::new(
                Point::new(77, 0),
                "P4",
                InputDeviceId::Button4 as InputId,
                Box::new(|_value: bool| true),
            ))
            .unwrap();
        buttons
            .push(Button::new(
                Point::new(102, 0),
                "P5",
                InputDeviceId::Button5 as InputId,
                Box::new(|_value: bool| true),
            ))
            .unwrap();

        buttons
    }

    fn setup_ui(&mut self) {
        self.panel = Some(Panel::new(self.setup_buttons(), self.setup_knobs()))
    }

    pub fn trigger_note(&mut self, trigger: bool) {
        unsafe {
            SetGate(trigger);
        }
    }

    fn update_knobs(&mut self) {
        if let Some(panel) = &mut self.panel {
            panel.input_update(
                InputDeviceId::Knob1 as InputId,
                Value::Int(self.encoders.0.read_enc() as i32),
            );
            panel.input_update(
                InputDeviceId::Knob2 as InputId,
                Value::Int(self.encoders.1.read_enc() as i32),
            );
            panel.input_update(
                InputDeviceId::Knob3 as InputId,
                Value::Int(self.encoders.2.read_enc() as i32),
            );
            panel.input_update(
                InputDeviceId::Knob4 as InputId,
                Value::Int(self.encoders.3.read_enc() as i32),
            );
        };
    }

    fn update_buttons(&mut self) {
        if let Some(panel) = &mut self.panel {
            panel.input_update(
                InputDeviceId::Button1 as InputId,
                Value::Bool(!self.button_pins.0.is_high().unwrap()),
            );
            panel.input_update(
                InputDeviceId::Button2 as InputId,
                Value::Bool(!self.button_pins.1.is_high().unwrap()),
            );
            panel.input_update(
                InputDeviceId::Button3 as InputId,
                Value::Bool(!self.button_pins.2.is_high().unwrap()),
            );
            panel.input_update(
                InputDeviceId::Button4 as InputId,
                Value::Bool(!self.button_pins.3.is_high().unwrap()),
            );
            panel.input_update(
                InputDeviceId::Button5 as InputId,
                Value::Bool(!self.button_pins.4.is_high().unwrap()),
            );
        }
    }

    fn update(&mut self) {
        self.update_knobs();
        self.update_buttons();

        if let Some(panel) = &mut self.panel {
            let invalidate = panel.render(&mut self.display);
            if invalidate.1.width != 0 && invalidate.1.height != 0 {
                self.display
                    .flush_region_graphics(invalidate, &mut self.delay)
                    .expect("could not flush display");
            }
        }
    }
}

static mut APP: *mut App = 0 as *mut App;

#[entry]
fn main() -> ! {
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    let mut app = App::new();
    unsafe {
        APP = &mut app as *mut App;
    }

    app.setup_ui();
    loop {
        app.update();
    }
}

#[interrupt]
fn DMA1_STREAM5() {
    unsafe {
        Elements_DMA1_Stream5_IRQHandler();
    }
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();

    loop {}
}
