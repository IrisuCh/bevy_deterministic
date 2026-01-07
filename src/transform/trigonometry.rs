use crate::Fx;

#[inline]
pub fn sin_cos_fixed(angle: Fx) -> (Fx, Fx) {
    let pi = Fx::PI;
    let pi2 = Fx::from_num(2) * pi;

    // 1. Нормализация угла в диапазон [-π, π]
    let mut x = angle % pi2;
    if x > pi {
        x -= pi2;
    }
    if x < -pi {
        x += pi2;
    }

    // 2. Вспомогательная функция для синуса (аппроксимация Паде/Бхаскара)
    // Работает для диапазона [-π, π]
    let calculate_sin = |val: Fx| -> Fx {
        let four = Fx::from_num(4);
        let five = Fx::from_num(5);

        // Формула: 16x(π - |x|) / (5π² - 4|x|(π - |x|))
        let abs_x = val.abs();
        let common = val * (pi - abs_x);
        let denominator = five * pi * pi - four * abs_x * (pi - abs_x);
        Fx::from_num(16) * common / denominator
    };

    // 3. Вычисляем синус напрямую
    let s = calculate_sin(x);

    // 4. Вычисляем косинус через сдвиг фазы БЕЗ рекурсии
    let mut x_cos = x + Fx::FRAC_PI_2;
    if x_cos > pi {
        x_cos -= pi2;
    } // Ручная нормализация
    let c = calculate_sin(x_cos);

    (s, c)
}
