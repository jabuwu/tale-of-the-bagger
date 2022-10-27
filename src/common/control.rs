use std::{
    f32::consts::PI,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Default, Clone)]
pub struct SecondOrderController<T> {
    input_previous: T,
    output: T,
    output_velocity: T,

    frequency: f32,
    damping: f32,
    response: f32,

    k1: f32,
    k2: f32,
    k3: f32,
}

impl<
        T: Default
            + Copy
            + Sub<T, Output = T>
            + Div<f32, Output = T>
            + Add<T, Output = T>
            + Mul<f32, Output = T>,
    > SecondOrderController<T>
{
    pub fn new(initial: T, frequency: f32, damping: f32, response: f32) -> Self {
        let mut control = Self {
            input_previous: initial,
            output: initial,
            output_velocity: T::default(),
            frequency,
            damping,
            response,
            k1: 0.,
            k2: 0.,
            k3: 0.,
        };
        control.calculate_constants();
        control
    }

    fn calculate_constants(&mut self) {
        self.k1 = self.damping / (PI * self.frequency);
        self.k2 = 1. / (2. * PI * self.frequency).powf(2.);
        self.k3 = self.response * self.damping / (2. * PI * self.frequency);
    }

    pub fn update(&mut self, input: T, delta_seconds: f32) -> T {
        let k2_stable = self
            .k2
            .max(delta_seconds * delta_seconds / 2. + delta_seconds * self.k1 / 2.)
            .max(delta_seconds * self.k1);

        let vec_velocity = (input - self.input_previous) / delta_seconds;
        self.input_previous = input;
        self.output = self.output + self.output_velocity * delta_seconds;
        self.output_velocity = self.output_velocity
            + (input + vec_velocity * self.k3 - self.output - self.output_velocity * self.k1)
                / k2_stable
                * delta_seconds;
        self.output
    }
}
