
#[cfg(test)]
mod tests {
    use std::ops::Range;

    use crate::range::Range2;
    use super::super::*;

    
    #[test]
    fn series_of_changes_simple_np_remained() {

        let mut list_object: Vec<String> = list_object();

        // (3..6)
        // [A  B  C  A2  B2  C2  A3  B3  C3]
        // [         A2  B2  C2            ]
        let mut range = Range2::new(3, 3);

        let series_of_changes = ChangeRequests {
            requests: vec![

                // Shift to the end by 2 elements,
                // [A  B  C  A2  B2  C2  A3  B3  C3]
                // [         A2  B2  C2            ]
                // Shift to end by 2, so the range will then be (5..8)
                // [                 C2  A3  B3    ]
                // [A  B  C  A2  B2  C2  A3  B3  C3]
                Change::shift(2, ChangeDirection::End),

                // [A  B  C  A2  B2  C2  A3  B3  C3]
                // [                 C2  A3  B3    ]
                // Add the range:
                // [                 New  New      ]
                // so the range will be (5..10)
                // [                 New  New  C2  A3  B3    ]
                // [A  B  C  A2  B2  New  New  C2  A3  B3  C3]
                Change::add(5, 2),
                
                // [A  B  C  A2  B2  New  New  C2  A3  B3  C3]
                // [                 New  New  C2  A3  B3    ]
                // Shift to the start by 3 elements, so the range will be (2..7)
                // [      C  A2  B2  New  New                ]
                // [A  B  C  A2  B2  New  New  C2  A3  B3  C3]
                Change::shift(3, ChangeDirection::Start),
                
                // [A  B  C  A2  B2  New  New  C2  A3  B3  C3]
                // [      C  A2  B2  New  New                ]
                // Remove the range:
                // [   B  C  A2  B2                          ]
                // so the range will decrease in length, and will be (1..4)
                // [   New  New                ]
                // [A  New  New      A3  B3  C3]
                Change::remove(1, 4),
            ],
        };
        
        let mut summary = ChangeSessionSummary::new();
        
        handle_changes(&series_of_changes, &mut list_object, &mut range, &mut summary, true);
        
        let vec: Vec<String> = vec!["New_0".into(), "New_1".into()];

        assert_eq!(&list_object[range.range()], vec.as_slice());

        eprintln!("{:?}", &list_object[range.range()]);
    }

    #[test]
    fn series_of_changes_simple_with_remained() {

        let mut list_object: Vec<String> = list_object();
        let list_object_original = list_object.clone();

        // (3..6)
        // [A  B  C  A2  B2  C2  A3  B3  C3]
        // [         A2  B2  C2            ]
        let mut range = Range2::new(3, 3);

        let series_of_changes = ChangeRequests {
            requests: vec![

                // Shift to the end by 2 elements,
                // [A  B  C  A2  B2  C2  A3  B3  C3]
                // [         A2  B2  C2            ]
                // Shift to end by 2, so the range will then be (5..8)
                // [                 C2  A3  B3    ]
                // [A  B  C  A2  B2  C2  A3  B3  C3]
                Change::shift(2, ChangeDirection::End),

                // [A  B  C  A2  B2  C2  A3  B3  C3]
                // [                 C2  A3  B3    ]
                // Add the range:
                // [                 New  New      ]
                // so the range will be (5..10)
                // [                 New  New  C2  A3  B3    ]
                // [A  B  C  A2  B2  New  New  C2  A3  B3  C3]
                Change::add(5, 2),
                
                // [A  B  C  A2  B2  New  New  C2  A3  B3  C3]
                // [                 New  New  C2  A3  B3    ]
                // Shift to the start by 2 elements, so the range will be (2..7)
                // [         A2  B2  New  New  C2            ]
                // [A  B  C  A2  B2  New  New  C2  A3  B3  C3]
                Change::shift(2, ChangeDirection::Start),
                
                // [A  B  C  A2  B2  New  New  C2  A3  B3  C3]
                // [         A2  B2  New  New  C2             ]
                // Remove the range:
                // [   B  C  A2  B2                          ]
                // so the range will decrease in length, and will be (1..4)
                // [   New  New  C2            ]
                // [A  New  New  C2  A3  B3  C3]
                Change::remove(1, 4),
            ],
        };
        
        let mut summary = ChangeSessionSummary::new();

        handle_changes(&series_of_changes, &mut list_object, &mut range, &mut summary, true);
        
        let vec: Vec<String> = vec!["New_0".into(), "New_1".into(), "C2".into()];

        assert_eq!(&list_object[range.range()], vec.as_slice());

        let remained = summary.remained().last().unwrap();

        assert_eq!(remained.remained_current(), 3..4);
        assert_eq!(remained.remained_original(), 5..6);
        assert_eq!(list_object[remained.remained_current()].last().unwrap().as_str(), "C2");
        assert_eq!(list_object_original[remained.remained_original()].last().unwrap().as_str(), "C2");
    }

