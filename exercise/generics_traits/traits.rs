fn main() {

    println!("hey");
    println!("{}", vec![Some(1.2), Some(2.3), Some(3.4), None].to_json());

    let fmi_student = Student{
        age: 30,
        full_name: String::from("Gosho Peshev"),
        number: 12345,
        hobby: Some(String::from("algebra")),
    };

    println!("{}", fmi_student.to_json());

    println!("{}", Celsius::from(Fahrenheit(95.5)).0)
    
}

impl From<Fahrenheit> for Celsius {
    fn from(t: Fahrenheit) -> Celsius {
        Celsius((t.0 - 32.0) / 1.8)
    }
}

struct Kelvin(f64);
struct Celsius(f64);
struct Fahrenheit(f64);

fn room_temp() -> Fahrenheit {

    Fahrenheit(68.0)
}

trait ToJson {
    
    fn to_json(&self) -> String;
}

impl ToJson for i32 {

    fn to_json(&self) -> String {
        format!("{}", self)
    }
}

impl ToJson for f32 {

    fn to_json(&self) -> String {
        format!("{}", self)
    }
}

impl ToJson for String {

    fn to_json(&self) -> String {

        format!("\"{}\"", self)
    }    
}

impl<T> ToJson for Option<T> where T: ToJson {

    fn to_json(&self) -> String {

        match self {
            Some(val) => val.to_json(),
            None      => String::from("null"),
        }
    }
}

impl<T> ToJson for Vec<T> where T: ToJson {

    fn to_json(&self) -> String {

        let mut it = self.iter();
        let first = it.next();

        let mut result = match first {
            Some(first) => first.to_json(),
            None => String::new(),
        };

        for num in it {
            result.push_str(", ");
            result.push_str(&num.to_json());
        }

        format!("[{}]", result)
    }
}

struct Student {

    age: i32,
    full_name: String,
    number: i32,
    hobby: Option<String>,
}

impl ToJson for Student {

    fn to_json(&self) -> String {

        format!(

r#"{{
    "age": {},
    "full_name": {},
    "number": {},
    "hobby": {}
}}"#,

            self.age.to_json(), self.full_name.to_json(), self.number.to_json(), self.hobby.to_json()
        )
    }
}