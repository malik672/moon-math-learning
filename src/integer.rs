use rand::Rng;

/// Represents an integer value.
#[derive(Clone, Copy, Debug)]
pub struct Integer(pub i64);

impl Integer {
    /// returns a random number
    pub fn random_element(max: i64) -> Self {
        let mut rng = rand::thread_rng();
        Integer(rng.gen_range(0..max))
    }

    /// returns an addition of two numbers
    pub fn add(&self, other: Integer) -> Integer {
        Integer(self.0 + other.0)
    }

    /// returns a multiplication of two integers
    pub fn multiply(&self, other: Integer) -> Integer {
        Integer(self.0 * other.0)
    }

    /// returns a division of two integers
    pub fn div(&self, other: Integer) -> Integer {
        Integer(self.0 / other.0)
    }

    /// performs a modulo and returns the remainder
    pub fn rem(&self, other: Integer) -> Integer {
        Integer(self.0 % other.0)
    }

    /// checks if it's a divisor
    pub fn divides(&self, other: Integer) -> bool {
        other.0 % self.0 == 0
    }

    /// performs euclidean_div
    pub fn euclidean_div(&self, other: Integer) -> (Integer, Integer) {
        (self.div(other), self.rem(other))
    }

    /// returns the gcd(greatest common denominator)
    pub fn gcd(&mut self, mut other: Integer) -> Integer {
        while other.0 != 0 {
            let temp = other.0;
            other.0 = self.0 % other.0;
            other.0 = temp;
        }
        Integer(self.0)
    }

    /// return all factors of an integer
    pub fn factor(&self) -> String {
        let mut factors = Vec::new();
        let mut number = self.0;
        let mut divisor = 2;

        while number >= divisor * divisor {
            let mut count = 0;
            while number % divisor == 0 {
                count += 1;
                number /= divisor;
            }
            if count > 0 {
                factors.push((divisor, count));
            }
            divisor += 1;
        }

        if number > 1 {
            factors.push((number, 1));
        }

        factors
            .into_iter()
            .map(|(prime, exponent)| {
                if exponent == 1 {
                    format!("{}", prime)
                } else {
                    format!("{}^{}", prime, exponent)
                }
            })
            .collect::<Vec<String>>()
            .join(" * ")
    }
}
