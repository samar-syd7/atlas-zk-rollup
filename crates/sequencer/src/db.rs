use anyhow::Result;

use rocksdb::{
    DB,
    Options,
};

use primitives::batch::RollupBatch;

pub struct Database {

    db: DB,
}

impl Database {

    pub fn new(
        path: &str,
    ) -> Result<Self> {

        let mut options =
            Options::default();

        options.create_if_missing(true);

        let db =
            DB::open(
                &options,
                path,
            )?;

        Ok(
            Self {
                db,
            }
        )
    }

    /*
        Persist batch.
    */
    pub fn save_batch(

        &self,

        batch: &RollupBatch,

    ) -> Result<()> {

        let key =
            format!(
                "batch:{}",
                batch.batch_id
            );

        let encoded =
            bincode::serialize(batch)?;

        self.db.put(
            key.as_bytes(),
            encoded,
        )?;

        Ok(())
    }

    /*
        Load batch.
    */

    #[allow(dead_code)]
    pub fn load_batch(

        &self,

        batch_id: u64,

    ) -> Result<Option<RollupBatch>> {

        let key =
            format!(
                "batch:{}",
                batch_id
            );

        if let Some(data) =
            self.db.get(
                key.as_bytes()
            )?
        {

            let batch:
                RollupBatch =
                    bincode::deserialize(
                        &data
                    )?;

            Ok(Some(batch))

        } else {

            Ok(None)
        }
    }
}