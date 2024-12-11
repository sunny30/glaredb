use std::fmt::Debug;
use std::marker::PhantomData;

use rayexec_bullet::array::{Array, ArrayData};
use rayexec_bullet::datatype::{DataType, DataTypeId};
use rayexec_bullet::executor::builder::{ArrayBuilder, PrimitiveBuffer};
use rayexec_bullet::executor::physical_type::{
    PhysicalF16,
    PhysicalF32,
    PhysicalF64,
    PhysicalI128,
    PhysicalI16,
    PhysicalI32,
    PhysicalI64,
    PhysicalI8,
    PhysicalStorage,
    PhysicalU128,
    PhysicalU16,
    PhysicalU32,
    PhysicalU64,
    PhysicalU8,
};
use rayexec_bullet::executor::scalar::BinaryExecutor;
use rayexec_bullet::storage::PrimitiveStorage;
use rayexec_error::Result;

use crate::expr::Expression;
use crate::functions::scalar::{PlannedScalarFunction, ScalarFunction, ScalarFunctionImpl};
use crate::functions::{invalid_input_types_error, plan_check_num_args, FunctionInfo, Signature};
use crate::logical::binder::table_list::TableList;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rem;

impl FunctionInfo for Rem {
    fn name(&self) -> &'static str {
        "%"
    }

    fn aliases(&self) -> &'static [&'static str] {
        &["rem", "mod"]
    }

    fn signatures(&self) -> &[Signature] {
        &[
            Signature {
                input: &[DataTypeId::Float16, DataTypeId::Float16],
                variadic: None,
                return_type: DataTypeId::Float16,
            },
            Signature {
                input: &[DataTypeId::Float32, DataTypeId::Float32],
                variadic: None,
                return_type: DataTypeId::Float32,
            },
            Signature {
                input: &[DataTypeId::Float64, DataTypeId::Float64],
                variadic: None,
                return_type: DataTypeId::Float64,
            },
            Signature {
                input: &[DataTypeId::Int8, DataTypeId::Int8],
                variadic: None,
                return_type: DataTypeId::Int8,
            },
            Signature {
                input: &[DataTypeId::Int16, DataTypeId::Int16],
                variadic: None,
                return_type: DataTypeId::Int16,
            },
            Signature {
                input: &[DataTypeId::Int32, DataTypeId::Int32],
                variadic: None,
                return_type: DataTypeId::Int32,
            },
            Signature {
                input: &[DataTypeId::Int64, DataTypeId::Int64],
                variadic: None,
                return_type: DataTypeId::Int64,
            },
            Signature {
                input: &[DataTypeId::Int128, DataTypeId::Int128],
                variadic: None,
                return_type: DataTypeId::Int128,
            },
            Signature {
                input: &[DataTypeId::UInt8, DataTypeId::UInt8],
                variadic: None,
                return_type: DataTypeId::UInt8,
            },
            Signature {
                input: &[DataTypeId::UInt16, DataTypeId::UInt16],
                variadic: None,
                return_type: DataTypeId::UInt16,
            },
            Signature {
                input: &[DataTypeId::UInt32, DataTypeId::UInt32],
                variadic: None,
                return_type: DataTypeId::UInt32,
            },
            Signature {
                input: &[DataTypeId::UInt64, DataTypeId::UInt64],
                variadic: None,
                return_type: DataTypeId::UInt64,
            },
            Signature {
                input: &[DataTypeId::UInt128, DataTypeId::UInt128],
                variadic: None,
                return_type: DataTypeId::UInt128,
            },
            // Signature {
            //     input: &[DataTypeId::Date32, DataTypeId::Int64],
            //     variadic: None,
            //     return_type: DataTypeId::Date32,
            // },
            // Signature {
            //     input: &[DataTypeId::Interval, DataTypeId::Int64],
            //     variadic: None,
            //     return_type: DataTypeId::Interval,
            // },
            // Signature {
            //     input: &[DataTypeId::Decimal64, DataTypeId::Decimal64],
            //     variadic: None,
            //     return_type: DataTypeId::Decimal64,
            // },
        ]
    }
}

