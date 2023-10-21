pub trait VRGDA {
    fn get_price(&self, sec_since_start: i64, num_sold: i64) -> f64;
}

pub struct LinearVRGDA {
    pub target_price: f64,
    pub per_period_price_dec: f64,
    pub time_scale: f64,
}

pub struct LogisticVRGDA {
    pub target_price: f64,
    pub per_period_price_dec: f64,
    pub time_scale: f64,
    pub logistic_scale: f64,
}

impl VRGDA for LinearVRGDA {
    fn get_price(&self, sec_since_start: i64, num_sold: i64) -> f64 {
        let days_since_start: f64 = (sec_since_start as f64) / 86400.0;
        let f_inv = num_sold as f64 / self.time_scale;
        return self.target_price
            * ((-(1.0 - self.per_period_price_dec).ln() * (f_inv - days_since_start)).exp());
    }
}

impl VRGDA for LogisticVRGDA {
    fn get_price(&self, sec_since_start: i64, num_sold: i64) -> f64 {
        let days_since_start: f64 = (sec_since_start as f64) / 86400.0;
        let initial_value = self.logistic_scale / 2.0; // no time_shift
        let logistic_value = num_sold as f64 + initial_value;
        let base = 1.0 - self.per_period_price_dec;
        let exp = days_since_start
            + ((-1.0 + self.logistic_scale / logistic_value).ln() / self.time_scale);
        return self.target_price * base.powf(exp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logistic() {
        let logistic = LogisticVRGDA {
            target_price: 100.0,
            per_period_price_dec: 0.31,
            time_scale: 0.066,
            logistic_scale: 1002.0,
        };

        let test_cases = [
            (3846533, 242, 0.002579233482286165),
            (6003305, 201, 0.00000007766500411351446),
            (3, 68, 475.1760319926989),
        ];

        for (sec_since_start, num_sold, expected) in test_cases.iter() {
            let result = logistic.get_price(*sec_since_start, *num_sold + 1);
            assert_eq!(result, *expected);
        }
    }

    #[test]
    fn test_linear() {
        let linear = LinearVRGDA {
            target_price: 0.0042069,
            per_period_price_dec: 0.31,
            time_scale: 5.0,
        };

        let test_cases = [
            (3846533, 242, 0.019118269207403576),
            (6003305, 201, 0.00000008654842363564268),
            (3, 68, 0.7044321050537729),
        ];

        for (sec_since_start, num_sold, expected) in test_cases.iter() {
            let result = linear.get_price(*sec_since_start, *num_sold + 1);
            assert_eq!(result, *expected);
        }
    }
}
