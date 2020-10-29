use ordered_float::OrderedFloat;

// TODO transform it to a Trait applied on the Vec structure instead
#[derive(Debug, Default, PartialEq)]
pub struct Statistic {
    length: usize,
    max: Option<f64>,
    // TODO compute it on a lazy loading way instead
    mean: Option<f64>,
    min: Option<f64>,
    standard_deviation: Option<f64>,
    sum: f64,
    variance: Option<f64>,
}

impl Statistic {
    pub fn new(data: &Vec<f64>) -> Self {
        let mut statistic = Self {
            length: data.len(),
            max: match data.iter().max_by_key(|n| OrderedFloat(n.abs())) {
                Some(&result) => Some(result),
                _ => None,
            },
            min: match data.iter().min_by_key(|n| OrderedFloat(n.abs())) {
                Some(&result) => Some(result),
                _ => None,
            },
            sum: data.iter().sum::<f64>(),
            ..Default::default()
        };
        statistic.mean = match statistic.length {
            positive if positive > 0 => Some(statistic.sum / statistic.length as f64),
            _ => None,
        };
        statistic.variance = match statistic.length {
            positive if positive > 0 => Some(
                data.iter()
                    .map(|&value| {
                        // FIXME manage the unwrap() into the match
                        let diff = statistic.mean.unwrap() - value;
                        diff * diff
                    })
                    .sum::<f64>()
                    / statistic.length as f64,
            ),
            _ => None,
        };
        statistic.standard_deviation = match statistic.variance {
            Some(value) => Some(value.sqrt()),
            _ => None,
        };
        statistic
    }
}

#[test]
fn test_3_values() {
    let statistic = Statistic::new(&[150.0, 50.0, 350.0].to_vec());
    assert_eq!(Some(50.0), statistic.min);
    assert_eq!(Some(350.0), statistic.max);
    assert_eq!(Some(183.33333333333334), statistic.mean);
    assert_eq!(550.0, statistic.sum);
    assert_eq!(3, statistic.length);
    assert_eq!(Some(15555.555555555557), statistic.variance);
    assert_eq!(Some(124.72191289246472), statistic.standard_deviation);
}

#[test]
fn test_3_first_values() {
    let statistic = Statistic::new(&[1.0, 2.0, 3.0].to_vec());
    assert_eq!(Some(1.0), statistic.min);
    assert_eq!(Some(3.0), statistic.max);
    assert_eq!(Some(2.0), statistic.mean);
    assert_eq!(6.0, statistic.sum);
    assert_eq!(3, statistic.length);
    assert_eq!(Some(0.6666666666666666), statistic.variance);
    assert_eq!(Some(0.816496580927726), statistic.standard_deviation);
}
