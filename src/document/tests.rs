use UnQLite;
use document::{Map, Value};
use document::Jx9;
use std::thread;

// For view stdout of tests run:
// cargo test document -- --nocapture

#[test]
fn output_std() {
    // let db = UnQLite::create_in_memory();
    let db = UnQLite::create("data.base");
    let prog = "print '-- hello '; print 'world --\n';";
    db.compile(prog)
        .and_then(|mut vm| {
            vm.output_to_stdout()?;
            vm.report_errors_to_output()?;
            vm.exec_void()?;
            vm.output_length().map(|len| assert_eq!(18, len))
        })
        .unwrap()
}

#[test]
#[ignore]
fn dump_vm_instructions() {
    let db = UnQLite::create_in_memory();
    let prog = "print 'hello '; print 'world'..'!';";
    db.compile(prog).and_then(|vm| vm.dump()).unwrap();
}

#[test]
fn output_channel() {
    let db = UnQLite::create_in_memory();
    let prog = "print 'hello';\
                print 'world';";

    db.compile(prog)
        .and_then(|mut vm| {
            vm.report_errors_to_output()?;
            let receiver = vm.output_to_channel()?;

            let handle = thread::spawn(move || {
                let output: Vec<Vec<u8>> = receiver.iter().collect();
                assert_eq!(2, output.len());
                assert_eq!(b"hello", output[0].as_slice());
                assert_eq!(b"world", output[1].as_slice());
                assert!(receiver.recv().is_err());
            });

            vm.exec_void()?;
            vm.output_length().map(|len| assert_eq!(10, len))?;

            // reset output channel
            let new_receiver = vm.output_to_channel()?;
            handle.join().unwrap();

            vm.exec_void()?;
            assert_eq!(20, vm.output_length()?);

            let output: Vec<Vec<u8>> = new_receiver.iter().take(2).collect();
            assert_eq!(2, output.len());
            assert_eq!(b"hello", output[0].as_slice());
            assert_eq!(b"world", output[1].as_slice());
            Ok(())
        })
        .unwrap()
}

#[test]
fn output_extract() {
    let db = UnQLite::create_in_memory();
    let prog = "print 'hello '; print 'world';";
    db.compile(prog)
        .and_then(|mut vm| {
            vm.exec().and_then(|_| {
                vm.extract_output()
                    .map(|msg| assert_eq!("hello world", String::from_utf8_lossy(msg)))
            })
        })
        .unwrap();
}

#[test]
fn arguments() {
    let db = UnQLite::create_in_memory();
    let prog = "print $argv[0]..' '..$argv[1];";
    db.compile(prog)
        .and_then(|mut vm| {
            vm.add_argument("hello")?;
            vm.add_argument("world")?;
            let output = vm.exec().and_then(|_| vm.extract_output())?;
            assert_eq!(b"hello world", output);
            Ok(())
        })
        .unwrap()
}

#[test]
fn env_attributes() {
    let db = UnQLite::create_in_memory();
    let prog = "print $_ENV['_']..' '..$_ENV['name']..$_ENV['NAME'];";
    db.compile(prog)
        .and_then(|mut vm| {
            vm.add_env_attr("_", "Hello")?;
            vm.add_env_attr("name", "world")?;
            vm.add_env_attr("NAME", "!")?;
            let output = vm.exec().and_then(|_| vm.extract_output())?;
            let output = String::from_utf8_lossy(output);
            // println!("{:?}", output);
            assert_eq!("Hello world!", output);
            Ok(())
        })
        .unwrap()
}

