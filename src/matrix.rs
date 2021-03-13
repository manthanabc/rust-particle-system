#[derive(Debug)]
pub struct Matrix {
	pub arr : Vec<Vec<f32>>
}

impl Matrix {
	pub fn add(&self, other: &Matrix) -> Matrix {

		if self.arr.len() != other.arr.len() {
			panic!("incompatable matrises");
		}

		let mut result:Matrix = Matrix {
			arr: self.arr.clone()
		};

		for colomn in 0..self.arr.len() {
			let mut temp_colom=self.arr[colomn].clone();
			for element in 0..self.arr[colomn].len() {
				temp_colom[element] = other.arr[colomn][element] + self.arr[colomn][element];
			}
			result.arr[colomn] = temp_colom;
		}
		result
	}

	pub fn dot(&self, other: &Matrix) -> Matrix {

		if self.arr.len() != other.arr.len() {
			panic!("incompatable matrises");
		}

		let mut result:Matrix = Matrix {
			arr: self.arr.clone()
		};

		for colomn in 0..self.arr.len() {
			let mut temp_colom=self.arr[colomn].clone();
			for element in 0..self.arr[colomn].len() {
				temp_colom[element] = other.arr[colomn][element] * self.arr[colomn][element];
			}
			result.arr[colomn] = temp_colom;
		}
		result
	}

	// here we get a colomn and a row and returns the resulting number
	pub fn one_dimension_multiply<'a>(&self ,other_matrix:&'a Vec<Vec<f32>>, colomn_number: usize, colomn:&Vec<f32>) -> Result<f32, &'a str>
	{
		if other_matrix.len() != colomn.len() {
			return Err("incompatable Matrix")
		}

		let mut result: f32 = 0.0 ;

		for re in 0..colomn.len() {
			result += colomn[re] + other_matrix[re][colomn_number];
		}

		Ok(result)
	}

	pub fn mul(&self, other: &Matrix) -> Matrix {

		if self.arr[0].len() != other.arr.len() {
			panic!("incompatable matrises");
		}

		let mut nv: Vec<Vec<f32>> = self.arr.clone();

		for row in 0..self.arr.len() {
			nv[row] = other.arr[0].clone();
		}

		for colomn in 0..self.arr.len() {
			for el in 0..other.arr[colomn].len() {
				nv[colomn][el] = self.one_dimension_multiply(&other.arr, el, &self.arr[colomn]).unwrap();
			}
		}

		Matrix {
			arr : nv
		}
	}

	pub fn get_dir(&self) -> Matrix {
		let x = self.arr[0][0];
		let y = self.arr[1][0];
		let scale_factor = (1.0 / ((x*x + y*y)as f64).sqrt()) as f32;
		Matrix {
			arr: vec![vec![x * scale_factor], vec![y * scale_factor]],
		}
	}
}