pub use nalgebra::{Const, DMatrix, Dyn, Matrix, ViewStorage};
pub use nalgebra::{DVector, ViewStorageMut};

use super::AtlasError;
pub type AtlasMap<K, V> = std::collections::HashMap<K, V>;

pub type MutableColumnSlice<'a> =
    Matrix<f64, Dyn, Const<1>, ViewStorageMut<'a, f64, Dyn, Const<1>, Const<1>, Dyn>>;
pub type ColumnSlice<'a> =
    Matrix<f64, Dyn, Const<1>, ViewStorage<'a, f64, Dyn, Const<1>, Const<1>, Dyn>>;
pub type ColumnBlock<'a> = Matrix<f64, Dyn, Dyn, ViewStorage<'a, f64, Dyn, Dyn, Const<1>, Dyn>>;
pub type MatrixXd = DMatrix<f64>;

pub type AtlasResult<T> = Result<T, AtlasError>;