    #[test]
    fn series_of_changes_long() {

        let mut list_object: Vec<String> = list_object();

        // (3..6)
        // [A  B  C  A2  B2  C2  A3  B3  C3]
        // [         A2  B2  C2            ]
        let mut range = Range2::new(3, 3);

        let series_of_changes = ChangeRequests {
            requests: vec![

                // Shift to the end by 2 elements,
                // [A  B  C  A2  B2  C2  A3  B3  C3]
                // [         A2  B2  C2            ]
                // Shift to end by 2, so the range will then be (5..8)
                // [                 C2  A3  B3    ]
                // [A  B  C  A2  B2  C2  A3  B3  C3]
                Change::shift(2, ChangeDirection::End),

                // [A  B  C  A2  B2  C2  A3  B3  C3]
                // [                 C2  A3  B3    ]
                // Add the range:
                // [                 New  New      ]
                // so the range will be (5..10)
                // [                 New  New  C2  A3  B3    ]
                // [A  B  C  A2  B2  New  New  C2  A3  B3  C3]
                Change::add(5, 2),
                
                // [A  B  C  A2  B2  New  New  C2  A3  B3  C3]
                // [                 New  New  C2  A3  B3    ]
                // Shift to the start by 3 elements, so the range will be (2..7)
                // [      C  A2  B2  New  New                ]
                // [A  B  C  A2  B2  New  New  C2  A3  B3  C3]
                Change::shift(3, ChangeDirection::Start),
                
                // [A  B  C  A2  B2  New  New  C2  A3  B3  C3]
                // [      C  A2  B2  New  New                ]
                // Remove the range:
                // [   B  C  A2  B2                          ]
                // so the range will decrease in length, and will be (1..4)
                // [   New  New                ]
                // [A  New  New      A3  B3  C3]
                Change::remove(1, 4),

                // [A  New  New  C2  A3  B3  C3]
                // [   New  New                ]
                // Add the range:
                // [                 New  New  New]
                // so the range will be (1..4)
                // [   New  New                               ]
                // [A  New  New  C2  New  New  New  A3  B3  C3]
                Change::add(4, 3),

                // [A  New  New  C2  New  New  New  A3  B3  C3]
                // [   New  New                               ]
                // Remove the range:
                // [             C2  New                      ]
                // so the range will will be (1..3)
                // [   New  New                      ]
                // [A  New  New  New  New  A3  B3  C3]
                Change::remove(3, 2),

                // [A  New  New  New  New  A3  B3  C3]
                // [   New  New                      ]
                // Remove the range:
                // [             New  New  A3        ]
                // so the range will will be (1..3)
                // [   New  New                      ]
                // [A  New  New  B3  C3]
                Change::remove(3, 2),

                // [A  New  New  B3  C3]
                // [   New  New        ]
                // Remove the range:
                // [        New  B3    ]
                // so the range will will be (1..2)
                // [   New    ]
                // [A  New  C3]
                Change::remove(2, 1),

                // this will result in empty range,
                // do we consider this a valid range? (for example a caret range?)

                // // [A  New  C3]
                // // [   New    ]
                // // Remove the range:
                // // [   New  C3]
                // // so the range will will be (1..2)
                // // [ ]
                // // [A]
                // Change::remove(2, 2),
            ],
        };
        
        let mut summary = ChangeSessionSummary::new();

        handle_changes(&series_of_changes, &mut list_object, &mut range, &mut summary, true);
        
        let vec: Vec<String> = vec!["New_0".into()];

        assert_eq!(&list_object[range.range()], vec.as_slice());

        eprintln!("Range was {:?}", &list_object[range.range()]);
    }

