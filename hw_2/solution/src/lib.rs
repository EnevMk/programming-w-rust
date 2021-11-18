use std::ops::Add;
use std::ops::Mul;

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

    pub fn by_row(&self) -> Vec<Cell<T>> {
        //todo!()
        vec![self.cell_a.clone(), self.cell_b.clone(), self.cell_c.clone(), self.cell_d.clone()]
    }

    pub fn by_col(&self) -> Vec<Cell<T>> {
        //todo!()
        vec![self.cell_a.clone(), self.cell_c.clone(), self.cell_b.clone(), self.cell_d.clone()]
    }
}

impl Add<Matrix<String>> for Matrix<i32> {

    type Output = Matrix<String>;

    fn add(self, rhs : Matrix<String>) -> Self::Output {

        let result_values = Matrix::<String>
        {
            cell_a: self.cell_a + rhs.cell_a,
            cell_b: self.cell_b + rhs.cell_b,
            cell_c: self.cell_c + rhs.cell_c,
            cell_d: self.cell_d + rhs.cell_d,
        };

        result_values
    }
}

fn concat_vector_elements(v: Vec<String>) -> String {

    let mut res;

    let mut iter = v.iter();
    let first = iter.next();

    match first {
        Some(s) => res = String::from(s),
        None    => return String::new(),
    }

    let size = v.len();
    for i in 1..size {
        res.push(' ');
        res.push_str(&v[i]);
    }

    res
}

impl Mul<Matrix<String>> for Matrix<i32> {

    type Output = String;

    fn mul(self, rhs: Matrix<String>) -> Self::Output {

        let v = self.by_row();
        let v2 = rhs.by_col();

        let mut res = Vec::new();
        let elements_count = 4;

        for i in 0..elements_count {
            res.push((v[i].clone() * v2[i].clone()).0)
        }

        concat_vector_elements(res) 
    }
}

impl Mul<Cell<String>> for Cell<i32> {
    type Output = Cell<String>;

    fn mul(self, rhs : Cell<String>) -> Cell<String> {

        let mut res = String::new();

        if self.0 >= 0 {

            for _ in 0..self.0 {
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

            for _ in 0..count {

                res.push_str(&reversed);
            }
        }
        Cell(res)
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
            
            while let Some(ch) = rhs_reversed.next() {

                res.push(ch);
            }

            res.push(' ');
            res.push_str(&(self.0 - 2 * self.0).to_string());

        }
        Cell(res)
    }   
}

#[cfg(test)]
mod tests {

    use super::*;

    fn string_cell_vec(s1: &str, s2: &str, s3: &str, s4: &str) -> Vec<Cell<String>> {
        [s1, s2, s3, s4].into_iter().map(String::from).map(Cell).collect::<Vec<Cell<String>>>()
    }
    
    #[test]
    fn test_cell_add() {
        assert_eq!((Cell(4) + Cell(String::from("badger"))).0, String::from("4 badger"));
        assert_eq!((Cell(0) + Cell(String::from("Milano"))).0, String::from("0 Milano"));
    
        assert_eq!((Cell(-1899) + Cell(String::from("ecnerolF"))).0, String::from("Florence 1899"));
        assert_eq!((Cell(0) + Cell(String::new())).0, String::from("0 "));
    
    }
    
    #[test]
    fn test_cell_mul() {
    
        assert_eq!((Cell(0) * Cell(String::from("Bari"))).0, String::new());
        assert_eq!((Cell(-3) * Cell(String::from("ecceL"))).0, String::from("LecceLecceLecce"));
        assert_eq!((Cell(3) * Cell(String::from("Venezia"))).0, String::from("VeneziaVeneziaVenezia"));
        assert_eq!((Cell(-1) * Cell(String::from("kinreP"))).0, String::from("Pernik"));
        assert_eq!((Cell(-1) * Cell(String::new())).0, String::new());
    }
    
    #[test]
    fn test_matrix_add() {
    
        let matrix1 = Matrix::new(&[7, -2, 0, 4]);
        let matrix2 = Matrix::new(&[
            String::from("one"), String::from("owt"),
            String::from("three"), String::from("four")
        ]);
    
        assert_eq!(
            (matrix1 + matrix2).by_row(),
            string_cell_vec("7 one", "two 2", "0 three", "4 four")
        );
    }
    
    #[test]
    fn test_matrix_mul() {
        let matrix1 = Matrix::new(&[7, -2, 0, 4]);
        let matrix2 = Matrix::new(&[
            String::from("one"), String::from("two"),
            String::from("eerht"), String::from("four")
        ]);
    
        assert_eq!(
            (matrix1 * matrix2),
            String::from("oneoneoneoneoneoneone threethree  fourfourfourfour")
        );
    }
}