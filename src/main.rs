#![no_std]
#![no_main]

use gba::prelude::*;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    #[cfg(debug_assertions)]
    if let Ok(mut logger) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Fatal) {
        use core::fmt::Write;
        writeln!(logger, "{info}").ok();
    }
    loop {}
}

extern "C" fn irq_handler(_bits: IrqBits) {}

#[no_mangle]
extern "C" fn main() -> ! {
    // interrupt configuration
    RUST_IRQ_HANDLER.write(Some(irq_handler));
    DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
    IE.write(IrqBits::VBLANK);
    IME.write(true);

    // bg
    BG_PALETTE.index(1).write(Color::MAGENTA);

    Cga8x8Thick.bitunpack_4bpp(CHARBLOCK0_4BPP.as_region(), 0);

    BG0CNT.write(BackgroundControl::new().with_screenblock(8));
    let screenblock = TEXT_SCREENBLOCKS.get_frame(8).unwrap();

    DISPCNT.write(DisplayControl::new().with_show_bg0(true));

    for (i, &c) in b"Pre  VBlankIntrWait".iter().enumerate() {
        screenblock
            .get(5 + i, 5)
            .unwrap()
            .write(TextEntry::from_tile(c.into()));
    }

    loop {
        // wait for vblank, update graphics afterwards
        VBlankIntrWait();

        for (i, &c) in b"Post VBlankIntrWait".iter().enumerate() {
            screenblock
                .get(5 + i, 5)
                .unwrap()
                .write(TextEntry::from_tile(c.into()));
        }
    }
}
