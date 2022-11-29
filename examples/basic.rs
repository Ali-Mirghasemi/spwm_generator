use spwm_generator::*;

fn main() {
    let wave = SinWave::new_offset(1.0, 1.0, 1.0);

    let samples = wave.samples_range(0.0, 1.0, 0.125);
    println!("{:?}", samples);

    let pwm = PWM::new(wave, 255, 0);

    let duty_cycles = pwm.duty_cycles_fixed(0.0, 1.0, 16);
    println!("{:?}", duty_cycles);

    let spwm = SPWM::new(1.0, 100.0, 1000, 0);
    let table = spwm.lookup_table();

    print!("{:?}", table);
}
