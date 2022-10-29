use std::string::ToString;

use jerris::access_flags::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jerris::attribute::Attribute;
use jerris::class::{Class, JavaVersion};
use jerris::constant_pool::Constant;
use jerris::field::Field;
use jerris::method::Method;

#[test]
fn parse_class() {
    let class = Class {
        java_version: JavaVersion {
            minor: 0,
            major: 63,
        },
        constant_pool: vec![
            Constant::Method {
                class_index: 1,
                name_and_type_index: 2,
            },
            Constant::Class {
                name_index: 3,
            },
            Constant::NameAndType {
                name_index: 4,
                descriptor_index: 5,
            },
            Constant::UTF8String(
                "java/lang/Object".to_string(),
            ),
            Constant::UTF8String(
                "<init>".to_string(),
            ),
            Constant::UTF8String(
                "()V".to_string(),
            ),
            Constant::Field {
                class_index: 7,
                name_and_type_index: 8,
            },
            Constant::Class {
                name_index: 9,
            },
            Constant::NameAndType {
                name_index: 10,
                descriptor_index: 11,
            },
            Constant::UTF8String(
                "Main".to_string(),
            ),
            Constant::UTF8String(
                "a".to_string(),
            ),
            Constant::UTF8String(
                "I".to_string(),
            ),
            Constant::Field {
                class_index: 13,
                name_and_type_index: 14,
            },
            Constant::Class {
                name_index: 15,
            },
            Constant::NameAndType {
                name_index: 16,
                descriptor_index: 17,
            },
            Constant::UTF8String(
                "java/lang/System".to_string(),
            ),
            Constant::UTF8String(
                "out".to_string(),
            ),
            Constant::UTF8String(
                "Ljava/io/PrintStream;".to_string(),
            ),
            Constant::String {
                string_index: 19,
            },
            Constant::UTF8String(
                "Hello World!".to_string(),
            ),
            Constant::Method {
                class_index: 21,
                name_and_type_index: 22,
            },
            Constant::Class {
                name_index: 23,
            },
            Constant::NameAndType {
                name_index: 24,
                descriptor_index: 25,
            },
            Constant::UTF8String(
                "java/io/PrintStream".to_string(),
            ),
            Constant::UTF8String(
                "println".to_string(),
            ),
            Constant::UTF8String(
                "(Ljava/lang/String;)V".to_string(),
            ),
            Constant::UTF8String(
                "Code".to_string(),
            ),
            Constant::UTF8String(
                "LineNumberTable".to_string(),
            ),
            Constant::UTF8String(
                "main".to_string(),
            ),
            Constant::UTF8String(
                "([Ljava/lang/String;)V".to_string(),
            ),
            Constant::UTF8String(
                "SourceFile".to_string(),
            ),
            Constant::UTF8String(
                "Main.java".to_string(),
            ),
        ],
        access_flags: ClassAccessFlags::ACC_PUBLIC | ClassAccessFlags::ACC_SUPER,
        this_class: 8,
        super_class: 2,
        interfaces: vec![],
        fields: vec![
            Field {
                access_flags: FieldAccessFlags::ACC_PUBLIC,
                name_index: 11,
                descriptor_index: 12,
                attributes: vec![],
            },
        ],
        methods: vec![
            Method {
                access_flags: MethodAccessFlags::ACC_PUBLIC,
                name_index: 5,
                descriptor_index: 6,
                attributes: vec![
                    Attribute {
                        attribute_name_index: 27,
                        info: vec![
                            0,
                            2,
                            0,
                            1,
                            0,
                            0,
                            0,
                            10,
                            42,
                            183,
                            0,
                            1,
                            42,
                            4,
                            181,
                            0,
                            7,
                            177,
                            0,
                            0,
                            0,
                            1,
                            0,
                            28,
                            0,
                            0,
                            0,
                            10,
                            0,
                            2,
                            0,
                            0,
                            0,
                            1,
                            0,
                            4,
                            0,
                            2,
                        ],
                    },
                ],
            },
            Method {
                access_flags: MethodAccessFlags::ACC_PUBLIC | MethodAccessFlags::ACC_STATIC,
                name_index: 29,
                descriptor_index: 30,
                attributes: vec![
                    Attribute {
                        attribute_name_index: 27,
                        info: vec![
                            0,
                            2,
                            0,
                            1,
                            0,
                            0,
                            0,
                            9,
                            178,
                            0,
                            13,
                            18,
                            19,
                            182,
                            0,
                            21,
                            177,
                            0,
                            0,
                            0,
                            1,
                            0,
                            28,
                            0,
                            0,
                            0,
                            10,
                            0,
                            2,
                            0,
                            0,
                            0,
                            4,
                            0,
                            8,
                            0,
                            5,
                        ],
                    },
                ],
            },
        ],
        attributes: vec![
            Attribute {
                attribute_name_index: 31,
                info: vec![
                    0,
                    32,
                ],
            },
        ],
    };
    match Class::from_file("tests/Main.class") {
        Ok(actual_class) => {
            assert_eq!(class, actual_class);
        }
        Err(_e) => panic!("{_e}")
    };
}