#[test]
fn exec_result() {
    let db = UnQLite::create_in_memory();

    db.compile("print 'will return null';")
        .map(|mut vm| {
            assert_eq!(Some(Value::Null), vm.exec().unwrap());
        })
        .unwrap();

    db.compile("return -42;")
        .map(|mut vm| {
            assert_eq!(Some(Value::Int(-42)), vm.exec().unwrap());
        })
        .unwrap();

    db.compile("return 12.345;")
        .map(|mut vm| {
            assert_eq!(Some(Value::Real(12.345)), vm.exec().unwrap());
        })
        .unwrap();

    db.compile("return 'hello';")
        .map(|mut vm| {
            assert_eq!(Some(Value::String("hello".to_string())), vm.exec().unwrap());
        })
        .unwrap();

    db.compile("return [1,2,'test',[3,4.5,null]];")
        .map(|mut vm| {
            let test = Value::Array(vec![
                Value::Int(1),
                Value::Int(2),
                Value::string("test"),
                Value::Array(vec![Value::Int(3), Value::Real(4.5), Value::Null]),
            ]);
            assert_eq!(Some(test), vm.exec().unwrap());
        })
        .unwrap();

    db.compile(
        "return {'key_one': 123, 'key_2': 'string_value', 'key_3': [1,2,3],
               'key_4': {'sub_key': 'sub_value', 'sub_key2': 42}, 'key_5': 1.2};",
    ).map(|mut vm| {
            vm.exec()
                .map(|opt| opt.unwrap())
                .map(|result| {
                    let map: Option<Map> = result.into();
                    assert!(map.is_some());
                    if let Some(map) = map {
                        assert_eq!(5, map.len());
                        assert_eq!(Value::Int(123), map["key_one"]);
                        let s: Option<String> = map["key_2"].clone().into();
                        assert_eq!("string_value", s.unwrap());
                        assert_eq!(
                            Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]),
                            map["key_3"]
                        );
                        if let &Value::Object(ref sub_map) = &map["key_4"] {
                            assert_eq!(Value::Int(42), sub_map["sub_key2"]);
                        } else {
                            assert!(false);
                        }
                    }
                })
                .unwrap()
        })
        .unwrap();
}

#[test]
fn add_extract_variable() {
    let db = UnQLite::create_in_memory();
    let prog = "$val1 = $in1;\
                $val2 = $in2..'_mod';\
                $val3 = {key1: $in3, key2: $in4};
                $val4 = $map1 + $map2;\
                $val5 = $map2 + $map1;\
                $arr2 = $arr1 + 'new_elem';\
                if ($val6 >= 0) { return 'pos'; } \
                else { return 'neg'; }";
    db.compile(prog)
        .and_then(|mut vm| {
            vm.add_variable("in1", Value::Int(10))?;
            vm.add_variable("in2", Value::string("test"))?;
            vm.add_variable("in3", Value::string("value"))?;
            vm.add_variable("in4", Value::Real(12.34))?;
            let mut map1 = Map::new();
            map1.insert("name".to_string(), Value::string("one"));
            let mut map2 = Map::new();
            map2.insert("name".to_string(), Value::string("two"));
            map2.insert("key".to_string(), Value::Int(12));
            vm.add_variable("map1", Value::Object(map1))?;
            vm.add_variable("map2", Value::Object(map2))?;
            vm.add_variable("arr1", Value::Array(vec![Value::Null, Value::Int(100)]))?;
            vm.add_variable("val6", Value::Int(10))?;
            vm.exec().and_then(|result| {
                assert_eq!(Some(Value::string("pos")), result);
                assert_eq!(Some(Value::Int(10)), vm.extract_variable("val1"));
                assert_eq!(Some(Value::string("test_mod")), vm.extract_variable("val2"));

                let val3: Option<Map> = vm.extract_variable("val3").unwrap().into();
                let val3 = val3.unwrap();
                assert_eq!(2, val3.len());
                assert_eq!(Value::string("value"), val3["key1"]);
                assert_eq!(Value::Real(12.34), val3["key2"]);

                let val4: Option<Map> = vm.extract_variable("val4").unwrap().into();
                let val4 = val4.unwrap();
                assert_eq!(2, val4.len());
                assert_eq!(Value::string("one"), val4["name"]);
                assert_eq!(Value::Int(12), val4["key"]);

                let val5: Option<Map> = vm.extract_variable("val5").unwrap().into();
                let val5 = val5.unwrap();
                assert_eq!(2, val5.len());
                assert_eq!(Value::string("two"), val5["name"]);
                assert_eq!(Value::Int(12), val5["key"]);

                let arr2: Option<Vec<Value>> = vm.extract_variable("arr2").unwrap().into();
                let arr2 = arr2.unwrap();
                assert_eq!(3, arr2.len());
                assert_eq!(Value::string("new_elem"), arr2[2]);

                Ok(())
            })
        })
        .unwrap()
}
