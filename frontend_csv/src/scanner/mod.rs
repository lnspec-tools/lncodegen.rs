pub mod scanner;
/// Scanner implementation!
pub mod token;

#[cfg(test)]
mod test {
    use crate::scanner::scanner;
    use crate::scanner::token;
    use std::fs;

    // Test if scanner read each symbols correctly to its type.
    #[test]
    fn scan_simple_one() {
        let path_file = std::env::var_os("CSV_PATH").unwrap();
        let contents = fs::read_to_string(format!("{}/bolt1.csv", path_file.to_str().unwrap()))
            .expect("Something went wrong reading the file");
        let char_vec: Vec<char> = contents.chars().collect();
        let mut scanner = scanner::Scanner::new();
        let result = scanner.scan(&char_vec);
        assert!(result.len() > 0);
        for c in result {
            match c.ty {
                token::CSVTokenType::MsgTy => assert_eq!(c.val, "msgtype"),
                token::CSVTokenType::MsgData => assert_eq!(c.val, "msgdata"),
                token::CSVTokenType::TlvType => assert_eq!(c.val, "tlvtype"),
                token::CSVTokenType::TlvData => assert_eq!(c.val, "tlvdata"),
                token::CSVTokenType::ShortChannelId => assert_eq!(c.val, "short_channel_id"),
                token::CSVTokenType::Sha256 => assert_eq!(c.val, "sha256"),
                token::CSVTokenType::U16 => assert_eq!(c.val, "u16"),
                token::CSVTokenType::U32 => assert_eq!(c.val, "u32"),
                token::CSVTokenType::U64 => assert_eq!(c.val, "u64"),
                token::CSVTokenType::ChannelId => assert_eq!(c.val, "channel_id"),
                token::CSVTokenType::Signature => assert_eq!(c.val, "signature"),
                token::CSVTokenType::Point => assert_eq!(c.val, "point"),
                token::CSVTokenType::ChainHash => assert_eq!(c.val, "chain_hash"),
                token::CSVTokenType::Byte => assert_eq!(c.val, "byte"),
                token::CSVTokenType::BigSize => assert_eq!(c.val, "bigsize"),
                token::CSVTokenType::LiteralString => continue,
                token::CSVTokenType::Number => continue,
                token::CSVTokenType::Tu32 => assert_eq!(c.val, "tu32"),
                token::CSVTokenType::Tu64 => assert_eq!(c.val, "tu64"),
                token::CSVTokenType::Tlvs => assert_eq!(c.val, "tlvs"),
                token::CSVTokenType::Dotdotdot => assert_eq!(c.val, "..."),
                token::CSVTokenType::Data => assert_eq!(c.val, "data"),
                token::CSVTokenType::SubTy => assert_eq!(c.val, "subtype"),
                token::CSVTokenType::SubMsgData => assert_eq!(c.val, "subtypedata"),
                token::CSVTokenType::EOF => continue,
            }
        }
    }

    // Test if scanner read one line correctly
    #[test]
    fn test_one_line() {
        let contents = "msgtype,init,16\n";
        let char_vec: Vec<char> = contents.chars().collect();
        let mut scanner = scanner::Scanner::new();
        let result = scanner.scan(&char_vec);
        assert!(result.len() > 0);
        let expected = vec![
            token::CSVToken {
                ty: token::CSVTokenType::MsgTy,
                val: "msgtype".to_string(),
            },
            token::CSVToken {
                ty: token::CSVTokenType::LiteralString,
                val: "init".to_string(),
            },
            token::CSVToken {
                ty: token::CSVTokenType::Number,
                val: "16".to_string(),
            },
        ];
        for c in 0..expected.len() - 1 {
            assert!(result[c].val == expected[c].val);
            debug_assert_eq!(result[c].ty, expected[c].ty);
        }
    }

