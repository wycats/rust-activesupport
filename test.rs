extern mod active_support;
use active_support::Period;
use active_support::date::Date;

#[test]
pub fn test_add_months() {
    let start_date = Date{
        year: 2013,
        month: 11u,
        day: 20u,
    };

    assert_eq!(
        start_date.advance_months(11),
        Date{
            year: 2014,
            month: 10u,
            day: 20u,
        }
    );

    assert_eq!(
        start_date.advance_months(12),
        Date{
            year: 2014,
            month: 11u,
            day: 20u,
        }
    );

    assert_eq!(
        start_date.advance_months(13),
        Date{
            year: 2015,
            month: 0u,
            day: 20u,
        }
    );
}

// Building these tests with: ```rustc lib.rs && rustc --test test.rs -L .```.