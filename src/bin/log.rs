#![no_std]
#![no_main]

use cyw43::{Runner, SpiBus, aligned_bytes};
use cyw43_pio::{PioSpi, RM2_CLOCK_DIVIDER};
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, DMA_CH1, PIO0, PIO1};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::pio_programs::ws2812::{PioWs2812, PioWs2812Program};
use embassy_rp::{bind_interrupts, dma};
use embassy_time::{Duration, Timer};
use embedded_alloc::LlffHeap as Heap;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    info!("Start");

    loop {
        Timer::after(Duration::from_millis(1_000)).await;
        info!("Loop");
    }
}
