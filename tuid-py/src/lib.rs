use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use pyo3::{basic::CompareOp, prelude::*, PyObjectProtocol};
use tuid::bstr::ByteSlice;

#[pymodule]
fn tuid(_py: Python, m: &PyModule) -> PyResult<()> {
    #[allow(clippy::upper_case_acronyms)]
    #[pyclass(freelist = 1000)]
    #[derive(Clone)]
    struct TUID {
        id: tuid::Tuid,
    }

    #[pymethods]
    impl TUID {
        #[getter]
        fn bytes(&self) -> &[u8] {
            self.id.as_bytes()
        }

        #[getter]
        unsafe fn hex(&self) -> String {
            self.id.as_hex().to_str_unchecked().to_string()
        }
    }

    #[pyproto]
    impl<'p> PyObjectProtocol<'p> for TUID {
        fn __str__(&self) -> PyResult<String> {
            Ok(self
                .id
                .as_uuid()
                .to_hyphenated()
                .encode_lower(&mut tuid::uuid::Uuid::encode_buffer())
                .to_string())
        }

        fn __repr__(&self) -> PyResult<String> {
            let s = self.__str__()?;
            Ok(format!("UUID('{}')", s))
        }

        fn __richcmp__(&self, other: TUID, op: CompareOp) -> PyResult<bool> {
            match op {
                CompareOp::Eq => Ok(self.id == other.id),
                CompareOp::Ne => Ok(self.id != other.id),
                CompareOp::Lt => Ok(self.id < other.id),
                CompareOp::Gt => Ok(self.id > other.id),
                CompareOp::Le => Ok(self.id <= other.id),
                CompareOp::Ge => Ok(self.id >= other.id),
            }
        }

        fn __hash__(&self) -> PyResult<isize> {
            let mut s = DefaultHasher::new();
            self.id.hash(&mut s);
            let result = s.finish() as isize;

            Ok(result)
        }
    }

    #[pyfn(m, name = "once")]
    fn once() -> PyResult<TUID> {
        Ok(TUID { id: tuid::gen::once() })
    }

    m.add_class::<TUID>()?;
    Ok(())
}