    // Test if scanner read middle line correctly
    #[test]
    fn test_middle_line() {
        let contents = "msgtype,init,16\nmsgdata,init,gflen,u16,\n
        msgdata,init,globalfeatures,byte,gflen\n";
        let char_vec: Vec<char> = contents.chars().collect();
        let mut scanner = scanner::Scanner::new();
        let result = scanner.scan(&char_vec);
        assert!(result.len() > 0);
        let expected = vec![
            token::CSVToken {
                ty: token::CSVTokenType::MsgData,
                val: "msgdata".to_string(),
            },
            token::CSVToken {
                ty: token::CSVTokenType::LiteralString,
                val: "init".to_string(),
            },
            token::CSVToken {
                ty: token::CSVTokenType::LiteralString,
                val: "gflen".to_string(),
            },
            token::CSVToken {
                ty: token::CSVTokenType::U16,
                val: "u16".to_string(),
            },
        ];
        for c in 0..expected.len() - 1 {
            debug_assert_eq!(result[c + 3].val, expected[c].val);
            debug_assert_eq!(result[c + 3].ty, expected[c].ty);
        }
    }

    // Test if scanner read last line correctly
    #[test]
    fn test_last_line() {
        let contents = "msgtype,init,16\nmsgdata,init,gflen,u16,\n
        msgdata,init,globalfeatures,byte,gflen\n";
        let char_vec: Vec<char> = contents.chars().collect();
        let mut scanner = scanner::Scanner::new();
        let mut result = scanner.scan(&char_vec);
        result.reverse();
        assert!(result.len() > 0);
        let expected = vec![
            token::CSVToken {
                ty: token::CSVTokenType::EOF,
                val: "EOF".to_string(),
            },
            token::CSVToken {
                ty: token::CSVTokenType::LiteralString,
                val: "gflen".to_string(),
            },
            token::CSVToken {
                ty: token::CSVTokenType::Byte,
                val: "byte".to_string(),
            },
            token::CSVToken {
                ty: token::CSVTokenType::LiteralString,
                val: "globalfeatures".to_string(),
            },
            token::CSVToken {
                ty: token::CSVTokenType::LiteralString,
                val: "init".to_string(),
            },
            token::CSVToken {
                ty: token::CSVTokenType::MsgData,
                val: "msgdata".to_string(),
            },
        ];
        for c in 0..expected.len() - 1 {
            assert_eq!(result[c].val, expected[c].val);
            assert_eq!(result[c].ty, expected[c].ty);
        }
    }

    // Test for double commas in seqence between 2 tokens
    #[test]
    #[should_panic(expected = "Empty space between two separator `,` are not allowed")]
    fn test_empty_middle() {
        let contents = "msgtype,  ,16\n";
        let char_vec: Vec<char> = contents.chars().collect();
        let mut scanner = scanner::Scanner::new();
        scanner.scan(&char_vec);
    }

    // Test for double commas in seqence in front of first token
    #[test]
    #[should_panic(expected = "Empty space between two separator `,` are not allowed")]
    fn test_empty_front() {
        let contents = ",,16\n";
        let char_vec: Vec<char> = contents.chars().collect();
        let mut scanner = scanner::Scanner::new();
        scanner.scan(&char_vec);
    }

    // Test if the content has a end of line character at the end
    // which include \n and commas
    #[test]
    #[should_panic(expected = "Need EOF symbol")]
    fn test_empty_eof() {
        let contents = "msgtype,init,16\nmsgdata,init,gflen,u16,\n
        msgdata,init,globalfeatures,byte,gflen";
        let char_vec: Vec<char> = contents.chars().collect();
        let mut scanner = scanner::Scanner::new();
        scanner.scan(&char_vec);
    }

    #[test]
    fn test_subtype_parsing() {
        let contents = "subtype,init, \
                        subtypedata,init,gflen,u16\n";
        let char_vec: Vec<char> = contents.chars().collect();
        let mut scanner = scanner::Scanner::new();
        let tokens = scanner.scan(&char_vec);
        assert_eq!(tokens.len(), 7);
    }
}
