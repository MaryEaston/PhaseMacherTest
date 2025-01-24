pub fn conv_csv(input: String, n: usize) -> Vec<(String, String)> {
    let mut data_map: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    log::debug!("CSVを変換中...");
    for line in input.lines().skip(1) {
        let parts: Vec<&str> = line.split(',').collect();
        let key = parts[..n].join("-");
        let freq = parts[n].replace(" GHz", "");
        let value = parts[n + 1];
        let value_str = format!("{},{}", freq, value);

        data_map.entry(key).or_insert_with(Vec::new).push(value_str);
    }
    log::debug!("CSVの変換完了");

    data_map
        .into_iter()
        .map(|(key, values)| {
            let value_str = values.join("\n");
            (key, value_str)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use libpicker::data_digital::DataDigital;
    use rust_decimal_macros::dec;

    #[test]
    fn test_conv_csv_1() {
        let input_csv = "\
hoge,Freq,Value
A,23 GHz,1
A,24 GHz,2
A,25 GHz,3
A,26 GHz,4
B,23 GHz,10
B,24 GHz,20
B,25 GHz,30
B,26 GHz,40
C,23 GHz,11
C,24 GHz,21
C,25 GHz,31
C,26 GHz,41"
            .to_string();

        let mut expected_output_csv = vec![
            (
                "A".to_string(),
                "\
23,1
24,2
25,3
26,4"
                    .to_string(),
            ),
            (
                "B".to_string(),
                "\
23,10
24,20
25,30
26,40"
                    .to_string(),
            ),
            (
                "C".to_string(),
                "\
23,11
24,21
25,31
26,41"
                    .to_string(),
            ),
        ];

        let mut output_csv = conv_csv(input_csv.to_string(), 1);
        // ソートして順番を無視して比較
        expected_output_csv.sort();
        output_csv.sort();
        assert_eq!(output_csv, expected_output_csv);
    }

    #[test]
    fn test_conv_csv_2() {
        let input_csv = "\
foo,baz,bar,Freq,Value
A,1,3,23 GHz,11
A,1,3,24 GHz,21
A,1,3,25 GHz,31
A,1,3,26 GHz,41
A,1,3,27 GHz,11
A,1,3,28 GHz,21
A,1,3,29 GHz,31
A,1,3,30 GHz,41
A,1,3,31 GHz,11
A,1,3,32 GHz,21
A,1,3,33 GHz,31
A,1,3,34 GHz,41
B,5,4,23 GHz,16489
B,5,4,24 GHz,26489
B,5,4,25 GHz,36489
B,5,4,26 GHz,46489
B,5,4,27 GHz,16489
B,5,4,28 GHz,26489
B,5,4,29 GHz,36489
B,5,4,30 GHz,46489
B,5,4,31 GHz,16489
B,5,4,32 GHz,26489
B,5,4,33 GHz,36489
B,5,4,34 GHz,46489
C,7,2,23 GHz,1848
C,7,2,24 GHz,2848
C,7,2,25 GHz,3848
C,7,2,26 GHz,4848
C,7,2,27 GHz,1848
C,7,2,28 GHz,2848
C,7,2,29 GHz,3848
C,7,2,30 GHz,4848
C,7,2,31 GHz,1848
C,7,2,32 GHz,2848
C,7,2,33 GHz,3848
C,7,2,34 GHz,4848
";

        let mut expected_output_csv = vec![
            (
                "A-1-3".to_string(),
                "\
23,11
24,21
25,31
26,41
27,11
28,21
29,31
30,41
31,11
32,21
33,31
34,41"
                    .to_string(),
            ),
            (
                "B-5-4".to_string(),
                "\
23,16489
24,26489
25,36489
26,46489
27,16489
28,26489
29,36489
30,46489
31,16489
32,26489
33,36489
34,46489"
                    .to_string(),
            ),
            (
                "C-7-2".to_string(),
                "\
23,1848
24,2848
25,3848
26,4848
27,1848
28,2848
29,3848
30,4848
31,1848
32,2848
33,3848
34,4848"
                    .to_string(),
            ),
        ];

        let mut output_csv = conv_csv(input_csv.to_string(), 3);
        // ソートして順番を無視して比較
        expected_output_csv.sort();
        output_csv.sort();
        assert_eq!(output_csv, expected_output_csv);
    }

    #[test]
    fn test_csv_to_datas() {
        let input = "\
        23.0,1329.0
        24.0,2329.0
        25.0,3329.0
        26.0,4329.0";

        let expected: DataDigital = DataDigital::new(vec![
            (dec!(23.0), dec!(1329.0)),
            (dec!(24.0), dec!(2329.0)),
            (dec!(25.0), dec!(3329.0)),
            (dec!(26.0), dec!(4329.0)),
        ]);

        let output = DataDigital::build_from_csv(input).unwrap();
        assert_eq!(output, expected);
    }

    //     #[test]
    //     fn conv_result_csv_1() {
    //         let input = "\
    // 0,A-1-3,23,11,24,12,25,13,26,14,27,15,28,16,29,17,30,18,31,19,32,20,33,21,34,22
    // 1,B-5-4,23,101,24,102,25,103,26,104,27,105,28,106,29,107,30,108,31,109,32,110,33,111,34,112
    // 2,C-7-2,23,1001,24,1002,25,1003,26,1004,27,1005,28,1006,29,1007,30,1008,31,1009,32,1010,33,1011,34,1012";
    //         let expected = "\
    // 0,A-1-3,1,B-5-4,2,C-7-2
    // 23,11,23,101,23,1001
    // 24,12,24,102,24,1002
    // 25,13,25,103,25,1003
    // 26,14,26,104,26,1004
    // 27,15,27,105,27,1005
    // 28,16,28,106,28,1006
    // 29,17,29,107,29,1007
    // 30,18,30,108,30,1008
    // 31,19,31,109,31,1009
    // 32,20,32,110,32,1010
    // 33,21,33,111,33,1011
    // 34,22,34,112,34,1012";
    //         let output = conv_result_csv(input.to_string());
    //         assert_eq!(output, expected);
    //     }
}
