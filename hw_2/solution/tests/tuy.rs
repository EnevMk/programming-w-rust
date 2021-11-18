use std::ops::Add;
use std::ops::Mul;

fn main() {
    let x : Cell<String> = Cell(String::from("Milano"));
    let y = x.0;
    //println!("{:?}", x);

    println!("{}", (Cell(0) + Cell(String::from("Milano"))) == Cell(String::from("0 Milano")));
    println!("{}", y);
    println!("{}", ((Cell(0) * Cell(String::from("Bari"))).0 == String::new()));
    println!("{}", ((Cell(-3) * Cell(String::from("ecceL"))).0 == String::from("LecceLecceLecce")));

    let z = 1.5;
    let p = z.clone();

    print!("{}, {}", z, p);

}

#[derive(Debug)]
pub struct Matrix<T: Clone> {
    // Каквито данни ви вършат работа
    cell_a : Cell<T>,
    cell_b : Cell<T>,
    cell_c : Cell<T>,
    cell_d : Cell<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cell<T>(pub T);

impl<T: Clone> Matrix<T> {
    /// Данните се очаква да бъдат подадени със статичен масив -- вижте по-долу за примери за
    /// конструиране. Какви може да са елементите? Ще тестваме само с два типа: String и i32.
    ///
    /// Очаква се да бъдат подадени по редове, от ляво надясно. Тоест, ако подадем като вход списък
    /// с елементи: 1, 2, 3, 4, се очаква конструираната матрица:
    ///
    /// | 1 2 |
    /// | 3 4 |
    ///
    /// Забележете, че подаваме като вход някакъв slice -- reference тип. Не очакваме матрицата да
    /// държи reference, клонирайте си данните, за да имате ownership.
    ///
    pub fn new(data: &[T; 4]) -> Matrix<T> {
        //todo!()
        //let a = data[0];
        Self {
            cell_a : Cell(data[0].clone()),
            cell_b : Cell(data[1].clone()),
            cell_c : Cell(data[2].clone()),
            cell_d : Cell(data[3].clone()),
        }
    }

    /// Връща вектор, който съдържа в себе си всички 4 елемента на матрицата, наредени по редове,
    /// от ляво надясно и от горе надолу, обвити в `Cell`. Тоест, ако матрицата изглежда така:
    ///
    /// | 1 2 |
    /// | 3 4 |
    ///
    /// Очакваме `.by_row` да върне елементите в ред: 1, 2, 3, 4
    ///
    pub fn by_row(&self) -> Vec<Cell<T>> {
        //todo!()
        vec![self.cell_a.clone(), self.cell_b.clone(), self.cell_c.clone(), self.cell_d.clone()]
    }

    /// Връща вектор, който съдържа в себе си всички 4 елемента на матрицата, наредени по колони,
    /// от горе надолу и от ляво надясно, Обвити в `Cell`. Тоест, ако матрицата изглежда така:
    ///
    /// | 1 2 |
    /// | 3 4 |
    ///
    /// Очакваме `.by_col` да върне елементите в ред: 1, 3, 2, 4
    ///
    pub fn by_col(&self) -> Vec<Cell<T>> {
        //todo!()
        vec![self.cell_a.clone(), self.cell_c.clone(), self.cell_b.clone(), self.cell_d.clone()]
    }
}

impl Add<Cell<String>> for Cell<i32> {
    type Output = Cell<String>;

    fn add(self, rhs : Cell<String>) -> Cell<String> {

        let mut res = String::new();

        if self.0 >= 0 {
            
            res.push_str(&self.0.to_string());
            res.push(' ');
            res.push_str(&rhs.0);
        }

        else {

            let mut rhs_reversed = rhs.0.chars().rev();
            //rhs_reversed.rev();

            while let Some(ch) = rhs_reversed.next() {

                res.push(ch);
            }

            res.push(' ');
            res.push_str(&(self.0 - 2 * self.0).to_string());

        }
        //Cell(String::from("Misho"))
        Cell(res)
    }   
}

impl Mul<Cell<String>> for Cell<i32> {
    type Output = Cell<String>;

    fn mul(self, rhs : Cell<String>) -> Cell<String> {

        let mut res = String::new();

        if self.0 >= 0 {

            for _ in [0..self.0] {
                res.push_str(&rhs.0);
            }
        }

        else {

            let mut reversed = String::new();
            let mut rev_iter = rhs.0.chars().rev();

            while let Some(ch) = rev_iter.next() {

                reversed.push(ch)
            }

            let count = self.0 - 2 * self.0;

            for _ in [0..count] {
                res.push_str(&reversed);
            }
        }
        Cell(res)
        //Cell(String::from("Genoa"))
    }
}