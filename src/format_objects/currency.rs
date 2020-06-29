use std::fmt::{Display, Formatter};
use std::fmt;

pub struct Currency {
    pub value: String,
    pub is_neg: bool,

}
// 10000.10
// dec_place = 5
// dec_place / 3.0 = 1.666666666666667

impl Currency {
    pub fn new( value: f64 ) -> Self {
        let decimal_place_cnt: usize = 2;
        let mut cur_str = format!("{:.1$}", value, decimal_place_cnt);
        let dec_place = cur_str.find('.').unwrap();
        let n = (dec_place as f64 / 3.0).floor() as usize;
        let mod_u = (dec_place as f64 % 3.0) as usize;
        let mut add_x_units = 0;
        let mut add_one = false;
        if n == 1 && mod_u == 0 {
            add_x_units = 0;
            add_one = false;
        } else if n > 0 {
            if n == 1 {
                add_x_units = 0;
                add_one = true;
            } else {
                add_x_units = n - 1;
                add_one = true;
            }
        }

        let start_at = mod_u + add_x_units * 3;

        let mut n = 0;
        while n < add_x_units || add_one {
            cur_str.insert(start_at - n * 3, ',');

            add_one = false;
            if n == add_x_units {
                break;
            }

            n = n + 1;
        }

        Self {
            is_neg: value < 0.0,
            value: cur_str,
        }
    }
}

impl Display for Currency {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value )
    }
}