    #[test]
    fn series_of_changes_long_zero_range() {

        let mut list_object: Vec<String> = list_object();

        // (3..6)
        // [A  B  C  A2  B2  C2  A3  B3  C3]
        // [         A2  B2  C2            ]
        let mut range = Range2::new(3, 3);

        let series_of_changes = ChangeRequests {
            requests: vec![

                // Shift to the end by 2 elements,
                // [A  B  C  A2  B2  C2  A3  B3  C3]
                // [         A2  B2  C2            ]
                // Shift to end by 2, so the range will then be (5..8)
                // [                 C2  A3  B3    ]
                // [A  B  C  A2  B2  C2  A3  B3  C3]
                Change::shift(2, ChangeDirection::End),

                // [A  B  C  A2  B2  C2  A3  B3  C3]
                // [                 C2  A3  B3    ]
                // Add the range:
                // [                 New  New      ]
                // so the range will be (5..10)
                // [                 New  New  C2  A3  B3    ]
                // [A  B  C  A2  B2  New  New  C2  A3  B3  C3]
                Change::add(5, 2),
                
                // [A  B  C  A2  B2  New  New  C2  A3  B3  C3]
                // [                 New  New  C2  A3  B3    ]
                // Shift to the start by 3 elements, so the range will be (2..7)
                // [      C  A2  B2  New  New                ]
                // [A  B  C  A2  B2  New  New  C2  A3  B3  C3]
                Change::shift(3, ChangeDirection::Start),
                
                // [A  B  C  A2  B2  New  New  C2  A3  B3  C3]
                // [      C  A2  B2  New  New                ]
                // Remove the range:
                // [   B  C  A2  B2                          ]
                // so the range will decrease in length, and will be (1..4)
                // [   New  New                ]
                // [A  New  New      A3  B3  C3]
                Change::remove(1, 4),

                // [A  New  New  C2  A3  B3  C3]
                // [   New  New                ]
                // Add the range:
                // [                 New  New  New]
                // so the range will be (1..4)
                // [   New  New                               ]
                // [A  New  New  C2  New  New  New  A3  B3  C3]
                Change::add(4, 3),

                // [A  New  New  C2  New  New  New  A3  B3  C3]
                // [   New  New                               ]
                // Remove the range:
                // [             C2  New                      ]
                // so the range will will be (1..3)
                // [   New  New                      ]
                // [A  New  New  New  New  A3  B3  C3]
                Change::remove(3, 2),

                // [A  New  New  New  New  A3  B3  C3]
                // [   New  New                      ]
                // Remove the range:
                // [             New  New  A3        ]
                // so the range will will be (1..3)
                // [   New  New                      ]
                // [A  New  New  B3  C3]
                Change::remove(3, 2),

                // [A  New  New  B3  C3]
                // [   New  New        ]
                // Remove the range:
                // [        New  B3    ]
                // so the range will will be (1..2)
                // [   New    ]
                // [A  New  C3]
                Change::remove(2, 1),

                // this will result in empty range,
                // do we consider this a valid range? (for example a caret range?)

                // [A  New  C3]
                // [   New    ]
                // Remove the range:
                // [   New  C3]
                // so the range will will be (1..2)
                // [ ]
                // [A]
                Change::remove(1, 2),
            ],
        };
        
        let mut summary = ChangeSessionSummary::new();

        handle_changes(&series_of_changes, &mut list_object, &mut range, &mut summary, true);
        
        let vec: Vec<String> = vec![];

        assert_eq!(&list_object[range.range()], vec.as_slice());
        assert_eq!(range.len(), 0);

        eprintln!("Range was {:?}", &list_object[range.range()]);
    }

