
#![no_std]
#![no_main]


// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;


use fugit::ExtU32;
use usbd_human_interface_device::device::consumer::MultipleConsumerReport;

use cortex_m::prelude::_embedded_hal_timer_CountDown;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;

// The macro for our start-up function
use rp_pico::entry;

// Some traits we need
// use embedded_hal::digital::v2::OutputPin;
// use rp2040_hal::clocks::Clock;

// use hal::pac::interrupt;
// use hal::pac::Peripherals;
// use hal::pac::CorePeripherals;
// use cortex_m::interrupt::free as disable_interrupts;

use usb_device::bus::UsbBusAllocator;

use embedded_hal::digital::v2::*;
// use embedded_hal::prelude::*;
use usbd_human_interface_device::page::{Consumer};

// USB Device support
use usb_device::class_prelude::*;
use usb_device::prelude::*;
use usbd_human_interface_device::prelude::UsbHidClassBuilder;
// use usbd_hid::descriptor::generator_prelude::*;
// use usbd_hid::descriptor::{MouseReport, SerializedDescriptor};
// use usbd_hid::hid_class::HIDClass;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
/// Note: This boot block is not necessary when using a rp-hal based BSP
/// as the BSPs already perform this step.
// #[link_section = ".boot2"]
// #[used]
// pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;



// use fugit::MicrosDurationU32;

/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz. Adjust
/// if your board has a different frequency
// const XTAL_FREQ_HZ: u32 = 12_000_000u32;


// static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
// static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
// static mut USB_HID: Option<HIDClass<UsbBus>> = None;



// const KEYBOARD_MOUSEKEYBOARD_MOUSE_POLL: MicrosDurationU32 = MicrosDurationU32::millis(10);
// const KEYBOARD_MOUSE_POLL: MicrosDurationU32 = MicrosDurationU32::millis(10);
// const CONSUMER_POLL: MicrosDurationU32 = MicrosDurationU32::millis(50);
// const WRITE_PENDING_POLL: MicrosDurationU32 = MicrosDurationU32::millis(10);

/// Entry point to our bare-metal application.
///
/// The `#[rp2040_hal::entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables and the spinlock are initialised.
///
/// The function configures the RP2040 peripherals, then toggles a GPIO pin in
/// an infinite loop. If there is an LED connected to that pin, it will blink.
#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // let core = pac::CorePeripherals::take().unwrap();


    // let mut peripherals = Peripherals::take().unwrap();
    // let core = CorePeripherals::take().unwrap();
    // let mut core = CorePeripherals::take().unwrap();
    // let mut clocks = GenericClockController::with_internal_32kosc(
    //     peripherals.GCLK,
    //     &mut peripherals.PM,
    //     &mut peripherals.SYSCTRL,
    //     &mut peripherals.NVMCTRL,
    // );


    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());


    // Set up the USB driver
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    let mut consumer = UsbHidClassBuilder::new()
        .add_interface(
            usbd_human_interface_device::device::consumer::ConsumerControlInterface::default_config(
            ),
        )
        .build(&usb_bus);


       //https://pid.codes
       let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1209, 0x0001))
       .manufacturer("usbd-human-interface-device")
       .product("Consumer Control")
       .serial_number("TEST")
       .build();

       let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS);

       let sio = hal::Sio::new(pac.SIO);
       let pins = hal::gpio::Pins::new(
           pac.IO_BANK0,
           pac.PADS_BANK0,
           sio.gpio_bank0,
           &mut pac.RESETS,
       );
       let keys: &[&dyn InputPin<Error = core::convert::Infallible>] = &[
        &pins.gpio1.into_pull_up_input(),
       ];



       let mut last = get_report(keys);

       let mut input_count_down = timer.count_down();
       input_count_down.start(50.millis());

       loop {
           //Poll the keys every 10ms
           if input_count_down.wait().is_ok() {
               let report = get_report(keys);
               if report != last {
                   match consumer.interface().write_report(&report) {
                       Err(UsbError::WouldBlock) => {}
                       Ok(_) => {
                           last = report;
                       }
                       Err(e) => {
                           core::panic!("Failed to write consumer report: {:?}", e)
                       }
                   }
               }
           }

           if usb_dev.poll(&mut [&mut consumer]) {}
       }
   }

   fn get_report(keys: &[&dyn InputPin<Error = core::convert::Infallible>]) -> MultipleConsumerReport {
       #[rustfmt::skip]
           let keys = [
        //    if keys[0].is_low().unwrap() { Consumer::PlayPause } else { Consumer::Unassigned },
        //    if keys[1].is_low().unwrap() { Consumer::ScanPreviousTrack } else { Consumer::Unassigned },
        //    if keys[2].is_low().unwrap() { Consumer::ScanNextTrack } else { Consumer::Unassigned },
           if keys[0].is_low().unwrap() { Consumer::Mute } else { Consumer::Unassigned },
        //    if keys[4].is_low().unwrap() { Consumer::VolumeDecrement } else { Consumer::Unassigned },
        //    if keys[5].is_low().unwrap() { Consumer::VolumeIncrement } else { Consumer::Unassigned },
        //    if keys[6].is_low().unwrap() { Consumer::ALCalculator } else { Consumer::Unassigned },
        //    if keys[7].is_low().unwrap() { Consumer::ALInternetBrowser } else { Consumer::Unassigned },
        //    if keys[8].is_low().unwrap() { Consumer::ALFileBrowser } else { Consumer::Unassigned },
       ];

       let mut report = MultipleConsumerReport {
           codes: [Consumer::Unassigned; 4],
       };

       let mut it = keys.iter().filter(|&&c| c != Consumer::Unassigned);
       for c in report.codes.iter_mut() {
           if let Some(&code) = it.next() {
               *c = code;
           } else {
               break;
           }
       }
       report
   }