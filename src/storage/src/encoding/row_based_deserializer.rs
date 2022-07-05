// Copyright 2022 Singularity Data
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use risingwave_common::array::{Row, RowDeserializer};
use risingwave_common::error::Result;
use risingwave_common::types::DataType;

type ValueBytes = Vec<u8>;

#[derive(Clone)]
pub struct RowBasedDeserializer {
    data_types: Vec<DataType>,
}

impl RowBasedDeserializer {
    pub fn new(data_types: Vec<DataType>) -> Self {
        Self { data_types }
    }

    /// Serialize key and value. The `row` must be in the same order with the column ids in this
    /// serializer.
    pub fn deserialize(&mut self, row: ValueBytes) -> Result<Row> {
        let de = RowDeserializer::new(self.data_types.clone());
        let res = de.deserialize(&row)?;
        Ok(res)
    }
}
