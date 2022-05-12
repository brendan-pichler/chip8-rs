use super::Cpu;

pub fn update_timers(cpu: &mut Cpu) {
    if cpu.delay_timer > 0 {
        cpu.delay_timer -= 1;
    }

    if cpu.sound_timer > 0 {
        if cpu.sound_timer == 1 { println!("BEEP!") }
        cpu.sound_timer -= 1;
    }
}