impl ScalarFunction for Rem {
    fn plan(
        &self,
        table_list: &TableList,
        inputs: Vec<Expression>,
    ) -> Result<PlannedScalarFunction> {
        plan_check_num_args(self, &inputs, 2)?;

        let (function_impl, return_type): (Box<dyn ScalarFunctionImpl>, _) = match (
            inputs[0].datatype(table_list)?,
            inputs[1].datatype(table_list)?,
        ) {
            (DataType::Float16, DataType::Float16) => (
                Box::new(RemImpl::<PhysicalF16>::new(DataType::Float16)),
                DataType::Float16,
            ),
            (DataType::Float32, DataType::Float32) => (
                Box::new(RemImpl::<PhysicalF32>::new(DataType::Float32)),
                DataType::Float32,
            ),
            (DataType::Float64, DataType::Float64) => (
                Box::new(RemImpl::<PhysicalF64>::new(DataType::Float64)),
                DataType::Float64,
            ),
            (DataType::Int8, DataType::Int8) => (
                Box::new(RemImpl::<PhysicalI8>::new(DataType::Int8)),
                DataType::Int8,
            ),
            (DataType::Int16, DataType::Int16) => (
                Box::new(RemImpl::<PhysicalI16>::new(DataType::Int16)),
                DataType::Int16,
            ),
            (DataType::Int32, DataType::Int32) => (
                Box::new(RemImpl::<PhysicalI32>::new(DataType::Int32)),
                DataType::Int32,
            ),
            (DataType::Int64, DataType::Int64) => (
                Box::new(RemImpl::<PhysicalI64>::new(DataType::Int64)),
                DataType::Int64,
            ),
            (DataType::Int128, DataType::Int128) => (
                Box::new(RemImpl::<PhysicalI128>::new(DataType::Int128)),
                DataType::Int128,
            ),
            (DataType::UInt8, DataType::UInt8) => (
                Box::new(RemImpl::<PhysicalU8>::new(DataType::UInt8)),
                DataType::UInt8,
            ),
            (DataType::UInt16, DataType::UInt16) => (
                Box::new(RemImpl::<PhysicalU16>::new(DataType::UInt16)),
                DataType::UInt16,
            ),
            (DataType::UInt32, DataType::UInt32) => (
                Box::new(RemImpl::<PhysicalU32>::new(DataType::UInt32)),
                DataType::UInt32,
            ),
            (DataType::UInt64, DataType::UInt64) => (
                Box::new(RemImpl::<PhysicalU64>::new(DataType::UInt64)),
                DataType::UInt64,
            ),
            (DataType::UInt128, DataType::UInt128) => (
                Box::new(RemImpl::<PhysicalU128>::new(DataType::UInt128)),
                DataType::UInt128,
            ),

            // TODO: Interval, date, decimal
            (a, b) => return Err(invalid_input_types_error(self, &[a, b])),
        };

        Ok(PlannedScalarFunction {
            function: Box::new(*self),
            return_type,
            inputs,
            function_impl,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RemImpl<S> {
    datatype: DataType,
    _s: PhantomData<S>,
}

impl<S> RemImpl<S> {
    fn new(datatype: DataType) -> Self {
        RemImpl {
            datatype,
            _s: PhantomData,
        }
    }
}

impl<S> ScalarFunctionImpl for RemImpl<S>
where
    S: PhysicalStorage,
    for<'a> S::Type<'a>: std::ops::Rem<Output = S::Type<'static>> + Default + Copy,
    ArrayData: From<PrimitiveStorage<S::Type<'static>>>,
{
    fn execute(&self, inputs: &[&Array]) -> Result<Array> {
        let a = inputs[0];
        let b = inputs[1];

        let builder = ArrayBuilder {
            datatype: self.datatype.clone(),
            buffer: PrimitiveBuffer::with_len(a.logical_len()),
        };

        BinaryExecutor::execute::<S, S, _, _>(a, b, builder, |a, b, buf| buf.put(&(a % b)))
    }
}

#[cfg(test)]
mod tests {
    use rayexec_bullet::datatype::DataType;

    use super::*;
    use crate::expr;
    use crate::functions::scalar::ScalarFunction;

    #[test]
    fn rem_i32() {
        let a = Array::from_iter([4, 5, 6]);
        let b = Array::from_iter([1, 2, 3]);

        let mut table_list = TableList::empty();
        let table_ref = table_list
            .push_table(
                None,
                vec![DataType::Int32, DataType::Int32],
                vec!["a".to_string(), "b".to_string()],
            )
            .unwrap();

        let planned = Rem
            .plan(
                &table_list,
                vec![expr::col_ref(table_ref, 0), expr::col_ref(table_ref, 1)],
            )
            .unwrap();

        let out = planned.function_impl.execute(&[&a, &b]).unwrap();
        let expected = Array::from_iter([0, 1, 0]);

        assert_eq!(expected, out);
    }
}