    fn list_object() -> Vec<String> {
        vec![
            // 0
            "A".into(),
            // 1
            "B".into(),
            // 2
            "C".into(),
            // 3
            "A2".into(),
            // 4
            "B2".into(),
            // 5
            "C2".into(),
            // 6
            "A3".into(),
            // 7
            "B3".into(),
            // 8
            "C3".into(),
        ]
    }

    fn handle_changes(
        series_of_changes: &ChangeRequests, 
        list_object: &mut Vec<String>,
        range: &mut Range2,
        summary: &mut ChangeSessionSummary,
        debugging: bool,
    ) {

        match debugging {
            true => {
                let list_object_original = list_object.clone();
                eprintln!("{:#?}", series_of_changes);
        
                eprintln!("Initial list_object {:?}", &list_object);
                eprintln!(" \n .. .. .. .. \n");
        
                for change in series_of_changes.requests.iter() {
                    eprintln!(" <*> <*> {:?}", change);
                    // eprintln!("Current range {:?}", range.range());
                    // eprintln!("Current span {:?}. Current list object {:?}", &list_object[range.range()], &list_object);
                    print_ranged_and_full_list_object("Current", &list_object, range.range());
                    range.change_cont(change, summary);
                    handle_add_and_remove(change, list_object);
                    // eprintln!("Resulting range {:?}", range.range());
                    // eprintln!("Resulting span {:?}. Resulting list object {:?}", &list_object[range.range()], &list_object);
                    print_ranged_and_full_list_object("Resulting", &list_object, range.range());
                    // summary.remained().last().map(|r| {
                    //     eprintln!("Remained current is {:?}, (original {:?})", r.remained_current(), r.remained_original());
                    // });
                    if let Some(remained) = summary.remained().last() {
                        // eprintln!("Remained is {:?}", &list_object[remained.remained_current()]);
                        print_ranged_and_full_list_object("Remained in latest(cur.)", &list_object, remained.remained_current());
                        print_ranged_and_full_list_object("Remained in orig.", &list_object_original, remained.remained_original());
                    } else {
                        println!("No remained ranges..");
                    }
                    eprintln!(" \n .. .. .. .. \n");
                }
            }
            false => {
                eprintln!("{:#?}", series_of_changes);
        
                eprintln!("Initial list_object {:?}", &list_object);
                eprintln!(" \n .. .. .. .. \n");
        
                for change in series_of_changes.requests.iter() {
                    handle_add_and_remove(change, list_object);
                }

                series_of_changes.apply_changes(range, summary);

            }
        }
        
    }

    fn handle_add_and_remove(change: &Change, list_object: &mut Vec<String>) {
        if let Change::Add{ index, len } = change {

            for i in 0..*len {
                list_object.insert(*index + i, format!("New_{}", i));
            }
        } else if let Change::Remove { index, len } = change {
            for _ in 0..*len {
                list_object.remove(*index);
            }
        }
    }

    fn print_ranged_and_full_list_object(message: &str, list: &[String], range: Range<usize>) {
        let mut combined_full = String::new();
        let mut combined_range = String::new();
        list.iter().enumerate().for_each(|(ix, s)| {
            combined_full.push(' ');
            combined_range.push(' ');

            combined_full.push_str(s.as_str());
            match range.contains(&ix) {
                true => combined_range.push_str(s.as_str()),
                false => for _ in 0..s.len() {
                    combined_range.push(' ');
                }
            }

            combined_full.push(' ');
            combined_range.push(' ');
        });
        // println!("\n {message} range list and object list for range {:?} are: \n {combined_full} \n {combined_range} \n", range);
        println!("\n {message} for {:?}: \n {combined_full} \n {combined_range} \n", range);
    }

    fn print_ranged(message: &str, list: &[String], range: Range<usize>) {
        let mut combined = String::new();
        list.iter().enumerate().for_each(|(ix, s)| {
            combined.push(' ');

            match range.contains(&ix) {
                true => combined.push_str(s.as_str()),
                false => for _ in 0..s.len() {
                    combined.push(' ');
                }
            }
            combined.push(' ');
        });
        println!("\n {message} for {:?} is: \n {combined}", range);
    }
}