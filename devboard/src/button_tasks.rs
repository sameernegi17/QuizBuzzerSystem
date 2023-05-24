#[macro_export]
macro_rules! button_task {
    // Optimization for one field
    ($name:ident) => {
        #[embassy_executor::task]
        async fn $name(
            id: usize,
            button: embassy_stm32::gpio::AnyPin,
            exti_inp: embassy_stm32::exti::AnyChannel,
        ) -> ! {
            let button = embassy_stm32::gpio::Input::new(button, embassy_stm32::gpio::Pull::Up);
            let mut button = embassy_stm32::exti::ExtiInput::new(button, exti_inp);

            loop {
                button.wait_for_rising_edge().await;
                devboard::Q
                    .enqueue((id, embassy_time::Instant::now().as_millis()))
                    .ok();
                embassy_time::Timer::after(embassy_time::Duration::from_millis(DEBOUNCE_MS)).await;
            }
        }
    };
}

#[macro_export]
macro_rules! button_tasks {
    ($($name:ident),+) => {
        $(
            button_task!($name);
        )+
    };
}

pub use button_task;
pub use button_tasks;
